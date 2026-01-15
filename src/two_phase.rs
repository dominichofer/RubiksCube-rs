use crate::cube::{CornersCube, CosetCube, SubsetCube};
use crate::twist::{Twist, TwistSet};
use crate::twister::{Twister, Twistable};
use crate::tables::{DistanceTable, DirectionsTable};
use std::collections::VecDeque;
use num_format::ToFormattedString;

pub struct TwoPhaseSolver {
    twister: Twister,
    phase_1: DirectionsTable,
    phase_2: DistanceTable,
    corners: DistanceTable,
    relevant_twists: Vec<TwistSet>,
    phase_1_probes: usize,
    phase_2_probes: usize,
    subset_cuts: usize,
    corner_probes: usize,
    corner_cuts: usize,
    no_twists_cut: usize,
    solution: VecDeque<Twist>,  
}

impl TwoPhaseSolver {
    pub fn new(
        twister: Twister,
        phase_1: DirectionsTable,
        phase_2: DistanceTable,
        corners: DistanceTable,
    ) -> Self {
        let relevant_twists = vec![
            TwistSet::from_bits(0b111_111_111_111_111_000),
            TwistSet::from_bits(0b111_111_111_111_111_000),
            TwistSet::from_bits(0b111_111_111_111_111_000),
            TwistSet::from_bits(0b111_111_111_111_000_000),
            TwistSet::from_bits(0b111_111_111_111_000_000),
            TwistSet::from_bits(0b111_111_111_111_000_000),
            TwistSet::from_bits(0b111_111_111_000_111_111),
            TwistSet::from_bits(0b111_111_111_000_111_111),
            TwistSet::from_bits(0b111_111_111_000_111_111),
            TwistSet::from_bits(0b111_111_000_000_111_111),
            TwistSet::from_bits(0b111_111_000_000_111_111),
            TwistSet::from_bits(0b111_111_000_000_111_111),
            TwistSet::from_bits(0b111_000_111_111_111_111),
            TwistSet::from_bits(0b111_000_111_111_111_111),
            TwistSet::from_bits(0b111_000_111_111_111_111),
            TwistSet::from_bits(0b000_000_111_111_111_111),
            TwistSet::from_bits(0b000_000_111_111_111_111),
            TwistSet::from_bits(0b000_000_111_111_111_111),
            TwistSet::from_bits(0b111_111_111_111_111_111),
        ];
        Self {
            twister,
            phase_1,
            phase_2,
            corners,
            relevant_twists,
            phase_1_probes: 0,
            phase_2_probes: 0,
            subset_cuts: 0,
            corner_probes: 0,
            corner_cuts: 0,
            no_twists_cut: 0,
            solution: VecDeque::new(),
        }
    }

    pub fn print_stats(&self) {
        let locale = &num_format::Locale::de_CH;
        println!("Phase 1 probes: {}", self.phase_1_probes.to_formatted_string(locale));
        println!("Phase 2 probes: {}", self.phase_2_probes.to_formatted_string(locale));
        println!("Subset cuts: {}", self.subset_cuts.to_formatted_string(locale));
        println!("Corner probes: {}", self.corner_probes.to_formatted_string(locale));
        println!("Corner cuts: {}", self.corner_cuts.to_formatted_string(locale));
        println!("No twists cuts: {}", self.no_twists_cut.to_formatted_string(locale));
    }

    pub fn solve(&mut self, subset: SubsetCube, coset: CosetCube, max_solution_length: u8) -> Result<Vec<Twist>, String> {
        let subset_distance = self.phase_1.distance(coset.index());
        for p1_depth in subset_distance..=max_solution_length {
            let result = self.search_phase_1(subset, coset, p1_depth, max_solution_length - p1_depth, Twist::None);
            if result {
                return Ok(self.solution.drain(..).collect());
            }
        }
        Err("No solution found".into())
    }

    fn search_phase_2(&mut self, subset: SubsetCube, depth: u8) -> bool {
        let mut subset = subset;
        self.phase_2_probes += 1;
        let solution_distance = self.phase_2.distance(subset.index());
        if solution_distance <= depth {
            self.solution.clear();
            for d in (1..=solution_distance).rev() {
                for twist in TwistSet::h0().iter() {
                    let next = subset.twisted(&self.twister, twist);
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

    fn search_phase_1(&mut self, subset: SubsetCube, coset: CosetCube, p1_depth: u8, p2_depth: u8, twist: Twist) -> bool {
        self.phase_1_probes += 1;
        let c_prm = self.twister.twisted_c_prm(subset.c_prm, twist);
        let c_ori = self.twister.twisted_c_ori(coset.c_ori, twist);
        
        if p1_depth == 0 {
            let e_slice_prm = self.twister.twisted_e_slice_prm(subset.e_slice_prm, coset.e_slice_loc, twist);
            let e_non_slice_prm = self.twister.twisted_e_non_slice_prm(subset.e_non_slice_prm, coset.e_slice_loc, twist);
            let subset_cube = SubsetCube{ e_slice_prm, e_non_slice_prm, c_prm };
            return self.search_phase_2(subset_cube, p2_depth);
        }

        if p1_depth + p2_depth < 9 {
            self.corner_probes += 1;
            let corner_distance = self.corners.distance(CornersCube{ prm: c_prm, ori: c_ori }.index());
            if corner_distance > p1_depth + p2_depth {
                self.corner_cuts += 1;
                return false;
            }
        }
        
        let e_ori = self.twister.twisted_e_ori(coset.e_ori, twist);
        let e_slice_loc = self.twister.twisted_e_slice_loc(coset.e_slice_loc, twist);
        let next_coset = CosetCube{ c_ori, e_ori, e_slice_loc };
        if next_coset.in_subset() {
            self.subset_cuts += 1;
            return false;
        }
        let coset_index = next_coset.index();
        let subset_distance = self.phase_1.distance(coset_index);
        let mut twists = self.relevant_twists[twist as usize];
        if p1_depth == subset_distance {
            twists.keep_only(self.phase_1.less_distance(coset_index));
        }
        else if p1_depth == subset_distance + 1 {
            twists.unset_twists(self.phase_1.more_distance(coset_index));
        }
        if p1_depth == 1 {
            twists.unset_twists(TwistSet::h0());
        }
        if twists.is_empty() {
            self.no_twists_cut += 1;
            return false;
        }

        let e_slice_prm = self.twister.twisted_e_slice_prm(subset.e_slice_prm, coset.e_slice_loc, twist);
        let e_non_slice_prm = self.twister.twisted_e_non_slice_prm(subset.e_non_slice_prm, coset.e_slice_loc, twist);
        let next_subset = SubsetCube{ e_slice_prm, e_non_slice_prm, c_prm };
        for twist in twists.iter() {
            let next_p1_depth: u8 = if twist == Twist::None { p1_depth } else { p1_depth - 1 };
            let result = self.search_phase_1(next_subset, next_coset, next_p1_depth, p2_depth, twist);
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