use draw_utils::Drawable;
use macroquad::prelude::*;

mod draw_utils;
mod movement;
mod snake;

#[macroquad::main("Shnek")]
async fn main() {
    let test_cube = draw_utils::Cube {
        position: vec3(-10., 0., 0.),
        size: vec3(5., 5., 5.),
        color: RED,
        repeat: 10,
    };
    let mut player = snake::Shnek::new();
    player.set_position(0., 0., 0.);
    player.set_direction(vec3(1., 0., 0.));
    for _ in 0..15 {
        player.add_segment();
    }

    let grid = draw_utils::Grid::new();

    let mut rot_mat = Mat3::IDENTITY;
    loop {
        clear_background(DARKGRAY);
        let dir = rot_mat * vec3(1., 0., 0.);
        let up = rot_mat * vec3(0., 0., 1.);

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

        player.set_direction(dir);
        player.move_forward(0.5);

        let cam_offset = up * 5.0 - dir * 5.0;
        set_camera(&Camera3D {
            position: player.get_position() + cam_offset,
            up: up,
            target: player.get_position() + rot_mat * vec3(1., 0., 0.) + cam_offset,

            ..Default::default()
        });

        grid.draw();

        player.draw();
        test_cube.draw();
        // Back to screen space, render some text

        // set_default_camera();
        // draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await;
    }
}
