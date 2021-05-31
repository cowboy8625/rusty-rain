mod arguments;
mod characters;
mod gen;
mod rain;
mod term;
mod update;
mod user_settings;

// None Standard Crates
use crossterm::{cursor, event, execute, queue, style, terminal, Result};
use rand::{thread_rng, Rng};

// Standard Library Crates
use std::io::{stdout, BufWriter, Stdout, Write};
use std::time::Duration;

// Modules
use arguments::cargs;
use characters::Characters;
use gen::{
    create_drop_chars, gen_charater_vecs, gen_colors, gen_lengths, gen_times,
    gen_color_function,
};
use rain::Rain;
use term::{clear, draw};
use update::{reset, update_locations, update_queue};

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
        draw(&mut stdout, &rain, user_settings.group.width())?;
        stdout.flush()?;
        update_locations(&mut rain);
        reset(create_color, &mut rain, &user_settings, h);
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
