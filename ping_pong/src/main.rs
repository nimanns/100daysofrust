use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};
use std::thread::sleep;
use std::time::{Duration, Instant};

const PADDLE_HEIGHT: usize = 4;
const PADDLE_WIDTH: usize = 1;
const BALL_SIZE: usize = 1;
const TICK_RATE: Duration = Duration::from_millis(50);

struct Game {
    width: usize,
    height: usize,
    paddle1_y: usize,
    paddle2_y: usize,
    ball_x: usize,
    ball_y: usize,
    ball_vel_x: isize,
    ball_vel_y: isize,
}

impl Game {
    fn new(width: usize, height: usize) -> Game {
        Game {
            width,
            height,
            paddle1_y: height / 2 - PADDLE_HEIGHT / 2,
            paddle2_y: height / 2 - PADDLE_HEIGHT / 2,
            ball_x: width / 2,
            ball_y: height / 2,
            ball_vel_x: 1,
            ball_vel_y: 1,
        }
    }

    fn draw(&self) {
        let mut stdout = stdout();
        stdout.execute(cursor::Hide).unwrap();
        execute!(stdout, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All)).unwrap();

        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_paddle1(x, y) || self.is_paddle2(x, y) {
                    print!("|");
                } else if self.is_ball(x, y) {
                    print!("O");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
        stdout.flush().unwrap();
    }

    fn is_paddle1(&self, x: usize, y: usize) -> bool {
        x == 1 && y >= self.paddle1_y && y < self.paddle1_y + PADDLE_HEIGHT
    }

    fn is_paddle2(&self, x: usize, y: usize) -> bool {
        x == self.width - 2 && y >= self.paddle2_y && y < self.paddle2_y + PADDLE_HEIGHT
    }

    fn is_ball(&self, x: usize, y: usize) -> bool {
        x == self.ball_x && y == self.ball_y
    }

    fn update(&mut self) {
        self.ball_x = (self.ball_x as isize + self.ball_vel_x) as usize;
        self.ball_y = (self.ball_y as isize + self.ball_vel_y) as usize;

        if self.ball_y == 0 || self.ball_y == self.height - 1 {
            self.ball_vel_y *= -1;
        }

        if self.ball_x == 2 && self.ball_y >= self.paddle1_y && self.ball_y < self.paddle1_y + PADDLE_HEIGHT {
            self.ball_vel_x *= -1;
        }

        if self.ball_x == self.width - 3 && self.ball_y >= self.paddle2_y && self.ball_y < self.paddle2_y + PADDLE_HEIGHT {
            self.ball_vel_x *= -1;
        }

        if self.ball_x == 0 || self.ball_x == self.width - 1 {
            self.reset_ball();
        }
    }

    fn reset_ball(&mut self) {
        self.ball_x = self.width / 2;
        self.ball_y = self.height / 2;
        self.ball_vel_x = 1;
        self.ball_vel_y = 1;
    }

    fn handle_input(&mut self) {
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                match key_event.code {
                    KeyCode::Char('w') => {
                        if self.paddle1_y > 0 {
                            self.paddle1_y -= 1;
                        }
                    }
                    KeyCode::Char('s') => {
                        if self.paddle1_y < self.height - PADDLE_HEIGHT {
                            self.paddle1_y += 1;
                        }
                    }
                    KeyCode::Up => {
                        if self.paddle2_y > 0 {
                            self.paddle2_y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if self.paddle2_y < self.height - PADDLE_HEIGHT {
                            self.paddle2_y += 1;
                        }
                    }
                    KeyCode::Esc => {
                        execute!(stdout(), cursor::Show).unwrap();
                        std::process::exit(0);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    stdout.execute(terminal::EnterAlternateScreen).unwrap();

    let (width, height) = terminal::size().unwrap();
    let mut game = Game::new(width as usize, height as usize);

    let mut last_tick = Instant::now();

    loop {
        if last_tick.elapsed() >= TICK_RATE {
            game.update();
            game.draw();
            last_tick = Instant::now();
        }

        game.handle_input();
        sleep(Duration::from_millis(10));
    }
}

