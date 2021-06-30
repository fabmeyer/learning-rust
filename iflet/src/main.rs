fn main() {
    let some_u8_value = Some(0u8);
    // similiar to match but without exhausting search
    if let Some(3) = some_u8_value {
        println!("three");
        // use else as fallback
    } else {
        println!("not three");
    }
}
