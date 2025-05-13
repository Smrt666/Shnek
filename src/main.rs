use std::{fs, env};

use draw_utils::Drawable;
use macroquad::{prelude::*, ui::root_ui};
use tobj::load_obj;

mod draw_utils;
mod food;
mod movement;
mod snake;

#[derive(Debug, PartialEq)]
enum GameState {
    Running,
    Paused,
    GameOver,
}

#[macroquad::main("Shnek")]
async fn main() {
    let (models, materials) = load_obj(
        "assets/test_obj/eyeball.obj",
        &tobj::GPU_LOAD_OPTIONS,
    )
    .expect("Failed to load OBJ file");
    
    let snake_start_len = 3;
    let mut player = snake::Shnek::new();
    player.set_position(0., 0., 0.);
    player.set_direction(vec3(1., 0., 0.));
    for _ in 0..snake_start_len {
        player.add_segment();
    }

    let mut food_factory = food::FoodFactory::new();

    let grid = draw_utils::Grid::new();

    let mut view = movement::View::new();

    let mut game_state = GameState::Running;

    loop {
        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Space) {
            game_state = match game_state {
                GameState::Running => GameState::Paused,
                GameState::Paused => GameState::Running,
                GameState::GameOver => GameState::GameOver,
            };
        }

        let dt = get_frame_time();

        if game_state == GameState::Running {
            // Only update if not paused
            view.rotate(dt);

            player.set_direction(view.forward());
            player.move_forward(dt);

            if player.check_tail_collision() {
                game_state = GameState::GameOver;
            }

            food_factory.check_food_collision(&mut player);
        }

        // Set the camera to follow the player
        view.set_camera(player.get_position());

        clear_background(DARKGRAY);
        // draw

        grid.draw();
        food_factory.draw_food();
        player.draw();

        // Back to screen space, render some text
        set_default_camera();
        draw_text(&format!("fps: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);
        draw_text(
            &format!("score: {}", player.get_length() - snake_start_len),
            10.0,
            50.0,
            30.0,
            BLACK,
        );

        // Pause menu
        if game_state == GameState::Paused || game_state == GameState::GameOver {
            let screen_size = vec2(screen_width(), screen_height());
            draw_rectangle(
                // draw a semi-transparent rectangle over the screen
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                color_u8!(0, 0, 0, 128),
            );
            if game_state == GameState::Paused && root_ui().button(0.5 * screen_size, "Resume") {
                game_state = GameState::Running;
            }
            if root_ui().button(0.5 * screen_size + vec2(0., 25.), "Reset") {
                player.set_position(0., 0., 0.);
                player.set_direction(vec3(1., 0., 0.));
                player.reset();
                view.reset();
                for _ in 0..snake_start_len {
                    player.add_segment();
                }
                game_state = GameState::Running;
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
