mod lib;
mod systems;
mod utils;

mod example {
  use super::utils::{nanotime};
  use super::lib::{Reactor};

  pub fn basic() {
    println!("reactor - basic example");
    let seed = nanotime();

    println!("seed = {}", seed);
    let mut reactor = Reactor::new(seed);
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
}

fn main() {
  example::basic();

  let mut sa = systems::idempotent::create(42, &[0.5, 2.0, 5.0]);
  println!("sa -> r: {:?} c: {:?}", sa.0, sa.1.lock().unwrap().counts);
  for _ in 0..10 {
    sa.0.step();
    println!("sa -> r: {:?} c: {:?}", sa.0, sa.1.lock().unwrap().counts);
  }
}
