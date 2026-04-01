use crate::math::{is_even_permutation, factorial};
use crate::twist::*;
use crate::twist_set::*;
use crate::tables::*;
use crate::edges::*;
use crate::cube::*;
use crate::twister::Twister;
use crate::multi_twister::MultiTwister;
use std::sync::atomic::{AtomicBool, Ordering};
use rayon::prelude::*;

pub struct CosetToSubsetPathsIterator<'a> {
    twister: &'a Twister,
    phase_1: &'a DirectionsTable,
    relevant_twists: Vec<TwistSet>,
    coset_cube: CosetCube,
    depth: u8,
    stack: Vec<TwistSet>,
}

impl<'a> CosetToSubsetPathsIterator<'a> {
    pub fn new(twister: &'a Twister, phase_1: &'a DirectionsTable, coset_cube: CosetCube) -> Self {
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
        let mut ret = Self {
            twister,
            phase_1,
            relevant_twists,
            coset_cube,
            depth: phase_1.distance(coset_cube.index()),
            stack: Vec::new(),
        };
        ret.fill_stack();
        ret

    }

    fn current_twists(&self) -> Vec<Twist> {
        self.stack.iter().map(|ts| ts.iter().next().unwrap()).collect()
    }
    
    fn pop_from_stack(&mut self) {
        while self.stack.is_empty() == false {
            let last_twist_set = self.stack.last_mut().unwrap();
            if last_twist_set.count() == 1 {
                self.stack.pop();
                continue;
            }
            else {
                let twist = last_twist_set.iter().next().unwrap();
                last_twist_set.unset_twist(twist);
                return;
            }
        }
        // Stack is empty
        self.depth += 1;
    }
    
    fn fill_stack(&mut self) {
        while self.stack.len() < self.depth as usize {
            let current_cube = self.coset_cube.twisted_by(&self.twister, &self.current_twists());
            let subset_distance = self.phase_1.distance(current_cube.index());
            let remaining_depth = self.depth - self.stack.len() as u8;
            let last_twist = if self.stack.is_empty() { Twist::None } else { self.stack.last().unwrap().iter().next().unwrap() };
            let mut twist_set = self.relevant_twists[last_twist as usize];
            if remaining_depth == subset_distance {
                twist_set.keep_only(self.phase_1.less_distance(current_cube.index()));
            }
            else if remaining_depth == subset_distance + 1 {
                twist_set.unset_twists(self.phase_1.more_distance(current_cube.index()));
            }
            if remaining_depth == 1 {
                twist_set.unset_twists(TwistSet::h0());
            }
            if twist_set.is_empty() {
                self.pop_from_stack();
            }
            else {
                self.stack.push(twist_set);
            }
        }
    }
}

impl<'a> std::iter::Iterator for CosetToSubsetPathsIterator<'a> {
    type Item = Vec<Twist>;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.fill_stack();
        let ret = self.current_twists();
        self.pop_from_stack();
        Some(ret)
    }
}


pub struct CosetCover<'a> {
    twister: MultiTwister<'a>,
    phase_2: &'a DistanceTable,
    even_prm: Vec<bool>,
    max_solution_length: u8,
    coset_cube: CosetCube,
    coset: Vec<AtomicBool>,
}

impl<'a> CosetCover<'a> {
    pub fn new(
        twister: &'a Twister,
        phase_2: &'a DistanceTable,
        max_solution_length: u8,
    ) -> Self {
        Self {
            twister: MultiTwister::new(twister),
            phase_2,
            even_prm: (0..factorial(8)).map(|i| { is_even_permutation(i as i64)}).collect(),
            max_solution_length,
            coset_cube: CosetCube::solved(),
            coset: (0..SubsetCube::INDEX_SIZE).map(|_| AtomicBool::new(false)).collect(),
        }
    }

    pub fn reset_for(&mut self, coset_cube: CosetCube) {
        self.coset_cube = coset_cube;
        self.coset.par_iter().for_each(|val| {
            val.store(false, Ordering::Relaxed);
        });
    }

    pub fn cover_with(&mut self, twists: &[Twist]) {
        let start = std::time::Instant::now();
        self.twister.set_for(twists);

        let max_distance = self.max_solution_length - twists.len() as u8;
        // Iterate through all elements in the subset
        (0..SubsetCube::INDEX_SIZE).into_par_iter().for_each(|subset_index| {
            if self.phase_2.distance(subset_index) <= max_distance {
                let mut subset_index = subset_index;
                let e_slice_prm = subset_index % Edges::SLICE_PRM_SIZE;
                subset_index /= Edges::SLICE_PRM_SIZE;
                let e_non_slice_prm = subset_index % Edges::NON_SLICE_PRM_SIZE;
                subset_index /= Edges::NON_SLICE_PRM_SIZE;
                let mut c_prm = subset_index * 2;
                let e_even_prm = self.even_prm[e_non_slice_prm]
                    ^ self.even_prm[e_slice_prm]
                    ^ true; // in subset e_slice_loc is an even permutation
                if e_even_prm != self.even_prm[c_prm] {
                    c_prm += 1;
                }
                let index = SubsetCube{
                    e_slice_prm: self.twister.twisted_e_slice_prm(e_slice_prm),
                    e_non_slice_prm: self.twister.twisted_e_non_slice_prm(e_non_slice_prm),
                    c_prm: self.twister.twisted_c_prm(c_prm),
                }.index();
                self.coset[index].store(true, Ordering::Relaxed);
            }
        });

        println!("Covered coset with {:?}. Time: {:?}", twists, start.elapsed());
        let count = self.coset.par_iter().filter(|val| val.load(Ordering::Acquire)).count();
        println!("Covered {} ({}%) elements in {:?}", count, count as f64 * 100.0 / SubsetCube::INDEX_SIZE as f64, start.elapsed());
    }
}