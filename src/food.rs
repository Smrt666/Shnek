use std::vec;

use crate::draw_utils::Drawable;
use crate::snake::*;
use macroquad::prelude::*;
use macroquad::rand::*;

use crate::models3d::Model3D;

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    vec3(
        gen_range(min, max),
        gen_range(min, max),
        gen_range(min, max),
    )
}

pub struct Food {
    pub position: Vec3,
    pub up: Vec3,
    pub front: Vec3,
    pub size: f32,
    pub quality: u32,
}

pub struct FoodFactory {
    spawn_region: f32,
    quality_range: (u32, u32),
    all_the_apples: Vec<Food>,
    max_food: u32,
    // size_range: Vec<u32>,
    // color_range: Vec<Color>,
    model: Model3D,
}

impl FoodFactory {
    pub fn new() -> Self {
        let model = Model3D::from_file("assets/apfel/apfel.obj");
        let mut s = Self {
            spawn_region: 50.,
            quality_range: (1, 1),
            all_the_apples: Vec::new(),
            max_food: 1,
            model,
        };
        s.new_custom(vec3(10., 10., 10.), 1., 1);
        s
    }

    pub fn new_custom(&mut self, position: Vec3, size: f32, quality: u32) {
        self.all_the_apples.push(Food::new_custom(
            position,
            vec3(0., 0., 1.),
            vec3(0., 1., 0.),
            size * 0.1,
            quality,
        ))
    }

    fn get_spawn(&self) -> f32 {
        self.spawn_region
    }

    pub fn raise_max_food(&mut self) {
        self.max_food += 1;
    }

    pub fn check_food_collision(&mut self, snake: &mut Shnek) {
        let mut removed = vec![];
        let mut new_food = vec![];
        for i in 0..self.all_the_apples.len() {
            let dist = snake
                .get_position()
                .distance(self.all_the_apples[i].get_position());
            if dist < 3. {
                for _ in 0..self.all_the_apples[i].quality {
                    snake.add_segment();
                }
                removed.push(i);
                // raise_max_food(food_factory);

                for _ in 0..gen_range(1, self.max_food) {
                    new_food.push(Food::new_random(50., 2));
                }
            }
        }
        for i in removed {
            self.all_the_apples.remove(i);
        }
        for food in new_food {
            self.all_the_apples.push(food);
        }
    }
}

impl Food {
    fn new_custom(position: Vec3, up: Vec3, front: Vec3, size: f32, quality: u32) -> Self {
        Self {
            position,
            up,
            front,
            size,
            quality,
        }
    }

    fn new_random(max_pos: f32, max_quality: u32) -> Self {
        Self {
            position: random_vec3(0., max_pos),
            up: vec3(0., 0., 1.),
            front: vec3(0., 1., 0.),
            quality: gen_range(1, max_quality),
            size: 0.1,
        }
    }

    fn get_quality(&self) -> u32 {
        self.quality
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }
}

impl Drawable for FoodFactory {
    fn get_repeat(&self) -> i32 {
        2
    }

    fn get_position(&self) -> Vec3 {
        vec3(0., 0., 0.)
    }

    fn draw_at(&self, position: Vec3, _saturation: f32) {
        for food in self.all_the_apples.iter() {
            let food_translation = Mat4::from_translation(position + food.position);
            let right = food.front.cross(food.up).normalize();
            let food_rotation = Mat3::from_cols(right, food.front, food.up);
            let scale = food.size * (food.quality as f32).powf(1. / 3.);
            let food_matrix = Mat4::from_mat3(scale * food_rotation);
            self.model
                .draw_meshes(food_translation.mul_mat4(&food_matrix));
        }
    }
}
