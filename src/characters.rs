use clap::ValueEnum;

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum CharWidth {
    Single = 1,
    Double = 2,
}

#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum Characters {
    All,
    Alphalow,
    Alphaup,
    AlphaNum,
    Arrow,
    Bin,
    Cards,
    Clock,
    Crab,
    Dominosh,
    Dominosv,
    Earth,
    Emojis,
    Jap,
    LargeLetters,
    Moon,
    Num,
    NumberedBalls,
    NumberedCubes,
    Plants,
    Smile,
    Shapes,
}

impl std::fmt::Display for Characters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Characters::All => "all",
            Characters::Alphalow => "alphalow",
            Characters::Alphaup => "alphaup",
            Characters::AlphaNum => "alphanum",
            Characters::Arrow => "arrow",
            Characters::Bin => "bin",
            Characters::Cards => "cards",
            Characters::Clock => "clock",
            Characters::Crab => "crab",
            Characters::Dominosh => "dominosh",
            Characters::Dominosv => "dominosv",
            Characters::Earth => "earth",
            Characters::Emojis => "emojis",
            Characters::Jap => "jap",
            Characters::LargeLetters => "largeletters",
            Characters::Moon => "moon",
            Characters::Num => "num",
            Characters::NumberedBalls => "numberedballs",
            Characters::NumberedCubes => "numberedcubes",
            Characters::Plants => "plants",
            Characters::Smile => "smile",
            Characters::Shapes => "shapes",
        };
        write!(f, "{value}")
    }
}

impl Characters {
    fn variants_for_all() -> &'static [Characters] {
        &[
            Characters::Alphalow,
            Characters::Alphaup,
            Characters::AlphaNum,
            Characters::Arrow,
            Characters::Bin,
            Characters::Cards,
            Characters::Clock,
            Characters::Crab,
            Characters::Dominosh,
            Characters::Dominosv,
            Characters::Earth,
            Characters::Emojis,
            Characters::Jap,
            Characters::LargeLetters,
            Characters::Moon,
            Characters::Num,
            Characters::NumberedBalls,
            Characters::NumberedCubes,
            Characters::Plants,
            Characters::Smile,
            Characters::Shapes,
        ]
    }

    pub fn as_vec_u32(&self) -> Vec<u32> {
        match self {
            Characters::All => Self::variants_for_all()
                .iter()
                .flat_map(|v| v.as_vec_u32())
                .collect(),

            Characters::Alphalow => (97..=122).collect(),
            Characters::Alphaup => (65..=90).collect(),
            Characters::AlphaNum => Self::Alphalow
                .as_vec_u32()
                .into_iter()
                .chain(Self::Alphaup.as_vec_u32())
                .chain(Self::Num.as_vec_u32())
                .collect(),

            Characters::Arrow => (129024..=129035)
                .chain(129040..=129095)
                .chain(129168..=129195)
                .chain(129104..=129113)
                .collect(),

            Characters::Bin => (48..=49).collect(),
            Characters::Cards => (127137..=127166)
                .chain(127169..=127182)
                .chain(127185..=127198)
                .collect(),

            Characters::Clock => (128336..=128359).collect(),
            Characters::Crab => vec![129408],
            Characters::Dominosh => (127024..=127073).collect(),
            Characters::Dominosv => (127074..=127123).collect(),
            Characters::Earth => (127757..=127760).collect(),

            Characters::Emojis => (129292..=129400)
                .chain(129402..=129482)
                .chain(129484..=129535)
                .collect(),

            Characters::Jap => (65382..=65437).collect(),
            Characters::LargeLetters => (127462..=127487).collect(),
            Characters::Moon => (127760..=127773).collect(),
            Characters::Num => (48..=57).collect(),
            Characters::NumberedBalls => (127312..=127337).collect(),
            Characters::NumberedCubes => (127344..=127369).collect(),
            Characters::Plants => (127793..=127827).collect(),
            Characters::Smile => (128512..=128518).collect(),
            Characters::Shapes => (128992..=129003).collect(),
        }
    }

    pub fn width(&self) -> u16 {
        use Characters::*;
        match self {
            Alphalow | Alphaup | AlphaNum | Bin | Dominosv | Num | Jap => CharWidth::Single as u16,
            _ => CharWidth::Double as u16,
        }
    }
}
