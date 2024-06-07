<h1 align="center">
<br>
<img src="https://user-images.githubusercontent.com/43012445/105452071-411e4880-5c43-11eb-8ae2-4de61f310bf9.gif" alt="GIF" width="800">
<img src="https://cdn.discordapp.com/attachments/509849754583302154/812942011400847391/emoji_rain.gif" alt="GIF" width="800">
<br>
Rusty Rain
<br>
<br>
</h1>

<p align="center">
<a href="https://crates.io/crates/rusty-rain"><img alt="crates.io" src="https://img.shields.io/crates/v/rusty-rain.svg"></a>
<a><img alt="lastupdated" src="https://img.shields.io/github/last-commit/cowboy8625/rusty-rain"></a>
<a><img alt="GitHub repo size" src="https://img.shields.io/github/repo-size/cowboy8625/rusty-rain"></a>
<a><img alt="issuse" src="https://img.shields.io/github/issues/cowboy8625/rusty-rain"></a>
<a><img alt="Lines of Code" src="https://img.shields.io/tokei/lines/github/cowboy8625/rusty-rain"></a>
<a><img alt="License" src="https://img.shields.io/badge/License-MIT-blue.svg"></a>
<a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
</p>

A cross platform matrix rain terminal program that runs well and looks good.

## To Use

Simply run the following command on windows/mac/linux:

```
git clone https://github.com/cowboy8625/rusty-rain.git
cd rusty-rain
```
```
cargo run --release
```

or to install:

```
cargo install rusty-rain
```

## Exit

To exit just press `ESC` or `Ctrl + C`


## Command Line Arguments

```
USAGE:
rusty-rain [FLAGS] [OPTIONS]

FLAGS:
-h, --help       Prints help information
-s, --shade      Set Rain shading to fade or stay constant
-V, --version    Prints version information

OPTIONS:
-c, --chars <characters>     Set what kind of characters are printed as rain.
                             OPTIONS:
                               all            - This shows most of the Character Groups all at once.
                               alphalow       - Lower Case Alphabet Characters
                               alphaup        - Upper Case Alphabet Characters
                               alphanum       - All Alphabets and Numbers
                               arrow          - Arrow Emojis or Fancy Characters
                               bin            - All Ones and Zeros
                               cards          - Playing Cards
                               clock          - 🕑
                               crab           - 🦀
                               dominosh       - 🀽
                               dominosv       - 🁫
                               earth          - 🌎
                               emojis         - This is just a bunch of random Emojis
                               jap            - Japanese Characters
                               large-letters  - Cool Looking Large Letters
                               moon           - 🌕
                               num            - Good ol fashion Numbers
                               numbered-balls - These are like pool balls
                               numbered-cubes - These are like the pool balls but just cubes
                               plants         - Plants of sorts
                               smile          - 😃
                               shapes         - Squares and Circles of a few colors
-C, --color <color>          Set color of Rain with color string name or tuple
                             OPTIONS:
                             white,
                               red,
                               blue,
                               green,
                               r,g,b
-d, --direction <direction>  Set the direction of the Rain.
                             Default is set to down/south
                             OPTIONS:
                               up, north,
                               down, south,
                               left, west,
                               right, east
-H, --head <head>            Set the color of the first char in Rain.
                             OPTIONS:
                               white,
                               red,
                               blue,
                               green,
                               r,g,b
-S, --speed <speed>          Set speed of rain MAX,MIN -S 200,400
```

### Example

  using cargo to run:

  `cargo run --release -- -C 0,139,139 -H 255,255,255 -c jap -s`

  after installing:

  `rusty-rain -C 0,139,139 -H 255,255,255 -c jap -s`

# Help

  If you find any bugs or performance is not up to par please submit a issue so I can better improve
  the project.

## Star History

<a href="https://star-history.com/#cowboy8625/rusty-rain&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=cowboy8625/rusty-rain&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=cowboy8625/rusty-rain&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=cowboy8625/rusty-rain&type=Date" />
  </picture>
</a>

