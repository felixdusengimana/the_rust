use std::fs::File;
use std::io;
use std::io::ErrorKind;

fn main() {
    let mut input = String::new();
    println!("Enter a number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Please type a number!");

    if input < 0 {
        panic!("Negative numbers are not allowed!");
    } else {
        println!("You entered: {}", input);
    }

    // result: error handling
    // let file_to_read = File::open("hello.txt");

    // let file = match file_to_read {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => panic!("Problem opening the file: {:?}", other_error),
    //     },
    // };
    //
    // using closures
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
