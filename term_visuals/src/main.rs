use crossterm::{
    ExecutableCommand, QueueableCommand, style::{Color, Print, SetBackgroundColor, ResetColor}, terminal::{Clear, ClearType}, cursor::MoveTo
};
use rand::Rng;
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();

    let rectangles = [
        (5, 5, 10, 5), 
        (20, 5, 10, 5),
        (35, 5, 10, 5),
    ];

    loop {
        stdout.execute(Clear(ClearType::All)).unwrap();
        
        for &(x, y, width, height) in &rectangles {
            let color = Color::Rgb {
                r: rng.gen_range(0..=255),
                g: rng.gen_range(0..=255),
                b: rng.gen_range(0..=255),
            };

            for i in 0..height {
                stdout.queue(MoveTo(x, y + i)).unwrap()
                      .queue(SetBackgroundColor(color)).unwrap()
                      .queue(Print(" ".repeat(width as usize))).unwrap();
            }
        }

        stdout.queue(ResetColor).unwrap();
        stdout.flush().unwrap();

        sleep(Duration::from_millis(500));
    }
}
