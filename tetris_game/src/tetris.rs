use crossterm::style::Color;
use stopwatch::Stopwatch;
#[allow(unused_variables)]
use crate::{
    frame::{
        Frame,
        Drawable,
    },
    block::{
    Box,
    Block,
    },
    NX_PIXELS, NY_PIXELS, FPS
};

pub struct Tetris{
    wall: [[Option<Box>; NY_PIXELS]; NX_PIXELS],
    pub stopwatch: Stopwatch, 
}

impl Tetris{
    pub fn new() -> Tetris {
        let tetris = Tetris{
            wall: [[None; NY_PIXELS]; NX_PIXELS],
            stopwatch: Stopwatch::start_new(),
        };
        tetris
    }
    pub fn update(&mut self, fast: bool){
        let time = self.stopwatch.elapsed().as_millis() as u16;
        if fast {

        } else if time > FPS {

            for row in 0..NY_PIXELS{
                let clear_row= self.check_row(row);
                if clear_row { self.clear_row(row)}
            }
            self.add_block();
        }
    }
    pub fn add_block(&mut self){
        let block = Block::new();
        let boxes = block.boxes;
        for r#box in boxes.iter(){
            self.wall[r#box.pos.0][r#box.pos.1] = Some(*r#box);
        }
    }
    fn check_row(&self, row: usize) -> bool {
        let mut some_count = 0;
        for col in 0..NX_PIXELS{
            match self.wall[col][row] {
                Some(_) =>  some_count += 1,
                None => {},
            }
        }
        if some_count == NX_PIXELS {true} else { false }
    }
    fn clear_row(&mut self, row: usize) {
        for col in 0..NX_PIXELS{
            self.wall[col][row] = None;
        }
    }
    pub fn in_bounds((i,j): (usize, usize)) -> bool {
        if i < NX_PIXELS{
            if j < NY_PIXELS{
                return true
            }
        }
        return false
    }
}

impl Drawable for Tetris{
    fn draw(&self, frame: &mut Frame){
        for x in 0..NX_PIXELS{
            for y in 0..NY_PIXELS{
                match self.wall[x][y] {
                    Some(r#box) => frame[x][y] = ('▇', r#box.color),
                    None => frame[x][y] = (' ', Color::Black),
                }
            }
        }
    }
}