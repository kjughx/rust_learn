use crossterm::style::Color;

use crate::{
    NX_PIXELS,
    NY_PIXELS,
};
#[allow(unused_variables)]
pub trait Drawable {
    fn draw(&self, frame: &mut Frame){}
}

pub type Frame = [[(char, Color); NY_PIXELS]; NX_PIXELS];
pub fn new_frame() -> Frame{
    [[(' ', Color::Black); NY_PIXELS]; NX_PIXELS]
}