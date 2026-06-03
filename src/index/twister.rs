use crate::math::*;
use crate::corners::*;
use crate::edges::*;
use crate::twist::*;
use rayon::prelude::*;

// Size: 1’742’022 bytes (~1.66 MiB)
pub struct Twister {
    c_ori: Vec<u16>, // 18 * 3^7 = 39’366
    c_prm: Vec<u16>, // 18 * 8! = 725’760
    e_ori: Vec<u16>, // 18 * 2^11 = 36’864
    e_loc_prm: Vec<LocPrm>, // 18 * (12 choose 4) * 4! = 213’840
    subset_e_xy_prm: Vec<u16>, // 18 * 8! = 725’760
    subset_e_z_prm: Vec<u8>, // 18 * 4! = 432
}

const COUNT: usize = ALL_TWISTS.len();

impl Twister {
    pub fn new() -> Self {
        let mut c_ori = vec![0u16; COUNT * Corners::ORI_SIZE];
        let mut c_prm = vec![0u16; COUNT * Corners::PRM_SIZE];
        let mut e_ori = vec![0u16; COUNT * Edges::ORI_SIZE];
        let mut e_loc_prm = vec![LocPrm::new(0, 0); COUNT * LocPrm::INDEX_SIZE];
        let mut subset_e_xy_prm = vec![0u16; COUNT * factorial(8)];
        let mut subset_e_z_prm = vec![0u8; COUNT * factorial(4)];

        c_ori
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Corners::from_indices(0, i);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (Corners::twist(twist) * obj).ori_index() as u16;
                }
            });
        c_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Corners::from_indices(i, 0);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (Corners::twist(twist) * obj).prm_index() as u16;
                }
            });
        e_ori
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_indices(LocPrm::new(0, 0), LocPrm::new(0, 0), LocPrm::new(0, 0), i);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (Edges::twist(twist) * obj).ori_index() as u16;
                }
            });
        e_loc_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let z_loc_prm = LocPrm::from_index(i);
                let obj = Edges::from_indices(LocPrm::new(0, 0), LocPrm::new(0, 0), z_loc_prm, 0);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (Edges::twist(twist) * obj).z_loc_prm_index();
                }
            });
        subset_e_xy_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_subset_indices(i, 0);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (Edges::twist(twist) * obj).xy_prm_index() as u16;
                }
            });
        subset_e_z_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_subset_indices(0, i);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (Edges::twist(twist) * obj).z_loc_prm_index().prm() as u8;
                }
            });

        Self { c_ori, c_prm, e_ori, e_loc_prm, subset_e_xy_prm, subset_e_z_prm }
    }

    pub fn twisted_c_ori(&self, c_ori: usize, twist: Twist) -> usize {
        self.c_ori[c_ori * COUNT + twist as usize] as usize
    }
    pub fn twisted_c_prm(&self, c_prm: usize, twist: Twist) -> usize {
        self.c_prm[c_prm * COUNT + twist as usize] as usize
    }
    pub fn twisted_e_ori(&self, e_ori: usize, twist: Twist) -> usize {
        self.e_ori[e_ori * COUNT + twist as usize] as usize
    }
    pub fn twisted_e_loc_prm(&self, e_loc_prm: LocPrm, twist: Twist) -> LocPrm {
        self.e_loc_prm[e_loc_prm.index() * COUNT + twist as usize]
    }
    pub fn twisted_subset_e_xy_prm(&self, e_xy_prm: usize, twist: Twist) -> usize {
        self.subset_e_xy_prm[e_xy_prm * COUNT + twist as usize] as usize
    }
    pub fn twisted_subset_e_z_prm(&self, e_z_prm: usize, twist: Twist) -> usize {
        self.subset_e_z_prm[e_z_prm * COUNT + twist as usize] as usize
    }
}

pub trait Twistable: Sized + Copy {
    fn twisted(&self, twister: &Twister, twist: Twist) -> Self;
    fn twisted_by(&self, twister: &Twister, twists: &[Twist]) -> Self;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::twist_generator::*;

    // Tests 'twisted_c_prm' and 'twisted_c_ori'
    #[test]
    fn test_corners() {
        let twister = Twister::new();
        let mut rnd = RandomTwistGen::new(42, &ALL_TWISTS);
        let mut c = Corners::solved();
        let mut prm = c.prm_index();
        let mut ori = c.ori_index();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            c = Corners::twist(twist) * c;
            prm = twister.twisted_c_prm(prm, twist);
            ori = twister.twisted_c_ori(ori, twist);
            assert_eq!(c.prm_index(), prm);
            assert_eq!(c.ori_index(), ori);
        }
    }

    // Tests 'twisted_e_ori' and 'twisted_e_loc_prm'
    #[test]
    fn test_edges() {
        let twister = Twister::new();
        let mut rnd = RandomTwistGen::new(42, &ALL_TWISTS);
        let mut e = Edges::solved();
        let mut ori = e.ori_index();
        let mut x_loc_prm = e.x_loc_prm_index();
        let mut y_loc_prm = e.y_loc_prm_index();
        let mut z_loc_prm = e.z_loc_prm_index();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            e = Edges::twist(twist) * e;
            x_loc_prm = twister.twisted_e_loc_prm(x_loc_prm, twist);
            y_loc_prm = twister.twisted_e_loc_prm(y_loc_prm, twist);
            z_loc_prm = twister.twisted_e_loc_prm(z_loc_prm, twist);
            ori = twister.twisted_e_ori(ori, twist);
            assert_eq!(e.x_loc_prm_index(), x_loc_prm);
            assert_eq!(e.y_loc_prm_index(), y_loc_prm);
            assert_eq!(e.z_loc_prm_index(), z_loc_prm);
            assert_eq!(e.ori_index(), ori);
        }
    }
}
