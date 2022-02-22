use crate::invaders::Invaders;
use crate::frame::{Frame, Drawable};
#[derive(Clone)]
pub struct Shots{ 
    positions: Vec<(usize, usize)>
}

impl Shots {
    pub fn new() -> Shots{
        Shots{
            positions: Vec::new()
        }
    }

    pub fn add_shot(&mut self, (i,j): (usize, usize)){
        self.positions.push((i,j));
    }

    pub fn update(&mut self, invaders: &mut Invaders){
        let mut positions = Vec::new();
        for (i,j) in &mut self.positions{
            *j -= 1;

            let killed = invaders.kill((*i,*j));
            if !killed && !(*j==0) {
                positions.push((*i,*j))
            }
        }
        self.positions = positions.clone();
        // println!("{:?}", self.positions);
    }
}

impl Drawable for Shots {
    fn draw(&self, frame: &mut Frame){
        for (x,y) in self.positions.iter(){
            frame[*x][*y] = '\'';
        }
    }
}