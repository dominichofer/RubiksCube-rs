use crate::math::*;
use crate::edges::*;
use crate::twist::*;
use rayon::prelude::*;

// Size: 1’451’952 bytes (~1.4 MiB)
pub struct SubsetTwister {
    subset_e_xy_prm: Vec<u16>, // 18 * 8! = 725’760
    subset_e_z_prm: Vec<u8>, // 18 * 4! = 432
}

const COUNT: usize = ALL_TWISTS.len();

impl SubsetTwister {
    pub fn new() -> Self {
        let mut subset_e_xy_prm = vec![0u16; COUNT * factorial(8)];
        let mut subset_e_z_prm = vec![0u8; COUNT * factorial(4)];

        subset_e_xy_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_subset_indices(i, 0);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (twist * obj).xy_prm_index() as u16;
                }
            });
        subset_e_z_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_subset_indices(0, i);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (twist * obj).loc_prm(Axis::Z).prm() as u8;
                }
            });
        Self { subset_e_xy_prm, subset_e_z_prm }
    }

    pub fn twisted_subset_e_xy_prm(&self, e_xy_prm: usize, twist: Twist) -> usize {
        self.subset_e_xy_prm[e_xy_prm * COUNT + twist as usize] as usize
    }
    pub fn twisted_subset_e_z_prm(&self, e_z_prm: usize, twist: Twist) -> usize {
        self.subset_e_z_prm[e_z_prm * COUNT + twist as usize] as usize
    }
}

pub static SUBSET_TWISTER: std::sync::LazyLock<SubsetTwister> = std::sync::LazyLock::new(SubsetTwister::new);

pub fn init_subset_twister() {
    std::sync::LazyLock::force(&SUBSET_TWISTER);
}
// TODO: Add tests for subset_twister