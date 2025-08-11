mod characters;
mod cli;
#[cfg(test)]
mod test;

use characters::Characters;
use clap::{Parser, ValueEnum};
use crossterm::{
    cursor, event, execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use std::io::{BufWriter, Stdout, Write, stdout};
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

    fn display(&self, width: usize) -> String {
        let c = if width >= 2 && !self.is_visible() {
            " ".repeat(width)
        } else {
            self.char.to_string()
        };
        if cfg!(test) {
            c
        } else {
            format!("{}{}", crossterm::style::SetForegroundColor(self.color), c)
        }
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

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Up => write!(f, "Up"),
            Self::Down => write!(f, "down"),
            Self::Left => write!(f, "Left"),
            Self::Right => write!(f, "Right"),
        }
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
    /// In the case that shading the second value is the precompiled list of colors
    ///               Base             Shaded
    body_colors: Vec<(Color, Option<Vec<Color>>)>,
    /// Shading of the rain
    shading: bool,
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
    fn new(mut width: usize, height: usize, settings: &cli::Cli) -> Self {
        width /= settings.chars.width() as usize;

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

        let body_colors = if settings.shade {
            let base_color: Color = settings.rain_color().into();
            (0..width)
                .map(|i| {
                    let window = windows[i].saturating_sub(1);
                    let colors = gen_shade_color(base_color, window as u8);
                    (base_color, Some(colors))
                })
                .collect::<Vec<_>>()
        } else {
            vec![(settings.rain_color().into(), None); width]
        };

        Self {
            shading: settings.shade,
            body_colors,
            chars,
            directions: vec![settings.direction; width],
            group: settings.chars,
            head_colors: vec![settings.head_color().into(); width],
            height,
            positions: vec![0; width],
            previous_screen_buffer: vec![Cell::default(); width * height],
            queue: Vec::with_capacity(width),
            rng,
            screen_buffer: vec![Cell::default(); width * height],
            speed,
            starts,
            time,
            width,
            windows,
        }
    }

    #[inline(always)]
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

    #[inline(always)]
    fn reset_time(&mut self, i: usize) {
        let (start, duration) = &mut self.time[i];
        *start = Instant::now();
        let milli_seconds = self.rng.random_range(self.speed.start..self.speed.end);
        *duration = Duration::from_millis(milli_seconds);
    }

    #[inline(always)]
    fn reset_start(&mut self, i: usize) {
        self.starts[i] = self.rng.random_range(0..self.chars.len());
    }

    #[inline(always)]
    fn reset_window(&mut self, i: usize) {
        self.windows[i] = self.rng.random_range(
            Self::MIN_LENGTH_OF_RAIN..self.height.saturating_sub(Self::MAX_LENGTH_OFFSET_OF_RAIN),
        );
    }

    #[inline(always)]
    fn reset_position(&mut self, i: usize) {
        self.positions[i] = 0;
    }

    #[inline(always)]
    fn reset_body_colors(&mut self, i: usize) {
        if !self.shading {
            return;
        }
        let base_color: Color = self.body_colors[i].0;
        let window = self.windows[i].saturating_sub(1);
        let colors = gen_shade_color(base_color, window as u8);
        self.body_colors[i] = (base_color, Some(colors));
    }

    fn reset(&mut self, i: usize) {
        self.reset_time(i);
        self.reset_start(i);
        self.reset_window(i);
        self.reset_position(i);
        self.reset_body_colors(i);
    }

    fn update_screen_buffer(&mut self) -> std::io::Result<()> {
        for i in self.queue.drain(..).collect::<Vec<usize>>() {
            let pos = self.positions[i];
            let start_idx = self.starts[i];
            let window_len = self.windows[i];
            let direction = self.directions[i];

            let get_index = |x: usize, y: usize| -> Option<usize> {
                if x < self.width && y < self.height {
                    Some(y * self.width + x)
                } else {
                    None
                }
            };

            let finished = match direction {
                Direction::Down | Direction::Up => pos > (self.height + window_len),
                Direction::Right | Direction::Left => pos > (self.width + window_len),
            };
            if finished {
                self.reset(i);
                continue;
            }

            let is_tail_visible = pos >= window_len;
            if is_tail_visible {
                let buf_idx = match direction {
                    Direction::Down => {
                        let tail_y = pos - window_len;
                        get_index(i, tail_y)
                    }
                    Direction::Up => {
                        let tail_y = self.height.saturating_sub(pos - window_len + 1);
                        get_index(i, tail_y)
                    }
                    Direction::Right => {
                        let tail_x = pos - window_len;
                        get_index(tail_x, i)
                    }
                    Direction::Left => {
                        let tail_x = self.width.saturating_sub(pos - window_len + 1);
                        get_index(tail_x, i)
                    }
                };
                if let Some(buf_idx) = buf_idx {
                    self.screen_buffer[buf_idx] = Cell::default();
                }
            }

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

            let visible_len = (pos + 1).min(window_len);
            for offset in 0..visible_len {
                let (x, y) = match direction {
                    Direction::Down => (i, pos - offset),
                    Direction::Up => (i, self.height.saturating_sub(pos - offset + 1)),
                    Direction::Right => (pos - offset, i),
                    Direction::Left => (self.width.saturating_sub(pos - offset + 1), i),
                };

                if let Some(buf_idx) = get_index(x, y) {
                    let char_idx = (start_idx + pos - offset) % self.chars.len();
                    let c = self.chars[char_idx];
                    let color = if offset == 0 {
                        self.head_colors[i]
                    } else if let Some(fade) = &self.body_colors[i].1 {
                        fade[(fade.len() - (visible_len - 1)) + (offset - 1)]
                    } else {
                        self.body_colors[i].0
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

        if redraw_screen && matches!(self.directions[0], Direction::Left | Direction::Right) {
            let char_width = self.group.width() as usize;
            for (y, chunk) in self.screen_buffer.chunks(self.width).enumerate() {
                let screen = chunk
                    .iter()
                    .map(|c| c.display(char_width))
                    .collect::<String>();
                queue!(w, cursor::MoveTo(0, y as u16), Print(screen))?;

                self.queue.clear();
            }

            return Ok(());
        } else if redraw_screen {
            execute!(w, cursor::MoveTo(0, 0))?;
            let char_width = self.group.width() as usize;
            let screen = self
                .screen_buffer
                .iter()
                .map(|c| c.display(char_width))
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

            if cell.is_visible() {
                queue!(
                    w,
                    cursor::MoveTo(x as u16, y as u16),
                    SetForegroundColor(cell.color),
                    Print(cell.char)
                )?;
            } else {
                queue!(
                    w,
                    cursor::MoveTo(x as u16, y as u16),
                    SetForegroundColor(cell.color),
                    Print(" ".repeat(group_width))
                )?;
            }

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
                    event::Event::Resize(w, h) => {
                        // TODO: make a method that handle resizing so we dont regenerate the rain
                        rain = Rain::<1024>::new(w as usize, h as usize, &self.settings);
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

    #[inline(always)]
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

/// Generates a vector of Colors that fade to black over the length of the column.
fn gen_shade_color(bc: Color, length: u8) -> Vec<Color> {
    let mut colors = Vec::with_capacity(length as usize);
    let Color::Rgb { r, g, b } = bc else {
        return colors;
    };
    let nr = r / length;
    let ng = g / length;
    let nb = b / length;
    for i in 0..length {
        colors.push((nr * i, ng * i, nb * i).into());
    }
    colors.reverse();
    colors
}

fn main() -> std::io::Result<()> {
    let settings = cli::Cli::parse();
    App::new(settings).run()
}
