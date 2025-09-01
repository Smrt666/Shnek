use crate::draw_utils::SPACE_SIZE;
use crate::snake::*;
use macroquad::prelude::*;
use macroquad::rand::*;
use crate::models3d::{Model3D, MultiModel};

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    vec3(
        gen_range(min, max),
        gen_range(min, max),
        gen_range(min, max),
    )
}

#[derive(PartialEq, Copy, Clone)]
pub enum FoodVariant {
    Normal,
    Bad,
    Poop,
}

#[derive(Copy, Clone)]
pub struct Food {
    pub time_created: f32,
    pub position: Vec3,
    pub up: Vec3,
    pub front: Vec3,
    pub size: f32,
    pub quality: u32,
    id: usize,
}

pub struct FoodFactory<'a> {
    quality_range: (u32, u32),
    good_food: Vec<Food>,
    good_food_model: MultiModel<'a>,
    bad_food: Vec<Food>,
    bad_food_model: MultiModel<'a>,
    poop: Vec<Food>,
    poop_model: MultiModel<'a>,
    pub max_food: u32,
    id_counter: usize,
}

impl<'a> FoodFactory<'a> {
    const FOOD_COLLISION_DISTANCE: f32 = 10.0;
    const BAD_FOOD_LIFETIME: f32 = 30.0;  // seconds (or halved if boost moving)

    pub fn new(base_good_food_model: &'a Model3D, base_bad_food_model: &'a Model3D, base_poop_model: &'a Model3D) -> Self {
        let mut s = Self {
            quality_range: (1, 2),
            good_food: Vec::new(),
            bad_food: Vec::new(),
            poop: Vec::new(),
            max_food: 1,
            good_food_model: MultiModel::new(base_good_food_model, 3),
            bad_food_model: MultiModel::new(base_bad_food_model, 3),
            poop_model: MultiModel::new(base_poop_model, 3),
            id_counter: 0,
        };
        let front = vec3(0., 1., 0.);
        let up = vec3(0., 0., 1.);
        s.new_custom(vec3(10., 10., 10.), 1., 1, FoodVariant::Normal, front, up, 0.0);
        s
    }

    pub fn new_custom(
        &mut self,
        position: Vec3,
        size: f32,
        quality: u32,
        variant: FoodVariant,
        front: Vec3,
        up: Vec3,
        snake_time: f32,
    ) {
        let food = Food::new_custom(position, up, front, size, quality, self.id_counter, snake_time);
        let food_translation = Mat4::from_translation(food.position);
        let right = food.front.cross(food.up).normalize();
        let food_rotation = Mat3::from_cols(right, food.front, food.up);
        let scale = food.size * (food.quality as f32).powf(1. / 3.);
        let food_matrix = food_translation.mul_mat4(&Mat4::from_mat3(scale * food_rotation));

        // Update models
        match variant {
            FoodVariant::Normal => {
                self.good_food_model.add_transformed(&food_matrix, self.id_counter);
                self.good_food.push(food);
            }
            FoodVariant::Poop => {
                self.poop_model
                    .add_transformed(&food_matrix, self.id_counter);
                self.poop.push(food);
            }
            FoodVariant::Bad => {
                self.bad_food_model.add_transformed(&food_matrix, self.id_counter);
                self.bad_food.push(food);
            }
        }
        self.id_counter += 1;
    }

    pub fn new_random(&mut self, max_pos: f32, food_variant: FoodVariant, snake_time: f32) {
        let position = random_vec3(0., max_pos);
        let quality = gen_range(self.quality_range.0, self.quality_range.1);
        let front = vec3(0., 1., 0.);
        let up = vec3(0., 0., 1.);
        self.new_custom(position, 1., quality, food_variant, front, up, snake_time);
    }

    pub fn new_random_with_quality(&mut self, max_pos: f32, food_variant: FoodVariant, quality: u32, snake_time: f32) {
        let position = random_vec3(0., max_pos);
        let front = vec3(0., 1., 0.);
        let up = vec3(0., 0., 1.);
        self.new_custom(position, 1., quality, food_variant, front, up, snake_time);
    }

    pub fn remove_food_model(&mut self, i: usize, variant: FoodVariant) {
        match variant {
            FoodVariant::Normal => {
                self.good_food_model.remove_transformed(self.good_food[i].id);
                self.good_food_model.refresh_transformed();
                self.good_food.remove(i);
            }
            FoodVariant::Bad => {
                self.bad_food_model.remove_transformed(self.bad_food[i].id);
                self.bad_food_model.refresh_transformed();
                self.bad_food.remove(i);
            }
            FoodVariant::Poop => {
                self.poop_model.remove_transformed(self.poop[i].id);
                self.poop_model.refresh_transformed();
                self.poop.remove(i);
            }
        }
    }

