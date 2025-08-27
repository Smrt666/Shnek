use std::env::current_dir;
use image::{GenericImageView, ImageReader};
use macroquad::color::{RED, WHITE};
use macroquad::miniquad::gl;
use macroquad::models::{draw_cube, draw_mesh, Mesh, Vertex};
use macroquad::prelude::{get_internal_gl, vec2, vec3, vec4, Color, DrawMode, Texture2D, Vec3};
use macroquad::prelude::FilterMode::Nearest;
use tobj::{Material, Model};

pub struct TestObject {
    position: Vec3,
    mesh: Mesh
}

impl TestObject {
    pub unsafe fn new(model: Model, material: Material) -> TestObject {
        let texture = material.diffuse_texture.unwrap();

        let filename = &format!("assets/head_test/{}", &texture.replace("\\\\", "/"));
        println!("Loading texture: {}", filename);
        println!("Current dir: {:?}", current_dir().unwrap());
        println!("file exists: {:?}", std::fs::exists(filename));
        let image = ImageReader::open(filename).unwrap().decode().unwrap();
        let (width, height) = image.dimensions();
        let bytes = image.to_rgba8();
        let texture = Texture2D::from_rgba8(width as u16, height as u16, &bytes);
        Self {
            position: vec3(50.0, 50.0, 50.0),
            mesh: tobj_model_to_mesh(model, texture),
            // mesh: make_cube(vec3(50.0, 50.0, 50.0), vec3(10.0, 10.0, 10.0), texture, WHITE)
        }
    }

    pub unsafe fn draw(&mut self) {
        // let _ = self.mesh.vertices.iter_mut().map(|v| v.position += self.position);
        // self.position += Vec3::new(1.0,0.0,0.0);
        // let gl = get_internal_gl().quad_gl;
        // gl.draw_mode(DrawMode::Triangles);
        // gl.texture(self.mesh.texture.as_ref());
        // gl.geometry(&self.mesh.vertices, &self.mesh.indices);
        draw_mesh(&self.mesh);
        // draw_cube(self.position, vec3(10.0, 10.0, 10.0), self.mesh.texture.as_ref(), RED);
        // make_cube(self.position, vec3(10.0, 10.0, 10.0), self.mesh.texture.as_ref(), RED);
    }
}

fn tobj_model_to_mesh(model: Model, texture: Texture2D) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    // let max_index = *model.mesh.indices.iter().max().expect("No mesh indices found");
    println!("Loading model with {} triangles", model.mesh.indices.len() / 3);

    for i in 0..model.mesh.positions.len() / 3 {
        let i = i as usize;
        let x = model.mesh.positions[i * 3];
        let y = model.mesh.positions[i * 3 + 1];
        let z = model.mesh.positions[i * 3 + 2];

        let u = model.mesh.texcoords.get(i * 2).unwrap();
        let v = model.mesh.texcoords.get(i * 2 + 1).unwrap();

        // Is allowed to not exist (normals are not used by default macroquad)
        let nx = model.mesh.normals.get(i * 3);
        let ny = model.mesh.normals.get(i * 3 + 1);
        let nz = model.mesh.normals.get(i * 3 + 2);
        let normal = match (nx, ny, nz) {
            (Some(nx), Some(ny), Some(nz)) => vec4(*nx, *ny, *nz, 0.0),
            _ => panic!("Missing normals in model"),
        };

        let scale = 20.0;
        vertices.push(Vertex::new(x * scale + 50., y * scale + 50., z * scale + 50., *u, *v, WHITE));
    }

    let indices = model.mesh.indices.iter().map(|i| *i as u16).collect::<Vec<u16>>();
    println!("Vertices: {:?}", vertices);
    println!("Indices: {:?}", indices);
    texture.set_filter(Nearest);
    Mesh { vertices, indices, texture: Some(texture) }
}