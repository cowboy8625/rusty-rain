use crate::{
    CharGroups, CharWidth::*, EmojiGroups, RustyTypes, UserSettings, AUTHOR, MAXSPEED, MINSPEED,
};
use clap::{crate_description, crate_name, crate_version, App, Arg};

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
                .help("Set speed of rain")
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

    let (characters, double_wide) = match matches.value_of("characters").unwrap_or("bin") {
        "alphalow" => (CharGroups::Custom(RustyTypes::LowerAlpha), Single),
        "alphaup" => (CharGroups::Custom(RustyTypes::UpperAlpha), Single),
        "bin" => (CharGroups::Custom(RustyTypes::Bin), Single),
        "num" => (CharGroups::Custom(RustyTypes::Numbers), Single),

        "all" => (EmojiGroups::All.into(), Double),
        "arrow" => (EmojiGroups::Arrow.into(), Double),
        "cards" => (EmojiGroups::Cards.into(), Double),
        "clock" => (EmojiGroups::Clock.into(), Double),
        "crab" => (EmojiGroups::Crab.into(), Double),
        "dominosh" => (EmojiGroups::HorizontalDominos.into(), Double),
        "dominosv" => (EmojiGroups::VerticalDominos.into(), Single),
        "earth" => (EmojiGroups::Earth.into(), Double),
        "emojis" => (EmojiGroups::Emojis.into(), Double),
        "jap" => (EmojiGroups::Japanese.into(), Single),
        "large-letters" => (EmojiGroups::LargeLetter.into(), Double),
        "moon" => (EmojiGroups::Moon.into(), Double),
        "numbered-balls" => (EmojiGroups::NumberedBalls.into(), Double),
        "numbered-cubes" => (EmojiGroups::NumberedCubes.into(), Double),
        "plants" => (EmojiGroups::Plant.into(), Double),
        "smile" => (EmojiGroups::Smile.into(), Double),
        "shapes" => (EmojiGroups::Shape.into(), Double),

        // "fancyalphaup" => ((127460, 127487), true), // (127460, 127487)
        // "more-emoji" => ((127744, 128727), true),
        // "alphanumsim" => ((33, 127), false),
        _ => (CharGroups::Custom(RustyTypes::Bin), Single),
    };

    let speed = match matches.value_of("speed") {
        Some(value) => value.to_string().into_tuple(),
        None => (MAXSPEED, MINSPEED),
    };

    let shading = matches.is_present("shade");

    UserSettings::new(color, head, characters, shading, double_wide, speed)
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
