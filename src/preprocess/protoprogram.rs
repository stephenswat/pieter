use std::collections::HashMap;

use crate::machine::Program;

use super::machinenode::MachineNode;

#[derive(Clone)]
pub struct ProtoProgram(pub HashMap<MachineNode, MachineNode>);

impl ProtoProgram {
    pub fn to_program(&self) -> Program {
        let mut ret = HashMap::new();
        
        for (k, v) in &self.0 {
            let (nk, nv) = k.delta(&v);
            ret.insert(nk, nv);
        }
        
        Program::new(0, ret)
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