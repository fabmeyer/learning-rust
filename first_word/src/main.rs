fn main() {
    let mut s = String::from("hello world");

    let word = first_word(& s);
    
    // s.clear();
    // does not work because words value is connected to a string-slice (reference) to s
    // when manipulating s, also word gets changed
    // the compiler lets us manipulating s only after words scope is finished
    
    println!("the first word is: {}", word);

    s.clear(); // this does work, as long as word won't be accessed/called again
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