use crate::{
    NUM_COLS,
    shots::Shots,
    frame::{
        Frame,
        Drawable,
    },
};

pub struct Player {
    position: (usize, usize)
}

impl Player {
    pub fn new() -> Player {
        Player {
            position: (20, 26)
        }
    }

    pub fn move_left(&mut self){
        if self.position.0 > 0{
            self.position.0 -= 1;
        }
    }

    pub fn move_right(&mut self){
        if self.position.0 < NUM_COLS-1{
            self.position.0 += 1;
        }
    }

    pub fn shoot(&self, shots: &mut Shots){
        let (x,y) = self.position;
        shots.add_shot((x,y-1));
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame){
        let (x,y) = self.position;
        frame[x][y] = 'A';
    }
}