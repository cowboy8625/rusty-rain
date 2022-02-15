mod arguments;
mod characters;
mod direction;
mod gen;
mod rain;
mod term;
mod update;
mod user_input;
mod user_settings;

// None Standard Crates
use crossterm::{cursor, execute, queue, style, terminal, Result};
use rand::{thread_rng, Rng};

// Standard Library Crates
use std::io::{stdout, Stdout, Write};

// Modules
use arguments::cargs;
use characters::Characters;
use direction::Direction;
use rain::Rain;
use term::{clear, draw};
use update::{reset, update};
use user_input::user_input;
use user_settings::UserSettings;

const MAXSPEED: u64 = 40;
const MINSPEED: u64 = 200;
const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

fn main() -> Result<()> {
    let mut stdout = stdout();
    let user_settings = cargs();
    let (width, height) = match user_settings.direction {
        Direction::Left | Direction::Right => {
            let (w, h) = terminal::size()?;
            (h, w)
        }
        Direction::Up | Direction::Down => terminal::size()?,
    };

    let create_color = gen::color_function(user_settings.shading);

    let mut rain = Rain::new(create_color, width, height, &user_settings);
    let mut is_running = true;

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    while is_running {
        is_running = user_input(&mut stdout, &mut rain, &user_settings, create_color)?;
        draw(
            &mut stdout,
            &rain,
            user_settings.group.width(),
            &user_settings.direction,
        )?;
        stdout.flush()?;
        update(&mut rain);
        reset(create_color, &mut rain, &user_settings);
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
