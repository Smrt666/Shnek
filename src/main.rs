use crate::draw_utils::SPACE_SIZE;
use crate::menu::{draw_status, main_menu, paused, running, score_menu, FPSCounter};
use crate::models3d::Model3D;
use macroquad::{
    prelude::*,
    ui::{root_ui, Skin},
};

use crate::button::{
    load_button_style, load_font, load_label_style, load_window_background, load_window_style,
    loading_sound,
};
use crate::food::FoodFactory;

mod button;
mod draw_utils;
mod food;
mod menu;
mod models3d;
mod movement;
mod score;
mod snake;

#[derive(Debug, PartialEq, Copy, Clone)]
enum GameState {
    MainMenu,
    Running,
    Paused,
    GameOver,
    Score,
}

#[macroquad::main("Shnek")]
async fn main() {
    let head_model = Model3D::from_file("assets/head/snake_head.obj");
    let body_model = Model3D::from_file("assets/body/snake_body.obj");

    let mut player = snake::Shnek::new(&head_model, &body_model, 3);
    player.set_position(0., 0., 0.);
    player.set_direction(vec3(1., 0., 0.), vec3(0., 0., 1.));

    let food_model = Model3D::from_file("assets/apfel/apfel.obj");
    let bad_food_model = Model3D::from_file("assets/bad_apfel/bad_apfel.obj");
    let poop_model = Model3D::from_file("assets/poop/poop.obj");
    let mut food_factory = FoodFactory::new(&food_model, &bad_food_model, &poop_model);

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
    let mut fps_counter = FPSCounter::new();
    loop {
        main_menu(&mut game_state, &click, &mut score_file);

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
        fps_counter.add_frame_dt(dt);
        let score = player.get_score();

        running(
            &mut game_state,
            &eat_sound,
            &collision_sound,
            &mut player,
            &mut view,
            &mut food_factory,
            dt,
            &mut food_distance,
        );

        // Set the camera to follow the player
        view.set_camera(player.get_camera_position());

        clear_background(Color::new(0.68, 0.85, 0.90, 1.0));
        // draw

        food_factory.draw();
        player.draw();

        // Back to screen space, render some text
        set_default_camera();
        high_score = high_score.max(score);
        draw_status(
            score,
            high_score,
            food_distance,
            food_factory.food_count(),
            food_factory.max_food as usize,
            &fps_counter,
        ); // TODO: max_food should be usize

        // Pause menu

        paused(
            &mut game_state,
            &click,
            &mut high_score,
            &mut player,
            &mut view,
            &mut food_factory,
            &food_model,
            &bad_food_model,
            &poop_model,
            &mut score_file,
        );

        //Score screen

        score_menu(&mut game_state, &click, high_score, &mut score_file);

        next_frame().await;
    }
}
