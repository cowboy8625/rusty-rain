use super::{AUTHOR, Direction, MAXSPEED, MINSPEED};
use clap::{Parser, crate_description, crate_name, crate_version};
use ezemoji::{CharGroup, CharWidth, GroupKind, MultiRange};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CharGroupKind(pub CharGroup);

impl FromStr for CharGroupKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            // Idea was brought up to use these nerd fonts icons by
            // [tonogdlp](https://github.com/tonogdlp) in PR
            // https://github.com/cowboy8625/ezemoji/pull/4
            // Once merged into ezemoji we can remove this
            "classic" => Ok(CharGroupKind(CharGroup::new(
                GroupKind::Custom("Classic"),
                MultiRange::new(&[
                    ezemoji::JAP_RANGE,
                    ezemoji::NUM_RANGE,
                    34..35,
                    42..44,
                    45..47,
                    58..59,
                    60..63,
                    124..127,
                    166..167,
                ]),
                CharWidth::Double,
            ))),

            // Idea was brought up to use these nerd fonts icons by
            // [hasecilu](https://github.com/hasecilu) in PR
            // https://github.com/cowboy8625/ezemoji/pull/5
            // Once merged into ezemoji we can remove this
            "opensource" => Ok(CharGroupKind(CharGroup::new(
                GroupKind::Custom("OpenSource"),
                MultiRange::new(&[
                    62208..62210,
                    // Remove Apple logo
                    62211..62326,
                    59205..59206,
                    // Devicons
                    59257..59258,
                    58930..58932,
                    // Nerd Fonts custom icons
                    59054..59055,
                    // Seti-UI
                    58975..58976,
                    983211..983212,
                    983714..983715,
                    // Material Design Icons
                    984444..984445,
                ]),
                CharWidth::Double,
            ))),

            // Idea was brought up to use these nerd fonts icons by
            // [hasecilu](https://github.com/hasecilu) in PR
            // https://github.com/cowboy8625/ezemoji/pull/5
            // Once merged into ezemoji we can remove this
            "pglangs" => Ok(CharGroupKind(CharGroup::new(
                GroupKind::Custom("ProgrammingLanguages"),
                MultiRange::new(&[
                    // From all Nerd Fonts
                    57918..57919,
                    58888..58889,
                    58909..58911,
                    58912..58913,
                    58916..58917,
                    58919..58921,
                    58923..58926,
                    58927..58929,
                    58930..58931,
                    58932..58933,
                    58949..58950,
                    58956..58957,
                    58960..58961,
                    58975..58976,
                    58995..58996,
                    58999..59000,
                    59002..59003,
                    59006..59007,
                    59018..59019,
                    59031..59032,
                    59040..59041,
                    59049..59050,
                    59057..59059,
                    59061..59062,
                    59190..59193,
                    59196..59197,
                    59198..59199,
                    59209..59210,
                    59211..59212,
                    59214..59215,
                    59217..59218,
                    59242..59243,
                    59253..59254,
                    59255..59256,
                    59303..59305,
                    59313..59314,
                    60175..60176,
                    60362..60363,
                    61118..61119,
                    62227..62228,
                    62283..62284,
                    983835..983836,
                    984965..984966,
                    985207..985208,
                    985610..985611,
                    987674..987675,
                ]),
                CharWidth::Double,
            ))),
            name => Ok(CharGroupKind(CharGroup::from_str(name)?)),
        }
    }
}

const HELP_DIRECTION: &str = "Set the direction of the Rain.
Default is set to down/south
OPTIONS:
    up or north,
    down or south,
    left or west,
    right or east
";

const HELP_COLORS: &str = "Set color of Rain with color string name or tuple
OPTIONS:
    white,
    red,
    blue,
    green,
    r,g,b
";

