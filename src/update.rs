use crate::{create_drop_chars, style, thread_rng, Rain, Rng, UserSettings};
use itertools::izip;
use std::time::{Duration, Instant};

pub fn update(rain: &mut Rain) {
    rain.queue.clear();
    let now = Instant::now();
    for (idx, ((time, delay), location)) in izip!(&mut rain.time, &mut rain.locations).enumerate() {
        if *time <= now {
            *time += *delay;
            *location += 1;
            rain.queue.push(idx);
        }
    }
}

pub fn reset<F>(create_color: F, rain: &mut Rain, us: &UserSettings)
where
    F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>,
{
    let mut rng = thread_rng();
    let h16 = rain.height() as u16;
    let now = Instant::now();
    for i in rain.queue.iter() {
        if rain.locations[*i] > rain.height() + rain.length[*i] {
            rain.charaters[*i] = create_drop_chars(h16, &us.group);
            rain.locations[*i] = 0;
            rain.length[*i] = rng.gen_range(4..rain.height() - 10);
            rain.colors[*i] = create_color(
                us.rain_color.into(),
                us.head_color.into(),
                rain.length[*i] as u8,
            );
            rain.time[*i] = (
                now,
                Duration::from_millis(rng.gen_range(us.speed.0..us.speed.1)),
            );
        }
    }
}
