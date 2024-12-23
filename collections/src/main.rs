use std::collections::HashMap;
use std::io;

fn main() {
    // vector
    let mut v1 = vec![1, 2, 23];

    // let first = &v1[0];
    v1.push(5);
    v1.push(6);

    // println!("v1 first {}", first);

    v1.pop();
    // let second: &i32 = &v1[0];
    v1.push(7);
    v1.push(8);

    println!("v1: {:?}", v1);

    let third: Option<&i32> = v1.get(2);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s3 = format!("{s1}------{s2}");
    println!("{s3}");

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");

    let mut v = vec![1, 2, 3, 3, 4, 5, 6, 2, 3, 4, 5];

    println!("Median, {}", median(&mut v));
    println!("Mode, {}", mode(&v));
    println!("Pig latin of hello, {}", pig_latin("hello"));
    println!("Pig latin of apple, {}", pig_latin("apple"));
    department_manager();
}

fn median(v: &mut Vec<i32>) -> i32 {
    v.sort();
    let mid = v.len() / 2;
    return v[mid];
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    let mut max = 0;
    let mut mode = 0;

    for &i in v {
        let count = map.entry(i).or_insert(0);
        *count += 1;

        if *count > max {
            max = *count;
            mode = i;
        }
    }

    return mode;
}

fn pig_latin(s: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    let first_char = s.chars().next().unwrap();

    if vowels.contains(&first_char) {
        return format!("{s}-hay");
    } else {
        let rest = &s[1..];
        return format!("{rest}-{first_char}ay");
    }
}

fn department_manager() {
    let mut departments = HashMap::new();

    loop {
        println!("Enter command");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let command: Vec<String> = guess
            .trim()
            .to_lowercase()
            .split_whitespace()
            .map(String::from)
            .collect();

        if command[0] == "add" {
            if command.len() == 4 && command[2] == "to" {
                let employee = command[1].clone();
                let department = command[3].clone();

                let employees = departments.entry(department).or_insert(Vec::new());
                employees.push(employee);
            } else {
                println!("Please try again");
            }
        } else if command[0] == "remove" {
            let employee = command[1].clone();
            let department = command[3].clone();

            if !departments.contains_key(&department) {
                println!("Department not found");
                return;
            }

            let employees = departments.entry(department).or_insert(Vec::new());
            if employees.contains(&employee) {
                let index = employees.iter().position(|e| e == &employee).unwrap();
                employees.remove(index);
            } else {
                println!("Employee not found");
            }
        } else if command[0] == "list" {
            if command.len() == 1 {
                for (department, employees) in &departments {
                    println!("{}: {:?}", department, employees);
                }
            } else {
                let department = command[1].clone();
                if departments.contains_key(&department) {
                    let employees = departments.entry(department).or_insert(Vec::new());
                    println!("{:?}", employees);
                } else {
                    println!("Department not found");
                }
            }
        } else if command[0] == "quit" {
            break;
        } else {
            println!("Invalid command");
        }
    }
}
