use clap::{App, Arg};
use crossterm::{cursor, event, execute, queue, style, terminal, Result};
use rand::{thread_rng, Rng};
use buffy::{Line, Cell, Buffer};

use std::char;
use std::cmp::{max, min};
use std::io::BufWriter;
use std::io::{stdout, Stdout, Write};
use std::time::{Duration, Instant};

fn cargs() -> ((u8, u8, u8), (u32, u32), bool) {
    let matches = App::new("Matrix Rain")
        .version("0.0.1")
        .author("Cowboy8625 <cowboy8625@protonmail.com>")
        .about("A terminal program the makes all your friends think you are a hacker.")
        .arg(
            Arg::with_name("red")
                .short("r")
                .long("red")
                .help("Set color of characters RED value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("green")
                .short("g")
                .long("green")
                .help("Set color of characters GREEN value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("blue")
                .short("b")
                .long("blue")
                .help("set color of characters BLUE value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("characters")
                .short("c")
                .long("chars")
                .help("Set what kind of characters are printed as rain")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shade")
                .short("s")
                .long("shade")
                .help("Set Rain shading to fade or stay constant")
                .takes_value(true),
        )
        .get_matches();

    let r = matches
        .value_of("red")
        .and_then(|v| {
            v.parse::<u8>()
                .map_err(|_| println!("This is not a Valid Red Option"))
                .ok()
        })
        .unwrap_or(0);
    let g = matches
        .value_of("green")
        .and_then(|v| {
            v.parse::<u8>()
                .map_err(|_| println!("This is not a Valid Green Option"))
                .ok()
        })
        .unwrap_or(255);
    let b = matches
        .value_of("blue")
        .and_then(|v| {
            v.parse::<u8>()
                .map_err(|_| println!("This is not a Valid Blue Option"))
                .ok()
        })
        .unwrap_or(0);

    let characters = match matches.value_of("characters").unwrap_or("01") {
        "jap" => (65382, 65437),
        "01" | _ => (48, 50),
    };

    let shading = match matches.value_of("shade").unwrap_or("0") {
        "1" => true,
        "0" | _ => false,
    };

    ((r, g, b), characters, shading)
}

fn ran_ch((min, max): (u32, u32)) -> char {
    let c: u32 = thread_rng().gen_range(min, max);
    char::from_u32(c).unwrap()
}

fn create_drop_chars(height: u16, characters: (u32, u32)) -> Vec<char> {
    (0..=height).map(|_| ran_ch(characters)).collect()
}

#[derive(Debug)]
struct Droplets {
    char_list: Vec<char>,
    loc: (u16, u16),
    length: i16,
    time: (Instant, Duration),
}

impl Droplets {
    fn new(x: u16, h: u16, characters: (u32, u32)) -> Self {
        Self {
            char_list: create_drop_chars(h, characters),
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

    fn reset(&mut self, characters: (u32, u32)) {
        self.char_list = create_drop_chars(self.char_list.len() as u16, characters);
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
    color: (u8, u8, u8),
    characters: (u32, u32),
    shading: bool,
}

impl Rain {
    fn new(color: (u8, u8, u8), characters: (u32, u32), shading: bool) -> Self {
        let (w, h) = terminal::size().expect("Could not find terminal size");
        let droplets = (0..w).map(|x| Droplets::new(x, h, characters)).collect();
        Self {
            dim: (w, h),
            droplets,
            drawables: Vec::with_capacity(w as usize),
            removeables: Vec::with_capacity(w as usize),
            color,
            characters,
            shading,
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
            self.droplets[*idx].reset(self.characters);
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

    fn draw(&mut self, stdout: &mut BufWriter<Stdout>) -> Result<()> {
        let mut buffer = Buffer::new(self.dim.0 as usize, self.dim.1 as usize, ' ');
        for &idx in &self.drawables {
            let droplet = &self.droplets[idx];
            let (x, y) = droplet.loc;
            let window = droplet.window();
            let color_step_r = self.color.0 as usize / window.len();
            let color_step_g = self.color.1 as usize / window.len();
            let color_step_b = self.color.2 as usize / window.len();
            let mut line: Vec<Cell> = window.iter().enumerate().map(|(i, c)| {
                if i == 0 {
                    (
                        *c,
                        Some(style::SetForegroundColor(style::Color::Rgb {
                            r: 255,
                            g: 255,
                            b: 255
                        }).to_string()),
                        Some(style::SetForegroundColor(style::Color::Reset).to_string()),
                        ).into()
                } else {
                    (
                        *c,
                        Some(style::SetForegroundColor(style::Color::Rgb {
                            r: self.color.0,
                            g: self.color.1,
                            b: self.color.2,
                        }).to_string()),
                        Some(style::SetForegroundColor(style::Color::Reset).to_string()),
                        ).into()
                }
                }).collect();
            line.reverse();
            buffer.insert_vline(x, y, &line);
            // for (idx, c) in window.iter().enumerate() {
            //     let dy = y - idx as u16;
            //     if dy < self.dim.1 {
            //         match idx {
            //             0 => {
            //                 queue!(
            //                     stdout,
            //                     cursor::MoveTo(x, dy),
            //                     style::SetForegroundColor(style::Color::Rgb {
            //                         r: 255,
            //                         g: 255,
            //                         b: 255
            //                     }),
            //                     style::Print(c)
            //                 )?;
            //             }
            //             _ => {
            //                 let color = if self.shading {
            //                     style::SetForegroundColor(style::Color::Rgb {
            //                     })
            //                 } else {
            //                     style::SetForegroundColor(style::Color::Rgb {
            //                         r: self.color.0,
            //                         g: self.color.1,
            //                         b: self.color.2,
            //                     })
            //                 };
            //                 queue!(stdout, cursor::MoveTo(x, dy), color, style::Print(c))?;
            //             }
            //         }
            //     }
            // }
        }
        execute!(stdout, cursor::MoveTo(0, 0), style::Print(&buffer))?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut stdout = BufWriter::with_capacity(8_192, stdout());
    let (color, characters, shading) = cargs();
    let mut rain = Rain::new(color, characters, shading);

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    loop {
        if event::poll(Duration::from_millis(100))? {
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
    terminal::disable_raw_mode()?;

    Ok(())
}
