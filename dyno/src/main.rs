use std::io::{self, Write};
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{self, ClearType},
    cursor,
    ExecutableCommand,
};

const GROUND_Y: u16 = 20;
const DINO_X: u16 = 10;
const OBSTACLE_START_X: u16 = 80;

struct Game {
    dino_y: u16,
    obstacle_x: u16,
    score: u32,
    jumping: bool,
    jump_height: u16,
}

impl Game {
    fn new() -> Self {
        Self {
            dino_y: GROUND_Y,
            obstacle_x: OBSTACLE_START_X,
            score: 0,
            jumping: false,
            jump_height: 0,
        }
    }

    fn update(&mut self) {
        if self.jumping {
            if self.jump_height < 10 {
                self.dino_y -= 1;
                self.jump_height += 1;
            } else {
                self.dino_y += 1;
                self.jump_height -= 1;
                if self.jump_height == 0 {
                    self.jumping = false;
                }
            }
        }

        self.obstacle_x -= 1;
        if self.obstacle_x == 0 {
            self.obstacle_x = OBSTACLE_START_X;
            self.score += 1;
        }
    }

    fn draw(&self) -> io::Result<()> {
        let mut stdout = io::stdout();
        stdout.execute(terminal::Clear(ClearType::All))?;
        stdout.execute(cursor::MoveTo(0, 0))?;

        for y in 0..=GROUND_Y {
            for x in 0..OBSTACLE_START_X {
                if x == DINO_X && y == self.dino_y {
                    print!("D");
                } else if x == self.obstacle_x && y == GROUND_Y {
                    print!("X");
                } else if y == GROUND_Y {
                    print!("=");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        println!("Score: {}", self.score);
        stdout.flush()?;
        Ok(())
    }

    fn is_collision(&self) -> bool {
        self.obstacle_x == DINO_X && self.dino_y == GROUND_Y
    }
}

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut game = Game::new();
    let mut last_update = Instant::now();

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(' ') if !game.jumping => game.jumping = true,
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }

        if last_update.elapsed() >= Duration::from_millis(50) {
            game.update();
            game.draw()?;
            last_update = Instant::now();

            if game.is_collision() {
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;
    println!("\nGame Over! Final Score: {}", game.score);
    Ok(())
}
