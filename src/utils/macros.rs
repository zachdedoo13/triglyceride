/// alternative to function macro
#[macro_export]
macro_rules! profile_event_mac {
    ($profiler: ident, $name: literal, $code: block) => {
       triglyceride::open_profiler(&$profiler, |mut p| p.time_event_start($name));

       $code

       triglyceride::open_profiler(&$profiler, |mut p| p.time_event_end($name));
    };
}




#[macro_export]
macro_rules! init_profiler {
   ($name: ident, $settings: expr) => {
      triglyceride::lazy_static! {
         pub static ref $name: std::sync::RwLock<triglyceride::PerformanceProfiler> = std::sync::RwLock::new(triglyceride::PerformanceProfiler::new($settings));
      }
   }
}
