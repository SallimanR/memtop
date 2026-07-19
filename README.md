# `memtop` is utility to monitor Linux and system processes


## Process tree mode
<p align="center">
  <img src="./assets/process-tree.png" alt="Process tree" width="600">
</p>

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
