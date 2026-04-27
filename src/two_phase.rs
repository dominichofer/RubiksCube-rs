use crate::*;
use num_format::ToFormattedString;
use std::collections::{HashMap, VecDeque};

pub struct TwoPhaseSolver<'a> {
    twister: &'a Twister,
    phase_1: &'a DirectionsTable,
    phase_2: &'a DistanceTable,
    corners: &'a DistanceTable,
    solution: VecDeque<Twist>,
    relevant_twists: Vec<TwistSet>,
    search_depths: HashMap<u8, usize>,
    phase_1_probes: usize,
    phase_2_probes: usize,
    subset_cuts: usize,
    corner_probes: usize,
    corner_cuts: usize,
    no_twist_cut: usize,
}

impl<'a> TwoPhaseSolver<'a> {
    pub fn new(
        twister: &'a Twister,
        phase_1: &'a DirectionsTable,
        phase_2: &'a DistanceTable,
        corners: &'a DistanceTable,
    ) -> Self {
        let relevant_twists = vec![
            TwistSet::from(0b111_111_111_111_111_000),
            TwistSet::from(0b111_111_111_111_111_000),
            TwistSet::from(0b111_111_111_111_111_000),
            TwistSet::from(0b111_111_111_111_000_000),
            TwistSet::from(0b111_111_111_111_000_000),
            TwistSet::from(0b111_111_111_111_000_000),
            TwistSet::from(0b111_111_111_000_111_111),
            TwistSet::from(0b111_111_111_000_111_111),
            TwistSet::from(0b111_111_111_000_111_111),
            TwistSet::from(0b111_111_000_000_111_111),
            TwistSet::from(0b111_111_000_000_111_111),
            TwistSet::from(0b111_111_000_000_111_111),
            TwistSet::from(0b111_000_111_111_111_111),
            TwistSet::from(0b111_000_111_111_111_111),
            TwistSet::from(0b111_000_111_111_111_111),
            TwistSet::from(0b000_000_111_111_111_111),
            TwistSet::from(0b000_000_111_111_111_111),
            TwistSet::from(0b000_000_111_111_111_111),
            TwistSet::from(0b111_111_111_111_111_111),
        ];
        Self {
            twister,
            phase_1,
            phase_2,
            corners,
            search_depths: HashMap::new(),
            solution: VecDeque::new(),
            relevant_twists,
            phase_1_probes: 0,
            phase_2_probes: 0,
            subset_cuts: 0,
            corner_probes: 0,
            corner_cuts: 0,
            no_twist_cut: 0,
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
        println!(
            "Phase 1 probes: {}",
            self.phase_1_probes.to_formatted_string(locale)
        );
        println!(
            "Phase 2 probes: {}",
            self.phase_2_probes.to_formatted_string(locale)
        );
        println!(
            "Subset cuts: {}",
            self.subset_cuts.to_formatted_string(locale)
        );
        println!(
            "Corner probes: {}",
            self.corner_probes.to_formatted_string(locale)
        );
        println!(
            "Corner cuts: {} ({:.2}%)",
            self.corner_cuts.to_formatted_string(locale),
            (self.corner_cuts as f64 / self.corner_probes as f64) * 100.0
        );
        println!(
            "No twist cuts: {}",
            self.no_twist_cut.to_formatted_string(locale)
        );
    }

    pub fn solve(
        &mut self,
        cube: CubeIndex,
        max_solution_length: u8,
    ) -> Result<Vec<Twist>, String> {
        let cubes = [
            cube,
            cube.rotated_colours(Rotation::L),
            cube.rotated_colours_by(&[Rotation::U, Rotation::L]),
        ];
        let solution_transforms = [
            |twists: &[Twist]| twists.to_vec(),
            |twists: &[Twist]| simplify_rot_twists(Rotation::L, twists),
            |twists: &[Twist]| simplify_rots_twists(&[Rotation::U, Rotation::L], twists),
        ];
        let subset_distances =
            Vec::from_iter(cubes.iter().map(|c| self.phase_1.distance(c.coset.index())));
        let min_distance = *subset_distances.iter().min().unwrap();

        for p1_depth in min_distance..=max_solution_length {
            for i in 0..cubes.len() {
                let cube = &cubes[i];
                let subset_distance = subset_distances[i];

                if subset_distance > p1_depth {
                    continue;
                }
                *self.search_depths.entry(p1_depth).or_insert(0) += 1;
                let result = self.search_phase_1(
                    cube.subset,
                    cube.coset,
                    p1_depth,
                    max_solution_length - p1_depth,
                    Twist::None,
                );
                if result {
                    let drained_solution: Vec<Twist> = self.solution.drain(..).collect();
                    let solution = solution_transforms[i](&drained_solution);
                    return Ok(solution);
                }
            }
        }
        Err("No solution found".into())
    }

    pub fn search_phase_2(&mut self, subset: SubsetIndex, depth: u8) -> bool {
        let mut subset = subset;
        self.phase_2_probes += 1;
        let solution_distance = self.phase_2.distance(subset.index());
        if solution_distance <= depth {
            self.solution.clear();
            for d in (1..=solution_distance).rev() {
                for twist in H0_TWISTS {
                    let next = subset.twisted(self.twister, twist);
                    let next_d = self.phase_2.distance(next.index());
                    if next_d < d {
                        self.solution.push_back(twist);
                        subset = next;
                        break;
                    }
                }
            }
            return true;
        }
        return false;
    }

    fn search_phase_1(
        &mut self,
        subset: SubsetIndex,
        coset: CosetIndex,
        p1_depth: u8,
        p2_depth: u8,
        twist: Twist,
    ) -> bool {
        self.phase_1_probes += 1;
        let c_prm = self.twister.twisted_c_prm(subset.c_prm, twist);
        let c_ori = self.twister.twisted_c_ori(coset.c_ori, twist);

        if p1_depth == 0 {
            let e_slice_prm =
                self.twister
                    .twisted_e_slice_prm(subset.e_slice_prm, coset.e_slice_loc, twist);
            let e_non_slice_prm = self.twister.twisted_e_non_slice_prm(
                subset.e_non_slice_prm,
                coset.e_slice_loc,
                twist,
            );
            let subset_cube = SubsetIndex {
                e_slice_prm,
                e_non_slice_prm,
                c_prm,
            };
            return self.search_phase_2(subset_cube, p2_depth);
        }

        if p1_depth + p2_depth < 9 {
            self.corner_probes += 1;
            let corner_distance = self.corners.distance(
                CornerIndex {
                    prm: c_prm,
                    ori: c_ori,
                }
                .index(),
            );
            if corner_distance > p1_depth + p2_depth {
                self.corner_cuts += 1;
                return false;
            }
        }

        let e_ori = self.twister.twisted_e_ori(coset.e_ori, twist);
        let e_slice_loc = self.twister.twisted_e_slice_loc(coset.e_slice_loc, twist);
        let next_coset = CosetIndex {
            c_ori,
            e_ori,
            e_slice_loc,
        };
        if next_coset.in_subset() {
            self.subset_cuts += 1;
            return false;
        }
        let coset_index = next_coset.index();
        let subset_distance = self.phase_1.distance(coset_index);
        let mut twist_set = self.relevant_twists[twist as usize];
        if p1_depth == subset_distance {
            twist_set.keep_only(self.phase_1.less_distance(coset_index));
        } else if p1_depth == subset_distance + 1 {
            twist_set.unset_twists(self.phase_1.more_distance(coset_index));
        }
        if p1_depth == 1 {
            twist_set.unset_twists(TwistSet::H0);
        }
        if twist_set.is_empty() {
            self.no_twist_cut += 1;
            return false;
        }

        let e_slice_prm =
            self.twister
                .twisted_e_slice_prm(subset.e_slice_prm, coset.e_slice_loc, twist);
        let e_non_slice_prm =
            self.twister
                .twisted_e_non_slice_prm(subset.e_non_slice_prm, coset.e_slice_loc, twist);
        let next_subset = SubsetIndex {
            e_slice_prm,
            e_non_slice_prm,
            c_prm,
        };
        for twist in twist_set.iter() {
            let result =
                self.search_phase_1(next_subset, next_coset, p1_depth - 1, p2_depth, twist);
            if result {
                self.solution.push_front(twist);
                return true;
            }
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
