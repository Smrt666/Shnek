use macroquad::{hash, prelude::*, ui::{root_ui, Skin}};
use macroquad::audio::{play_sound, PlaySoundParams};
use crate::draw_utils::SPACE_SIZE;
use crate::models3d::Model3D;

use crate::button::{
    load_button_style, load_font, load_label_style, load_window_background, load_window_style,
    loading_sound,
};
use crate::food::FoodFactory;

mod button;
mod draw_utils;
mod food;
mod models3d;
mod movement;
mod score;
mod snake;

#[derive(Debug, PartialEq)]
enum GameState {
    MainMenu,
    Running,
    Paused,
    GameOver,
    Score,
}

#[macroquad::main("Schnek")]
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
    let mut food_factory = FoodFactory::new(&food_model);

    let mut view = movement::View::new();

    let mut game_state = GameState::MainMenu;

    let mut high_score = 0;

    let mut score_file = score::Score::new();


    let window_style =
        load_window_style(load_window_background("assets/Solid_black.png").await).await;
    let button_style = load_button_style(load_font("assets/yoster.ttf").await).await;
    let label_style = load_label_style(load_font("assets/yoster.ttf").await).await;
    let collision_sound = loading_sound("assets/spongebob-fog-horn.wav").await;
    let eat_sound = loading_sound("assets/eating-sound-effect.wav").await;
    let click = loading_sound("assets/computer-mouse-click.wav").await;

    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    let mut food_distance = SPACE_SIZE * 3.0;
    loop {
        if game_state == GameState::MainMenu {
            let window_size = vec2(370.0, 320.0);
            let window_pos = vec2(
                screen_width() / 2.0 - window_size.x / 2.0,
                screen_height() / 2.0 - window_size.y / 2.0,
            );
            let main_menu_id = hash!();
            root_ui().window(main_menu_id, window_pos, window_size, |ui| {
                ui.label(vec2(80.0, -34.0), "Main Menu");
                if ui.button(vec2(65.0, 25.0), "Play") {
                    play_sound(
                        &click,
                        PlaySoundParams {
                            looped: false,
                            volume: 0.1,
                        },
                    );
                    game_state = GameState::Running;
                }
                if ui.button(vec2(65.0, 125.0), "Quit") {
                    std::process::exit(0);
                }
            });
            root_ui().move_window(main_menu_id, window_pos);
        }

        if is_key_pressed(KeyCode::Backspace) {
            game_state = GameState::GameOver
        }

        if is_key_pressed(KeyCode::Escape) || is_key_pressed(KeyCode::Space) {
            game_state = match game_state {
                GameState::MainMenu => GameState::MainMenu,
                GameState::Running => GameState::Paused,
                GameState::Paused => GameState::Running,
                GameState::GameOver => GameState::GameOver,
                GameState::Score => GameState::GameOver,
            };
        }

        let dt = get_frame_time();
        let score = player.get_length() - snake_start_len;

        if game_state == GameState::Running {
            // Only update if not paused
            view.rotate(dt);

            player.set_direction(view.forward(), view.up());
            player.move_forward(dt);

            player.check_boost_and_move(dt);

            if player.check_boost_time(&mut food_factory, snake_start_len) {
                game_state = GameState::GameOver;
            }

            if player.check_tail_collision() {
                play_sound(
                    &collision_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.01,
                    },
                );
                game_state = GameState::GameOver;
            }

            let eaten: bool;
            (food_distance, eaten) = food_factory.check_food_collision(&mut player);
            if eaten {
                play_sound(
                    &eat_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.1,
                    },
                )
            }
        }

        // Set the camera to follow the player
        view.set_camera(player.get_camera_position());

        clear_background(DARKGRAY);
        // draw

        food_factory.draw();
        player.draw();

        // Back to screen space, render some text
        set_default_camera();
        draw_text(&format!("fps: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);

        draw_text(&format!("score: {}", score), 10.0, 50.0, 30.0, BLACK);
        high_score = high_score.max(score);
        draw_text(
            &format!("high score: {}", high_score),
            10.0,
            70.0,
            30.0,
            BLACK,
        );
        draw_text(
            &format!("food distance: {}", food_distance.round()),
            10.0,
            100.0,
            30.0,
            BLACK,
        );

        // Pause menu

        if game_state == GameState::Paused || game_state == GameState::GameOver {
            draw_rectangle(
                // draw a semi-transparent rectangle over the screen
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                color_u8!(0, 0, 0, 128),
            );

            let window_size = vec2(400.0, 500.0);
            let window_pos = vec2(
                screen_width() / 2.0 - window_size.x / 2.0,
                screen_height() / 2.0 - window_size.y / 2.0,
            );
            let menu_id = hash!();
            root_ui().window(menu_id, window_pos, window_size, |ui| {
                ui.label(vec2(10.0, 0.0), "Paused");
                if game_state == GameState::Paused && ui.button(vec2(30.0, 50.0), "Resume") {
                    play_sound(
                        &click,
                        PlaySoundParams {
                            looped: false,
                            volume: 0.1,
                        },
                    );
                    game_state = GameState::Running;
                }
                if game_state == GameState::GameOver && ui.button(vec2(45.0, 50.0), "Score") {
                    play_sound(
                        &click,
                        PlaySoundParams {
                            looped: false,
                            volume: 0.1,
                        },
                    );
                    game_state = GameState::Score;
                }

                if ui.button(vec2(50.0, 150.0), "Reset") {
                    play_sound(
                        &click,
                        PlaySoundParams {
                            looped: false,
                            volume: 0.1,
                        },
                    );
                    high_score = 0;
                    player.reset();
                    view.reset();
                    food_factory = FoodFactory::new(&food_model);
                    score_file.reset();
                    for _ in 0..snake_start_len {
                        player.add_segment();
                    }
                    game_state = GameState::Running;
                }
                if ui.button(vec2(65.0, 250.0), "Quit") {
                    std::process::exit(0);
                }
            });
            root_ui().move_window(menu_id, window_pos);
        }

        //Score screen

        if game_state == GameState::Score {
            let window_size = vec2(250., 100.);
            let window_pos = vec2(
                screen_width() - window_size.x,
                screen_height() - window_size.y,
            );
            let menu_id = hash!();

            root_ui().window(menu_id, window_pos, window_size, |ui| {
                // ui.label(vec2(10.0, 0.0), "Scores");

                if ui.button(vec2(-15., -30.), "Back") {
                    play_sound(
                        &click,
                        PlaySoundParams {
                            looped: false,
                            volume: 0.1,
                        },
                    );
                    game_state = GameState::GameOver
                }
            });
            root_ui().move_window(menu_id, window_pos);

            score_file.write(high_score);

            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), BLACK);

            draw_rectangle(0.0, 0.0, screen_width(), screen_height(), BLACK);

            let contents = &score_file.read();

            draw_multiline_text(
                &format!("scores:\n{}", *contents),
                10.0,
                50.0,
                100.0,
                None,
                WHITE,
            );

            // Collect into a Vec<i32> (ignoring empty lines)
            let mut numbers: Vec<i32> = contents
                .lines()
                .filter_map(|line| line.trim().parse::<i32>().ok())
                .collect();

            // Sort descending
            numbers.sort_by(|a, b| b.cmp(a));

            // Take top 3
            let top3: Vec<i32> = numbers.into_iter().take(3).collect();

            let best = top3
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            draw_multiline_text(
                &format!("best: \n{}", best),
                screen_width() - 250.,
                50.,
                100.,
                None,
                GOLD,
            );
        }

        next_frame().await;
    }
}
