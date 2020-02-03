use std::collections::{HashMap};

use image::{ImageBuffer, Rgb, Rgba};

use super::machine::{Operation, Program};

type ProtoProgram = HashMap<MachineNode, MachineNode>;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn rotate(&self, r: Chooser) -> Direction {
        match (self, r) {
            (Direction::Up,    Chooser::Left)  => Direction::Left,
            (Direction::Up,    Chooser::Right) => Direction::Right,
            (Direction::Left,  Chooser::Left)  => Direction::Down,
            (Direction::Left,  Chooser::Right) => Direction::Up,
            (Direction::Down,  Chooser::Left)  => Direction::Right,
            (Direction::Down,  Chooser::Right) => Direction::Left,
            (Direction::Right, Chooser::Left)  => Direction::Up,
            (Direction::Right, Chooser::Right) => Direction::Down
        }
    }
    
    pub fn clockwise(&self) -> Direction {
        self.rotate(Chooser::Right)
    }
}

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

#[derive(Debug)]
enum Colour {
    Colour { hue: u8, lightness: u8 },
    Black,
    White,
    Other
}

struct Block {
    colour: Colour,
    size: u32
}

fn parse_colour(Rgb(c): Rgb<u32>) -> Colour {
    match c {
        [255, 192, 192] => Colour::Colour { hue: 0, lightness: 0 },
        [255,   0,   0] => Colour::Colour { hue: 0, lightness: 1 },
        [192,   0,   0] => Colour::Colour { hue: 0, lightness: 2 },

        [255, 255, 192] => Colour::Colour { hue: 1, lightness: 0 },
        [255, 255,   0] => Colour::Colour { hue: 1, lightness: 1 },
        [192, 192,   0] => Colour::Colour { hue: 1, lightness: 2 },

        [192, 255, 192] => Colour::Colour { hue: 2, lightness: 0 },
        [  0, 255,   0] => Colour::Colour { hue: 2, lightness: 1 },
        [  0, 192,   0] => Colour::Colour { hue: 2, lightness: 2 },

        [192, 255, 255] => Colour::Colour { hue: 3, lightness: 0 },
        [  0, 255, 255] => Colour::Colour { hue: 3, lightness: 1 },
        [  0, 192, 192] => Colour::Colour { hue: 3, lightness: 2 },

        [192, 192, 255] => Colour::Colour { hue: 4, lightness: 0 },
        [  0,   0, 255] => Colour::Colour { hue: 4, lightness: 1 },
        [  0,   0, 192] => Colour::Colour { hue: 4, lightness: 2 },

        [255, 192, 255] => Colour::Colour { hue: 5, lightness: 0 },
        [255,   0, 255] => Colour::Colour { hue: 5, lightness: 1 },
        [192,   0, 192] => Colour::Colour { hue: 5, lightness: 2 },

        [  0,   0,   0] => Colour::Black,
        [255, 255, 255] => Colour::White,
        _               => Colour::Other
    }
}

