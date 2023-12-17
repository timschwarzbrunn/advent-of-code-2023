#!/bin/sh

# Check if the correct number of arguments is provided.
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <project_name>"
    exit 1
fi

project_name=$1

# Create a new Rust project using cargo.
cargo new --bin "$project_name"

# Replace the new main.rs file with the template file.
rm $project_name/src/main.rs
cp template/src/main.rs $project_name/src/main.rs

# Create the template for the input.
cd $project_name
touch input
touch input.test
echo "\n[profile.release]\ndebug=true" >> Cargo.toml
