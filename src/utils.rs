use crate::lib::Process;

impl Process for f64 {
  fn rate(&self) -> f64 { *self }
  fn perform(&mut self) { println!("chosen: {}", *self) }
}

#[allow(dead_code)]
pub fn nanotime() -> u128 {
  use std::time::{SystemTime, UNIX_EPOCH};
  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}
