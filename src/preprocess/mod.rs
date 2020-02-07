mod block;
mod colour;
mod chooser;
mod blockimage;
mod machinenode;
mod protoprogram;
mod direction;

use image::{ImageBuffer, Rgb};

use crate::preprocess::blockimage::BlockImage;
use crate::machine::Program;

pub fn read_program(i: ImageBuffer<Rgb<u8>, Vec<u8>>) -> Program {
    let proto = BlockImage::from_imagebuffer(i).to_protoprogram();
    
    for (key, value) in &proto.0 {
        println!("{:?}: {:?}", key, value);
    }
    
    println!("Number of elements: {}", proto.0.len());
    let q = proto.optimize();
    println!("Number of elements: {}", q.0.len());
    
    for (key, value) in &q.0 {
        println!("{:?}: {:?}", key, value);
    }
    
    let p = proto.to_program();

    for (key, value) in &p.transitions {
        println!("{:?}: {:?}", key, value);
    }
    
    p
}