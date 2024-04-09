// A variable can be used only if it has been initialized.
// fn main() {
//     let x: i32 = 5; // Uninitialized but used, ERROR !
//     let y: i32; // Uninitialized but also unused, only a Warning !

//     assert_eq!(x, 5);
//     println!("Success!");
// }

// Use mut to mark a variable as mutable.
// fn main() {
//     let mut x = 1;
//     x += 2; 
    
//     assert_eq!(x, 3);
//     println!("Success!");
// }

//scope
// fn main() {
//     let mut x: i32 = 10;
//     {
//         let y: i32 = 5;
//         println!("The value of x is {} and value of y is {}", x, y);
//         x += 1;
//     }
//     println!("The value of x is {}", x); 
// }

//destructuring
// fn main() {
//     let (x, y);
//     (x,..) = (3, 4);
//     [.., y] = [1, 2];
//     // Fill the blank to make the code work
//     assert_eq!([x,y], [3,2]);

//     println!("Success!");
// }

fn main(){
    struct Point {
        x: i32,
        y: i32,
    }
    
    let point = Point { x: 5, y: 10 };
    let Point { x, y } = point; // Destructure Point struct
    
    println!("x: {}, y: {}", x, y);
}