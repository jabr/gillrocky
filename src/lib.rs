extern crate oorandom;

pub trait Process {
  fn rate(&self) -> f64;
  fn perform(&self);
}

pub struct Reactor {
  rng: oorandom::Rand64,
  steps: u64,
  time: f64,
  pub processes: Vec<Box<dyn Process>>,
}

impl Reactor {
  pub fn new(seed: u128) -> Self {
    Self {
      rng: oorandom::Rand64::new(seed),
      steps: 0,
      time: 0.0,
      processes: Vec::new(),
    }
  }

  pub fn step(&mut self) {
    let rates = self.processes.iter().map(|p| p.rate());
    let total_rate: f64 = rates.clone().sum();
    if total_rate > 0.0 {
      // Select an elapsed time from the probability distribution.
      let elapsed = -self.rng.rand_float().ln() / total_rate;

      // Randomly select a specific process to occur, weighted by the relative rates.
      let mut target = self.rng.rand_float() * total_rate;
      for (index, rate) in rates.enumerate() {
        if target < rate {
          self.processes[index].perform();
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
