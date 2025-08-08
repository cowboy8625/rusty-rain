use crate::{style, Characters, Rng};
use std::char;
use std::time::{Duration, Instant};

/// Generates a single column of characters for rain drops.
pub fn create_drop_chars(height: u16, group: &Characters) -> Vec<char> {
    let mut rng = rand::rng();
    let chars_u32 = group.as_vec_u32();
    (0..=height)
        .map(|_| {
            chars_u32
                .get(rng.random_range(0..chars_u32.len()))
                .and_then(|&c| char::from_u32(c))
                .unwrap_or('#') // fallback character
        })
        .collect()
}

/// Generates vectors of characters for all columns.
pub fn character_vecs(width: usize, height: u16, group: &Characters) -> Vec<Vec<char>> {
    (0..width)
        .map(|_| create_drop_chars(height, group))
        .collect()
}

/// Returns a color generating function based on whether shading is enabled.
///
/// This function creates a closure that generates color gradients or flat colors.
pub fn color_function(shading: bool) -> fn(style::Color, style::Color, u8) -> Vec<style::Color> {
    match shading {
        true => |bc: style::Color, head: style::Color, length: u8| {
            let mut colors = Vec::with_capacity(length as usize);
            if let style::Color::Rgb { r, g, b } = bc {
                let nr = r / length;
                let ng = g / length;
                let nb = b / length;
                for i in 0..length {
                    colors.push((nr * i, ng * i, nb * i).into());
                }
                colors.push(head);
                colors.reverse();
            }
            colors
        },
        false => |bc: style::Color, head: style::Color, length: u8| {
            let mut colors = Vec::with_capacity(length as usize);
            colors.push(head);
            if let style::Color::Rgb { r, g, b } = bc {
                for _ in 0..length {
                    colors.push((r, g, b).into());
                }
            }
            colors
        },
    }
}

/// Generates a vector of `(Instant, Duration)` tuples for timing each column.
pub fn times(width: usize, (fastest, slowest): (u64, u64)) -> Vec<(Instant, Duration)> {
    let now = Instant::now();
    let mut rng = rand::rng();
    (0..width.max(1))
        .map(|_| {
            (
                now,
                Duration::from_millis(rng.random_range(fastest..slowest)),
            )
        })
        .collect()
}

/// Generates the visible lengths of each column for rain drops.
pub fn lengths(width: usize, height: usize) -> Vec<usize> {
    let mut rng = rand::rng();
    (0..width.max(1))
        .map(|_| rng.random_range(4..(height.saturating_sub(10)).max(5)))
        .collect()
}

/// Generates all the color vectors for rain columns.
pub fn colors<F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>>(
    create_color: F,
    head: (u8, u8, u8),
    lengths: &[usize],
    bc: style::Color,
) -> Vec<Vec<style::Color>> {
    lengths
        .iter()
        .map(|&l| create_color(bc, head.into(), l as u8))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style;

    #[test]
    fn test_create_drop_chars() {
        let group = Characters::Alphalow;
        let height = 10;
        let chars = create_drop_chars(height, &group);
        assert_eq!(chars.len(), (height + 1) as usize);

        for &ch in &chars {
            assert!(('a'..='z').contains(&ch));
        }
    }

    #[test]
    fn test_character_vecs() {
        let width = 5;
        let height = 10;
        let group = Characters::Num;
        let chars_vec = character_vecs(width, height, &group);
        assert_eq!(chars_vec.len(), width);

        for col in &chars_vec {
            assert_eq!(col.len(), (height + 1) as usize);
            for &ch in col {
                assert!(('0'..='9').contains(&ch));
            }
        }
    }

    #[test]
    fn test_color_function_shading_true() {
        let create_color = color_function(true);
        let base_color = style::Color::Rgb {
            r: 100,
            g: 100,
            b: 100,
        };
        let head = style::Color::Rgb { r: 0, g: 0, b: 0 };
        let length = 10;
        let colors = create_color(base_color, head, length);
        assert_eq!(colors.len(), length as usize + 1);
        assert_eq!(colors.last(), Some(&head));
    }

    #[test]
    fn test_color_function_shading_false() {
        let create_color = color_function(false);
        let base_color = style::Color::Rgb {
            r: 50,
            g: 50,
            b: 50,
        };
        let head = style::Color::Rgb {
            r: 150,
            g: 150,
            b: 150,
        };
        let length = 10;
        let colors = create_color(base_color, head, length);
        assert_eq!(colors.len(), length as usize + 1);
        assert_eq!(colors.first(), Some(&head));
    }

    #[test]
    fn test_lengths_and_times() {
        let width = 10;
        let height = 20;

        let lens = lengths(width, height);
        assert_eq!(lens.len(), width);
        for &l in &lens {
            assert!(l >= 4);
            assert!(l <= (height.saturating_sub(10)).max(5));
        }

        let (fastest, slowest) = (100, 200);
        let times_vec = times(width, (fastest, slowest));
        assert_eq!(times_vec.len(), width);
        for &(_, dur) in &times_vec {
            assert!(dur.as_millis() >= fastest.into());
            assert!(dur.as_millis() < slowest.into());
        }
    }

    #[test]
    fn test_colors_produces_expected_length() {
        let create_color = color_function(false);
        let lengths = vec![3, 5, 7, 2, 1];
        let base_color = style::Color::Rgb {
            r: 10,
            g: 20,
            b: 30,
        };
        let head = (100, 100, 100);
        let colors_vec = colors(create_color, head, &lengths, base_color);

        assert_eq!(colors_vec.len(), lengths.len());
        for (color_vec, &len) in colors_vec.iter().zip(lengths.iter()) {
            assert_eq!(color_vec.len(), len as usize + 1);
        }
    }
}
