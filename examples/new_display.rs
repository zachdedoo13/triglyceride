use std::thread::sleep;
use std::time::Duration;

use eframe::{App, Frame, NativeOptions};
use egui::Context;

use triglyceride::{open_profiler, Settings, time_event, time_event_mac};
use triglyceride::init_profiler;

fn main() {
   let options = NativeOptions {
      ..Default::default()
   };

   eframe::run_native(
      "NewDisplay",
      options,
      Box::new(|_cc| Ok(Box::new(TestTreePass::new()))),
   ).expect("Failed to run");
}

init_profiler!(PROF, Settings::default());

pub struct TestTreePass;
impl TestTreePass {
   pub fn new() -> Self {
      Self {}
   }
}
impl App for TestTreePass {
   #[time_event(PROF, "Main update interloop")]
   fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
      time_event_mac!(PROF, "DISPLAY_OLD", {
         open_profiler(&PROF, |mut p| {
            p.display_floating_window(ctx);
         });
      });

      time_event_mac!(PROF, "TEST", {
         sleep(Duration::from_millis(1));
         time_event_mac!(PROF, "TEST_D2", {
            sleep(Duration::from_millis(1));
         });
      });

      // time_event_mac!(PROF, "DISPLAY_NEW_PLACEHOLDER", {
      //    Window::new("test")
      //        .resizable(true)
      //        .show(ctx, |ui| {
      //           open_profiler(&PROF, |mut p| {
      //              if let Some(root) = p.latest_tree.root {
      //                 let mut tree = p.generate_generic_tree_bars(root);
      //                 p.display_new_tree(ui, &mut tree);
      //              }
      //           });
      //        });
      // });

      ctx.request_repaint();
   }
}