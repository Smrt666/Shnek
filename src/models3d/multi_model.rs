use macroquad::math::{vec3, Vec3};
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::{get_internal_gl, DrawMode, Mat3};
use macroquad::texture::Texture2D;
use crate::models3d::Model3D;
use crate::draw_utils::{Drawable, SPACE_SIZE};


struct PartialMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    texture_id: usize,
}

pub struct MultiModel<'a> {
    combined_model: Vec<PartialMesh>,
    textures: Vec<&'a Texture2D>,
    origin: Vec3,
}

impl<'a> MultiModel<'a> {
    pub fn new(base_model: &'a Model3D, origin: Vec3, repeat: i32) -> MultiModel<'a> {
        let mut combined_model = Vec::new();
        let mut textures = Vec::new();
        for (i, mesh) in base_model.meshes.iter().enumerate() {
            combined_model.push(Self::repeat_mesh(mesh, origin, repeat, i));
            textures.push(mesh.texture.as_ref().unwrap());
        }
        MultiModel { origin, combined_model, textures }
    }

    fn repeat_mesh(mesh: &Mesh, origin: Vec3, repeat: i32, texture_id: usize) -> PartialMesh {
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
                    let position = position + origin;
                    for vertex in mesh.vertices.iter() {
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

    pub fn from_transformed(base_model: &'a Model3D, transform: Mat3, origin: Vec3, repeat: i32) -> MultiModel<'a> {
        let mut multi_model = MultiModel::new(base_model, origin, repeat);
        multi_model.combined_model.iter_mut().for_each(
            |mesh|
                mesh.vertices.iter_mut().for_each(
                    |vertex| vertex.position = transform.mul_vec3(vertex.position)
                )
        );
        multi_model
    }
}

impl<'a> Drawable for MultiModel<'a> {
    fn get_repeat(&self) -> i32 { 0 }
    fn get_position(&self) -> Vec3 { self.origin }
    fn draw_at(&self, _position: Vec3, _saturation: f32) {
        unsafe {
            let gl = get_internal_gl().quad_gl;
            gl.draw_mode(DrawMode::Triangles);
            for mesh in self.combined_model.iter() {
                gl.texture(Some(self.textures[mesh.texture_id]));
                gl.geometry(&mesh.vertices, &mesh.indices);
            }
        }
    }
}

