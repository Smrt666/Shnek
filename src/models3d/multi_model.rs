use macroquad::math::{vec3, Mat4, Vec3, Vec4};
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::{get_internal_gl, DrawMode};
use macroquad::texture::Texture2D;
use crate::models3d::Model3D;
use crate::draw_utils::SPACE_SIZE;


struct PartialMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    texture_id: usize,
}

pub struct MultiModel<'a> {
    base_model: &'a Model3D,
    combined_model: Vec<PartialMesh>,
    textures: Vec<&'a Texture2D>,
    repeat: i32,
}

impl<'a> MultiModel<'a> {
    pub fn new(base_model: &'a Model3D, transform: &Mat4, repeat: i32) -> MultiModel<'a> {
        let mut combined_model = Vec::new();
        let mut textures = Vec::new();
        for (i, mesh) in base_model.meshes.iter().enumerate() {
            combined_model.push(Self::repeat_mesh(mesh, transform, repeat, i));
            textures.push(mesh.texture.as_ref().unwrap());
        }
        MultiModel { base_model, combined_model, textures, repeat }
    }

    pub fn new2(base_model: &'a Model3D, origin: Vec3, repeat: i32) -> MultiModel<'a> {
        let transform = Mat4::from_translation(origin);
        MultiModel::new(base_model, &transform, repeat)
    }

    fn repeat_mesh(mesh: &Mesh, transform: &Mat4, repeat: i32, texture_id: usize) -> PartialMesh {
        let base_vertices: Vec<Vertex> = mesh.vertices.iter().map(
            |v| Vertex {
                position: transform.mul_vec4(Vec4::from((v.position, 1.0))).truncate(),
                uv: v.uv,
                color: v.color,
                normal: v.normal,
            }
        ).collect();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut index_offset = 0;
        for i in -repeat..=repeat {
            for j in -repeat..=repeat {
                for k in -repeat..=repeat {
                    let position = vec3(
                        i as f32 * SPACE_SIZE,
                        j as f32 * SPACE_SIZE,
                        k as f32 * SPACE_SIZE,
                    );
                    for vertex in base_vertices.iter() {
                        let mut moved_vertex = vertex.clone();
                        moved_vertex.position += position;
                        vertices.push(moved_vertex);
                    }
                    for index in mesh.indices.iter() {
                        indices.push(index + index_offset);
                    }
                    index_offset += mesh.indices.len() as u16;
                }
            }
        }
        PartialMesh { vertices, indices, texture_id}
    }

    pub fn add_transformed(&mut self, transform: &Mat4) {
         for (i, mesh) in self.base_model.meshes.iter().enumerate() {
            self.combined_model.push(Self::repeat_mesh(mesh, transform, self.repeat, i));
            self.textures.push(mesh.texture.as_ref().unwrap());
        }
    }

    pub fn draw(&self) {
        let gl;
        unsafe {
            gl = get_internal_gl().quad_gl;
        }
        gl.draw_mode(DrawMode::Triangles);
        for mesh in self.combined_model.iter() {
            gl.texture(Some(self.textures[mesh.texture_id]));  // This can be sometimes avoided texture_ids repeat
            gl.geometry(&mesh.vertices, &mesh.indices);
        }
    }
}
