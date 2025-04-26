use draw_utils::Drawable;
use food::check_tail_collision;
use macroquad::prelude::*;
// use macroquad::rand::*;
use crate::food::check_food_collision;

mod draw_utils;
// mod movement;
mod snake;
mod food;

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

    // let mut japka = food::Food {
    //     position: vec3(10., 10., 10.),
    //     size: vec3(3., 3., 3.),
    //     quality: 1,
    //     color: YELLOW,
    //     repeat: 5,
    // };
    

    let i = 22;
    let mut player = snake::Shnek::new();
    player.set_position(0., 0., 0.);
    player.set_direction(vec3(1., 0., 0.));
    for _ in 0..i {
        player.add_segment();
    }

    let mut food_factory = food::FoodFactory::new();

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


        check_food_collision(&mut player, &mut food_factory);
        check_tail_collision(&player);


        // for seg in player.get_segments() {
        //     if seg.get_position().distance(player.get_position()) < 1. {
        //         set_default_camera();
        //         draw_text("GAME OVER", 40., 40., 50.0, BLACK);
                
        //     }
        // }


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

        for food in food_factory.get_apples() {
            food.draw();
        }




        // set_default_camera();
        // draw_text("GAME OVER", 40.0, 40.0, 50.0, BLACK);

        next_frame().await;
    }
}


