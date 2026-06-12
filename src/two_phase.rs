use crate::*;
use num_format::ToFormattedString;
use std::collections::HashMap;

#[derive(Clone)]
pub struct TwoPhaseSolver<'a> {
    phase_1: &'a DirectionsTable,
    phase_2: &'a DistanceTable,
    corners: &'a DistanceTable,
    twists: Vec<Twist>,
    search_depths: HashMap<u8, usize>,
    phase_1_probes: usize,
    phase_2_probes: usize,
    corner_probes: usize,
    corner_cuts: usize,
}

impl<'a> TwoPhaseSolver<'a> {
    pub fn new(
        phase_1: &'a DirectionsTable,
        phase_2: &'a DistanceTable,
        corners: &'a DistanceTable,
    ) -> Self {
        Self {
            phase_1,
            phase_2,
            corners,
            search_depths: HashMap::new(),
            twists: Vec::new(),
            phase_1_probes: 0,
            phase_2_probes: 0,
            corner_probes: 0,
            corner_cuts: 0,
        }
    }

    pub fn print_stats(&self) {
        let locale = &num_format::Locale::de_CH;
        println!("Search depths:");
        let mut sorted_depths: Vec<_> = self.search_depths.iter().collect();
        sorted_depths.sort_by_key(|(depth, _)| *depth);
        for (depth, count) in sorted_depths {
            println!("  Depth {}: {}", depth, count.to_formatted_string(locale));
        }
        println!("Phase 1 probes: {}", self.phase_1_probes.to_formatted_string(locale));
        println!("Phase 2 probes: {}", self.phase_2_probes.to_formatted_string(locale));
        println!("Corner probes: {}", self.corner_probes.to_formatted_string(locale));
        println!("Corner cuts: {} ({:.2}%)", self.corner_cuts.to_formatted_string(locale), (self.corner_cuts as f64 / self.corner_probes as f64) * 100.0);
    }

    pub fn solve(&mut self, cube: Cube, max_solution_length: u8) -> Result<Vec<Twist>, String> {
        let cubes = [
            cube,
            cube.conjugated_by(Axis::X),
            cube.conjugated_by(Axis::Y),
            cube.inverse(),
            cube.inverse().conjugated_by(Axis::X),
            cube.inverse().conjugated_by(Axis::Y),
        ];
        let solution_transforms = [
            |twists: &[Twist]| twists.to_vec(),
            |twists: &[Twist]| conjugate_by_inv(twists, Axis::X),
            |twists: &[Twist]| conjugate_by_inv(twists, Axis::Y),
            |twists: &[Twist]| inverse(twists),
            |twists: &[Twist]| inverse(&conjugate_by_inv(twists, Axis::X)),
            |twists: &[Twist]| inverse(&conjugate_by_inv(twists, Axis::Y)),
        ];
        let subset_distances = cubes.map(|c| self.phase_1.distance(c.coset_index()));
        let min_distance = *subset_distances.iter().min().unwrap();

        for p1_depth in min_distance..=max_solution_length {
            for i in 0..cubes.len() {
                let cube = cubes[i];
                let subset_distance = subset_distances[i];

                if subset_distance > p1_depth {
                    continue;
                }
                *self.search_depths.entry(p1_depth).or_insert(0) += 1;
                let result = self.search_phase_1(cube, p1_depth, max_solution_length - p1_depth);
                if result {
                    let drained_solution: Vec<Twist> = self.twists.drain(..).collect();
                    let solution = solution_transforms[i](&drained_solution);
                    return Ok(solution);
                }
            }
        }
        Err("No solution found".into())
    }

    pub fn search_phase_2(&mut self, mut subset_cube: SubsetCube, depth: u8) -> bool {
        self.phase_2_probes += 1;

        let solution_distance = self.phase_2.distance(subset_cube.index());
        if solution_distance > depth {
            return false;
        }

        for d in (1..=solution_distance).rev() {
            for twist in H0_TWISTS {
                let next = subset_cube.twisted(twist);
                let next_d = self.phase_2.distance(next.index());
                if next_d < d {
                    self.twists.push(twist);
                    subset_cube = next;
                    break;
                }
            }
        }
        return true;
    }

    fn search_phase_1(&mut self, cube: Cube, p1_depth: u8, p2_depth: u8) -> bool {
        self.phase_1_probes += 1;

        if p1_depth == 0 {
            return self.search_phase_2(cube.subset_cube(), p2_depth);
        }

        if p1_depth + p2_depth < 10 {
            self.corner_probes += 1;
            let corner_distance = self.corners.distance(cube.corner_index());
            if corner_distance > p1_depth + p2_depth {
                self.corner_cuts += 1;
                return false;
            }
        }

        let mut twists;
        if let Some(&previous_twist) = self.twists.last() {
            twists = unique_twists_after(previous_twist);
        } else {
            twists = TwistSet::FULL;
        }
        if p1_depth == 1 {
            twists.remove(TwistSet::H0);
        }

        let coset_index = cube.coset_index();
        let subset_distance = self.phase_1.distance(coset_index);
        let slack = p1_depth - subset_distance;
        if slack == 0 {
            // Without slack, we need to take the shortest path.
            twists.keep_only(self.phase_1.less_distance(coset_index));
        }
        else if slack == 1 {
            // With 1 move of slack, we cannot take any moves that increase the distance.
            twists.remove(self.phase_1.more_distance(coset_index));
        }
        if subset_distance == 1 && p1_depth > 1 {
            // If we're 1 move away from the subset, but have more than 1 move to get there, we must not take the move that gets us there immediately.
            twists.remove(self.phase_1.less_distance(coset_index));
        }
        
        for twist in twists.iter() {
            let next_cube = cube.twisted(twist);
            self.twists.push(twist);
            let found_solution = self.search_phase_1(next_cube, p1_depth - 1, p2_depth);
            if found_solution {
                return true;
            }
            self.twists.pop();
        }
        false
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_two_phase_solver() {
//         let phase_1_table = DirectionsTable::from_file("tables/phase_1_directions.tbl");
//         let phase_2_table = subset_distance_table("tables/subset_distance.tbl");
//         let corner_table = coset_distance_table("tables/coset_distance.tbl");

//         let mut solver = TwoPhaseSolver::new(phase_1_table, phase_2_table, corner_table);

//         let scramble = vec![
//             Twist::U, Twist::R, Twist::UPrime, Twist::L, Twist::D, Twist::FPrime,
//             Twist::B, Twist::RPrime, Twist::DPrime, Twist::LPrime,
//         ];
//         let mut cube = Cube::solved();
//         for twist in &scramble {
//             cube = cube.twisted(*twist);
//         }

//         let solution = solver.solve(cube, 20);
//         assert!(solution.is_some());
//         let solution = solution.unwrap();

//         let mut test_cube = cube;
//         for twist in &solution {
//             test_cube = test_cube.twisted(*twist);
//         }
//         assert_eq!(test_cube, Cube::solved());
//     }
// }
