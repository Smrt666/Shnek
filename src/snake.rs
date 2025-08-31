use std::collections::VecDeque;

use crate::food::{Food, FoodFactory};
use crate::draw_utils::SPACE_SIZE;
use crate::models3d::{Model3D, MultiModel};
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

    (dx * dx + dy * dy + dz * dz).sqrt()
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
        model.add_transformed(&Mat4::IDENTITY, 0);
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
        let transform = Mat4::from_translation(self.position).mul_mat4(&Mat4::from_mat3(rotation));
        self.model.base_transform(transform);
        self.model.draw();
    }
}

struct ShnekSegment {
    /// This is the position of the segment, position is not modulus-ed.
    position: Vec3,
    forward: Vec3,
    up: Vec3,
}
impl ShnekSegment {
    pub fn new(position: Vec3, forward: Vec3, up: Vec3) -> Self {
        Self {
            position,
            forward,
            up,
        }
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }

    pub fn set_direction(&mut self, direction: Vec3, up: Vec3) {
        self.forward = direction;
        self.up = up;
    }

    pub fn get_position(&self) -> Vec3 {
        self.position
    }
}

#[derive(Copy, Clone, Debug)]
struct HeadSnapshot {
    position: Vec3,
    direction: Vec3,
    up: Vec3,
    time: f32,
}

pub struct Shnek<'a> {
    segments: Vec<ShnekSegment>,
    head: ShnekHead<'a>,
    base_body_model: &'a Model3D,
    // historical positions of the head, used to know where the segments should be
    head_positions: VecDeque<HeadSnapshot>,
    speed: f32,
    time_moving: f32,
    time_boosted: f32,
}

impl<'a> Shnek<'a> {
    const SPACING: f32 = 10.0; // Approximate distance between segments
    const HEAD_SPACE: f32 = 10.0; // Distance between the head and the first segment

