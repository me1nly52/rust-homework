mod homework;
use homework::hw07::invert_the_case;

fn main() {
    let inputs = ["Hello", "Привет"];
    for &s in &inputs {
        println!("{}", invert_the_case(s.to_string()));
    }
}