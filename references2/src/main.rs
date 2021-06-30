fn main() {
    // variables are immutable per default
    let mut s1 = String::from("hello");

    // references are immutable per default
    // you have to explicitly call reference as mutable
    // there can be only one mutable reference at once
    change(&mut s1);

    println!("s1: {}", s1); // nothing special here

    // create a new variable that is a reference to a existing variable
    // scope of mutable reference s2 starts here
    let mut s2 = &mut s1;
    
    // manipulating s2 also manipulates s1 because s2 is just a reference to s1
    s2.push_str("!!!");
    
    println!("s2: {}", s2);
    // scope of s2 ends here
    // println!("s1: {}", s1); // does not compile as function println wants to borrow s1
    
    let s3 = &mut s2;
    // scope of s3 starts here
    
    s3.push_str("...");
    println!("s3: {}", s3);
    // scope of s3 ends here
    // no more references to s1 are used here
    
    println!("s1: {}", s1); // same value as s2 and s3 before
    
    let s4 = &s1;

    println!("s4: {}", s4); // this does work as there can be an arbitrary number of immutable references to the same variable
}

// you have to explicitly call the argument as mutable
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}