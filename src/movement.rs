use macroquad::prelude::*;

pub struct View {
    rot_mat: Mat3,
    time_rotating: f32, // How long have wasdqe been pressed
}

impl View {
    pub fn new() -> Self {
        Self {
            rot_mat: Mat3::IDENTITY,
            time_rotating: 0.0,
        }
    }

    pub fn up(&self) -> Vec3 {
        self.rot_mat * vec3(0., 1., 0.)
    }

    pub fn forward(&self) -> Vec3 {
        self.rot_mat * vec3(1., 0., 0.)
    }

    pub fn right(&self) -> Vec3 {
        self.rot_mat * vec3(0., 0., 1.)
    }

    pub fn cam_offset(&self) -> Vec3 {
        self.up() * 5.0 - self.forward() * 5.0
    }

    pub fn rotate(&mut self, dt: f32) {
        let dir = self.forward();
        let up = self.up();
        let right = self.right();

        let rot_speed = (self.time_rotating * 0.5 + 0.5) * dt;
        let rot_speed = rot_speed.min(10.0); // Limit the rotation speed

        if is_key_down(KeyCode::E) {
            self.rot_mat = Mat3::from_axis_angle(dir, rot_speed) * self.rot_mat;
        }
        if is_key_down(KeyCode::Q) {
            self.rot_mat = Mat3::from_axis_angle(dir, -rot_speed) * self.rot_mat;
        }
        if is_key_down(KeyCode::A) {
            self.rot_mat = Mat3::from_axis_angle(up, rot_speed) * self.rot_mat;
        }
        if is_key_down(KeyCode::D) {
            self.rot_mat = Mat3::from_axis_angle(up, -rot_speed) * self.rot_mat;
        }
        if is_key_down(KeyCode::S) {
            self.rot_mat = Mat3::from_axis_angle(right, -rot_speed) * self.rot_mat;
        }
        if is_key_down(KeyCode::W) {
            self.rot_mat = Mat3::from_axis_angle(right, rot_speed) * self.rot_mat;
        }

        if is_key_down(KeyCode::E)
            || is_key_down(KeyCode::Q)
            || is_key_down(KeyCode::A)
            || is_key_down(KeyCode::D)
            || is_key_down(KeyCode::S)
            || is_key_down(KeyCode::W)
        {
            self.time_rotating += dt;
        } else {
            self.time_rotating = 0.0;
        }

        // Cam rotation debug
        // if is_key_down(KeyCode::Enter) {
        //     println!("{}", self.rot_mat);
        // }
        //
        // if is_key_down(KeyCode::Space) {
        //     println!("{}", self.rot_mat);
        //     println!("Resetting camera");
        //     self.rot_mat = Mat3::IDENTITY;
        // }
    }

    pub fn set_camera(&self, player_pos: Vec3) {
        let cam_offset = self.cam_offset();
        set_camera(&Camera3D {
            position: player_pos + cam_offset,
            up: self.up(),
            target: player_pos + self.forward() + cam_offset,
            ..Default::default()
        });
    }

    pub fn reset(&mut self) {
        self.rot_mat = Mat3::IDENTITY;
        self.time_rotating = 0.0;
    }
}
