extern crate oorandom;
extern crate getrandom;

pub fn seed() -> u128 {
  let mut bytes = [0u8; 16];
  if getrandom::getrandom(&mut bytes).is_err() {
    // as a fallback, use the system time and process id...
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    return nanos ^ std::process::id() as u128;
  }
  bytes.iter().fold(0u128, |a, b| a << 8 | (*b as u128))
}

pub trait Process {
  fn rate(&self) -> f64;
  fn perform(&mut self);
}

use std::collections::BTreeMap;
pub struct Reactor {
  sequence: u64,
  rng: oorandom::Rand64,
  steps: u64,
  time: f64,
  processes: BTreeMap<u64, Box<dyn Process>>,
}

impl Reactor {
  pub fn new(seed: u128) -> Self {
    Self {
      sequence: 0,
      rng: oorandom::Rand64::new(seed),
      steps: 0,
      time: 0.0,
      processes: BTreeMap::new(),
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
    let mut total_rate: f64 = 0.0;
    let mut pairs = Vec::new();
    for p in self.processes.values_mut() {
      let r = p.rate();
      total_rate += r;
      pairs.push((p, r));
    }

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
  use std::fmt::{Display, Debug, Formatter, Result};
  use super::{Reactor};

  impl Display for Reactor {
    fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "t: {}, n: {}", self.time, self.steps)
    }
  }

  impl Debug for Reactor {
    fn fmt(&self, f: &mut Formatter) -> Result {
      write!(
        f, "t: {:.9}, n: {} [seq={}, processes={}]",
        self.time, self.steps,
        self.sequence, self.processes.len()
      )
    }
  }
}
