use crate::{style, cursor, queue, terminal, Result, Rain, BufWriter, Stdout};

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

        let start = clamp(usub(*loc, *len), chr.len(), 0);
        let end = clamp(loc + 1, chr.len(), 1);
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
                cursor::MoveTo(*x as u16 * spacing, (usub(*loc, *len)) as u16),
                style::Print(" ".repeat(spacing as usize)),
            )?;
        }
    }
    Ok(())
}

pub trait Unsigned {}
impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl Unsigned for usize {}


fn usub<T>(x: T, y: T) -> T
where
    T: std::ops::Sub<Output = T> + std::cmp::PartialOrd + From<u8> + Unsigned,
{
    if y > x {
        T::from(0)
    } else {
        x - y
    }
}

fn clamp(x: usize, mx: usize, mn: usize) -> usize {
    std::cmp::max(mn, std::cmp::min(x, mx))
}
