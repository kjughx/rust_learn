use block::{
    Block,
    Box,
};
use frame::new_frame;
use tetris::Tetris;

pub const BLOCK_SIZE: usize = 4;
pub const NX_PIXELS: usize = 30;
pub const NY_PIXELS: usize = 50;
pub const FPS: u16 = 1000; // in ms

pub mod block;
pub mod frame;
pub mod render;
pub mod tetris;

#[test]
fn many_blocks(){
    // for _ in 0..10{
        let block = Block::new();
        // println!("{:?}", block.boxes);
        for b in &block.boxes{
            assert!(b.pos.0 < NX_PIXELS);
            // println!("{}", block.pos.0 + b.pos.0);

        }
        // assert!(Tetris::in_bounds((29,0)));
        // let mut tetris = Tetris::new();
        // tetris.add_block();
    // }
}