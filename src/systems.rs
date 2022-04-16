use crate::lib::{Reactor, Process};

pub mod idempotent {
  use super::{Reactor, Process};

  #[derive(Debug)]
  pub struct State {
    pub counts: Vec<u64>,
  }

  struct IncrementingProcess {
    rate: f64,
    index: usize,
  }

  impl Process<State> for IncrementingProcess {
    fn rate(&self, _: &State) -> f64 { self.rate }
    fn perform(&mut self, state: &mut State) {
      state.counts[self.index] += 1;
    }
  }

  pub fn create(seed: u128, rates: &[f64]) -> (Reactor<State>, State) {
    let mut reactor = Reactor::new(seed);
    let state = State { counts: vec![0; rates.len()] };

    for (index, rate) in rates.iter().enumerate() {
      let p = IncrementingProcess { rate: *rate, index };
      reactor.add(p);
    }

    (reactor, state)
  }
}

pub mod dimer {
  use super::{Reactor, Process};

  #[derive(Debug)]
  pub struct State {
    pub concentrations: Vec<u64>,
  }

  struct Reaction {
    k: f64,
    reactants: Vec<usize>,
    products: Vec<usize>,
  }

  impl Process<State> for Reaction {
    fn rate(&self, state: &State) -> f64 {
      self.reactants.iter().fold(self.k, |rate, index| {
        rate * (state.concentrations[*index] as f64)
      })
    }

    fn perform(&mut self, state: &mut State) {
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
  ) -> (Reactor<State>, State) {
    let mut reactor = Reactor::new(seed);
    let state = State { concentrations: Vec::from(concentrations) };

    reactor.add(Reaction {
      k: k_formation,
      reactants: vec![0, 1],
      products: vec![2],
    });

    reactor.add(Reaction {
      k: k_dissociation,
      reactants: vec![2],
      products: vec![0, 1],
    });

    (reactor, state)
  }
}
