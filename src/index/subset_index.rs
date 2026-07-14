use crate::math::*;
use crate::edges::*;
use crate::permutation::*;
use rayon::prelude::*;

// Size: 141’134’400 bytes (~134.6 MiB)
pub struct SubsetIndex {
    e_xy_prm: Vec<u16>, // (12 choose 4) * 4! * (12 choose 4) * 4! = 141’134’400
}

impl SubsetIndex {
    pub fn new() -> Self {
        let mut e_xy_prm = vec![0u16; Edges::LOC_PRM_SIZE * Edges::LOC_PRM_SIZE];

        
        e_xy_prm
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, val)| {
                let x_loc_prm = LocPrm::from_index(i / Edges::LOC_PRM_SIZE);
                let y_loc_prm = LocPrm::from_index(i % Edges::LOC_PRM_SIZE);
                let x_loc = nth_combination(12, 4, x_loc_prm.loc());
                let y_loc = nth_combination(12, 4, y_loc_prm.loc());
                let x_prm = Permutation::<4>::from_index(x_loc_prm.prm());
                let y_prm = Permutation::<4>::from_index(y_loc_prm.prm());
                let mut prm = [12; 12];
                for i in 0..4 {
                    prm[x_loc[i]] = x_prm[i];
                    prm[y_loc[i]] = y_prm[i] + 4;
                }
                let mut prm2 = [0; 8];
                let mut j = 0;
                for &p in prm.iter() {
                    if p < 8 {
                        prm2[j] = p;
                        j += 1;
                    }
                }
                *val = permutation_index(&prm2) as u16;
            });

        Self { e_xy_prm }
    }

    pub fn e_xy_prm(&self, x_loc_prm: LocPrm, y_loc_prm: LocPrm) -> usize {
        self.e_xy_prm[x_loc_prm.index() * Edges::LOC_PRM_SIZE + y_loc_prm.index()] as usize
    }
}

pub static SUBSET_INDEX: std::sync::LazyLock<SubsetIndex> = std::sync::LazyLock::new(SubsetIndex::new);

pub fn init_subset_index() {
    std::sync::LazyLock::force(&SUBSET_INDEX);
}

// TODO: Add tests for subset_twister