    pub fn new(base_head_model: &'a Model3D, base_body_model: &'a Model3D) -> Self {
        Self {
            segments: Vec::new(),
            base_body_model,
            head: ShnekHead::new(0.0, 0.0, 0.0, base_head_model),
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
                    self.get_position()
                } else {
                    self.segments[self.segments.len() - 2].get_position()
                };
                let new_pos = last_segment.get_position()
                    + (last_segment.get_position() - before_last_pos).normalize() * Shnek::SPACING;
                let forward = (last_segment.position - new_pos).normalize();
                ShnekSegment::new(new_pos, forward, last_segment.up)
            }
            None => {
                let head_pos = self.get_position();
                let head_dir = self.head.get_direction();
                let pos = head_pos - head_dir.normalize() * Shnek::HEAD_SPACE;

                ShnekSegment::new(pos, head_dir, self.head.up)
            }
        };
        self.segments.push(new_segment);
    }

    pub fn pop_segment(&mut self) {
        self.segments.pop();
    }

    pub fn move_forward(&mut self, dt: f32) {
        // Segments are some time behind the head
        // If there is no suitable position, the oldest one is used

        self.time_moving += dt;

        self.head.move_forward(dt * self.speed);
        self.head_positions.push_back(HeadSnapshot {
            position: self.get_position(),
            time: self.time_moving,
            up: self.head.up,
            direction: self.head.direction,
        });

        let mut j = (self.head_positions.len() - 1) as i32;
        for i in 0..self.segments.len() {
            let t = self.time_moving
                - i as f32 * (Shnek::SPACING / self.speed)
                - Shnek::HEAD_SPACE / self.speed;
            while j >= 0 && self.head_positions[j as usize].time > t {
                j -= 1;
            }
            if j >= 0 {
                let head_snapshot = self.head_positions[j as usize];
                self.segments[i].set_position(head_snapshot.position);
                self.segments[i].set_direction(head_snapshot.direction, head_snapshot.up);
            } else {
                let head_snapshot = self.head_positions[0];
                self.segments[i].set_position(head_snapshot.position);
                self.segments[i].set_direction(head_snapshot.direction, head_snapshot.up);
            }
        }
    }

    pub fn reset(&mut self) {
        self.time_moving = 0.0;
        self.time_boosted = 0.0;
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

    pub fn get_camera_position(&self) -> Vec3 {
        self.head.position + self.head.up * 2.0
    }

    pub fn get_length(&self) -> usize {
        self.segments.len()
    }

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
            // poops out food and shrinks
            food_factory.new_custom(self.segments.last().unwrap().get_position(), 3.0, 1);
        } else if self.time_boosted > 3. {
            return true;
        }
        false
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

    fn create_body_model(&mut self) -> MultiModel<'a> {
        let mut model = MultiModel::new(self.base_body_model, 3);
        for (id, segment) in self.segments.iter().enumerate() {
            let translation = Mat4::from_translation(segment.get_position());
            let right = segment.forward.cross(segment.up).normalize();
            let rotation = Mat4::from_mat3(Mat3::from_cols(segment.forward, segment.up, right));
            model.add_transformed(&translation.mul_mat4(&rotation), id);
        }
        model
    }

    pub fn draw(&mut self) {
        self.head.draw();
        self.create_body_model().draw(); // This could be cached in pause screen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn almost_eq(a: Vec3, b: Vec3) -> bool {
        (a.x - b.x).abs() < 1e-3 && (a.y - b.y).abs() < 1e-3 && (a.z - b.z).abs() < 1e-3
    }

    #[test]
    fn test_modulus() {
        assert!((modulus(5.0, 10.0) - 5.0).abs() < 1e-3);
        assert!((modulus(-5.0, 10.0) - 5.0).abs() < 1e-3);
        assert!((modulus(15.0, 10.0) - 5.0).abs() < 1e-3);
        assert!((modulus(-15.0, 10.0) - 5.0).abs() < 1e-3);
        assert!((modulus(0.0, 10.0) - 0.0).abs() < 1e-3);
        assert!((modulus(10.0, 10.0) - 0.0).abs() < 1e-3);
        assert!((modulus(-10.0, 10.0) - 0.0).abs() < 1e-3);
        assert!((modulus(-156.0, 10.0) - 4.0).abs() < 1e-3);

        assert_eq!(modulus(-0.0, 10.), 0.0)
    }

    #[test]
    fn test_mod_vec3() {
        assert!(almost_eq(
            modulus_vec3(vec3(101.3, 102.4, 103.6), SPACE_SIZE),
            vec3(1.3, 2.4, 3.6)
        ));
        assert!(almost_eq(
            modulus_vec3(vec3(300.0, 500.44, 200.55), SPACE_SIZE),
            vec3(0.0, 0.44, 0.55)
        ));
        assert!(almost_eq(
            modulus_vec3(vec3(-3., -0.0, -2.), SPACE_SIZE),
            modulus_vec3(vec3(97., 100., 98.), SPACE_SIZE)
        ));
        assert!(almost_eq(
            modulus_vec3(vec3(-13., 3., 115.), SPACE_SIZE),
            vec3(87., 3., 15.)
        ));
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
        assert!(mod_distance(vec3(10., 20., 30.), vec3(110., 120., 130.)) < 1e-3);
        assert!(mod_distance(vec3(10.2, 33.22, 3.1), vec3(5.6, 20.0, 49.3)) - 48.273889 < 1e-3);
        assert!(mod_distance(vec3(3.5, 88.6, 0.), vec3(198.3, 101.2, -130.)) - 32.951479 < 1e-3);
        assert!(
            mod_distance(
                vec3(0.33333333, 0.44444444, 0.55555555),
                vec3(-0.33333333, -0.44444444, -0.55555555)
            ) - 1.571348
                < 1e-3
        );
        assert!(
            mod_distance(
                vec3(100000000., 100000000., 100000000.),
                vec3(-200., -200., -200.)
            ) < 1e-3
        );
    }
}
