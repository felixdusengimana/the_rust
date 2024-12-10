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

    let wl2 = Rectangle {
        width: 10,
        height: 140,
    };

    println!("The area of the rectangle is {} square pixels.", wl1.area());
    // can_hold, if a function takes a self parameter, it is a method
    println!("Can wl1 hold wl2? {}", wl1.can_hold(&wl2));

    // square, associated function does not have a self parameter
    let sq = Rectangle::square(3);
    println!("The area of the square is {} square pixels.", sq.area());
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
