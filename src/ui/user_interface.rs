use std::hash::{DefaultHasher, Hash, Hasher};
use eframe::emath::Align;
use egui::{Layout, Stroke, TextStyle};
use egui::{CollapsingHeader, Color32, DragValue, menu, ScrollArea, Ui};
use egui_plot::{Bar, BarChart, Corner, Legend, Line, Plot, PlotPoint};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::profiler::PerformanceProfiler;
use crate::StatString;
use crate::utils::ui_modules::ToggleSwitch;

/// data held by the ui for
/// ``PerformanceProfiler::handy_performance_benchmarking_ui_section_with_cool_looking_graphs_and_knobs_and_things_and_stuff_looks_very_cool()``
#[derive(Debug)]
pub struct UiData {
   pub last_hovered_profile_tree: Option<StatString>,
   pub focused_profiles: Vec<StatString>,
   pub tree_or_list: bool,
   pub graph_included_upper_ms: f64,
   pub graph_included_upper_fps: f64,
}
impl Default for UiData {
   fn default() -> Self {
      Self {
         last_hovered_profile_tree: None,
         focused_profiles: vec![],
         tree_or_list: true,
         graph_included_upper_ms: 0.0,
         graph_included_upper_fps: 0.0,
      }
   }
}


/// main display
impl PerformanceProfiler {
   fn menubar(&mut self, ui: &mut Ui) {
      ui.group(|ui| {
         menu::bar(ui, |ui| {
            let settings = &mut self.settings;

            ui.horizontal(|ui| {
               ui.add(ToggleSwitch::new(&mut settings.active));
               ui.label("On/Off")
            });

            ui.menu_button("Settings", |ui| {
               ui.add(DragValue::new(&mut settings.update_interval_sec).speed(0.01).range(0.0..=f32::MAX).prefix("Data Update Interval -> ").suffix(" sec"));
               ui.add(DragValue::new(&mut settings.stored_cash_amount).speed(0.1).range(1..=200).prefix("Data averaging cash -> "));
               ui.add(DragValue::new(&mut settings.stored_data_amount).speed(0.5).range(1..=u32::MAX).prefix("stored datapoint's for graph -> "));


               ui.add(DragValue::new(&mut self.ui_data.graph_included_upper_ms).speed(1.0).range(0.0..=f64::MAX).prefix("Included upper milliseconds -> "));
            });

            if ui.button("Clear").clicked() {
               self.ui_data.focused_profiles.clear();
            }

            ui.menu_button("Help", |ui| {
               ui.label("Imagine some helpfully words")
            });
         });
      });
   }

   /// a cool looking ui for all the collected statistics, can be used inside any egui container, e.g. window or central panel,
   /// name is WIP
   pub fn handy_performance_benchmarking_ui_section_with_cool_looking_graphs_and_knobs_and_things_and_stuff_looks_very_cool(&mut self, ui: &mut Ui) {
      // menubar
      self.menubar(ui);

      let mw = ui.available_width();
      let hw = ui.available_height();

      ui.vertical(|ui| {
         ui.horizontal(|ui| {
            ui.set_min_height(hw * 0.7);

            ui.group(|ui| {
               ui.set_max_width(mw * 0.25);
               self.simple_function_tree_dropdown(ui);
            });


            ui.group(|ui| {
               self.display_graph_of_selected(ui);
            });

         });

         ui.group(|ui| {
            self.tree_bar_chart(ui);
         });

      });
   }
}

/// lists
impl PerformanceProfiler {
   pub fn list_all_functions(&mut self, ui: &mut Ui) {
      ui.group(|ui| {
         ScrollArea::vertical()
             .id_source("List all functions")
             .show(ui, |ui| {
                for (name, profile) in self.all_profiles.iter() {
                   ui.horizontal(|ui| {
                      ui.label(format!("{} => {name}", show_time(profile.pull_latest())));
                   });
                };
             });
      });
   }

   fn recursive_dropdown_of_children(&self, name: StatString, ui: &mut Ui) {
      let self_ms = self.all_profiles.get(name).unwrap().pull_latest();
      let children = &self.latest_tree.nodes.get(name).unwrap().children;
      let text = format!("{name} => {}", show_time(self_ms));
      match children.is_empty() {
         true => { ui.label(text); }
         false => {
            CollapsingHeader::new(text).id_source(name).show(ui, |ui| {
               let mut child_tot = 0.0;
               for child in children.iter() {
                  self.recursive_dropdown_of_children(child, ui);
                  child_tot += self.all_profiles.get(child).unwrap().pull_latest()
               }
               ui.label(format!(".. => {}", show_time(self_ms - child_tot)))
            });
         }
      };
   }
   pub fn simple_function_tree_dropdown(&mut self, ui: &mut Ui) {
      ui.group(|ui| {
         ScrollArea::vertical()
             .auto_shrink([true, true])
             .id_source("Simple dropdown")
             .show(ui, |ui| {
                match self.latest_tree.root {
                   None => { ui.label("No root node detected"); }
                   Some(root) => {
                      self.recursive_dropdown_of_children(root, ui);
                   }
                };
             });
      });
   }
}


/// tree
impl PerformanceProfiler {
   fn pull_data(&self, node: StatString) -> f64 {
      self.all_profiles.get(node).unwrap().pull_latest()
   }

