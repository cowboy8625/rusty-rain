//! <h1 align="center">
//!   <br>
//!   <img src="https://user-images.githubusercontent.com/43012445/105452071-411e4880-5c43-11eb-8ae2-4de61f310bf9.gif" alt="GIF" width="800">
//!   <br>
//!   Rusty Rain
//!   <br>
//!   <br>
//! </h1>
//!
//! <p align="center">
//!   <a><img alt="lastupdated" src="https://img.shields.io/github/last-commit/cowboy8625/rusty-rain"></a>
//!   <a><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/cowboy8625/rusty-rain"></a>
//!   <a><img alt="issuse" src="https://img.shields.io/github/issues/cowboy8625/rusty-rain"></a>
//!   <a><img alt="Lines of Code" src="https://img.shields.io/tokei/lines/github/cowboy8625/rusty-rain"></a>
//!   <a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
//!   <a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
//! </p>
//!
//! A cross platform matrix rain terminal program that runs well and looks good.
//!
//! ## To Use
//!
//! Simply run the following command on windows/mac/linux:
//!
//! ```
//! git clone https://github.com/cowboy8625/rusty-rain.git
//! cd rusty-rain
//! ```
//! ```
//! cargo run --release
//! ```
//!
//! or to install:
//!
//! ```
//!  cargo install rusty-rain
//! ```
//!
//! ## Exit
//!
//! To exit just press `ESC`
//!
//!
//! ## Command Line Arguments
//!
//! ```
//! ```
//!
//! ### Example
//!
//! using cargo to run:
//!
//! `cargo run --release -- -C "(0, 139, 139)" -H "(255, 255, 255)" 1 -c jap -s`
//!
//! after installing:
//!
//! `rusty-rain -C "(0, 139, 139)" -H "(255, 255, 255)" -c jap -s`
//!
//! # Help
//!
//! If find any bugs or performance is not up to par please submit a issue so I can better improve
//! the project.
use crossterm::{cursor, event, execute, queue, style, terminal, Result};
use rand::{thread_rng, Rng};
use std::char;
use std::io::{stdout, BufWriter, Stdout, Write};
use std::time::{Duration, Instant};
mod arguments;
use arguments::cargs;

const MAXSPEED: u64 = 40;
const MINSPEED: u64 = 200;
const VERSION: &str = "0.0.2";
const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";
const ABOUT: &str = "A terminal program the makes all your friends think you are a hacker.";

pub trait Unsigned {}
impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl Unsigned for usize {}

fn ran_ch((min, max): (u32, u32)) -> char {
    let c: u32 = thread_rng().gen_range(min..max);
    char::from_u32(c).unwrap()
}

fn create_drop_chars(height: u16, characters: (u32, u32)) -> Vec<char> {
    (0..height + 1).map(|_| ran_ch(characters)).collect()
}

fn gen_charater_vecs(width: usize, height: u16, characters: (u32, u32)) -> Vec<Vec<char>> {
    let mut ch = Vec::new();
    for _ in 0..width {
        ch.push(create_drop_chars(height, characters));
    }
    ch
}

fn gen_times(width: usize) -> Vec<(Instant, Duration)> {
    let now = Instant::now();
    let mut times = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..width {
        times.push((
            now,
            Duration::from_millis(rng.gen_range(MAXSPEED..MINSPEED)),
        ));
    }
    times
}

fn gen_lengths(width: usize, height: usize) -> Vec<usize> {
    let mut len = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..width {
        len.push(rng.gen_range(4..height - 10));
    }
    len
}

fn gen_colors<F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>>(
    create_color: F,
    head: (u8, u8, u8),
    width: usize,
    length: &[usize],
    bc: style::Color,
) -> Vec<Vec<style::Color>> {
    let mut colors = Vec::with_capacity(width);
    for l in length.iter() {
        colors.push(create_color(bc, head.into(), *l as u8));
    }
    colors
}

fn usub<T>(x: T, y: T) -> T
where
    T: std::ops::Sub<Output = T> + std::cmp::PartialOrd + From<u8> + Unsigned,
{
    if y > x {
        T::from(0)
    } else {
        x - y
    }
}

fn clamp(x: usize, mx: usize, mn: usize) -> usize {
    std::cmp::max(mn, std::cmp::min(x, mx))
}

fn update_queue(rain: &mut Rain) {
    rain.queue.clear();
    let now = Instant::now();
    for (i, (t, d)) in rain.time.iter_mut().enumerate() {
        if *t <= now {
            *t += *d;
            rain.queue.push(i);
        }
    }
}

