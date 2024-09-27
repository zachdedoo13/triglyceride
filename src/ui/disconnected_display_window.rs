use std::sync::RwLock;
use std::thread;

use eframe::{EventLoopBuilder, Frame, UserEvent};
use egui::{CentralPanel, Context};
use crate::{open_profiler, PerformanceProfiler};

// todo disable this on web
pub fn spawn_disconnected_window(prof: &'static RwLock<PerformanceProfiler>) {
   println!("pre");

   let app = DisplayApp {
      prof,
   };

   struct DisplayApp {
      prof: &'static RwLock<PerformanceProfiler>,
   }
   impl eframe::App for DisplayApp {
      fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
         CentralPanel::default()
             .show(ctx, |ui| {
                open_profiler(self.prof, |mut p| p.handy_performance_benchmarking_ui_section_with_cool_looking_graphs_and_knobs_and_things_and_stuff_looks_very_cool(ui));
             });

         ctx.request_repaint();
      }
   }

   let _ = thread::Builder::new()
       .name("ProfilerWindowThread".to_string())
       .stack_size(4 * 1024 * 1024) // 4 MB stack size
       .spawn(|| -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
          let native_options = eframe::NativeOptions {
             vsync: true,
             event_loop_builder: Some(Box::new(|builder: &mut EventLoopBuilder<UserEvent>| {
                use winit::platform::windows::EventLoopBuilderExtWindows;
                builder.with_any_thread(true);
             })),
             viewport: egui::ViewportBuilder::default()
                 .with_inner_size([400.0, 300.0])
                 .with_min_inner_size([300.0, 220.0]),
             ..Default::default()
          };

          eframe::run_native(
             "Display example",
             native_options,
             Box::new(|_cc| Ok(Box::new(app))),
          ).expect("failed to run");
          Ok(())
       });

   println!("post");
}