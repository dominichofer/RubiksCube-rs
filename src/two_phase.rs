use crate::*;
use num_format::ToFormattedString;

#[derive(Clone)]
pub struct TwoPhaseSolver<'a> {
    phase_1: &'a DirectionsTable,
    phase_2: &'a DistanceTable,
    corners: &'a DistanceTable,
    twists: Vec<Twist>,
    fkt_phase_1: usize,
    fkt_phase_2: usize,
    fkt_phase_1_dst: usize,
    fkt_phase_2_dst: usize,
    fkt_corner_dst: usize,
    corner_cuts: usize,
    fkt_twist: usize,
    slack_cuts: usize,
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
            twists: Vec::new(),
            fkt_phase_1: 0,
            fkt_phase_2: 0,
            fkt_phase_1_dst: 0,
            fkt_phase_2_dst: 0,
            fkt_corner_dst: 0,
            corner_cuts: 0,
            fkt_twist: 0,
            slack_cuts: 0,
        }
    }

    pub fn print_stats(&self) {
        let locale = &num_format::Locale::de_CH;
        println!("Phase 1: {}", self.fkt_phase_1.to_formatted_string(locale));
        println!("Phase 2: {}", self.fkt_phase_2.to_formatted_string(locale));
        println!("Phase 1 dst: {}", self.fkt_phase_1_dst.to_formatted_string(locale));
        println!("Phase 2 dst: {}", self.fkt_phase_2_dst.to_formatted_string(locale));
        println!("Corner dst: {}", self.fkt_corner_dst.to_formatted_string(locale));
        println!("Corner cuts: {} ({:.2}%)", self.corner_cuts.to_formatted_string(locale), (self.corner_cuts as f64 / self.fkt_corner_dst as f64) * 100.0);
        println!("Twists: {}", self.fkt_twist.to_formatted_string(locale));
        println!("Slack cuts: {}", self.slack_cuts.to_formatted_string(locale));
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
        self.fkt_phase_2 += 1;

        self.fkt_phase_2_dst += 1;
        let solution_distance = self.phase_2.distance(subset_cube.index());
        if solution_distance > depth {
            return false;
        }

        for d in (1..=solution_distance).rev() {
            for twist in H0_TWISTS {
                let next = subset_cube.twisted(twist);
                self.fkt_phase_2_dst += 1;
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
        self.fkt_phase_1 += 1;

        // Check corner distance
        if p1_depth + p2_depth < 10 {
            self.fkt_corner_dst += 1;
            let corner_distance = self.corners.distance(cube.corner_index());
            if corner_distance > p1_depth + p2_depth {
                self.corner_cuts += 1;
                return false;
            }
        }

        if p1_depth == 0 {
            return self.search_phase_2(cube.subset_cube(), p2_depth);
        }

        let mut twists;
        if let Some(&previous_twist) = self.twists.last() {
            twists = unique_twists_after(previous_twist);
        } else {
            twists = TwistSet::FULL;
        }
        if p1_depth == 1 {
            // H0 twists don't lead to a subset cube, so we omit them.
            twists.remove(TwistSet::H0);
        }

        let coset_index = cube.coset_index();
        self.fkt_phase_1_dst += 1;
        let subset_distance = self.phase_1.distance(coset_index);
        let slack = p1_depth - subset_distance;

        if subset_distance == 0 && p1_depth < 5 {
            // It takes at least 5 moves to reach a subset cube from an other subset cube, so we can prune this branch.
            self.slack_cuts += 1;
            return false;
        }

        if slack == 0 {
            // Without slack, we need to take the shortest path.
            twists.keep_only(self.phase_1.less_distance(coset_index));
        }
        else if slack == 1 {
            // With 1 move of slack, we cannot take any moves that increase the distance.
            twists.remove(self.phase_1.more_distance(coset_index));
        }
        
        for twist in twists.iter() {
            self.fkt_twist += 1;
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
