use super::{cli::Cli, Parser, Rain};
use std::fmt::Write;

struct SnapshotOptions {
    label: String,
    cycles: usize,
    width: usize,
    height: usize,
    direction: super::Direction,
    chars: super::Characters,
}

impl Default for SnapshotOptions {
    fn default() -> Self {
        Self {
            label: String::new(),
            cycles: 25,
            width: 40,
            height: 20,
            direction: super::Direction::Down,
            chars: super::Characters::Bin,
        }
    }
}

fn display<const N: usize>(id: usize, window: &mut String, rain: &Rain<N>) {
    let width = rain.width;
    let height = rain.height;
    let char_width = rain.group.width() as usize;
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
        chars,
        direction,
    } = options;
    let mut cli = Cli::parse();
    cli.chars = chars;
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
    ($name:ident, $cycles:expr,$width:expr, $height:expr, $direction:expr, $chars:expr) => {
        #[test]
        fn $name() {
            set_up_snapshot(SnapshotOptions {
                label: stringify!($name).replace("test_", "").to_string(),
                cycles: $cycles,
                width: $width,
                height: $height,
                direction: $direction,
                chars: $chars,
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
    super::Characters::Moon
);
snapshot!(
    test_screen_buffer_direction_left_emoji_crab_double_width,
    50,
    32,
    10,
    super::Direction::Left,
    super::Characters::Crab
);

#[test]
fn test_gen_shade_color() {
    use super::{gen_shade_color, Color};
    use pretty_assertions::assert_eq;
    let bc = Color::Rgb { r: 0, g: 255, b: 0 };
    let length = 10;

    let colors = gen_shade_color(bc, length);

    assert_eq!(colors.len(), length as usize);
    assert_eq!(colors.first(), Some(&Color::Rgb { r: 0, g: 225, b: 0 }));
    assert_eq!(colors.last(), Some(&Color::Rgb { r: 0, g: 0, b: 0 }));
}
