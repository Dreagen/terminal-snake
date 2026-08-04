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
    const WIDTH: i32 = 40;
    const HEIGHT: i32 = 20;
    let snake_id = Uuid::new_v4();

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
            snakes: vec![],
            player_snake_id: snake_id
        };

        game.apple = game.find_empty_position();

        game
    }

    fn to_byte_array(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.push(self.state as u8);

        bytes.extend(self.width.to_le_bytes());
        bytes.extend(self.height.to_le_bytes());

        bytes.extend(self.apple.to_byte_array());

        bytes.extend(self.player_snake_id.to_bytes_le());

        let snakes_count: i32 = self.snakes.len().try_into().unwrap();
        bytes.extend(snakes_count.to_le_bytes());

        for snake in &self.snakes {
            bytes.extend(snake.to_byte_array());
        }

        bytes
    }

    fn from_byte_array(input: Vec<u8>) -> Game {
        let state = match input[0] {
            0 => GameState::Running,
            1 => GameState::GameOver,
            invalid_state => panic!("{} not a valid state", invalid_state)
        };

        let width = i32::from_le_bytes(input[1..5].try_into().unwrap());
        let height = i32::from_le_bytes(input[5..9].try_into().unwrap());

        let apple = Point::from_byte_array(input[9..17].to_vec());

        let player_snake_id = Uuid::from_bytes_le(input[17..33].try_into().unwrap());
        let number_of_snakes = i32::from_le_bytes(input[33..37].try_into().unwrap());

        let mut snakes: Vec<Snake> = vec![];
        let mut offset: usize = 0;
        for _ in 0..number_of_snakes {
            let snake_length = i32::from_le_bytes(input[(37+offset)..(41+offset)].try_into().unwrap());
            let snake_bytes = input[(41 + offset) .. (41 + offset + (snake_length as usize))].to_vec();

            snakes.push(Snake::from_byte_array(snake_bytes));

            offset += (snake_length + 4) as usize; // length of snake in bytes + the byte that says how long
                                        // the snake is
        }

        let snake = snakes.into_iter().find(|s| s.id == player_snake_id)
            .expect("player snake id not present in game byte array");

        Game {
            state,
            width,
            height,
            apple,
            player_snake_id,
            snake,
            snakes: vec![]
        }
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
    player_snake_id: Uuid,
    apple: Point,
    state: GameState,
    width: i32,
    height: i32,
}

#[derive(PartialEq, Clone, Copy)]
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

struct Snake {
    id: Uuid,
    next_direction: Option<Direction>,
    body: VecDeque<BodyPart>,
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

    fn from_byte_array(input: Vec<u8>) -> Snake {
        let snake_id = Uuid::from_bytes_le(input[..16].try_into().expect("failed to get snake id from bytes"));
        let next_direction = match input[5] {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            invalid_state => panic!("{} not a valid state", invalid_state)
        };

        let body_part_count = i32::from_le_bytes(input[6..10].try_into().unwrap());

        let mut body_parts: VecDeque<BodyPart> = VecDeque::new();
        let mut offset: usize = 0;
        for _ in 0..body_part_count {
            body_parts.push_back(BodyPart::from_byte_array(input[offset..10].to_vec()));
            offset += 10;
        }

        Snake {
            id: snake_id,
            next_direction: Option::Some(next_direction),
            body: body_parts,
        }
    }

    fn to_byte_array(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(&self.id.to_bytes_le());
        bytes.push(*self.direction() as u8);
        let body_part_count: &i32 = &self.body.len().try_into().expect("body parts didn't fit into i32");
        bytes.extend(body_part_count.to_le_bytes());
        for part in &self.body {
            bytes.extend(part.to_byte_array());
        }

        bytes
    }
}

struct BodyPart {
    point: Point,
    direction: Direction,
}

impl BodyPart {
    fn to_byte_array(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(self.point.to_byte_array());
        bytes.push(self.direction as u8);

        bytes
    }

    fn from_byte_array(input: Vec<u8>) -> BodyPart {
        let point = Point::from_byte_array(input[0..8].to_vec());
        let direction = match input[8] {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            invalid_state => panic!("{} not a valid state", invalid_state)
        };

        BodyPart {
            point,
            direction
        }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Point {
    fn from_byte_array(input: Vec<u8>) -> Point {
        let x = i32::from_le_bytes(input[0..4].try_into().unwrap());
        let y = i32::from_le_bytes(input[0..4].try_into().unwrap());

        Point {
            x,
            y
        }
    }

    fn to_byte_array(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];

        bytes.extend(self.x.to_le_bytes());
        bytes.extend(self.y.to_le_bytes());

        bytes
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: i32,
    y: i32,
}
