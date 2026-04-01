use rubikscube::*;
use kiss3d::prelude::*;

fn pos_from_location(s: impl ToString) -> Vec3 {
    let mut v = Vec3::ZERO;
    for c in s.to_string().chars() {
        match c {
            'L' => v.x = -1.0,
            'R' => v.x = 1.0,
            'U' => v.y = 1.0,
            'D' => v.y = -1.0,
            'F' => v.z = 1.0,
            'B' => v.z = -1.0,
            _ => panic!("Invalid position character: {}", c),
        }
    }
    v
}

fn color_from_location(c: char) -> Color {
    match c {
        'U' => WHITE,
        'F' => RED,
        'R' => BLUE,
        'B' => ORANGE,
        'L' => GREEN,
        'D' => YELLOW,
        _ => panic!("Invalid colour character: {}", c),
    }
}

fn twist_from_char(c: char) -> Twist {
    match c {
        'L' => Twist::L1,
        'R' => Twist::R1,
        'U' => Twist::U1,
        'D' => Twist::D1,
        'F' => Twist::F1,
        'B' => Twist::B1,
        _ => panic!("Invalid twist character: {}", c),
    }
}

fn axis_from_rotation(rot: Rotation) -> Vec3 {
    match rot {
        Rotation::L => Vec3::new(-1.0, 0.0, 0.0),
        Rotation::U => Vec3::new(0.0, 1.0, 0.0),
        Rotation::F => Vec3::new(0.0, 0.0, 1.0),
    }
}

fn create_scene(corners: Corners, edges: Edges) -> (SceneNode3d, RubiksCube) {
    let mut scene = SceneNode3d::empty();
    scene.add_light(Light::point(50.0))
        .set_position(Vec3::new(5.0, 8.0, 5.0));
    let cube = RubiksCube::new(&mut scene, corners, edges);
    (scene, cube)
}

struct Cubie {
    node: SceneNode3d,
    pos: Vec3,
}

impl Cubie {
    pub fn new(scene: &mut SceneNode3d, color: &str, location: &str) -> Self {
        let mut group = scene.add_group();
        let mut core = group.add_cube(1.0, 1.0, 1.0);
        core.set_color(BLACK);
        for (c, l) in color.chars().zip(location.chars()) {
            let mut face = group.add_cube(0.9, 0.9, 0.9);
            face.set_color(color_from_location(c));
            face.set_position(pos_from_location(l) * 0.1);
        }
        let pos = pos_from_location(location);
        group.set_position(pos);
        Self { node: group, pos }
    }
}

struct RubiksCube {
    cubies: Vec<Cubie>,
}

impl RubiksCube {
    const CENTER_LOCATIONS: [&str; 6] = ["U", "F", "R", "B", "L", "D"];
    const EDGE_LOCATIONS: [&str; 12] = [
        "UF", "UB", "DB", "DF",
        "UL", "UR", "DR", "DL",
        "LF", "RF", "RB", "LB",
    ];
    const CORNER_LOCATIONS: [&str; 8] = [
        "UFL", "URF", "ULB", "UBR",
        "DLF", "DFR", "DBL", "DRB",
    ];

    pub fn new(scene: &mut SceneNode3d, corners: Corners, edges: Edges) -> Self {
        let mut cubies: Vec<Cubie> = Self::CENTER_LOCATIONS
            .iter()
            .map(|&loc| Cubie::new(scene, loc, loc))
            .collect();

        for i in 0..12 {
            let cubie = edges.prm()[i];
            let orientation = edges.ori()[i];
            let color: String = Self::EDGE_LOCATIONS[cubie]
                .chars()
                .cycle()
                .skip(2 - orientation)
                .take(2)
                .collect();
            cubies.push(Cubie::new(scene, &color, Self::EDGE_LOCATIONS[i]));
        }

        for i in 0..8 {
            let cubie = corners.prm()[i];
            let orientation = corners.ori()[i];
            let color: String = Self::CORNER_LOCATIONS[cubie]
                .chars()
                .cycle()
                .skip(3 - orientation)
                .take(3)
                .collect();
            cubies.push(Cubie::new(scene, &color, Self::CORNER_LOCATIONS[i]));
        }

        Self { cubies }
    }

    /// Rotate a single face by the given angle (in radians)
    pub fn rotate_face(&mut self, face: char, angle: f32) {
        let axis = pos_from_location(face);
        let rot = Quat::from_axis_angle(-axis.normalize(), angle);
        for cubie in &mut self.cubies {
            if cubie.pos.dot(axis) > 0.5 {
                cubie.node.append_rotation(rot);
                cubie.pos = rot * cubie.pos;
            }
        }
    }

