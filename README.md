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

profiling with Tracy
1. build with profile feature flag:
```sh
cargo build --profile dev
```
2. run
```sh
./target/release-profile/memtop
```

3. build tracy profiler with specific version (13.1)
4. run tracy -> connect to running `memtop`

<img src="./assets/profiling-tracy.png" alt="Profiling with tracy" width="1000">

View statistics of usage
<img src="./assets/profiling-tracy-statistics.png" width="800">

## Architecture:
`memtop` uses a ratatui library for TUI interface
