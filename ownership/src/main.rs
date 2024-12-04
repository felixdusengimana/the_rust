fn main() {
    let s = String::from("Hello");
    println!("{}", s);
    take_ownership(s);
    // this gives error: error[E0382]: borrow of moved value: `s`
    // println!("S after {}", s)
    let s = String::from("Hello");
    println!("{}", s);
    take_ownership(s);
    // this gives error: error[E0382]: borrow of moved value: `s`
    // println!("S after {}", s)
    //
    // References and Borrowing
    //
    let name = String::from("Felix");
    let size_of_name = print_name(&name);
    println!("The size of name: {name} is {size_of_name}");

    // Mutable reference
    let mut m = String::from("Hello");
    change(&mut m, ", World");
    println!("{}", m);
    change(&mut m, " my love");
    println!("{}", m);
}

fn take_ownership(s: String) {
    println!("{}", s);
}

fn print_name(name: &String) -> usize {
    println!("The name is {name}");
    return name.len();
}

fn change(s: &mut String, append_value: &str) {
    s.push_str(append_value);
}
