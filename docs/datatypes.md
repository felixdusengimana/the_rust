##### Scalar Data types
A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters. You may recognize these from other programming languages. Let’s jump into how they work in Rust.

##### Compound Data types
Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
println!("The value of y is: {}", y);
```
