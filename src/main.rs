trait Process {
  fn rate(&self) -> f64;
  fn perform(&self);
}

struct Reactor {
  steps: u64,
  time: f64,
  processes: Vec<Box<dyn Process>>
}

impl Reactor {
  fn new() -> Self {
    Self {
      steps: 0,
      time: 0.0,
      processes: Vec::new()
    }
  }

  fn step(&mut self) {
    let total_rate:f64 = self.processes.iter().map(|p| p.rate()).sum();
    println!("{}", total_rate);
    self.steps += 1;
    self.time += 0.1; // todo
    // self.choose().perform(); // todo
  }
}

impl Process for f64 {
  fn rate(&self) -> f64 { *self }
  fn perform(&self) {}
}

fn main() {
  let mut reactor = Reactor::new();
  reactor.step();
  reactor.processes.push(Box::new(44.5));
  reactor.processes.push(Box::new(2.25));
  reactor.step();
  println!("Hello World!");
}
