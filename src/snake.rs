use std::collections::VecDeque;

use crate::{draw_utils::{Drawable, SPACE_SIZE}, food::{Food, FoodFactory}};
use macroquad::prelude::*;

/// A function to calculate the modulus of a float value with a given modulus.
/// It ensures that the result is always non-negative.
pub fn modulus(value: f32, m: f32) -> f32 {
    let mut result = value % m;
    if result < 0.0 {
        result += m;
    }
    result
}

pub fn modulus_vec3(value: Vec3, m: f32) -> Vec3 {
    vec3(
        modulus(value.x, m),
        modulus(value.y, m),
        modulus(value.z, m),
    )
}


pub fn mod_distance(v1: Vec3, v2: Vec3) -> f32 {
    fn mod_dist(x1: f32, x2: f32, m: f32) -> f32 {
        let diff = modulus((x1 - x2).abs(), m);
        diff.min(m - diff)
    }
    let m = SPACE_SIZE;
    let dx = mod_dist(v1.x, v2.x, m);
    let dy = mod_dist(v1.y, v2.y, m);
    let dz = mod_dist(v1.z, v2.z, m);

    return (dx * dx + dy * dy + dz * dz).sqrt()
}

pub struct ShnekHead {
    position: Vec3,
    direction: Vec3,
    /*
    Position is location within [0, SPACE_SIZE]^3
    Be careful, some things get weird when using modulus on floats.
     */
}

impl ShnekHead {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: vec3(x, y, z),
            direction: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.position = modulus_vec3(self.position + (self.direction * distance), SPACE_SIZE);
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = vec3(x, y, z);
    }

    pub fn set_direction(&mut self, d: Vec3) {
        self.direction = d;
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }
}

impl Drawable for ShnekHead {
    fn get_repeat(&self) -> i32 {
        5
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn draw_at(&self, position: Vec3, _saturation: f32) {
        draw_cube(position, vec3(5.0, 5.0, 5.0), None, GREEN);
    }
}

struct ShnekSegment {
    /// This is the position of the segment, position is not modulus-ed.
    /// This struct might get deleted once the snake will be drawn as a nice
    /// connected object.
    position: Vec3,
}
impl ShnekSegment {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: vec3(x, y, z),
        }
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = vec3(x, y, z);
    }

    pub fn get_position(&self) -> Vec3 {
        vec3(self.position.x, self.position.y, self.position.z)
    }
}
impl Drawable for ShnekSegment {
    fn get_repeat(&self) -> i32 {
        5
    }

    fn get_position(&self) -> Vec3 {
        vec3(self.position.x, self.position.y, self.position.z)
    }

    fn draw_at(&self, position: Vec3, _saturation: f32) {
        draw_cube(position, vec3(4.0, 4.0, 4.0), None, BLUE);
    }
}

pub struct Shnek {
    segments: Vec<ShnekSegment>,
    head: ShnekHead,
    // historical positions of the head, used to know where the segments should be
    head_positions: VecDeque<(Vec3, f32)>,
    speed: f32,
    time_moving: f32,
    time_boosted: f32,
}

impl Shnek {
    const SPACING: f32 = 5.0; // Approximate distance between segments

    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            head: ShnekHead::new(0.0, 0.0, 0.0),
            head_positions: VecDeque::new(),
            speed: 10.0,
            time_moving: 0.0,
            time_boosted: 0.0,
        }
    }

    pub fn add_segment(&mut self) {
        let new_segment = match self.segments.last() {
            Some(last_segment) => {
                let before_last_pos = if self.segments.len() < 2 {
                    self.head.get_position()
                } else {
                    self.segments[self.segments.len() - 2].get_position()
                };
                let new_pos = last_segment.get_position()
                    + (last_segment.get_position() - before_last_pos).normalize() * Shnek::SPACING;
                ShnekSegment::new(new_pos.x, new_pos.y, new_pos.z)
            }
            None => {
                let head_pos = self.head.get_position();
                let head_dir = self.head.get_direction();
                let pos = head_pos - head_dir.normalize() * Shnek::SPACING;

                ShnekSegment::new(pos.x, pos.y, pos.z)
            }
        };
        self.segments.push(new_segment);
    }

    pub fn move_forward(&mut self, dt: f32) {
        // Segments are some time behind the head
        // If there is no suitable position, the oldest one is used

        self.time_moving += dt;

        self.head.move_forward(dt * self.speed);
        self.head_positions
            .push_back((self.head.get_position(), self.time_moving));

        let mut j = (self.head_positions.len() - 1) as i32;
        for i in 0..self.segments.len() {
            let t = self.time_moving - i as f32 * (Shnek::SPACING / self.speed);
            while j >= 0 && self.head_positions[j as usize].1 > t {
                j -= 1;
            }
            if j >= 0 {
                let (pos, _) = self.head_positions[j as usize];
                self.segments[i].set_position(pos.x, pos.y, pos.z);
            } else {
                let (pos, _) = self.head_positions[0];
                self.segments[i].set_position(pos.x, pos.y, pos.z);
            }
        }
    }

    pub fn reset(&mut self) {
        self.time_moving = 0.0;
        self.time_boosted = 0.0;
        self.segments.clear();
        self.head_positions.clear();
    }

    pub fn set_direction(&mut self, d: Vec3) {
        self.head.set_direction(d);
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.head.set_position(x, y, z);
    }

    pub fn get_length(&self) -> usize {
        self.segments.len()
    }

    // pub fn get_speed(&self) -> f32 {
    //     self.speed
    // }
    // pub fn set_speed(&mut self, speed: f32) {
    //     self.speed = speed;
    // }

    pub fn check_boost_and_move(&mut self, dt: f32) {
        if is_key_down(KeyCode::LeftShift) {
            self.move_forward(dt * 2.);
            self.time_boosted += dt;
        } else {
            self.move_forward(dt);
        }
    }

    pub fn check_boost_time(&mut self, food_factory: &mut FoodFactory, start_len: usize) -> bool {
        if self.time_boosted > 3. && self.segments.len() > start_len {
            self.segments.pop();
            self.time_boosted -= 3.;
            food_factory.all_the_apples.push(
                Food::new_custom(self.segments.last().unwrap().get_position(), vec3(3., 3., 3.), 1, BROWN)
            ); // poops out food and shrinks
            
        } else if self.time_boosted > 3. {
            return true
        }
        return false
    }


    pub fn check_tail_collision(&self) -> bool {
        if self.time_moving < 2.0 {
            return false; // 2 s of spawn immunity
        }
        for segment in self.segments[1..].iter() {
            let dist = mod_distance(self.get_position(), segment.get_position());
            if dist < Shnek::SPACING * 0.8 {
                return true; // Collision detected
            }
        }
        false // No collision
    }
}

