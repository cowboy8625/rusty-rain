<h1 align="center">
  <br>
  <img src="https://user-images.githubusercontent.com/43012445/105452071-411e4880-5c43-11eb-8ae2-4de61f310bf9.gif" alt="Matrix rain demo" width="800">
  <br>
  Rusty Rain
  <br>
</h1>

<p align="center">
<a href="https://crates.io/crates/rusty-rain"><img alt="crates.io" src="https://img.shields.io/crates/v/rusty-rain.svg"></a>
<img alt="last updated" src="https://img.shields.io/github/last-commit/cowboy8625/rusty-rain">
<img alt="repo size" src="https://img.shields.io/github/repo-size/cowboy8625/rusty-rain">
<img alt="issues" src="https://img.shields.io/github/issues/cowboy8625/rusty-rain">
<img alt="lines of code" src="https://img.shields.io/tokei/lines/github/cowboy8625/rusty-rain">
<img alt="license" src="https://img.shields.io/badge/License-MIT-blue.svg">
<a href="https://discord.gg/KwnGX8P"><img alt="Discord Chat" src="https://img.shields.io/discord/509849754155614230"></a>
</p>

> **Rusty Rain** is a fast, cross-platform Matrix-style rain effect for your terminal, written in Rust.  
> Fully customizable: colors, characters, speed, direction — make it as chill or chaotic as you want.

---

## 🚀 Quick Install

| Method                                   | Command                                                                                     |
| ---------------------------------------- | ------------------------------------------------------------------------------------------- |
| Cargo (stable)                           | `cargo install rusty-rain`                                                                  |
| Cargo (latest)                           | `cargo install --git https://github.com/cowboy8625/rusty-rain.git`                          |
| [eget](https://github.com/zyedidia/eget) | `eget cowboy8625/rusty-rain`                                                                |
| Docker (build)                           | `docker build -t rusty-rain . && docker run --rm -it rusty-rain alpha-num -s`               |
| Docker (pull)                            | `docker run --rm -it ghcr.io/cowboy8625/rusty-rain:latest -c alpha-num -s`                  |
| Debian Package                           | curl -sSL https://raw.githubusercontent.com/cowboy8625/rusty-rain/master/install.sh \| bash |

---

## 🎯 Quick Start

```bash
rusty-rain
```

Default mode: green binary rain, classic Matrix look.
Press **`ESC`**, **`Ctrl + C`**, or **`q`** to quit.

---

## 🎨 Examples

```bash
# Japanese characters, teal rain, white head, flowing left
rusty-rain -C 0,139,139 -H 255,255,255 -g jap -s -d left

# Emoji chaos
rusty-rain -g emojis -C red -H yellow -S 0,50

# Large letters in bright blue, rain up
rusty-rain -g large-letters -C blue -H white -d up
```

---

## ⚙️ Customization

<details>
<summary>Full CLI Options</summary>

```
Cross platform CMatrix like program.

Usage: rusty-rain [OPTIONS]

Options:
  -s, --shade


  -g, --group <GROUP>
          Set what kind of characters are printed as rain.
          OPTIONS:
              all            - This shows most of the Character Groups all at once.
              alphalow       - Lower Case Alphabet Characters
              alphaup        - Upper Case Alphabet Characters
              arrow          - Arrow Emojis or Fancy Characters
              bin            - All Ones and Zeros
              cards          - Playing Cards
              classic        - closer to what the default look is for cmatrix
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
              open-source    - Open Source icon emojis
              pglangs        - These are programming language icons emojis
              plants         - Plants of sorts
              shapes         - Squares and Circles of a few colors
              smile          - 😃


          [default: bin]

  -C, --color <COLOR>
          Set color of Rain with color string name or tuple
          OPTIONS:
              white,
              red,
              blue,
              green,
              r,g,b


          [default: green]

  -H, --head <HEAD>
          Set the color of the first char in Rain.
          OPTIONS:
              white,
              red,
              blue,
              green,
              r,g,b,
              #RRGGBB


          [default: white]

  -d, --direction <DIRECTION>
          Set the direction of the Rain.
          Default is set to down/south
          OPTIONS:
              up or north,
              down or south,
              left or west,
              right or east


          [default: south]

  -S, --speed <SPEED>
          [default: 0,200]

  -D, --display-group
          Display Char Group

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

</details>

<details>
<summary>Config Options</summary>

```toml
# windows path %APPDATA%\\rusty-rain\\config.toml
# linux   path ~/.config/rusty-rain/config.toml
# mac    path ~/.config/rusty-rain/config.toml
speed = "0,200"
[groups.neovim]
range = [
  { start = 62319, end = 62320 },
]
width = 2

[groups.rust]
range = [
  { start = 59304, end = 59305 },
]
width = 2
```

## </details>

## 🛠 Contributing

We welcome:

- 🐛 Bug reports
- 💡 Feature requests
- 🔤 New character groups
- 🧑‍💻 Code contributions

Open an issue or pull request — let’s make it rain together. 🤣

---

## ⭐ Star History

<a href="https://star-history.com/#cowboy8625/rusty-rain&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=cowboy8625/rusty-rain&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=cowboy8625/rusty-rain&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=cowboy8625/rusty-rain&type=Date" />
  </picture>
</a>

---

## 📜 License

APACHE © [cowboy8625](https://github.com/cowboy8625)
