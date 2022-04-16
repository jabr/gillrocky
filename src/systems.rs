use crate::lib::{Reactor, Process};

use std::sync::{Arc, Mutex, MutexGuard};
pub struct Shared<T>(Arc<Mutex<T>>);
impl<T> Shared<T> {
  fn new(data: T) -> Self { Self(Arc::new(Mutex::new(data))) }
  fn clone(&self) -> Self { Self(Arc::clone(&self.0)) }
  pub fn using(&self) -> MutexGuard<T> { self.0.lock().unwrap() }
}

pub mod idempotent {
  use super::{Reactor, Process, Shared};

  #[derive(Debug)]
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

pub mod dimer {
  use super::{Reactor, Process, Shared};

  #[derive(Debug)]
  pub struct State {
    pub concentrations: Vec<u64>,
  }

  struct Reaction {
    k: f64,
    reactants: Vec<usize>,
    products: Vec<usize>,
    state: Shared<State>,
  }

  impl Process for Reaction {
    fn rate(&self) -> f64 {
      let state = self.state.using();
      self.reactants.iter().fold(self.k, |rate, index| {
        rate * (state.concentrations[*index] as f64)
      })
    }

    fn perform(&mut self) {
      let mut state = self.state.using();
      for index in &self.reactants {
        state.concentrations[*index] -= 1;
      }
      for index in &self.products {
        state.concentrations[*index] += 1;
      }
    }
  }

  pub fn create(
    seed: u128,
    k_formation: f64, k_dissociation: f64,
    concentrations: &[u64]
  ) -> (Reactor, Shared<State>) {
    let mut reactor = Reactor::new(seed);
    let state = Shared::new(State { concentrations: Vec::from(concentrations) });

    reactor.add(Reaction {
      k: k_formation,
      reactants: vec![0, 1],
      products: vec![2],
      state: state.clone(),
    });

    reactor.add(Reaction {
      k: k_dissociation,
      reactants: vec![2],
      products: vec![0, 1],
      state: state.clone(),
    });

    (reactor, state)
  }
}