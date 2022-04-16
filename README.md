# gillrocky

<img alt="Rocket Gillespie" title="Rocket Gillespie" src="./demo/src/icon.png" align="right" height="64px">

An implementation of the [Gillespie algorithm](https://en.wikipedia.org/wiki/Gillespie_algorithm) in [Rust](https://www.rust-lang.org/).

## Example

```sh
cargo build
cargo run
```

```rust
use gillrocky::{Reactor, Process};

struct State { ... }
struct Reaction { ... }

impl Process<State> for Reaction {
  fn rate(&self, state: &State) -> f64 { ... }
  fn perform(&mut self, state: &mut State) { ... }
}

let mut reactor = Reactor::new(seed);
reactor.add(Reaction { ... });
reactor.add(Reaction { ... });

let state = State { ... };
loop {
  reactor.step(&mut state);
  // output: reactor.time, state.*, etc
}
```

## See also

* [Animated demo of algorithm](./demo/README.md)

## References

* [Exact stochastic simulation of coupled chemical reactions](https://pubs.acs.org/doi/10.1021/j100540a008)
* [PCG, A Better Random Number Generator](https://www.pcg-random.org/)

## License

This project is licensed under the terms of the [MIT license](LICENSE.txt).

Icon derived from work by [Selman Design](https://selman.nyc/).
