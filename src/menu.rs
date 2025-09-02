use crate::food::FoodFactory;
use crate::movement::View;
use crate::score::Score;
use crate::snake::Shnek;
use crate::GameState;

use crate::models3d::Model3D;
use macroquad::audio::Sound;
use macroquad::audio::{play_sound, PlaySoundParams};
use macroquad::prelude::*;
use macroquad::{hash, ui::root_ui};

pub fn main_menu(game_state: &mut GameState, click_sound: &Sound, score_file: &mut Score) {
    if *game_state == GameState::MainMenu {
        let window_size = vec2(400.0, 500.0);
        let window_pos = vec2(
            screen_width() / 2.0 - window_size.x / 2.0,
            screen_height() / 2.0 - window_size.y / 2.0,
        );
        let main_menu_id = hash!();
        root_ui().window(main_menu_id, window_pos, window_size, |ui| {
            ui.label(vec2(80.0, -0.0), "Main Menu");
            if ui.button(vec2(65.0, 50.0), "Play") {
                play_sound(
                    click_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.1,
                    },
                );
                *game_state = GameState::Running;
            }
            if ui.button(vec2(45.0, 150.0), "Score") {
                play_sound(
                    click_sound,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.1,
                    },
                );
                score_file.prev_game_state = *game_state;
                *game_state = GameState::Score;
            }
            if ui.button(vec2(65.0, 250.0), "Quit") {
                std::process::exit(0);
            }
        });
        root_ui().move_window(main_menu_id, window_pos);
    }
}

pub fn paused<'a>(
    game_state: &mut GameState,
    click: &Sound,
    mut high_score: i32,
    player: &mut Shnek,
    view: &mut View,
    food_factory: &mut FoodFactory<'a>,
    food_model: &'a Model3D,
    poop_model: &'a Model3D,
    score_file: &mut Score,
) {
    if *game_state == GameState::Paused || *game_state == GameState::GameOver {
        if *game_state == GameState::GameOver {
            score_file.write(high_score as usize);
        }

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
            if *game_state == GameState::Paused {
                ui.label(vec2(10.0, 0.0), "Paused")
            } else {
                ui.label(vec2(10.0, 0.0), "Game over")
            }

            if *game_state == GameState::Paused && ui.button(vec2(30.0, 50.0), "Resume") {
                play_sound(
                    click,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.1,
                    },
                );
                *game_state = GameState::Running;
            }
            if *game_state == GameState::GameOver && ui.button(vec2(45.0, 50.0), "Score") {
                play_sound(
                    click,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.1,
                    },
                );
                score_file.prev_game_state = *game_state;
                *game_state = GameState::Score;
            }

            if ui.button(vec2(50.0, 150.0), "Reset") {
                play_sound(
                    click,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.1,
                    },
                );
                high_score = 0;
                player.reset();
                view.reset();
                *food_factory = FoodFactory::new(food_model, poop_model);
                score_file.reset();
                for _ in 0..player.start_length {
                    player.add_segment();
                }
                *game_state = GameState::Running;
            }
            if ui.button(vec2(65.0, 250.0), "Quit") {
                std::process::exit(0);
            }
        });
        root_ui().move_window(menu_id, window_pos);
    }
}

pub fn score_menu(
    game_state: &mut GameState,
    click: &Sound,
    high_score: i32,
    score_file: &mut Score,
) {
    if *game_state == GameState::Score {
        let window_size = vec2(250., 100.);
        let window_pos = vec2(
            screen_width() - window_size.x,
            screen_height() - window_size.y,
        );
        let menu_id = hash!();

        root_ui().window(menu_id, window_pos, window_size, |ui| {
            if ui.button(vec2(-15., -30.), "Back") {
                play_sound(
                    &click,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.1,
                    },
                );
                *game_state = score_file.prev_game_state
            }
        });
        root_ui().move_window(menu_id, window_pos);

        score_file.write(high_score as usize);

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
}

pub struct FPSCounter {
    samples: [f32; FPSCounter::SAMPLE_SIZE],
    i: usize,
}

impl FPSCounter {
    const SAMPLE_SIZE: usize = 20;
    pub fn new() -> Self {
        Self {
            samples: [0.0; FPSCounter::SAMPLE_SIZE],
            i: 0,
        }
    }

    pub fn add_frame_dt(&mut self, dt: f32) {
        self.samples[self.i] = dt;
        self.i = (self.i + 1) % FPSCounter::SAMPLE_SIZE;
    }

    pub fn fps(&self) -> f32 {
        (FPSCounter::SAMPLE_SIZE as f32) / self.samples.iter().sum::<f32>().max(0.001)
    }
}

pub fn draw_status(
    score: i32,
    high_score: i32,
    food_distance: f32,
    food_count: usize,
    max_food: usize,
    fps_counter: &FPSCounter,
) {
    draw_text(
        &format!("fps: {}", fps_counter.fps().round()),
        10.0,
        20.0,
        30.0,
        BLACK,
    );

    draw_text(&format!("score: {}", score), 10.0, 50.0, 30.0, BLACK);
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
    draw_text(
        &format!("food count: {}", food_count),
        10.0,
        130.0,
        30.0,
        BLACK,
    );
    draw_text(&format!("max food: {}", max_food), 10.0, 150.0, 30.0, BLACK);
}

pub fn running<'a>(
    game_state: &mut GameState,
    eat_sound: &Sound,
    collision_sound: &Sound,
    player: &mut Shnek,
    view: &mut View,
    food_factory: &mut FoodFactory<'a>,
    dt: f32,
    food_distance: &mut f32,
) {
    if *game_state == GameState::Running {
        // Only update if not paused
        view.rotate(dt);

        player.set_direction(view.forward(), view.up());
        player.move_forward(dt);

        player.check_boost_and_move(dt);

        if player.check_boost_time(food_factory, player.start_length) {
            *game_state = GameState::GameOver;
        }

        if player.check_tail_collision() {
            play_sound(
                &collision_sound,
                PlaySoundParams {
                    looped: false,
                    volume: 0.01,
                },
            );
            *game_state = GameState::GameOver;
        }
        let eaten: bool;
        (*food_distance, eaten) = food_factory.check_food_collision(player);
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
}
