// use rand::Rng;
// use rand::ErrorKind::Transient;
// use rand::CryptoRng;

use rand::{Rng, CryptoRng, ErrorKind::Transient};

use std::io::{self, Write};
// mod front_of_house{

//     mod hosting{
//         fn add_to_waitlist(){

//         }
//         fn seat_at_table(){

//         }
//     }
//     mod serving{

//         fn take_order(){}

//         fn serve_order(){}

//         fn take_payment(){}

//     }
// }

// mod front_of_house{
//    pub mod hosting{
//        pub fn add_to_waitlist(){}
//     }
// }

// pub fn eat_at_resturant(){
//     //Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     //Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order(){}

// mod back_of_house{
//     fn fix_incorrect_order(){
//         cook_order();
//         super::serve_order();
//     }
//     fn cook_order(){}
// }

// mod back_of_house{

//    pub struct Breakfast{

//        pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast{

//         pub fn summer(toast: &str) -> Breakfast{

//             Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches"),
//         }
//         }
//     }
// }

// pub fn eat_at_resturant(){
//     let mut meal = back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat")
// }

// mod back_of_house{
//     pub enum Appetizer{
//         Soup,
//         Salad,
//     }
// }

// pub fn eat_at_resturant(){

//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// mod front_of_house{

//     pub mod hosting{

//         pub fn add_to_waitlist(){}
//     }
// }

// use self::front_of_house::hosting;

// pub fn eat_at_resturant(){

//     // front_of_house::hosting::add_to_waitlist();
//     // front_of_house::hosting::add_to_waitlist();
//     // front_of_house::hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// use std::fmt;
// use std::io;

// //different result type coming from different module

// fn function1() -> fmt::Result{

// }

// fn function2() -> io::Result<()>{

// }

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result{

//     Ok(())
// }
// fn function2() -> IoResult<()>{

//     Ok(())
// }

mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_resturant(){

    let secret_number = rand::thread_rng().gen_range(1, 101);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}