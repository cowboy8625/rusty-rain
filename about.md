# Why

I made Rusty Rain a while ago trying to learn Rust and man it was super fun!
[version 0.1.0](https://github.com/cowboy8625/rusty-rain/tree/v0.0.1) was the
first try and boy oh boy I had no clue what I was doing.
That version was so slow and was so badly designed.
I started looking into DOD (Data Oriented Design) and I was very intrigued so
to try and put this new design pattern to work.  I made a new branch
[dod](https://github.com/cowboy8625/rusty-rain/tree/dod) and basically started from
scratch.  It was pretty easy to be honest to implement. So did it speed the program up?
About 90% I would say, the CPU usage went from 98% to 2-5% usage depending on what
options are given.  Shading the characters can make a lot more draw calls and can lead
to more CPU usage.
