// fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
//     let mut largest = &list[0];
//     for item in list{
//         if item > largest{
//             largest = item;
//         }
//     }
//     largest
// }

// struct Point<T, U>{
//     x: T,
//     y: U,
// }

// fn main() {
//     let mut list = vec![21, 53, 4, 6, 8, 10, 12, 14, 16];
//     let result = largest(&list);
//     println!("largest number : {}", result);

//     let point = Point{x: 5, y: 10.12};
//     println!("x: {}, y: {}", point.x, point.y);
// }

pub trait Summery{
    fn summary(&self) -> String;
}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}  

impl Summery for NewsArticle{
    fn summary(&self) -> String{
        format!("{} by {} and {}", self.headline, self.author, self.location)
    }
}

pub struct Tweet{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summery for Tweet{
    fn summary(&self) -> String{
        format!("{}: {}", self.username, self.content)
    }
    
}

fn main() {
    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false
    };
    println!("{}", tweet.summary());
}