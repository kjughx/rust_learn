use crate::{NUM_COLS, NUM_ROWS, N_ENEMIES};
use rand::Rng;
use crate::frame::{Frame, Drawable};
#[derive(Clone)]
pub struct Invaders {
    pub positions: Vec<(usize, usize)>
}

impl Invaders {
    pub fn new() -> Invaders{
        let mut invaders = Vec::with_capacity(N_ENEMIES);
        let mut rng = rand::thread_rng();
        for _ in 0..N_ENEMIES{
           let i = rng.gen_range(0..NUM_COLS); 
           let j = rng.gen_range(0..NUM_ROWS/3);
           invaders.push((i,j));
        }
        Invaders {
            positions: invaders
        }
    }

    pub fn kill(&mut self, (i,j): (usize, usize)) -> bool{
        for idx in 0..self.positions.len(){
            let (k,l) = (self.positions[idx].0, self.positions[idx].1);
            if (i,j) == (k,l) {
                self.positions.remove(idx);
                return true;
            }
        }
        false
    }
    fn in_bounds(position: (i32, i32)) -> bool{
        if 0 <= position.0 && position.0 < NUM_COLS as i32 {
            if 0 <= position.1 && position.1 < (NUM_ROWS/3) as i32{
                return true;
            }
        }
        false
    }
    
    pub fn update(&mut self){
        let mut rng = rand::thread_rng();
        let directions = [(-1,0), (1,0), (0,1), (0,-1)];
        for position in &mut self.positions{
            let idx = rng.gen_range(0..4);
            let direction = directions[idx];
            match direction {
                (-1, 0) => {
                    let point = (position.0 as i32 + direction.0, position.1 as i32 + direction.1);
                    if Invaders::in_bounds(point){
                        position.0 -= 1;
                    }
                }
                (1, 0) => {
                    let point = (position.0 as i32 + direction.0, position.1 as i32 + direction.1);
                    if Invaders::in_bounds(point){
                        position.0 += 1;
                    }
                }
                (0, 1) => {
                    let point = (position.0 as i32 + direction.0, position.1 as i32 + direction.1);
                    if Invaders::in_bounds(point){
                        position.1 += 1;
                    }
                }
                (0,-1) => {
                    let point = (position.0 as i32 + direction.0, position.1 as i32 + direction.1);
                    if Invaders::in_bounds(point){
                        position.1 -= 1;
                    }
                }
                _ => {}
            }
        }
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame){
        for (x,y) in self.positions.iter(){
            frame[*x][*y] = '*';
        }
    }
}
