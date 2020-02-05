use super::chooser::Chooser;

#[derive(Hash, Eq, PartialEq, Clone, Debug, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn rotate(&self, r: Chooser) -> Direction {
        match (self, r) {
            (Direction::Up,    Chooser::Left)  => Direction::Left,
            (Direction::Up,    Chooser::Right) => Direction::Right,
            (Direction::Left,  Chooser::Left)  => Direction::Down,
            (Direction::Left,  Chooser::Right) => Direction::Up,
            (Direction::Down,  Chooser::Left)  => Direction::Right,
            (Direction::Down,  Chooser::Right) => Direction::Left,
            (Direction::Right, Chooser::Left)  => Direction::Up,
            (Direction::Right, Chooser::Right) => Direction::Down
        }
    }
    
    pub fn clockwise(&self) -> Direction {
        self.rotate(Chooser::Right)
    }
}