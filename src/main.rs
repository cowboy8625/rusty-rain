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
use crossterm::{cursor, execute, queue, style, terminal};
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

struct App {
    stdout: Stdout,
    user_settings: UserSettings,
}

impl App {
    fn new(user_settings: UserSettings) -> Self {
        Self {
            stdout: stdout(),
            user_settings,
        }
    }
    fn run(&mut self) -> std::io::Result<()> {
        let (width, height) = match self.user_settings.direction {
            Direction::Left | Direction::Right => {
                let (w, h) = terminal::size()?;
                (h, w)
            }
            Direction::Up | Direction::Down => terminal::size()?,
        };

        let create_color = gen::color_function(self.user_settings.shading);

        let mut rain = Rain::new(create_color, width, height, &self.user_settings);
        let mut is_running = true;

        terminal::enable_raw_mode()?;
        execute!(self.stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

        while is_running {
            is_running = user_input(
                &mut self.stdout,
                &mut rain,
                &self.user_settings,
                create_color,
            )?;
            draw(
                &mut self.stdout,
                &rain,
                self.user_settings.group.width(),
                &self.user_settings.direction,
            )?;
            self.stdout.flush()?;
            update(&mut rain);
            reset(create_color, &mut rain, &self.user_settings);
        }
        Ok(())
    }
}

impl Drop for App {
    fn drop(&mut self) {
        execute!(self.stdout, cursor::Show, terminal::LeaveAlternateScreen)
            .expect("failed to leave alternate screen");
        terminal::disable_raw_mode().expect("failed to disable raw mode");
    }
}

fn main() -> std::io::Result<()> {
    let user_settings = cargs();
    App::new(user_settings).run()
}
