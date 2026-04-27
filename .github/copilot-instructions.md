# RustiksCube - AI Coding Instructions

## Project Overview
High-performance Rubik's Cube solver using **Kociemba's two-phase algorithm**. Solves cubes via memory-mapped lookup tables stored on disk (referenced in `config.txt`).

## Architecture

### Core Cube Representations
The solver uses **three complementary cube representations** (see [architecture.mmd](../architecture.mmd)):

1. **`CornerIndex`** - Corner permutation + orientation indices
2. **`SubsetIndex`** - H0 subgroup state (slice edges + all corners in home position)
3. **`CosetIndex`** - Coset representative (orientations + slice locations)
4. **`Cube`** - Full state combining `SubsetIndex` + `CosetIndex`

**Critical:** Each representation uses **index-based encoding** (e.g., `c_prm: usize`, `e_ori: usize`). These are NOT raw cubie arrays but compressed indices for table lookups.

### Two-Phase Solver ([two_phase.rs](../src/two_phase.rs))
**Phase 1:** Find moves to reach H0 subgroup (subset cube)
- Uses `DirectionsTable` for pruning with twists that decrease/increase distance
- `relevant_twists` array enforces move order constraints (avoid redundant F B sequences, etc.)
- Cuts branches via corner distance heuristic and subset detection

**Phase 2:** Solve within H0 using only `<L2, R2, U, D, F2, B2>`
- Uses `DistanceTable` for greedy best-first search
- Much smaller state space

### Precomputed Tables ([stored_tables.rs](../src/stored_tables.rs))
Three massive lookup tables (loaded via `config.txt` paths):
- **Corners table** (~88M entries) - Distance to solved for corner states
- **Subset table** (~19.5B entries) - Distance in H0 subgroup  
- **Coset table** (~2.2M entries) - Directions to reach H0 with pruning data

Tables are created on first run (slow) then memory-mapped from disk. **Data integrity verified** against OEIS sequences.

### Twister System ([twister.rs](../src/twister.rs))
The `Twister` struct holds **precomputed transition tables** for all 18 twist types on indices:
```rust
twisted_c_prm(c_prm: usize, twist: Twist) -> usize
twisted_e_ori(e_ori: usize, twist: Twist) -> usize
```
Enables O(1) state transitions without reconstructing cubie arrays. `Twistable` trait provides `.twisted()` method.

## Key Conventions

### Index Encoding
All cube state uses **indices, not arrays**. Example from [cube.rs](../src/cube.rs#L78-L86):
```rust
SubsetIndex::index() -> (c_prm/2) * E_SLICE * E_NON_SLICE + e_non_slice_prm * E_SLICE + e_slice_prm
```
Division by 2 exploits parity constraints (corner/edge permutations must have matching parity).

### TwistSet Bitmask Pattern
`TwistSet` uses bit positions 0-17 for twists + bit 18 for `Twist::None`:
```rust
TwistSet::full()  // 0b111_111_111_111_111_111 (all 18 moves)
TwistSet::h0()    // 0b010_010_111_111_010_010 (H0 moves only)
```
Used extensively for pruning in Phase 1 search.

### Parallel Table Generation
Tables use Rayon `into_par_iter()` with atomic operations for breadth-first distance filling ([tables.rs](../src/tables.rs#L23-L43)). Pattern:
```rust
let table: Vec<AtomicU8> = (0..size).into_par_iter().map(|_| AtomicU8::new(SENTINEL)).collect();
```

## Development Workflows

### Build & Run
```bash
cargo build --release  # First build takes ~2-5 minutes (complex generics)
cargo run --release    # Requires config.txt with table paths
```
**Note:** Release builds use `codegen-units = 1` and thin LTO for optimal runtime performance. This trades longer compile time for faster execution. If compile times are excessive (>10 min), check available RAM (build needs ~5-10GB peak).

### Testing
```bash
cargo test --release  # Mathematical invariants in corners.rs, edges.rs
```

### Benchmarking
```bash
cargo bench           # Criterion benches in benches/
```
Measures twist operations at ~13-140ns per operation (see README benchmarks).

### Adding New Cube Types
1. Implement index encoding/decoding (`index()`, `from_index()`)
2. Add `Twistable` trait implementation
3. Update `Twister::new()` to precompute transition tables
4. Consider parity constraints in index calculation

### Memory-Mapped Tables
Tables use `memmap2` crate (see dependencies). To add new tables:
1. Create via `DistanceTable::create()` or `DirectionsTable::create()`
2. Save with `.save_to_file()`
3. Add path to `config.txt`
4. Verify correctness (see data integrity checks in `stored_tables.rs`)

## Common Pitfalls

- **Don't run without `--release`** - Debug builds are 10-100x slower due to array bounds checks
- **First build is slow** - Complex generics in `tables.rs` cause extensive monomorphization (~2-5 min, needs 5-10GB RAM)
- **Tables are huge** - Subset table is 19.5GB on disk; ensure adequate storage
- **Index arithmetic is tricky** - Off-by-one errors break parity constraints (causes panics in `from_index`)
- **Parallel iteration order** - Use `AtomicU8` with `compare_exchange` for race-free table fills
- **TwistSet filtering** - `relevant_twists` array prevents move reordering (F followed by B is illegal)

## File Navigation
- Cube state: [cube.rs](../src/cube.rs), [corners.rs](../src/corners.rs), [edges.rs](../src/edges.rs)
- Solver logic: [two_phase.rs](../src/two_phase.rs), [coset_solver.rs](../src/coset_solver.rs)
- Tables: [tables.rs](../src/tables.rs), [stored_tables.rs](../src/stored_tables.rs)
- Performance: [benches/twist_bench.rs](../benches/twist_bench.rs)
