use crate::corners::*;
use crate::edges::*;
use crate::is_even_permutation_array;
use crate::twist::*;
use rayon::prelude::*;
use std::sync::OnceLock;

pub struct AllTwister<Index> {
    table: Vec<Index>,
}

impl<Index> AllTwister<Index>
where
    Index: Into<usize> + Default + Copy + Send + Sync,
{
    // pub const fn empty() -> Self {
    //     Self { table: Vec::new() }
    // }

    pub fn new<Obj>(
        index: impl Fn(Obj) -> Index + Sync,
        from_index: impl Fn(usize) -> Obj + Sync,
        index_space: usize,
    ) -> Self
    where
        Obj: Twistable,
    {
        let all_twists = Twists::all();
        let mut table = vec![Index::default(); index_space * all_twists.size()];

        table
            .par_chunks_mut(all_twists.size())
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
        self.table[i.into() * Twists::all().size() + twist as usize]
    }
}

pub fn c_ori_table() -> &'static AllTwister<u16> {
    static C_ORI: OnceLock<AllTwister<u16>> = OnceLock::new();
    C_ORI.get_or_init(|| {
        AllTwister::new(
            |c: Corners| c.ori_index(),
            |i: usize| Corners::from_index(0, i as u16),
            Corners::ORI_SIZE as usize,
        )
    })
}

pub fn c_prm_table() -> &'static AllTwister<u16> {
    static C_PRM: OnceLock<AllTwister<u16>> = OnceLock::new();
    C_PRM.get_or_init(|| {
        AllTwister::new(
            |c: Corners| c.prm_index(),
            |i: usize| Corners::from_index(i as u16, 0),
            Corners::PRM_SIZE as usize,
        )
    })
}

pub fn e_ori_table() -> &'static AllTwister<u16> {
    static E_ORI: OnceLock<AllTwister<u16>> = OnceLock::new();
    E_ORI.get_or_init(|| {
        AllTwister::new(
            |e: Edges| e.ori_index(),
            |i: usize| Edges::from_index(0, 0, 0, i as u16),
            Edges::ORI_SIZE as usize,
        )
    })
}

pub fn e_slice_prm_table() -> &'static AllTwister<u8> {
    static E_SLICE_PRM: OnceLock<AllTwister<u8>> = OnceLock::new();
    E_SLICE_PRM.get_or_init(|| {
        AllTwister::new(
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
        )
    })
}

pub fn e_non_slice_prm_table() -> &'static AllTwister<u16> {
    static E_NON_SLICE_PRM: OnceLock<AllTwister<u16>> = OnceLock::new();
    E_NON_SLICE_PRM.get_or_init(|| {
        AllTwister::new(
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
        )
    })
}

pub fn e_slice_loc_table() -> &'static AllTwister<u16> {
    static E_SLICE_LOC: OnceLock<AllTwister<u16>> = OnceLock::new();
    E_SLICE_LOC.get_or_init(|| {
        AllTwister::new(
            |e: Edges| e.slice_loc_index(),
            |i: usize| Edges::from_index(0, 0, i as u16, 0),
            Edges::SLICE_LOC_SIZE as usize,
        )
    })
}

pub struct SliceLocPermutationParity {
    table: Vec<bool>,
}

impl SliceLocPermutationParity {
    pub fn new() -> Self {
        let mut table = vec![false; Edges::SLICE_LOC_SIZE as usize];
        for i in 0..Edges::SLICE_LOC_SIZE {
            let e = Edges::from_index(
                Edges::solved().slice_prm_index(),
                Edges::solved().non_slice_prm_index(),
                i as u16,
                Edges::solved().ori_index(),
            );
            table[i as usize] = is_even_permutation_array(&e.cubies());
        }
        Self { table }
    }

    pub fn get(&self, index: u16) -> bool {
        self.table[index as usize]
    }
}

pub fn e_slice_loc_parity_table() -> &'static SliceLocPermutationParity {
    static E_SLICE_LOC_PARITY: OnceLock<SliceLocPermutationParity> = OnceLock::new();
    E_SLICE_LOC_PARITY.get_or_init(|| SliceLocPermutationParity::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corners() {
        let mut rnd = RandomTwistGen::new(42, Twists::all());
        let mut c = Corners::solved();
        let mut prm = c.prm_index();
        let mut ori = c.ori_index();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            c = c.twisted(twist);
            prm = c_prm_table().get(prm, twist);
            ori = c_ori_table().get(ori, twist);
            assert_eq!(c.prm_index(), prm);
            assert_eq!(c.ori_index(), ori);
        }
    }

    #[test]
    fn test_edges() {
        let mut rnd = RandomTwistGen::new(42, Twists::all());
        let mut e = Edges::solved();
        let mut ori = e.ori_index();
        let mut slice_prm = e.slice_prm_index();
        let mut non_slice_prm = e.non_slice_prm_index();
        let mut slice_loc = e.slice_loc_index();
        for _ in 0..100_000 {
            let twist = rnd.gen_twist();
            e = e.twisted(twist);
            ori = e_ori_table().get(ori, twist);
            slice_prm = e_slice_prm_table().get(slice_prm as u16 * Edges::SLICE_LOC_SIZE + slice_loc, twist) as u8;
            non_slice_prm = e_non_slice_prm_table().get(non_slice_prm as usize * Edges::SLICE_LOC_SIZE as usize + slice_loc as usize, twist) as u16;
            slice_loc = e_slice_loc_table().get(slice_loc, twist);
            assert_eq!(e.ori_index(), ori);
            assert_eq!(e.slice_prm_index(), slice_prm);
            assert_eq!(e.non_slice_prm_index(), non_slice_prm);
            assert_eq!(e.slice_loc_index(), slice_loc);
        }
    }
}
