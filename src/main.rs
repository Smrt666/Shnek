use draw_utils::Drawable;
use macroquad::prelude::*;

mod draw_utils;

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

    let grid = draw_utils::Grid::new();

    loop {
        clear_background(DARKGRAY);

        // Going 3d!
        phi += 0.001;
        set_camera(&Camera3D {
            position: vec3(r * phi.sin(), r, r * phi.cos()),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        grid.draw();

        test_cube.draw();

        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
