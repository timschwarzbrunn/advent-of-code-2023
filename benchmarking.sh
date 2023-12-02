#!/bin/sh

# Check if the correct number of arguments is provided.
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <project-name>"
    exit 1
fi

project_name=$1

# Change into the projects directory and execute the program in release mode.
cd $project_name
cargo build --release

# Execute the benchmarking using hyperfine for the first and the second task.
hyperfine \
"./target/release/$project_name ./input first" \
"./target/release/$project_name ./input second" \
--warmup 20 -N
