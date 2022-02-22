#![allow(unused_variables)]
use std::io;
use std::error::Error;
use rusty_audio::Audio;
use crossterm::{terminal, ExecutableCommand};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use std::thread;

use invaders_game::{
    invaders::Invaders,
    player::Player,
    shots::Shots,
    frame::{
        new_frame,
        Drawable,
        Frame
    },
    render::render,
};

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    for item in &["explode", "lose", "move", "pew", "startup", "win"] {
        audio.add(item, &format!("sounds/{}.wav", item));
    }

    audio.play("startup");

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    let mut player = Player::new();
    let mut invaders = Invaders::new();
    let mut shots = Shots::new();

    let mut frame = new_frame();
    player.draw(&mut frame);
    invaders.draw(&mut frame);
    shots.draw(&mut frame);
    let mut last_frame: Frame;


    render(&mut stdout, &frame, &frame, true)?;

    render(&mut stdout, &frame, &frame, true)?;
    // Game loop
    'gameloop: loop {
        last_frame = frame.clone();
        frame = new_frame();
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Left | KeyCode::Char('h') =>{
                        player.move_left();
                    }
                    KeyCode::Right | KeyCode::Char('l') =>{
                        player.move_right();
                    }
                    KeyCode::Char(' ') =>{
                        audio.play("pew");
                        player.shoot(&mut shots);
                    }
                    _ => { }
                }
            }
        }

        player.draw(&mut frame);

        invaders.update();
        invaders.draw(&mut frame);

        shots.update(&mut invaders);
        shots.draw(&mut frame);


        render(&mut stdout, &last_frame, &frame, false)?;
        if invaders.positions.len() == 0{
            audio.play("win");
            break 'gameloop;
        }
        pause_ms(50);
    }

// cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
//

    Ok(())
}
