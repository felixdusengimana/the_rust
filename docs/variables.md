### Rust variables and data types

All varibles are immutable by default in Rust. You can make a variable mutable by using the `mut` keyword.

```rust
let mut guess = String::new();
```

The above code creates a mutable variable `guess` of type `String`. We  then assign a new instance of `String` to `guess`.
The value that is bound to variable `guess` is a result of calling the `new` function which is associated to the `String` type.

Other way of creating a variable

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

The above code creates a constant variable `THREE_HOURS_IN_SECONDS` of type `u32` and assigns it the value of `60 * 60 * 3`.

`Constant evaluation` is a process of evaluating the value of a constant at compile time. Rust performs `constant evaluation` when the value of a constant is known at compile time.

`Const context` refers to `code` that is evaluated at compile time, as opposed to runtime.
Rust provides tools like const and static for working in constant contexts.

1. const Keyword:
```rust
const MAX_POINTS: u32 = 100_000;
```

2. static Keyword:
```rust
static GREETING: &str = "Hello, world!";
```

3. const functions:
```rust
const fn square(x: u32) -> u32 {
    x * x
}

const SQUARE_OF_TWO: u32 = square(2);
```

`Constant expression` is an expression that can be evaluated at compile time.
The following expressions are constant expressions,as long as any operands are themselves constant expressions and do not cause any `Drop::drop` calls to be run:

- Literals: integer and floating-point literals, character literals, `true`, `false`, and `()` (the unit type)
- Const parameters: any constant expression that is a `const` parameter
- Paths: references to functions or constants
- Paths to `statics`: references to static allowed only within the initializer of a static.
- Tuple expressions: tuple struct and tuple expressions, if they only contain other constant expressions
- Array expressions: array expressions, if they only contain other constant expressions
- Struct expressions: struct expressions, if they only contain other constant expressions
- Block expressions, including unsafe and const blocks.
- Field expressions.
- `Index` expressions, array indexing or slice indexing with `usize`.
- Range expressions.
- Closure expressions which don’t capture variables from the environment.
- Built-in negation, arithmetic, logical, comparison or lazy boolean operators used on integer and floating point types, bool, and char.
- Grouped expressions.
- Shared borrows, except if applied to a type with interior mutability.
- Cast expressions, except pointer to address casts and function pointer to address casts.
- Calls of const functions and const methods.
- loop, while and while let expressions.
- if, if let and match expressions.
