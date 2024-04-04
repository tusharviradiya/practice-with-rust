// // Don't modify code in main!
// fn main() {
//     let s1 = String::from("Hello world");
//     let s3 = String::from("viradiya tushar rajeshbhai");
//     let s2 = take_ownership(s1);
//     println!("{:?}", s2);
//     println!("{:?}", s3);
// }

// // Only modify the code below!
// fn take_ownership(s: String) {
//     println!("{}", s);
// }


fn main() {
    let x = Box::new(5);
    
    let mut y = *x;     // update this line, don't change other lines!
    
    y = 4;
    
    assert_eq!(*x, 5);

    println!("Success!");
}