use clap::{App, Arg};
use crossterm::{cursor, event, execute, queue, style, terminal, Result};
use rand::{thread_rng, Rng};

use std::char;
// use std::cmp::{max, min};
 use std::io::BufWriter;
use std::io::{stdout, Stdout, Write};
use std::time::{Duration, Instant};

const VERSION: &str = "0.0.1";
const AUTHOR: &str = "Cowboy8625 <cowboy8625@protonmail.com>";
const ABOUT: &str = "A terminal program the makes all your friends think you are a hacker.";

fn cargs() -> ((u8, u8, u8), (u32, u32), bool) {
    let matches = App::new("Matrix Rain")
        .version(VERSION)
        .author(AUTHOR)
        .about(ABOUT)
        .arg(
            Arg::with_name("red")
                .short("r")
                .long("red")
                .help("Set color of characters RED value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("green")
                .short("g")
                .long("green")
                .help("Set color of characters GREEN value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("blue")
                .short("b")
                .long("blue")
                .help("set color of characters BLUE value")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("characters")
                .short("c")
                .long("chars")
                .help("Set what kind of characters are printed as rain")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("shade")
                .short("s")
                .long("shade")
                .help("Set Rain shading to fade or stay constant")
                .takes_value(true),
        )
        .get_matches();

    let r = matches
        .value_of("red")
        .and_then(|v| {
            v.parse::<u8>()
                .map_err(|_| println!("This is not a Valid Red Option"))
                .ok()
        })
        .unwrap_or(0);
    let g = matches
        .value_of("green")
        .and_then(|v| {
            v.parse::<u8>()
                .map_err(|_| println!("This is not a Valid Green Option"))
                .ok()
        })
        .unwrap_or(255);
    let b = matches
        .value_of("blue")
        .and_then(|v| {
            v.parse::<u8>()
                .map_err(|_| println!("This is not a Valid Blue Option"))
                .ok()
        })
        .unwrap_or(0);

    let characters = match matches.value_of("characters").unwrap_or("01") {
        "jap" => (65382, 65437),
        "01" | _ => (48, 50),
    };

    let shading = match matches.value_of("shade").unwrap_or("0") {
        "1" => true,
        "0" | _ => false,
    };

    ((r, g, b), characters, shading)
}

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
    (0..height+1).map(|_| ran_ch(characters)).collect()
}

fn gen_charater_vecs(width: usize, height: u16, characters: (u32, u32)) -> Vec<Vec<char>>{
    let mut ch = Vec::new();
    for _ in 0..width {
        ch.push(create_drop_chars(height, characters));
    }
    ch
}

fn gen_times(width:usize) -> Vec<(Instant, Duration)> {
    let now = Instant::now();
    let mut times = Vec::new();
    let mut rng = thread_rng();
    for _ in 0..width {
        times.push(
            (now,
            Duration::from_millis(rng.gen_range(40..400))
            )
            );
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

fn get_visable_rain<'a>(
    rain: &'a [char], loc: usize, len: usize
    ) -> (&'a [char], char) {
    let start = clamp(usub(loc, len), rain.len(), 0);
    let end = clamp(loc+1, rain.len(), 1);
    (&rain[start..end], if loc > len {' '} else {'\0'})
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
    let (mut chr, mut loc, mut len);
    for x in rain.queue.iter() {
        chr = &rain.charaters[*x];
        loc = &rain.locations[*x];
        len = &rain.length[*x];
        let (slice, tail) = get_visable_rain(&chr, *loc, *len);
        for (y, c) in slice.iter().rev().chain(vec![tail, tail].iter()).enumerate() {
            queue!(
                w,
                cursor::MoveTo(*x as u16, (loc - y) as u16),
                style::Print(c),
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

fn update_reset(rain: &mut Rain) {
    rain.reset.clear();
    let h = rain.height();
    for (i, l) in rain.locations.iter().enumerate() {
        if l > &h {
            rain.reset.push(i);
        }
    }
}

fn reset(rain: &mut Rain, characters: (u32, u32)) {
    let mut rng = thread_rng();
    let reset = &rain.reset;
    let h16 = rain.height() as u16;
    let hsize = rain.height();
    let now = Instant::now();
    for i in reset.iter() {
        // let length = vec![rng.gen_range(4..h - 2); w];
        // let _colors = Vec::with_capacity(w);
        // let time = gen_times(w);
        rain.charaters[*i] = create_drop_chars(h16, characters);
        rain.locations[*i] = 0;
        rain.length[*i] = rng.gen_range(4..hsize - 10);
        // rain._colors
        rain.time[*i] = (now, Duration::from_millis(rng.gen_range(40..400)));
    }
}

struct Rain {
    charaters: Vec<Vec<char>>,
    locations: Vec<usize>,
    length   : Vec<usize>,
    _colors  : Vec<Vec<style::Color>>,
    time     : Vec<(Instant, Duration)>,
    queue    : Vec<usize>,
    reset    : Vec<usize>,
}

impl Rain {
    fn new(width: u16, height: u16, _color: style::Color, characters: (u32, u32), _shading: bool) -> Self {
        let w = width as usize;
        let h = height as usize;
        let charaters = gen_charater_vecs(w, height, characters);
        let locations = vec![0; w];
        let length = gen_lengths(w, h);
        let _colors = Vec::with_capacity(w);
        let time = gen_times(w);
        let queue = Vec::with_capacity(w);
        let reset = Vec::with_capacity(w);
        Self {
            charaters,
            locations,
            length,
            _colors,
            time,
            queue,
            reset,
        }
    }

    fn _width(&self) -> usize {
        self.charaters[0].len()
    }

    fn height(&self) -> usize {
        self.charaters.len()
    }
}

fn main() -> Result<()> {
    let mut stdout = BufWriter::with_capacity(8_192, stdout());
    let ((r, g, b), characters, shading) = cargs();
    let (width, height) = terminal::size()?;
    let color = style::Color::Rgb { r, g, b };
    let mut rain = Rain::new(width, height, color, characters, shading);

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    loop {
        if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;
            if event == event::Event::Key(event::KeyCode::Esc.into()) {
                break;
            }
        }
        update_reset(&mut rain);
        reset(&mut rain, characters);
        update_queue(&mut rain);
        draw(&mut stdout, &rain)?;
        update_locations(&mut rain);
        stdout.flush()?;
    }

    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
