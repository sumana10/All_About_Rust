// fn main() {

//     let mut x = 45;

//     println!("The value of x is: {}", x);

//     x = 60;

//     println!("The value of x is: {}", x);

// }

// fn main(){
//     let x = 45;//i32 signed 32 bit
//     let y: i64 = 54;
//     //i64  signed 64 bit
//     let z: u64 = 64;
//     // unsigned
//     let f: f32 = 6.7;
//     let b: bool = false;
    

//     println!()

// }

// fn main() {
//     // rust infers the type of x
//     let x = 13;
//     println!("{}", x);

//     // rust can also be explicit about the type
//     let x: f64 = 3.14159;
//     println!("{}", x);

//     // rust can also declare and initialize later, but this is rarely done
//     let x;
//     x = 0;
//     println!("{}", x);
// }

fn main() {
    let x = 12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );
}
