//! <h1 align="center">
//!   <br>
//!   <img src="https://user-images.githubusercontent.com/43012445/105452071-411e4880-5c43-11eb-8ae2-4de61f310bf9.gif" alt="GIF" width="800">
//!   <img src="https://cdn.discordapp.com/attachments/509849754583302154/812942011400847391/emoji_rain.gif" alt="GIF" width="800">
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
//! USAGE:
//!     rusty-rain [FLAGS] [OPTIONS]
//!
//! FLAGS:
//!     -h, --help       Prints help information
//!     -s, --shade      Set Rain shading to fade or stay constant
//!     -V, --version    Prints version information
//!
//! OPTIONS:
//!     -c, --chars <characters>    Set what kind of characters are printed as rain.
//!                                 OPTIONS:
//!                                 -------------------------
//!                                 all            - List Shows most of the Character Groups all at once.
//!                                 alphalow       - Lower Case Alphabet Characters
//!                                 alphaup        - Upper Case Alphabet Characters
//!                                 arrow          - Arrow Emojis or Fancy Characters
//!                                 bin            - All Ones and Zeros
//!                                 cards          - Playing Cards
//!                                 clock          - Clock Emojis
//!                                 dominosh       - Domino's that are laying horizontal
//!                                 dominosv       - Domino's that are laying vertical
//!                                 earth          - Earth Emojis and different rotations
//!                                 emojis         - This is just a bunch of random Emojis
//!                                 jap            - Japanese Characters
//!                                 large-letters  - Cool Looking Large Letters
//!                                 moon           - Like the Earths but with the moon
//!                                 num            - Good ol fashion Numbers
//!                                 numbered-balls - These are like pool balls
//!                                 numbered-cubes - These are like the pool balls but just cubes
//!                                 plants         - Plants of sorts
//!                                 crab           - Crabs
//!                                 smile          - Smiley faces!!!!
//!                                 shapes         - Squares and Circles of a few colors
//!                                 -------------------------
//!     -C, --color <color>         Set color of Rain with color string name or tuple
//!                                 OPTIONS:
//!                                 -------------------------
//!                                 white,
//!                                 red,
//!                                 blue,
//!                                 green,
//!                                 r,g,b
//!                                 -------------------------
//!     -H, --head <head>           Set the color of the first char in Rain.
//!                                 OPTIONS:
//!                                 -------------------------
//!                                 white,
//!                                 red,
//!                                 blue,
//!                                 green,
//!                                 r,g,b
//!                                 -------------------------
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
//! To exit just press `ESC` or `Ctrl + c`
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
//! `cargo run --release -- -C 0,139,139 -H 255,255,255 1 -c jap -s`
//!
//! after installing:
//!
//! `rusty-rain -C 0,139,139 -H 255,255,255 -c jap -s`
//!
//! # Help
//!
//! If you find any bugs or performance is not up to par please submit a issue so I can better improve
//! the project.

mod gen;
mod rain;
mod term;
mod update;
use crossterm::{cursor, event, execute, queue, style, terminal, Result};
use gen::{create_drop_chars, gen_charater_vecs, gen_colors, gen_lengths, gen_times};
use rain::Rain;
use rand::{thread_rng, Rng};
use std::io::{stdout, BufWriter, Stdout, Write};
use std::time::Duration;
use term::{clear, draw};
use update::{reset, update_locations, update_queue};
mod arguments;
use arguments::cargs;

use ezemoji::{CharGroups, EZEmojis, EmojiGroups};

const MAXSPEED: u64 = 40;
const MINSPEED: u64 = 200;
const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum RustyTypes {
    Bin,
    Numbers,
    LowerAlpha,
    UpperAlpha,
}

fn main() -> Result<()> {
    let mut stdout = BufWriter::with_capacity(8_192, stdout());
    let (color, head, characters, shading, double_wide) = cargs();
    let (width, height) = terminal::size()?;
    let h = height as usize;

    let mut e = EZEmojis::default();
    e.add(CharGroups::Custom(RustyTypes::Numbers), (48..57).collect());
    e.add(CharGroups::Custom(RustyTypes::Bin), (48..50).collect());
    e.add(
        CharGroups::Custom(RustyTypes::LowerAlpha),
        (97..122).collect(),
    );
    e.add(
        CharGroups::Custom(RustyTypes::UpperAlpha),
        (65..90).collect(),
    );
    let default_vec = &vec![96];
    let characters = e.get_u32(&characters).unwrap_or(default_vec);

    // This Creates a closure off of the args
    // given to the program at start that will crates the colors for the rain
    let create_color = match shading {
        // Creates shading colors
        true => |bc: style::Color, head: style::Color, length: u8| {
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
        },
        // creates with out color
        false => |bc: style::Color, head: style::Color, length: u8| {
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

    let spacing = if double_wide { 2 } else { 1 };

    let mut rain = Rain::new(
        create_color,
        head,
        width,
        height,
        color.into(),
        &characters,
        spacing,
    );

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
                    rain = Rain::new(create_color, head, w, h, color.into(), characters, spacing);
                }
                _ => {}
            }
        }
        update_queue(&mut rain);
        draw(&mut stdout, &rain, spacing)?;
        stdout.flush()?;
        update_locations(&mut rain);
        reset(create_color, head, &mut rain, characters, h, color.into());
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
