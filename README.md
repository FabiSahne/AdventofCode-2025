# Advent of Code 2025

This year, I did Advent of Code in C# and Rust.

Please look in the respective subdirectory for the project.

---

Each day (except day 12) first checks each solution against the provided sample via assertions before running with the actual input.

## Building

If you want to build this for yourself, create a new subdirectory called `input` within this repository's root.
Then place each day named `NN.txt` where `NN` is the number of the day in question filled to two digits.
E.g. The input file of day 6 will be named `06.txt`.

### C#

Open the solution in your favorite IDE - for example Rider, or Visual Studio - and run the program.

### Rust

Enter the Rust directory in your terminal and run `cargo run --release --bin NN` where `NN` is the day in question.
Or likewise open the directory in your favorite IDE - for example RustRover - and run the desired day, likely by pressing a play button next to the `main` function.
