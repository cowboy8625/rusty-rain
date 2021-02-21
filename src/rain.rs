use std::time::{Instant, Duration};
use crate::{style, gen_charater_vecs, gen_lengths, gen_colors, gen_times};

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
    pub fn new<F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>>(
        create_color: F,
        head: (u8, u8, u8),
        width: u16,
        height: u16,
        base_color: style::Color,
        characters: (u32, u32),
        spacing: u16,
    ) -> Self {
        let w = (width / spacing) as usize;
        let h = height as usize;
        let charaters = gen_charater_vecs(w, height, characters);
        let locations = vec![0; w];
        let length = gen_lengths(w, h);
        let colors = gen_colors(create_color, head, w, &length, base_color);
        let time = gen_times(w);
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
