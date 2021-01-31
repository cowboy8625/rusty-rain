use crate::{ABOUT, AUTHOR, VERSION};
use clap::{App, Arg};

pub fn cargs() -> ((u8, u8, u8), (u32, u32), bool) {
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
