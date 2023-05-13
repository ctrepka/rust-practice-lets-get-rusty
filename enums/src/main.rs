fn main() {
    println!("Hello, world!");
    //returns 6
    println!("One nickel plus one penny equals {} cents.", Coin::Nickel.value_in_cents() + Coin::Penny.value_in_cents());
    //returns "one"
    println!("{}", num_as_text_1to3(Some(1)));
    //not between 1,3
    println!("{}", num_as_text_1to3(Some(4)));
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> i8 {
        match self {
            Coin::Penny => return 1,
            Coin::Nickel => return 5,
            Coin::Dime => return 10,
            Coin::Quarter => return 25,
            _ => 0,
        }
    }
}
//use option types 
fn num_as_text_1to3(x: Option<i32>) -> String {
    match x {
        Some(1) => return String::from("one"),
        Some(2) => return String::from("two"),
        Some(3) => return String::from("three"),
        _ => String::from("Not a number between one and three, inclusive.")
    }
}