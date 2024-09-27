use triglyceride::{init_profiler, time_event_mac, Settings, open_profiler};

init_profiler!(PROF, Settings::default());


fn main() {
   triglyceride::spawn_disconnected_window(&PROF);



   loop {
      time_event_mac!(PROF, "YEENS", {
         time_event_mac!(PROF, "T1", {
            std::thread::sleep(std::time::Duration::from_millis(70));
            test();
         });
      });



      time_event_mac!(PROF, "DISCONNECTED", {
         std::thread::sleep(std::time::Duration::from_millis(30));
      });

      open_profiler(&PROF, |mut p| p.set_constant_reference("MAIN"));


      open_profiler(&PROF, |mut p| p.resolve_profiler(true));
   }
}

#[tri_macros::time_event(PROF, "TEST")]
fn test() {

}