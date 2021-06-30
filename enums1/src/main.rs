fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4(127,0,0,1)
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6(String::from("::1"))
    };

    let m = Message::Write(String::from("hello"));
    m.call();
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
}

// enum with four variants of different type
enum Message {
    Quit, // no data attached
    Move { x: i32, y: i32 }, // enum attached
    Write(String), // String object attached
    ChangeColor(i32, i32, i32), // tuple attached
}

// enums can implement methods like structs
impl Message {
    fn call(&self) {
        println!("test");
    }
}