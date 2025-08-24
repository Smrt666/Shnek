use crate::draw_utils::Drawable;
use crate::draw_utils::SPACE_SIZE;
use crate::snake::*;
use macroquad::prelude::*;
use macroquad::rand::*;
// use macroquad::audio::{load_sound, play_sound, play_sound_once};
// use rand::seq::SliceRandom;
// use rand::thread_rng;

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    vec3(
        gen_range(min, max),
        gen_range(min, max),
        gen_range(min, max),
    )
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Food {
    pub position: Vec3,
    pub size: Vec3,
    pub quality: i32,
    pub color: Color,
    pub repeat: i32,
}

pub struct FoodFactory {
    // spawn_region: f32,
    // quality_range: (u32, u32),
    pub all_the_apples: Vec<Food>,
    max_food: u32,
    // size_range: Vec<u32>,
    color_range: Vec<Color>,
}

impl FoodFactory {
    pub fn new() -> Self {
        Self {
            // spawn_region: SPACE_SIZE,
            // quality_range: (1, 1),
            all_the_apples: vec![Food::new_custom(
                vec3(10., 0., 0.),
                vec3(3., 3., 3.),
                1,
                YELLOW,
            )],
            max_food: 1,
            color_range: vec![YELLOW, PURPLE],
        }
    }

    // fn get_spawn(&self) -> f32 {
    //     self.spawn_region
    // }

    // pub fn raise_max_food(&mut self) {
    //     self.max_food += 1;
    // }

    pub fn check_food_collision(&mut self, snake: &mut Shnek, score: usize) -> bool {
        // println!("{:?}", self.all_the_apples);
        // println!("{}", self.max_food);

        for &food in self.all_the_apples.clone().iter() {
            let dist = mod_distance(snake.get_position(), food.get_position());
            if dist < 3. {
                if food.quality > 0 {
                    for _ in 0..food.quality {
                        snake.add_segment();
                    }
                } else {
                    for _ in 0..(-food.quality as u32) {
                        snake.pop_segment();
                        draw_rectangle(
                            // draw a semi-transparent rectangle over the screen
                            0.0,
                            0.0,
                            screen_width(),
                            screen_height(),
                            RED
                        );
                    }
                }
                

                self.all_the_apples.retain(|&x| x != food);
                if food.color != BROWN {
                    let non_red = self.all_the_apples.iter().filter(|apple| apple.color != RED).count();
                    for _ in 0..gen_range(1, self.max_food as usize + 1 - non_red)
                        {
                        if let Some(choice) = self.color_range.choose() {
                            self.all_the_apples.push(
                                Food::new_random(
                                SPACE_SIZE,
                                (((score + 1) as f64).log10()).round() as i32 + 1,
                                *choice
                                ),
                            );
                        }
                    }

                    if score > 10 && gen_range(0, 100) < 5 {
                        self.all_the_apples.push(
                        Food::new_custom(
                            random_vec3(0., SPACE_SIZE),
                            random_vec3(3., 5.),
                            (-(score as f64)/10.).round() as i32,
                            RED
                        )
                       );
                    }
                    self.max_food = ((score as f64 *2.).log10()).round() as u32 + 1;
                }
            return true
            }
        }

        return false
    }
    

    pub fn draw_food(&self) {
        for food in self.all_the_apples.iter() {
            food.draw();
        }
    }

    pub fn reset(&mut self) {
        self.all_the_apples = vec![Food::new_custom(
            vec3(10., 0., 0.),
            vec3(3., 3., 3.),
            1,
            YELLOW,
        )]
    }
}

impl Food {
    pub fn new_custom(position: Vec3, size: Vec3, quality: i32, color: Color) -> Self {
        Self {
            position,
            size,
            quality,
            color,
            repeat: 5,
        }
    }

    fn new_random(max_pos: f32, max_quality: i32, color: Color) -> Self {
        Self {
            position: random_vec3(0., max_pos),
            size: random_vec3(3., 5.),
            quality: gen_range(1, max_quality),
            color: color,
            repeat: 5,
        }
    }

    // fn get_quality(&self) -> u32 {
    //     self.quality
    // }

    fn get_position(&self) -> Vec3 {
        modulus_vec3(self.position, SPACE_SIZE)
    }
}

impl Drawable for Food {
    fn get_repeat(&self) -> i32 {
        self.repeat
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn draw_at(&self, position: Vec3, _saturation: f32) {
        draw_cube(position, self.size, None, self.color);
    }
}
