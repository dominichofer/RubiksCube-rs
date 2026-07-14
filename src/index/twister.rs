use crate::corners::*;
use crate::edges::*;
use crate::twist::*;
use rayon::prelude::*;

// Size: 1’015’830 bytes (~0.97 MiB)
pub struct Twister {
    c_ori: Vec<u16>, // 18 * 3^7 = 39’366
    c_prm: Vec<u16>, // 18 * 8! = 725’760
    e_ori: Vec<u16>, // 18 * 2^11 = 36’864
    e_loc_prm: Vec<LocPrm>, // 18 * (12 choose 4) * 4! = 213’840
}

const COUNT: usize = ALL_TWISTS.len();

impl Twister {
    pub fn new() -> Self {
        let mut c_ori = vec![0u16; COUNT * Corners::ORI_SIZE];
        let mut c_prm = vec![0u16; COUNT * Corners::PRM_SIZE];
        let mut e_ori = vec![0u16; COUNT * Edges::ORI_SIZE];
        let mut e_loc_prm = vec![LocPrm::new(0, 0); COUNT * LocPrm::INDEX_SIZE];

        c_ori
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Corners::from_indices(0, i);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (twist * obj).ori_index() as u16;
                }
            });
        c_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Corners::from_indices(i, 0);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (twist * obj).prm_index() as u16;
                }
            });
        e_ori
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = Edges::from_indices(LocPrm::new(0, 0), LocPrm::new(0, 0), LocPrm::new(0, 0), i);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (twist * obj).ori_index() as u16;
                }
            });
        e_loc_prm
            .par_chunks_mut(COUNT)
            .enumerate()
            .for_each(|(i, chunk)| {
                let z_loc_prm = LocPrm::from_index(i);
                let obj = Edges::from_indices(LocPrm::new(0, 0), LocPrm::new(0, 0), z_loc_prm, 0);
                for twist in ALL_TWISTS {
                    chunk[twist as usize] = (twist * obj).loc_prm(Axis::Z);
                }
            });

        Self { c_ori, c_prm, e_ori, e_loc_prm }
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
}

pub static TWISTER: std::sync::LazyLock<Twister> = std::sync::LazyLock::new(Twister::new);

pub fn init_twister() {
    std::sync::LazyLock::force(&TWISTER);
}

pub trait Twistable: Sized + Copy {
    fn twisted(&self, twist: Twist) -> Self;
    fn twisted_by(&self, twists: &[Twist]) -> Self;
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
            c = twist * c;
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
        let mut x_loc_prm = e.loc_prm(Axis::X);
        let mut y_loc_prm = e.loc_prm(Axis::Y);
        let mut z_loc_prm = e.loc_prm(Axis::Z);
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            e = twist * e;
            x_loc_prm = twister.twisted_e_loc_prm(x_loc_prm, twist);
            y_loc_prm = twister.twisted_e_loc_prm(y_loc_prm, twist);
            z_loc_prm = twister.twisted_e_loc_prm(z_loc_prm, twist);
            ori = twister.twisted_e_ori(ori, twist);
            assert_eq!(e.loc_prm(Axis::X), x_loc_prm);
            assert_eq!(e.loc_prm(Axis::Y), y_loc_prm);
            assert_eq!(e.loc_prm(Axis::Z), z_loc_prm);
            assert_eq!(e.ori_index(), ori);
        }
    }
}
