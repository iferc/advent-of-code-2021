[aoc]: https://adventofcode.com/2021 "Advent of Code 2021"
[rust]: https://www.rust-lang.org/ "Rust"
[rustup]: https://rustup.rs/ "Rust toolchain installer - rustup"

# Advent of Code 2021

This repository contains code solutions written in [Rust][rust] for some of the [Advent of Code 2021][aoc] challenges.

Copies of the questions and inputs will be kept in this repository for posterity. Note that all questions are from the awesome work that the folks who put together the [Advent of Code 2021][aoc] have done. The challenge inputs included will be different than those you would get if you login to the site yourself.

# Quick Start

## Install Rust Toolchain

Recommended method for install rust toolchain is with [rustup][rustup]. There are shell commands and release downloads available for installation on their website.

Recommended extensions to the toolchain:

```sh
# provides `cargo add <package>` and `cargo remove <package>`
# to add external modules (crates) from https://crates.io/
# otherwise requires manual editing of Cargo.toml
cargo install cargo-edit

# provides ability to run commands on project file changes, respects .gitignore
# `cargo watch -x run` will auto-rebuild and run the application
# `cargo watch -s <expr>` will run shell commands
cargo install cargo-watch

# optional for learning urposes
# can be used to see precompiled source before final compilation
cargo install cargo-expand
```

## Repository Structure

The overall repository is setup as a workspace of challenge libraries with no executable. Each day is it's own crate / library with it's own copy of the challenges for the day, set of input data, as well as tests and benchmarks.

The example given in each problem, as well as the main challenge input, are run against unit tests made for each challenge function. Since there is no executable, running unit tests is the way to execute the challenges.

Benchmark methods are also included to measure the performance of solutions.

## Solving Challenges

### To build the code, performing all compile-time checking

Note that this step is not required, other cargo commands will automatically build.

```sh
# build an unoptimized debug binary
cargo build

# build an optimized release binary
cargo build --release
```

### To run unit tests for all challenges across all days

```sh
# run unit tests against the unoptimized debug build
cargo test

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x test

# run unit tests against the optimized release build
cargo test --release

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x "test --release"
```

### To run unit tests for a specific day

```sh
# run unit tests against only the day01 unoptimized debug build
cargo test -p day01

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x "test -p day01"
```

### To run unit tests for a specific challenge

```sh
# run unit tests with "first" in the test name
# against the unoptimized debug build
cargo test first

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x "test first"

# run unit tests with "challenge" in the test name
# against the unoptimized debug build
cargo test challenge

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x "test challenge"

# run unit tests with "second" in the test name
# against only the day01 unoptimized debug build
cargo test -p day01 second

# auto rebuild and run the above on file change (convenience for development)
cargo watch -x "test -p day01 second"
```

### To run benchmarks

```sh
# run performance benchmarking tests for all challenges across all days
cargo bench

# run performance benchmarking tests for all challenges
# for only the day01 challenge benchmarks
cargo bench -p day01
```

Note that `criterion`, the benchmarking library, can generate HTML reports with detailed graphs if `gnuplot` is installed. On Debian systems this would look like `apt install gnuplot`. This is not required to use this repository, but is fun to look at.

These html reports are generated under the folder `target/criterion` from the root of the repository. One easy way to host the files for viewing is with the following utility via cargo not listed earlier in the readme.

```sh
# provides a quick way to serve files for viewing in a web browser
cargo install https

# launch the server for the criterion benchmarking report
http ./target/criterion
```

From there you can navigate to [http://localhost:8000/report](http://localhost:8000/report).
