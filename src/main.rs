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
    const WIDTH: isize = 40;
    const HEIGHT: isize = 20;

    terminal::enable_raw_mode().unwrap();
    let mut game = Game::new_game(WIDTH, HEIGHT);
    let mut next_tick = Instant::now();
    loop {
        if let Ok(true) = poll(next_tick - Instant::now()) {
            if let Ok(Event::Key(key)) = read() {
                match key.code {
                    crossterm::event::KeyCode::Up => {
                        game.snake.set_incoming_direction(Direction::Up)
                    }
                    crossterm::event::KeyCode::Right => {
                        game.snake.set_incoming_direction(Direction::Right)
                    }
                    crossterm::event::KeyCode::Down => {
                        game.snake.set_incoming_direction(Direction::Down)
                    }
                    crossterm::event::KeyCode::Left => {
                        game.snake.set_incoming_direction(Direction::Left)
                    }
                    crossterm::event::KeyCode::Char('r') => {
                        if game.state == GameState::GameOver {
                            game = Game::new_game(WIDTH, HEIGHT)
                        }
                    }
                    crossterm::event::KeyCode::Char('q') => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        if next_tick <= Instant::now() {
            match game.state {
                GameState::Running => {
                    print_game(&game);
                    game.update();
                }
                GameState::GameOver => print_game_over(&game),
            }
            next_tick = next_tick + Duration::from_millis(100);
        }
    }

    terminal::disable_raw_mode().unwrap();
}

fn print_game(game: &Game) {
    clear_console();

    move_cursor(0, game.height + 1);
    print!("└");
    move_cursor(0, 0);
    print!("┌");
    move_cursor(game.width + 1, 0);
    print!("┐");
    move_cursor(game.width + 1, game.height + 1);
    print!("┘");

    for x in 0..game.width {
        move_cursor(x + 1, 0);
        print!("─");
        move_cursor(x + 1, game.height + 1);
        print!("─");
    }
    for y in 0..game.height {
        move_cursor(0, y + 1);
        print!("│");
        move_cursor(game.width + 1, y + 1);
        print!("│");
    }

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
    for i in 0..game.snake.body.len() {
        let point = &game.snake.body[i];
        move_cursor(point.x + 1, point.y + 1);
        if let Some(next) = game.snake.body.get(i + 1) {
            match next.get_direction(&previous_position) {
                PointDirection::Up | PointDirection::Down => print!("│"),
                PointDirection::Right | PointDirection::Left => print!("─"),
                PointDirection::UpRight => print!("┘"),
                PointDirection::DownRight => print!("┐"),
                PointDirection::UpLeft => print!("└"),
                PointDirection::DownLeft => print!("┌"),
            }
            previous_position = point;
        } else {
            match point.get_direction(&previous_position) {
                PointDirection::Up | PointDirection::Down => print!("│"),
                PointDirection::Right | PointDirection::Left => print!("─"),
                _ => unreachable!(),
            }
        }
    }

    move_cursor(game.apple.x + 1, game.apple.y + 1);
    print!("@");

    move_cursor(0, game.height + 2);
    std::io::stdout().flush().unwrap();
}

fn print_game_over(game: &Game) {
    print_game(game);
    print_centered("Game Over!", game.width, game.height, -1);
    print_centered("r to restart", game.width, game.height, 0);
    print_centered("q to quit", game.width, game.height, 1);
    move_cursor(0, game.height + 2);
    std::io::stdout().flush().unwrap();
}

fn print_centered(value: &str, width: isize, height: isize, y_index: isize) {
    move_cursor(width / 2 - (value.len() as isize / 2), height / 2 + y_index);
    println!("{}", value);
}

fn move_cursor(x: isize, y: isize) {
    print!("\x1B[{};{}H", y + 1, x + 1);
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

impl Game {
    fn new_game(width: isize, height: isize) -> Game {
        let mut game = Game {
            state: GameState::Running,
            width: width,
            height: height,
            apple: Point { x: 0, y: 0 },
            snake: Snake {
                head_position: Point {
                    x: width / 2,
                    y: height / 2,
                },
                direction: Direction::Right,
                next_direction: None,
                body: VecDeque::from_iter(vec![
                    Point {
                        x: (width / 2) - 1,
                        y: (height / 2),
                    },
                    Point {
                        x: (width / 2) - 2,
                        y: (height / 2),
                    },
                ]),
            },
        };

        game.apple = game.find_empty_position();

        game
    }

    fn update(&mut self) {
        if self.state != GameState::Running {
            return;
        }

        let apple_eaten = self.snake.update(&self.apple);
        if apple_eaten {
            self.apple = self.find_empty_position();
        }

        if self.is_game_over() {
            self.state = GameState::GameOver;
        }
    }

    fn find_empty_position(&self) -> Point {
        let mut rng = rand::rng();
        let random_x = rng.random_range(..self.width as u64) as isize;
        let random_y = rng.random_range(..self.height as u64) as isize;

        return Point {
            x: random_x,
            y: random_y,
        };
    }

    fn is_game_over(&self) -> bool {
        if self.snake.head_position.x >= self.width
            || self.snake.head_position.x < 0
            || self.snake.head_position.y >= self.height
            || self.snake.head_position.y < 0
        {
            return true;
        }

        if self
            .snake
            .body
            .iter()
            .any(|body_part| body_part == &self.snake.head_position)
        {
            return true;
        }

        false
    }
}

struct Game {
    snake: Snake,
    apple: Point,
    state: GameState,
    width: isize,
    height: isize,
}

#[derive(PartialEq)]
enum GameState {
    Running,
    GameOver,
}

impl Snake {
    fn update(&mut self, apple: &Point) -> bool {
        self.change_direction();
        self.move_forward(apple)
    }

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

    fn set_incoming_direction(&mut self, direction: Direction) {
        match (&direction, &self.direction) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {}
            _ => self.next_direction = Some(direction),
        }
    }

    fn change_direction(&mut self) {
        if let Some(next_direction) = self.next_direction.take() {
            self.direction = next_direction;
        }
    }
}

struct Snake {
    head_position: Point,
    direction: Direction,
    next_direction: Option<Direction>,
    body: VecDeque<Point>,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum PointDirection {
    Up,
    Right,
    Down,
    Left,

    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Point {
    fn get_direction(&self, to: &Point) -> PointDirection {
        if to.y < self.y && to.x == self.x {
            return PointDirection::Up;
        } else if to.y < self.y && to.x > self.x {
            return PointDirection::UpRight;
        } else if to.y < self.y && to.x < self.x {
            return PointDirection::UpLeft;
        } else if to.y > self.y && to.x > self.x {
            return PointDirection::DownRight;
        } else if to.y > self.y && to.x < self.x {
            return PointDirection::DownLeft;
        } else if to.y > self.y && to.x == self.x {
            return PointDirection::Down;
        } else if to.x < self.x && to.y == self.y {
            return PointDirection::Left;
        } else {
            return PointDirection::Right;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: isize,
    y: isize,
}
