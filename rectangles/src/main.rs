#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let wl1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&wl1));
    dbg!(wl1);
}

fn area(s: &Rectangle) -> u32 {
    s.width * s.height
}
