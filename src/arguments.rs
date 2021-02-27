use crate::{AUTHOR, EmojiGroups, CharGroups, RustyTypes};
use clap::{App, Arg, crate_name, crate_version, crate_description}; type COLOR = (u8, u8, u8);
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
                white,
                red,
                blue,
                green,
                r,g,b",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("head")
                .short("H")
                .long("head")
                .help(
                    "Set the color of the first char in Rain.
                white,
                red,
                blue,
                green,
                r,g,b",
                )
                .takes_value(true),
        )
        .arg(
            Arg::with_name("characters")
                .short("c")
                .long("chars")
                .help(
                    "Set what kind of characters are printed as rain.
                jap          - for Japanese characters
                bin          - for binary characters
                alphalow     - for lowercase characters
                alphaup      - for uppercase characters
                fancyalphaup - for fancy uppercase characters
                moon         - for moon characters
                earth        - for earth characters
                more-emoji   - some colored some black and white emojis
                emoji        - yes emojis!
                num          - for numbers",
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
        "jap" => (EmojiGroups::Japanese.into(), false),           // 65382, 65437
        "bin" => (CharGroups::Custom(RustyTypes::Bin), false), // 48, 50
        "alphalow" => (CharGroups::Custom(RustyTypes::LowerAlpha), false), // (97, 122)
        "alphaup" => (CharGroups::Custom(RustyTypes::UpperAlpha), false),   // (65, 90)
        // "fancyalphaup" => ((127460, 127487), true), // (127460, 127487)
        "num" => (CharGroups::Custom(RustyTypes::Numbers), false), // (48, 57)
        "moon" => (EmojiGroups::Moon.into(), true),
        "earth" => (EmojiGroups::Earth.into(), true),
        // "more-emoji" => ((127744, 128727), true),
        // "emoji" => ((129292, 129535), true),
        "shapes" => (EmojiGroups::Shape.into(), true),
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

