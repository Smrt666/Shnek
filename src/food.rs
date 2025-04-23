use macroquad::prelude::*;
use macroquad::rand::*;
use crate::draw_utils::Drawable;
use crate::snake::{modulus, modulus_vec3};


pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    vec3(
        gen_range(min, max),
        gen_range(min, max),
        gen_range(min, max),
    )
}


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
    fn get_spawn(&self) -> f32 {
        self.spawn_region
    }

    // fn get_apples(&self) -> Vec<Food> {
    //     self.all_the_apples.clone()
    // }
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



