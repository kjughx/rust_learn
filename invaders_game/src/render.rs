#![allow(unused_variables, unused_imports)]
use std::io::{self, Stdout, Write}; 
use crossterm::{
    ExecutableCommand,
    QueueableCommand,
    terminal, cursor, style::{self, Print},
    execute,
    Result, 
    style::{
        SetForegroundColor,
        SetBackgroundColor,
        Stylize, 
        ResetColor, 
        Color, 
        Attribute,
    }
};

use crate::{
    frame::Frame,
    NUM_COLS,
    NUM_ROWS,
};

pub fn render(stdout: &mut Stdout, then: &Frame, now: &Frame, force: bool) -> io::Result<()> {

    stdout
        .execute(SetBackgroundColor(Color::Blue))?
        .execute(SetForegroundColor(Color::Black))?;

    for x in 0..NUM_COLS{
        for y in 0..NUM_ROWS{
            if then[x][y] != now[x][y] || force {
                stdout
                    .queue(cursor::MoveTo(x as u16,y as u16))?
                    .queue(Print(now[x][y]))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}