fn clear(w: &mut BufWriter<Stdout>) -> Result<()> {
    queue!(w, terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

fn draw(w: &mut BufWriter<Stdout>, rain: &Rain) -> Result<()> {
    let (mut chr, mut loc, mut len, mut clr);
    let height = rain.height();
    for x in rain.queue.iter() {
        chr = &rain.charaters[*x];
        loc = &rain.locations[*x];
        len = &rain.length[*x];
        clr = &rain.colors[*x];

        let start = clamp(usub(*loc, *len), chr.len(), 0);
        let end = clamp(loc + 1, chr.len(), 1);
        let slice = chr[start..end].iter();

        let cstart = if loc > len {
            clr.len() - slice.len()
        } else {
            0
        };

        let color = &clr[cstart..clr.len()];

        for (y, ch) in slice.rev().enumerate() {
            queue!(
                w,
                cursor::MoveTo(*x as u16, (*loc.min(&height) - y) as u16),
                style::SetForegroundColor(color[y]),
                style::Print(ch),
            )?;
        }
        if loc >= len {
            queue!(
                w,
                cursor::MoveTo(*x as u16, (usub(*loc, *len)) as u16),
                style::Print(' '),
            )?;
        }
    }
    Ok(())
}

fn update_locations(rain: &mut Rain) {
    let queue = &rain.queue;
    for i in queue.iter() {
        rain.locations[*i] += 1;
    }
}

fn reset<F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>>(
    create_color: F,
    head: (u8, u8, u8),
    rain: &mut Rain,
    characters: (u32, u32),
    height: usize,
    bc: style::Color,
) {
    // assert_eq!(height, rain.height());
    let mut rng = thread_rng();
    let h16 = height as u16;
    let now = Instant::now();
    for i in rain.queue.iter() {
        if rain.locations[*i] > height + rain.length[*i] {
            rain.charaters[*i] = create_drop_chars(h16, characters);
            rain.locations[*i] = 0;
            rain.length[*i] = rng.gen_range(4..height - 10);
            rain.colors[*i] = create_color(bc, head.into(), rain.length[*i] as u8);
            rain.time[*i] = (now, Duration::from_millis(rng.gen_range(10..200)));
        }
    }
}

#[derive(Debug)]
struct Rain {
    charaters: Vec<Vec<char>>,
    locations: Vec<usize>,
    length: Vec<usize>,
    colors: Vec<Vec<style::Color>>,
    time: Vec<(Instant, Duration)>,
    queue: Vec<usize>,
}

impl Rain {
    fn new<F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>>(
        create_color: F,
        head: (u8, u8, u8),
        width: u16,
        height: u16,
        base_color: style::Color,
        characters: (u32, u32),
    ) -> Self {
        let w = width as usize;
        let h = height as usize;
        let charaters = gen_charater_vecs(w, height, characters);
        let locations = vec![0; w];
        let length = gen_lengths(w, h);
        let colors = gen_colors(create_color, head, w, &length, base_color);
        let time = gen_times(w);
        let queue = Vec::with_capacity(w);
        Self {
            charaters,
            locations,
            length,
            colors,
            time,
            queue,
        }
    }

    fn height(&self) -> usize {
        self.charaters[0].len() - 1
    }
}

fn main() -> Result<()> {
    let mut stdout = BufWriter::with_capacity(8_192, stdout());
    let (color, characters, shading, head) = cargs();
    let (width, height) = terminal::size()?;
    let h = height as usize;

    // This Creates a closure off of the args
    // given to the program at start that will crates the colors for the rain
    let create_color = match (color, characters, shading) {
        // Creates shading colors
        (_, (65382, 65437), true) | (_, (48, 50), true) => {
            |bc: style::Color, head: style::Color, length: u8| {
                let mut c: Vec<style::Color> = Vec::with_capacity(length as usize);
                let (mut nr, mut ng, mut nb);
                if let style::Color::Rgb { r, g, b } = bc {
                    for i in 0..length {
                        nr = r / length;
                        ng = g / length;
                        nb = b / length;
                        c.push((nr * i, ng * i, nb * i).into());
                    }
                    c.push(head);
                    c.reverse();
                }
                c
            }
        }
        // creates with out color
        (_, (65382, 65437), false) | (_, (48, 50), false) => {
            |bc: style::Color, head: style::Color, length: u8| {
                let mut c: Vec<style::Color> = Vec::with_capacity(length as usize);
                c.push(head);
                if let style::Color::Rgb { r, g, b } = bc {
                    for _ in 0..length {
                        c.push((r, g, b).into());
                    }
                }
                c
            }
        }
        // Same as with out color
        _ => |bc: style::Color, head: style::Color, length: u8| {
            let mut c: Vec<style::Color> = Vec::with_capacity(length as usize);
            c.push(head);
            if let style::Color::Rgb { r, g, b } = bc {
                for _ in 0..length {
                    c.push((r, g, b).into());
                }
            }
            c
        },
    };

    let mut rain = Rain::new(create_color, head, width, height, color.into(), characters);

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
                    rain = Rain::new(create_color, head, w, h, color.into(), characters);
                }
                _ => {}
            }
        }
        update_queue(&mut rain);
        draw(&mut stdout, &rain)?;
        stdout.flush()?;
        update_locations(&mut rain);
        reset(create_color, head, &mut rain, characters, h, color.into());
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
