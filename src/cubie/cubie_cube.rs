use crate::twist::*;
use super::corners::*;
use super::edges::*;
use std::ops::Mul;

/// Cubie representation of a Rubik's Cube.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CubieCube {
    pub corners: Corners,
    pub edges: Edges,
}

impl CubieCube {
    pub fn solved() -> Self {
        Self {
            corners: Corners::solved(),
            edges: Edges::solved()
        }
    }

    pub fn twist(twist: Twist) -> Self {
        Self {
            corners: Corners::twist(twist),
            edges: Edges::twist(twist)
        }
    }

    pub fn twists(twists: &[Twist]) -> Self {
        Self {
            corners: Corners::twists(twists),
            edges: Edges::twists(twists)
        }
    }

    pub fn inverse(&self) -> Self {
        Self {
            corners: self.corners.inverse(),
            edges: self.edges.inverse()
        }
    }

    pub fn conjugated_by(&self, rot: Axis) -> Self {
        Self {
            corners: self.corners.conjugated_by(rot),
            edges: self.edges.conjugated_by(rot)
        }
    }
}

/// CubieCube * CubieCube
impl Mul for CubieCube {
    type Output = CubieCube;

    fn mul(self, rhs: CubieCube) -> CubieCube {
        CubieCube { corners: self.corners * rhs.corners, edges: self.edges * rhs.edges }
    }
}

/// Twist * CubieCube
impl Mul<CubieCube> for Twist {
    type Output = CubieCube;

    fn mul(self, rhs: CubieCube) -> CubieCube {
        CubieCube { corners: self * rhs.corners, edges: self * rhs.edges }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycle_length(twists: &[Twist]) -> usize {
        let mut current = CubieCube::twists(twists);
        let mut length = 1;
        while current != CubieCube::solved() {
            current = CubieCube::twists(twists) * current;
            length += 1;
        }
        length
    }

    #[test]
    fn test_twist_commutation() {
        let l = CubieCube::twist(Twist::L1);
        let r = CubieCube::twist(Twist::R1);
        let u = CubieCube::twist(Twist::U1);
        let d = CubieCube::twist(Twist::D1);
        let f = CubieCube::twist(Twist::F1);
        let b = CubieCube::twist(Twist::B1);
        
        assert_eq!(l * r, r * l, "L and R should commutate");
        assert_eq!(u * d, d * u, "U and D should commutate");
        assert_eq!(f * b, b * f, "F and B should commutate");
    }

    #[test]
    fn test_twist_cycles() {
        for t in [Twist::L2, Twist::R2, Twist::U2, Twist::D2, Twist::F2, Twist::B2] {
            assert_eq!(cycle_length(&[t]), 2, "Twist {:?} should have cycle length 2", t);
        }
        for t in [Twist::L1, Twist::L3, Twist::R1, Twist::R3, Twist::U1, Twist::U3, Twist::D1, Twist::D3, Twist::F1, Twist::F3, Twist::B1, Twist::B3] {
            assert_eq!(cycle_length(&[t]), 4, "Twist {:?} should have cycle length 4", t);
        }
        for t in [Twist::U1, Twist::D1, Twist::F1, Twist::B1] {
            assert_eq!(cycle_length(&[Twist::L1, t]), 105, "Twists L1 and {:?} should have cycle length 105", t);
        }
        for t in [Twist::U2, Twist::D2, Twist::F2, Twist::B2] {
            assert_eq!(cycle_length(&[Twist::L1, t]), 30, "Twists L1 and {:?} should have cycle length 30", t);
        }
        for t in [Twist::U3, Twist::D3, Twist::F3, Twist::B3] {
            assert_eq!(cycle_length(&[Twist::L1, t]), 63, "Twists L1 and {:?} should have cycle length 63", t);
        }
        assert_eq!(cycle_length(&[Twist::R1, Twist::U2, Twist::D3, Twist::B1, Twist::D3]), 1260);
    }

    #[test]
    fn test_inverse() {
        // Fuzzing
        let mut rnd = RandomTwistGen::new(12345678, &ALL_TWISTS);
        for _ in 0..100_000 {
            let rnd_cube = CubieCube::twists(&rnd.gen_twists(100));
            assert_eq!(rnd_cube * rnd_cube.inverse(), CubieCube::solved(), "A cube multiplied by its inverse should yield the solved state, failed for cube {:?}", rnd_cube);
            assert_eq!(rnd_cube.inverse() * rnd_cube, CubieCube::solved(), "The inverse of a cube multiplied by the cube should yield the solved state, failed for cube {:?}", rnd_cube);
        }
    }

    #[test]
    fn test_conjugation() {
        // Trivial cases (conjugating the solved state should yield the solved state)
        assert_eq!(CubieCube::solved().conjugated_by(Axis::X), CubieCube::solved());
        assert_eq!(CubieCube::solved().conjugated_by(Axis::Y), CubieCube::solved());
        assert_eq!(CubieCube::solved().conjugated_by(Axis::Z), CubieCube::solved());

        // Some simple cases
        assert_eq!(CubieCube::twist(Twist::F1).conjugated_by(Axis::X), CubieCube::twist(Twist::D1));
        assert_eq!(CubieCube::twist(Twist::L1).conjugated_by(Axis::Y), CubieCube::twist(Twist::U1));
        assert_eq!(CubieCube::twist(Twist::F1).conjugated_by(Axis::Z), CubieCube::twist(Twist::R1));

        // Fuzzing
        let mut rnd = RandomTwistGen::new(12345678, &ALL_TWISTS);
        for _ in 0..100_000 {
            let rnd_twists = rnd.gen_twists(100);
            let rnd_cube = CubieCube::twists(&rnd_twists);
            for rot in [Axis::X, Axis::Y, Axis::Z] {
                let conj_cube = rnd_cube.conjugated_by(rot);
                let conj_twists = conjugate_by_inv(&conjugate_by_inv(&conjugate_by_inv(&rnd_twists, rot), rot), rot);
                assert_eq!(CubieCube::twists(&conj_twists), conj_cube, "Failed for cube {:?} and rotation {:?}", rnd_cube, rot);
            }
        }
    }
}