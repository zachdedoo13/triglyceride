/// alternative to function macro
/// ```
///
/// use triglyceride::{time_event_mac, init_profiler, Settings};
/// init_profiler!(PROF, Settings::default());
/// 
/// fn test() {
///    time_event_mac!(PROF, "EVENT", {
///       let code = 1 + 1;
///    });
/// }
/// 
///  
/// 
///
/// ```
#[macro_export]
macro_rules! time_event_mac {
    ($profiler: ident, $name: literal, $code: block) => {
       triglyceride::open_profiler(&$profiler, |mut p| p.time_event_start($name));

       $code

       triglyceride::open_profiler(&$profiler, |mut p| p.time_event_end($name));
    };
}



/// initialized a ``public`` profiler static taking a name and settings as an input,
/// is used by all profiling functions 
/// ```
/// use triglyceride::{init_profiler, Settings};
/// 
/// 
/// init_profiler!(PROF, Settings::default());
/// ```
#[macro_export]
macro_rules! init_profiler {
   ($name: ident, $settings: expr) => {
      triglyceride::lazy_static! {
         pub static ref $name: std::sync::RwLock<triglyceride::PerformanceProfiler> = std::sync::RwLock::new(triglyceride::PerformanceProfiler::new($settings));
      }
   }
}
