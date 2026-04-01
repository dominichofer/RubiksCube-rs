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

### Running the Project

```bash
cargo run --release
```

### Running Tests

```bash
cargo test --release
```

## Speed
On an AMD Ryzen 9 9950X3D 16-Core Processor with HT and DDR5 RAM with 4x 32-bit channels at 3600 MT/s, this program solves
test_cubes_big.txt in 13.8s (720 cubes per second) with a single thread.
```
Corners table loaded in: 19.3856ms
Subset table loaded in: 3.4855558s
Coset table loaded in: 5.6195334s
Total time taken: 13.8777383s
Average time per solve: 1.387773ms
Search depths:
  Depth 5: 1
  Depth 6: 13
  Depth 7: 180
  Depth 8: 1’661
  Depth 9: 10’801
  Depth 10: 20’119
  Depth 11: 11’225
  Depth 12: 3’150
  Depth 13: 244
  Depth 14: 3
Phase 1 probes: 106’069’810
Phase 2 probes: 15’558’460
Subset cuts: 1’048’070
Corner probes: 4’584’027
Corner cuts: 2’717’995 (59.29%)
No twist cuts: 17’105’909
```

### Benchmarking

```bash
cargo run --release --bin benchmark
```

On an AMD Ryzen 9 9950X3D 16-Core Processor with DDR5 RAM at 3600 MT/s, we measured twisting

Corners twisted               27.7 ns
Corners rotated_colours       53.2 ns
Corners from_indices          71.6 ns
Corners prm_index              7.9 ns
Corners ori_index              5.8 ns
Edges twisted                 32.8 ns
Edges rotated_colours         81.4 ns
Edges from_indices           203.5 ns
Edges slice_prm_index         20.4 ns
Edges non_slice_prm_index     25.2 ns
Edges slice_loc_index         27.1 ns
Edges ori_index                4.1 ns
SubsetCube twisted            13.6 ns
CosetCube twisted              7.3 ns
