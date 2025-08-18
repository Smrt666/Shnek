use draw_utils::Drawable;
use macroquad::{audio::set_sound_volume, prelude::*, ui::{hash, root_ui, Skin}};
// use crate::miniquad::window::screen_size;
// use std::time::Duration;
// use std::thread::sleep;
use macroquad::audio::{load_sound, play_sound, play_sound_once, PlaySoundParams};

use std::env;
use std::fs;

mod draw_utils;
mod food;
mod movement;
mod snake;
mod score;

#[derive(Debug, PartialEq)]
enum GameState {
    MainMenu,
    Running,
    Paused,
    GameOver,
    Score
}

fn window_conf() -> Conf {
    Conf {
        // window_title: "My Microquad Window".to_owned(),
        // window_width: 800,
        // window_height: 600,
        fullscreen: true,
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    // set_fullscreen(true);
    let test_cube = draw_utils::Cube {
        position: vec3(-10., 0., 0.),
        size: vec3(5., 5., 5.),
        color: RED,
        repeat: 10,
    };

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

    let mut game_state = GameState::MainMenu;

    let mut high_score = 0;

    let mut score_file = score::Score::new();



// Buttons

    let window_background = load_image("assets/Solid_black.png").await.unwrap();
    // let button_background = load_image("assets/green_button.png").await.unwrap();
    // let button_clicked_background = load_image("assets/pressed_button.png").await.unwrap();
    let font = load_file("assets/yoster.ttf").await.unwrap();


    let window_style = root_ui()
        .style_builder()
        .background(window_background)
        .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
        .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
        .build();


    let button_style = root_ui()
        .style_builder()
        // .background(button_background)
        // .background_clicked(button_clicked_background)
        .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
        .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
        .font(&font)
        .unwrap()
        .text_color(BLACK)
        .font_size(64)
        .build();

    let label_style = root_ui()
        .style_builder()
        .font(&font)
        .unwrap()
        .text_color(WHITE)
        .font_size(28)
        .build();

    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    let button_sound = load_sound("assets/spongebob-fog-horn.wav").await.unwrap();
    

    // play_sound(main_theme)...
    // set_sound_volume(&button_sound, 0.);



    loop {

        if game_state == GameState::MainMenu {
            let window_size = vec2(370.0, 320.0);
            root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - window_size.x / 2.0,
                        screen_height() / 2.0 - window_size.y / 2.0,
                    ),
                    window_size,
                    |ui| {
                        ui.label(vec2(80.0, -34.0), "Main Menu");
                        if ui.button(vec2(65.0, 25.0), "Play") {
                            game_state = GameState::Running;
                        }
                        if ui.button(vec2(65.0, 125.0), "Quit") {
                            std::process::exit(0);
                        }
                    },
                );
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

        if game_state == GameState::Running {
            // Only update if not paused
            view.rotate(dt);

            player.set_direction(view.forward());

            player.check_boost_and_move(dt);

            if player.check_boost_time(&mut food_factory, snake_start_len) {
                game_state = GameState::GameOver;
            }

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
        test_cube.draw();

        // Back to screen space, render some text
        set_default_camera();
        draw_text(&format!("fps: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);

        let score = player.get_length() - snake_start_len;
        draw_text(
            &format!("score: {}", score),
            10.0,
            50.0,
            30.0,
            BLACK,
        );
        high_score = high_score.max(score);
        draw_text(
            &format!("high score: {}", high_score),
            10.0,
            70.0,
            30.0,
            BLACK,
        );

        // Pause menu

        if game_state == GameState::Paused || game_state == GameState::GameOver {
            let window_size = vec2(400.0, 500.0);
            draw_rectangle(
                // draw a semi-transparent rectangle over the screen
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                color_u8!(0, 0, 0, 128),
            );

            root_ui().window(
                    hash!(),
                    vec2(
                        screen_width() / 2.0 - window_size.x / 2.0,
                        screen_height() / 2.0 - window_size.y / 2.0,
                        // 0.0, 0.0
                    ),
                    window_size,
                    |ui| {
                        ui.label(vec2(10.0, 0.0), "Paused");
                        if game_state == GameState::Paused {
                            if ui.button(vec2(30.0, 50.0), "Resume") {
                            game_state = GameState::Running;
                            }
                        }
                        if game_state == GameState::GameOver {
                            if ui.button(vec2(45.0, 50.0), "Score") {
                            // play_sound_once(&button_sound);
                            game_state = GameState::Score;
                            play_sound(&button_sound, PlaySoundParams { looped: false, volume: 0.01 });


                            }
                        }

                        
                        if ui.button(vec2(50.0, 150.0), "Reset") {
                            player.set_position(0., 0., 0.);
                            player.set_direction(vec3(1., 0., 0.));
                            player.reset();
                            view.reset();
                            food_factory.reset();
                            score_file.reset();
                            for _ in 0..snake_start_len {
                                player.add_segment();
                            }
                            game_state = GameState::Running;
                        }
                        if ui.button(vec2(65.0, 250.0), "Quit") {
                            std::process::exit(0);
                        }
                    },
                );
        }

        //Score screen

        if game_state == GameState::Score {
            let window_size = vec2(250., 100.);
            root_ui().window(
                hash!(),
                vec2(
                    screen_width() - window_size.x,
                    screen_height() - window_size.y
                ),
                window_size,
                |ui| {
                    // ui.label(vec2(10.0, 0.0), "Scores");

                if ui.button(vec2(-15., -30.), "Back") {
                    game_state = GameState::GameOver
                }
            }
            );

            score_file.write(score);


            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                BLACK);

            let contents = score_file.read();

                draw_multiline_text(
                &format!("score:\n{}", contents),
                10.0,
                50.0,
                100.0,
                None,
                WHITE,);

            
        }


        // request_new_screen_size();
        next_frame().await;

}
        



            // // let mut speed_input = player.get_speed().to_string();
            // root_ui().input_text(hash!(), "Set speed", &mut speed_input);
            // match speed_input.parse::<f32>() {
            //     Ok(speed) => player.set_speed(speed),
            //     Err(_) => {}
            // }

        // }

        // let a = get_frame_time();
        // // while get_frame_time() < 1./60. {
        // //     sleep(Duration::new( 0, 1000))
        // // }
        // let nans = ((1./60. - a).max(0.) * 1000000.).round() as u32;
        // if a < 1./60. {
        //     sleep(Duration::new( 0, nans));
        // }

        // unsafe {
        //     let ctx = macroquad::window::get_internal_gl();
        //     ctx.quad_context.commit_frame();
        // }

    }