    /// Rotate the entire cube by the given angle around the rotation axis
    pub fn rotate_whole(&mut self, rotation: Rotation, angle: f32) {
        let axis = axis_from_rotation(rotation);
        let rot = Quat::from_axis_angle(axis, angle);
        for cubie in &mut self.cubies {
            cubie.node.append_rotation(rot);
            cubie.pos = rot * cubie.pos;
        }
    }
}

const ANIMATION_SPEED: f32 = std::f32::consts::FRAC_PI_2 / 120.0;

enum Animation {
    /// Face twist animation (L, R, U, D, F, B keys)
    FaceTwist { face: char, remaining: f32 },
    /// Whole cube rotation animation (Q, W, E keys)
    CubeRotation { rotation: Rotation, remaining: f32 },
}

impl Animation {
    fn new_face_twist(face: char) -> Self {
        Animation::FaceTwist {
            face,
            remaining: std::f32::consts::FRAC_PI_2,
        }
    }

    fn new_cube_rotation(rotation: Rotation) -> Self {
        Animation::CubeRotation {
            rotation,
            remaining: std::f32::consts::FRAC_PI_2,
        }
    }

    /// Advance the animation by one step, returning true if finished
    fn step(&mut self, cube: &mut RubiksCube) -> bool {
        match self {
            Animation::FaceTwist { face, remaining } => {
                let step = ANIMATION_SPEED.min(*remaining);
                cube.rotate_face(*face, step);
                *remaining -= step;
                *remaining <= 0.0
            }
            Animation::CubeRotation { rotation, remaining } => {
                let step = ANIMATION_SPEED.min(*remaining);
                cube.rotate_whole(*rotation, step);
                *remaining -= step;
                *remaining <= 0.0
            }
        }
    }
}

struct CubeState {
    corners: Corners,
    edges: Edges,
}

impl CubeState {
    fn solved() -> Self {
        Self {
            corners: Corners::solved(),
            edges: Edges::solved(),
        }
    }

    fn apply_twist(&mut self, twist: Twist) {
        self.corners = self.corners.twisted(twist);
        self.edges = self.edges.twisted(twist);
    }

    fn apply_rotation(&mut self, rotation: Rotation) {
        self.corners = self.corners.rotated_colours(rotation);
        self.edges = self.edges.rotated_colours(rotation);
    }

    fn reset(&mut self) {
        *self = Self::solved();
    }
}

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Rubik's Cube").await;
    let mut camera = OrbitCamera3d::new(Vec3::new(3.0, 4.0, 6.0), Vec3::ZERO);
    window.set_background_color(GRAY);

    let mut state = CubeState::solved();
    let (mut scene, mut cube) = create_scene(state.corners, state.edges);

    let mut animation: Option<Animation> = None;

    while window.render_3d(&mut scene, &mut camera).await {
        // Handle input when no animation is running
        if animation.is_none() {
            animation = handle_input(&window, &mut state, &mut scene, &mut cube);
        }

        // Step the running animation
        if let Some(ref mut anim) = animation {
            let finished = anim.step(&mut cube);
            if finished {
                match anim {
                    Animation::FaceTwist { face, .. } => {
                        state.apply_twist(twist_from_char(*face));
                    }
                    Animation::CubeRotation { rotation, .. } => {
                        state.apply_rotation(*rotation);
                    }
                }
                (scene, cube) = create_scene(state.corners, state.edges);
                animation = None;
            }
        }
    }
}

fn handle_input(
    window: &Window,
    state: &mut CubeState,
    scene: &mut SceneNode3d,
    cube: &mut RubiksCube,
) -> Option<Animation> {
    // Face twist keys
    const FACE_KEYS: [(Key, char); 6] = [
        (Key::L, 'L'), (Key::R, 'R'),
        (Key::U, 'U'), (Key::D, 'D'),
        (Key::F, 'F'), (Key::B, 'B'),
    ];

    for (key, face) in FACE_KEYS {
        if window.get_key(key) == kiss3d::event::Action::Press {
            return Some(Animation::new_face_twist(face));
        }
    }

    // Cube rotation keys
    const ROTATION_KEYS: [(Key, Rotation); 3] = [
        (Key::Q, Rotation::L),
        (Key::W, Rotation::U),
        (Key::E, Rotation::F),
    ];

    for (key, rotation) in ROTATION_KEYS {
        if window.get_key(key) == kiss3d::event::Action::Press {
            return Some(Animation::new_cube_rotation(rotation));
        }
    }

    // Reset key (immediate, no animation)
    if window.get_key(Key::S) == kiss3d::event::Action::Press {
        state.reset();
        let (new_scene, new_cube) = create_scene(state.corners, state.edges);
        *scene = new_scene;
        *cube = new_cube;
    }

    None
}
