use crate::{style, thread_rng, Characters, Rng};
use std::char;
use std::time::{Duration, Instant};

/// Generates a single column of Characters.
pub fn create_drop_chars(height: u16, group: &Characters) -> Vec<char> {
    let g = group.as_vec_u32();
    (0..height + 1)
        .map(|_| char::from_u32(g[thread_rng().gen_range(0..g.len())]).unwrap_or('#'))
        .collect()
}

/// Generates all Characters in columns.
pub fn charater_vecs(width: usize, height: u16, group: &Characters) -> Vec<Vec<char>> {
    (0..width)
        .map(|_| create_drop_chars(height, group))
        .collect()
}

/// Generates the color function on startup to remove branching if statements from code.
pub fn color_function(shading: bool) -> fn(style::Color, style::Color, u8) -> Vec<style::Color> {
    // This Creates a closure off of the args
    // given to the program at start that will crates the colors for the rain
    match shading {
        // Creates shading colors
        true => |bc: style::Color, head: style::Color, length: u8| {
            let mut c: Vec<style::Color> = Vec::with_capacity(length as usize);
            let (mut nr, mut ng, mut nb);
            if let style::Color::Rgb { r, g, b } = bc {
                for i in 0..length {
                    nr = r / length;
                    ng = g / length;
                    nb = b / length;
                    c.push((nr * i, ng * i, nb * i).into());
                }
                c.push(head);
                c.reverse();
            }
            c
        },
        // creates with out color
        _ => |bc: style::Color, head: style::Color, length: u8| {
            let mut c: Vec<style::Color> = Vec::with_capacity(length as usize);
            c.push(head);
            if let style::Color::Rgb { r, g, b } = bc {
                for _ in 0..length {
                    c.push((r, g, b).into());
                }
            }
            c
        },
    }
}

// TODO: I feel like slowest and fastest are labeled wrong.........
/// Generates Timing for rain to fall. AKA the speed of the rain fall.
pub fn times(width: usize, (slowest, fastest): (u64, u64)) -> Vec<(Instant, Duration)> {
    let now = Instant::now();
    let mut rng = thread_rng();
    (0..width)
        .map(|_| (now, Duration::from_millis(rng.gen_range(slowest..fastest))))
        .collect()
}

/// Generates the visable length of each column.
pub fn lengths(width: usize, height: usize) -> Vec<usize> {
    let mut rng = thread_rng();
    (0..width).map(|_| rng.gen_range(4..height - 10)).collect()
}

/// Uses Generates function to create all the color of the Rain/Characters.
pub fn colors<F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>>(
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
