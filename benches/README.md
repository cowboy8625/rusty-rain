I use this to automatically kill the program after 60 seconds and generate a flamegraph.

```shell
(sleep 60 && xdotool key Escape) & cargo flamegraph --bin rusty-rain

# using time

(sleep 60 && xdotool key Escape) & command time -v ./target/release/rusty-rain
```

# Using perf to compare previous versions

`perf diff old.data new.data > diff-vX.X.X.data`
