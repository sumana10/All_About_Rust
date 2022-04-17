fn main() {

    let a = [1, 2, 3, 4, 5];
    let slice = &a[0..2];
    let mut s = String::from("hello world");
    let s2 = "hello world";
    // let hello = &s[0..5];
    // let hello = &s[..5];
    // let world = &s[6..11];
    // let world = &s[6..];
    // let world = &s[..];//entire string
    // let word = first_word(&s);
    let word = first_word(s2);
    // s.clear();
    println!("the first word is: {}", word);
    
}

// fn first_word(s: &String) -> usize {

//     let bytes = s.as_bytes(); 

// for(i, &item) in bytes.iter().enumerate(){
//     if item == b' '{
//         return i;
//     }
// }

// s.len()

// }
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes(); 

for(i, &item) in bytes.iter().enumerate(){
    if item == b' '{
        return &s[0..i];
    }
}

&s[..]

}
