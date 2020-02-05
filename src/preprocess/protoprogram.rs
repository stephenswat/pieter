use std::collections::{HashMap, HashSet};

use crate::machine::{Operation, Program};

use super::chooser::Chooser;
use super::colour::Colour;
use super::block::Block;
use super::machinenode::MachineNode;
use super::direction::Direction;

#[derive(Clone)]
pub struct ProtoProgram(pub HashMap<MachineNode, MachineNode>);

impl ProtoProgram {
    pub fn to_program(&self) -> Program {
        Program::new(0, HashMap::new())
    }
    
    pub fn optimize(&self) -> ProtoProgram {
        let new = self.opt_all();

        if new.0.len() == self.0.len() {
            (*self).clone()
        } else {
            new.optimize()
        }
    }
    
    fn opt_all(&self) -> ProtoProgram {
        self.opt_remove_noops().opt_remove_unreachable()
    }
    
    fn opt_remove_unreachable(&self) -> ProtoProgram {
        let mut ret = HashMap::new();
        
        for (k, v) in self.0.iter() {
            let mut reachable = k.is_root();
            
            for (_, t) in self.0.iter() {
                if k == t {
                    reachable = true;
                    break;
                }
            }
            
            if reachable {
                ret.insert(k.clone(), v.clone());
            }
        }
        
        ProtoProgram(ret)
    }
    
    fn opt_remove_noops(&self) -> ProtoProgram {
        let mut ret = HashMap::new();
        
        for (k, v) in self.0.iter() {
            if k.block == v.block {
                ret.insert(k.clone(), self.0.get(v).unwrap().clone());
            } else {
                ret.insert(k.clone(), v.clone());
            }
        }
        
        ProtoProgram(ret)
    }
}