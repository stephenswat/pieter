use super::direction::Direction;
use super::chooser::Chooser;
use super::block::Block;
use crate::machine::Operation;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct MachineNode {
    pub block: Block, 
    pub direction: Direction, 
    pub chooser: Chooser, 
    pub flipped: bool
}

impl MachineNode {
    pub fn redirect(&self) -> MachineNode {
        if self.flipped {
            MachineNode {
                block: self.block.clone(), 
                direction: self.direction.clockwise(), 
                chooser: self.chooser, 
                flipped: false
            }
        } else {
            MachineNode { 
                block: self.block.clone(), 
                direction: self.direction, 
                chooser: self.chooser.flip(), 
                flipped: true
            }
        }
    }
    
    pub fn is_root(&self) -> bool {
        (
            self.block.id == 0 && 
            self.direction == Direction::Right &&
            self.chooser == Chooser::Left &&
            !self.flipped
        )
    }
    
    pub fn identifier(&self) -> u64 {
        let extra = match (self.direction, self.chooser, self.flipped) {
            (Direction::Up,    Chooser::Left,  false) => 0,
            (Direction::Up,    Chooser::Left,  true ) => 1,
            (Direction::Up,    Chooser::Right, false) => 2,
            (Direction::Up,    Chooser::Right, true ) => 3,
            
            (Direction::Down,  Chooser::Left,  false) => 4,
            (Direction::Down,  Chooser::Left,  true ) => 5,
            (Direction::Down,  Chooser::Right, false) => 6,
            (Direction::Down,  Chooser::Right, true ) => 7,
            
            (Direction::Left,  Chooser::Left,  false) => 8,
            (Direction::Left,  Chooser::Left,  true ) => 9,
            (Direction::Left,  Chooser::Right, false) => 10,
            (Direction::Left,  Chooser::Right, true ) => 11,
            
            (Direction::Right, Chooser::Left,  false) => 12,
            (Direction::Right, Chooser::Left,  true ) => 13,
            (Direction::Right, Chooser::Right, false) => 14,
            (Direction::Right, Chooser::Right, true ) => 15,
        };
        
        (self.block.id as u64) * 16 + extra
    }
    
    pub fn delta(&self, rhs: &MachineNode) -> (u64, (u64, Operation)) {
        (self.identifier(), (rhs.identifier(), self.block.delta(&rhs.block)))
    }
}