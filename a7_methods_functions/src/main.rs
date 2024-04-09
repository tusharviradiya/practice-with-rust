// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32{
//         self.width * self.height
//     }
// }

// fn main() {
//     println!("Hello, world!");
// }
#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    // Using `Self` to fill in the blank.
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }

    // Fill in the blank, DON'T use any variants of `Self`.
    pub fn change_state(&mut self) {
        self.color = "green".to_string()
    }

    //associate function
    pub fn new(color: String) -> Self {
        Self {
            color,
        }
    }
}
fn main() {
    let l = TrafficLight::new( "tushar viradiya".to_string());
    l.show_state();
}