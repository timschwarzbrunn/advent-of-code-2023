# Advent of Code 2023

Solutions for [Advent of Code 2023](https://adventofcode.com/2023) written in Rust.  

## Usage

Change into the directory of a specific day and use `cargo run`.  

```
cd day-<n>
cargo run <input-file> <task>
```

You can specify the name of the input file (default: `./input`) and the task to solve (either `first` or `second`, the default is `first`).  

## Benchmarking

The script `benchmarking.sh` executes [hyperfine](https://github.com/sharkdp/hyperfine) for the given day for both tasks (first and second).  

e.g.:
```
./benchmarking day-01
```
