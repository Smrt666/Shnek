use std::collections::VecDeque;

use crate::draw_utils::{Drawable, SPACE_SIZE};
use macroquad::prelude::*;
use crate::models3d::{Model3D, MultiModel};

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

pub struct ShnekHead<'a> {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    model: MultiModel<'a>,
    /*
    Position is location within [0, SPACE_SIZE]^3
    Be careful, some things get weird when using modulus on floats.
     */
}

impl<'a> ShnekHead<'a> {
    pub fn new(x: f32, y: f32, z: f32, base_model: &'a Model3D) -> Self {
        let mut model = MultiModel::new(base_model, 3);
        model.add_transformed(&Mat4::IDENTITY);
        Self {
            position: vec3(x, y, z),
            direction: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 0.0, 1.0),
            model,
        }
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.position = modulus_vec3(self.position + (self.direction * distance), SPACE_SIZE);
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = vec3(x, y, z);
    }

    pub fn set_direction(&mut self, d: Vec3, up: Vec3) {
        self.direction = d;
        self.up = up;
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    pub fn draw(&mut self) {
        let right = self.direction.cross(self.up).normalize();
        let rotation = Mat3::from_cols(self.direction, self.up, right);
        let transform = Mat4::from_translation(self.position)
            .mul_mat4(&Mat4::from_mat3(rotation));
        self.model.base_transform(&transform);
        self.model.draw();
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

pub struct Shnek<'a> {
    segments: Vec<ShnekSegment>,
    head: ShnekHead<'a>,
    // historical positions of the head, used to know where the segments should be
    head_positions: VecDeque<(Vec3, f32)>,
    speed: f32,
    time_moving: f32,
}

impl<'a> Shnek<'a> {
    const SPACING: f32 = 5.0; // Approximate distance between segments

    pub fn new(base_head_model: &'a Model3D) -> Self {
        Self {
            segments: Vec::new(),
            head: ShnekHead::new(0.0, 0.0, 0.0, base_head_model),
            head_positions: VecDeque::new(),
            speed: 10.0,
            time_moving: 0.0,
        }
    }

    pub fn add_segment(&mut self) {
        let new_segment = match self.segments.last() {
            Some(last_segment) => {
                let before_last_pos = if self.segments.len() < 2 {
                    self.get_position()
                } else {
                    self.segments[self.segments.len() - 2].get_position()
                };
                let new_pos = last_segment.get_position()
                    + (last_segment.get_position() - before_last_pos).normalize() * Shnek::SPACING;
                ShnekSegment::new(new_pos.x, new_pos.y, new_pos.z)
            }
            None => {
                let head_pos = self.get_position();
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
            .push_back((self.get_position(), self.time_moving));

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
        self.segments.clear();
        self.head_positions.clear();
        self.set_position(0., 0., 0.);
        self.set_direction(vec3(1., 0., 0.), vec3(0., 0., 1.));
    }

    pub fn set_direction(&mut self, d: Vec3, up: Vec3) {
        self.head.set_direction(d, up);
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.head.set_position(x, y, z);
    }

    pub fn get_position(&self) -> Vec3 {
        self.head.position
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

    pub fn check_tail_collision(&self) -> bool {
        if self.time_moving < 2.0 {
            return false; // 2 s of spawn immunity
        }
        for segment in self.segments[1..].iter() {
            let dist = self.get_position().distance(segment.get_position());
            if dist < Shnek::SPACING * 0.8 {
                return true; // Collision detected
            }
        }
        false // No collision
    }

    pub fn draw(&mut self) {
        self.head.draw();
        // TODO: Draw body
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
