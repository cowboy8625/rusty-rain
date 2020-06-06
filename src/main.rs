// TODO: Shading.

use crossterm::{cursor, event, execute, queue, style, terminal, Result};
use rand::{thread_rng, Rng};

use std::char;
use std::cmp::{max, min};
use std::io::{stdout, Stdout, Write};
use std::time::{Duration, Instant};

fn ran_ch() -> char {
    // This gets a Jap Char.
    let c: u32 = thread_rng().gen_range(65382, 65437);
    //let c: u32 = thread_rng().gen_range(48, 50);
    char::from_u32(c).unwrap()
}

fn create_drop_chars(height: u16) -> Vec<char> {
    (0..=height).map(|_| ran_ch()).collect()
}

#[derive(Debug)]
struct Droplets {
    char_list: Vec<char>,
    loc: (u16, u16),
    length: i16,
    time: (Instant, Duration),
}

impl Droplets {
    fn new(x: u16, h: u16) -> Self {
        Self {
            char_list: create_drop_chars(h),
            loc: (x, 0),
            length: thread_rng().gen_range(5, h - 9) as i16,
            time: (
                Instant::now(),
                Duration::from_millis(thread_rng().gen_range(40, 400)),
            ),
        }
    }

    fn down(&mut self) {
        if self.loc.1 < self.char_list.len() as u16 + self.length as u16 {
            self.loc.1 += 1;
        }
    }

    fn last_char(&self) -> u16 {
        max(self.loc.1 as i16 - self.length as i16, 0) as u16
    }

    fn window(&self) -> Vec<char> {
        let left = max((self.loc.1 as isize - self.length as isize) + 1, 0) as usize;
        let right = min(self.loc.1 as usize, self.char_list.len() - 1);
        let mut slice = self.char_list[left..=right].to_vec();
        slice.reverse();
        if self.loc.1 > self.length as u16 - 1 {
            slice.push(' ');
        }
        if self.loc.1 > self.length as u16 && self.loc.1 >= self.char_list.len() as u16 {
            let spaces = self.loc.1 - self.char_list.len() as u16;
            for _ in 0..=spaces {
                slice.insert(0, ' ');
            }
        }
        slice
    }

    fn reset(&mut self) {
        self.char_list = create_drop_chars(self.char_list.len() as u16);
        self.loc.1 = 0;
        self.time = (
            Instant::now(),
            Duration::from_millis(thread_rng().gen_range(40, 400)),
        );
    }
}

struct Rain {
    dim: (u16, u16),
    droplets: Vec<Droplets>,
    drawables: Vec<usize>,
    removeables: Vec<usize>,
}

impl Rain {
    fn new() -> Self {
        let (w, h) = terminal::size().unwrap();
        let droplets = (0..w).map(|x| Droplets::new(x, h)).collect();
        Self {
            dim: (w, h),
            droplets,
            drawables: Vec::with_capacity(w as usize),
            removeables: Vec::with_capacity(w as usize),
        }
    }

    fn move_drop_down(&mut self) {
        // Moves droplet down if it had been drawn.
        for idx in &self.drawables {
            self.droplets[*idx].down();
        }
    }

    fn check_for_off_screen(&mut self) {
        // Checks to see if tail of drop is off screen
        for (idx, droplet) in self.droplets.iter().enumerate() {
            if droplet.last_char() > self.dim.1 {
                self.removeables.push(idx);
            }
        }
    }

    fn remove_droplets(&mut self) {
        // Sorts then removes any droplets that have been off screen.
        self.removeables.sort();
        for idx in &self.removeables {
            self.droplets[*idx].reset();
        }
    }

    fn check_time(&mut self) {
        // Checks droplet time to see if it needs to be drawn.
        let now = Instant::now();
        for (idx, droplet) in self.droplets.iter_mut().enumerate() {
            if droplet.time.0 <= now {
                droplet.time.0 += droplet.time.1;
                self.drawables.push(idx);
            }
        }
    }

    fn update(&mut self) {
        self.move_drop_down();

        // clears Vec's
        self.drawables.clear();
        self.removeables.clear();

        self.check_for_off_screen();
        self.remove_droplets();
        self.check_time();
    }

    fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        for &idx in &self.drawables {
            let droplet = &self.droplets[idx];
            let (x, y) = droplet.loc;
            let window = droplet.window();
            let color_step = 255 / window.len();
            for (idx, c) in window.iter().enumerate() {
                let dy = y - idx as u16;
                if dy < self.dim.1 {
                    match idx {
                        0 => {
                            queue!(
                                stdout,
                                cursor::MoveTo(x, dy),
                                style::SetForegroundColor(style::Color::Rgb {
                                    r: 255,
                                    g: 255,
                                    b: 255
                                }),
                                style::Print(c)
                            )?;
                        }
                        _ => {
                            queue!(
                                stdout,
                                cursor::MoveTo(x, dy),
                                style::SetForegroundColor(style::Color::Rgb{r:0, g:255 - (color_step * idx) as u8, b:0}),
                                //style::SetForegroundColor(style::Color::Rgb { r: 0, g: 255, b: 0 }),
                                style::Print(c)
                            )?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut rain = Rain::new();

    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    loop {
        if event::poll(Duration::from_millis(1))? {
            let event = event::read()?;
            if event == event::Event::Key(event::KeyCode::Esc.into()) {
                break;
            }
        }
        rain.update();
        rain.draw(&mut stdout)?;
        stdout.flush()?;
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;

    Ok(())
}
