
fn main() {
    println!("Hello, world!");

    // let tup1 = (20, 25, 30, 35);

    // let tup1 = (20, "Rust", 3.4, false);
    // let tup1 = (20, "Rust", 3.4, false, (1, 4, 7));

    let tup1 = (45, 6.7, "Computer");

    let (a, b, c) = tup1;

    // println!("{}", tup1.2);
    // println!("{}", (tup1.4).0);

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
}
