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

  reactor.processes.push(Box::new(0.13));
  reactor.processes.push(Box::new(9.58));
  reactor.processes.push(Box::new(2.25));

  reactor.step();
  println!("{}", reactor);
}
