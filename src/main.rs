use draw_utils::Drawable;
use macroquad::{prelude::*, ui::root_ui};

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

    let mut paused = false;

    loop {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        let dt = get_frame_time();

        if !paused {
            // Only update if not paused
            view.rotate(dt);

            player.set_direction(view.forward());
            player.move_forward(dt);
        }

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

        // Pause menu
        if paused {
            let screen_size = vec2(screen_width(), screen_height());
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                color_u8!(0, 0, 0, 128),
            );
            if root_ui().button(0.5 * screen_size, "Resume") {
                paused = false;
            }
            if root_ui().button(0.5 * screen_size + vec2(0., 50.), "Quit") {
                break;
            }
            // let mut speed_input = player.get_speed().to_string();
            // root_ui().input_text(hash!(), "Set speed", &mut speed_input);
            // match speed_input.parse::<f32>() {
            //     Ok(speed) => player.set_speed(speed),
            //     Err(_) => {}
            // }
        }

        next_frame().await;
    }
}
