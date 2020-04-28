# mmheroes-core-rs

This is a library-level implementation of the "Heroes of Math & Mech" game.

It is designed to be very portable and hence doesn't have any dependencies.

It can be used in `no_std` contexts (you can run the game in an OS kernel if you want).
For that, build it with the feature `std` disabled by passing the `--no-default-features`
flag to cargo.

Clients of this library are required to implement the `Renderer` trait, which abstracts
away any actual text drawing.
For example, [mmheroes-rs](https://github.com/mmheroes/mmheroes-rs) is a curses-based
implementation.

This library also provides a pure C interface (see `mmheroes.h` and the `ffi` module)
that can be used to run the game from non-Rust code. If I ever finish this (spoiler: I won't),
I may implement an iOS version in Swift using this interface.
