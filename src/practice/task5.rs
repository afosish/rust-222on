use std::iter;

fn draw_tree(triangle_count: usize) {
    let max_width = 2 * triangle_count + 1;

    let draw_triangle = |level: usize| {
        for i in 0..level {
            let star_count = 2 * i + 3; // Починаємо з 3 зірочок, додаємо по 2 для кожного рівня
            let padding = (max_width - star_count) / 2;
            let line: String = iter::repeat(" ").take(padding)
                .chain(iter::repeat("*").take(star_count))
                .chain(iter::repeat(" ").take(padding))
                .collect();
            println!("{}", line);
        }
    };

    for level in 1..=triangle_count {
        let trunk_padding = (max_width - 1) / 2;
        let trunk_line: String = iter::repeat(" ").take(trunk_padding)
            .chain(iter::repeat("*").take(1))
            .chain(iter::repeat(" ").take(trunk_padding))
            .collect();
        println!("{}", trunk_line);

        draw_triangle(level);
    }
}
#[test]
fn main() {
    let triangle_count = 5;
    draw_tree(triangle_count);
}

