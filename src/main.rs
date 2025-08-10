mod characters;
mod cli;
mod direction;
#[cfg(test)]
mod test;

use characters::Characters;
use clap::Parser;
use crossterm::{
    cursor, event, execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use direction::Direction;
use std::io::{stdout, BufWriter, Stdout, Write};
use std::time::{Duration, Instant};

const MAXSPEED: u64 = 0;
const MINSPEED: u64 = 400;
const POLL_INTERVAL: Duration = Duration::from_millis(50);

const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

use rand::Rng;

#[cfg(test)]
use rand::SeedableRng;

/// rand crate wrapper for testing.
/// being able to have deterministic tests is important
#[derive(Debug)]
pub struct Random {
    #[cfg(test)]
    rng: rand::rngs::StdRng,
    #[cfg(not(test))]
    rng: rand::rngs::ThreadRng,
}

impl Random {
    pub fn new() -> Self {
        Self {
            #[cfg(test)]
            rng: rand::rngs::StdRng::seed_from_u64(42),
            #[cfg(not(test))]
            rng: rand::rng(),
        }
    }

    pub fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: rand::distr::uniform::SampleUniform + PartialOrd,
        R: rand::distr::uniform::SampleRange<T>,
    {
        self.rng.random_range(range)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cell {
    char: char,
    color: Color,
}

impl Cell {
    fn new(char: char) -> Self {
        Self {
            char,
            color: Color::Reset,
        }
    }

    fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    fn is_visible(&self) -> bool {
        self.char != ' '
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            char: ' ',
            color: Color::Reset,
        }
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            crossterm::style::SetForegroundColor(self.color),
            self.char
        )
    }
}

#[derive(Debug)]
struct Rain<const LENGTH: usize> {
    rng: Random,
    /// Characters to use for the rain
    chars: [char; LENGTH],
    /// Starting positions of the rain within the chars array
    /// If the amount left in the array is less then the height of the screen/terminal then
    /// the slice with wrap around the chars array.
    starts: Vec<usize>,
    /// Window size for each column of rain
    windows: Vec<usize>,
    /// Current positions of the rain falling
    positions: Vec<usize>,
    /// Color of the rain body
    body_colors: Vec<Color>,
    /// Color of the rain head
    head_colors: Vec<Color>,
    /// Direction of the rain
    directions: Vec<Direction>,
    /// Animation timing
    time: Vec<(Instant, Duration)>,
    /// List of columns that need to be updated
    queue: Vec<usize>,

    speed: std::ops::Range<u64>,
    group: Characters,
    width: usize,
    height: usize,
    screen_buffer: Vec<Cell>,
    previous_screen_buffer: Vec<Cell>,
}

impl<const LENGTH: usize> Rain<LENGTH> {
    const MIN_LENGTH_OF_RAIN: usize = 4;
    const MAX_LENGTH_OFFSET_OF_RAIN: usize = 4;
    fn new(mut width: usize, mut height: usize, settings: &cli::Cli) -> Self {
        if matches!(settings.direction, Direction::Up | Direction::Down) {
            width /= settings.chars.width() as usize;
        } else if matches!(settings.direction, Direction::Left | Direction::Right) {
            height /= settings.chars.width() as usize;
        }

        let mut rng = Random::new();
        let chars_u32 = settings.chars.as_vec_u32();
        let chars: [char; LENGTH] = std::array::from_fn(|_| {
            chars_u32
                .get(rng.random_range(0..chars_u32.len()))
                .and_then(|&c| char::from_u32(c))
                .unwrap_or('#') // fallback character
        });

        let starts: Vec<usize> = (0..width)
            .map(|_| rng.random_range(0..chars.len()))
            .collect();

        let windows: Vec<usize> = (0..width)
            .map(|_| {
                rng.random_range(
                    Self::MIN_LENGTH_OF_RAIN
                        ..height.saturating_sub(Self::MAX_LENGTH_OFFSET_OF_RAIN),
                )
            })
            .collect();

        let speed = settings.speed_range();
        let time: Vec<(Instant, Duration)> = (0..width)
            .map(|_| {
                let start = Instant::now();
                let duration = Duration::from_millis(rng.random_range(speed.start..speed.end));
                (start, duration)
            })
            .collect();

        Self {
            rng,
            body_colors: vec![settings.rain_color().into(); width],
            chars,
            directions: vec![Direction::Down; width],
            head_colors: vec![settings.head_color().into(); width],
            height,
            positions: vec![0; width],
            previous_screen_buffer: vec![Cell::default(); width * height],
            queue: Vec::with_capacity(width),
            screen_buffer: vec![Cell::default(); width * height],
            starts,
            group: settings.chars,
            speed,
            time,
            width,
            windows,
        }
    }

    #[inline]
    fn update(&mut self) {
        for i in 0..self.width {
            let (start, duration) = self.time[i];
            if start.elapsed() > duration {
                self.queue.push(i);
                let (start, _) = &mut self.time[i];
                *start = Instant::now();
            }
        }
    }

    #[inline]
    fn reset_time(&mut self, i: usize) {
        let (start, duration) = &mut self.time[i];
        *start = Instant::now();
        *duration = Duration::from_millis(self.rng.random_range(self.speed.start..self.speed.end));
    }

    #[inline]
    fn reset_start(&mut self, i: usize) {
        self.starts[i] = self.rng.random_range(0..self.chars.len());
    }

    #[inline]
    fn reset_window(&mut self, i: usize) {
        self.windows[i] = self.rng.random_range(
            Self::MIN_LENGTH_OF_RAIN..self.height.saturating_sub(Self::MAX_LENGTH_OFFSET_OF_RAIN),
        );
    }

    #[inline]
    fn reset_position(&mut self, i: usize) {
        self.positions[i] = 0;
    }

    fn reset(&mut self, i: usize) {
        self.reset_time(i);
        self.reset_start(i);
        self.reset_window(i);
        self.reset_position(i);
    }

    fn update_screen_buffer(&mut self) -> std::io::Result<()> {
        for i in self.queue.drain(..).collect::<Vec<usize>>() {
            let pos = self.positions[i];
            let start_idx = self.starts[i];
            let window_len = self.windows[i];

            // Remove trailing character if rain is longer than window
            if pos >= window_len {
                let tail_y = pos - window_len;
                let idx = tail_y * self.width + i;

                if idx >= self.screen_buffer.len() {
                    self.reset(i);
                    continue;
                }

                self.screen_buffer[idx] = Cell::default();
            }

            // If head is below the screen, just advance position
            if pos > self.height {
                self.positions[i] += 1;
                continue;
            }

            // Draw visible portion of the rain
            let visible_len = (pos + 1).min(window_len);
            for offset in 0..visible_len {
                let y = pos - offset;
                if y >= self.height {
                    continue;
                }

                let char_idx = (start_idx + pos - offset) % self.chars.len();
                let c = self.chars[char_idx];
                let color = if offset == 0 {
                    self.head_colors[i]
                } else {
                    self.body_colors[i]
                };
                self.screen_buffer[y * self.width + i] = Cell::new(c).color(color);
            }

            self.positions[i] += 1;
        }

        Ok(())
    }

    fn draw_frame(&mut self, w: &mut BufWriter<Stdout>) -> std::io::Result<()> {
        let group_width = self.group.width() as usize;
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                if self.screen_buffer[idx] == self.previous_screen_buffer[idx] {
                    continue;
                }
                let cell = &self.screen_buffer[idx];
                let x = x * group_width;
                queue!(
                    w,
                    cursor::MoveTo(x as u16, y as u16),
                    SetForegroundColor(cell.color),
                    if cell.is_visible() {
                        Print(cell.char.to_string())
                    } else {
                        Print(" ".repeat(group_width))
                    },
                )?;
                self.previous_screen_buffer[idx] = self.screen_buffer[idx];
            }
        }

        Ok(())
    }
}

