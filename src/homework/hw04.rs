const WIDTH: usize = 15;

pub fn draw_rhombus() {
    let mut buf = String::new();

    for i in 0..=(WIDTH / 2) {
        let stars = 2 * i + 1;
        let spaces = (WIDTH - stars) / 2;
        buf.push_str(&" ".repeat(spaces));
        buf.push_str(&"*".repeat(stars));
        buf.push('\n');
    }

    for i in (0..(WIDTH / 2)).rev() {
        let stars = 2 * i + 1;
        let spaces = (WIDTH - stars) / 2;
        buf.push_str(&" ".repeat(spaces));
        buf.push_str(&"*".repeat(stars));
        buf.push('\n');
    }

    print!("{}", buf);
}
