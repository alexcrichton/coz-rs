# coz-rs

Rust support for the [`coz` Causal Profiler](https://github.com/plasma-umass/coz)

[![Documentation](https://docs.rs/coz/badge.svg)](https://docs.rs/coz)

## Disclaimer

As a disclaimer, note that this crate hasn't actually been proven out to work
locally. The crate is an attempt to port `coz.h` to Rust, and allow usage in
Rust through macros as well. I'm relatively certain that it's a faithful and
accurate translation of `coz.h` into Rust, but I have yet to actually get a good
profile out of `coz` yet!

All that to say, beware when using this crate. Or at least be ready for a few
bumps. If you get something working though please let me know! I'd love to
figure out how to wrangle `coz` myself to produce a nice pretty output!

## Usage

First add this to your `Cargo.toml`

```toml
[dependencies]
coz = "0.1"
```

Then you can use trace points just like `coz.h`, only in Rust-like syntax
instead:

```rust
fn foo() {
    coz::progress!(); // equivalent of `COZ_PROGRESS`
}

fn bar() {
    coz::progress!("named"); // equivalent of `COZ_PROGRESS_NAMED`
}

// equivalents of `COZ_BEGIN` and `COZ_END`
fn transaction() {
    coz::begin!("named");
    // ...
    coz::end!("named");
}
```

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
