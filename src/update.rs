use crate::{create_drop_chars, style, thread_rng, Rain, Rng, UserSettings};
use std::time::{Duration, Instant};

pub fn update_queue(rain: &mut Rain) {
    rain.queue.clear();
    let now = Instant::now();
    for (i, (t, d)) in rain.time.iter_mut().enumerate() {
        if *t <= now {
            *t += *d;
            rain.queue.push(i);
        }
    }
}

pub fn update_locations(rain: &mut Rain) {
    let queue = &rain.queue;
    for i in queue.iter() {
        rain.locations[*i] += 1;
    }
}

pub fn reset<F>(create_color: F, rain: &mut Rain, us: &UserSettings, height: usize)
where
    F: Fn(style::Color, style::Color, u8) -> Vec<style::Color>,
{
    // assert_eq!(height, rain.height());
    let mut rng = thread_rng();
    let h16 = height as u16;
    let now = Instant::now();
    for i in rain.queue.iter() {
        if rain.locations[*i] > height + rain.length[*i] {
            rain.charaters[*i] = create_drop_chars(h16, &us.group);
            rain.locations[*i] = 0;
            rain.length[*i] = rng.gen_range(4..height - 10);
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
