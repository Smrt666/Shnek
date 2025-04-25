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

    let mut view = movement::View::new();
    loop {
        let dt = get_frame_time();

        view.rotate(dt);

        player.set_direction(view.forward());
        player.move_forward(dt);

        // Set the camera to follow the player
        view.set_camera(player.get_position());

        clear_background(DARKGRAY);
        // draw

        grid.draw();

        player.draw();
        test_cube.draw();

        // Back to screen space, render some text
        set_default_camera();
        draw_text(&format!("fps: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);

        next_frame().await;
    }
}
