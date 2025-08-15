pub fn draw_christmas_tree() {
    const LEVELS: usize = 6;
    let max_width = 2 * LEVELS - 1;

    for level in 1..=LEVELS {
        for row in 1..=level {
            let stars = 2 * row - 1;
            let spaces = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        }
    }
}
