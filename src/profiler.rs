use std::collections::HashMap;

use instant::Instant;

use crate::function_profile::FunctionProfile;
use crate::StatString;
use crate::ui::user_interface::UiData;
use crate::utils::tree::Tree;

#[derive(Debug)]
pub struct Settings {
   pub active: bool,
   pub stored_data_amount: u32,
   pub stored_cash_amount: u32,
   pub update_interval_sec: f64,
}

impl Default for Settings {
   fn default() -> Self {
      Self {
         active: true,
         stored_data_amount: 3,
         stored_cash_amount: 2,
         update_interval_sec: 0.5,
      }
   }
}


/// a data structure for all the collected data, created globally and accessed
/// with ``get_profiler()``
#[derive(Debug)]
pub struct PerformanceProfiler {
   /// all timed functions ``HashMap<function, profile>``
   pub all_profiles: HashMap<StatString, FunctionProfile>,

   /// profiler settings
   pub settings: Settings,

   /// latest computed function tree
   pub latest_tree: Tree,

   /// queues a tree processes for the next iteration
   pub queue_processes_tree: bool,

   /// self-explanatory
   pub ui_data: UiData,

   pub(crate) is_actually_active_or_not: bool,
   pub(crate) last_dump: Instant,
   pub(crate) processioning_tree: bool,
   pub(crate) active_tree: Tree,
   pub(crate) traverser: Vec<StatString>,

   ticks_since_last_dump: u32,

   inner_constant_reference: Option<StatString>,

   /// first start event
   outermost_upper: Option<StatString>,

   /// last end event
   outermost_lower: Option<StatString>,
}
impl PerformanceProfiler {
   pub fn new(settings: Settings) -> Self {
      Self {
         all_profiles: Default::default(),
         settings,

         is_actually_active_or_not: true,
         latest_tree: Default::default(),
         last_dump: Instant::now(),
         queue_processes_tree: false,
         processioning_tree: false,
         active_tree: Default::default(),
         traverser: vec![],

         ticks_since_last_dump: 0,
         inner_constant_reference: None,
         outermost_upper: None,
         ui_data: UiData::default(),
         outermost_lower: None,
      }
   }
}

/// time functions
impl PerformanceProfiler {
   /// starts a profiler for a general function, use event loop variant for a function tree
   pub fn start_time_function(&mut self, name: StatString) {
      if !self.is_actually_active_or_not { return; }

      match self.all_profiles.get_mut(name) {
         None => {
            self.all_profiles.insert(name, FunctionProfile::default());
            self.start_time_function(name);
         }
         Some(profile) => {
            profile.start();
         }
      }
   }


   /// ends a profiler for a general function, use event loop variant for a function tree
   pub fn end_time_function(&mut self, name: StatString) -> Result<(), ()> {
      if !self.is_actually_active_or_not { return Ok(()); }

      match self.all_profiles.get_mut(name) {
         None => {
            self.all_profiles.insert(name, FunctionProfile::default());
            self.all_profiles.get_mut(name).unwrap().start();
            Ok(())
         }
         Some(profile) => {
            profile.end();
            Ok(())
         }
      }
   }
}

/// new implementation
impl PerformanceProfiler {
   fn at_outermost_upper(&mut self) {
      self.resolve_profiler(true);

      let (upper, lower) = (self.outermost_upper.unwrap(), self.outermost_lower.unwrap());

      match self.inner_constant_reference {
         None => {
            if upper != lower {
               panic!(
                  "The loop and no overarching function,\n
                   if you cannot encase the entire event loop in one function use set_constant_reference()\n
                     Upper = {:?} | Lower = {:?}
                   ", upper, lower
               )
            } // invalid check

            // start tree
            if self.queue_processes_tree {
               self.latest_tree = std::mem::take(&mut self.active_tree);
               self.active_tree.clear();
               self.traverser.clear();

               self.active_tree.set_root(upper);
               self.traverser.push(upper);
            }
         }

         Some(reference) => {
            if self.queue_processes_tree {
               self.latest_tree = std::mem::take(&mut self.active_tree);
               self.active_tree.clear();
               self.traverser.clear();

               self.active_tree.set_root(reference);
               self.traverser.push(reference);

               let parent = self.traverser.last().unwrap();
               self.active_tree.add_child(parent, upper);
               self.traverser.push(upper);
            }
         }
      }
   }

   /// starts profiling an inner event function
   pub fn time_event_start(&mut self, name: StatString) {
      match self.outermost_upper {
         None => {
            self.outermost_upper = Some(name);
         }

         Some(outer) => {
            if outer == name {
               self.at_outermost_upper();
            }

            // not outermost loop
            else {
               if self.queue_processes_tree {
                  let parent = self.traverser.last().unwrap();
                  self.active_tree.add_child(parent, name);
                  self.traverser.push(name);
               }
            }
         }
      }

      self.start_time_function(name);
   }

   /// ends profiling an inner event function
   pub fn time_event_end(&mut self, name: StatString) {
      self.end_time_function(name).unwrap();

      // is checked in outermost upper
      self.outermost_lower = Some(name);

      // function tree
      if self.queue_processes_tree {
         self.traverser.pop();
      }
   }


   /// sets a reference that is called every frame instead of an overarching function to start the tree
   /// todo hacky
   pub fn set_constant_reference(&mut self, name: StatString) {
      match self.inner_constant_reference {
         None => {
            self.inner_constant_reference = Some(name);
         }
         Some(_) => {
            self.end_time_function(name).unwrap();
         }
      }

      self.start_time_function(name);
   }
}

/// resolve functions
impl PerformanceProfiler {
   /// calculate averages, only runs every ``Settings::update_interval``
   pub fn resolve_profiler(&mut self, queue_tree: bool) {
      if self.is_actually_active_or_not != self.settings.active {
         if self.settings.active == false {
            self.inner_resolve(queue_tree);
         }
         self.is_actually_active_or_not = self.settings.active;
      }

      if self.is_actually_active_or_not { self.inner_resolve(queue_tree); }
   }

   fn inner_resolve(&mut self, queue_tree: bool) {
      self.ticks_since_last_dump += 1;
      if (self.last_dump.elapsed().as_secs_f64() > self.settings.update_interval_sec) && self.ticks_since_last_dump > 3 {
         self.ticks_since_last_dump = 0;
         self.last_dump = Instant::now();

         for (name, profile) in self.all_profiles.iter_mut() {
            if *name == self.inner_constant_reference.unwrap() {
               profile.resolve(self.settings.stored_cash_amount, self.settings.stored_data_amount, true);
            } else {
               profile.resolve(self.settings.stored_cash_amount, self.settings.stored_data_amount, false);
            }
         }

         self.queue_processes_tree = queue_tree;
      }
   }
}




