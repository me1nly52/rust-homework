mod homework;

fn main() {
    let numbers = [0, 1, 2, 3, 4, 5, 100, 10007];

    for n in numbers {
        let result = homework::hw08::is_prime(&n);
        println!("{} is prime? {}", n, result);
    }
}
