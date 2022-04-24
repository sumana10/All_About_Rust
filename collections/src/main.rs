fn main() {


    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.03),
        SpreadsheetCell::Text(String::from("blue")),

    ];

    match &row[1]{
        SpreadsheetCell::Int(i)=>println!("{}", i),
        _=>println!("Not a integer!")
    }
    // let mut v = vec![1, 2, 3, 4, 5];

    // for i in &mut v{

    //     *i += 50;
    //     //dereferencing operator to get the underline value

    //     println!("{}", i);
    // }

    // let third = &v[2];

    // // println!("The third element is {}", third);

    // match v.get(6){
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element")
    // }
    // println!("Hello, world!");

    // let a = [1, 2, 3];
    // let mut v:Vec<i32> = Vec::new();

    // v.push(1);
    // v.push(2);
    // v.push(3);

    // {
    //   let v2 = vec![1, 2, 3];
    // }
    //Rust can infer the types from the values passed into that

    
}
