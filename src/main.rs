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

    let mut head = snake::ShnekHead::new(0., 0., 0.);
    head.set_direction(1., 0., 0.);

    let grid = draw_utils::Grid::new();

    loop {
        clear_background(DARKGRAY);

        // Going 3d!
        phi += 0.001;
        head.move_forward(0.1);
        set_camera(&Camera3D {
            position: head.get_position()
                + vec3(2.5, 6.0, 2.5)
                + vec3(r * phi.cos(), 0.0, r * phi.sin()),
            up: vec3(0., 1., 0.),
            target: head.get_position() + vec3(2.5, 6.0, 2.5),
            ..Default::default()
        });

        grid.draw();

        head.draw();
        test_cube.draw();

        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}
