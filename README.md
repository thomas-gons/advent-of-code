# Advent of Code (AoC) Solutions

This repository hosts my solutions to the annual Advent of Code (AoC) programming challenges.

The [Advent of Code](https://adventofcode.com/about) is a series of daily programming puzzles released from December 1st to December 25th by Eric Wastl. It serves as an excellent platform to practice algorithmic thinking, explore new language features, and refine problem-solving skills in a competitive environment.

## Repository Structure

Solutions are organized logically by year and day, with each day's challenge encapsulated within its own folder.

```rs
.
├── 2025/
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── common/              // Shared utilities and modules
│   │   ├── Cargo.toml
│   │   └── src
│   │       └── lib.rs       // Common runner and utilities
│   ├── day-01/
│   │    ├── data/           // Input files for the challenge
│   │    ├── src/        
│   │    │   └── main.rs     // Solution code for the day's challenge
│   │    └── Cargo.toml      // Cargo configuration file specific to the day's challenge
│   ├── day-02/
│   ├── ...                  // Additional day folders
│   │
│   └── scripts/
│       ├── day-init.sh      // Script to initialize new day folders
│       └── day-template.rs  // Template for new day solutions
│
├── 2024/
├── ...
└── README.md
```

## Technology Stack ![Rust](https://img.shields.io/badge/rust-1.91.1-red.svg?logo=rust&logoColor=white)

My primary focus for the 2025 challenges is the Rust programming language. Given my background in low-level C, I am leveraging this project to bridge the gap between low-level control and modern, safe systems programming.

## Getting Started

### 2025 Solutions

To run the solutions, ensure you have Rust installed on your machine. You can install Rust using [rustup](https://rustup.rs/).

Navigate to the desired year and execute the following command to run a specific day's solution:

```bash
cd <year>
cargo run -p day-<day-number>
```

#### Adding New Days
To add a new day's challenge, use the provided script:

```bash
./scripts/day-init.sh <day-number>
```

This script will create a new folder for the day, copy the template solution, and set up the necessary structure.
It will also add this new day as a member in the year's `Cargo.toml`.

You can also remove a day's folder using the `--rm` flag:

```bash
./scripts/day-init.sh --rm <day-number>
```
Note: A warning will be displayed if some parts have been implemented in the day's folder before removal.
