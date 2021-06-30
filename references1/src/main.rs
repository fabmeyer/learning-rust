fn main() {
    let mut s1 = String::from("hello");

    // use a reference (refer to some value without taking ownership of it)
    let len = calculate_length(&s1);

    s1 = "hello, World".to_string();

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.