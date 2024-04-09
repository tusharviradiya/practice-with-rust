fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T{
    let mut largest = &list[0];
    for item in list{
        if item > largest{
            largest = item;
        }
    }
    largest
}

struct Point<T, U>{
    x: T,
    y: U,
}

fn main() {
    let mut list = vec![21, 53, 4, 6, 8, 10, 12, 14, 16];
    let result = largest(&list);
    println!("largest number : {}", result);

    let point = Point{x: 5, y: 10.12};
    println!("x: {}, y: {}", point.x, point.y);
}
