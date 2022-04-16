// fn main() {
//     println!("Hello, world!");
//     a();
    
// }
// fn a(){
//     // let x = "hello";
//     // let y = 32;
//     b();
// }
// fn b(){
//     let x = String::from("world");
//     println!("{}", x);
// }
fn main(){
    // Ownership
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped
    {
        let s = "hello beautiful";
        //string literal store in stack
    }
    {
        let s = String::from("hii");
        //string type store in heap
    }

    let z = 5;
    let m = z;//copy

    let s1 = String::from("hello");
    // let s2 = s1;//value moved to s2
    let s2 = s1.clone();
    println!("{}, world", s1);



}