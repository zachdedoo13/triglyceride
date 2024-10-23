use eframe::{App, Frame, NativeOptions};
use eframe::epaint::Rect;
use egui::{Color32, Context, Painter, Pos2, Rounding, Stroke, Ui, Vec2, Window};

fn main() {
   let options = NativeOptions {
      ..Default::default()
   };

   eframe::run_native(
      "NewDisplay",
      options,
      Box::new(|_cc| Ok(Box::new(MyApp::new()))),
   ).expect("Failed to run");
}

pub struct MyApp {}
impl MyApp {
   pub fn new() -> Self {
      Self {}
   }
}
impl App for MyApp {
   fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
      Window::new("test")
          .show(ctx, |ui| {
             ui.label("Balls");

             ui.group(|ui| {
                ui.set_max_size(Vec2::new(250.0, 150.0));


                let data = vec![(0, [0.0, 250.0]), (1, [0.0, 150.0]), (1, [170.0, 200.0])];
                test(ui, data);
             });

          });
   }
}

const MAX_SEGMENTS: u32 = 6;


fn test(ui: &mut Ui, data: Vec<(u32, [f32; 2])>) {
   // setup
   let target_size = ui.available_size();

   let (widget_rect, mut responce) = ui.allocate_exact_size(target_size, egui::Sense {
      click: true,
      drag: true,
      focusable: true,
   });

   // interaction

   // draw

   if ui.is_rect_visible(widget_rect) {
      let _visuals = ui.style().noninteractive();
      let painter = ui.painter();
      let rect = widget_rect.expand(1.0);

      let segmentation = widget_rect.height() / MAX_SEGMENTS as f32;

      // painter.rect(
      //    Rect { min: Pos2::new(rect.min.x, lower_y + segmentation), max: Pos2::new(rect.max.x, rect.max.y) },
      //    Rounding::same(5.0),
      //    Color32::GRAY,
      //    Stroke::new(2.5, Color32::GOLD),
      // );

      // compass(painter, &rect);

      for prof in data.iter() {
         let data_rect = rect_from_seg_x(prof.1[0], prof.1[1], prof.0, segmentation, rect);

         painter.rect(
            data_rect,
            Rounding::same(5.0),
            Color32::DARK_GRAY,
            Stroke::new(1.0, Color32::GOLD),
         );
      }
   }
}

/// 0 is lowest segment
fn rect_from_seg_x(
   lower_x: f32,
   upper_x: f32,
   segment: u32,
   segmentation: f32,
   rect: Rect,
) -> Rect {
   Rect {
      min: Pos2::new(rect.min.x + lower_x, rect.max.y - segmentation * (segment + 1) as f32), // top
      max: Pos2::new(rect.min.x + upper_x, rect.max.y - segmentation * (segment) as f32), // bottom
   }
}

fn compass(painter: &Painter, rect: &Rect) {
   painter.circle(rect.center(), 5.0, Color32::RED, Stroke::default());

   painter.circle(rect.center() + Vec2::new(0.0, -25.0), 5.0, Color32::GOLD, Stroke::default());
   painter.circle(rect.center() + Vec2::new(0.0, 25.0), 5.0, Color32::BLUE, Stroke::default());

   painter.circle(rect.center() + Vec2::new(-25.0, 0.0), 5.0, Color32::GREEN, Stroke::default());
   painter.circle(rect.center() + Vec2::new(25.0, 0.0), 5.0, Color32::BROWN, Stroke::default());
}