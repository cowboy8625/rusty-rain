use crate::{gen_charater_vecs, gen_colors, gen_lengths, gen_times, style, UserSettings};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Rain {
    pub charaters: Vec<Vec<char>>,
    pub locations: Vec<usize>,
    pub length: Vec<usize>,
    pub colors: Vec<Vec<style::Color>>,
    pub time: Vec<(Instant, Duration)>,
    pub queue: Vec<usize>,
}

impl Rain {
    pub fn new<F>(
        create_color: F,
        width: u16,
        height: u16,
        us: &UserSettings,
    ) -> Self
        where
        F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>,
    {
        let w = (width / &us.spacing.value()) as usize;
        let h = height as usize;
        let charaters = gen_charater_vecs(w, height, &us.group);
        let locations = vec![0; w];
        let length = gen_lengths(w, h);
        let colors = gen_colors(
            create_color,
            us.head_color,
            w,
            &length,
            us.rain_color.into(),
        );
        let time = gen_times(w, us.speed);
        let queue = Vec::with_capacity(w);
        Self {
            charaters,
            locations,
            length,
            colors,
            time,
            queue,
        }
    }

    pub fn height(&self) -> usize {
        self.charaters[0].len() - 1
    }
}
