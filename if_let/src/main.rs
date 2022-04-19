fn main() {
    // println!("Hello, world!");

    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);

    let some_value = Some(3);
    match some_value{
        Some(3) => println!("three"),
        _ =>(),
    }

    if let Some(3) = some_value{
        println!("three");
    }
}

fn plus_one(x: Option<i32>) ->
Option<i32>{

    match x{
        _ => None,
        Some(i)=>Some(i + 1),
    }
}