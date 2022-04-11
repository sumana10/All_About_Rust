fn main() {
    println!("Hello, world!");
    print_numbers_to(10);
    // if is_even(30){
    //     println!("It's Even");
    // }
}

fn print_numbers_to(num: u32){

    for n in 1..num {
        if is_even(n){
            println!("{} is Even", n);
        }
        else{
            println!("{} is Odd", n);
        }
    }
}

fn is_even(num: u32) -> bool{
    return num % 2 == 0;
}
