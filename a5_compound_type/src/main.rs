// // Fix error without adding new line
// fn main() {
//     let s: &str = "hello, world";

//     println!("Success!");
// }


// Fix the error with at least two solutions
// fn main() {
//     let s: Box<str> = "hello, world".into();
//     greetings(&s)
// }

// fn greetings(s: &str) {
//     println!("{}",s)
// }


// Fill the blank
fn main() {
    let mut s = String::from("");
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success!");
}