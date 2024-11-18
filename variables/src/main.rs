fn main() {
    //constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Hello, world! {}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let x = 5;
    println!("The value of x is: {}", x);
    let x = 1;
    println!("The value of x is: {}", x);
    let x = x + THREE_HOURS_IN_SECONDS;
    println!("The value of x is: {}", x);

    // number literals with _(underscore)
    let x = 1_000_000;
    println!("The value of x is: {}", x * 2);
}
