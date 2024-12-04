fn main() {
    // slices are references to a contiguous sequence of elements in a collection
    let sentence = String::from("felix is a programmer");
    let word = first_word(&sentence);
    println!("The first word is {word}");

    let a = &sentence[0..5];
    let b = &sentence[..5];
    println!("a is {}", a);
    println!("b is {}", b);
    let c = &sentence[6..];
    println!("c is {}", c);
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
