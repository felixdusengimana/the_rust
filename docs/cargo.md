This is to define more about what goes in Cargo.toml

This file is in the [TOML (Tom’s Obvious, Minimal Language)](https://toml.io/en/) format

```toml
[package]
name = "<your_project_name>"
version = "0.1.0"
edition = "2018"
[dependencies]
```

The `package` section contains the metadata of the project.
The `name` is the name of the project, `version` is the version of the project, and `edition` is the Rust edition. The `dependencies` section contains the dependencies of the project.

### What is the Rust edition?

Every two or three years, Rust releases a new edition. The edition is a way to introduce new features and changes to the language. The current edition (as per writing this file) is 2021.
The edition specified in the `Cargo.toml` file, indicates which edition the compiler should use for your code. if the edition is not specified, the compiler will use the default edition which is 2015 for backward compatibility reasons.

more details about Cargo.toml can be found [here](https://doc.rust-lang.org/cargo/reference/manifest.html)


### Commands with Cargo

1. `cargo new <your_project_name>` - Create a new Rust project
2. `cargo build` - Build the project
3. `cargo run` - Build and run the project
4. `cargo check` - Check the project to make sure it compile without building it

For more information about Cargo, check out its [documentation](https://doc.rust-lang.org/cargo/).
