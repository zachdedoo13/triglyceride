use std::thread::sleep;
use std::time::Duration;
use eframe::{App, Frame, NativeOptions};
use eframe::epaint::Rect;
use egui::{Align2, Color32, Context, FontId, Painter, Pos2, Response, Rounding, Sense, Stroke, TextBuffer, Ui, Vec2, Window};

use triglyceride::{open_profiler, Settings, time_event, time_event_mac};
use triglyceride::init_profiler;
use triglyceride::ui::user_interface::GenericTreeBarThing;

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

pub struct TestSegment {}
impl TestSegment {
   pub fn new() -> Self {
      Self {}
   }
}
impl App for TestSegment {
   fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
      Window::new("data")
          .resizable(true)
          .show(ctx, |ui| {
             ui.label("Balls");

             ui.group(|ui| {
                ui.set_max_size(ui.max_rect().size());


                let data = vec![(0, [0.0, 250.0]), (1, [0.0, 150.0]), (1, [170.0, 200.0])];
                test(ui, data);
             });
          });

      Window::new("segment")
          .resizable(true)
          .min_size(Vec2::ZERO)
          .default_size(Vec2::new(200.0, 100.0))
          .show(ctx, |ui| {
             ui.allocate_space(ui.available_size());

             let _resp = display_segment(ui, ui.max_rect(), "MAIN_UPDATE_INTERLOOP", 32.4, Color32::DARK_GRAY);

             let points = [Pos2::new(ui.min_rect().center().x, ui.min_rect().min.y), Pos2::new(ui.min_rect().center().x, ui.min_rect().max.y)];
             draw_divider_line(ui, points);
          });
   }
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

      time_event_mac!(PROF, "DISPLAY_NEW_PLACEHOLDER", {
         Window::new("test")
             .resizable(true)
             .show(ctx, |ui| {
                open_profiler(&PROF, |mut p| {
                   if let Some(root) = p.latest_tree.root {
                      let mut tree = p.generate_generic_tree_bars(root);
                      display_new_tree(ui, &mut tree);
                   }
                });
             });
      });

      ctx.request_repaint();
   }
}

pub fn display_new_tree(ui: &mut Ui, generic_tree_bar_thing: &mut GenericTreeBarThing) {
   // setup
   let target_size = ui.available_size();

   let (widget_rect, mut _response) = ui.allocate_exact_size(target_size, egui::Sense {
      click: true,
      drag: true,
      focusable: true,
   });

   // draw
   generic_tree_bar_thing.normalize();
   generic_tree_bar_thing.sort_layers();
   let depth = generic_tree_bar_thing.layers.len();

   if ui.is_rect_visible(widget_rect) {
      let _visuals = ui.style().noninteractive();
      let rect = widget_rect.expand(1.0);

      let segmentation = widget_rect.height() / depth as f32;

      for (depth, layer) in generic_tree_bar_thing.layers.iter().enumerate() {
         for bar in layer.iter() {

            let bar_rect = rect_from_seg_x(
               bar.positions[0] as f32 * rect.width(),
               (bar.positions[0] + bar.positions[1]) as f32 * rect.width(),
               depth as u32,
               segmentation,
               rect,
            );

            println!("{}  |  {bar_rect}, bar => {bar:?}", bar.name);

            display_segment(
               ui,
               bar_rect,
               bar.name,
               bar.time as f32,
               Color32::DARK_GRAY,
            );
         }
      }
   }
}

const MAX_SEGMENTS: u32 = 6;


