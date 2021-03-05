use crate::{style, thread_rng, Rng, MAXSPEED, MINSPEED};
use std::char;
use std::time::{Duration, Instant};

pub fn ran_ch((min, max): (u32, u32)) -> char {
    let c: u32 = thread_rng().gen_range(min..max);
    char::from_u32(c).unwrap()
}

pub fn create_drop_chars(height: u16, characters: &[u32]) -> Vec<char> {
    (0..height + 1)
        .map(|_| {
            char::from_u32(characters[thread_rng().gen_range(0..characters.len())]).unwrap_or('#')
        })
        .collect()
}

pub fn gen_charater_vecs(width: usize, height: u16, characters: &[u32]) -> Vec<Vec<char>> {
    let mut ch = Vec::new();
    for _ in 0..width {
        ch.push(create_drop_chars(height, characters));
    }
    ch
}

pub fn gen_times(width: usize) -> Vec<(Instant, Duration)> {
    let now = Instant::now();
    let mut times = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..width {
        times.push((
            now,
            Duration::from_millis(rng.gen_range(MAXSPEED..MINSPEED)),
        ));
    }
    times
}

pub fn gen_lengths(width: usize, height: usize) -> Vec<usize> {
    let mut len = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..width {
        len.push(rng.gen_range(4..height - 10));
    }
    len
}

pub fn gen_colors<F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>>(
    create_color: F,
    head: (u8, u8, u8),
    width: usize,
    length: &[usize],
    bc: style::Color,
) -> Vec<Vec<style::Color>> {
    let mut colors = Vec::with_capacity(width);
    for l in length.iter() {
        colors.push(create_color(bc, head.into(), *l as u8));
    }
    colors
}
