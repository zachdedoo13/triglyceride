use instant::Instant;
use lazy_static::lazy_static;

lazy_static!(
   static ref ST: Instant = Instant::now();
);

#[derive(Debug)]
pub struct FunctionProfile {
   st: f64,
   max_stored_cash_amount: u32,

   average_cash: Vec<f64>,

   /// 0 is a rolling index, used for graphing with ``egui_graph``
   /// 1 is the actual time elapsed in ms
   pub timings: Vec<[f64; 2]>,
}
impl Default for FunctionProfile {
   fn default() -> Self {
      Self {
         st: get_ct(),
         max_stored_cash_amount: 10,
         average_cash: vec![],
         timings: vec![],
      }
   }
}
impl FunctionProfile {
   pub(crate) fn start(&mut self) {
      if (self.average_cash.len() as u32) < self.max_stored_cash_amount {
         self.st = get_ct();
      }
   }
   pub(crate) fn end(&mut self) {
      if (self.average_cash.len() as u32) < self.max_stored_cash_amount {
         self.average_cash.push(get_ct() - self.st);
      }
   }
   pub(crate) fn resolve(&mut self, stored_cash_amount: u32, stored_data_amount: u32, cull_first_average: bool, counter: u32) {
      self.max_stored_cash_amount = stored_cash_amount;

      if cull_first_average { self.average_cash.remove(0); }

      let ave: f64 = self.average_cash.iter().sum::<f64>() / self.average_cash.len() as f64;

      self.timings.push(
         [counter as f64, ave]
      );

      let diff = self.timings.len() as i32 - stored_data_amount as i32;
      if diff > 0 { self.timings.drain(0..(diff as usize)); }

      self.average_cash.clear();
   }

   /// pulls the latest elapsed time in ms from ``FunctionProfile::timings``
   pub fn pull_latest(&self) -> f64 {
      self.timings.last().unwrap_or(&[0.0, 0.0])[1]
   }
}

fn get_ct() -> f64 {
   ST.elapsed().as_secs_f64() * 1000.0
}