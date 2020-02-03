use super::direction::Direction;
use super::chooser::Chooser;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct MachineNode {
    pub block: u32, 
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