use macroquad::prelude::*;
use macroquad::rand::*;
use crate::draw_utils;
use crate::draw_utils::Drawable;
use crate::snake::*;


pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    vec3(
        gen_range(min, max),
        gen_range(min, max),
        gen_range(min, max),
    )
}

#[derive(PartialEq, Clone, Copy)]
pub struct Food {
    pub position: Vec3,
    pub size: Vec3,
    pub quality: u32,
    pub color: Color,
    pub repeat: i32,
}

pub struct FoodFactory {
    spawn_region: f32,
    quality_range: (u32, u32),
    all_the_apples: Vec<Food>,
    // size_range: Vec<u32>,
    // color_range: Vec<Color>,
}

impl FoodFactory {
    pub fn new() -> Self {
        Self { 
            spawn_region: 50., 
            quality_range: (1,1), 
            all_the_apples: vec![Food::new_custom(vec3(10., 0., 0.), vec3(3., 3., 3.), 1, YELLOW)],
        }
    }

    fn get_spawn(&self) -> f32 {
        self.spawn_region
    }

    pub fn get_apples(&self) -> Vec<Food> {
        self.all_the_apples.clone()
    }
}

impl Food {
    fn new_custom(position: Vec3, size: Vec3, quality: u32, color: Color) -> Self {
        Self{
            position: position,
            size: size,
            quality: quality,
            color: color,
            repeat: 5,
        }
    } 

    fn new_random(max: f32) -> Self {
        Self { 
            position: random_vec3(0., max),
            size: random_vec3(3., 5.), 
            quality: 1, 
            color: YELLOW,
            repeat: 5,
        }
    }

    fn get_quality(&self) -> u32 {
        self.quality
    }

    fn get_position(&self) -> Vec3 {
        self.position
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


pub fn check_food_collision(snake: &mut Shnek, food_factory: &mut FoodFactory) {
    for food in food_factory.all_the_apples.clone() {
        let dist = snake.get_position().distance(food.get_position());
        if dist < 3. {
            for _ in 0..food.quality {
                snake.add_segment();
            }
            food_factory.all_the_apples.retain(|&x| x != food);
            food_factory.all_the_apples.push(Food::new_random(50.));
        }
    }
}


pub fn check_tail_collision(snake: &Shnek) {
    for segment in snake.get_segments() {
        let dist = snake.get_position().distance(segment.get_position());
        if dist < 4. {
            set_default_camera();
            draw_text("collision", 40., 40., 40., BLACK);
        }
    }
}

