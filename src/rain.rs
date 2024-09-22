use crate::cli::Cli;
use crate::{gen, style};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Rain {
    pub charaters: Vec<Vec<char>>,
    pub locations: Vec<usize>,
    pub length: Vec<usize>,
    pub colors: Vec<Vec<style::Color>>,
    pub time: Vec<(Instant, Duration)>,
    pub queue: Vec<usize>,
    pub height: u16,
}

impl Rain {
    pub fn new<F>(create_color: F, width: u16, height: u16, settings: &Cli) -> Self
    where
        F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>,
    {
        let w = (width / settings.chars.width()) as usize;
        let h = height as usize;
        let charaters = gen::charater_vecs(w, height, &settings.chars);
        let locations = vec![0; w];
        let length = gen::lengths(w, h);
        let colors = gen::colors(
            create_color,
            settings.head_color(),
            w,
            &length,
            settings.rain_color().into(),
        );
        let time = gen::times(w, settings.speed());
        let queue = Vec::with_capacity(w);
        Self {
            charaters,
            locations,
            length,
            colors,
            time,
            queue,
            height,
        }
    }
}
