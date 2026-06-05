# RustiksCube

This Rust project aims to prove that every reachable state of the 3×3×3 Rubik’s cube can be solved in at most 20 moves, measured in the half-turn metric.

## Explanation

A Rubik’s Cube is a 3×3×3 grid of smaller cubes, called cubies, arranged to form a larger cube. It consists of:

- 8 corner cubies
- 12 edge cubies
- 6 fixed center cubies

The core cubie is not modeled, as it does not affect the state.

A cube state is defined entirely by the positions and orientations of the corner and edge cubies.

### Coordinate System

We define a right-handed coordinate system:

- x-axis: Left → Right
- y-axis: Down → Up
- z-axis: Back → Front

Faces are defined as:

- U (Up): +y
- D (Down): −y
- L (Left): −x
- R (Right): +x
- F (Front): +z
- B (Back): −z

### Faces and Colours

Each face has a fixed colour in the solved state:

U: White
D: Yellow
L: Green
R: Blue
F: Red
B: Orange

Center cubies define the identity of each face and do not move.

### Edge Cubies

Each edge cubie:

- occupies one of 12 positions
- has 2 possible orientations

In the solved state, edges are numbered as follows:

| Index | Position |
| ----- | -------- |
| 0     | UF       |
| 1     | UB       |
| 2     | DB       |
| 3     | DF       |
| 4     | UL       |
| 5     | UR       |
| 6     | DR       |
| 7     | DL       |
| 8     | FL       |
| 9     | FR       |
| 10    | BR       |
| 11    | BL       |


Each edge has orientation 0 or 1.
For each edge we define a primary sticker. For edges with a U/D-coloured sticker, this is the primary sticker. For edges without a U/D-coloured sticker, the L/R-coloured sticker is the primary sticker.
An edge that is part of the U or D face, has orientation 0 if its primary sticker is in the U or D face. Otherwise its orientation is 1.
An edge that is not part of the U or D face, has orientation 0 if its primary sticker is in the L or R face. Otherwise its orientation is 1.

### Corner Cubies

Each corner cubie:

- occupies one of 8 positions
- has 3 possible orientations

In the solved state, corners are numbered as follows:

| Index | Position |
| ----- | -------- |
| 0     | UFL      |
| 1     | UFR      |
| 2     | UBL      |
| 3     | UBR      |
| 4     | DFL      |
| 5     | DFR      |
| 6     | DBL      |
| 7     | DBR      |

Each corner has orientation 0 or 1 or 2.
A corner has orientation 0 if its U/D-coloured sticker is in the U or D face.
A corner has orientation 1 if a counterclockwise twist (looking at the corner) would put its U/D-coloured sticker in the U or D face.
A corner has orientation 2 if a clockwise twist (looking at the corner) would put its U/D-coloured sticker in the U or D face.

### Layer
A layer is a slice of the cube that can rotate as a unit. Each layer consists of 9 cubies. There are 3 layers along each axis (x, y, z).

### Twist
A twist is the rotation of a layer by 90°, 180°, or 270° clockwise (viewing the face directly). The 18 possible twists are denoted by face (L, R, U, D, F, B) and rotation count (1, 2, 3):

| Twist | Face | Rotation |
|-------|------|----------|
| L1, L2, L3 | Left (−x) | 90°, 180°, 270° |
| R1, R2, R3 | Right (+x) | 90°, 180°, 270° |
| U1, U2, U3 | Up (+y) | 90°, 180°, 270° |
| D1, D2, D3 | Down (−y) | 90°, 180°, 270° |
| F1, F2, F3 | Front (+z) | 90°, 180°, 270° |
| B1, B2, B3 | Back (−z) | 90°, 180°, 270° |

## Getting Started

To build and run this project, you'll need Rust installed on your system.

