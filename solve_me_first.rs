use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let a: i32 = buffer.trim().parse().unwrap();
    buffer.clear();
    io::stdin().read_line(&mut buffer).unwrap();
    let b: i32 = buffer.trim().parse().unwrap();
    println!("{}", a + b);
}
