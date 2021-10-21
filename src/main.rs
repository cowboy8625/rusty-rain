mod arguments;
mod characters;
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
use std::io::{stdout, BufWriter, Stdout, Write};

// Modules
use arguments::cargs;
use characters::Characters;
use gen::{
    create_drop_chars, gen_charater_vecs, gen_color_function, gen_colors, gen_lengths, gen_times,
};
use rain::Rain;
use term::{clear, draw};
use update::{reset, update_locations, update_queue};
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
    let mut stdout = BufWriter::with_capacity(8_192, stdout());
    let user_settings = cargs();
    let (width, height) = terminal::size()?;
    let h = height as usize;

    let create_color = gen_color_function(user_settings.shading);

    let mut rain = Rain::new(create_color, width, height, &user_settings);
    let mut is_running = true;

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    while is_running {
        is_running = user_input(&mut stdout, &mut rain, &user_settings, create_color)?;
        update_queue(&mut rain);
        draw(&mut stdout, &rain, user_settings.group.width())?;
        stdout.flush()?;
        update_locations(&mut rain);
        reset(create_color, &mut rain, &user_settings, h);
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
