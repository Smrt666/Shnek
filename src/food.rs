use std::collections::HashMap;
use std::env::current_dir;

use crate::draw_utils::Drawable;
use crate::snake::*;
use macroquad::file;
use macroquad::prelude::*;
use macroquad::rand::*;
use tobj::{Model, Material};

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    vec3(
        gen_range(min, max),
        gen_range(min, max),
        gen_range(min, max),
    )
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Food {
    pub position: Vec3,
    pub size: Vec3,
    pub quality: u32,
    pub color: Color,
    pub repeat: i32,
}

pub struct FoodFactory {
    spawn_region: f32,
    quality_range: (u32, u32),
    all_the_apples: Vec<Food>,
    max_food: u32,
    // size_range: Vec<u32>,
    // color_range: Vec<Color>,
    models: Vec<Model>,
    materials: Vec<Material>,
    textures: HashMap<String, Texture2D>,
}

impl FoodFactory {
    pub fn new() -> Self {
        Self {
            spawn_region: 50.,
            quality_range: (1, 1),
            all_the_apples: vec![Food::new_custom(
                vec3(10., 0., 0.),
                vec3(3., 3., 3.),
                1,
                YELLOW,
            )],
            max_food: 1,
            models: vec![],
            materials: vec![],
            textures: HashMap::new(),
        }
    }

    pub async fn add_modelerial(&mut self, model: Model, material: Material) {
        self.models.push(model);
        let ntexture = match &material.normal_texture {
            Some(texture_file_name) => texture_file_name.clone(),
            None => return,
            
        };
        self.materials.push(material);
        let filename = &format!("assets/test_obj/{}", &ntexture.replace("\\\\", "/"));
        println!("Loading texture: {}", filename);
        println!("Current dir: {:?}", current_dir().unwrap());
        println!("file exists: {:?}", std::fs::exists(filename));
        self.textures.insert(
            ntexture.clone(),
            Texture2D::from_image(&load_image(filename).await.unwrap()), 
        );
    }

    fn get_spawn(&self) -> f32 {
        self.spawn_region
    }

    pub fn raise_max_food(&mut self) {
        self.max_food += 1;
    }

    pub fn check_food_collision(&mut self, snake: &mut Shnek) {
        for &food in self.all_the_apples.clone().iter() {
            let dist = snake.get_position().distance(food.get_position());
            if dist < 3. {
                for _ in 0..food.quality {
                    snake.add_segment();
                }

                self.all_the_apples.retain(|&x| x != food);

                for _ in 0..gen_range(1, self.max_food) {
                    self.all_the_apples.push(Food::new_random(50., 2));
                }

                // raise_max_food(food_factory);
            }
        }
    }

    pub fn draw_food(&self) {
        for food in self.all_the_apples.iter() {
            food.draw(Some(&self.models), Some(&self.materials), Some(&self.textures));
        }
    }
}

impl Food {
    fn new_custom(position: Vec3, size: Vec3, quality: u32, color: Color) -> Self {
        Self {
            position,
            size,
            quality,
            color,
            repeat: 5,
        }
    }

    fn new_random(max_pos: f32, max_quality: u32) -> Self {
        Self {
            position: random_vec3(0., max_pos),
            size: random_vec3(3., 5.),
            quality: gen_range(1, max_quality),
            color: YELLOW,
            repeat: 5,
        }
    }

    fn get_quality(&self) -> u32 {
        self.quality
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }
}

fn tobj_model_to_mesh(model: &Model, material: &Material, textures: &HashMap<String, Texture2D>) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();
    for i in model.mesh.indices.iter() {
        let i = *i as usize;
        let x = model.mesh.positions[i * 3];
        let y = model.mesh.positions[i * 3 + 1];
        let z = model.mesh.positions[i * 3 + 2];

        // Could not exist
        let u = model.mesh.texcoords.get(i * 2);
        let v = model.mesh.texcoords.get(i * 2 + 1);
        let uv = match (u, v) {
            (Some(u), Some(v)) => vec2(*u, *v),
            _ => vec2(0.0, 0.0),
        };

        // Could not exist (normals are not used by default macroquad)
        let nx = model.mesh.normals.get(i * 3);
        let ny = model.mesh.normals.get(i * 3 + 1);
        let nz = model.mesh.normals.get(i * 3 + 2);
        let normal = match (nx, ny, nz) {
            (Some(nx), Some(ny), Some(nz)) => vec4(*nx, *ny, *nz, 1.0),
            _ => vec4(0.0, 0.0, 0.0, 1.0),
        };

        vertices.push(Vertex {
            position: vec3(x, y, z),
            uv: uv,
            color: [0, 0, 0, 0],
            normal: normal,
        });
        indices.push(i as u16);
    }
    let texture = match &material.normal_texture {
        Some(texture_file_name) => textures.get(texture_file_name),
        None => None,
    };
    Mesh { vertices, indices: indices, texture: texture.cloned()}
}

impl Drawable for Food {
    fn get_repeat(&self) -> i32 {
        self.repeat
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn draw_at(&self, position: Vec3, _saturation: f32, models: Option<&Vec<Model>>, materials: Option<&Vec<Material>>, textures: Option<&HashMap<String, Texture2D>>) {
        match (models, materials, textures) {
            (Some(models), Some(materials), Some(textures)) => {
                for model in models.iter() {
                    let mat_id = model.mesh.material_id.unwrap_or(0);
                    let material = &materials[mat_id];
                    let mesh = tobj_model_to_mesh(model, material, textures);
                    draw_mesh(&mesh);
                }
            }
            _ => {
                draw_cube(position, self.size, None, self.color);
            }
        }
    }
}
