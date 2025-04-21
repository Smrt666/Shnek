use draw_utils::Drawable;
use macroquad::prelude::*;
use macroquad::rand::*;

mod draw_utils;
// mod movement;
mod snake;

#[macroquad::main("Shnek")]
async fn main() {
    let test_cube = draw_utils::Cube {
        position: vec3(-10., 0., 0.),
        size: vec3(5., 5., 5.),
        color: RED,
        repeat: 10,
    };

    // let japka = draw_utils::Sphere {
    //     position : vec3(15., 15., 15.),
    //     radius: 5.,
    //     color: YELLOW,
    //     repeat: 2,
    // };

    let mut japka = draw_utils::Cube {
        position: vec3(10., 0., 0.),
        size: vec3(3., 3., 3.),
        color: YELLOW,
        repeat: 10,
    };
    

    let i = 20;
    let mut player = snake::Shnek::new();
    player.set_position(0., 0., 0.);
    player.set_direction(vec3(1., 0., 0.));
    for _ in 0..i {
        player.add_segment();
    }

    let grid = draw_utils::Grid::new();

    let mut rot_mat = Mat3::IDENTITY;
    loop {
        clear_background(DARKGRAY);
        let dir = rot_mat * vec3(1., 0., 0.);
        let up = rot_mat * vec3(0., 0., 1.);

        if is_key_down(KeyCode::E) {
            rot_mat = Mat3::from_axis_angle(dir, 0.02) * rot_mat;
        }
        if is_key_down(KeyCode::Q) {
            rot_mat = Mat3::from_axis_angle(dir, -0.02) * rot_mat;
        }
        if is_key_down(KeyCode::A) {
            rot_mat = Mat3::from_axis_angle(up, 0.02) * rot_mat;
        }
        if is_key_down(KeyCode::D) {
            rot_mat = Mat3::from_axis_angle(up, -0.02) * rot_mat;
        }
        if is_key_down(KeyCode::S) {
            rot_mat = Mat3::from_axis_angle(up.cross(dir), 0.02) * rot_mat;
        }
        if is_key_down(KeyCode::W) {
            rot_mat = Mat3::from_axis_angle(up.cross(dir), -0.02) * rot_mat;
        }



        let dist = player.get_position().distance(japka.get_position());
        if dist < 3. {
            player.add_segment();
            japka.position = random_vec3(0., 20.)
        }

        for seg in player.get_segments() {
            if seg.get_position().distance(player.get_position()) < 1. {
                set_default_camera();
                draw_text("GAME OVER", 40., 40., 50.0, BLACK);
                
            }
        }


        player.set_direction(dir);
        player.move_forward(0.5);

        let cam_offset = up * 5.0 - dir * 5.0;
        set_camera(&Camera3D {
            position: player.get_position() + cam_offset,
            up,
            target: player.get_position() + rot_mat * vec3(1., 0., 0.) + cam_offset,

            ..Default::default()
        });

        grid.draw();

        player.draw();
        test_cube.draw();

        japka.draw();




        // set_default_camera();
        // draw_text("GAME OVER", 40.0, 40.0, 50.0, BLACK);

        next_frame().await;
    }
}



fn random_vec3(min: f32, max: f32) -> Vec3 {
    vec3(
        gen_range(min, max),
        gen_range(min, max),
        gen_range(min, max),
    )
}

