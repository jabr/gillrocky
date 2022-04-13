extern crate oorandom;

pub trait Process {
  fn rate(&self) -> f64;
  fn perform(&self);
}

use std::collections::HashMap;
pub struct Reactor {
  sequence: u64,
  rng: oorandom::Rand64,
  steps: u64,
  time: f64,
  processes: HashMap<u64, Box<dyn Process>>,
}

impl Reactor {
  pub fn new(seed: u128) -> Self {
    Self {
      sequence: 0,
      rng: oorandom::Rand64::new(seed),
      steps: 0,
      time: 0.0,
      processes: HashMap::new(),
    }
  }

  pub fn add(&mut self, p: impl Process + 'static) -> u64 {
    let id = self.sequence;
    self.sequence += 1;
    self.processes.insert(id, Box::new(p));
    id
  }

  pub fn remove(&mut self, id: u64) {
    self.processes.remove(&id);
  }

  pub fn step(&mut self) {
    let pairs = self.processes.values().map(|p| (p, p.rate()));
    let total_rate: f64 = pairs.clone().map(|(_,r)| r).sum();

    if total_rate > 0.0 {
      // Select an elapsed time from the probability distribution.
      let elapsed = -self.rng.rand_float().ln() / total_rate;

      // Randomly select a specific process to occur, weighted by the relative rates.
      let mut target = self.rng.rand_float() * total_rate;
      for (process, rate) in pairs {
        if target < rate {
          process.perform();
          break;
        }
        target -= rate;
      }

      // Increment the current time.
      self.time += elapsed;
    }

    // Increment the reactor's total steps.
    self.steps += 1;
  }
}

mod utils {
  use std::fmt::{Display, Formatter, Result};

  impl Display for super::Reactor {
    fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "t: {}, n: {}", self.time, self.steps)
    }
  }
}
