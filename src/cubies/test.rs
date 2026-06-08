#[cfg(test)]
mod tests {
    use crate::corners::*;
    use crate::edges::*;
    use crate::twist_generator::*;
    use crate::twist::*;
    use std::ops::Mul;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Cubies {
        pub corners: Corners,
        pub edges: Edges,
    }

    impl Cubies {
        pub fn solved() -> Self { Self { corners: Corners::solved(), edges: Edges::solved() } }

        pub fn twist(twist: Twist) -> Self { Self { corners: Corners::twist(twist), edges: Edges::twist(twist) } }

        pub fn twists(twists: &[Twist]) -> Self { Self { corners: Corners::twists(twists), edges: Edges::twists(twists) } }
        
        pub fn inverse(&self) -> Self { Self { corners: self.corners.inverse(), edges: self.edges.inverse() } }

        pub fn conjugated_by(&self, rot: Axis) -> Self { Self { corners: self.corners.conjugated_by(rot), edges: self.edges.conjugated_by(rot) } }
    }

    impl Mul for Cubies {
        type Output = Cubies;

        fn mul(self, rhs: Cubies) -> Cubies {
            Cubies { corners: self.corners * rhs.corners, edges: self.edges * rhs.edges }
        }
    }

    impl Mul<Cubies> for Twist {
        type Output = Cubies;

        fn mul(self, rhs: Cubies) -> Cubies {
            Cubies { corners: self * rhs.corners, edges: self * rhs.edges }
        }
    }

    fn cycle_length(twists: &[Twist]) -> usize {
        let mut current = Cubies::twists(twists);
        let mut length = 1;
        while current != Cubies::solved() {
            current = Cubies::twists(twists) * current;
            length += 1;
        }
        length
    }

    #[test]
    fn test_twist_commutation() {
        let l = Cubies::twist(Twist::L1);
        let r = Cubies::twist(Twist::R1);
        let u = Cubies::twist(Twist::U1);
        let d = Cubies::twist(Twist::D1);
        let f = Cubies::twist(Twist::F1);
        let b = Cubies::twist(Twist::B1);
        
        assert_eq!(l * r, r * l, "L and R should commute");
        assert_eq!(u * d, d * u, "U and D should commute");
        assert_eq!(f * b, b * f, "F and B should commute");
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
            let rnd_cube = Cubies::twists(&rnd.gen_twists(100));
            assert_eq!(rnd_cube * rnd_cube.inverse(), Cubies::solved(), "Cube multiplied by its inverse should yield the solved state, failed for cube {:?}", rnd_cube);
        }
    }

    #[test]
    fn test_conjugation() {
        // Trivial cases (conjugating the solved state should yield the solved state)
        assert_eq!(Cubies::solved().conjugated_by(Axis::X), Cubies::solved());
        assert_eq!(Cubies::solved().conjugated_by(Axis::Y), Cubies::solved());
        assert_eq!(Cubies::solved().conjugated_by(Axis::Z), Cubies::solved());

        // Some simple cases
        assert_eq!(Cubies::twist(Twist::F1).conjugated_by(Axis::X), Cubies::twist(Twist::D1));
        assert_eq!(Cubies::twist(Twist::L1).conjugated_by(Axis::Y), Cubies::twist(Twist::U1));
        assert_eq!(Cubies::twist(Twist::F1).conjugated_by(Axis::Z), Cubies::twist(Twist::R1));

        // Fuzzing
        let mut rnd = RandomTwistGen::new(12345678, &ALL_TWISTS);
        for _ in 0..100_000 {
            let rnd_twists = rnd.gen_twists(100);
            let rnd_cube = Cubies::twists(&rnd_twists);
            for rot in [Axis::X, Axis::Y, Axis::Z] {
                let conj_cube = rnd_cube.conjugated_by(rot);
                let conj_twists = conjugate_by_inv(&conjugate_by_inv(&conjugate_by_inv(&rnd_twists, rot), rot), rot);
                assert_eq!(Cubies::twists(&conj_twists), conj_cube, "Failed for cube {:?} and rotation {:?}", rnd_cube, rot);
            }
        }
    }
}