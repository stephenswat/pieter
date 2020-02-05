use std::collections::{HashMap, HashSet};

use image::{ImageBuffer, Rgb};

use crate::machine::{Operation, Program};

use super::chooser::Chooser;
use super::colour::Colour;
use super::machinenode::MachineNode;
use super::direction::Direction;

type ProtoProgram = HashMap<MachineNode, MachineNode>;

#[derive(Clone, Debug)]
struct Block {
    id: usize,
    colour: Colour,
    pixels: Vec<(u32, u32)>
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
}

fn parse_block_change(a: Block, b:Block) -> Operation {
    match (a, b) {
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

fn blocks_to_proto(i: &PietImage) -> ProtoProgram {
    let mut ret = HashMap::new();

    for Block { id, pixels: codels, .. } in i.blocks.iter() {
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
                        (_, _, Direction::Left)  => Some((cx + 1, cy)),
                        (_, _, Direction::Right) => Some((cx, cy + 1))
                    };
                    
                    let new_block = new_coordinate.and_then(|p| i.block_by_pixel(&p));
                    
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
    
    ret
}

fn proto_to_program(p: &ProtoProgram) -> Program {
    Program::new(0, HashMap::new())
}

pub fn read_program(i: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Program {
    let blocks = identify_blocks(i);
    
    for b in blocks.blocks.iter() {
        println!("{:?}", b);
    }
    
    let proto = blocks_to_proto(&blocks);
    
    for (key, value) in &proto {
        println!("{:?}: {:?}", key, value);
    }

    proto_to_program(&proto)
}