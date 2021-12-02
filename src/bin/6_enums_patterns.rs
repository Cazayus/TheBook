fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(&four);
    route(&six);

    struct IpAddrStruct {
        _kind: IpAddrKind,
        _address: String,
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let _ = match home {
        IpAddr::V4(125, _, _, _) => {}
        IpAddr::V6(_) => {}
        IpAddr::V4(_, _, _, _) => {}
    };
    let _loopback = IpAddr::V6(String::from("::1"));
    let _home = IpAddrStruct {
        _kind: IpAddrKind::V4,
        _address: String::from("127.0.0.1"),
    };

    let _loopback = IpAddrStruct {
        _kind: IpAddrKind::V6,
        _address: String::from("::1"),
    };
    let toto = Message::Move { x: 20, y: 320 };
    match toto {
        Message::Move { x, y: 32 } => println!("{}", x),
        Message::Move { .. } => {
            dbg!(toto);
        }
        Message::_Write(_) => (),
        Message::_ChangeColor(_, _, _) => (),
        Message::_Rectangle(_) => (),
        Message::_Quit => (),
    };
    let m = Message::_Write(String::from("hello"));
    m.call();

    let _some_number: Option<i32> = Some(5);
    let _some_string: Option<&str> = Some("a string");
    let _absent_number: Option<i32> = None;
    let five = Some(5);
    let six = plus_one(five);
    dbg!(six);
    let none = plus_one(None);
    dbg!(none);
    value_in_cents(Coin::Quarter(UsState::Alaska));
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::_Penny => 1,
        Coin::_Nickel => 5,
        Coin::_Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    _Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    _Penny,
    _Nickel,
    _Dime,
    Quarter(UsState),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum Message {
    _Quit,
    Move { x: i32, y: i32 },
    _Write(String),
    _ChangeColor(i32, i32, i32),
    _Rectangle(Rectangle),
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(_ip_kind: &IpAddrKind) {}
