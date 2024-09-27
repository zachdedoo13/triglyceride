use std::sync::{RwLock, RwLockWriteGuard};

pub use lazy_static::lazy_static;

pub use function_profile::FunctionProfile;
pub use profiler::{PerformanceProfiler, Settings};
pub use ui::disconnected_display_window::spawn_disconnected_window;
pub use tri_macros::time_event;

pub(crate) mod function_profile;
pub(crate) mod profiler;
pub(crate) mod utils {
   pub(crate) mod tree;
   pub(crate) mod macros;
   pub(crate) mod ui_modules;
}
pub mod ui {
   pub mod user_interface;
   pub mod disconnected_display_window;
}

pub type StatString = &'static str;


#[inline(always)]
pub fn open_profiler<F>(profiler: &'static RwLock<PerformanceProfiler>, code: F)
where
    F: FnOnce(RwLockWriteGuard<'static, PerformanceProfiler>),
{
   code(profiler.write().unwrap());
}

#[inline(always)]
pub fn change_settings<F>(profiler: &'static RwLock<PerformanceProfiler>, code: F)
where
    F: FnOnce(&mut Settings),
{
   open_profiler(profiler, |mut p| {
      code(&mut p.settings);
   });
}