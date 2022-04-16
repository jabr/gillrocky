mod lib;
mod systems;
mod utils;

fn seed() -> u128 {
  let seed = 42;
  // let seed = utils::nanotime();
  println!("seed = {}", seed);
  seed
}

pub mod example {
  use super::lib::{Reactor};
  use super::systems;
  use super::seed;

  pub fn basic() {
    println!("reactor - basic example");
    let mut reactor = Reactor::new(seed());
    println!("{}", reactor);

    reactor.step();
    println!("{:?}", reactor);

    reactor.add(0.13);
    let pid = reactor.add(9.58);
    reactor.add(2.25);

    reactor.step();
    println!("{:?}", reactor);

    reactor.remove(pid);

    reactor.step();
    println!("{:?}", reactor);
  }

  fn run<T: std::fmt::Debug>(mut reactor: Reactor, state: systems::Shared<T>, steps: u32) {
    println!("sa -> r: {:?} c: {:?}", reactor, state.using());
    for _ in 0..steps {
      reactor.step();
      println!("sa -> r: {:?} c: {:?}", reactor, state.using());
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
