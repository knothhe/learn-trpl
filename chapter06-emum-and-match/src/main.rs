enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let cents = value_in_cents(Coin::Penny);
    println!("cents: {cents}");

    let five = Some(5);
    let six = plus_one(five);
    println!("six: {six:?}");
    let none = plus_one(None);
    println!("none: {none:?}");
}
