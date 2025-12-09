# Advent of Code (AoC) Solutions

This repository hosts my solutions to the annual Advent of Code (AoC) programming challenges.

The [Advent of Code](https://adventofcode.com/about) is a series of daily programming puzzles released from December 1st to December 25th by Eric Wastl. It serves as an excellent platform to practice algorithmic thinking, explore new language features, and refine problem-solving skills in a competitive environment.

## Repository Structure

Solutions are organized logically by year and day, with each day's challenge encapsulated within its own folder.

```rs
.
├── 2025/
│   ├─── day-01/
│   │    ├── data/        // Input files for the challenge
│   │    ├── src/        
│   │    │   └── main.rs  // Solution code for the day's challenge
│   │    ├── Cargo.lock   // Cargo lock file for environment consistency
│   │    └── Cargo.toml   // Cargo configuration file
│   ├── day-02/
│   └── ...
├── ...
└── README.md
```

## Technology Stack ![Rust](https://img.shields.io/badge/rust-1.91.1-red.svg?logo=rust&logoColor=white)

My primary focus for the 2025 challenges is the Rust programming language. Given my background in low-level C, I am leveraging this project to bridge the gap between low-level control and modern, safe systems programming.

## Getting Started

To run the solutions, ensure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

Go to the desired day's directory and run the solution using Cargo:

```bash
cd <year>/day-<day-number>/
cargo run
```
