use crossterm::{
    ExecutableCommand,
    terminal::{
        self,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    cursor::{
        Hide,
        Show,
    },
    event::{
        self,
        Event,
        KeyCode,
        },
    };
use std::{
    io::{
        self,
    },
    time::Duration,
    error::Error,
    thread,
};

use tetris_game::{
    block::{
        Block,
    },
    frame::{
        Frame,
        new_frame,
        Drawable,
    },
    render::{
        render,
    }, tetris::Tetris
};

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() -> Result<(), Box<dyn Error>> {

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let mut tetris = Tetris::new();
    let mut frame = new_frame();

    tetris.add_block();
    tetris.draw(&mut frame);
    pause_ms(500);
    println!("{}", tetris.stopwatch.elapsed().as_millis());
    pause_ms(500);

    render(&mut stdout, &frame, &frame, true)?;

    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => { }
                }
            }
        }

        tetris.update();
    }


// cleanup
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;
//

    Ok(())
}