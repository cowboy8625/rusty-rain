use std::io::BufWriter;

use crate::{cursor, queue, style, terminal, Direction, Rain, Stdout};

pub fn clear(w: &mut BufWriter<Stdout>) -> std::io::Result<()> {
    queue!(w, terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

/// Converts logical rain coordinates to terminal cursor positions based on direction.
fn calc_position(direction: &Direction, x: u16, y: u16, offset: u16) -> cursor::MoveTo {
    use Direction::*;
    match direction {
        Down => cursor::MoveTo(x, y),
        Up => cursor::MoveTo(x, offset - y),
        Right => cursor::MoveTo(y, x),
        Left => cursor::MoveTo(offset - y, x),
    }
}

/// Draws one column of rain at the given index.
fn draw_column(
    w: &mut BufWriter<Stdout>,
    rain: &Rain,
    row: usize,
    spacing: u16,
    offset: u16,
    direction: &Direction,
) -> std::io::Result<()> {
    let characters = &rain.characters[row];
    let col = rain.locations[row];
    let len = rain.length[row];
    let colors = &rain.colors[row];

    let height = rain.height as usize;

    let start = col.saturating_sub(len).clamp(0, characters.len());
    let end = (col + 1).clamp(1, characters.len());
    let slice = &characters[start..end];

    let cstart = if col > len {
        colors.len().saturating_sub(slice.len())
    } else {
        0
    };
    let color_slice = &colors[cstart..];

    for (y, (&ch, &color)) in slice.iter().rev().zip(color_slice.iter()).enumerate() {
        queue!(
            w,
            calc_position(
                direction,
                row as u16 * spacing,
                (col.min(height) - y) as u16,
                offset
            ),
            style::SetForegroundColor(color),
            style::Print(ch),
        )?;
    }

    // Clear the old tail character if the rain has moved past its length
    if col >= len {
        queue!(
            w,
            calc_position(
                direction,
                row as u16 * spacing,
                col.saturating_sub(len) as u16,
                offset
            ),
            style::Print(" ".repeat(spacing as usize)),
        )?;
    }

    Ok(())
}

pub fn draw(
    w: &mut BufWriter<Stdout>,
    rain: &Rain,
    spacing: u16,
    direction: &Direction,
) -> std::io::Result<()> {
    let offset = match direction {
        Direction::Down | Direction::Right => 0,
        Direction::Up | Direction::Left => rain.height,
    };

    for &row in &rain.queue {
        draw_column(w, rain, row, spacing, offset, direction)?;
    }

    Ok(())
}
