extern crate oorandom;

trait Process {
  fn rate(&self) -> f64;
  fn perform(&self);
}

struct Reactor {
  rng: oorandom::Rand64,
  steps: u64,
  time: f64,
  processes: Vec<Box<dyn Process>>,
}

impl Reactor {
  fn new(seed: u128) -> Self {
    Self {
      rng: oorandom::Rand64::new(seed),
      steps: 0,
      time: 0.0,
      processes: Vec::new(),
    }
  }

  fn step(&mut self) {
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

impl std::fmt::Display for Reactor {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "t: {}, n: {}", self.time, self.steps)
  }
}

impl Process for f64 {
  fn rate(&self) -> f64 { *self }
  fn perform(&self) { println!("chosen: {}", *self) }
}

fn nanotime() -> u128 {
  use std::time::{SystemTime, UNIX_EPOCH};
  SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos()
}

fn main() {
  println!("reactor example");
  let seed = nanotime();
  // let seed = 42;
  println!("seed = {}", seed);
  let mut reactor = Reactor::new(seed);
  println!("{}", reactor);

  reactor.step();
  println!("{}", reactor);

  reactor.processes.push(Box::new(0.13));
  reactor.processes.push(Box::new(9.58));
  reactor.processes.push(Box::new(2.25));

  reactor.step();
  println!("{}", reactor);
}
