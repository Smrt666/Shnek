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

    // let mut phi: f32 = 0.;
    // let mut psi: f32 = 0.;
    // let mut r: f32 = 20.;
    let px: f32 = 0.;
    let py: f32 = 0.;
    let pz: f32 = 10.;

    let mut tx: f32 = 20.;
    let mut ty: f32 = 0.;
    let tz: f32 = 0.;

    let mut dir: Direction = Direction::X;

    enum Direction {
        X,
        Y,
        // Z,
        MinusX,
        MinusY,
        // MinusZ,
    }

    loop {
        clear_background(DARKGRAY);

        // if is_key_pressed(KeyCode::Left) {
        //     psi -= 0.1;
        // } else if is_key_pressed(KeyCode::Right) {
        //     psi += 0.1;
        // } else if is_key_pressed(KeyCode::Up) {
        //     phi += 0.1;
        // } else if is_key_pressed(KeyCode::Down) {
        //     phi -= 0.1;
        // }

        // if is_key_down(KeyCode::W) {
        //     px += 1.;
        //     tx += 1.;
        // } else if is_key_down(KeyCode::S) {
        //     px -= 1.;
        //     tx -= 1.;
        // } else if is_key_down(KeyCode::A) {
        //     py += 1.;
        //     ty += 1.;
        // } else if is_key_down(KeyCode::D) {
        //     py -= 1.;
        //     ty -= 1.;
        // } else if is_key_down(KeyCode::Up) {
        //     pz += 1.;
        //     tz += 1.;
        // } else if is_key_down(KeyCode::Down) {
        //     pz -= 1.;
        //     tz -= 1.;
        // }

        if is_key_pressed(KeyCode::Left) {
            match dir {
                Direction::X => {
                    let a = tx - px;
                    tx -= a;
                    ty += a;
                    dir = Direction::Y;
                }
                Direction::Y => {
                    let a = ty - py;
                    tx -= a;
                    ty -= a;
                    dir = Direction::MinusX;
                }
                // Direction::Z => {}
                Direction::MinusX => {
                    let a = tx - px;
                    tx -= a;
                    ty += a;
                    dir = Direction::MinusY;
                }
                Direction::MinusY => {
                    let a = ty - py;
                    tx -= a;
                    ty -= a;
                    dir = Direction::X;
                }
                // Direction::MinusZ => {}
            };
        } else if is_key_pressed(KeyCode::Right) {
            match dir {
                Direction::X => {
                    let a = tx - px;
                    tx -= a;
                    ty -= a;
                    dir = Direction::MinusY;
                }
                Direction::Y => {
                    let a = ty - py;
                    tx += a;
                    ty -= a;
                    dir = Direction::X;
                }
                // Direction::Z => {}
                Direction::MinusX => {
                    let a = tx - px;
                    tx -= a;
                    ty -= a;
                    dir = Direction::Y;
                }
                Direction::MinusY => {
                    let a = ty - py;
                    tx += a;
                    ty -= a;
                    dir = Direction::MinusX;
                }
                // Direction::MinusZ => {}
            };
        }

        set_camera(&Camera3D {
            position: vec3(px, py, pz),
            // up: vec3(0., 1., 0.),
            target: vec3(tx, ty, tz),

            // position: vec3(r * phi.cos() * psi.cos(), r * phi.cos() * psi.sin(), r * phi.sin()),
            // // up: vec3(0., 1., 0.),
            // target: vec3(0., 0., 0.),
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
