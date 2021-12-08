use crate::{cursor, queue, style, terminal, Rain, Result, Stdout};

pub fn clear(w: &mut Stdout) -> Result<()> {
    queue!(w, terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

// TODO: Clean this crap up
pub fn draw(w: &mut Stdout, rain: &Rain, spacing: u16) -> Result<()> {
    let (mut chr, mut col, mut len, mut clr);
    let height = rain.height();
    for row in rain.queue.iter() {
        chr = &rain.charaters[*row];
        col = &rain.locations[*row];
        len = &rain.length[*row];
        clr = &rain.colors[*row];

        let start = col.saturating_sub(*len).clamp(0, chr.len());
        let end = (col + 1).clamp(1, chr.len());
        let slice = chr[start..end].iter();

        let cstart = if col > len {
            clr.len() - slice.len()
        } else {
            0
        };

        let color = &clr[cstart..clr.len()];

        for (y, ch) in slice.rev().enumerate() {
            queue!(
                w,
                cursor::MoveTo(*row as u16 * spacing, (*col.min(&height) - y) as u16),
                style::SetForegroundColor(color[y]),
                style::Print(ch),
            )?;
        }
        if col >= len {
            queue!(
                w,
                cursor::MoveTo(*row as u16 * spacing, col.saturating_sub(*len) as u16),
                style::Print(" ".repeat(spacing as usize)),
            )?;
        }
    }
    Ok(())
}
