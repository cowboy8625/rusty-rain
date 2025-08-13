# ---------- Version 0.4.0 Sun 4 Aug 2025 ----------

## Changed

In this release I decided to do a mostly rewrite of the code.
This code is a lot more readable and more performant.
We also removed itertools as it was only using zip! Which could be replaced with .zip().

## Fixed

- [Emojis not working with left and right directions](https://github.com/cowboy8625/rusty-rain/issues/18)
- [Reduced draw calls](https://github.com/cowboy8625/rusty-rain/issues/22)
- [Reduced cpu usage](https://github.com/cowboy8625/rusty-rain/issues/21)

# ---------- Version 0.3.7 Sun 22 Sep 2024 ----------

## Changed

In this release we absorbed the ezemoji crate. When I made ezemoji it made sense to me at the time but now not so much.

## Added

- [quit with `q` add by `hasecilu`](https://github.com/cowboy8625/rusty-rain/pull/27)

## Fixed

- [Use of random number generation causes thread panic](https://github.com/cowboy8625/rusty-rain/issues/25)

# ---------- Version 0.3.6 Sun 22 May 2022 ----------

## Added

## Fixed

- [Fixed crash when screen was 130+ in size with direction horizontal](#19)

# ---------- Version 0.3.5 Tue 15 Feb 2022 ----------

## Added

- Direction was added to rain #15
- Added cyan to head and tail colors

## Fixed
