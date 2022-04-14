fn main() {
    
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_0000;

    let y = 6;
    let y = "shadowing";
    // tuples
    let tup = ("Sumana", 10);
    let (name, score) = tup;
    let score = tup.1;
    // arrays
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let byte = [0; 8];

    // println!("{}",byte[1])

    let sum = my_function(11, 22);

    println!("The sum is: {}", sum);

    let condition = true;

    let number =  if condition { 5 } else { 6 };

    // 1. loop example

    // let mut counter = 0;
    // let result = loop{
    //     counter += 1;

    //     if counter == 10{
    //          break counter;
    //     }
    // };
    // println!("The result is {}", result);

    // 2. while-loop
    // let mut number = 3;
    // while number !=0 {
    //     println!("{}", number);
    //     number -= 1;
    // }
    // println!("LIGHTOFF!!!")
    // 3. for in
    let a = [10, 20, 30,40];
    for element in a.iter(){
        println!("the value is: {}", element);
    }
    for number in 1..4{
        println!("{}!", number);
    }
}

fn my_function(x: i32, y: i32) -> i32{
    println!("Another function: {}", x);
    // let sum = x + y;

    // return sum;
    // Or
    // sum
    // Or
    x + y

}
