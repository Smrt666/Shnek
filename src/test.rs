use std::env::current_dir;
use image::{GenericImageView, ImageReader};
use macroquad::color::RED;
use macroquad::miniquad::gl;
use macroquad::models::{draw_cube, draw_mesh, Mesh, Vertex};
use macroquad::prelude::{get_internal_gl, vec2, vec3, vec4, Color, DrawMode, Texture2D, Vec3};
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
            position: Vec3::ZERO,
            // mesh: tobj_model_to_mesh(model, texture),
            mesh: make_cube(vec3(0.0, 0.0, 0.0), vec3(10.0, 10.0, 10.0), texture, RED)
        }
    }

    pub unsafe fn draw(&mut self) {
        let _ = self.mesh.vertices.iter_mut().map(|v| v.position += self.position);
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

unsafe fn make_cube(position: Vec3, size: Vec3, texture: Texture2D, color: Color) -> Mesh {
    let (x, y, z) = (position.x, position.y, position.z);
    let (width, height, length) = (size.x, size.y, size.z);

    // Front face
    let bl_pos = vec3(x - width / 2., y - height / 2., z + length / 2.);
    let bl_uv = vec2(0., 0.);
    let br_pos = vec3(x + width / 2., y - height / 2., z + length / 2.);
    let br_uv = vec2(1., 0.);

    let tr_pos = vec3(x + width / 2., y + height / 2., z + length / 2.);
    let tr_uv = vec2(1., 1.);

    let tl_pos = vec3(x - width / 2., y + height / 2., z + length / 2.);
    let tl_uv = vec2(0., 1.);

    let mut vertices = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    vertices.push(Vertex::new2(bl_pos, bl_uv, color));
    vertices.push(Vertex::new2(br_pos, br_uv, color));
    vertices.push(Vertex::new2(tr_pos, tr_uv, color));
    vertices.push(Vertex::new2(tl_pos, tl_uv, color));
    indices.extend([0,1,2,0,2,3]);

    // Back face
    let bl_pos = vec3(x - width / 2., y - height / 2., z - length / 2.);
    let bl_uv = vec2(0., 0.);
    let br_pos = vec3(x + width / 2., y - height / 2., z - length / 2.);
    let br_uv = vec2(1., 0.);

    let tr_pos = vec3(x + width / 2., y + height / 2., z - length / 2.);
    let tr_uv = vec2(1., 1.);

    let tl_pos = vec3(x - width / 2., y + height / 2., z - length / 2.);
    let tl_uv = vec2(0., 1.);

    vertices.push(Vertex::new2(bl_pos, bl_uv, color));
    vertices.push(Vertex::new2(br_pos, br_uv, color));
    vertices.push(Vertex::new2(tr_pos, tr_uv, color));
    vertices.push(Vertex::new2(tl_pos, tl_uv, color));
    indices.extend([4,5,6,4,6,7]);

    // Top face
    let bl_pos = vec3(x - width / 2., y + height / 2., z - length / 2.);
    let bl_uv = vec2(0., 1.);
    let br_pos = vec3(x - width / 2., y + height / 2., z + length / 2.);
    let br_uv = vec2(0., 0.);

    let tr_pos = vec3(x + width / 2., y + height / 2., z + length / 2.);
    let tr_uv = vec2(1., 0.);

    let tl_pos = vec3(x + width / 2., y + height / 2., z - length / 2.);
    let tl_uv = vec2(1., 1.);

    vertices.push(Vertex::new2(bl_pos, bl_uv, color));
    vertices.push(Vertex::new2(br_pos, br_uv, color));
    vertices.push(Vertex::new2(tr_pos, tr_uv, color));
    vertices.push(Vertex::new2(tl_pos, tl_uv, color));
    indices.extend([8,9,10,8,10,11]);

    // Bottom face
    let bl_pos = vec3(x - width / 2., y - height / 2., z - length / 2.);
    let bl_uv = vec2(0., 1.);
    let br_pos = vec3(x - width / 2., y - height / 2., z + length / 2.);
    let br_uv = vec2(0., 0.);

    let tr_pos = vec3(x + width / 2., y - height / 2., z + length / 2.);
    let tr_uv = vec2(1., 0.);

    let tl_pos = vec3(x + width / 2., y - height / 2., z - length / 2.);
    let tl_uv = vec2(1., 1.);

    vertices.push(Vertex::new2(bl_pos, bl_uv, color));
    vertices.push(Vertex::new2(br_pos, br_uv, color));
    vertices.push(Vertex::new2(tr_pos, tr_uv, color));
    vertices.push(Vertex::new2(tl_pos, tl_uv, color));
    indices.extend([12,13,14,12,14,15]);

    // Right face
    let bl_pos = vec3(x + width / 2., y - height / 2., z - length / 2.);
    let bl_uv = vec2(0., 1.);
    let br_pos = vec3(x + width / 2., y + height / 2., z - length / 2.);
    let br_uv = vec2(0., 0.);

    let tr_pos = vec3(x + width / 2., y + height / 2., z + length / 2.);
    let tr_uv = vec2(1., 0.);

    let tl_pos = vec3(x + width / 2., y - height / 2., z + length / 2.);
    let tl_uv = vec2(1., 1.);

    vertices.push(Vertex::new2(bl_pos, bl_uv, color));
    vertices.push(Vertex::new2(br_pos, br_uv, color));
    vertices.push(Vertex::new2(tr_pos, tr_uv, color));
    vertices.push(Vertex::new2(tl_pos, tl_uv, color));
    indices.extend([16,17,18,16,18,19]);

    // Left face
    let bl_pos = vec3(x - width / 2., y - height / 2., z - length / 2.);
    let bl_uv = vec2(0., 1.);
    let br_pos = vec3(x - width / 2., y + height / 2., z - length / 2.);
    let br_uv = vec2(0., 0.);

    let tr_pos = vec3(x - width / 2., y + height / 2., z + length / 2.);
    let tr_uv = vec2(1., 0.);

    let tl_pos = vec3(x - width / 2., y - height / 2., z + length / 2.);
    let tl_uv = vec2(1., 1.);

    vertices.push(Vertex::new2(bl_pos, bl_uv, color));
    vertices.push(Vertex::new2(br_pos, br_uv, color));
    vertices.push(Vertex::new2(tr_pos, tr_uv, color));
    vertices.push(Vertex::new2(tl_pos, tl_uv, color));
    indices.extend([20,21,22,20,22,23]);

    Mesh {
        vertices, indices, texture: Some(texture)
    }
}
fn tobj_model_to_mesh(model: Model, texture: Texture2D) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    let texsize = vec2(texture.width() as f32, texture.height() as f32);

    for i in model.mesh.indices.iter() {
        let i = *i as usize;
        let x = model.mesh.positions[i * 3];
        let y = model.mesh.positions[i * 3 + 1];
        let z = model.mesh.positions[i * 3 + 2];

        // Could not exist
        let u = model.mesh.texcoords.get(i * 2).unwrap() * 1.0; // texsize.x;
        let v = model.mesh.texcoords.get(i * 2 + 1).unwrap() * 1.0; // texsize.y;
        let uv = vec2(u, v);

        // Could not exist (normals are not used by default macroquad)
        let nx = model.mesh.normals.get(i * 3);
        let ny = model.mesh.normals.get(i * 3 + 1);
        let nz = model.mesh.normals.get(i * 3 + 2);
        let normal = match (nx, ny, nz) {
            (Some(nx), Some(ny), Some(nz)) => vec4(*nx, *ny, *nz, 0.0),
            _ => panic!("nope2"),
        };

        let scale = 100.0;
        vertices.push(Vertex {
            position: vec3(x * scale, y * scale, z * scale),
            uv,
            color: [100, 0, 0, 0],
            normal,
        });
        indices.push(i as u16);
    }

    println!("Vertices: {:?}", vertices);
    println!("Indices: {:?}", indices);

    Mesh { vertices, indices, texture: Some(texture)}
}