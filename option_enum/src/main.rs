
fn main() {

    value_in_cents(Coin::Quarter(UsState::Alaska));
   
    // enum Option<T>{
    //     Some(T),
    //     None
    // }
    //Rust inferred the type from the values being passed in
   //for the none case no values passed in so we do have annotate the type

    // let some_number = Some(5);
    // let some_string = Some("a string");

    // let absent_number: Option<i32> = None;

    let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    let y: Option<i8> = None;

    //set some default value
    let sum = x + y.unwrap_or(0);
}
#[derive(Debug)]
enum UsState{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {

    match coin{
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}