impl Drawable for Shnek {
    fn get_repeat(&self) -> i32 {
        5
    }

    fn get_position(&self) -> Vec3 {
        modulus_vec3(self.head.get_position(), SPACE_SIZE)
    }

    fn draw_at(&self, position: Vec3, saturation: f32) {
        self.head.draw_at(position, saturation);
        for segment in &self.segments {
            segment.draw_at(
                position + segment.get_position() - self.head.get_position(),
                saturation,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn almost_eq(a: Vec3, b: Vec3) -> bool {
        (a.x - b.x).abs() < 1e-5 && (a.y - b.y).abs() < 1e-5 && (a.z - b.z).abs() < 1e-5
    }

    #[test]
    fn test_modulus() {
        assert!((modulus(5.0, 10.0) - 5.0).abs() < 1e-5);
        assert!((modulus(-5.0, 10.0) - 5.0).abs() < 1e-5);
        assert!((modulus(15.0, 10.0) - 5.0).abs() < 1e-5);
        assert!((modulus(-15.0, 10.0) - 5.0).abs() < 1e-5);
        assert!((modulus(0.0, 10.0) - 0.0).abs() < 1e-5);
        assert!((modulus(10.0, 10.0) - 0.0).abs() < 1e-5);
        assert!((modulus(-10.0, 10.0) - 0.0).abs() < 1e-5);
        assert!((modulus(-156.0, 10.0) - 4.0).abs() < 1e-5);

        assert_eq!(modulus(-0.0, 10.), 0.0)
    }

    #[test]
    fn test_mod_vec3() {
        assert!(almost_eq(modulus_vec3(vec3(101.3, 102.4, 103.6), SPACE_SIZE), vec3(1.3, 2.4, 3.6)));
        assert!(almost_eq(modulus_vec3(vec3(300.0, 500.44, 200.55), SPACE_SIZE), vec3(0.0, 0.44, 0.55)));
        assert!(almost_eq(modulus_vec3(vec3(-3., -0.0, -2.), SPACE_SIZE), modulus_vec3(vec3(97., 100., 98.), SPACE_SIZE)));
        assert!(almost_eq(modulus_vec3(vec3(-13., 3., 115.), SPACE_SIZE), vec3(87., 3., 15.)));
    }

    #[test]
    fn test_addition() {
        let point1 = vec3(5.0, 10.0, 15.0);
        let change = vec3(10.0, 20.0, 30.0);
        let result = modulus_vec3(point1 + change, 20.0);
        assert!(almost_eq(result, vec3(15.0, 10.0, 5.0)));

        let point2 = vec3(5.0, 10.0, 15.0);
        let change2 = vec3(-10.0, -20.0, -30.0);
        let result2 = modulus_vec3(point2 + change2, 20.0);
        assert!(almost_eq(result2, vec3(15.0, 10.0, 5.0)));
    }

    #[test]
    fn test_mod_distance() {
        assert!(mod_distance(vec3(10., 20., 30.), vec3(110., 120., 130.)) < 1e-5);
        assert!(mod_distance(vec3(10.2, 33.22, 3.1), vec3(5.6, 20.0, 49.3)) - 48.273889 < 1e-5);
        assert!(mod_distance(vec3(3.5, 88.6, 0.), vec3(198.3, 101.2, -130.)) - 32.951479 < 1e-5);
        assert!(mod_distance(vec3(0.33333333, 0.44444444, 0.55555555), vec3(-0.33333333, -0.44444444, -0.55555555)) - 1.571348 < 1e-5);
        assert!(mod_distance(vec3(100000000., 100000000., 100000000.), vec3(-200., -200., -200.)) < 1e-5);
    }
}


