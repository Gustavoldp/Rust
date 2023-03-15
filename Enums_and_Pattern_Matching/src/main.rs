enum IpAddrKind{
    V4(u8, u8, u8, u8),
    V6(String),
}
enum Message {
    Quit,
    Move {x: u32, y: u32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self){
        // method body
    }
}

//Generic options can verify if all the possibilities
enum Options<T>{
    Some(T),
    None,
}
#[derive(Debug)] // so we can inspect the state.
enum UsState{
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    {
        let four: IpAddrKind = IpAddrKind::V4(127, 0, 0 ,1);
        let six: IpAddrKind = IpAddrKind::V6(String::from("::1"));
    }
    {
        let m: Message = Message::Write(String::from("hello"));
        m.call();
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
        println!("State quarter from {:?}", state);
            25
        },
    }
}
