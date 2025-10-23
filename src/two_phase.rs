use crate::cube::*;
use crate::twist::*;
use crate::tables::*;

pub struct TwoPhaseSolver {
    phase_1: DirectionsTable,
    phase_2: DistanceTable,
    corners: DistanceTable,
    twists: Vec<Twist>,
    max_solution_length: usize,
    phase_1_probes: usize,
    phase_2_probes: usize,
    corner_probes: usize,
    corner_cuts: usize,
    subset_cuts: usize,
}

impl TwoPhaseSolver {
    pub fn new(
        phase_1: DirectionsTable,
        phase_2: DistanceTable,
        corners: DistanceTable,
    ) -> Self {
        Self {
            phase_1,
            phase_2,
            corners,
            twists: Vec::new(),
            max_solution_length: 0,
            phase_1_probes: 0,
            phase_2_probes: 0,
            corner_probes: 0,
            corner_cuts: 0,
            subset_cuts: 0,
        }
    }

    pub fn solve(&mut self, cube: Cube, max_solution_length: usize) -> Option<Vec<Twist>> {
        self.twists.clear();
        self.max_solution_length = max_solution_length;
        self.clear_stats();
        
        for depth in 0..=max_solution_length {
            let result = self.search_phase_1(cube, depth);
            if !result.is_empty() {
                return Some(result);
            }
        }
        None
    }

    fn search_phase_1(&mut self, cube: Cube, p1_depth: usize) -> Vec<Twist> {
        if p1_depth == 0 {
            self.phase_2_probes += 1;
            let subset_cube = cube.to_subset();
            if self.phase_2.distance(subset_cube.index() as usize) as usize + self.twists.len() <= self.max_solution_length {
                self.twists.extend(self.phase_2.solution(
                    subset_cube,
                    Twists::h0(),
                    |c: SubsetCube| c.index() as usize,
                ));
                return self.twists.clone();
            }
        }
        if cube.in_subset() {
            self.subset_cuts += 1;
            return Vec::new();
        }

        let mut twists = Twists::all();
        twists.unset_twists(Twists::face_of(self.twists.last().copied().unwrap()));

        self.phase_1_probes += 1;
        let coset_index = cube.coset_index() as usize;
        let subset_distance = self.phase_1.distance(coset_index) as usize;
        if p1_depth == subset_distance {
            twists.keep_only(self.phase_1.less_distance(coset_index));
        }
        if p1_depth == subset_distance + 1 {
            twists.unset_twists(self.phase_1.more_distance(coset_index));
        }
        for twist in twists.iter() {
            self.twists.push(twist);
            let result = self.search_phase_1(cube.twisted(twist), p1_depth - 1);
            if !result.is_empty() {
                return result;
            }
            self.twists.pop();
        }
        Vec::new()
    }

    fn clear_stats(&mut self) {
        self.phase_1_probes = 0;
        self.phase_2_probes = 0;
        self.corner_probes = 0;
        self.corner_cuts = 0;
        self.subset_cuts = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_phase_solver() {
        let phase_1_table = DirectionsTable::from_file("tables/phase_1_directions.tbl");
        let phase_2_table = subset_distance_table("tables/subset_distance.tbl");
        let corner_table = coset_distance_table("tables/coset_distance.tbl");

        let mut solver = TwoPhaseSolver::new(phase_1_table, phase_2_table, corner_table);

        let scramble = vec![
            Twist::U, Twist::R, Twist::UPrime, Twist::L, Twist::D, Twist::FPrime,
            Twist::B, Twist::RPrime, Twist::DPrime, Twist::LPrime,
        ];
        let mut cube = Cube::solved();
        for twist in &scramble {
            cube = cube.twisted(*twist);
        }

        let solution = solver.solve(cube, 20);
        assert!(solution.is_some());
        let solution = solution.unwrap();

        let mut test_cube = cube;
        for twist in &solution {
            test_cube = test_cube.twisted(*twist);
        }
        assert_eq!(test_cube, Cube::solved());
    }
}