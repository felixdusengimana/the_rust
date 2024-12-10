enum IPAdressKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IPAdressKind {
    fn unwrap(&self) -> String {
        match self {
            IPAdressKind::V4(a, b, c, d) => format!("{}.{}.{}.{}", a, b, c, d),
            IPAdressKind::V6(ip) => ip.to_string(),
        }
    }
}

fn main() {
    let four = IPAdressKind::V4(127, 0, 0, 1);
    let six = IPAdressKind::V6(String::from("::1"));

    take_kind(&four);
    take_kind(&six);

    // Option<T> enum
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    let absent_string: Option<&str> = None;
    let add_some_number = some_number.unwrap() + 5;

    println!("{:?}", some_number);
    println!("{:?}", some_string);
    println!("{:?}", absent_number);
    println!("{}", absent_string.unwrap_or_default());
    println!("{}", add_some_number);
}

fn take_kind(kind: &IPAdressKind) {
    println!("The ip address kind is: {}", kind.unwrap());
    // do other things
}
