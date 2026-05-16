use core::num;
use std::{
    collections::VecDeque, error::Error, io::Write, time::{Duration, Instant}
};

use crossterm::{
    event::{Event, poll, read},
    terminal,
};
use rand::RngExt;
use uuid::Uuid;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
// const YELLOW: &str = "\x1b[33m";
// const BLUE: &str = "\x1b[34m";
const RESET: &str = "\x1b[0m";


fn main() {
    const WIDTH: isize = 40;
    const HEIGHT: isize = 20;
    let snake_id: Uuid = Uuid::new_v4();

    terminal::enable_raw_mode().unwrap();
    let mut game = Game::new_game(WIDTH, HEIGHT, snake_id);
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
                            game = Game::new_game(WIDTH, HEIGHT, snake_id)
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

    print_border(game);
    print_snake(&game.snake);
    print_apple(game);

    print!("{RESET}");
    move_cursor(0, (game.height + 2) as i32);
    std::io::stdout().flush().unwrap();
}

fn print_apple(game: &Game) {
    print!("{GREEN}");
    move_cursor(game.apple.x + 1, game.apple.y + 1);
    print!("@");
}

fn print_border(game: &Game) {
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
}

fn print_snake(snake: &Snake) {
    move_cursor(
        snake.head_position().x + 1,
        snake.head_position().y + 1,
    );
    print!("{RED}");
    match snake.direction() {
        Direction::Up => print!("⬆"),
        Direction::Right => print!("➡"),
        Direction::Down => print!("⬇"),
        Direction::Left => print!("⬅"),
    }

    let mut next_direction = snake.direction();
    for i in 1..snake.body.len() {
        let point = &snake.body[i].point;
        let current_direction = &snake.body[i].direction;
        move_cursor(point.x + 1, point.y + 1);
        match (current_direction, next_direction) {
            (Direction::Up, Direction::Up) => print!("│"),
            (Direction::Up, Direction::Right) => print!("┌"),
            (Direction::Up, Direction::Left) => print!("┐"),
            (Direction::Right, Direction::Up) => print!("┘"),
            (Direction::Right, Direction::Right) => print!("─"),
            (Direction::Right, Direction::Down) => print!("┐"),
            (Direction::Down, Direction::Right) => print!("└"),
            (Direction::Down, Direction::Down) => print!("│"),
            (Direction::Down, Direction::Left) => print!("┘"),
            (Direction::Left, Direction::Up) => print!("└"),
            (Direction::Left, Direction::Down) => print!("┌"),
            (Direction::Left, Direction::Left) => print!("─"),
            _ => unreachable!(),
        }

        next_direction = current_direction;
    }
}

fn print_game_over(game: &Game) {
    print_game(game);
    print_centered("Game Over!", game.width, game.height, -1);
    print_centered("r to restart", game.width, game.height, 0);
    print_centered("q to quit", game.width, game.height, 1);
    move_cursor(0, game.height + 2);
    std::io::stdout().flush().unwrap();
}

fn print_centered(value: &str, width: i32, height: i32, y_index: i32) {
    move_cursor(width / 2 - (value.len() as isize / 2) as i32, height / 2 + y_index);
    println!("{}", value);
}

fn move_cursor(x: i32, y: i32) {
    print!("\x1B[{};{}H", y + 1, x + 1);
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}

impl Game {
    fn new_game(width: i32, height: i32, snake_id: Uuid) -> Game {
        let mut game = Game {
            state: GameState::Running,
            width: width,
            height: height,
            apple: Point { x: 0, y: 0 },
            snake: Snake {
                    id: snake_id,
                    next_direction: None,
                    body: VecDeque::from_iter(vec![
                        BodyPart {
                            point: Point {
                                x: (width / 2),
                                y: (height / 2),
                            },
                            direction: Direction::Right,
                        },
                        BodyPart {
                            point: Point {
                                x: (width / 2) - 1,
                                y: (height / 2),
                            },
                            direction: Direction::Right,
                        },
                        BodyPart {
                            point: Point {
                                x: (width / 2) - 2,
                                y: (height / 2),
                            },
                            direction: Direction::Right,
                        },
                    ]),
                },
            snakes: vec![]
        };

        game.apple = game.find_empty_position();

        game
    }

