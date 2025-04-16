use std::{collections::VecDeque, vec};

use crate::draw_utils::{Drawable, SPACE_SIZE};
use macroquad::prelude::*;

fn modulus(value: f32, m: f32) -> f32 {
    let mut result = value % m;
    if result < 0.0 {
        result += m;
    }
    result
}

fn modulus_vec3(value: Vec3, m: f32) -> Vec3 {
    vec3(modulus(value.x, m), modulus(value.y, m), modulus(value.z, m))
}

pub struct ShnekHead {
    position: Vec3,
    direction: Vec3,
}

impl ShnekHead {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: vec3(x, y, z),
            direction: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.position = self.position + (self.direction * distance);
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = vec3(x, y, z);
    }

    pub fn set_direction(&mut self, x: f32, y: f32, z: f32) {
        self.direction = Vec3::new(x, y, z);
    }

    pub fn get_position(&self) -> Vec3 {
        vec3(self.position.x, self.position.y, self.position.z)
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
        vec3(self.position.x, self.position.y, self.position.z)
    }

    fn draw_at(&self, position: Vec3, saturation: f32) {
        draw_cube(position, vec3(5.0, 5.0, 5.0), None, GREEN);
    }
}

struct ShnekSegment {
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

    fn draw_at(&self, position: Vec3, saturation: f32) {
        draw_cube(position, vec3(4.0, 4.0, 4.0), None, BLUE);
    }
}

pub struct Shnek {
    segments: Vec<ShnekSegment>,
    head: ShnekHead,
    head_positions: VecDeque<Vec3>,
}

impl Shnek {
    const SPACING : f32 = 5.0;
    const FRAMES_DISTANCE : usize = 50;

    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
            head: ShnekHead::new(0.0, 0.0, 0.0),
            head_positions: VecDeque::new(),
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
                let new_pos = last_segment.get_position() + (last_segment.get_position() - before_last_pos).normalize() * Shnek::SPACING;
                ShnekSegment::new(new_pos.x, new_pos.y, new_pos.z)
            }
            None => {
                let head_pos = self.head.get_position();
                let head_dir = self.head.get_direction();
                let pos = head_pos - head_dir.normalize() * Shnek::SPACING;
                let segment = ShnekSegment::new(pos.x, pos.y, pos.z);
                segment
            }
        };
        self.segments.push(new_segment);
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.head.move_forward(distance);
        self.head_positions.push_back(self.head.get_position());

        for i in 0..self.segments.len() {
            let j = Shnek::FRAMES_DISTANCE * (i + 1);
            if j < self.head_positions.len() {
                let pos = self.head_positions[self.head_positions.len() - j];
                self.segments[i].set_position(pos.x, pos.y, pos.z);
            } else {
                let pos = self.head_positions[0];
                self.segments[i].set_position(pos.x, pos.y, pos.z);
            }
        }
    }

    pub fn set_direction(&mut self, x: f32, y: f32, z: f32) {
        self.head.set_direction(x, y, z);
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.head.set_position(x, y, z);
    }
}


impl Drawable for Shnek {
    fn get_repeat(&self) -> i32 {
        5
    }

    fn get_position(&self) -> Vec3 {
        vec3(
            modulus(self.head.get_position().x, SPACE_SIZE),
            modulus(self.head.get_position().y, SPACE_SIZE),
            modulus(self.head.get_position().z, SPACE_SIZE),
        )
    }

    fn draw_at(&self, position: Vec3, saturation: f32) {
        self.head.draw_at(position, saturation);
        for segment in &self.segments {
            segment.draw_at(position + segment.get_position() - self.head.get_position(), saturation);
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
}
