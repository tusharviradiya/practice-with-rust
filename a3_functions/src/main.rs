fn test() {
    println!("Hello, world!");
}

fn diverging() -> ! {
    panic!("This function never returns!");
}

fn main(){
    test();
    diverging();
}