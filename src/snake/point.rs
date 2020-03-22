use std::cmp;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub size: u32,
}

impl Point {
    pub fn move_up(&mut self) {
        self.y -= 1;
    }
    pub fn move_down(&mut self) {
        self.y += 1;
    }
    pub fn move_rihgt(&mut self) {
        self.x += 1;
    }
    pub fn move_left(&mut self) {
        self.x -= 1;
    }
}

impl cmp::PartialEq<Point> for Point {
    fn eq(&self, _point: &Point) -> bool {
        (self.x + 5 >= _point.x && self.x + 5 <= _point.x + 10)
            && (self.y + 5 >= _point.y && self.y + 5 <= _point.y + 10)
    }
}
