use std::vec;
use crate::draw_utils::SPACE_SIZE;
use crate::snake::*;
use macroquad::prelude::*;
use macroquad::rand::*;

use crate::models3d::{Model3D, MultiModel};

const DEBUG: bool = false;
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
    pub quality: i32,
    id: usize,
}

pub struct FoodFactory<'a> {
    quality_range: (i32, i32),
    all_the_apples: Vec<Food>,
    max_food: u32,
    model: MultiModel<'a>,
    id_counter: usize,
}

impl<'a> FoodFactory<'a> {
    pub fn new(base_model: &'a Model3D) -> Self {
        let model = MultiModel::new(base_model, 3);
        let mut s = Self {
            quality_range: (1, 2),
            all_the_apples: Vec::new(),
            max_food: 1,
            model,
            id_counter: 0,
        };
        s.new_custom(vec3(10., 10., 10.), 1., 1);
        s
    }

    pub fn new_custom(&mut self, position: Vec3, size: f32, quality: i32) {
        let front = vec3(0., 1., 0.);
        let up = vec3(0., 0., 1.);
        let food = Food::new_custom(position, up, front, size, quality, self.id_counter);
        let food_translation = Mat4::from_translation(food.position);
        let right = food.front.cross(food.up).normalize();
        let food_rotation = Mat3::from_cols(right, food.front, food.up);
        let scale = food.size * (food.quality as f32).powf(1. / 3.);
        let food_matrix = Mat4::from_mat3(scale * food_rotation);
        self.model
            .add_transformed(&food_translation.mul_mat4(&food_matrix), self.id_counter);
        self.id_counter += 1;
        self.all_the_apples.push(food);
    }

    pub fn new_random(&mut self, max_pos: f32, max_quality: i32) {
        let position = random_vec3(0., max_pos);
        let quality = gen_range(1, max_quality);
        self.new_custom(position, 1., quality);
    }

    pub fn remove_food(&mut self, i: usize) {
        self.model.remove_transformed(self.all_the_apples[i].id);
        self.model.refresh_transformed();
        self.all_the_apples.remove(i);
    }

    // pub fn check_food_collision(&mut self, snake: &mut Shnek, score: usize) -> bool {
    //     for &food in self.all_the_apples.clone().iter() {
    //         let dist = mod_distance(snake.get_position(), food.get_position());
    //         if dist < 3. {
    //             if food.quality > 0 {
    //                 for _ in 0..food.quality {
    //                     snake.add_segment();
    //                 }
    //             } else {
    //                 for _ in 0..(-food.quality as u32) {
    //                     snake.pop_segment();
    //                     draw_rectangle(
    //                         // draw a semi-transparent rectangle over the screen
    //                         0.0,
    //                         0.0,
    //                         screen_width(),
    //                         screen_height(),
    //                         RED,
    //                     );
    //                 }
    //             }
    //
    //             self.all_the_apples.retain(|&x| x != food);
    //             if food.color != BROWN {
    //                 let non_red = self
    //                     .all_the_apples
    //                     .iter()
    //                     .filter(|apple| apple.color != RED)
    //                     .count();
    //                 for _ in 0..gen_range(1, self.max_food as usize + 1 - non_red) {
    //                     if let Some(choice) = self.color_range.choose() {
    //                         self.all_the_apples.push(Food::new_random(
    //                             SPACE_SIZE,
    //                             (((score + 1) as f64).log10()).round() as i32 + 1,
    //                             *choice,
    //                         ));
    //                     }
    //                 }
    //
    //                 if score > 10 && gen_range(0, 100) < 5 {
    //                     self.all_the_apples.push(Food::new_custom(
    //                         random_vec3(0., SPACE_SIZE),
    //                         random_vec3(3., 5.),
    //                         (-(score as f64) / 10.).round() as i32,
    //                         RED,
    //                     ));
    //                 }
    //                 self.max_food = ((score as f64 * 2.).log10()).round() as u32 + 1;
    //             }
    //             return true;
    //         }
    //     }
    //
    //     false
    // }

    pub fn check_food_collision(&mut self, snake: &mut Shnek) -> (f32, bool) {
        let mut removed = vec![];
        let mut min_dist = SPACE_SIZE * 3.0;
        let mut eaten = false;
        for i in 0..self.all_the_apples.len() {
            let dist = snake
                .get_position()
                .distance(self.all_the_apples[i].get_position());
            if dist < min_dist {
                min_dist = dist;
            }
            if dist < 10. {
                eaten = true;
                for _ in 0..self.all_the_apples[i].quality {
                    snake.add_segment();
                }
                removed.push(i);
                // raise_max_food(food_factory);

                for _ in 0..gen_range(1, self.max_food) {
                    self.new_random(50., self.quality_range.1);
                }
            }
        }
        for i in removed {
            self.remove_food(i)
        }
        (min_dist, eaten)
    }
}

impl Food {
    fn new_custom(
        position: Vec3,
        up: Vec3,
        front: Vec3,
        size: f32,
        quality: i32,
        id: usize,
    ) -> Self {
        Self {
            position,
            up,
            front,
            size,
            quality,
            id,
        }
    }

    fn get_position(&self) -> Vec3 {
        modulus_vec3(self.position, SPACE_SIZE)
    }
}

impl FoodFactory<'_> {
    pub fn draw(&self) {
        self.model.draw();

        if DEBUG {
            let repeat = 2;
            for food in self.all_the_apples.iter() {
                for i in -repeat..=repeat {
                    for j in -repeat..=repeat {
                        for k in -repeat..=repeat {
                            let position = vec3(
                                i as f32 * SPACE_SIZE,
                                j as f32 * SPACE_SIZE,
                                k as f32 * SPACE_SIZE,
                            );
                            draw_cube(
                                food.get_position() + position,
                                vec3(15.0, 15.0, 15.0),
                                None,
                                Color::from_rgba(255, 0, 0, 100),
                            );
                        }
                    }
                }
            }
        }
    }
}
