use crossterm::style::Color;
use rand::{Rng, thread_rng};
use crate::{
    BLOCK_SIZE, NX_PIXELS, tetris::Tetris,
};

#[derive(Clone, Copy, Debug)]
pub struct Box {
    pub pos: (usize, usize),
    pub color: Color,
}
impl Box{
    pub fn new((i,j): (usize, usize), color: Color) -> Box {
        Box { pos: (i,j), 
              color,
        }
    }
}
#[derive(Debug)]
pub struct Block {
    pub boxes: Vec<Box>,
    pub color: Color,
}

impl Block {
    pub fn new() -> Block {
        let mut rng = thread_rng();
        
        let color = Color::AnsiValue(rng.gen_range(1..=4));
        let mut block = Block{
            boxes: vec![Box::new((rng.gen_range(0..NX_PIXELS),0), color)],
            color
        };

        let mut point = block.boxes[0].pos;
        let directions = [(1,0), (0,1)];
        for _ in 1..BLOCK_SIZE{
            let mut direction = directions[rng.gen_range(0..2)];

            while !(Tetris::in_bounds((point.0 + direction.0, point.1 + direction.1))){
                direction = directions[rng.gen_range(0..2)];
            }

            point.0 += direction.0;
            point.1 += direction.1;

            let new_box = Box::new((point.0, point.1), block.color);
            block.boxes.push(new_box);
            }
    block
    }
}