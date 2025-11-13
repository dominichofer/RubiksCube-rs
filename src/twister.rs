use crate::corners::*;
use crate::edges::*;
use crate::twist::*;
use rayon::prelude::*;

pub struct Twister<Index> {
    table: Vec<Index>,
}

impl<Index> Twister<Index>
where
    Index: Into<usize> + Default + Copy + Send + Sync,
{
    pub fn new<Obj>(
        index: impl Fn(Obj) -> Index + Sync,
        from_index: impl Fn(usize) -> Obj + Sync,
        index_space: usize,
    ) -> Self
    where
        Obj: Twistable,
    {
        let all_twists = Twists::all();
        let mut table = vec![Index::default(); index_space * all_twists.count()];

        table
            .par_chunks_mut(all_twists.count())
            .enumerate()
            .for_each(|(i, chunk)| {
                let obj = from_index(i);
                for twist in all_twists.iter() {
                    chunk[twist as usize] = index(obj.twisted(twist));
                }
            });

        Self { table }
    }

    pub fn get(&self, i: impl Into<usize>, twist: Twist) -> Index {
        self.table[i.into() * Twists::all().count() + twist as usize] //TODO: optimize!
    }
}

// pub struct SliceLocPermutationParity {
//     table: Vec<bool>,
// }

// impl SliceLocPermutationParity {
//     pub fn new() -> Self {
//         let mut table = vec![false; Edges::SLICE_LOC_SIZE as usize];
//         let e_solved = Edges::solved();
//         for i in 0..Edges::SLICE_LOC_SIZE {
//             let e = Edges::from_index(
//                 e_solved.slice_prm_index(),
//                 e_solved.non_slice_prm_index(),
//                 i as u16,
//                 e_solved.ori_index(),
//             );
//             table[i as usize] = is_even_permutation_array(&e.cubies());
//         }
//         Self { table }
//     }

//     pub fn get(&self, index: u16) -> bool {
//         self.table[index as usize]
//     }
// }

pub struct AllTwister {
    c_ori: Twister<u16>,
    c_prm: Twister<u16>,
    e_ori: Twister<u16>,
    e_slice_prm: Twister<u8>,
    e_non_slice_prm: Twister<u16>,
    e_slice_loc: Twister<u16>,
}

impl AllTwister {
    pub fn new() -> Self {
        let c_ori = Twister::new(
            |c: Corners| c.ori_index(),
            |i: usize| Corners::from_index(0, i as u16),
            Corners::ORI_SIZE as usize,
        );
        let c_prm = Twister::new(
            |c: Corners| c.prm_index(),
            |i: usize| Corners::from_index(i as u16, 0),
            Corners::PRM_SIZE as usize,
        );
        let e_ori = Twister::new(
            |e: Edges| e.ori_index(),
            |i: usize| Edges::from_index(0, 0, 0, i as u16),
            Edges::ORI_SIZE as usize,
        );
        let e_slice_prm = Twister::new(
            |e: Edges| e.slice_prm_index(),
            |i: usize| {
                Edges::from_index(
                    (i / Edges::SLICE_LOC_SIZE as usize) as u8,
                    0,
                    (i % Edges::SLICE_LOC_SIZE as usize) as u16,
                    0,
                )
            },
            (Edges::SLICE_PRM_SIZE as usize) * (Edges::SLICE_LOC_SIZE as usize),
        );
        let e_non_slice_prm = Twister::new(
            |e: Edges| e.non_slice_prm_index(),
            |i: usize| {
                Edges::from_index(
                    0,
                    (i / Edges::SLICE_LOC_SIZE as usize) as u16,
                    (i % Edges::SLICE_LOC_SIZE as usize) as u16,
                    0,
                )
            },
            (Edges::NON_SLICE_PRM_SIZE as usize) * (Edges::SLICE_LOC_SIZE as usize),
        );
        let e_slice_loc = Twister::new(
            |e: Edges| e.slice_loc_index(),
            |i: usize| Edges::from_index(0, 0, i as u16, 0),
            Edges::SLICE_LOC_SIZE as usize,
        );
        Self {
            c_ori,
            c_prm,
            e_ori,
            e_slice_prm,
            e_non_slice_prm,
            e_slice_loc,
        }
    }

    pub fn twisted_c_ori(&self, c_ori: u16, twist: Twist) -> u16 {
        self.c_ori.get(c_ori, twist)
    }
    pub fn twisted_c_prm(&self, c_prm: u16, twist: Twist) -> u16 {
        self.c_prm.get(c_prm, twist)
    }
    pub fn twisted_e_ori(&self, e_ori: u16, twist: Twist) -> u16 {
        self.e_ori.get(e_ori, twist)
    }
    pub fn twisted_e_slice_prm(&self, e_slice_prm: u8, e_slice_loc: u16, twist: Twist) -> u8 {
        self.e_slice_prm.get(e_slice_prm as u16 * Edges::SLICE_LOC_SIZE + e_slice_loc, twist) as u8
    }
    pub fn twisted_e_non_slice_prm(&self, e_non_slice_prm: u16, e_slice_loc: u16, twist: Twist) -> u16 {
        self.e_non_slice_prm.get(e_non_slice_prm as usize * Edges::SLICE_LOC_SIZE as usize + e_slice_loc as usize, twist) as u16
    }
    pub fn twisted_e_slice_loc(&self, e_slice_loc: u16, twist: Twist) -> u16 {
        self.e_slice_loc.get(e_slice_loc, twist)
    }
}

pub static TWISTER: once_cell::sync::Lazy<AllTwister> = once_cell::sync::Lazy::new(|| {
    AllTwister::new()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corners() {
        let twister = AllTwister::new();
        let mut rnd = RandomTwistGen::new(42, Twists::all());
        let mut c = Corners::solved();
        let mut prm = c.prm_index();
        let mut ori = c.ori_index();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            c = c.twisted(twist);
            prm = twister.twisted_c_prm(prm, twist);
            ori = twister.twisted_c_ori(ori, twist);
            assert_eq!(c.prm_index(), prm);
            assert_eq!(c.ori_index(), ori);
        }
    }

    #[test]
    fn test_edges() {
        let twister = AllTwister::new();
        let mut rnd = RandomTwistGen::new(42, Twists::all());
        let mut e = Edges::solved();
        let mut ori = e.ori_index();
        let mut slice_prm = e.slice_prm_index();
        let mut non_slice_prm = e.non_slice_prm_index();
        let mut slice_loc = e.slice_loc_index();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            e = e.twisted(twist);
            ori = twister.twisted_e_ori(ori, twist);
            slice_prm = twister.twisted_e_slice_prm(slice_prm, slice_loc, twist);
            non_slice_prm = twister.twisted_e_non_slice_prm(non_slice_prm, slice_loc, twist);
            slice_loc = twister.twisted_e_slice_loc(slice_loc, twist);
            assert_eq!(e.ori_index(), ori);
            assert_eq!(e.slice_prm_index(), slice_prm);
            assert_eq!(e.non_slice_prm_index(), non_slice_prm);
            assert_eq!(e.slice_loc_index(), slice_loc);
        }
    }
}
