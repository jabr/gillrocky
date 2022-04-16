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

  pub fn idempotent() {
    let mut sa = systems::idempotent::create(seed(), &[0.5, 2.0, 5.0]);
    println!("sa -> r: {:?} c: {:?}", sa.0, sa.1.using().counts);
    for _ in 0..9 {
      sa.0.step();
      println!("sa -> r: {:?} c: {:?}", sa.0, sa.1.using().counts);
    }
  }
}

fn main() {
  // example::basic();
  example::idempotent();
}
