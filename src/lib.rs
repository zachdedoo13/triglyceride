use std::sync::{RwLock, RwLockWriteGuard};

pub use lazy_static::lazy_static;

pub use function_profile::FunctionProfile;
pub use profiler::{PerformanceProfiler, Settings};
pub use tri_macros::time_event;

#[cfg(not(target_arch = "wasm32"))]
pub use ui::disconnected_display_window::spawn_disconnected_window;

pub(crate) mod function_profile;
pub(crate) mod profiler;
pub(crate) mod utils {
   pub(crate) mod tree;
   pub(crate) mod macros;
   pub(crate) mod ui_modules;
}
pub mod ui {
   pub mod user_interface;
   
   #[cfg(not(target_arch = "wasm32"))]
   pub(crate) mod disconnected_display_window;
}

pub type StatString = &'static str;


/// used to access a profiler static, takes a closure to a mut profiler 
/// ```
/// use triglyceride::{init_profiler, open_profiler, Settings};
/// 
/// init_profiler!(PROF, Settings::default());
/// 
/// fn main() {
/// 
///    // you can do anything with p such as manually starting and stopping timers
///    // mut this is the main use for it 
///    open_profiler(&PROF, |mut p| {
///       p.set_constant_reference("REFERENCE")
///    })
/// 
/// }
/// 
/// 
/// ```
#[inline(always)]
pub fn open_profiler<F>(profiler: &'static RwLock<PerformanceProfiler>, code: F)
where
    F: FnOnce(RwLockWriteGuard<'static, PerformanceProfiler>),
{
   code(profiler.write().unwrap());
}

/// used to modify a profilers settings though code instead of the ui
/// ```
/// use triglyceride::{init_profiler, open_profiler, Settings, change_profiler_settings};
///
/// init_profiler!(PROF, Settings::default());
///
/// fn main() {
///
///    change_profiler_settings(&PROF, |s: &mut Settings| {
///       s.active = false;
///       s.smoothing_amount = 1000;
///    });
///
/// }
///
///
/// ```
#[inline(always)]
pub fn change_profiler_settings<F>(profiler: &'static RwLock<PerformanceProfiler>, code: F)
where
    F: FnOnce(&mut Settings),
{
   open_profiler(profiler, |mut p| {
      code(&mut p.settings);
   });
}