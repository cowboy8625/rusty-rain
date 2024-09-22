// mod arguments;
mod characters;
mod cli;
mod direction;
mod gen;
mod rain;
mod term;
mod update;
mod user_input;

// None Standard Crates
use clap::Parser;
use crossterm::{cursor, execute, queue, style, terminal};
use rand::{thread_rng, Rng};

// Standard Library Crates
use std::io::{stdout, Stdout, Write};

// Modules
//use arguments::cargs;
use characters::Characters;
use direction::Direction;
use rain::Rain;
use term::{clear, draw};
use update::{reset, update};
use user_input::user_input;

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
    settings: cli::Cli,
}

impl App {
    fn new(settings: cli::Cli) -> Self {
        Self {
            stdout: stdout(),
            settings,
        }
    }
    fn run(&mut self) -> std::io::Result<()> {
        let (width, height) = match self.settings.direction {
            Direction::Left | Direction::Right => {
                let (w, h) = terminal::size()?;
                (h, w)
            }
            Direction::Up | Direction::Down => terminal::size()?,
        };

        let create_color = gen::color_function(self.settings.shade);

        let mut rain = Rain::new(create_color, width, height, &self.settings);
        let mut is_running = true;

        terminal::enable_raw_mode()?;
        execute!(self.stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

        while is_running {
            is_running = user_input(&mut self.stdout, &mut rain, &self.settings, create_color)?;
            draw(
                &mut self.stdout,
                &rain,
                self.settings.chars.width(),
                &self.settings.direction,
            )?;
            self.stdout.flush()?;
            update(&mut rain);
            reset(create_color, &mut rain, &self.settings);
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
    let settings = cli::Cli::parse();
    App::new(settings).run()
}
