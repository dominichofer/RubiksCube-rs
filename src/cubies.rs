#[cfg(test)]
mod tests {
    use crate::corners::*;
    use crate::edges::*;
    use crate::twist::*;
    use crate::twist_set::*;
    use crate::twist_generator::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    struct Cubies {
        pub corners: Corners,
        pub edges: Edges,
    }

    impl Cubies {
        pub fn solved() -> Self {
            Self {
                corners: Corners::solved(),
                edges: Edges::solved(),
            }
        }

        pub fn twisted(&self, twist: Twist) -> Self {
            Self {
                corners: self.corners.twisted(twist),
                edges: self.edges.twisted(twist),
            }
        }

        pub fn twisted_by(&self, twists: &[Twist]) -> Self {
            twists.iter().fold(*self, |cube, &twist| {
                cube.twisted(twist)
            })
        }

        pub fn rotated_colours(&self, rot: Rotation) -> Self {
            Self {
                corners: self.corners.rotated_colours(rot),
                edges: self.edges.rotated_colours(rot),
            }
        }
    }

    #[test]
    fn test_twists_cycle() {
        let mut rnd = RandomTwistGen::new(3423598, TwistSet::full());
        let mut c = Cubies::solved();
        for _ in 0..100_000 {
            c = c.twisted(rnd.gen_twist());
            for t in TwistSet::full().iter() {
                assert_eq!(c.twisted_by(&[t, t, t, t]), c, "Twist {:?} did not cycle correctly after 4 applications", t);
            }
        }
    }

    fn commute(a: Twist, b: Twist, cube: Cubies) -> bool {
        cube.twisted_by(&[a, b]) == cube.twisted_by(&[b, a])
    }

    #[test]
    fn test_twist_commutation() {
        let mut rnd = RandomTwistGen::new(32468723, TwistSet::full());
        let mut c = Cubies::solved();
        for _ in 0..100_000 {
            c = c.twisted(rnd.gen_twist());
            assert!(commute(Twist::L1, Twist::R1, c));
            assert!(commute(Twist::U1, Twist::D1, c));
            assert!(commute(Twist::F1, Twist::B1, c));
        }
    }

    #[test]
    fn test_rotated_colours() {
        let solved = Cubies::solved();

        // Trivial cases
        assert!(solved.rotated_colours(Rotation::L) == solved);
        assert!(solved.rotated_colours(Rotation::U) == solved);
        assert!(solved.rotated_colours(Rotation::F) == solved);

        // Some simple cases
        assert!(solved.twisted(Twist::F1).rotated_colours(Rotation::L) == solved.twisted(Twist::U1));
        assert!(solved.twisted(Twist::F1).rotated_colours(Rotation::U) == solved.twisted(Twist::R1));
        assert!(solved.twisted(Twist::L1).rotated_colours(Rotation::F) == solved.twisted(Twist::D1));

        // Fuzzing
        let mut rnd = RandomTwistGen::new(12345678, TwistSet::full());
        for _ in 0..100_000 {
            let twists = rnd.gen_twists(1);
            for rot in [Rotation::L, Rotation::U, Rotation::F].iter() {
                let rotated_twists = twists.iter().map(|&t| t.counter_rotated(*rot)).collect::<Vec<_>>();
                assert!(solved.twisted_by(&twists).rotated_colours(*rot) == solved.twisted_by(&rotated_twists), "Failed for twists {:?} and rotation {:?}", twists, rot);
            }
        }
    }
}