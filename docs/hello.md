# Hello to Rust

These are essential command and keywords in Rust.

1. `rustc` - Rust compiler
2. `cargo` - Rust package manager
3. `fn` - Function keyword
4. `marco!` - macro keywords are followed by `!`sign, like `println!`
5. `crate` - A package in Rust
6. `Packages`: A Cargo feature that lets you build, test, and share crates
7. `Crates`: A tree of modules that produces a library or executable
8. `Modules and use`: Let you control the organization, scope, and privacy of paths
9. `Paths`: A way of naming an item, such as a struct, function, or module


Using `cargo` is the best way to start a new Rust project. It will create a new directory with the project name and a `Cargo.toml` file. The `Cargo.toml` file contains the metadata of the project and dependencies.

```bash
$ cargo new your_project_name
$ cd your_project_name
$ ls
Cargo.toml  src
```
