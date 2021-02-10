extern crate kiss3d;
extern crate rubik;

mod parsing;
mod display;
mod algo;
mod pruning;

use pruning::pruning::{Pruning};
use algo::solve::*;
use parsing::parse::*;
// use display::display::{Display};
// use display::gl_rubik::{GlRubik};
use parsing::args::{Config};
use rubik::cubie_cube::{CubieCube};
// use nalgebra::{Vector3, UnitQuaternion, Unit};
use rubik::enums::{ACTIONS_STR_LIST};

fn main() {
    let config: Config = Config::new();
    let mut pruning_tables: Pruning = Pruning::new();
    
    println!("flip_pruning_table : {:?}", pruning_tables.flip_pruning_table);
    println!("twist_pruning_table : {:?}", pruning_tables.twist_pruning_table);
    println!("uds_e_location_pruning_table : {:?}", pruning_tables.uds_e_location_pruning_table);
    
    let input_sequence: Vec<usize> = parse_inputs(&config);
    println!("visualisator: {}{}", config.visualisator, if config.visualisator { format!(" | speed: {}", config.speed_selection) } else { String::from("") });
    println!("sequence: {}", input_sequence.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" "));

    let mut cb_cube: CubieCube = CubieCube::new_solved();
    cb_cube.set_uds_e_sorted_coord(0);
    println!("c_p_coord: {}",  cb_cube.get_uds_e_sorted_coord());
    cb_cube.set_uds_e_sorted_coord(23);
    println!("c_p_coord: {}",  cb_cube.get_uds_e_sorted_coord());
    cb_cube.set_uds_e_sorted_coord(12);
    println!("c_p_coord: {}",  cb_cube.get_uds_e_sorted_coord());
    cb_cube.set_uds_e_sorted_coord(6);
    println!("c_p_coord: {}",  cb_cube.get_uds_e_sorted_coord());
    cb_cube.set_uds_e_sorted_coord(1);
    println!("c_p_coord: {}",  cb_cube.get_uds_e_sorted_coord());
    cb_cube.set_uds_e_sorted_coord(0);
    println!("c_p_coord: {}",  cb_cube.get_uds_e_sorted_coord());

    cb_cube.apply_sequence(&input_sequence);
    match solve(&mut cb_cube, 20, pruning_tables) {
        Some(solution) => eprintln!("solution: {}", solution.iter().map(|a| ACTIONS_STR_LIST[*a]).collect::<Vec<&str>>().join(" ")),
        None => println!("Search timed out without finding any solution")
    }

    // if config.visualisator {
    //     let mut display: Display = Display::new(&config);
    //     let mut gl_rubik: GlRubik = GlRubik::new(&mut display.window);

    //     let rubik_rot = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), (0.5 as f32).to_radians());

    //     let mut current_angle: f32 = 0.0;
    //     let mut current_cubies: Vec<Cubie> = gl_rubik.get_face_cubies(&input_sequence[display.moves].face); // stupid init, find something else
    //     let mut current_axis: Unit::<Vector3::<f32>> = gl_rubik.get_face_axis(&input_sequence[display.moves].face); // stupid init, find something else
    //     let mut sequence: Vec<Action> = input_sequence.clone();
    //     while display.window.render_with_camera(&mut display.camera) {
    //         display.handle_events();
    //         if display.started {
    //             if !display.animating {
    //                 if display.moves == sequence.len() {
    //                     display.started = false;
    //                     display.moves = 0;
    //                     sequence = solution.clone();
    //                 }
    //                 current_cubies = gl_rubik.get_face_cubies(&sequence[display.moves].face);
    //                 current_axis = gl_rubik.get_face_axis(&sequence[display.moves].face);
    //                 current_angle = 0.0;
    //                 display.animating = true;
    //             } else if display.animating {
    //                 let angle = sequence[display.moves].rot.to_angle().unwrap().signum() * display.speed;
    //                 let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&current_axis, angle.to_radians());
    //                 for cubie in current_cubies.iter_mut() {
    //                     cubie.rotate(rot);
    //                 }
    //                 current_angle += angle;
    //                 if current_angle == sequence[display.moves].rot.to_angle().unwrap() {
    //                     display.animating = false;
    //                     display.moves += 1;
    //                 }
    //             }
    //         }
    //         if display.rotating {
    //             gl_rubik.scene_node.append_rotation(&rubik_rot);
    //         }
    //     }
    // }
}