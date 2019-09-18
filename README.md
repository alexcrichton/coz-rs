# coz-rs

Rust support for the [`coz` Causal Profiler](https://github.com/plasma-umass/coz)

[![Documentation](https://docs.rs/coz/badge.svg)](https://docs.rs/coz)

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
