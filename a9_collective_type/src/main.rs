// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
//     println!("{:?}", v);
//     v.push(6);
//     v.push(7);
//     v.push(8);
//     v.push(9);
//     println!("{:?}", v);
//     let a = &v[8];
//     println!("{:?}", a);
//     v.push(8);//after use mutable borrow we are not allowed to use immutalbe borrow or visa versa.
//     println!("{:?}", a);

//     match v.get(8) {
//         Some(x) => println!("their is a number: {:?}", x),
//         None => println!("their is no number"),
//     }
// }

use std::collections::HashMap;

fn main() {
    let str1 = String::from("hello");
    let str2 = String::from("tushar");

    let mut collection = HashMap::new();
    collection.insert(str1, 10);
    collection.insert(str2, 20);
    println!("{:?}", collection);
}