const HELP_CHARS: &str = "Set what kind of characters are printed as rain.
OPTIONS:
    all            - This shows most of the Character Groups all at once.
    alphalow       - Lower Case Alphabet Characters
    alphaup        - Upper Case Alphabet Characters
    arrow          - Arrow Emojis or Fancy Characters
    bin            - All Ones and Zeros
    cards          - Playing Cards
    clock          - 🕑
    crab           - 🦀
    dominosh       - 🀽
    dominosv       - 🁫
    earth          - 🌎
    emojis         - This is just a bunch of random Emojis
    jap            - Japanese Characters
    large-letters  - Cool Looking Large Letters
    moon           - 🌕
    num            - Good ol fashion Numbers
    numbered-balls - These are like pool balls
    numbered-cubes - These are like the pool balls but just cubes
    open-source    - Open Source icon emojis
    pglangs        - These are programming language icons emojis
    plants         - Plants of sorts
    shapes         - Squares and Circles of a few colors
    smile          - 😃
";

const HELP_HEAD: &str = "Set the color of the first char in Rain.
OPTIONS:
    white,
    red,
    blue,
    green,
    r,g,b,
    #RRGGBB
";

#[derive(Debug, Parser)]
#[command(
    author = AUTHOR,
    about = "A cross platform matrix rain made with Rust.",
    long_about = Some(crate_description!()),
    color = clap::ColorChoice::Always,
    name = crate_name!(),
    version = crate_version!())]
pub struct Cli {
    #[arg(short, long, default_value_t = false)]
    pub shade: bool,
    #[arg(short, long, help = HELP_CHARS, default_value = "bin")]
    pub group: CharGroupKind,
    #[arg(short = 'C', long, help = HELP_COLORS, default_value_t = String::from("green"))]
    pub color: String,
    #[arg(short = 'H', long, help = HELP_HEAD, default_value_t = String::from("white"))]
    pub head: String,
    #[arg(short, long, help = HELP_DIRECTION, default_value = "south")]
    pub direction: Direction,
    #[arg(short = 'S', long, default_value_t = format!("{MAXSPEED},{MINSPEED}"))]
    pub speed: String,
    #[arg(
        short = 'D',
        long,
        help = "Display Char Group",
        default_value_t = false
    )]
    pub display_group: bool,
}

impl Cli {
    pub fn rain_color(&self) -> (u8, u8, u8) {
        into_color(&self.color)
    }
    pub fn head_color(&self) -> (u8, u8, u8) {
        into_color(&self.head)
    }

    pub fn speed(&self) -> (u64, u64) {
        match self.speed.into_tuple() {
            Ok((max, min)) => (max, min),
            _ => (MAXSPEED, MINSPEED),
        }
    }
    pub fn speed_range(&self) -> std::ops::Range<u64> {
        let (max, min) = self.speed();
        max..min
    }
}

pub fn into_color(value: &str) -> (u8, u8, u8) {
    if let Ok((r, g, b)) = StrTuple::<(u8, u8, u8)>::into_tuple(value) {
        return (r, g, b);
    }
    match value {
        "red" => (255, 0, 0),
        "blue" => (0, 0, 255),
        "green" => (0, 255, 0),
        _ => (255, 255, 255),
    }
}

impl StrTuple<(u64, u64)> for &str {
    type Error = std::num::ParseIntError;
    fn into_tuple(self) -> Result<(u64, u64), Self::Error> {
        let mut nums = Vec::new();
        for num in self.split(',') {
            nums.push(num.parse::<u64>()?);
        }
        let a = nums[0];
        let b = nums[1];
        Ok((a, b))
    }
}

impl StrTuple<(u8, u8, u8)> for &str {
    type Error = std::num::ParseIntError;
    fn into_tuple(self) -> Result<(u8, u8, u8), Self::Error> {
        if self.starts_with('#') {
            let r = u8::from_str_radix(&self[1..3], 16);
            let g = u8::from_str_radix(&self[3..5], 16);
            let b = u8::from_str_radix(&self[5..7], 16);
            return Ok((r?, g?, b?));
        }
        let mut nums = Vec::new();
        for num in self.split(',') {
            nums.push(num.parse::<u8>()?);
        }
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        Ok((a, b, c))
    }
}

trait StrTuple<T> {
    type Error;
    fn into_tuple(self) -> Result<T, Self::Error>;
}
