extern crate ndarray;
use crate::lib::{Reactor, Process};

pub mod constant {
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

pub mod diffusion {
  use super::{Reactor, Process};
  use ndarray::{Array, Dim};

  type State = Array<u64, Dim<[usize; 2]>>;

  struct Diffusion {
    k: f64,
    from: (usize, usize),
    to: (usize, usize),
  }

  impl Process<State> for Diffusion {
    fn rate(&self, state: &State) -> f64 {
      self.k * state[self.from] as f64
    }

    fn perform(&mut self, state: &mut State) {
      state[self.from] -= 1;
      state[self.to] += 1;
    }
  }

  pub fn create(
    seed: u128,
    k_diffusion: f64, size: usize,
    start: u64, periodic_boundary: bool
  ) -> (Reactor<State>, State) {
    let mut reactor = Reactor::new(seed);
    let mut state = Array::zeros((1,size));
    state[(0,0)] = start;

    for index in 1..size-1 {
      reactor.add(Diffusion { k: k_diffusion, from: (0,index), to: (0,index-1) });
      reactor.add(Diffusion { k: k_diffusion, from: (0,index), to: (0,index+1) });
    }

    // boundaries:
    reactor.add(Diffusion { k: k_diffusion, from: (0,0), to: (0,1) });
    reactor.add(Diffusion { k: k_diffusion, from: (0,size-1), to: (0,size-2) });
    if periodic_boundary {
      reactor.add(Diffusion { k: k_diffusion, from: (0,0), to: (0,size-1) });
      reactor.add(Diffusion { k: k_diffusion, from: (0,size-1), to: (0,0) });
    } else {
      // default to fixed (ie "bounce off wall" back to same location)
      reactor.add(Diffusion { k: k_diffusion, from: (0,0), to: (0,0) });
      reactor.add(Diffusion { k: k_diffusion, from: (0,size-1), to: (0,size-1) });
    }

    (reactor, state)
  }
}
