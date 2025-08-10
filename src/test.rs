use std::fmt::Write;
#[test]
fn test_screen_buffer() {
    use super::Rain;
    use crate::cli::Cli;
    use clap::Parser;

    let width = 40;
    let height = 20;
    let mut rain = Rain::<1024>::new(width, height, &Cli::parse());
    let mut window = String::new();
    for id in 0..25 {
        rain.update();
        rain.update_screen_buffer().unwrap();
        write!(&mut window, "---------- {} -------------\n", id).unwrap();

        for (i, chunk) in rain.screen_buffer.chunks(width).enumerate() {
            write!(&mut window, "{:02X} |", i).unwrap();
            write!(
                &mut window,
                "{}",
                &chunk.iter().map(|c| c.to_string()).collect::<String>()
            )
            .unwrap();
            if i == 20 {
                continue;
            }
            write!(&mut window, "\n").unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    insta::assert_snapshot!("screen_buffer", window);
}

#[test]
fn test_screen_buffer_direction_right_emoji_moon_double_width() {
    use super::Rain;
    use crate::cli::Cli;
    use clap::Parser;

    let width = 32;
    let height = 10;
    let mut settings = Cli::parse();
    settings.direction = super::Direction::Right;
    settings.chars = super::Characters::Moon;
    let mut rain = Rain::<1024>::new(width, height, &settings);
    let mut window = String::new();
    for id in 0..100 {
        rain.update();
        rain.update_screen_buffer().unwrap();
        write!(&mut window, "---------- {} -------------\n", id).unwrap();

        for (i, chunk) in rain.screen_buffer.chunks(width).enumerate() {
            write!(&mut window, "{:02X} |", i).unwrap();
            write!(
                &mut window,
                "{}|",
                &chunk.iter().map(|c| c.to_string()).collect::<String>()
            )
            .unwrap();
            if i == 20 {
                continue;
            }
            write!(&mut window, "\n").unwrap();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    insta::assert_snapshot!(
        "screen_buffer_direction_right_emoji_moon_double_width",
        window
    );
}
