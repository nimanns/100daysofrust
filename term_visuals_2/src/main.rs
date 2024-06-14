use crossterm::{
    ExecutableCommand, QueueableCommand, style::{Color, Print, SetBackgroundColor, ResetColor}, terminal::{Clear, ClearType}, cursor::MoveTo
};
use rand::Rng;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

#[derive(Clone, Copy)]
struct Rectangle {
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    dx: i16,
    dy: i16,
    color: Color,
}

fn main() {
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();

    let mut rectangles = [
        Rectangle { x: 5, y: 5, width: 10, height: 5, dx: 1, dy: 1, color: random_color(&mut rng) },
        Rectangle { x: 20, y: 5, width: 10, height: 5, dx: -1, dy: 1, color: random_color(&mut rng) },
        Rectangle { x: 35, y: 5, width: 10, height: 5, dx: 1, dy: -1, color: random_color(&mut rng) },
    ];

    loop {
        stdout.execute(Clear(ClearType::All)).unwrap();

        for rect in &mut rectangles {
            rect.color = random_color(&mut rng);
            draw_rectangle(&mut stdout, *rect);

            rect.x = (rect.x as i16 + rect.dx) as u16;
            rect.y = (rect.y as i16 + rect.dy) as u16;

            if rect.x == 0 || rect.x + rect.width >= 80 {
                rect.dx = -rect.dx;
            }
            if rect.y == 0 || rect.y + rect.height >= 24 {
                rect.dy = -rect.dy;
            }
        }

        stdout.queue(ResetColor).unwrap();
        stdout.flush().unwrap();

        sleep(Duration::from_millis(100));
    }
}

fn draw_rectangle<W: Write>(stdout: &mut W, rect: Rectangle) {
    for i in 0..rect.height {
        stdout.queue(MoveTo(rect.x, rect.y + i)).unwrap()
              .queue(SetBackgroundColor(rect.color)).unwrap()
              .queue(Print(" ".repeat(rect.width as usize))).unwrap();
    }
}

fn random_color(rng: &mut rand::rngs::ThreadRng) -> Color {
    Color::Rgb {
        r: rng.gen_range(0..=255),
        g: rng.gen_range(0..=255),
        b: rng.gen_range(0..=255),
    }
}
