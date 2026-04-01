use rubikscube::*;
use std::collections::HashMap;

/// Parse a string of space-separated twist names into a vector of Twist values.
/// Anything onwards from '#' is ignored.
fn parse_line(input: &str) -> Vec<Twist> {
    input.split('#').next().unwrap_or("") // Remove comment
    .split_whitespace().map(|s| s.parse().unwrap()).collect() // Parse
}

fn read_twist_file(path: &str) -> Vec<Vec<Twist>> {
    let content = std::fs::read_to_string(path).unwrap();
    content.lines()
        .map(|line| parse_line(line))
        .collect()
}

fn read_config(path: &str) -> HashMap<String, String> {
    let content = std::fs::read_to_string(path).unwrap();
    content.lines()
        .map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap().to_string();
            let value = parts.next().unwrap().to_string();
            (key, value)
        })
        .collect()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <path_to_pos_file>", args[0]);
        std::process::exit(1);
    }
    let pos_file_path = &args[1];

    let config = read_config("config.txt");

    let corners_table_path = config.get("corners_table").unwrap();
    let subset_table_path = config.get("subset_table").unwrap();
    let coset_table_path = config.get("coset_table").unwrap();

    let twister = Twister::new();
    let corners_table = corners_distance_table(&twister, corners_table_path);
    let subset_table = subset_distance_table(&twister, subset_table_path);
    let coset_table = coset_direction_table(&twister, coset_table_path);

    // let mut coset_solver = CosetCover::new(
    //     &twister,
    //     &subset_table,
    //     20,
    // );

    // let mut rnd = RandomTwistGen::new(42, TwistSet::full());
    // let coset_cube = CosetCube::solved().twisted_by(&twister, &rnd.gen_twists(100));
    // println!("Covering coset with subset distance {}", coset_table.distance(coset_cube.index()));

    // coset_solver.reset_for(coset_cube);
    
    // let path_iterator = CosetToSubsetPathsIterator::new(
    //     &twister,
    //     &coset_table,
    //     coset_cube,
    // );
    
    // let mut counter = 0;
    // let start = std::time::Instant::now();
    // for twists in path_iterator {
    //     counter += 1;
    //     if counter % 100000 == 0 {
    //         println!("Generated {} paths in {:?}. speed: {:.2} paths/sec", counter, start.elapsed(), counter as f64 / start.elapsed().as_secs_f64());
    //     }
    //     // coset_solver.cover_with(&twists);
    // }

    let mut solver = TwoPhaseSolver::new(
        &twister,
        &coset_table,
        &subset_table,
        &corners_table,
    );

    let mut total_time = std::time::Duration::ZERO;
    let twist_sequences = read_twist_file(pos_file_path);
    
    for (i, twists) in twist_sequences.iter().enumerate() {
        let cube = Cube::solved().twisted_by(&twister, twists);

        let start = std::time::Instant::now();
        let solution = solver.solve(cube, 20).unwrap();
        let elapsed = start.elapsed();
        total_time += elapsed;
        
        // Verify solution
        if !cube.twisted_by(&twister, &solution).is_solved() {
            panic!("Incorrect solution found on line {}! Solution: {:?}", i + 1, solution);
        }
    }
    
    println!("Total time taken: {:?}", total_time);
    if twist_sequences.len() > 0 {
        println!("Average time per solve: {:?}", total_time / twist_sequences.len() as u32);
    }
    solver.print_stats();
}

// use kiss3d::prelude::*;

// fn pos_from_location(s: impl ToString) -> Vec3 {
//     let mut v = Vec3::ZERO;
//     for c in s.to_string().chars() {
//         match c {
//             'L' => v.x = -1.0,
//             'R' => v.x = 1.0,
//             'U' => v.y = 1.0,
//             'D' => v.y = -1.0,
//             'F' => v.z = 1.0,
//             'B' => v.z = -1.0,
//             _ => panic!("Invalid position character: {}", c),
//         }
//     }
//     v
// }

// fn color_from_location(c: char) -> Color {
//      match c {
//         'U' => WHITE,
//         'F' => RED,
//         'R' => BLUE,
//         'B' => ORANGE,
//         'L' => GREEN,
//         'D' => YELLOW,
//         _ => panic!("Invalid colour character: {}", c),
//     }
// }

// struct Cubie
// {
//     node: SceneNode3d,
//     pos: Vec3,
// }

// impl Cubie {
//     pub fn new_cubie(scene: &mut SceneNode3d, color: &str, location: &str) -> Self {
//         let mut group = scene.add_group();
//         let mut core = group.add_cube(1.0, 1.0, 1.0);
//         core.set_color(BLACK);
//         for (c, l) in color.chars().zip(location.chars()) {
//             let mut face = group.add_cube(0.9, 0.9, 0.9);
//             face.set_color(color_from_location(c));
//             face.set_position(pos_from_location(l) * 0.1);
//         }
//         let pos = pos_from_location(location);
//         group.set_position(pos);
//         Self { node: group, pos }
//     }
// }

// struct Cube {
//     cubies: Vec<Cubie>
// }

