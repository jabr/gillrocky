use crate::lib;

pub mod idempotent {
  use super::lib::{Reactor, Process};
  use std::sync::{Arc, Mutex};

  pub struct System {
    pub counts: Vec<u64>,
  }

  struct IncrementingProcess {
    rate: f64,
    index: usize,
    system: Arc<Mutex<System>>,
  }

  impl Process for IncrementingProcess {
    fn rate(&self) -> f64 { self.rate }
    fn perform(&mut self) {
      println!("chosen {}", self.index);
      self.system.lock().unwrap().counts[self.index] += 1;
    }
  }

  pub fn create(seed: u128, rates: &[f64]) -> (Reactor, Arc<Mutex<System>>) {
    let mut reactor = Reactor::new(seed);
    let system = Arc::new(Mutex::new(System { counts: vec![0; rates.len()] }));

    for (index, rate) in rates.iter().enumerate() {
      let p = IncrementingProcess {
        rate: *rate,
        index,
        system: Arc::clone(&system),
      };
      reactor.add(p);
    }

    (reactor, system)
  }
}
