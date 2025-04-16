// use std::f32::INFINITY;

use draw_utils::Drawable;
use macroquad::prelude::*;
// use winit::raw_window_handle::XcbDisplayHandle;
// use std::f32::consts::PI;

mod draw_utils;
mod movement;

#[macroquad::main("nisem jost")]
async fn main() {
    let test_cube = draw_utils::Cube {
        position: vec3(10., 0., 0.),
        size: vec3(5., 5., 5.),
        color: BLACK,
        repeat: 10,
    };

    let test_cube2 = draw_utils::Cube {
        position: vec3(0., 10., 0.),
        size: vec3(5., 5., 5.),
        color: BLUE,
        repeat: 10,
    };

    let test_cube3 = draw_utils::Cube {
        position: vec3(-10., 0., 0.),
        size: vec3(5., 5., 5.),
        color: RED,
        repeat: 10,
    };

    let test_cube4 = draw_utils::Cube {
        position: vec3(0., -10., 0.),
        size: vec3(5., 5., 5.),
        color: GREEN,
        repeat: 10,
    };

    let grid = draw_utils::Grid::new();


    let mut px: f32 = 0.;
    let mut py: f32 = 0.;
    let mut pz: f32 = 10.;


    let mut rot_mat = Mat3::IDENTITY;

    let mut pos = vec3(px, py, pz);



    loop {
        clear_background(DARKGRAY);
        let dir = rot_mat * vec3(1., 0., 0.);
        let up = rot_mat * vec3(0., 0., 1.);

        pos += dir * 0.5;


        if is_key_down(KeyCode::E) {
            rot_mat = Mat3::from_axis_angle(dir, 0.02) * rot_mat;

        }
        if is_key_down(KeyCode::Q) {
            rot_mat = Mat3::from_axis_angle(dir, -0.02) * rot_mat;

        }
        if is_key_down(KeyCode::A) {
            rot_mat = Mat3::from_axis_angle(up, 0.02) * rot_mat;

        }
        if is_key_down(KeyCode::D) {
            rot_mat = Mat3::from_axis_angle(up, -0.02) * rot_mat;

        }
        if is_key_down(KeyCode::S) {
            rot_mat = Mat3::from_axis_angle(up.cross(dir), 0.02) * rot_mat;

        }
        if is_key_down(KeyCode::W) {
            rot_mat = Mat3::from_axis_angle(up.cross(dir), -0.02) * rot_mat;

        }

        set_camera(&Camera3D {
            position: pos,
            up: rot_mat * vec3(0., 0., 1.),
            target: pos + rot_mat * vec3(1., 0., 0.),

            ..Default::default()
        });

        grid.draw();

        test_cube.draw();
        test_cube2.draw();
        test_cube3.draw();
        test_cube4.draw();

        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await;
    }
}
