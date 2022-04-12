mod lib;
mod utils;

fn main() {
  println!("reactor example");
  let seed = utils::nanotime();
  // let seed = 42;
  println!("seed = {}", seed);
  let mut reactor = lib::Reactor::new(seed);
  println!("{}", reactor);

  reactor.step();
  println!("{}", reactor);

  reactor.add(0.13);
  reactor.add(9.58);
  reactor.add(2.25);

  reactor.step();
  println!("{}", reactor);

  reactor.remove(9.58);

  reactor.step();
  println!("{}", reactor);
}
