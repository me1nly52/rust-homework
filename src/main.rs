mod homework;

fn main() {
    let s = "abcdefgh";
    let n = 2;
    let rotated = homework::hw09::rotate(s, n);
    println!("Rotated: {}", rotated);
}
