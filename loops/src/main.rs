fn main() {
    // returning value in loop
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}, counter is {}", result, counter);

    // label loops
    let mut counter2 = 0;
    'counting_loop: loop {
        println!("counter2 is {}", counter2);
        let mut remaining = 10;
        loop {
            println!("remaining is {}", remaining);
            if remaining == 9 {
                break;
            }

            if counter2 == 2 {
                break 'counting_loop;
            }

            remaining -= 1;
        }
        counter2 += 1;
    }
    println!("counter2 is {}", counter2);

    // Conditional Loops with while

    let mut number = 5;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("Looping through a collection with for");
    let a = [10, 40, 50, 60, 70, 29, 60, 60];
    let mut index = 0;

    while index < (&a).len() {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // using for loop

    for element in a {
        println!("for::the value is: {}", element);
    }

    for n in (1..5).rev() {
        println!("Reversed n:{n}")
    }

    fibonacci(10);
}

fn fibonacci(n: u32) {
    let mut a = 0;
    let mut b = 1;
    let mut c = 1;
    if n <= 1 {
        println!("Fibonacci {}", n);
    } else {
        for _ in 1..n {
            c = a + b;
            a = b;
            b = c;
        }
    }
    println!("Fibonacci {}", c);
}
