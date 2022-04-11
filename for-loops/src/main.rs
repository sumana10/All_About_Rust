fn main() {
    // println!("Hello, world!");
    // let numbers = 30..51;
    let animals = vec!["Rabbit", "Tortoise", "Tigress", "Giraffe"];
    // for i in numbers{
    //     println!("The number is {}", i);
    // }
    // for a in animals.iter(){
    //     println!("The animal name is {}", a);
    // }
    for (index, a) in animals.iter().enumerate(){
        println!("The index is {} animal name is {}", index, a);
    }
}