    fn to_byte_array(&self) -> Vec<u8> {
        let mut game_array = vec![0; 1024];

        // game_array[]

        vec![]
    }

    fn from_byte_array(input: Vec<u8>) -> Result<Game, Box<dyn Error>> {
        let state = match input[0] {
            0 => GameState::Running,
            1 => GameState::GameOver,
            invalid_state => panic!("{} not a valid state", invalid_state)
        };

        let width = i32::from_le_bytes(input[1..5].try_into().unwrap());
        let height = i32::from_le_bytes(input[5..9].try_into().unwrap());

        let apple_x = i32::from_le_bytes(input[9..13].try_into().unwrap());
        let apple_y = i32::from_le_bytes(input[13..17].try_into().unwrap());

        let number_of_snakes = i32::from_le_bytes(input[17..21].try_into().unwrap());

        let snakes: Vec<Snake> = vec![];
        for i in 0..number_of_snakes {

        }


        Err("not a valid byte array".into())
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
        let random_x = rng.random_range(..self.width as u64) as i32;
        let random_y = rng.random_range(..self.height as u64) as i32;

        return Point {
            x: random_x,
            y: random_y,
        };
    }

    fn is_game_over(&self) -> bool {
        if self.snake.head_position().x >= self.width
            || self.snake.head_position().x < 0
            || self.snake.head_position().y >= self.height
            || self.snake.head_position().y < 0
        {
            return true;
        }

        if self
            .snake
            .body
            .iter()
            .skip(1)
            .any(|body_part| &body_part.point == self.snake.head_position())
        {
            return true;
        }

        false
    }
}

struct Game {
    snake: Snake,
    snakes: Vec<Snake>,
    apple: Point,
    state: GameState,
    width: i32,
    height: i32,
}

#[derive(PartialEq)]
#[repr(u8)]
enum GameState {
    Running = 0,
    GameOver = 1,
}

impl GameState {
    fn from_byte(state: u8) -> Result<GameState, Box<dyn Error>> {
        match state {
            0 => Ok(GameState::Running),
            1 => Ok(GameState::GameOver),
            _ => Err("game state can only be 0 or 1".into())
        }
    }
}

impl Snake {
    fn update(&mut self, apple: &Point) -> bool {
        self.move_forward(apple)
    }

    fn move_forward(&mut self, apple: &Point) -> bool {
        let direction = self
            .next_direction
            .take()
            .unwrap_or_else(|| self.direction().clone());

        let point = match direction {
            Direction::Up => Point {
                x: self.head_position().x,
                y: self.head_position().y - 1,
            },
            Direction::Right => Point {
                x: self.head_position().x + 1,
                y: self.head_position().y,
            },
            Direction::Down => Point {
                x: self.head_position().x,
                y: self.head_position().y + 1,
            },
            Direction::Left => Point {
                x: self.head_position().x - 1,
                y: self.head_position().y,
            },
        };

        self.body.push_front(BodyPart { point, direction });

        if &self.head_position() == &apple {
            return true;
        }

        self.body.pop_back();
        false
    }

    fn set_incoming_direction(&mut self, direction: Direction) {
        match (&direction, &self.direction()) {
            (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left) => {}
            _ => self.next_direction = Some(direction),
        }
    }

    fn head_position(&self) -> &Point {
        &self.body[0].point
    }

    fn direction(&self) -> &Direction {
        &self.body[0].direction
    }
}

struct Snake {
    id: Uuid,
    next_direction: Option<Direction>,
    body: VecDeque<BodyPart>,
}

struct BodyPart {
    point: Point,
    direction: Direction,
}

#[derive(Clone)]
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
    x: i32,
    y: i32,
}