// impl Cube {
//     pub fn new(scene: &mut SceneNode3d, corners: Corners, edges: Edges) -> Self {
//         const CENTER_LOCATIONS: [&str; 6] = ["U", "F", "R", "B", "L", "D"];
//         const EDGES_LOCATIONS: [&str; 12] = [
//             "UF", "UB", "DB", "DF",
//             "UL", "UR", "DR", "DL",
//             "LF", "RF", "RB", "LB"
//         ];
//         const CORNER_LOCATIONS: [&str; 8] = [
//             "UFL", "URF", "ULB", "UBR",
//             "DLF", "DFR", "DBL", "DRB"
//         ];
//         let mut cubies: Vec<Cubie> = CENTER_LOCATIONS.iter().map(|&loc| Cubie::new_cubie(scene, loc, loc)).collect();
//         for i in 0..12 {
//             let cubie = edges.prm()[i];
//             let orientation = edges.ori()[i];
//             let color: String = EDGES_LOCATIONS[cubie].chars().cycle().skip(2 - orientation).take(2).collect();
//             cubies.push(Cubie::new_cubie(scene, &color, EDGES_LOCATIONS[i]));
//         }
//         for i in 0..8 {
//             let cubie = corners.prm()[i];
//             let orientation = corners.ori()[i];
//             let color: String = CORNER_LOCATIONS[cubie].chars().cycle().skip(3 - orientation).take(3).collect();
//             cubies.push(Cubie::new_cubie(scene, &color, CORNER_LOCATIONS[i]));
//         }
//         Self { cubies }
//     }

//     pub fn twist_face(&mut self, face: char, angle: f32) {
//         let pos = pos_from_location(face);
//         let rot = Quat::from_axis_angle(-pos.normalize(), angle);
//         for cubie in &mut self.cubies {
//             if cubie.pos.dot(pos) > 0.5 {
//                 cubie.node.append_rotation(rot);
//                 cubie.pos = rot * cubie.pos;
//             }
//         }
//     }
// }


// struct Animation {
//     face: char,
//     remaining: f32,
// }

// impl Animation {
//     const SPEED: f32 = std::f32::consts::FRAC_PI_2 / 20.0;

//     fn new(face: char) -> Self {
//         Self { face, remaining: std::f32::consts::FRAC_PI_2 }
//     }
// }

// fn twist_from_char(c: char) -> Twist {
//     match c {
//         'L' => Twist::L1,
//         'R' => Twist::R1,
//         'U' => Twist::U1,
//         'D' => Twist::D1,
//         'F' => Twist::F1,
//         'B' => Twist::B1,
//         _ => panic!("Invalid twist character: {}", c),
//     }
// }

// #[kiss3d::main]
// async fn main() {
//     let mut window = Window::new("Rubik's Cube").await;
//     let mut camera = OrbitCamera3d::new(Vec3::new(3.0, 4.0, 6.0), Vec3::ZERO);

//     let mut scene = SceneNode3d::empty();
//     scene.add_light(Light::point(50.0))
//         .set_position(Vec3::new(5.0, 8.0, 5.0));

//     window.set_background_color(GRAY);

//     let mut corners = Corners::solved();
//     let mut edges = Edges::solved();
//     let mut cube = Cube::new(&mut scene, corners, edges);

//     let key_face_pairs = [
//         (Key::L, 'L'), (Key::R, 'R'),
//         (Key::U, 'U'), (Key::D, 'D'),
//         (Key::F, 'F'), (Key::B, 'B'),
//     ];
//     let mut animation: Option<Animation> = None;
//     let mut q_was_pressed = false;
//     let mut w_was_pressed = false;
//     let mut e_was_pressed = false;

//     while window.render_3d(&mut scene, &mut camera).await {
//         let q_is_pressed = window.get_key(Key::Q) == kiss3d::event::Action::Press;
//         let w_is_pressed = window.get_key(Key::W) == kiss3d::event::Action::Press;
//         let e_is_pressed = window.get_key(Key::E) == kiss3d::event::Action::Press;
//         if animation.is_none() {
//             for (key, face) in key_face_pairs.iter() {
//                 if window.get_key(*key) == kiss3d::event::Action::Press {
//                     animation = Some(Animation::new(*face));
//                     break;
//                 }
//             }
//             let colour_rot = if q_is_pressed && !q_was_pressed {
//                 Some(Rotation::L)
//             } else if w_is_pressed && !w_was_pressed {
//                 Some(Rotation::U)
//             } else if e_is_pressed && !e_was_pressed {
//                 Some(Rotation::F)
//             } else {
//                 None
//             };
//             if let Some(ct) = colour_rot {
//                 corners = corners.rotated_colours(ct);
//                 edges = edges.rotated_colours(ct);
//                 scene = SceneNode3d::empty();
//                 scene.add_light(Light::point(50.0))
//                     .set_position(Vec3::new(5.0, 8.0, 5.0));
//                 window.set_background_color(GRAY);
//                 cube = Cube::new(&mut scene, corners, edges);
//             }
//             if window.get_key(Key::S) == kiss3d::event::Action::Press {
//                 corners = Corners::solved();
//                 edges = Edges::solved();
//                 scene = SceneNode3d::empty();
//                 scene.add_light(Light::point(50.0))
//                     .set_position(Vec3::new(5.0, 8.0, 5.0));
//                 window.set_background_color(GRAY);
//                 cube = Cube::new(&mut scene, corners, edges);
//             }
//         }
//         q_was_pressed = q_is_pressed;
//         w_was_pressed = w_is_pressed;
//         e_was_pressed = e_is_pressed;

//         // Step the running animation
//         if let Some(ref mut anim) = animation {
//             let step = Animation::SPEED.min(anim.remaining);
//             cube.twist_face(anim.face, step);
//             anim.remaining -= step;
//             if anim.remaining <= 0.0 {
//                 corners = corners.twisted(twist_from_char(anim.face));
//                 edges = edges.twisted(twist_from_char(anim.face));
//                 scene = SceneNode3d::empty();
//                 scene.add_light(Light::point(50.0))
//                     .set_position(Vec3::new(5.0, 8.0, 5.0));

//                 window.set_background_color(GRAY);
//                 cube = Cube::new(&mut scene, corners, edges);
//                 animation = None;
//             }
//         }
//     }
// }
