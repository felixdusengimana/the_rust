struct User {
    first_name: String,
    last_name: String,
    email: String,
    age: u8,
    is_active: bool,
}

// Tuple Structs Without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs Without Any Fields: struct that don't have any fields
// struct UnitLikeStruct

fn main() {
    let user1 = User {
        first_name: String::from("Felix"),
        last_name: String::from("Doe"),
        email: String::from("phelixdusengimana@gmail.com"),
        age: 23,
        is_active: true,
    };
    // println!("{}", user1.first_name);
    let user2 = User {
        email: String::from("test@123.com"),
        ..user1
    };

    println!("{}", user1.age);

    // this won't work, because user1 (last_name and first_name) are moved to user2
    // println!("{}", user1.last_name);
    //
    // Using Tuple Structs Without Named Fields to Create Different Types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.0);
}
