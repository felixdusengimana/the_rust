### Rust variables and data types

All varibles are immutable by default in Rust. You can make a variable mutable by using the `mut` keyword.

```rust
let mut guess = String::new();
```

The above code creates a mutable variable `guess` of type `String`. We  then assign a new instance of `String` to `guess`.
The value that is bound to variable `guess` is a result of calling the `new` function which is associated to the `String` type.

##### What is associated function?

An associated function is a function that’s implemented on a type, in this case `String`

### Data types

1. `String`: A collection of characters. It is allocated on the heap and is growable.
2. `enum`: A type that can have a fixed set of values, called variants(possible state).
3. `i32`: A 32-bit signed integer.
