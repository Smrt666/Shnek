use macroquad::prelude::*;
// use macroquad::rand::*;


pub const SPACE_SIZE: f32 = 100.0;

pub trait Drawable {
    /// Returns the number of times to repeat the object in each direction.
    fn get_repeat(&self) -> i32;

    /// Returns the position of the object.
    fn get_position(&self) -> Vec3;

    fn draw_at(&self, position: Vec3, _saturation: f32);

    fn draw(&self) {
        let repeat = self.get_repeat();
        let origin = self.get_position();
        for i in -repeat..=repeat {
            for j in -repeat..=repeat {
                for k in -repeat..=repeat {
                    let position = vec3(
                        i as f32 * SPACE_SIZE,
                        j as f32 * SPACE_SIZE,
                        k as f32 * SPACE_SIZE,
                    );
                    let position = position + origin;
                    let saturation =
                        1.0 - (i * i + j * j + k * k) as f32 / (repeat * repeat * 3) as f32;
                    let saturation = saturation.max(0.0);

                    self.draw_at(position, saturation);
                }
            }
        }
    }
}

pub struct Cube {
    pub position: Vec3,
    pub size: Vec3,
    pub color: Color,
    pub repeat: i32,
}

impl Drawable for Cube {
    fn get_repeat(&self) -> i32 {
        self.repeat
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn draw_at(&self, position: Vec3, saturation: f32) {
        let mut color = self.color;
        color.r *= saturation;
        color.g *= saturation;
        color.b *= saturation;

        draw_cube(position, self.size, None, color);
    }
}

pub struct Sphere {
    pub position: Vec3,
    pub radius: f32,
    pub color: Color,
    pub repeat: i32,
}

impl Drawable for Sphere {
    fn get_repeat(&self) -> i32 {
        self.repeat
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn draw_at(&self, position: Vec3, _saturation: f32) {
        let color = self.color;
        // color.r *= saturation;
        // color.g *= saturation;
        // color.b *= saturation;

        draw_sphere(position, self.radius, None, color)
    }
}




pub struct Grid {
    repeat: i32,
}

impl Grid {
    pub fn new() -> Self {
        Self { repeat: 3 }
    }
}

impl Drawable for Grid {
    // TODO: This is very unoptimized,  many lines are drawn at the same place.
    fn get_repeat(&self) -> i32 {
        self.repeat
    }

    fn get_position(&self) -> Vec3 {
        vec3(0., 0., 0.)
    }

    fn draw_at(&self, position: Vec3, _: f32) {
        let color = Color::from_rgba(0, 0, 0, 10);
        draw_line_3d(
            position + vec3(-SPACE_SIZE * self.repeat as f32, 0., 0.),
            position + vec3(SPACE_SIZE * self.repeat as f32, 0., 0.),
            color,
        );
        draw_line_3d(
            position + vec3(0., -SPACE_SIZE * self.repeat as f32, 0.),
            position + vec3(0., SPACE_SIZE * self.repeat as f32, 0.),
            color,
        );
        draw_line_3d(
            position + vec3(0., 0., -SPACE_SIZE * self.repeat as f32),
            position + vec3(0., 0., SPACE_SIZE * self.repeat as f32),
            color,
        );
    }
}
