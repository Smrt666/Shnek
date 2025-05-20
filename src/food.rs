use std::collections::HashMap;
use std::env::current_dir;
use std::vec;

use crate::draw_utils::Drawable;
use crate::snake::*;
use image::GenericImageView;
use macroquad::prelude::*;
use macroquad::rand::*;

use image::ImageReader;
use tobj::{Model, Material};

pub fn random_vec3(min: f32, max: f32) -> Vec3 {
    vec3(
        gen_range(min, max),
        gen_range(min, max),
        gen_range(min, max),
    )
}

pub struct Food {
    pub position: Vec3,
    pub size: Vec3,
    pub quality: u32,
    pub color: Color,
    pub repeat: i32,
    mesh: Option<Mesh>,
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
        let image = ImageReader::open(filename).unwrap().decode().unwrap();
        let (width, height) = image.dimensions();
        let bytes = image.to_rgba8();
        self.textures.insert(
            ntexture.clone(),
            Texture2D::from_rgba8(width as u16, height as u16, &bytes), 
        );
    }

    fn get_spawn(&self) -> f32 {
        self.spawn_region
    }

    pub fn raise_max_food(&mut self) {
        self.max_food += 1;
    }

    pub fn check_food_collision(&mut self, snake: &mut Shnek) {
        let mut removed = vec![];
        let mut new_food = vec![];
        for i in 0..self.all_the_apples.len() {
            let dist = snake.get_position().distance(self.all_the_apples[i].get_position());
            if dist < 3. {
                for _ in 0..self.all_the_apples[i].quality {
                    snake.add_segment();
                }
                removed.push(i);
                // raise_max_food(food_factory);
                
                for _ in 0..gen_range(1, self.max_food) {
                    new_food.push(Food::new_random(50., 2));
                }
            }
        }
        for i in removed {
            self.all_the_apples.remove(i);
        }
        for food in new_food {
            self.all_the_apples.push(food);
        }
        self.load_meshes();
    }

    pub fn draw_food(&self) {
        for food in self.all_the_apples.iter() {
            food.draw(Some(&self.models), Some(&self.materials), Some(&self.textures));
        }
    }

    pub fn load_meshes(&mut self) {
        for food in self.all_the_apples.iter_mut() {
            food.load_mesh(&self.models, &self.materials, &self.textures);
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
            mesh: None,
        }
    }

    fn new_random(max_pos: f32, max_quality: u32) -> Self {
        Self {
            position: random_vec3(0., max_pos),
            size: random_vec3(3., 5.),
            quality: gen_range(1, max_quality),
            color: YELLOW,
            repeat: 5,
            mesh: None,
        }
    }

    fn get_quality(&self) -> u32 {
        self.quality
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    pub fn load_mesh(&mut self, models: &Vec<Model>, materials: &Vec<Material>, textures: &HashMap<String, Texture2D>) {
        if self.mesh.is_some() {
            return;
        }
        for model in models.iter() {
            let mat_id = model.mesh.material_id.unwrap_or(0);
            let material = &materials.get(mat_id);
            let material = match material {
                Some(material) => material,
                None => continue,
            };
            let mesh = tobj_model_to_mesh(model, material, textures);
            println!("Texture is here: {}", mesh.texture.is_some());
            self.mesh = Some(mesh);
        }
    }
}

fn tobj_model_to_mesh(model: &Model, material: &Material, textures: &HashMap<String, Texture2D>) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();
    let texture = match &material.normal_texture {
        Some(texture_file_name) => textures.get(texture_file_name),
        None => None,
    };

    let texsize = match texture {
        Some(texture) => vec2(texture.width() as f32, texture.height() as f32),
        None => vec2(1., 1.),
        
    };

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
    
    Mesh { vertices, indices: indices, texture: texture.cloned()}
}

impl Drawable for Food {
    fn get_repeat(&self) -> i32 {
        1
    }

    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn draw_at(&self, position: Vec3, _saturation: f32, models: Option<&Vec<Model>>, materials: Option<&Vec<Material>>, textures: Option<&HashMap<String, Texture2D>>) {
        match &self.mesh {
            Some(mesh) => {
                let mut i = 0;
                let n = 300;
                let vertices: Vec<Vertex> = mesh.vertices.iter().map(|v| Vertex {
                    position: v.position * 100. + position,
                    uv: v.uv,
                    color: [50, 50, 0, 0],
                    normal: v.normal,
                }).collect();

                while (i + 1) * n < mesh.indices.len() {
                    let tmp = Mesh {
                        vertices: vertices.clone(),
                        indices: mesh.indices[i * n..(i + 1) * n].to_vec(),
                        texture: mesh.texture.clone(),
                    };
                    draw_mesh(&tmp);
                    i += 1;
                }
                if mesh.indices.len() % n != 0 {
                    let tmp = Mesh {
                        vertices: vertices,
                        indices: mesh.indices[i * n..].to_vec(),
                        texture: mesh.texture.clone(),
                    };
                    draw_mesh(&tmp);
                }
            }
            None => {
                draw_cube(position, self.size, None, self.color);
            }
        }
    }
}
