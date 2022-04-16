// fn main() {

//     let s = String::from("hello");
//     takes_ownership(s);

//     println!("{}", s);

//     let x = 5;
//     makes_copy(x);
//     println!("{}", x);
// }

// fn takes_ownership(some_string: String){
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32){
//     println!("{}", some_integer);
// }

fn main(){
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = take_and_give_back(s2);
    println!("{} {},", s3, s1);

}

fn gives_ownership() -> String{
    let some_string = String::from("Sumana!");
    some_string
}

fn take_and_give_back(a_string: String) -> String{
    a_string
}