struct App {
    stdout: BufWriter<Stdout>,
    settings: cli::Cli,
}

impl App {
    fn new(settings: cli::Cli) -> Self {
        Self {
            stdout: BufWriter::with_capacity(640_000, stdout()),
            settings,
        }
    }

    fn run(&mut self) -> std::io::Result<()> {
        let (w, h) = terminal::size()?;
        let mut rain = Rain::<1024>::new(w as usize, h as usize, &self.settings);
        self.setup_terminal()?;

        let mut is_running = true;
        while is_running {
            if event::poll(POLL_INTERVAL)? {
                match event::read()? {
                    event::Event::Key(key) if Self::is_exit_key(&key) => {
                        is_running = false;
                    }
                    event::Event::Resize(_, _) => {
                        queue!(self.stdout, terminal::Clear(terminal::ClearType::All))?;
                    }
                    _ => {}
                }
            }

            rain.update();
            rain.update_screen_buffer()?;
            rain.draw_frame(&mut self.stdout)?;

            self.stdout.flush()?;
        }

        Ok(())
    }

    #[inline]
    fn setup_terminal(&mut self) -> std::io::Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(())
    }

    fn is_exit_key(key: &event::KeyEvent) -> bool {
        matches!(
            *key,
            event::KeyEvent {
                code: event::KeyCode::Char('c'),
                modifiers: event::KeyModifiers::CONTROL,
                ..
            } | event::KeyEvent {
                code: event::KeyCode::Esc,
                ..
            } | event::KeyEvent {
                code: event::KeyCode::Char('q' | 'Q'),
                modifiers: event::KeyModifiers::NONE,
                ..
            }
        )
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
