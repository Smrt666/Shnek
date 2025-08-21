use crate::draw_utils::Drawable;
use crate::draw_utils::SPACE_SIZE;
use crate::snake::*;
use macroquad::prelude::*;
use macroquad::rand::*;
// use macroquad::audio::{load_sound, play_sound, play_sound_once};

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
    pub quality: u32,
    pub color: Color,
    pub repeat: i32,
}

pub struct FoodFactory {
    // spawn_region: f32,
    // quality_range: (u32, u32),
    pub all_the_apples: Vec<Food>,
    max_food: u32,
    // size_range: Vec<u32>,
    // color_range: Vec<Color>,
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
        }
    }

    // fn get_spawn(&self) -> f32 {
    //     self.spawn_region
    // }

    // pub fn raise_max_food(&mut self) {
    //     self.max_food += 1;
    // }

    pub fn check_food_collision(&mut self, snake: &mut Shnek) -> bool {
        for &food in self.all_the_apples.clone().iter() {
            let dist = mod_distance(snake.get_position(), food.get_position());
            if dist < 3. {
                for _ in 0..food.quality {
                    snake.add_segment();
                }

                self.all_the_apples.retain(|&x| x != food);
                if food.color != BROWN {
                    for _ in 0..gen_range(1, self.max_food) {
                        self.all_the_apples.push(Food::new_random(SPACE_SIZE, 2));
                    }
                }
                return true

                // raise_max_food(food_factory);
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
    pub fn new_custom(position: Vec3, size: Vec3, quality: u32, color: Color) -> Self {
        Self {
            position,
            size,
            quality,
            color,
            repeat: 5,
        }
    }

    fn new_random(max_pos: f32, max_quality: u32) -> Self {
        Self {
            position: random_vec3(0., max_pos),
            size: random_vec3(3., 5.),
            quality: gen_range(1, max_quality),
            color: YELLOW,
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
