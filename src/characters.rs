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
        write!(f, "{}", value)
    }
}

impl Characters {
    pub fn as_vec_u32(&self) -> Vec<u32> {
        match self {
            Self::All => Self::Alphalow
                .as_vec_u32()
                .into_iter()
                .chain(Self::Alphaup.as_vec_u32())
                .chain(Self::AlphaNum.as_vec_u32())
                .chain(Self::Arrow.as_vec_u32())
                .chain(Self::Bin.as_vec_u32())
                .chain(Self::Cards.as_vec_u32())
                .chain(Self::Clock.as_vec_u32())
                .chain(Self::Crab.as_vec_u32())
                .chain(Self::Dominosh.as_vec_u32())
                .chain(Self::Dominosv.as_vec_u32())
                .chain(Self::Earth.as_vec_u32())
                .chain(Self::Emojis.as_vec_u32())
                .chain(Self::Jap.as_vec_u32())
                .chain(Self::LargeLetters.as_vec_u32())
                .chain(Self::Moon.as_vec_u32())
                .chain(Self::Num.as_vec_u32())
                .chain(Self::NumberedBalls.as_vec_u32())
                .chain(Self::NumberedCubes.as_vec_u32())
                .chain(Self::Plants.as_vec_u32())
                .chain(Self::Smile.as_vec_u32())
                .chain(Self::Shapes.as_vec_u32())
                .collect(),
            Self::Alphalow => (97..=122).collect(),
            Self::Alphaup => (65..=90).collect(),
            Self::AlphaNum => Self::Alphalow
                .as_vec_u32()
                .into_iter()
                .chain(Self::Alphaup.as_vec_u32())
                .chain(Self::Num.as_vec_u32())
                .collect(),
            Self::Arrow => (129024..=129035)
                .chain(129040..=129095)
                .chain(129168..=129195)
                .chain(129168..=129195)
                .chain(129104..=129113)
                .collect(),
            Self::Bin => (48..=49).collect(),
            Self::Cards => (127137..=127166)
                .chain(127169..=127182)
                .chain(127185..=127198)
                .collect(),
            Self::Clock => (128336..=128359).collect(),
            Self::Crab => vec![129408],
            Self::Dominosh => (127024..=127073).collect(),
            Self::Dominosv => (127074..=127123).collect(),
            Self::Earth => (127757..=127760).collect(),
            Self::Emojis => (129292..=129400) // Hearts
                .chain(129402..=129482) // Diamonds
                .chain(129484..=129535) // Clubs
                // Spades?
                .collect(),
            Self::Jap => (65382..=65437).collect(),
            Self::LargeLetters => (127462..=127487).collect(),
            Self::Moon => (127760..=127773).collect(),
            Self::Num => (48..=57).collect(),
            Self::NumberedBalls => (127312..=127337).collect(),
            Self::NumberedCubes => (127344..=127369).collect(),
            Self::Plants => (127793..=127827).collect(),
            Self::Smile => (128512..=128518).collect(),
            Self::Shapes => (128992..=129003).collect(),
        }
    }
}

impl Characters {
    pub fn width(&self) -> u16 {
        match self {
            Self::All => CharWidth::Double as u16,
            Self::Alphalow => CharWidth::Single as u16,
            Self::Alphaup => CharWidth::Single as u16,
            Self::AlphaNum => CharWidth::Single as u16,
            Self::Arrow => CharWidth::Double as u16,
            Self::Bin => CharWidth::Single as u16,
            Self::Cards => CharWidth::Double as u16,
            Self::Clock => CharWidth::Double as u16,
            Self::Crab => CharWidth::Double as u16,
            Self::Dominosh => CharWidth::Double as u16,
            Self::Dominosv => CharWidth::Single as u16,
            Self::Earth => CharWidth::Double as u16,
            Self::Emojis => CharWidth::Double as u16,
            Self::Jap => CharWidth::Single as u16,
            Self::LargeLetters => CharWidth::Double as u16,
            Self::Moon => CharWidth::Double as u16,
            Self::Num => CharWidth::Single as u16,
            Self::NumberedBalls => CharWidth::Double as u16,
            Self::NumberedCubes => CharWidth::Double as u16,
            Self::Plants => CharWidth::Double as u16,
            Self::Smile => CharWidth::Double as u16,
            Self::Shapes => CharWidth::Double as u16,
        }
    }
}
