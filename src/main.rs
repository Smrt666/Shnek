use crate::draw_utils::SPACE_SIZE;
use crate::models3d::Model3D;
use macroquad::{prelude::*, ui::root_ui};

mod draw_utils;
mod food;
mod models3d;
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
    let head_model = Model3D::from_file("assets/head/snake_head.obj");
    let body_model = Model3D::from_file("assets/body/snake_body.obj");

    let snake_start_len = 3;
    let mut player = snake::Shnek::new(&head_model, &body_model);
    player.set_position(0., 0., 0.);
    player.set_direction(vec3(1., 0., 0.), vec3(0., 0., 1.));
    for _ in 0..snake_start_len {
        player.add_segment();
    }
    let food_model = Model3D::from_file("assets/apfel/apfel.obj");
    let mut food_factory = food::FoodFactory::new(&food_model);

    let mut view = movement::View::new();

    let mut game_state = GameState::Running;

    let mut food_distance = SPACE_SIZE * 3.0;
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

            player.set_direction(view.forward(), view.up());
            player.move_forward(dt);

            if player.check_tail_collision() {
                game_state = GameState::GameOver;
            }

            food_distance = food_factory.check_food_collision(&mut player);
        }

        // Set the camera to follow the player
        view.set_camera(player.get_position());

        clear_background(DARKGRAY);
        // draw

        food_factory.draw();
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
        draw_text(
            &format!("food distance: {}", food_distance.round()),
            10.0,
            80.0,
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