   fn recursive_tree(
      &self,
      bars: &mut (Vec<Bar>, Vec<StatString>),
      node: StatString,
      depth: usize,
      start_from: f64,
      max: f64,
      farthest_depth: &mut usize,
   ) {
      if depth > *farthest_depth {
         *farthest_depth = depth;
      };

      let node_children = &self.latest_tree.nodes.get(node).unwrap().children;
      let data = self.pull_data(node) / max;

      bars.0.push(
         bar_from_x_plus(start_from, data, depth as f64, node)
      );

      bars.1.push(node);

      if !node_children.is_empty() {
         let mut rcs = start_from;
         for child in node_children.iter() {
            self.recursive_tree(bars, child, depth + 1, rcs, max, farthest_depth);
            rcs += self.pull_data(child) / max;
         }
      }
   }

   /// plots a horizontal (vertical breaks the math for now) barchart, tracks what's hovered / selected in ``self.ui_data``
   pub fn tree_bar_chart(&mut self, ui: &mut Ui) {
      let mut bars: (Vec<Bar>, Vec<StatString>) = (vec![], vec![]);

      // generate graph
      let mut _farthest_depth = 0;
      if let Some(root) = self.latest_tree.root {
         let max = self.pull_data(root);

         self.recursive_tree(&mut bars, root, 0, 0.0, max, &mut _farthest_depth);
      }

      let plot = Plot::new("FunctionTree")
          .show_grid([false, false])
          .show_axes([false, false])
          .allow_scroll(false)
          .allow_boxed_zoom(false)
          .allow_drag(false)
          .x_axis_label("Function tree");

      let barchart = BarChart::new(bars.0.clone());
      plot.show(ui, |plot_ui| {
         plot_ui.bar_chart(barchart);

         self.ui_data.last_hovered_profile_tree = None;

         if let Some(pos) = plot_ui.pointer_coordinate() {
            for (i, bar) in bars.0.iter().enumerate() {
               if aabb_collision_check(pos, gen_aabb(bar)) {
                  let n = bars.1[i];
                  if plot_ui.response().clicked() {
                     if let Some(index) = self.ui_data.focused_profiles.iter().position(|x| *x == n) {
                        self.ui_data.focused_profiles.remove(index);
                     } else {
                        self.ui_data.focused_profiles.push(n);
                     }
                  } else {
                     self.ui_data.last_hovered_profile_tree = Some(n);
                  }
               }
            }
         }
      });
   }
}


/// graph
impl PerformanceProfiler {
   pub fn display_graph_of_selected(&mut self, ui: &mut Ui) {
      let mut lines = vec![];

      // populate lines
      {
         for focused_profile in self.ui_data.focused_profiles.iter_mut() {
            let array = &self.all_profiles.get(focused_profile).unwrap().timings;
            let line = Line::new(array.clone())
                .name(focused_profile);
            lines.push(line);
         }

         if let Some(hovered) = self.ui_data.last_hovered_profile_tree {
            let array = &self.all_profiles.get(hovered).unwrap().timings;
            let line = Line::new(array.clone())
                .name(hovered);
            lines.push(line);
         }
      }

      Plot::new("Data plot")
          .allow_drag(false)
          .allow_scroll(false)
          .allow_zoom(false)
          .allow_boxed_zoom(false)
          .include_y(0.0)
          .include_y(self.ui_data.graph_included_upper_ms)
          .legend(
             Legend::default()
                 .position(Corner::LeftBottom)
                 .text_style(TextStyle::Small)
          )
             .show_axes([false, true])
             .y_axis_label("Milliseconds")
             .show(ui, |plot_ui| {
                for line in lines {
                   plot_ui.line(line);
                }
             },
          );
   }
}


// helper functions

fn show_time(t: f64) -> String {
   format!("{t:.2}ms")
}

fn gen_aabb(bar: &Bar) -> [PlotPoint; 2] {
   let left_x = bar.base_offset.unwrap();
   let right_x = left_x + bar.value;

   let middle_y = bar.argument;
   let upper_y = middle_y + HEIGHT / 2.0;
   let lower_y = middle_y - HEIGHT / 2.0;

   [PlotPoint::new(left_x, lower_y), PlotPoint::new(right_x, upper_y)]
}

fn aabb_collision_check(point: PlotPoint, aabb: [PlotPoint; 2]) -> bool {
   let bottom_left = aabb[0];
   let top_right = aabb[1];

   point.x >= bottom_left.x && point.x <= top_right.x &&
       point.y >= bottom_left.y && point.y <= top_right.y
}

const HEIGHT: f64 = 0.8;

fn bar_from_x_plus(x: f64, plus: f64, height: f64, name: StatString) -> Bar {
   Bar::new(height, plus)
       .horizontal()
       .base_offset(x)
       .width(HEIGHT)
       .name(name)
       .fill(rand_color(name))
       .stroke(
          Stroke {
             width: 1.0,
             color: Color32::GOLD,
          }
       )
}

fn stat_hash(key: StatString) -> u64 {
   let mut hasher = DefaultHasher::new();
   key.hash(&mut hasher);
   let hash = hasher.finish();

   hash
}

fn convert_to_color32(color_array: [f64; 4]) -> Color32 {
   let r = (color_array[0] * 255.0) as u8;
   let g = (color_array[1] * 255.0) as u8;
   let b = (color_array[2] * 255.0) as u8;
   let a = (color_array[3] * 255.0) as u8;

   Color32::from_rgba_unmultiplied(r, g, b, a)
}

fn rand_color(key: StatString) -> Color32 {
   let hash = stat_hash(key);
   let mut rng = StdRng::seed_from_u64(hash);

   let color_array = [
      rng.gen::<f64>(),
      rng.gen::<f64>(),
      rng.gen::<f64>(),
      0.5, // alpha channel
   ];

   convert_to_color32(color_array)
}
