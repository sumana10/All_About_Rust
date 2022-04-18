// fn main() {
//     let width1: u32 = 30;
//     let height1: u32 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32{
//     width * height
// }

// fn main(){
//     let rect = (30, 50);

//     println!("The area of the rectangle is {} square pixels", area(rect));
// }

// fn area(dimensions: (u32, u32)) -> u32{
//     dimensions.0 * dimensions.1
// }
#[derive(Debug)]
struct Rectangle{
    
    width: u32,
    height: u32
}

impl Rectangle {

    //instance of rectangle struct is first argument of method 
    fn area(&self) -> u32{

        self.width * self.height

    }

    fn can_hold(&self, other: &Rectangle) -> bool{

        self.width > other.width && self.height > other.height

    }
}

impl Rectangle {

    fn square(size: u32) ->Rectangle{

        Rectangle { 
            width: size, 
            height: size
        }
    }
}
fn main(){

    let rect = Rectangle{
        width: 30,
        height: 50
    };

    let rect1 = Rectangle {
        width: 20,
        height: 40
    };
    let rect2 = Rectangle{

        width: 40,
        height: 50
    };

    let rect3 = Rectangle::square(25);

    println!("rect can hold rect1: {}", rect.can_hold(&rect1));
    println!("rect can hold rect2: {}", rect.can_hold(&rect2));

    println!("rect: {:?}", rect);
    // println!("The area of the rectangle is {} square pixels.", area(&rect));
    println!("The area of the rectangle is {} square pixels.", rect.area());
   


}

// fn area(rectangle: &Rectangle) -> u32{
//     rectangle.height * rectangle.width
// }

