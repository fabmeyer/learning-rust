#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// a method starts inside the impl keyword block
// using a reference means borrowing: ownership doesn't change
// main still has ownership after can hold is called
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // does not have self in the signature: is an associated function, not a method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };

    let r2 = Rectangle {
        width: 10,
        height: 40,
    };

    let r3 = Rectangle::square(60);

    println!("The are of the rectangle is: {}", r1.area());
    // pretty-print
    println!("r1 is {:#?}", r1);
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r2 hold r3? {}", r2.can_hold(&r3));
}

// a function starts with the fn keyword
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.heigth
// }