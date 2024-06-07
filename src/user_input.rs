use crate::cli::Cli;
use crate::{clear, Rain};
use crossterm::{event, style};
use std::io::Stdout;
use std::time::Duration;

pub fn user_input(
    stdout: &mut Stdout,
    rain: &mut Rain,
    settings: &Cli,
    create_color: fn(style::Color, style::Color, u8) -> Vec<style::Color>,
) -> std::io::Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            event::Event::Key(keyevent) => {
                if keyevent
                    == event::KeyEvent::new(event::KeyCode::Char('c'), event::KeyModifiers::CONTROL)
                    || keyevent
                        == event::KeyEvent::new(event::KeyCode::Esc, event::KeyModifiers::NONE)
                    || keyevent
                        == event::KeyEvent::new(
                            event::KeyCode::Char('Q'),
                            event::KeyModifiers::NONE,
                        )
                    || keyevent
                        == event::KeyEvent::new(
                            event::KeyCode::Char('q'),
                            event::KeyModifiers::NONE,
                        )
                {
                    return Ok(false);
                }
            }
            event::Event::Resize(w, h) => {
                clear(stdout)?;
                *rain = Rain::new(create_color, w, h, settings);
            }
            _ => {}
        }
    }
    Ok(true)
}
