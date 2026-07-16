# RustiksCube

This Rust project aims to prove that every reachable state of the 3×3×3 Rubik’s cube can be solved in at most 20 moves, measured in the half-turn metric.

## Explanation

A Rubik’s Cube is a 3×3×3 grid of smaller cubes, called cubies, arranged to form a larger cube. It consists of:

- 8 corner cubies
- 12 edge cubies
- 6 fixed center cubies

The core cubie is not modeled, as it does not affect the state.
The center cubies are modeled as fixed to reduce the degrees of freedom by taking advantage of the symmetries of the cube.

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

This project comes with multiple files:

- `test_pos_small.txt` contains 1'000 input sequences. Containing:
  - 1 empty sequence
  - all 18 1-twist sequences
  - all 18*18 2-twist sequences
  - the [superflip](https://en.wikipedia.org/wiki/Superflip) sequence
  - the rest are randomly generated sequences
- `test_pos_10k.txt` contains 10'000 randomly generated sequences
- `test_pos_100k.txt` contains 100'000 randomly generated sequences
- `test_pos_1000k.txt` contains 1000'000 randomly generated sequences

To run them, run
```bash
cargo run --release --bin rubikscube test_pos_small.txt
cargo run --release --bin rubikscube test_pos_10k.txt
cargo run --release --bin rubikscube test_pos_100k.txt
cargo run --release --bin rubikscube test_pos_1000k.txt
```

Here's an example output of `test_pos_100k.txt` on an AMD Ryzen 9 9950X3D 16-Core Processor with HT and DDR5 RAM with 4x 32-bit channels at 3600 MT/s.
```
Total time taken: 30.7557862s
Average time per solve: 307.557µs
Phase 1: 443’308’238
Phase 2: 48’126’967
Phase 1 dst: 366’404’317
Phase 2 dst: 53’535’633
Corner dst: 95’107’772
Corner cuts: 28’776’954 (30.26%)
Twists: 442’683’682
Slack cuts: 4’252’291
```

Here's an example output of `test_pos_1000k.txt`
```
Total time taken: 306.6161887s
Average time per solve: 306.616µs
Phase 1: 4’413’210’006
Phase 2: 481’014’779
Phase 1 dst: 3’647’064’504
Phase 2 dst: 535’087’348
Corner dst: 944’285’944
Corner cuts: 285’130’723 (30.20%)
Twists: 4’406’961’429
Slack cuts: 42’431’134
```

### Running Benchmarks

To run the benchmark, execute
```bash
cargo run --release --bin benchmark
```

Here's an example output of an AMD Ryzen 9 9950X3D 16-Core Processor with DDR5 RAM at 3600 MT/s
```
Twister initialized in 0.003 seconds
SubsetTwister initialized in 0.001 seconds
SubsetIndex initialized in 0.660 seconds
nth_permutation (len 4)       39.9 ns
nth_permutation (len 8)       69.0 ns
nth_combination (12, 4)       51.4 ns
permutation_index (len 4)      5.1 ns
permutation_index (len 8)      9.5 ns
encode (base 2)               13.4 ns
encode (base 3)               11.4 ns
decode (base 2)               42.0 ns
decode (base 3)               34.7 ns
Corners twist                 27.6 ns
Corners conjugated_by         65.7 ns
Corners from_indices         108.5 ns
Corners prm_index              7.3 ns
Corners ori_index              5.3 ns
Edges twist                   28.0 ns
Edges conjugated_by           75.2 ns
Edges from_indices           368.2 ns
Edges from_subset_indices     62.9 ns
Edges x_loc_prm_index         30.8 ns
Edges y_loc_prm_index         31.6 ns
Edges z_loc_prm_index         31.5 ns
Edges xy_prm_index            26.9 ns
Edges ori_index                4.2 ns
SubsetCube twisted            14.4 ns
SubsetCube from_index         20.0 ns
SubsetCube index               0.6 ns
Cube twisted                   8.8 ns
Cube from_corner_index        28.7 ns
Cube from_coset_index         29.0 ns
Cube corner_index              0.7 ns
Cube subset_cube              10.2 ns
Cube coset_index               0.8 ns
Corners distance               4.4 ns
Coset distance                13.2 ns
Subset distance               53.2 ns
TwoPhaseSolver phase_2      2216.2 ns
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
