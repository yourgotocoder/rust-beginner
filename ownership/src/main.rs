fn main() {
    let mut s = String::from("hello from me");

    let word = first_word(&s);

    println!("The first space of '{}' is at {}.", s, word);

    s.clear();//Compiler will catch the bug here

    println!("The first word is {}", word);
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