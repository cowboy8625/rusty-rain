mod gen;
mod rain;
mod term;
mod update;
use crossterm::{cursor, event, execute, queue, style, terminal, Result};
use gen::{create_drop_chars, gen_charater_vecs, gen_colors, gen_lengths, gen_times};
use rain::Rain;
use rand::{thread_rng, Rng};
use std::io::{stdout, BufWriter, Stdout, Write};
use std::time::Duration;
use term::{clear, draw};
use update::{reset, update_locations, update_queue};
mod arguments;
use arguments::cargs;

use ezemoji::*;

const MAXSPEED: u64 = 40;
const MINSPEED: u64 = 200;
const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

#[derive(Clone, Copy)]
pub enum CharWidth {
    Single,
    Double,
}

impl CharWidth {
    pub fn value(self) -> u16 {
        match self {
            Self::Single => 1,
            Self::Double => 2,
        }
    }
}

// #[derive(Clone)]
pub struct UserSettings {
    rain_color: (u8, u8, u8),
    head_color: (u8, u8, u8),
    group: Box<dyn EZEmoji>,
    shading: bool,
    spacing: CharWidth,
    speed: (u64, u64),
}

impl UserSettings {
    pub fn new(
        rain_color: (u8, u8, u8),
        head_color: (u8, u8, u8),
        group: Box<dyn EZEmoji>,
        shading: bool,
        spacing: CharWidth,
        speed: (u64, u64),
    ) -> Self {
        Self {
            rain_color,
            head_color,
            group,
            shading,
            spacing,
            speed,
        }
    }
}

fn gen_color_function(shading: bool) -> fn(style::Color, style::Color, u8) -> Vec<style::Color> {
    // This Creates a closure off of the args
    // given to the program at start that will crates the colors for the rain
    match shading {
        // Creates shading colors
        true => |bc: style::Color, head: style::Color, length: u8| {
            let mut c: Vec<style::Color> = Vec::with_capacity(length as usize);
            let (mut nr, mut ng, mut nb);
            if let style::Color::Rgb { r, g, b } = bc {
                for i in 0..length {
                    nr = r / length;
                    ng = g / length;
                    nb = b / length;
                    c.push((nr * i, ng * i, nb * i).into());
                }
                c.push(head);
                c.reverse();
            }
            c
        },
        // creates with out color
        _ => |bc: style::Color, head: style::Color, length: u8| {
            let mut c: Vec<style::Color> = Vec::with_capacity(length as usize);
            c.push(head);
            if let style::Color::Rgb { r, g, b } = bc {
                for _ in 0..length {
                    c.push((r, g, b).into());
                }
            }
            c
        },
    }
}

fn main() -> Result<()> {
    let mut stdout = BufWriter::with_capacity(8_192, stdout());
    let user_settings = cargs();
    let (width, height) = terminal::size()?;
    let h = height as usize;

    let create_color = gen_color_function(user_settings.shading);

    let mut rain = Rain::new(create_color, width, height, &user_settings);

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    loop {
        if event::poll(Duration::from_millis(50))? {
            match event::read()? {
                event::Event::Key(keyevent) => {
                    if keyevent
                        == event::KeyEvent::new(
                            event::KeyCode::Char('c'),
                            event::KeyModifiers::CONTROL,
                        )
                        || keyevent
                            == event::KeyEvent::new(event::KeyCode::Esc, event::KeyModifiers::NONE)
                    {
                        break;
                    }
                }
                event::Event::Resize(w, h) => {
                    clear(&mut stdout)?;
                    rain = Rain::new(create_color, w, h, &user_settings);
                }
                _ => {}
            }
        }
        update_queue(&mut rain);
        draw(&mut stdout, &rain, user_settings.spacing.value())?;
        stdout.flush()?;
        update_locations(&mut rain);
        reset(create_color, &mut rain, &user_settings, h);
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
