#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Chooser {
    Left,
    Right
}

impl Chooser {
    pub fn flip(&self) -> Chooser {
        match self {
            Chooser::Left  => Chooser::Right,
            Chooser::Right => Chooser::Left
        }
    }
}