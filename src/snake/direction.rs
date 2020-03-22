#[derive(Clone)]
pub enum Direction {
    Stop = 0,
    Up = 1,
    Down = 2,
    Left = 3,
    Right = 4,
}

impl PartialEq<Direction> for Direction {
    fn eq(&self, _dir: &Direction) -> bool {
        let left = self.clone();
        let right = _dir.clone();
        left as u8 == right as u8
    }
    fn ne(&self, _dir: &Direction) -> bool {
        let left = self.clone();
        let right = _dir.clone();
        left as u8 != right as u8
    }
}
