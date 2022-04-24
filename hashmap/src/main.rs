
use std::collections::HashMap;

fn main() {
    
    // let blue = String::from("Blue");
    // let yellow = String::from("Yellow");

    // let mut scores = HashMap::new();

    // scores.insert(blue, 10);
    // scores.insert(yellow, 50);

    // // println!("{}", blue);

    // let team_name = String::from("Blue");

    // let score = scores.get(&team_name);

    // for(key, value) in &scores{

    //     println!("{}: {}", key, value);

    // }
    
//updating hashmap

// let mut scores = HashMap::new();

// //overwrite
// scores.insert( String::from("Blue"), 10);
// scores.insert( String::from("Blue"), 20);

// //insert if not exist 
// scores.entry( String::from("Yellow")).or_insert(30);
// scores.entry( String::from("Yellow")).or_insert(40);


let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace(){

    let count = map.entry(word).or_insert(0);
    println!("{}", count);
    *count += 1;

}

println!("{:?}", map);

}
