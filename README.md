# `memtop` is utility to monitor Linux and system processes


## Process tree mode
<p align="center">
  <img src="./assets/process-tree.png" alt="Process tree" width="1000">
</p>
<span style="color:gray">White</span> ones are user space processes  
   <span style="color:dodgerblue">└ Blue</span> ones are process' threads  
<span style="color:green">Green</span> ones are kernel threads

## Developing

run without optimizations:
```sh
cargo run
```

build in release mode:
```sh
cargo build --release
```

## Architecture:
`memtop` uses a ratatui library for TUI interface
