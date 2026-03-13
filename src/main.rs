use std::{
    collections::VecDeque,
    io::Write,
    time::{Duration, Instant},
};

use crossterm::{
    event::{Event, poll, read},
    terminal,
};
use rand::RngExt;

fn main() {
    terminal::enable_raw_mode().unwrap();
    let mut game = Game {
        state: GameState::NotStarted,
        width: 40,
        height: 20,
        apple: Point { x: 21, y: 9 },
        snake: Snake {
            head_position: Point { x: 20, y: 10 },
            direction: Direction::Right,
            body: VecDeque::from_iter(vec![Point { x: 19, y: 10 }]),
        },
    };

    let mut next_tick = Instant::now();
    loop {
        if let Ok(true) = poll(next_tick - Instant::now()) {
            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    crossterm::event::KeyCode::Up => game.snake.change_direction(Direction::Up),
                    crossterm::event::KeyCode::Right => {
                        game.snake.change_direction(Direction::Right)
                    }
                    crossterm::event::KeyCode::Down => game.snake.change_direction(Direction::Down),
                    crossterm::event::KeyCode::Left => game.snake.change_direction(Direction::Left),
                    crossterm::event::KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        if next_tick <= Instant::now() {
            print_game(&game);
            game.tick();
            next_tick = next_tick + Duration::from_millis(50);
        }
    }

    terminal::disable_raw_mode().unwrap();
}

fn print_game(game: &Game) {
    clear_console();
    for x in 0..game.width {
        move_cursor(x + 1, 0);
        print!("-");
        move_cursor(x + 1, game.height + 1);
        print!("-");
    }
    for y in 0..game.height {
        move_cursor(0, y + 1);
        print!("|");
        move_cursor(game.width + 1, y + 1);
        print!("|");
    }

    move_cursor(game.apple.x + 1, game.apple.y + 1);
    print!("@");

    move_cursor(
        game.snake.head_position.x + 1,
        game.snake.head_position.y + 1,
    );
    match game.snake.direction {
        Direction::Up => print!("^"),
        Direction::Right => print!(">"),
        Direction::Down => print!("v"),
        Direction::Left => print!("<"),
    }

    let mut previous_position = &game.snake.head_position;
    game.snake.body.iter().for_each(|body_part| {
        move_cursor(body_part.x + 1, body_part.y + 1);
        match body_part.get_direction(&previous_position) {
            Direction::Up | Direction::Down => print!("|"),
            Direction::Right | Direction::Left => print!("-"),
        }
        previous_position = &body_part;
    });

    move_cursor(0, game.height + 2);
    std::io::stdout().flush().unwrap(); // Make sure the text appears immediately
}

fn move_cursor(x: isize, y: isize) {
    print!("\x1B[{};{}H", y + 1, x + 1);
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

impl Game {
    fn tick(&mut self) {
        let apple_eaten = self.snake.move_forward(&self.apple);
        if apple_eaten {
            self.apple = find_empty_position(self);
        }
    }
}

fn find_empty_position(game: &Game) -> Point {
    let mut rng = rand::rng();
    let random_x = rng.random_range(..game.width as u64) as isize;
    let random_y = rng.random_range(..game.height as u64) as isize;

    return Point {
        x: random_x,
        y: random_y,
    };
}

struct Game {
    snake: Snake,
    apple: Point,
    state: GameState,
    width: isize,
    height: isize,
}

enum GameState {
    NotStarted,
    Running,
    Dead,
}

impl Snake {
    fn move_forward(&mut self, apple: &Point) -> bool {
        let head_position_clone = self.head_position.clone();

        self.body.push_front(head_position_clone);

        match self.direction {
            Direction::Up => {
                self.head_position = Point {
                    x: self.head_position.x,
                    y: self.head_position.y - 1,
                }
            }
            Direction::Right => {
                self.head_position = Point {
                    x: self.head_position.x + 1,
                    y: self.head_position.y,
                }
            }
            Direction::Down => {
                self.head_position = Point {
                    x: self.head_position.x,
                    y: self.head_position.y + 1,
                }
            }
            Direction::Left => {
                self.head_position = Point {
                    x: self.head_position.x - 1,
                    y: self.head_position.y,
                }
            }
        }

        if &self.head_position == apple {
            return true;
        }

        self.body.pop_back();
        false
    }

    fn change_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    fn grow(&mut self) {
        todo!()
    }
}
struct Snake {
    head_position: Point,
    direction: Direction,
    body: VecDeque<Point>,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Point {
    fn get_direction(&self, to: &Point) -> Direction {
        if to.y < self.y {
            return Direction::Up;
        } else if to.y > self.y {
            return Direction::Down;
        } else if to.x < self.x {
            return Direction::Left;
        } else {
            return Direction::Right;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}
