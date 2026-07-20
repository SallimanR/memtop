# `memtop` is utility to monitor Linux and system processes


## Process tree mode
<p align="center">
  <img src="./assets/process-tree.png" alt="Process tree" width="1000">
</p>

![white](https://img.shields.io/badge/White-gray) ones are user space processes  
 └ ![blue](https://img.shields.io/badge/Blue-blue) ones are process' threads  
![green](https://img.shields.io/badge/Green-green) ones are kernel threads

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