fn test(ui: &mut Ui, data: Vec<(u32, [f32; 2])>) {
   // setup
   let target_size = ui.available_size();

   let (widget_rect, mut _response) = ui.allocate_exact_size(target_size, egui::Sense {
      click: true,
      drag: true,
      focusable: true,
   });

   // interaction

   // draw

   if ui.is_rect_visible(widget_rect) {
      let _visuals = ui.style().noninteractive();
      let rect = widget_rect.expand(1.0);

      let segmentation = widget_rect.height() / MAX_SEGMENTS as f32;

      for prof in data.iter() {
         let data_rect = rect_from_seg_x(prof.1[0], prof.1[1], prof.0, segmentation, rect);

         display_segment(ui, data_rect, "TEST_NAME_PLACEHOLDER", 4.23, Color32::DARK_GRAY);
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

fn segment_height(ui: &mut Ui) -> f32 {
   let one_char_size = ui.painter().layout_no_wrap("P".to_string(), FONT, Color32::PLACEHOLDER).size();
   one_char_size.y + HEIGHT_BUFFER
}


// display
const ROUNDING: f32 = 5.0;
const STROKE_THICKNESS: f32 = 1.0;
const STROKE_COLOR: Color32 = Color32::GOLD;
const HEIGHT_BUFFER: f32 = 10.0;
const WIDTH_SHRINKAGE: f32 = 5.0;

// formating
const FONT: FontId = FontId::monospace(TEXT_SIZE);
const DOT_COUNT: usize = 2;
const MS_SPACING: usize = 2;
const LEFT_BUFFER: f32 = 7.5;
const TEXT_SIZE: f32 = 12.5;
fn display_segment(
   ui: &mut Ui,
   rect: Rect,
   name: &str,
   time_ms: f32,
   color: Color32,
) -> Response
{
   // setup
   let one_char_size = ui.painter().layout_no_wrap("P".to_string(), FONT, Color32::PLACEHOLDER).size();

   // format container
   let con_rect = Rect::from_center_size(rect.center(), Vec2::new(rect.width(), segment_height(ui)))
       .shrink2(Vec2::new(WIDTH_SHRINKAGE, 0.0));

   // input
   let response = ui.allocate_rect(con_rect.shrink(1.0), Sense {
      click: true,
      drag: true,
      focusable: true,
   });

   // draw container
   let col = if response.hovered() {
      color.gamma_multiply(1.5)
   } else {
      color
   };

   ui.painter().rect(
      con_rect,
      Rounding::same(ROUNDING),
      col,
      Stroke::new(STROKE_THICKNESS, STROKE_COLOR),
   );

   // concat text
   let one_char_width = one_char_size.x;
   let size = ui.painter().layout_no_wrap(name.to_string(), FONT, Color32::PLACEHOLDER).size();

   let max_size = (size.x + LEFT_BUFFER) + (one_char_width * MS_SPACING as f32) + (one_char_width * ((time_ms as u32).to_string().len() + 5) as f32);

   let mut sized_text = if max_size > rect.width() {
      let diff = (max_size - rect.width()) + one_char_width * DOT_COUNT as f32;
      let char_diff = (diff / one_char_width).ceil() as usize;

      let mut txt = name.to_string();
      if txt.len() < char_diff {
         txt.clear();
      } else {
         txt.drain((txt.len() - char_diff)..txt.len());
         txt.push_str(".".repeat(DOT_COUNT).as_str());
         txt.push_str(" ".repeat(MS_SPACING).as_str());
      }
      txt
   } else {
      let mut txt = name.to_string();
      txt.push_str(" ".repeat(MS_SPACING).as_str());
      txt
   };

   sized_text.push_str(format!("{time_ms:.2}ms").as_str());

   // TODO ass, switch to printing the elapsed on the far right
   if ((sized_text.len() as f32 * one_char_width) + LEFT_BUFFER) > rect.width() {
      sized_text = format!("{time_ms:.1}ms");
   }
   if ((sized_text.len() as f32 * one_char_width) + LEFT_BUFFER) > rect.width() {
      sized_text = format!("{time_ms:.0}ms");
   }
   if ((sized_text.len() as f32 * one_char_width) + LEFT_BUFFER) > rect.width() {
      sized_text = format!("{time_ms:.0}");
   }
   if ((sized_text.len() as f32 * one_char_width) + LEFT_BUFFER) > rect.width() {
      sized_text.clear();
   }

   // draw text
   ui.painter().text(Pos2::new(rect.min.x + LEFT_BUFFER, rect.center().y), Align2::LEFT_CENTER, sized_text, FONT, Color32::WHITE);

   response
}


const LINE_STROKE: Stroke = Stroke { width: 1.0, color: Color32::GRAY };
fn draw_divider_line(ui: &mut Ui, points: [Pos2; 2]) {
   ui.painter().line_segment(points, LINE_STROKE);
}