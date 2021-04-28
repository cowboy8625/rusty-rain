use crate::{
    CharWidth, CharWidth::*, UserSettings, AUTHOR, MAXSPEED, MINSPEED,
};
use clap::{crate_description, crate_name, crate_version, App, Arg};
use ezemoji::*;

pub fn cargs() -> UserSettings {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(AUTHOR)
        .about(crate_description!())
        .arg(
            Arg::with_name("color")
                .short("C")
                .long("color")
                .help(
                    "Set color of Rain with color string name or tuple
OPTIONS:
-------------------------
white,
red,
blue,
green,
r,g,b
-------------------------
",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("head")
                .short("H")
                .long("head")
                .help(
                    "Set the color of the first char in Rain.
OPTIONS:
-------------------------
white,
red,
blue,
green,
r,g,b
-------------------------
",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("characters")
                .short("c")
                .long("chars")
                .help(
                    "Set what kind of characters are printed as rain.
OPTIONS:
-------------------------
all            - List Shows most of the Character Groups all at once.
alphalow       - Lower Case Alphabet Characters
alphaup        - Upper Case Alphabet Characters
arrow          - Arrow Emojis or Fancy Characters
bin            - All Ones and Zeros
cards          - Playing Cards
clock          - Clock Emojis
crab           - Crab
dominosh       - Domino's that are laying horizontal
dominosv       - Domino's that are laying vertical
earth          - Earth Emojis and different rotations
emojis         - This is just a bunch of random Emojis
jap            - Japanese Characters
large-letters  - Cool Looking Large Letters
moon           - Like the Earths but with the moon
num            - Good ol fashion Numbers
numbered-balls - These are like pool balls
numbered-cubes - These are like the pool balls but just cubes
plants         - Plants of sorts
smile          - Smiley faces!!!!
shapes         - Squares and Circles of a few colors
-------------------------
",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("speed")
                .short("S")
                .long("speed")
                .help("Set speed of rain MAX,MIN -S 200,400")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shade")
                .short("s")
                .long("shade")
                .help("Set Rain shading to fade or stay constant")
                .takes_value(false),
        )
        .get_matches();

    let color = match matches.value_of("color").unwrap_or("green") {
        "white" => (255, 255, 255),
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "blue" => (0, 0, 255),
        a => a.to_string().into_tuple(),
    };

    let head = match matches.value_of("head").unwrap_or("white") {
        "white" => (255, 255, 255),
        "red" => (255, 0, 0),
        "green" => (0, 255, 0),
        "blue" => (0, 0, 255),
        a => a.to_string().into_tuple(),
    };

    let (group, double_wide): (Box<dyn EZEmoji>, CharWidth) = match matches.value_of("characters").unwrap_or("bin") {
        "all" => (Box::new(AllEmojis), Double),
        "alphalow" => (Box::new(LowerAlpha), Single),
        "alphaup" => (Box::new(UpperAlpha), Single),
        "arrow" => (Box::new(Arrow), Double),
        "bin" => (Box::new(Bin), Single),
        "cards" => (Box::new(Cards), Double),
        "clock" => (Box::new(Clock), Double),
        "crab" => (Box::new(Crab), Double),
        "dominosh" => (Box::new(HorizontalDominos), Double),
        "dominosv" => (Box::new(VerticalDominos), Single),
        "earth" => (Box::new(Earth), Double),
        "emojis" => (Box::new(Emojis), Double),
        "jap" => (Box::new(Japanese), Single),
        "large-letters" => (Box::new(LargeLetter), Double),
        "moon" => (Box::new(Moon), Double),
        "num" => (Box::new(Numbers), Single),
        "numbered-balls" => (Box::new(NumberedBalls), Double),
        "numbered-cubes" => (Box::new(NumberedCubes), Double),
        "plants" => (Box::new(Plant), Double),
        "smile" => (Box::new(Smile), Double),
        "shapes" => (Box::new(Shape), Double),
        _ => (Box::new(Bin), Single),
    };

    let speed = match matches.value_of("speed") {
        Some(value) => value.to_string().into_tuple(),
        None => (MAXSPEED, MINSPEED),
    };

    let shading = matches.is_present("shade");

    UserSettings::new(color, head, group, shading, double_wide, speed)
}

impl StrTuple<(u64, u64)> for String {
    fn into_tuple(self) -> (u64, u64) {
        let mut nums = Vec::new();
        for num in self.split(',') {
            nums.push(
                num.parse::<u64>()
                    .expect("This is not the correct format, expecting 0,0,0 or name like white"),
            );
        }
        let a = nums[0];
        let b = nums[1];
        (a, b)
    }
}

impl StrTuple<(u8, u8, u8)> for String {
    fn into_tuple(self) -> (u8, u8, u8) {
        let mut nums = Vec::new();
        for num in self.split(',') {
            nums.push(
                num.parse::<u8>()
                    .expect("This is not the correct format, expecting 0,0,0 or name like white"),
            );
        }
        let a = nums[0];
        let b = nums[1];
        let c = nums[2];
        (a, b, c)
    }
}

trait StrTuple<T> {
    fn into_tuple(self) -> T;
}
