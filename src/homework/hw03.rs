
const WIDTH: usize = 30;
const HEIGHT: usize = 15;

pub fn draw_envelope() {
    let mut buf = String::new();

    for y in 0..HEIGHT {
        let main_pos = (y * (WIDTH - 1) + (HEIGHT - 1) / 2) / (HEIGHT - 1);
        let anti_pos = (WIDTH - 1) - main_pos;

        for x in 0..WIDTH {
            buf.push(if x == 0
                || x == WIDTH - 1
                || y == 0
                || y == HEIGHT - 1
                || x == main_pos
                || x == anti_pos
            {
                '*'
            } else {
                ' '
            });
        }

        buf.push('\n');
    }

    print!("{}", buf);
}
