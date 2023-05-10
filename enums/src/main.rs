struct IpV4Addr; // Unit struct

struct IpV6Addr;

enum IpAddrKindUsingStruct {
    V4(IpV4Addr), // We can put any kind of data inside each variant of enum
    V6(IpV6Addr),
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddrKindWithoutValue {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKindWithoutValue,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

fn main() {
    // Instance of each kind of enum variant
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::12"));

    route(four);
    route(six);
    route(IpAddrKind::V6(String::from("::54")));

    let home = IpAddr {
        kind: IpAddrKindWithoutValue::V4,
        address: String::from("127.0.0.1"),
    };

    let loop_back = IpAddr {
        kind: IpAddrKindWithoutValue::V6,
        address: String::from("::12"),
    };

    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loop_back = IpAddrKind::V6(String::from("::21"));

    let message = Message::Write(String::from("hello world"));

    message.call();

    practice_option();
}

// Here ip_type can be either V4 or V6
fn route(ip_type: IpAddrKind) -> IpAddrKind {
    ip_type
}

fn practice_option() {
    let a = 5;
    let b: Option<i32> = Some(10);

    match b {
        Option::Some(value) => {
            println!("{}", value + a);
        }
        Option::None => {
            println!("a value is null")
        }
    }
    // Here a + b will not work because b is of option i32. So it might be null. In order to fix that we can use match to handle all the possible case for vlaue b
    // println!("{}", a + b);
}
