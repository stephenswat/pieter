use super::colour::Colour;
use crate::machine::Operation;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Block {
    pub id: usize,
    pub colour: Colour,
    pub pixels: Vec<(u32, u32)>
}

impl Block {
    fn delta(&self, rhs: &Block) -> Operation {
        match (self, rhs) {
            (
                Block { colour: Colour::Colour { hue: h1, lightness: l1 }, pixels: p, .. }, 
                Block { colour: Colour::Colour { hue: h2, lightness: l2 }, .. }
            ) => {
                let dh = (h2 - h1) % 6;
                let dl = (l2 - l1) % 3;

                match (dh, dl) {
                    (0, 0) => Operation::NoOp,
                    (0, 1) => Operation::Push(p.len() as i64),
                    (0, 2) => Operation::Pop,

                    (1, 0) => Operation::Add,
                    (1, 1) => Operation::Subtract,
                    (1, 2) => Operation::Multiply,
                    
                    (2, 0) => Operation::Divide,
                    (2, 1) => Operation::Mod,
                    (2, 2) => Operation::Not,
                    
                    (3, 0) => Operation::Greater,
                    (3, 1) => Operation::Pointer,
                    (3, 2) => Operation::Switch,
                    
                    (4, 0) => Operation::Duplicate,
                    (4, 1) => Operation::Roll,
                    (4, 2) => Operation::InNum,
                    
                    (5, 0) => Operation::InChar,
                    (5, 1) => Operation::OutNum,
                    (5, 2) => Operation::OutChar,

                    _      => panic!(
                        "Somehow, the difference between two the colour blocks \
                        was more than 5 in hue, or more than 2 in lightness."
                    )
                }
            }

            (Block { colour: Colour::Other, .. }, _) | 
            (_, Block { colour: Colour::Other, .. }) => Operation::Error(
                "Entering a block with non-standard colour is disallowed!"
            ),

            (Block { colour: Colour::White, .. }, _) | 
            (_, Block { colour: Colour::White, .. }) => Operation::NoOp,

            (Block { colour: Colour::Black, .. }, _) | 
            (_, Block { colour: Colour::Black, .. }) => panic!(
                "Somehow, we entered a black block. That is invalid."
            )
        }
    }
}