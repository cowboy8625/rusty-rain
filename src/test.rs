use crate::cli::CharGroupKind;

use super::{Parser, Rain, cli::Cli};
use ezemoji::CharGroup;
use std::fmt::Write;

struct SnapshotOptions {
    label: String,
    cycles: usize,
    width: usize,
    height: usize,
    direction: super::Direction,
    group: CharGroup,
}

impl Default for SnapshotOptions {
    fn default() -> Self {
        Self {
            label: String::new(),
            cycles: 25,
            width: 40,
            height: 20,
            direction: super::Direction::Down,
            group: CharGroup::BIN,
        }
    }
}

fn display<const N: usize>(id: usize, window: &mut String, rain: &Rain<N>) {
    let width = rain.width;
    let height = rain.height;
    let char_width = rain.group.0.width() as usize;
    let id_str = format!("{:02X}", id);
    write!(
        window,
        "{:-^width$}\n",
        id_str,
        width = width * char_width + 5
    )
    .unwrap();
    for (i, chunk) in rain.screen_buffer.chunks(width).enumerate() {
        write!(window, "{:02X} |", i).unwrap();
        write!(
            window,
            "{}|",
            &chunk
                .iter()
                .map(|c| c.display(char_width))
                .collect::<String>()
        )
        .unwrap();
        if i == height {
            continue;
        }
        write!(window, "\n").unwrap();
    }
}

fn set_up_snapshot(options: SnapshotOptions) {
    let SnapshotOptions {
        label,
        cycles,
        width,
        height,
        group,
        direction,
    } = options;
    let mut cli = Cli::parse();
    cli.group = CharGroupKind(group);
    cli.direction = direction;
    let mut rain = Rain::<1024>::new(width, height, &cli);
    let mut window = String::new();
    for id in 0..cycles {
        rain.update();
        rain.update_screen_buffer().unwrap();
        display(id, &mut window, &rain);

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    insta::assert_snapshot!(label, window);
}

macro_rules! snapshot {
    ($name:ident) => {
        #[test]
        fn $name() {
            set_up_snapshot(SnapshotOptions {
                label: stringify!($name).replace("test_", "").to_string(),
                ..Default::default()
            });
        }
    };
    ($name:ident, $cycles:expr,$width:expr, $height:expr, $direction:expr, $group:expr) => {
        #[test]
        fn $name() {
            set_up_snapshot(SnapshotOptions {
                label: stringify!($name).replace("test_", "").to_string(),
                cycles: $cycles,
                width: $width,
                height: $height,
                direction: $direction,
                group: $group,
            });
        }
    };
}

snapshot!(test_screen_buffer);
snapshot!(
    test_screen_buffer_direction_right_emoji_moon_double_width,
    50,
    32,
    10,
    super::Direction::Right,
    CharGroup::MOON
);
snapshot!(
    test_screen_buffer_direction_left_emoji_crab_double_width,
    50,
    32,
    10,
    super::Direction::Left,
    CharGroup::CRAB
);

#[test]
fn test_gen_shade_color() {
    use super::{Color, gen_shade_color};
    use pretty_assertions::assert_eq;
    let bc = Color::Rgb { r: 0, g: 255, b: 0 };
    let length = 10;

    let colors = gen_shade_color(bc, length);

    assert_eq!(colors.len(), length as usize);
    assert_eq!(colors.first(), Some(&Color::Rgb { r: 0, g: 225, b: 0 }));
    assert_eq!(colors.last(), Some(&Color::Rgb { r: 0, g: 0, b: 0 }));
}
