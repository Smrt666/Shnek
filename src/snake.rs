use std::ops::{Add, Sub, Mul};
use macroquad::prelude::*;
use crate::draw_utils::{
    Drawable,
    SPACE_SIZE,
};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModPoint {
    x: f32,
    y: f32,
    z: f32,
    m: f32,  // modulus
}

fn modulus(value: f32, m: f32) -> f32 {
    let mut result = value % m;
    if result < 0.0 {
        result += m;
    }
    result
}

impl ModPoint {
    fn new(x: f32, y: f32, z: f32, m: f32) -> Self {
        Self { x, y, z, m }
    }

    fn almost_eq(self, other: Vec3) -> bool {
        (self.x - other.x).abs() < 1e-5 &&
        (self.y - other.y).abs() < 1e-5 &&
        (self.z - other.z).abs() < 1e-5
    }
}

impl Mul<f32> for ModPoint {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: modulus(self.x * scalar, self.m),
            y: modulus(self.y * scalar, self.m),
            z: modulus(self.z * scalar, self.m),
            m: self.m,
        }
    }
}


impl Add<Vec3> for ModPoint {
    type Output = Self;

    fn add(self, other: Vec3) -> Self::Output {
        Self {
            x: modulus(self.x + other.x, self.m),
            y: modulus(self.y + other.y, self.m),
            z: modulus(self.z + other.z, self.m),
            m: self.m,
        }
    }
}

impl Sub<Vec3> for ModPoint {
    type Output = Self;

    fn sub(self, other: Vec3) -> Self::Output {
        Self {
            x: modulus(self.x - other.x, self.m),
            y: modulus(self.y - other.y, self.m),
            z: modulus(self.z - other.z, self.m),
            m: self.m,
        }
    }
}


pub struct ShnekHead {
    position: ModPoint,
    direction: Vec3,
}


impl ShnekHead {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: ModPoint { x, y, z, m: SPACE_SIZE },
            direction: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn move_forward(&mut self, distance: f32) {
        self.position = self.position + (self.direction * distance);
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.position = ModPoint::new(x, y, z, self.position.m);
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


#[cfg(test)]
mod tests {
    use super::*;

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
        let point1 = ModPoint::new(5.0, 10.0, 15.0, 20.0);
        let change = vec3(10.0, 20.0, 30.0);
        let result = point1 + change;
        assert!(result.almost_eq(vec3(15.0, 10.0, 5.0)));

        let point2 = ModPoint::new(5.0, 10.0, 15.0, 20.0);
        let change2 = vec3(-10.0, -20.0, -30.0);
        let result2 = point2 + change2;
        assert!(result2.almost_eq(vec3(15.0, 10.0, 5.0)));
    }

    #[test]
    fn test_move_head() {
        let mut head = ShnekHead::new(5.0, 10.0, 15.0);
        head.set_direction(1.0, 0.0, 0.0);
        head.move_forward(5.0);
        assert!(head.position.almost_eq(vec3(10.0, 10.0, 15.0)));

        head.set_direction(0.0, 1.0, 0.0);
        head.move_forward(150.0);
        assert!(head.position.almost_eq(vec3(10.0, 60.0, 15.0)));
    }
}