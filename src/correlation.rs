use plotters::prelude::*;
use rubikscube::*;
use std::collections::HashMap;

fn all_rotations(cube: CubeIndex) -> [CubeIndex; 24] {
    let u0 = cube;
    let u1 = u0.conjugated_by(Rotation::Z);
    let u2 = u1.conjugated_by(Rotation::Z);
    let u3 = u2.conjugated_by(Rotation::Z);
    let l1 = [u0, u1, u2, u3].map(|u| u.conjugated_by(Rotation::Y));
    let l2 = l1.map(|l| l.conjugated_by(Rotation::Y));
    let l3 = l2.map(|l| l.conjugated_by(Rotation::Y));
    let f1 = [u0, u1, u2, u3].map(|u| u.conjugated_by(Rotation::Y));
    let f2 = f1.map(|f| f.conjugated_by(Rotation::Y));
    let f3 = f2.map(|f| f.conjugated_by(Rotation::Y));
    [
        u0, u1, u2, u3,
        l1[0], l1[1], l1[2], l1[3],
        l2[0], l2[1], l2[2], l2[3],
        l3[0], l3[1], l3[2], l3[3],
        f1[0], f1[1], f1[2], f1[3],
        f3[0], f3[1], f3[2], f3[3],
    ]
}

fn main() {
    let twister = Twister::new();
    let tables = StoredTables::load("config.txt");
    let mut rnd = RandomTwistGen::new(42, &ALL_TWISTS);
    let mut correlation: HashMap<(usize, usize), HashMap<(u8, u8), usize>> = HashMap::new();
    for _ in 0..1_000_000 {
        let twists = rnd.gen_twists(100);
        let rnd_cube = CubeIndex::solved().twisted_by(&twister, &twists);
        let all_rotations = all_rotations(rnd_cube);
        let subset_distances = all_rotations.map(|cube| tables.coset.distance(cube.coset.index()));
        for (i, dst_i) in subset_distances.into_iter().enumerate() {
            for (j, dst_j) in subset_distances.into_iter().enumerate() {
                *correlation
                    .entry((i, j))
                    .or_default()
                    .entry((dst_i, dst_j))
                    .or_insert(0) += 1;
            }
        }
    }

    render_heatmaps(&correlation, "heatmap.png");
    println!("Saved heatmap.png");
}

/// Maps t ∈ [0, 1] to a white→blue heat color.
fn heat_color(t: f64) -> RGBColor {
    let v = (255.0 * (1.0 - t)) as u8;
    RGBColor(v, v, 255)
}

fn render_heatmaps(
    correlation: &HashMap<(usize, usize), HashMap<(u8, u8), usize>>,
    path: &str,
) {
    const N: usize = 24;
    const CELL: usize = 5; // pixels per distance bin
    const PAD: usize = 1;  // pixels between mini-heatmaps

    // Determine distance axis size from the data.
    let max_dist = correlation
        .values()
        .flat_map(|m| m.keys())
        .flat_map(|(a, b)| [*a, *b])
        .max()
        .unwrap_or(0) as usize
        + 1;

    let mini = max_dist * CELL;
    let total = N * (mini + PAD) + PAD;

    let root = BitMapBackend::new(path, (total as u32, total as u32)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    for i in 0..N {
        for j in 0..N {
            let x0 = (PAD + i * (mini + PAD)) as i32;
            let y0 = (PAD + j * (mini + PAD)) as i32;

            let inner = correlation.get(&(i, j));

            // Normalize each mini-heatmap independently.
            let local_max = inner
                .iter()
                .flat_map(|m| m.values())
                .copied()
                .max()
                .unwrap_or(1)
                .max(1) as f64;

            for di in 0..max_dist {
                for dj in 0..max_dist {
                    let count = inner
                        .and_then(|m| m.get(&(di as u8, dj as u8)))
                        .copied()
                        .unwrap_or(0);

                    let t = (count as f64 / local_max).sqrt();
                    let color = heat_color(t);

                    let x1 = x0 + (di * CELL) as i32;
                    let y1 = y0 + (dj * CELL) as i32;
                    root.draw(&Rectangle::new(
                        [(x1, y1), (x1 + CELL as i32, y1 + CELL as i32)],
                        color.filled(),
                    ))
                    .unwrap();
                }
            }
        }
    }
    root.present().unwrap();
}
