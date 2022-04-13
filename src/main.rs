mod lib;
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
}
