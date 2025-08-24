use draw_utils::Drawable;
use macroquad::{audio::set_sound_volume, prelude::*, ui::{hash, root_ui, widgets::Button, Skin}};
// use crate::miniquad::window::screen_size;
// use std::time::Duration;
// use std::thread::sleep;
use macroquad::audio::{load_sound, play_sound, play_sound_once, PlaySoundParams};

use crate::button::{loading_sound, load_button_style, load_font, load_label_style, load_ui_skin, load_window_background, load_window_style};

// use std::env;
// use std::fs;

mod draw_utils;
mod food;
mod movement;
mod snake;
mod score;
mod button;

#[derive(Debug, PartialEq)]
enum GameState {
    MainMenu,
    Running,
    Paused,
    GameOver,
    Score
}

// fn window_conf() -> Conf {
//     Conf {
//         // window_title: "My Microquad Window".to_owned(),
//         // window_width: 800,
//         // window_height: 600,
//         fullscreen: true,
//         ..Default::default()
//     }
// }


#[macroquad::main("Schnek")]
async fn main() {
    // set_fullscreen(true);
    // let test_cube = draw_utils::Cube {
    //     position: vec3(-10., 0., 0.),
    //     size: vec3(5., 5., 5.),
    //     color: RED,
    //     repeat: 10,
    // };

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

    // let menu_buttons_settings = button::MenuButtons::new(
    //     // load_window_background("assets/Solid_black.png").await.unwrap(),
    //     // load_font("assets/yoster.ttf").await,
    //     load_window_style(load_window_background("assets/Solid_black.png").await).await,
    //     load_button_style(load_font("assets/yoster.ttf").await).await,
    //     load_label_style(load_font("assets/yoster.ttf").await).await,
    //     // load_ui_skin("", "", "").await,
    //     load_button_sound("assets/spongebob-fog-horn.wav").await

    // );

    let window_style = load_window_style(load_window_background("assets/Solid_black.png").await).await;
    let button_style = load_button_style(load_font("assets/yoster.ttf").await).await;
    let label_style = load_label_style(load_font("assets/yoster.ttf").await).await;
    // load_ui_skin("", "", "").await,
    let collision_sound= loading_sound("assets/spongebob-fog-horn.wav").await;
    let eat_sound = loading_sound("assets/eating-sound-effect.wav").await;
    let click = loading_sound("assets/computer-mouse-click.wav").await;

    
    // root_ui().push_skin(&load_ui_skin(window_style, button_style, label_style).await);

    let ui_skin = Skin {
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };
    root_ui().push_skin(&ui_skin);

    // play_sound(main_theme)...
    // set_sound_volume(&button_sound, 0.);



    loop {



        if game_state == GameState::MainMenu {
            
            let window_size = vec2(370.0, 320.0);
            let window_pos = vec2(
                screen_width() / 2.0 - window_size.x / 2.0,
                screen_height() / 2.0 - window_size.y / 2.0,
            );
            let main_menu_id = hash!();
            root_ui().window(
                    main_menu_id,
                    window_pos,
                    window_size,
                    |ui| {
                        ui.label(vec2(80.0, -34.0), "Main Menu");
                        if ui.button(vec2(65.0, 25.0), "Play") {
                            play_sound(&click, PlaySoundParams { looped: false, volume: 0.1 });
                            game_state = GameState::Running;
                        }
                        if ui.button(vec2(65.0, 125.0), "Quit") {
                            std::process::exit(0);
                        }
                    },
                );
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

            player.set_direction(view.forward());

            player.check_boost_and_move(dt);

            if player.check_boost_time(&mut food_factory, snake_start_len) {
                game_state = GameState::GameOver;
            }

            if player.check_tail_collision() {
                play_sound(&collision_sound, PlaySoundParams { looped: false, volume: 0.01 });
                game_state = GameState::GameOver;
            }

            if food_factory.check_food_collision(&mut player, score) {
                play_sound(&eat_sound, PlaySoundParams { looped: false, volume: 0.1 })
            }
        }

        // Set the camera to follow the player
        view.set_camera(player.get_position());

        clear_background(DARKGRAY);
        // draw

        grid.draw();
        food_factory.draw_food();
        player.draw();
        // test_cube.draw();

        // Back to screen space, render some text
        set_default_camera();
        draw_text(&format!("fps: {}", get_fps()), 10.0, 20.0, 30.0, BLACK);

        
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
            root_ui().window(
                    menu_id,
                    window_pos,
                    window_size,
                    |ui| {
                        ui.label(vec2(10.0, 0.0), "Paused");
                        if game_state == GameState::Paused {
                            if ui.button(vec2(30.0, 50.0), "Resume") {
                            play_sound(&click, PlaySoundParams { looped: false, volume: 0.1 });
                            game_state = GameState::Running;
                            }
                        }
                        if game_state == GameState::GameOver {
                            if ui.button(vec2(45.0, 50.0), "Score") {
                            play_sound(&click, PlaySoundParams { looped: false, volume: 0.1 });
                            game_state = GameState::Score;
                            }
                        }

                        
                        if ui.button(vec2(50.0, 150.0), "Reset") {
                            play_sound(&click, PlaySoundParams { looped: false, volume: 0.1 });
                            high_score = 0;
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
            root_ui().move_window(menu_id, window_pos);

        }

        //Score screen

        if game_state == GameState::Score {

            let window_size = vec2(250., 100.);
            let window_pos = vec2(
                    screen_width() - window_size.x,
                    screen_height() - window_size.y
                );
            let menu_id = hash!();
            
            root_ui().window(
                menu_id,
                window_pos,
                window_size,
                |ui| {
                    // ui.label(vec2(10.0, 0.0), "Scores");

                if ui.button(vec2(-15., -30.), "Back") {
                    play_sound(&click, PlaySoundParams { looped: false, volume: 0.1 });
                    game_state = GameState::GameOver
                }
            }
            );
            root_ui().move_window(menu_id, window_pos);

            let previous = score_file.read();

            score_file.write(previous, high_score);


            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                BLACK);

            let contents = &score_file.read();

                draw_multiline_text(
                &format!("scores:\n{}", *contents),
                10.0,
                50.0,
                100.0,
                None,
                WHITE,);


                // Collect into a Vec<i32> (ignoring empty lines)
                let mut numbers: Vec<i32> = contents
                    .lines()
                    .filter_map(|line| line.trim().parse::<i32>().ok())
                    .collect();

                // Sort descending
                numbers.sort_by(|a, b| b.cmp(a));

                // Take top 3
                let top3: Vec<i32> = numbers.into_iter().take(3).collect();

                let best  = top3
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
                    GOLD);

            
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

