use super::direction::Direction;
use super::chooser::Chooser;
use super::block::Block;

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
}