use super::{AUTHOR, Direction, MAXSPEED, MINSPEED};
use clap::{Parser, crate_description, crate_name, crate_version};
use ezemoji::CharGroup;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct CharGroupKind(pub CharGroup);

impl FromStr for CharGroupKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "all" => Ok(CharGroupKind(CharGroup::ALL)),
            "alphalow" => Ok(CharGroupKind(CharGroup::ALPHALOW)),
            "alphaup" => Ok(CharGroupKind(CharGroup::ALPHAUP)),
            "alphanum" => Ok(CharGroupKind(CharGroup::ALPHANUM)),
            "arrow" => Ok(CharGroupKind(CharGroup::ARROW)),
            "bin" => Ok(CharGroupKind(CharGroup::BIN)),
            "cards" => Ok(CharGroupKind(CharGroup::CARDS)),
            "clock" => Ok(CharGroupKind(CharGroup::CLOCK)),
            "crab" => Ok(CharGroupKind(CharGroup::CRAB)),
            "dominosh" => Ok(CharGroupKind(CharGroup::DOMINOSH)),
            "dominosv" => Ok(CharGroupKind(CharGroup::DOMINOSV)),
            "earth" => Ok(CharGroupKind(CharGroup::EARTH)),
            "emojis" => Ok(CharGroupKind(CharGroup::EMOJIS)),
            "jap" => Ok(CharGroupKind(CharGroup::JAP)),
            "large-letters" => Ok(CharGroupKind(CharGroup::LARGELETTERS)),
            "moon" => Ok(CharGroupKind(CharGroup::MOON)),
            "num" => Ok(CharGroupKind(CharGroup::NUM)),
            "numbered-balls" => Ok(CharGroupKind(CharGroup::NUMBEREDBALLS)),
            "numbered-cubes" => Ok(CharGroupKind(CharGroup::NUMBEREDCUBES)),
            "plants" => Ok(CharGroupKind(CharGroup::PLANTS)),
            "smile" => Ok(CharGroupKind(CharGroup::SMILE)),
            "shapes" => Ok(CharGroupKind(CharGroup::SHAPES)),
            _ => Err(format!("Invalid group kind: {s}")),
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
    plants         - Plants of sorts
    smile          - 😃
    shapes         - Squares and Circles of a few colors
";

const HELP_HEAD: &str = "Set the color of the first char in Rain.
OPTIONS:
    white,
    red,
    blue,
    green,
    r,g,b
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
    match value {
        c if StrTuple::<(u8, u8, u8)>::into_tuple(c).is_ok() => match c.into_tuple() {
            Ok((r, g, b)) => (r, g, b),
            _ => (255, 255, 255),
        },
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
