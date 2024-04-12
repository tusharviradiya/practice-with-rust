fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let str1 = String::from("hello");
    let str2 = String::from("tushar");
    let rersult = longest(&str1, &str2);
    println!("{}", rersult);
}
