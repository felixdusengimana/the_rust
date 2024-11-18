Rust's own standard library is called `std`. It is included with the Rust compiler and is available to all Rust programs. The `std` library provides essential functionality for working with the operating system, file I/O, networking, and more.

The `std` library is divided into modules, each of which contains a set of related functionality. For example, the `std::fs` module contains functions for working with the file system, and the `std::net` module contains functions for working with networking.

To use a module from the `std` library, you need to bring it into scope using the `use` keyword. For example, to use the `std::fs` module, you would write:

```rust
use std::fs;
```

This makes all the functions and types in the `std::fs` module available to your program.

The `std` library is automatically included in every Rust program, so you don't need to add it to your `Cargo.toml` file. You can use the `std` library in your program by simply importing the modules you need.

For more information about the `std` library, check out its [documentation](https://doc.rust-lang.org/std/).


##### What is a prelude?

The `prelude` is the list of things that Rust automatically imports into every Rust program. It’s kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.
