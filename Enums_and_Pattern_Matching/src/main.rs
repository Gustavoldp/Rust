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
        //enum options
        let four: IpAddrKind = IpAddrKind::V4(127, 0, 0 ,1);
        let six: IpAddrKind = IpAddrKind::V6(String::from("::1"));
    }
    {
        let m: Message = Message::Write(String::from("hello"));
        m.call();
    }
    {
        //match options
        let five = Some(5);
        let six = plus_one(five);
        let none = plus_one(None);
    }
    {
        //placeholders
        let some_u8_value: u8 = 0u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (), //placeholder to catch any other value.
            
        }
    }
        //if let
    {
            let some_u8_value = Some(0u8);
            match some_u8_value {
                Some(3) => println!("three"),
                _ => (),
            }
    
            //this behaves the same as the code above, id let act like a match and uses an = sign to verify the condition
            if let Some(3) = some_u8_value{
                println!("three");
            }
            //those two does the same thing
            {
                fn count_coin(coin: Coin){
                let mut count = 0;
                match coin {
                    Coin::Quarter(state) => print!("State quarter from {:?}", state),
                    _ => count+=1,
                }
            }
            }
            {
                fn count_coin(coin: Coin){
                let mut count = 0;
                if let Coin::Quarter(state) = coin {
                    println!("State quarter from {:?}!", state);
                }else{
                    count+1;
                }
            }
            }
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
fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
