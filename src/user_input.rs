use crate::{clear, Rain, Result, UserSettings};
use crossterm::{event, style};
use std::io::{BufWriter, Stdout};
use std::time::Duration;
pub fn user_input(
    stdout: &mut BufWriter<Stdout>,
    rain: &mut Rain,
    user_settings: &UserSettings,
    create_color: fn(style::Color, style::Color, u8) -> Vec<style::Color>,
) -> Result<bool> {
    if event::poll(Duration::from_millis(50))? {
        match event::read()? {
            event::Event::Key(keyevent) => {
                if keyevent
                    == event::KeyEvent::new(event::KeyCode::Char('c'), event::KeyModifiers::CONTROL)
                    || keyevent
                        == event::KeyEvent::new(event::KeyCode::Esc, event::KeyModifiers::NONE)
                {
                    return Ok(false);
                }
            }
            event::Event::Resize(w, h) => {
                clear(stdout)?;
                *rain = Rain::new(create_color, w, h, &user_settings);
            }
            _ => {}
        }
    }
    Ok(true)
}
