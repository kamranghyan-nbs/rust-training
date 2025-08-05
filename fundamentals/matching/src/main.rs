#[derive(Debug)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}

fn main() {
    // let x = 8;

    // match x {
    //     1 => println!("One"),
    //     2 => println!("Two"),
    //     3 => println!("Three"),
    //     _ => println!("None")
    // }

    let my_coin = Coin::Dime;
    let value = value_in_cents(my_coin);

    println!("{}", value)
}
