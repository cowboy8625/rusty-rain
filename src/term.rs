use crate::{cursor, queue, style, terminal, BufWriter, Rain, Result, Stdout};

pub fn clear(w: &mut BufWriter<Stdout>) -> Result<()> {
    queue!(w, terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

pub fn draw(w: &mut BufWriter<Stdout>, rain: &Rain, spacing: u16) -> Result<()> {
    let (mut chr, mut loc, mut len, mut clr);
    let height = rain.height();
    for x in rain.queue.iter() {
        chr = &rain.charaters[*x];
        loc = &rain.locations[*x];
        len = &rain.length[*x];
        clr = &rain.colors[*x];

        let start = loc.saturating_sub(*len).clamp(0, chr.len());
        let end = (loc + 1).clamp(1, chr.len());
        let slice = chr[start..end].iter();

        let cstart = if loc > len {
            clr.len() - slice.len()
        } else {
            0
        };

        let color = &clr[cstart..clr.len()];

        for (y, ch) in slice.rev().enumerate() {
            queue!(
                w,
                cursor::MoveTo(*x as u16 * spacing, (*loc.min(&height) - y) as u16),
                style::SetForegroundColor(color[y]),
                style::Print(ch),
            )?;
        }
        if loc >= len {
            queue!(
                w,
                cursor::MoveTo(*x as u16 * spacing, loc.saturating_sub(*len) as u16),
                style::Print(" ".repeat(spacing as usize)),
            )?;
        }
    }
    Ok(())
}
