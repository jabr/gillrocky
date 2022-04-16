pub mod lib;
mod systems;

fn seed() -> u128 {
  // let seed = 42;
  let seed = lib::seed();
  println!("seed = {:x}", seed);
  seed
}

pub mod example {
  use super::lib::{Reactor, Process};
  use super::systems;
  use super::seed;

  pub fn basic() {
    struct Static; // unit struct for an unchanging state
    impl<S> Process<S> for f64 {
      fn rate(&self, _: &S) -> f64 { *self }
      fn perform(&mut self, _: &mut S) { println!("-> chosen: {}", *self) }
    }

    println!("reactor - basic example");
    let mut reactor = Reactor::new(seed());
    println!("{}", reactor);

    reactor.step(&mut Static);
    println!("{:?}", reactor);

    reactor.add(0.13);
    let pid = reactor.add(9.58);
    reactor.add(2.25);

    reactor.step(&mut Static);
    println!("{:?}", reactor);

    reactor.remove(pid);

    reactor.step(&mut Static);
    println!("{:?}", reactor);
  }

  fn run<S: std::fmt::Debug>(mut reactor: Reactor<S>, mut state: S, steps: u32) {
    println!("{:?} {:?}", reactor, state);
    for _ in 0..steps {
      reactor.step(&mut state);
      println!("{:?} {:?}", reactor, state);
    }
  }

  pub fn idempotent() {
    let (reactor, state) = systems::idempotent::create(seed(), &[0.5, 2.0, 5.0]);
    run(reactor, state, 9);
  }

  pub fn dimer() {
    let (reactor, state) =systems::dimer::create(seed(), 2.0, 1.0, &[10, 10, 0]);
    run(reactor, state, 99);
  }
}

fn main() {
  example::basic();
  example::idempotent();
  example::dimer();
}
