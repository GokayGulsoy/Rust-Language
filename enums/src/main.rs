enum IpAddrKind {
     V4, // IPv4 (IP version 4)
     V6, // IPv6 (IP version 6)
}

enum IpAddrParam {
     V4(String),
     V6(String),
}

struct IpAddr {
    kind: IpAddrKind,  
    address: String,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let _four = IpAddrKind::V4;
    let _six =  IpAddrKind::V6;

    // using enum fields inside the loopback
    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // using enum fields inside the loopback
    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let _home = IpAddrParam::V4(String::from("127.0.0.1"));
    let _loopback = IpAddrParam::V6(String::from("::1"));

    // match control flow construct
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // concise control flow with if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
           println!("The maximum is configured to be {}",max); 
    }
}

// match control flow structure with Option enum
fn plus_one (x: Option<i32>) -> Option<i32> {
   match x {
        None => None,    
        Some(i) => Some(i+1)
   }  
}

// function that uses match control flow statement 
fn _value_in_cents(coin: Coin) -> u8 {
    match coin {
         Coin::Penny => {
            println!("Lucky penny!");
            1
         }
         Coin::Nickel => 2,
         Coin::Dime => 10,
         Coin::Quarter => 25,
    }
}