    pub fn food_count(&self) -> usize {
        self.good_food.len()
    }

    fn generate_food(&mut self, snake: &Shnek, remove_count: usize) {
        let score = snake.get_score();
        // make new good food
        let max_new = gen_range(1, self.max_food as usize + 1 - self.food_count() + remove_count);
        for _ in 0..max_new {
            if self.food_count() < self.max_food as usize + remove_count {
                self.new_random(SPACE_SIZE, FoodVariant::Normal, snake.time_moving);
            }
        }
        // make new bad food 40 % of the time (when score > 5)
        if score > 5 && gen_range(0, 100) < 40 {
            self.new_random_with_quality(SPACE_SIZE, FoodVariant::Bad, (score / 5).min(1) as u32, snake.time_moving);
        }
    }

    fn check_good_food_collision(&mut self, snake: &mut Shnek) -> (f32, bool) {
        let mut min_dist = SPACE_SIZE * 3.0;
        let mut eaten = false;
        let mut remove: Vec<usize> = Vec::new();
        for i in 0..self.good_food.len() {
            let food = &self.good_food[i];
            let dist = mod_distance(snake.get_position(), food.get_position());
            if dist < min_dist {
                min_dist = dist;
            }
            // We are eating
            if dist < Self::FOOD_COLLISION_DISTANCE {
                eaten = true;
                for _ in 0..food.quality {
                    snake.add_segment();
                }
                remove.push(i);
                self.generate_food(snake, 1);
            }
        }
        for i in remove.iter().rev() {
            self.remove_food_model(*i, FoodVariant::Normal);
        }
        (min_dist, eaten)
    }

    fn check_bad_food_collision(&mut self, snake: &mut Shnek) -> (f32, bool) {
        let mut min_dist = SPACE_SIZE * 3.0;
        let mut eaten = false;
        let mut remove: Vec<usize> = Vec::new();
        for i in 0..self.bad_food.len() {
            let food = &self.bad_food[i];
            // Bad food expires after some time
            if snake.time_moving - food.time_created > Self::BAD_FOOD_LIFETIME {
                remove.push(i);
                continue;
            }
            let dist = mod_distance(snake.get_position(), food.get_position());
            if dist < min_dist {
                min_dist = dist;
            }
            // We are eating, do not generate new food
            if dist < Self::FOOD_COLLISION_DISTANCE {
                eaten = true;
                for _ in 0..food.quality {
                    snake.pop_segment();
                }
                remove.push(i);
            }
        }
        for i in remove.iter().rev() {
            self.remove_food_model(*i, FoodVariant::Bad);
        }
        (min_dist, eaten)
    }

    fn check_poop_collision(&mut self, snake: &mut Shnek) -> (f32, bool) {
        let mut min_dist = SPACE_SIZE * 3.0;
        let mut eaten = false;
        let mut remove: Vec<usize> = Vec::new();
        for i in 0..self.poop.len() {
            let food = &self.poop[i];
            let dist = mod_distance(snake.get_position(), food.get_position());
            if dist < min_dist {
                min_dist = dist;
            }
            // We are eating, do not generate new food
            if dist < Self::FOOD_COLLISION_DISTANCE {
                eaten = true;
                for _ in 0..food.quality {
                    snake.add_segment();
                }
                remove.push(i);
            }
        }
        for i in remove.iter().rev() {
            self.remove_food_model(*i, FoodVariant::Poop);
        }
        (min_dist, eaten)
    }

    pub fn check_food_collision(&mut self, snake: &mut Shnek) -> (f32, bool) {
        let score = snake.get_score();

        let (md1, eaten1) = self.check_good_food_collision(snake);
        let (md2, eaten2) = self.check_bad_food_collision(snake);
        let (md3, eaten3) = self.check_poop_collision(snake);

        self.quality_range.1 = (((score + 1) as f64).log10()).round() as u32 + 1;
        self.max_food = ((score as f64 * 2.).log10()).round() as u32 + 1;

        (md1.min(md2.min(md3)), eaten1 || eaten2 || eaten3)
    }
}

impl Food {
    fn new_custom(
        position: Vec3,
        up: Vec3,
        front: Vec3,
        size: f32,
        quality: u32,
        id: usize,
        time_created: f32,
    ) -> Self {
        Self {
            position,
            up,
            front,
            size,
            quality,
            id,
            time_created,
        }
    }

    fn get_position(&self) -> Vec3 {
        modulus_vec3(self.position, SPACE_SIZE)
    }
}

impl FoodFactory<'_> {
    pub fn draw(&self) {
        self.good_food_model.draw();
        self.bad_food_model.draw();
        self.poop_model.draw();
    }
}
