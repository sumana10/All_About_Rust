// fn main() {
    
//     let s1 = String::from("hello");
//     // let (s2, len) = calculate_length(s1);
//     let (len) = calculate_length(&s1);
//     println!("The length of '{}' is {}.", s1, len);
// }

// // fn calculate_length(s: String) -> (String, usize){
// //     let length = s.len();
// //     (s, length)
// // }
// fn calculate_length(s: &String) -> (usize){
//     let length = s.len();
//     (length)
// }

// fn main(){
//     let mut s1 = String::from("hello");
//     change(&mut s1);
// }
// fn change(some_string: &mut String){

//     some_string.push_str(", world");

// }

// fn main(){
//     let mut s = String::from("world");

//     let r1 = &s;
//     let r2 = &s;

//     println!("{}, {}", r1, r2);

//     let r3 = &mut s;
//     println!("{}", r3);
// }
fn main(){
    let reference_to_nothing = dangle();

}

// fn dangle() -> &String{

//     let s = String::from("hello");
//     &s
// }