fn parse_block_change(a: Block, b:Block) -> Operation {
    match (a, b) {
        (
            Block { colour: Colour::Colour { hue: h1, lightness: l1 }, size: s }, 
            Block { colour: Colour::Colour { hue: h2, lightness: l2 }, size: _ }
        ) => {
            let dh = (h2 - h1) % 6;
            let dl = (l2 - l1) % 3;

            match (dh, dl) {
                (0, 0) => Operation::NoOp,
                (0, 1) => Operation::Push(s as i64),
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

        (Block { colour: Colour::Other, size: _ }, _) | 
        (_, Block { colour: Colour::Other, size: _ }) => Operation::Error(
            "Entering a block with non-standard colour is disallowed!"
        ),

        (Block { colour: Colour::White, size: _ }, _) | 
        (_, Block { colour: Colour::White, size: _ }) => Operation::NoOp,

        (Block { colour: Colour::Black, size: _ }, _) | 
        (_, Block { colour: Colour::Black, size: _ }) => panic!(
            "Somehow, we entered a black block. That is invalid."
        )
    }
}

/*
 * Our first objective is to group codels together into blocks. We go from an
 * image in which the pixels are RGB values to an image in which each pixel is
 * an integer identifier, indicating to which block the codel belongs. We make
 * no guarantees about the values of these identifiers, only that each block
 * has a unique identifier, and that all codels in the same block have the same
 * identifier. We use Rgba-type pixels to represent these integer identifiers,
 * since we can use them to hold 4 values (R, G, B, block ID).
 */
fn identify_blocks(i: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u32>, Vec<u32>> {
    let zero = Rgba([0, 0, 0, 0]);

    let mut block_id = 0;
    let mut buf = ImageBuffer::from_pixel(i.width(), i.height(), zero);

    for (x, y, Rgb([r, g, b])) in i.enumerate_pixels() {
        let mut stack = vec![(x, y)];
        let op = i.get_pixel(x, y);
        
        block_id += 1;
        
        while let Some((cx, cy)) = stack.pop() {
            if *buf.get_pixel(cx, cy) != zero {
                continue;
            }

            *buf.get_pixel_mut(cx, cy) = Rgba([(*r) as u32, (*g) as u32, (*b) as u32, block_id]);

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
    }

    buf
}

fn block_colours(i: &ImageBuffer<Rgba<u32>, Vec<u32>>) -> HashMap<u32, Colour> {
    let mut map = HashMap::new();

    for (_, _, Rgba([r, g, b, id])) in i.enumerate_pixels() {
        map.insert(*id, parse_colour(Rgb([*r, *g, *b])));
    }

    map
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

fn block_transitions(i: &ImageBuffer<Rgba<u32>, Vec<u32>>) -> ProtoProgram {
    let mut blocks = HashMap::<u32, Vec<(u32, u32)>>::new();
    let mut ret = HashMap::new();

    for (x, y, Rgba([_, _, _, id])) in i.enumerate_pixels() {
        if let Some(v) = blocks.get_mut(id) {
            v.push((x, y));
        } else {
            blocks.insert(*id, vec![(x, y)]);
        }
    }

    for (id, codels) in blocks.iter() {
        println!("{:?} - {:?}", id, codels);

        for direction in &[Direction::Up, Direction::Down, Direction::Left, Direction::Right] {
            let filter1 = maximal_codels(codels, *direction);

            for chooser in &[Chooser::Left, Chooser::Right] {
                let filter2 = maximal_codels(&filter1, direction.rotate(*chooser));
                
                for flipped in &[true, false] {
                    assert_eq!(filter2.len(), 1);
                    let (cx, cy) = filter2[0];
                    let state = MachineNode {
                        block: *id,
                        direction: *direction,
                        chooser: *chooser,
                        flipped: *flipped
                    };
                    
                    let new_state = match direction {
                        // TODO: This code is in dire need of some deduplication!
                        Direction::Up => {
                            if cy == 0 {
                                state.redirect()
                            } else {
                                let Rgba([_, _, _, new_id]) = i.get_pixel(cx, cy - 1);
                                MachineNode {
                                    block: *new_id,
                                    direction: state.direction,
                                    chooser: state.chooser,
                                    flipped: false
                                }
                            }
                        }
                        
                        Direction::Down => {
                            if cy == i.height() - 1 {
                                state.redirect()
                            } else {
                                let Rgba([_, _, _, new_id]) = i.get_pixel(cx, cy + 1);
                                MachineNode {
                                    block: *new_id,
                                    direction: state.direction,
                                    chooser: state.chooser,
                                    flipped: false
                                }
                            }
                        }
                        
                        Direction::Left => {
                            if cx == 0 {
                                state.redirect()
                            } else {
                                let Rgba([_, _, _, new_id]) = i.get_pixel(cx - 1, cy);
                                MachineNode {
                                    block: *new_id,
                                    direction: state.direction,
                                    chooser: state.chooser,
                                    flipped: false
                                }
                            }
                        }
                        
                        Direction::Right => {
                            if cx == i.width() - 1 {
                                state.redirect()
                            } else {
                                let Rgba([_, _, _, new_id]) = i.get_pixel(cx + 1, cy);
                                MachineNode {
                                    block: *new_id,
                                    direction: state.direction,
                                    chooser: state.chooser,
                                    flipped: false
                                }
                            }
                        }
                    };
                    
                    ret.insert(state, new_state);
                }
            }
        }
    }
    
    ret
}

fn resolve_black_blocks(cm: &HashMap<u32, Colour>, tm: &ProtoProgram) -> ProtoProgram {
    let mut ret = HashMap::new();
    
    // for ((oi, od, oc, of), (ni, nd, nc, nf)) in tm.iter() {
    //     match cm.get(i) {
    //         Some(Colour::Black) => ret.insert((oi, od, oc, of), ())
    //     }
    // }
    
    ret
}

fn read_protoprogram(i: ImageBuffer<Rgb<u8>, Vec<u8>>) -> ProtoProgram {
    let annotated = identify_blocks(i);
    let colour_map = block_colours(&annotated);
    let trans_map = block_transitions(&annotated);
    
    trans_map
}

fn proto_to_program(p: ProtoProgram) -> Program {
    Program::new(0, HashMap::new())
}

pub fn read_program(i: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Program {
    let proto = read_protoprogram(i);
    
    for (key, value) in &proto {
        println!("{:?}: {:?}", key, value);
    }

    proto_to_program(proto)
}
