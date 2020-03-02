# Reproduction of ICE reported in [#69596](https://github.com/rust-lang/rust/issues/69596)

To trigger the issue:

1. Run `cargo check`
2. Delete the line that reads `pub(crate) use macros::xyz;` in `src/lib.rs`
3. Run `cargo check` again

Alternatively, running the `./demo` script will reproduce the error inside a docker build.
