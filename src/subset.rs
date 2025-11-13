use crate::corners::*;
use crate::edges::*;

pub struct C_prm(u16); // 0..=40'319
pub struct C_ori(u16); // 0..=2'186
pub struct E_non_slice_prm(u16); // 0..=40'319
pub struct E_slice_prm(u8); // 0..=23
pub struct E_slice_loc(u16); // 0..=494
pub struct E_ori(u16); // 0..=2'047

pub fn subset_index(
    c_prm: C_prm,
    e_non_slice_prm: E_non_slice_prm,
    e_slice_prm: E_slice_prm,
) -> u64 {
    const SLICE_PRM_SIZE: u64 = Edges::SLICE_PRM_SIZE as u64;
    const NON_SLICE_PRM_SIZE: u64 = Edges::NON_SLICE_PRM_SIZE as u64;

    (c_prm as u64 / 2) * SLICE_PRM_SIZE * NON_SLICE_PRM_SIZE
    + (e_non_slice_prm as u64) * SLICE_PRM_SIZE
    + (e_slice_prm as u64)
}

pub fn coset_index(
    c_ori: C_ori,
    e_ori: E_ori,
    e_slice_loc: E_slice_loc,
) -> u32 {
    const E_ORI_SIZE: u32 = Edges::ORI_SIZE as u32;
    const SLICE_LOC_SIZE: u32 = Edges::SLICE_LOC_SIZE as u32;

    (c_ori as u32) * (E_ORI_SIZE * SLICE_LOC_SIZE)
    + (e_ori as u32) * SLICE_LOC_SIZE
    + (e_slice_loc as u32)
}

pub fn in_subset(
    c_ori: C_ori,
    e_ori: E_ori,
    e_slice_loc: E_slice_loc,
) -> bool {
    c_ori == 0 && e_ori == 0 && e_slice_loc == 494
}