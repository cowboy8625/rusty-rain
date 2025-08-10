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
const MINSPEED: u64 = 200;
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

impl Default for Random {
    fn default() -> Self {
        Self {
            #[cfg(test)]
            rng: rand::rngs::StdRng::seed_from_u64(42),
            #[cfg(not(test))]
            rng: rand::rng(),
        }
    }
}

impl Random {
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
    /// Random number generator wrapper for testing purposes
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
    /// Speed of the rain
    speed: std::ops::Range<u64>,
    /// Group of characters defined by a array of u64 values
    group: Characters,
    /// Width of the terminal
    /// NOTE: This value is not a true width of the terminal but size in visible characters
    /// 🌕 is a single character but takes up 2 columns and so the width value would count this as
    /// 1.  This maybe be a wrong way to think about it 🤷 (two wide character).
    width: usize,
    /// Height of the terminal.
    /// Unlike width this is a true height of the terminal
    height: usize,
    /// Current screen buffer
    screen_buffer: Vec<Cell>,
    /// Previous screen buffer
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

        let mut rng = Random::default();
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

        let window_height = match settings.direction {
            Direction::Up | Direction::Down => height,
            Direction::Left | Direction::Right => width,
        };
        let windows: Vec<usize> = (0..width)
            .map(|_| {
                rng.random_range(
                    Self::MIN_LENGTH_OF_RAIN
                        ..window_height.saturating_sub(Self::MAX_LENGTH_OFFSET_OF_RAIN),
                )
            })
            .collect();

        let speed = settings.speed_range();
        let now = Instant::now();
        let time: Vec<(Instant, Duration)> = (0..width)
            .map(|_| {
                let milli_seconds = rng.random_range(speed.start..speed.end);
                let duration = Duration::from_millis(milli_seconds);
                let future_delay_ms = rng.random_range(0..2000);
                let start = now + Duration::from_millis(future_delay_ms);

                (start, duration)
            })
            .collect();

        Self {
            rng,
            body_colors: vec![settings.rain_color().into(); width],
            chars,
            directions: vec![settings.direction; width],
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
        let milli_seconds = self.rng.random_range(self.speed.start..self.speed.end);
        *duration = Duration::from_millis(milli_seconds);
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
            let direction = self.directions[i];

            // Helper closure: compute buffer index from x,y safely
            let idx = |x: usize, y: usize| -> Option<usize> {
                if x < self.width && y < self.height {
                    Some(y * self.width + x)
                } else {
                    None
                }
            };

            // 1. Tail cleanup
            let is_tail_visible = pos >= window_len;
            if is_tail_visible {
                match direction {
                    Direction::Down => {
                        let tail_y = pos - window_len;
                        if let Some(buf_idx) = idx(i, tail_y) {
                            self.screen_buffer[buf_idx] = Cell::default();
                        } else {
                            self.reset(i);
                            continue;
                        }
                    }
                    Direction::Up => {
                        let tail_y = self.height.saturating_sub(pos - window_len + 1);
                        if let Some(buf_idx) = idx(i, tail_y) {
                            self.screen_buffer[buf_idx] = Cell::default();
                        } else {
                            self.reset(i);
                            continue;
                        }
                    }
                    Direction::Right => {
                        let tail_x = pos - window_len;
                        if let Some(buf_idx) = idx(tail_x, i) {
                            self.screen_buffer[buf_idx] = Cell::default();
                        } else {
                            self.reset(i);
                            continue;
                        }
                    }
                    Direction::Left => {
                        let tail_x = self.width.saturating_sub(pos - window_len + 1);
                        if let Some(buf_idx) = idx(tail_x, i) {
                            self.screen_buffer[buf_idx] = Cell::default();
                        } else {
                            self.reset(i);
                            continue;
                        }
                    }
                }
            }

            // 2. Head beyond screen check
            let is_head_out_of_bounds = match direction {
                Direction::Down => pos > self.height,
                Direction::Up => pos > self.height,
                Direction::Right => pos > self.width,
                Direction::Left => pos > self.width,
            };
            if is_head_out_of_bounds {
                self.positions[i] += 1;
                continue;
            }

            // 3. Draw visible portion
            let visible_len = (pos + 1).min(window_len);
            for offset in 0..visible_len {
                let (x, y) = match direction {
                    Direction::Down => (i, pos - offset),
                    Direction::Up => (i, self.height.saturating_sub(pos - offset + 1)),
                    Direction::Right => (pos - offset, i),
                    Direction::Left => (self.width.saturating_sub(pos - offset + 1), i),
                };

                if let Some(buf_idx) = idx(x, y) {
                    let char_idx = (start_idx + pos - offset) % self.chars.len();
                    let c = self.chars[char_idx];
                    let color = if offset == 0 {
                        self.head_colors[i]
                    } else {
                        self.body_colors[i]
                    };
                    self.screen_buffer[buf_idx] = Cell::new(c).color(color);
                }
            }

            self.positions[i] += 1;
        }

        Ok(())
    }

    fn draw_frame(&mut self, w: &mut BufWriter<Stdout>) -> std::io::Result<()> {
        let total_cells = self.width * self.height;
        let mut redraw_screen = false;

        for (i, (a, b)) in self
            .screen_buffer
            .iter()
            .zip(&self.previous_screen_buffer)
            .enumerate()
        {
            if a != b {
                self.queue.push(i);
            }
            let is_50_percent_or_more_changed = self.queue.len() > total_cells / 2;
            if is_50_percent_or_more_changed {
                redraw_screen = true;
                break;
            }
        }

        if redraw_screen {
            execute!(w, cursor::MoveTo(0, 0))?;
            let screen = self
                .screen_buffer
                .iter()
                .map(|c| format!("{c}"))
                .collect::<String>();
            execute!(w, Print(screen))?;
            self.queue.clear();
            return Ok(());
        }

        let group_width = self.group.width() as usize;

        for idx in self.queue.drain(..) {
            let cell = &self.screen_buffer[idx];
            let x = (idx % self.width) * group_width;
            let y = idx / self.width;

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

            self.previous_screen_buffer[idx] = *cell;
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
