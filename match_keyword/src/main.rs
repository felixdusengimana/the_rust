use std::io;

enum Age {
    Young(i32),
    Mid(i32),
    Old(i32),
    Invalid,
}

fn main() {
    let mut age = String::new();

    println!("Enter a number:: ");
    io::stdin()
        .read_line(&mut age)
        .expect("Unable to read provided line");
    let age: Age = match age.trim().parse() {
        Ok(age) => {
            if age >= 0 && age < 30 {
                Age::Young(age)
            } else if age >= 30 && age < 60 {
                Age::Mid(age)
            } else if age >= 60 {
                Age::Old(age)
            } else {
                Age::Invalid
            }
        }
        Err(err) => panic!("Error: {}", err),
    };

    match age {
        Age::Young(age) => println!("You are young, {}", age),
        Age::Mid(age) => println!("You are mid, {}", age),
        Age::Old(age) => println!("You are old, {}", age),
        Age::Invalid => println!("Invalid age"),
    }

    let dice_number = 1;

    match dice_number {
        8 => give_hearts_off(),
        7 => give_diamonds_off(),
        6 => give_clubs_off(),
        5 => give_spades_off(),
        _ => give_other_off(),
    }

    // using if let to match single pattern
    if let Age::Young(age) = age {
        println!("You are young, {}", age);
    }
}

fn give_hearts_off() {
    println!("Give hearts off");
}

fn give_diamonds_off() {
    println!("Give diamonds off");
}

fn give_clubs_off() {
    println!("Give clubs off");
}

fn give_spades_off() {
    println!("Give spades off");
}

fn give_other_off() {
    println!("Other")
}
