use super::{AUTHOR, MAXSPEED, MINSPEED};
use crate::characters::Characters;
use crate::direction::Direction;
use clap::{crate_description, crate_name, crate_version, Parser};

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
    #[arg(short, long, default_value_t = Characters::Bin)]
    pub chars: Characters,
    #[arg(short = 'C', long, default_value_t = String::from("green"))]
    pub color: String,
    #[arg(short = 'H', long, default_value_t = String::from("white"))]
    pub head: String,
    #[arg(short, long, default_value_t = Direction::Down)]
    pub direction: Direction,
    #[arg(short = 'S', long, default_value_t = format!("{MINSPEED},{MAXSPEED}"))]
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
            Ok((min, max)) => (min, max),
            _ => (MINSPEED, MAXSPEED),
        }
    }
    pub fn speed_range(&self) -> std::ops::Range<u64> {
        let (min, max) = self.speed();
        min..max
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
