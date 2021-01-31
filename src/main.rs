use crossterm::{cursor, event, execute, queue, style, terminal, Result};
use rand::{thread_rng, Rng};
use std::char;
use std::io::{stdout, BufWriter, Stdout, Write};
use std::time::{Duration, Instant};
mod arguments;
use arguments::cargs;

const MAXSPEED: u64 = 40;
const MINSPEED: u64 = 200;
const VERSION: &str = "0.0.2";
const AUTHOR: &str = "
▞▀▖       ▌        ▞▀▖▞▀▖▞▀▖▛▀▘
▌  ▞▀▖▌  ▌▛▀▖▞▀▖▌ ▌▚▄▘▙▄  ▗▘▙▄
▌ ▖▌ ▌▐▐▐ ▌ ▌▌ ▌▚▄▌▌ ▌▌ ▌▗▘ ▖ ▌
▝▀ ▝▀  ▘▘ ▀▀ ▝▀ ▗▄▘▝▀ ▝▀ ▀▀▘▝▀
Email: cowboy8625@protonmail.com
";
const ABOUT: &str = "A terminal program the makes all your friends think you are a hacker.";

pub trait Unsigned {}
impl Unsigned for u8 {}
impl Unsigned for u16 {}
impl Unsigned for u32 {}
impl Unsigned for u64 {}
impl Unsigned for u128 {}
impl Unsigned for usize {}

fn ran_ch((min, max): (u32, u32)) -> char {
    let c: u32 = thread_rng().gen_range(min..max);
    char::from_u32(c).unwrap()
}

fn create_drop_chars(height: u16, characters: (u32, u32)) -> Vec<char> {
    (0..height + 1).map(|_| ran_ch(characters)).collect()
}

fn gen_charater_vecs(width: usize, height: u16, characters: (u32, u32)) -> Vec<Vec<char>> {
    let mut ch = Vec::new();
    for _ in 0..width {
        ch.push(create_drop_chars(height, characters));
    }
    ch
}

fn gen_times(width: usize) -> Vec<(Instant, Duration)> {
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

fn gen_lengths(width: usize, height: usize) -> Vec<usize> {
    let mut len = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..width {
        len.push(rng.gen_range(4..height - 10));
    }
    len
}

fn gen_colors(width: usize, length: &[usize], bc: style::Color, shading: bool) -> Vec<Vec<style::Color>> {
    let mut colors = Vec::with_capacity(width);
    for l in length.iter() {
        colors.push(create_color(bc, *l as u8, shading));
    }
    colors
}

fn create_color(bc: style::Color, length: u8, shading: bool) -> Vec<style::Color> {
    let mut c = Vec::with_capacity(length as usize);
    match bc {
        style::Color::Rgb { r, g, b } => {
            let (mut nr, mut ng, mut nb);
            if shading {
                for i in 0..length {
                    nr = r / length;
                    ng = g / length;
                    nb = b / length;
                    c.push((nr * i, ng * i, nb * i).into());
                }
            } else {
                for _ in 0..length {
                    c.push((r, g, b).into());
                }
            }
            c.push(style::Color::Rgb { r, g, b });
            c.reverse();
        }
        color => {
            for _ in 0..length {
                c.push(color);
            }
            c.push(color);
        }
    }
    c
}

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

fn update_queue(rain: &mut Rain) {
    rain.queue.clear();
    let now = Instant::now();
    for (i, (t, d)) in rain.time.iter_mut().enumerate() {
        if *t <= now {
            *t += *d;
            rain.queue.push(i);
        }
    }
}

fn draw(w: &mut BufWriter<Stdout>, rain: &Rain) -> Result<()> {
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

        // Fixme
        let cstart = if loc > len {
            clr.len() - slice.len()
        } else {
            0
        };
        let color = &clr[cstart..clr.len()];

        let mut color_idx = 0;
        for (y, ch) in slice.rev().enumerate() {
            queue!(
                w,
                cursor::MoveTo(*x as u16, (*loc.min(&height) - y) as u16),
                style::SetForegroundColor(color[color_idx]),
                style::Print(ch),
            )?;
            color_idx += 1;
        }
        if loc >= len {
            queue!(
                w,
                cursor::MoveTo(*x as u16, (usub(*loc, *len)) as u16),
                style::Print(' '),
            )?;
        }
    }
    Ok(())
}

fn update_locations(rain: &mut Rain) {
    let queue = &rain.queue;
    for i in queue.iter() {
        rain.locations[*i] += 1;
    }
}
fn reset(rain: &mut Rain, characters: (u32, u32), height: usize, bc: style::Color, shading: bool) {
    assert_eq!(height, rain.height());
    let mut rng = thread_rng();
    let h16 = height as u16;
    let now = Instant::now();
    for i in rain.queue.iter() {
        if rain.locations[*i] > height + rain.length[*i] {
            rain.charaters[*i] = create_drop_chars(h16, characters);
            rain.locations[*i] = 0;
            rain.length[*i] = rng.gen_range(4..height - 10);
            rain.colors[*i] = create_color(bc, rain.length[*i] as u8, shading);
            rain.time[*i] = (now, Duration::from_millis(rng.gen_range(40..400)));
        }
    }
}

#[derive(Debug)]
struct Rain {
    charaters: Vec<Vec<char>>,
    locations: Vec<usize>,
    length: Vec<usize>,
    colors: Vec<Vec<style::Color>>,
    time: Vec<(Instant, Duration)>,
    queue: Vec<usize>,
}

impl Rain {
    fn new(
        width: u16,
        height: u16,
        base_color: style::Color,
        characters: (u32, u32),
        shading: bool,
    ) -> Self {
        let w = width as usize;
        let h = height as usize;
        let charaters = gen_charater_vecs(w, height, characters);
        let locations = vec![0; w];
        let length = gen_lengths(w, h);
        let colors = gen_colors(w, &length, base_color, shading);
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

    fn _width(&self) -> usize {
        self.charaters.len()
    }

    fn height(&self) -> usize {
        self.charaters[0].len() - 1
    }
}

fn main() -> Result<()> {
    let mut stdout = BufWriter::with_capacity(8_192, stdout());
    let (color, characters, shading) = cargs();
    let (width, height) = terminal::size()?;
    let h = height as usize;
    let mut rain = Rain::new(width, height, color.into(), characters, shading);

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    loop {
        if event::poll(Duration::from_millis(50))? {
            let event = event::read()?;
            if event == event::Event::Key(event::KeyCode::Esc.into()) {
                break;
            }
        }
        update_queue(&mut rain);
        draw(&mut stdout, &rain)?;
        stdout.flush()?;
        update_locations(&mut rain);
        reset(&mut rain, characters, h, color.into(), shading);
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
