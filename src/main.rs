use draw_utils::Drawable;
use macroquad::prelude::*;

mod draw_utils;
mod snake;

#[macroquad::main("3D")]
async fn main() {
    let mut phi: f32 = 0.;
    let r = 20.;

    let test_cube = draw_utils::Cube {
        position: vec3(0., 0., 0.),
        size: vec3(10., 10., 10.),
        color: RED,
        repeat: 10,
    };

    let mut player = snake::Shnek::new();
    player.set_position(0., 0., 0.);
    player.set_direction(0., 0., 1.);
    player.add_segment();
    player.add_segment();
    player.add_segment();
    player.add_segment();
    player.add_segment();

    let grid = draw_utils::Grid::new();

    loop {
        clear_background(DARKGRAY);

        // Going 3d!
        phi += 0.001;

        if is_key_down(KeyCode::Right) {
            player.set_direction(1., 0., 0.);
        }
        if is_key_down(KeyCode::Left) {
            player.set_direction(-1., 0., 0.);
        }
        if is_key_down(KeyCode::Up) {
            player.set_direction(0., 0., -1.);
        }
        if is_key_down(KeyCode::Down) {
            player.set_direction(0., 0., 1.);
        }
        if is_key_down(KeyCode::W) {
            player.set_direction(0., 1., 0.);
        }
        if is_key_down(KeyCode::S) {
            player.set_direction(0., -1., 0.);
        }
        if is_key_down(KeyCode::A) {
            player.set_direction(-1., 0., 0.);
        }
        if is_key_down(KeyCode::D) {
            player.set_direction(1., 0., 0.);
        }
        if is_key_down(KeyCode::Q) {
            player.set_direction(0., 0., -1.);
        }

        player.move_forward(0.1);
        set_camera(&Camera3D {
            position: player.get_position()
                + vec3(2.5, 6.0, 2.5)
                + vec3(r * phi.cos(), 0.0, r * phi.sin()),
            up: vec3(0., 1., 0.),
            target: player.get_position() + vec3(2.5, 6.0, 2.5),
            ..Default::default()
        });

        grid.draw();

        player.draw();
        test_cube.draw();

        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
