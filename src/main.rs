fn main() {
    println!("Hello, world!");
    let a: i32 = 100;
    for i in 0..200 {
        println!("{}", if i < a { i } else { a - i });
    }
}
