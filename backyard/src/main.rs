use crate::garden::vegetables;

pub mod garden;

fn main() {
    let veg1 = vegetables::Asparagus {};
    println!("Hello, world! {:?}", veg1);
}
