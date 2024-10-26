use eframe::epaint::Rect;
use egui::{Align2, Color32, FontId, Pos2, Response, Rounding, Sense, Stroke, Ui, Vec2};
use crate::PerformanceProfiler;
use crate::ui::user_interface::{GenericTreeBarThing, rand_color};

impl PerformanceProfiler {
   pub fn display_new_tree(&mut self, ui: &mut Ui, generic_tree_bar_thing: &mut GenericTreeBarThing) -> Response {
      // setup
      self.ui_data.last_hovered_profile_tree = None;

      let mut target_size = ui.available_size();
      target_size.y = (segment_height(ui) * 1.4) * generic_tree_bar_thing.layers.len() as f32;

      let (widget_rect, response) = ui.allocate_exact_size(target_size, Sense {
         click: true,
         drag: true,
         focusable: true,
      });

      // draw
      ui.painter().rect(widget_rect, Rounding::ZERO, Color32::BLACK, Stroke::default());

      generic_tree_bar_thing.normalize();
      generic_tree_bar_thing.sort_layers();
      let depth = generic_tree_bar_thing.layers.len();

      if ui.is_rect_visible(widget_rect) {
         let _visuals = ui.style().noninteractive();
         let rect = widget_rect.expand(1.0);

         let segmentation = widget_rect.height() / depth as f32;

         for (depth, layer) in generic_tree_bar_thing.layers.iter().enumerate() {
            for (_layer_index, bar) in layer.iter().enumerate() {
               let bar_rect = rect_from_seg_x(
                  bar.positions[0] as f32 * rect.width(),
                  (bar.positions[0] + bar.positions[1]) as f32 * rect.width(),
                  depth as u32,
                  segmentation,
                  rect,
               );

               let seg_resp = display_segment(
                  ui,
                  bar_rect,
                  bar.name,
                  bar.time as f32,
                  rand_color(bar.name),
               );

               if seg_resp.hovered() {
                  self.ui_data.last_hovered_profile_tree = Some(bar.name);
               }

               if seg_resp.clicked() {
                  if self.ui_data.focused_profiles.contains(&bar.name) {
                     self.ui_data.focused_profiles.retain(|b| b != &bar.name);
                  } else {
                     self.ui_data.focused_profiles.push(bar.name);
                  }

               }
            }
         }
      }

      response
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
const TEXT_SIZE: f32 = 11.5;
const MIN_WIDTH: f32 = 1.5;
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
   let mut con_rect = Rect::from_center_size(rect.center(), Vec2::new(rect.width(), segment_height(ui)))
       .shrink2(Vec2::new(WIDTH_SHRINKAGE, 0.0));

   if con_rect.width() < MIN_WIDTH {
      con_rect.set_width(MIN_WIDTH);
   }

   // input
   let mut response = ui.allocate_rect(con_rect.shrink(1.0), Sense {
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

   response = response.on_hover_text(name);

   ui.painter().rect(
      con_rect,
      Rounding::same(ROUNDING),
      col,
      Stroke::new(STROKE_THICKNESS, STROKE_COLOR),
   );

   // concat text
   let one_char_width = one_char_size.x;
   let size = ui.painter().layout_no_wrap(name.to_string(), FONT, Color32::PLACEHOLDER).size();

   let max_size = (size.x + LEFT_BUFFER) + (one_char_width * MS_SPACING as f32) + (one_char_width * ((time_ms as u32).to_string().len() + 6) as f32);

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