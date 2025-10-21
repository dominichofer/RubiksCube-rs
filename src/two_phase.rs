// use crate::cube::*;
// use crate::twist::*;
// use crate::tables::*;

// pub struct TwoPhaseSolver {
//     phase1: DirectionsTable,
//     phase2: DistanceTable,
//     corners: DistanceTable,
//     twists: Vec<Twist>,
//     max_solution_length: usize,
//     phase_1_probes: usize,
//     phase_2_probes: usize,
//     corner_probes: usize,
//     corner_cuts: usize,
//     subset_cuts: usize,
// }

// impl TwoPhaseSolver {
//     pub fn new(
//         phase1: DirectionsTable,
//         phase2: DistanceTable,
//         corners: DistanceTable,
//     ) -> Self {
//         Self {
//             phase1,
//             phase2,
//             corners,
//             twists: Vec::new(),
//             max_solution_length: 0,
//             phase_1_probes: 0,
//             phase_2_probes: 0,
//             corner_probes: 0,
//             corner_cuts: 0,
//             subset_cuts: 0,
//         }
//     }

//     pub fn solve(&mut self, cube: Cube, max_solution_length: usize) -> Option<Vec<Twist>> {
//         self.twists.clear();
//         self.max_solution_length = max_solution_length;
//         // Phase 1 search implementation goes here
//         None
//     }

//     fn search(&mut self, cube: Cube, p1_depth: usize) {

//     }

//     fn clear_stats(&mut self) {
//         self.phase_1_probes = 0;
//         self.phase_2_probes = 0;
//         self.corner_probes = 0;
//         self.corner_cuts = 0;
//         self.subset_cuts = 0;
//     }
// }