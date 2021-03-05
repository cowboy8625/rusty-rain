use crate::{CharGroups, EmojiGroups, RustyTypes, AUTHOR};
use clap::{crate_description, crate_name, crate_version, App, Arg};
type COLOR = (u8, u8, u8);
pub fn cargs() -> (COLOR, COLOR, CharGroups<RustyTypes>, bool, bool) {
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
        "alphalow" => (CharGroups::Custom(RustyTypes::LowerAlpha), false),
        "alphaup" => (CharGroups::Custom(RustyTypes::UpperAlpha), false),
        "bin" => (CharGroups::Custom(RustyTypes::Bin), false),
        "num" => (CharGroups::Custom(RustyTypes::Numbers), false),

        "all" => (EmojiGroups::All.into(), true),
        "arrow" => (EmojiGroups::Arrow.into(), true),
        "cards" => (EmojiGroups::Cards.into(), true),
        "clock" => (EmojiGroups::Clock.into(), true),
        "dominosh" => (EmojiGroups::HorizontalDominos.into(), true),
        "dominosv" => (EmojiGroups::VerticalDominos.into(), false),
        "earth" => (EmojiGroups::Earth.into(), true),
        "emojis" => (EmojiGroups::Emojis.into(), true),
        "jap" => (EmojiGroups::Japanese.into(), false),
        "large-letters" => (EmojiGroups::LargeLetter.into(), true),
        "moon" => (EmojiGroups::Moon.into(), true),
        "numbered-balls" => (EmojiGroups::NumberedBalls.into(), true),
        "numbered-cubes" => (EmojiGroups::NumberedCubes.into(), true),
        "plants" => (EmojiGroups::Plant.into(), true),
        "smile" => (EmojiGroups::Smile.into(), true),
        "shapes" => (EmojiGroups::Shape.into(), true),

        // "fancyalphaup" => ((127460, 127487), true), // (127460, 127487)
        // "more-emoji" => ((127744, 128727), true),
        // "alphanumsim" => ((33, 127), false),
        _ => (CharGroups::Custom(RustyTypes::Bin), false),
    };

    let shading = matches.is_present("shade");

    (color, head, characters, shading, double_wide)
}

impl StrTuple for String {
    type Tuple = (u8, u8, u8);
    fn into_tuple(self) -> Self::Tuple {
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

trait StrTuple {
    type Tuple;
    fn into_tuple(self) -> Self::Tuple;
}
