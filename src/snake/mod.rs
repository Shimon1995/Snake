use array_tool::vec::Shift;

pub mod point;
pub use point::Point;

pub mod direction;
pub use direction::Direction;

pub struct Snake {
    pub score: i32,
    pub direction: Direction,
    pub position: Point,
    pub tail: Vec<Point>,
    pub tail_max_len: usize,
    pub low_score_counter: u8,
    head_zone: u32,
}

impl Snake {
    pub fn new() -> Snake {
        let position = Point {
            x: 260,
            y: 260,
            size: 10,
        };
        let tail = Vec::new();
        let score = 0;
        Snake {
            score,
            direction: Direction::Stop,
            position,
            tail,
            tail_max_len: 30,
            low_score_counter: 0,
            head_zone: 30,
        }
    }
    pub fn expand(&mut self) {
        self.score += 50;
        self.tail_max_len += 10;
    }
    pub fn check_collision(&mut self) {
        let len = self.tail.len();
        if len != 0 {
            for i in 0..len / 2 {
                if self.position == self.tail[i] {
                    if self.tail.len() >= 20 {
                        self.low_score_counter += 1;
                        self.score -= 50;
                        self.tail_max_len -= 10;
                        for _ in 0..10 {
                            self.tail.shift();
                        }
                    }
                }
            }
        }
    }
    pub fn change_dir(&mut self, direction: Direction) {
        self.direction = direction;
    }
}
