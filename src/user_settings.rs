use super::*;

#[derive(Debug, Clone)]
pub struct UserSettings {
    pub rain_color: (u8, u8, u8),
    pub head_color: (u8, u8, u8),
    pub group: Characters,
    pub shading: bool,
    pub speed: (u64, u64),
    pub direction: Direction,
}

impl UserSettings {
    pub fn new(
        rain_color: (u8, u8, u8),
        head_color: (u8, u8, u8),
        group: Characters,
        shading: bool,
        speed: (u64, u64),
        direction: Direction,
    ) -> Self {
        Self {
            rain_color,
            head_color,
            group,
            shading,
            speed,
            direction,
        }
    }
}
