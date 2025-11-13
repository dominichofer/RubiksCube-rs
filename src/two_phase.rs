use crate::cube::*;
use crate::twist::*;
use crate::twister::AllTwister;
use crate::tables::*;
use num_format::ToFormattedString;

pub struct TwoPhaseSolver {
    twister: AllTwister,
    phase_1: DirectionsTable,
    phase_2: DistanceTable,
    corners: DistanceTable,
    twists: Vec<Twist>,
    pub phase_1_probes: usize,
    pub phase_2_probes: usize,
    pub subset_cuts: usize,
    pub corner_probes: usize,
    pub corner_cuts: usize,
}

impl TwoPhaseSolver {
    pub fn new(
        phase_1: DirectionsTable,
        phase_2: DistanceTable,
        corners: DistanceTable,
    ) -> Self {
        Self {
            twister: AllTwister::new(),
            phase_1,
            phase_2,
            corners,
            twists: Vec::new(),
            phase_1_probes: 0,
            phase_2_probes: 0,
            subset_cuts: 0,
            corner_probes: 0,
            corner_cuts: 0,
        }
    }

    pub fn print_stats(&self) {
        let locale = &num_format::Locale::de_CH;
        println!("Phase 1 probes: {}", self.phase_1_probes.to_formatted_string(locale));
        println!("Phase 2 probes: {}", self.phase_2_probes.to_formatted_string(locale));
        println!("Subset cuts: {}", self.subset_cuts.to_formatted_string(locale));
        println!("Corner probes: {}", self.corner_probes.to_formatted_string(locale));
        println!("Corner cuts: {}", self.corner_cuts.to_formatted_string(locale));
    }

    pub fn solve(&mut self, cube: Cube, max_solution_length: usize) -> Result<Vec<Twist>, String> {
        self.twists.clear();
        
        let subset_distance = self.phase_1.distance(cube.coset_index() as usize) as usize;
        for depth in subset_distance..=max_solution_length {
            let result = self.search_phase_1(cube, depth, max_solution_length - depth);
            if result {
                return Ok(self.twists.clone());
            }
        }
        Err("No solution found".into())
    }

    fn search_phase_1(&mut self, cube: Cube, p1_depth: usize, p2_depth: usize) -> bool {
        self.phase_1_probes += 1;
        
        if p1_depth == 0 {
            self.phase_2_probes += 1;
            let subset_cube = cube.to_subset();
            let phase_2_distance = self.phase_2.distance(subset_cube.index() as usize) as usize;
            if phase_2_distance <= p2_depth {
                self.twists.extend(self.phase_2.solution(
                    subset_cube,
                    Twists::h0(),
                    |c: SubsetCube| c.index() as usize,
                ));
                return true;
            }
        }

        if cube.in_subset() {
            self.subset_cuts += 1;
            return false;
        }

        if p1_depth + p2_depth < 9 {
            self.corner_probes += 1;
            let corner_distance = self.corners.distance(cube.corners_index() as usize) as usize;
            if corner_distance > p1_depth + p2_depth {
                self.corner_cuts += 1;
                return false;
            }
        }

        let mut twists = Twists::all();
        if self.twists.is_empty() == false {
            let last_twist = *self.twists.last().unwrap();
            twists.unset_twists(Twists::face_of(last_twist));
        }

        let coset_index = cube.coset_index() as usize;
        let subset_distance = self.phase_1.distance(coset_index) as usize;
        if p1_depth == subset_distance {
            twists.keep_only(self.phase_1.less_distance(coset_index));
        }
        else if p1_depth == subset_distance + 1 {
            twists.unset_twists(self.phase_1.more_distance(coset_index));
        }
        for twist in twists.iter() {
            self.twists.push(twist);
            // let next_cube = Cube {
            //     c_prm: self.twister.twisted_c_prm(cube.c_prm, twist),
            //     c_ori: self.twister.twisted_c_ori(cube.c_ori, twist),
            //     e_non_slice_prm: self.twister.twisted_e_non_slice_prm(cube.e_non_slice_prm, cube.e_slice_loc, twist),
            //     e_slice_prm: self.twister.twisted_e_slice_prm(cube.e_slice_prm, cube.e_slice_loc, twist),
            //     e_slice_loc: self.twister.twisted_e_slice_loc(cube.e_slice_loc, twist),
            //     e_ori: self.twister.twisted_e_ori(cube.e_ori, twist),
            // };
            let next_cube = cube.twisted(twist);
            let result = self.search_phase_1(next_cube, p1_depth - 1, p2_depth);
            if result {
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