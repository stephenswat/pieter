use super::direction::Direction;
use super::chooser::Chooser;
use super::block::Block;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct MachineNode {
    pub block: usize, 
    pub direction: Direction, 
    pub chooser: Chooser, 
    pub flipped: bool
}

impl MachineNode {
    pub fn redirect(&self) -> MachineNode {
        if self.flipped {
            MachineNode {
                block: self.block, 
                direction: self.direction.clockwise(), 
                chooser: self.chooser, 
                flipped: false
            }
        } else {
            MachineNode { 
                block: self.block, 
                direction: self.direction, 
                chooser: self.chooser.flip(), 
                flipped: true
            }
        }
    }
}