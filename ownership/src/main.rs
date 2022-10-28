fn main() {
    let s1 = String::from("hello from me");

    let len = first_word(&s1);

    println!("The first space of '{}' is at {}.", s1, len);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}