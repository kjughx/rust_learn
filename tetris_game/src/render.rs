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
    NX_PIXELS,
    NY_PIXELS,
};

pub fn render(stdout: &mut Stdout, then: &Frame, now: &Frame, force: bool) -> io::Result<()> {

    stdout
        .execute(SetBackgroundColor(Color::Blue))?
        .execute(SetForegroundColor(Color::Black))?;

    for x in 0..NX_PIXELS{
        for y in 0..NY_PIXELS{
            if then[x][y] != now[x][y] || force {
                stdout
                    .execute(SetForegroundColor(now[x][y].1))?
                    .queue(cursor::MoveTo(x as u16,y as u16))?
                    .queue(Print(now[x][y].0))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}