To install Rust, follow the instructions at [rustup.rs](https://rustup.rs/).

### Building the Project

```bash
cargo build --release
```

### Solving cube states

To solve a file, run
```bash
cargo run --release --bin rubikscube <file>
```

The file is expected to contain a space-separated sequence of twists on each line of the file. Each line is interpreted as the sequence of twists that are applied to a solved cube. The result is interpreted as an input cube state.

This project comes with two files:

- `test_pos_small.txt` contains 1'000 input sequences. Containing:
  - 1 empty sequence
  - all 18 1-twist sequences
  - all 18*18 2-twist sequences
  - the [superflip](https://en.wikipedia.org/wiki/Superflip) sequence
  - the rest are randomly generated sequences
- `test_pos_10k.txt` contains 10'000 randomly generated sequences

To run them, run
```bash
cargo run --release --bin rubikscube test_pos_small.txt
cargo run --release --bin rubikscube test_pos_10k.txt
```

Here's an example output of `test_pos_10k.txt` on an AMD Ryzen 9 9950X3D 16-Core Processor with HT and DDR5 RAM with 4x 32-bit channels at 3600 MT/s.
If solved `test_pos_10k.txt` in 5.1.8s (1’960 cubes per second) with a single thread.
```
Corners table loaded in: 22.5914ms
Subset table loaded in: 3.4555108s
Coset table loaded in: 6.1924011s
Total time taken: 5.108103s
Average time per solve: 510.81µs
Search depths:
  Depth 5: 1
  Depth 6: 31
  Depth 7: 331
  Depth 8: 3’004
  Depth 9: 18’549
  Depth 10: 28’799
  Depth 11: 10’384
  Depth 12: 1’304
  Depth 13: 26
Phase 1 probes: 41’926’051
Phase 2 probes: 6’521’976
Subset cuts: 0
Corner probes: 3’204’916
Corner cuts: 757’023 (23.62%)
No twist cuts: 7’251’917
```

Here's an example output of `test_pos_1000k.txt`
```
Corners table loaded in: 24.3981ms
Subset table loaded in: 3.4369206s
Coset table loaded in: 5.8811926s
Total time taken: 537.1291727s
Average time per solve: 537.129µs
Search depths:
  Depth 4: 19
  Depth 5: 226
  Depth 6: 2’858
  Depth 7: 31’083
  Depth 8: 309’626
  Depth 9: 1’854’509
  Depth 10: 2’853’725
  Depth 11: 1’055’209
  Depth 12: 138’029
  Depth 13: 3’289
  Depth 14: 4
Phase 1 probes: 4’462’756’553
Phase 2 probes: 682’582’666
Subset cuts: 0
Corner probes: 370’546’203
Corner cuts: 91’704’115 (24.75%)
No twist cuts: 770’409’921
```

41M x table[c_ori * X + e_ori * Y + z_loc] (table size: 16.5 GB)
33M * (avg twists) x  cube.twisted (115 ns)
6M x multiple subset.twisted (13.0 ns) & table[c_prm * X + xy_prm * Y + z_prm] (table size: 18.1 GB), 
3.2M x table[c_prm * X + c_ori]  (table size: 84 MB)


### Running Benchmarks

To run the benchmark, execute
```bash
cargo run --release --bin benchmark
```

Here's an example output of an AMD Ryzen 9 9950X3D 16-Core Processor with DDR5 RAM at 3600 MT/s
```
Corners twisted               26.2 ns
Corners conjugated_by         62.5 ns
Corners from_indices         121.3 ns
Corners prm_index              7.6 ns
Corners ori_index              5.9 ns
Edges twisted                 27.8 ns
Edges conjugated_by           70.3 ns
Edges from_indices           325.7 ns
Edges from_subset_indices    106.5 ns
Edges x_loc_index             27.4 ns
Edges y_loc_index             28.0 ns
Edges z_loc_index             28.4 ns
Edges x_prm_index             20.4 ns
Edges y_prm_index             21.3 ns
Edges z_prm_index             20.7 ns
Edges xy_prm_index            24.9 ns
Edges ori_index                3.9 ns
SubsetIndex twisted           15.6 ns
CosetIndex twisted             7.2 ns
CubeIndex twisted             14.1 ns
```

### Running the GUI

To run the graphical user interface, execute
```bash
cargo run --release --bin gui
```

### Running Tests

```bash
cargo test --release
```
