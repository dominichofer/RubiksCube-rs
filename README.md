# RustiksCube

A Rust project for solving Rustik's cube.

## Getting Started

To build and run this project, you'll need a Rust installed on your system.

To install Rust, visit [rustup.rs](https://rustup.rs/) and follow the instructions.

### Building the Project

```bash
cargo build --release
```

### Running the Project

```bash
cargo run --release
```

### Running Tests

```bash
cargo test --release
```

## Speed
On a AMD Ryzen 9 9950X3D 16-Core Processor with DDR5 RAM at 3600 MT/s, this program solves
test_cubes_big.txt in 90.3s (110 cubes per second)
```
Corners table loaded in: 23.7748ms
Subset table loaded in: 3.3316315s
Coset table loaded in: 5.5608556s
Total time taken: 90.3888446s
Average time per solve: 9.038884ms
Phase 1 probes: 836’889’120
Phase 2 probes: 61’301’857
Subset cuts: 4’943’914
Corner probes: 128’910’381
Corner cuts: 77’147’476
No twists cuts: 141’514’107
```