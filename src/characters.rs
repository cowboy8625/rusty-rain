use ezemoji::*;

#[derive(Debug, Clone, Copy)]
pub enum CharWidth {
    Single,
    Double,
}

impl CharWidth {
    pub fn width(self) -> u16 {
        match self {
            Self::Single => 1,
            Self::Double => 2,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Characters {
    All(AllEmojis),
    Alphalow(LowerAlpha),
    Alphaup(UpperAlpha),
    Arrow(Arrow),
    Bin(Bin),
    Cards(Cards),
    Clock(Clock),
    Crab(Crab),
    Dominosh(HorizontalDominos),
    Dominosv(VerticalDominos),
    Earth(Earth),
    Emojis(Emojis),
    Jap(Japanese),
    LargeLetters(LargeLetter),
    Moon(Moon),
    Num(Numbers),
    NumberedBalls(NumberedBalls),
    NumberedCubes(NumberedCubes),
    Plants(Plant),
    Smile(Smile),
    Shapes(Shape),
}

impl Characters {
    pub fn width(&self) -> u16 {
        match self {
            Self::All(_) => CharWidth::Double.width(),
            Self::Alphalow(_) => CharWidth::Single.width(),
            Self::Alphaup(_) => CharWidth::Single.width(),
            Self::Arrow(_) => CharWidth::Double.width(),
            Self::Bin(_) => CharWidth::Single.width(),
            Self::Cards(_) => CharWidth::Double.width(),
            Self::Clock(_) => CharWidth::Double.width(),
            Self::Crab(_) => CharWidth::Double.width(),
            Self::Dominosh(_) => CharWidth::Double.width(),
            Self::Dominosv(_) => CharWidth::Single.width(),
            Self::Earth(_) => CharWidth::Double.width(),
            Self::Emojis(_) => CharWidth::Double.width(),
            Self::Jap(_) => CharWidth::Single.width(),
            Self::LargeLetters(_) => CharWidth::Double.width(),
            Self::Moon(_) => CharWidth::Double.width(),
            Self::Num(_) => CharWidth::Single.width(),
            Self::NumberedBalls(_) => CharWidth::Double.width(),
            Self::NumberedCubes(_) => CharWidth::Double.width(),
            Self::Plants(_) => CharWidth::Double.width(),
            Self::Smile(_) => CharWidth::Double.width(),
            Self::Shapes(_) => CharWidth::Double.width(),
        }
    }

    pub fn as_vec_u32(&self) -> Vec<u32> {
        match self {
            Self::All(c) => c.as_vec_u32(),
            Self::Alphalow(c) => c.as_vec_u32(),
            Self::Alphaup(c) => c.as_vec_u32(),
            Self::Arrow(c) => c.as_vec_u32(),
            Self::Bin(c) => c.as_vec_u32(),
            Self::Cards(c) => c.as_vec_u32(),
            Self::Clock(c) => c.as_vec_u32(),
            Self::Crab(c) => c.as_vec_u32(),
            Self::Dominosh(c) => c.as_vec_u32(),
            Self::Dominosv(c) => c.as_vec_u32(),
            Self::Earth(c) => c.as_vec_u32(),
            Self::Emojis(c) => c.as_vec_u32(),
            Self::Jap(c) => c.as_vec_u32(),
            Self::LargeLetters(c) => c.as_vec_u32(),
            Self::Moon(c) => c.as_vec_u32(),
            Self::Num(c) => c.as_vec_u32(),
            Self::NumberedBalls(c) => c.as_vec_u32(),
            Self::NumberedCubes(c) => c.as_vec_u32(),
            Self::Plants(c) => c.as_vec_u32(),
            Self::Smile(c) => c.as_vec_u32(),
            Self::Shapes(c) => c.as_vec_u32(),
        }
    }
}

impl From<ezemoji::AllEmojis> for Characters {
    fn from(e: ezemoji::AllEmojis) -> Self {
        Self::All(e)
    }
}

impl From<ezemoji::LowerAlpha> for Characters {
    fn from(e: ezemoji::LowerAlpha) -> Self {
        Self::Alphalow(e)
    }
}

impl From<ezemoji::UpperAlpha> for Characters {
    fn from(e: ezemoji::UpperAlpha) -> Self {
        Self::Alphaup(e)
    }
}

impl From<ezemoji::Arrow> for Characters {
    fn from(e: ezemoji::Arrow) -> Self {
        Self::Arrow(e)
    }
}

impl From<ezemoji::Bin> for Characters {
    fn from(e: ezemoji::Bin) -> Self {
        Self::Bin(e)
    }
}

impl From<ezemoji::Cards> for Characters {
    fn from(e: ezemoji::Cards) -> Self {
        Self::Cards(e)
    }
}

impl From<ezemoji::Clock> for Characters {
    fn from(e: ezemoji::Clock) -> Self {
        Self::Clock(e)
    }
}

impl From<ezemoji::Crab> for Characters {
    fn from(e: ezemoji::Crab) -> Self {
        Self::Crab(e)
    }
}

impl From<ezemoji::HorizontalDominos> for Characters {
    fn from(e: ezemoji::HorizontalDominos) -> Self {
        Self::Dominosh(e)
    }
}

impl From<ezemoji::VerticalDominos> for Characters {
    fn from(e: ezemoji::VerticalDominos) -> Self {
        Self::Dominosv(e)
    }
}

impl From<ezemoji::Earth> for Characters {
    fn from(e: ezemoji::Earth) -> Self {
        Self::Earth(e)
    }
}

impl From<ezemoji::Emojis> for Characters {
    fn from(e: ezemoji::Emojis) -> Self {
        Self::Emojis(e)
    }
}

impl From<ezemoji::Japanese> for Characters {
    fn from(e: ezemoji::Japanese) -> Self {
        Self::Jap(e)
    }
}

impl From<ezemoji::LargeLetter> for Characters {
    fn from(e: ezemoji::LargeLetter) -> Self {
        Self::LargeLetters(e)
    }
}

impl From<ezemoji::Moon> for Characters {
    fn from(e: ezemoji::Moon) -> Self {
        Self::Moon(e)
    }
}

impl From<ezemoji::Numbers> for Characters {
    fn from(e: ezemoji::Numbers) -> Self {
        Self::Num(e)
    }
}

impl From<ezemoji::NumberedBalls> for Characters {
    fn from(e: ezemoji::NumberedBalls) -> Self {
        Self::NumberedBalls(e)
    }
}

impl From<ezemoji::NumberedCubes> for Characters {
    fn from(e: ezemoji::NumberedCubes) -> Self {
        Self::NumberedCubes(e)
    }
}

impl From<ezemoji::Plant> for Characters {
    fn from(e: ezemoji::Plant) -> Self {
        Self::Plants(e)
    }
}

impl From<ezemoji::Smile> for Characters {
    fn from(e: ezemoji::Smile) -> Self {
        Self::Smile(e)
    }
}

impl From<ezemoji::Shape> for Characters {
    fn from(e: ezemoji::Shape) -> Self {
        Self::Shapes(e)
    }
}
