fn main() {
    // working with objects like String (content gets saved on the heap)
    // objects can be moved or cloned

    // Move
    println!("--- Move ---");
    let s1 = String::from("Hello");
    println!("s1 is: {}", s1);
    let mut s2 = s1;
    // the stack data of s1 (name, pointer, length and capacity) has moved to s2
    s2 += ", World!";
    // string concatenation
    println!("s2 is: {}", s2);
    // s1 is no longer valid

    // Clone
    println!("--- Clone ---");
    let s1 = String::from("Hello");
    let mut s2 = s1.clone();
    // the stack data of s1 is copied over to s2
    s2 += ", World!";
    println!("s1 is: {}, s2 is: {}", s1, s2);
    // s1 is still valid

    // Reference
    println!("--- Reference ---");
    let mut s1 = String::from("Hello");
    let s2 = &mut s1;
    // the pointer from s1 is copied to s2
    s2.push_str(", World!");
    println!("s2 is: {}", s2);
    // the scope of s2 ends here
    println!("s1 is: {}", s1);
    // manipulating s2 also manipulates s1


    // working with primitive types like int (content gets saved on the stack)
    // there is no move as stack gets copied (copy trait)

    // Copy
    println!("--- Copy ---");
    let i1 = 5;
    println!("i1 is: {}", i1);
    let mut i2 = i1;
    i2 += 1;
    println!("i1 is: {}, i2 is: {}", i1, i2);
    // s1 is still valid
}
