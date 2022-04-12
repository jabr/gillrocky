extern crate oorandom;

pub trait Process {
  fn rate(&self) -> f64;
  fn perform(&self);
}

use std::collections::HashSet;

pub struct Reactor {
  rng: oorandom::Rand64,
  steps: u64,
  time: f64,
  pub processes: HashSet::<Box<dyn Process>>,
}

impl Reactor {
  pub fn new(seed: u128) -> Self {
    Self {
      rng: oorandom::Rand64::new(seed),
      steps: 0,
      time: 0.0,
      processes: HashSet::new(),
    }
  }

  pub fn add(&mut self, b: Box<dyn Process>) {
    self.processes.insert(b);
  }

  // pub fn remove(&self, _p: &dyn Process) {
  //   // let b = Box::new(*p);
  //   // self.processes.remove(&b);
  //   println!("set count = {}", self.processes.len());
  // }

  pub fn step(&mut self) {
    let pairs = self.processes.iter().map(|p| (p, p.rate()));
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

// Implement traits needed to put Process references in a HashSet.
mod hash {
  use std::hash::{Hash, Hasher};
  use std::ptr::{eq, hash};
  use super::Process;

  impl Eq for Box<dyn Process> {}
  impl PartialEq for Box<dyn Process> {
    fn eq(&self, other: &Self) -> bool { eq(&self, &other) }
  }
  impl Hash for Box<dyn Process> {
    fn hash<H: Hasher>(&self, state: &mut H) { hash(self, state) }
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
