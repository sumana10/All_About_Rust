use unicode_segmentation::UnicodeSegmentation;

fn main() {
    

    let s1 = String::new();
    let s2 = "initial contents";
    let s3 = s2.to_string();
    let s4 = String::from("initial contents");


    let mut s = String::from("foo");

    s.push_str("bar");

    s.push('!');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3: String = s1 + &s2;
    let s3 = format!("{}{}", s1, s2);

    let hello: String = String::from("Hello");

    for b in hello.bytes(){
        println!("{}", b);
    }

    for b in hello.chars(){
        println!("{}", b);
    }

    for g in hello.graphemes(true){
        println!("{}", g);
    }

}
