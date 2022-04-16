use crate::lib;

use std::sync::{Arc, Mutex, MutexGuard};
pub struct Shared<T>(Arc<Mutex<T>>);
impl<T> Shared<T> {
  fn new(data: T) -> Self { Self(Arc::new(Mutex::new(data))) }
  fn clone(&self) -> Self { Self(Arc::clone(&self.0)) }
  pub fn using(&self) -> MutexGuard<T> { self.0.lock().unwrap() }
}

pub mod idempotent {
  use super::Shared;
  use super::lib::{Reactor, Process};

  pub struct State {
    pub counts: Vec<u64>,
  }

  struct IncrementingProcess {
    rate: f64,
    index: usize,
    state: Shared<State>,
  }

  impl Process for IncrementingProcess {
    fn rate(&self) -> f64 { self.rate }
    fn perform(&mut self) {
      self.state.using().counts[self.index] += 1;
    }
  }

  pub fn create(seed: u128, rates: &[f64]) -> (Reactor, Shared<State>) {
    let mut reactor = Reactor::new(seed);
    let state = Shared::new(State { counts: vec![0; rates.len()] });

    for (index, rate) in rates.iter().enumerate() {
      let p = IncrementingProcess {
        rate: *rate,
        index,
        state: state.clone(),
      };
      reactor.add(p);
    }

    (reactor, state)
  }
}
