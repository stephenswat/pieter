use std::collections::{HashMap, HashSet};

use image::{ImageBuffer, Rgb};

use crate::machine::{Operation, Program};

use super::chooser::Chooser;
use super::colour::Colour;
use super::block::Block;
use super::machinenode::MachineNode;
use super::direction::Direction;

#[derive(Clone)]
struct ProtoProgram(HashMap<MachineNode, MachineNode>);

impl ProtoProgram {
    fn to_program(&self) -> Program {
        Program::new(0, HashMap::new())
    }
    
    fn optimize(&self) -> ProtoProgram {
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
            let mut reachable = false;
            
            for (_, t) in self.0.iter() {
                if k == t {
                    reachable = true;
                    break;
                }
            }
            
            if reachable {
                ret.insert(*k, *v);
            }
        }
        
        ProtoProgram(ret)
    }
    
    fn opt_remove_noops(&self) -> ProtoProgram {
        let mut ret = HashMap::new();
        
        for (k, v) in self.0.iter() {
            if k.block == v.block {
                ret.insert(*k, *self.0.get(v).unwrap());
            } else {
                ret.insert(*k, *v);
            }
        }
        
        ProtoProgram(ret)
    }
}

struct PietImage {
    blocks: Vec<Block>,
    pixel_to_index: HashMap<(u32, u32), usize>
}

impl PietImage {
    fn new() -> PietImage {
        PietImage {
            blocks: Vec::new(),
            pixel_to_index: HashMap::new()
        }
    }
    
    fn insert_block(&mut self, p: &Vec<(u32, u32)>, c: &Colour) {
        let pos = self.blocks.len();
                
        self.blocks.push(Block {
            id: self.blocks.len(),
            colour: c.clone(),
            pixels: p.clone()
        });
        
        for q in p.iter() {
            self.pixel_to_index.insert(*q, pos);
        }
    }
    
    fn contains_pixel(&self, p: &(u32, u32)) -> bool {
        self.pixel_to_index.contains_key(&p)
    }
    
    fn block_by_pixel(&self, p: &(u32, u32)) -> Option<&Block> {
        self.pixel_to_index.get(p).map(|i| &self.blocks[*i])
    }
    
    fn to_protoprogram(&self) -> ProtoProgram {
        let mut ret = HashMap::new();

        for Block { id, pixels: codels, .. } in self.blocks.iter() {
            for direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
                let filter1 = maximal_codels(codels, *direction);

                for chooser in &[Chooser::Left, Chooser::Right] {
                    let filter2 = maximal_codels(&filter1, direction.rotate(*chooser));
                    assert_eq!(filter2.len(), 1);
                    let (cx, cy) = filter2[0];
                    
                    for flipped in &[true, false] {
                        let state = MachineNode {
                            block: *id,
                            direction: *direction,
                            chooser: *chooser,
                            flipped: *flipped
                        };
                        
                        let new_coordinate = match (cx, cy, direction) {
                            (_, 0, Direction::Up)    => None,
                            (_, _, Direction::Up)    => Some((cx, cy - 1)),
                            (_, _, Direction::Down)  => Some((cx, cy + 1)),
                            (0, _, Direction::Left)  => None,
                            (_, _, Direction::Left)  => Some((cx - 1, cy)),
                            (_, _, Direction::Right) => Some((cx + 1, cy))
                        };
                        
                        let new_block = new_coordinate.and_then(|p| self.block_by_pixel(&p));
                        
                        let new_state = match new_block {
                            Some(Block { id: new_id, .. }) => MachineNode {
                                block: *new_id,
                                direction: state.direction,
                                chooser: state.chooser,
                                flipped: false
                            },
                            None => state.redirect()
                        };
                        
                        ret.insert(state, new_state);
                    }
                }
            }
        }
        
        ProtoProgram(ret)
    }
}

fn identify_blocks(i: ImageBuffer<Rgb<u8>, Vec<u8>>) -> PietImage {
    let mut ret = PietImage::new();

    for (x, y, Rgb([r, g, b])) in i.enumerate_pixels() {
        if ret.contains_pixel(&(x, y)) {
            continue;
        }
        
        let mut stack = vec![(x, y)];
        let mut pixels = Vec::new();
        let mut visited = HashSet::new();
        let op = i.get_pixel(x, y);
        let colour = Colour::from_rgb(op);
        
        while let Some(p) = stack.pop() {
            let (cx, cy) = p;
            
            if visited.contains(&p) {
                continue
            }
            
            pixels.push(p);
            visited.insert(p);

            if cx > 0 && op == i.get_pixel(cx - 1, cy) {
                stack.push((cx - 1, cy))
            }
            
            if cx < i.width() - 1 && op == i.get_pixel(cx + 1, cy) {
                stack.push((cx + 1, cy))
            }
            
            if cy > 0 && op == i.get_pixel(cx, cy - 1) {
                stack.push((cx, cy - 1))
            }
            
            if cy < i.height() - 1 && op == i.get_pixel(cx, cy + 1) {
                stack.push((cx, cy + 1))
            }
        }
        
        if pixels.len() > 0 {
            ret.insert_block(&pixels, &colour)
        }
    }
    
    ret
}

fn maximal_codels(i: &Vec<(u32, u32)>, d: Direction) -> Vec<(u32, u32)> {
    match d {
        Direction::Up    => { 
            let m = i.iter().map(|(_, y)| y).min().unwrap();
            i.iter().filter(|(_, y)| y == m).copied().collect()
        },
        Direction::Down  => { 
            let m = i.iter().map(|(_, y)| y).max().unwrap();
            i.iter().filter(|(_, y)| y == m).copied().collect()
        },
        Direction::Left  => { 
            let m = i.iter().map(|(x, _)| x).min().unwrap();
            i.iter().filter(|(x, _)| x == m).copied().collect()
        },
        Direction::Right => { 
            let m = i.iter().map(|(x, _)| x).max().unwrap();
            i.iter().filter(|(x, _)| x == m).copied().collect()
        }
    }
}

pub fn read_program(i: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Program {
    let proto = identify_blocks(i).to_protoprogram();
    
    for (key, value) in &proto.0 {
        println!("{:?}: {:?}", key, value);
    }
    
    let q = proto.optimize();
    println!("Number of elements: {}", q.0.len());
    
    
    for (key, value) in &q.0 {
        println!("{:?}: {:?}", key, value);
    }
    
    proto.to_program()
}