mod utils;
use array_tool::vec::Shift;
use js_sys::{Array, Math};
// use std::{thread, time};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_usize(a: usize);
}

mod snake;

pub use snake::direction::Direction;
pub use snake::point::Point;
pub use snake::Snake;

#[wasm_bindgen]
pub struct Game {
    start: bool,
    game_over: bool,
    food: Point,
    snake: Snake,
    height: u32,
    width: u32,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        let snake = Snake::new();
        let food = Point {
            x: 50,
            y: 200,
            size: 10,
        };
        let width = 500;
        let height = 500;
        let start = false;
        let game_over = false;
        Game {
            start,
            game_over,
            food,
            snake,
            height,
            width,
        }
    }

    pub fn gen_food(&mut self) {
        let x = (Math::random() * self.width as f64) as u32;
        let y = (Math::random() * self.height as f64) as u32;

        self.food = Point { x, y, size: 10 };
    }

    pub fn food_finding(&mut self) {
        self.snake.expand();
        self.gen_food();
    }

    pub fn tick(&mut self) {
        if self.start == true {
            // thread::sleep(time::Duration::from_secs(1));
            self.snake.check_collision();
        }
        self.snake.tail.push(self.snake.position);
        match self.snake.direction {
            Direction::Stop => (),
            Direction::Up => self.snake.position.move_up(),
            Direction::Down => self.snake.position.move_down(),
            Direction::Left => self.snake.position.move_left(),
            Direction::Right => self.snake.position.move_rihgt(),
        }
        if self.snake.tail.len() >= self.snake.tail_max_len {
            self.snake.tail.shift();
        }
        if self.snake.position == self.food {
            self.food_finding();
        }
        self.passing_through();
    }

    pub fn set_directiion(&mut self, dir: u8) {
        let prev_dir = self.snake.direction.clone();
        match (dir, prev_dir) {
            (0, x) if x != Direction::Down => self.snake.change_dir(Direction::Up),
            (1, x) if x != Direction::Up => self.snake.change_dir(Direction::Down),
            (2, x) if x != Direction::Right => self.snake.change_dir(Direction::Left),
            (3..=std::u8::MAX, x) if x != Direction::Left => {
                self.snake.change_dir(Direction::Right)
            }
            (_, _) => (),
        }
    }

    pub fn passing_through(&mut self) {
        if self.snake.position.x >= self.width {
            self.snake.position.x = 0;
        } else if self.snake.position.x <= 0 {
            self.snake.position.x = self.width - 1;
        }
        if self.snake.position.y >= self.height {
            self.snake.position.y = 0;
        } else if self.snake.position.y <= 0 {
            self.snake.position.y = self.height - 1;
        }
        self.check_game_over();
    }

    fn check_game_over(&mut self) {
        if self.snake.low_score_counter >= 12 {
            self.game_over = true;
        }
    }

    pub fn start_game(&mut self) {
        self.start = true;
    }

    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn x(&self) -> u32 {
        self.snake.position.x
    }
    pub fn y(&self) -> u32 {
        self.snake.position.y
    }
    pub fn tail_x(&self) -> Array {
        let tail = self.snake.tail.clone();
        tail.into_iter().map(|i| JsValue::from(i.x)).collect()
    }
    pub fn tail_y(&self) -> Array {
        let tail = self.snake.tail.clone();
        tail.into_iter().map(|i| JsValue::from(i.y)).collect()
    }
    pub fn food_x(&self) -> u32 {
        self.food.x
    }
    pub fn food_y(&self) -> u32 {
        self.food.y
    }
    pub fn if_game_started(&self) -> bool {
        self.start
    }
    pub fn get_score(&self) -> String {
        let score = self.snake.score;
        format!("{}", score)
    }
    pub fn is_game_over(&self) -> bool {
        self.game_over
    }
    pub fn snake_cell_size(&self) -> u32 {
        self.snake.position.size
    }
    pub fn food_cell_size(&self) -> u32 {
        self.food.size
    }
}
