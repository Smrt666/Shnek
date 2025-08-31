use crate::draw_utils::SPACE_SIZE;
use crate::models3d::Model3D;
use macroquad::math::{vec3, Mat4, Vec4};
use macroquad::models::{Mesh, Vertex};
use macroquad::prelude::{get_internal_gl, DrawMode};
use macroquad::texture::Texture2D;
use std::collections::HashMap;

struct PartialMesh {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    texture_id: usize,
}

/** The same models will be drawn multiple times at different
positions. This struct combines meshes so less data has to be sent
to GPU. This is used instead of instancing which is not supported by macroquad.
*/
pub struct MultiModel<'a> {
    base_model: &'a Model3D,
    base_transform: Mat4,
    combined_model: Vec<PartialMesh>,
    textures: Vec<&'a Texture2D>,
    repeat: i32,
    add_transforms: HashMap<usize, Mat4>,
}

impl<'a> MultiModel<'a> {
    pub fn new(base_model: &'a Model3D, repeat: i32) -> MultiModel<'a> {
        let combined_model: Vec<PartialMesh> = Vec::new();
        let mut textures = Vec::new();
        for mesh in &base_model.meshes {
            textures.push(mesh.texture.as_ref().unwrap());
        }
        MultiModel {
            base_model,
            base_transform: Mat4::IDENTITY,
            combined_model,
            textures,
            repeat,
            add_transforms: HashMap::new(),
        }
    }

    fn repeat_mesh(
        mesh: &Mesh,
        transform: &Mat4,
        repeat: i32,
        texture_id: usize,
    ) -> Vec<PartialMesh> {
        let base_vertices: Vec<Vertex> = mesh
            .vertices
            .iter()
            .map(|v| Vertex {
                position: transform.mul_vec4(Vec4::from((v.position, 1.0))).truncate(),
                uv: v.uv,
                color: v.color,
                normal: v.normal,
            })
            .collect();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        let mut index_offset = 0;
        let mut result = Vec::new();
        for i in -repeat..=repeat {
            for j in -repeat..=repeat {
                for k in -repeat..=repeat {
                    let position = vec3(
                        i as f32 * SPACE_SIZE,
                        j as f32 * SPACE_SIZE,
                        k as f32 * SPACE_SIZE,
                    );
                    // There is a limit to geometry call size
                    if indices.len() + mesh.indices.len() > 5000 {
                        result.push(PartialMesh {
                            vertices,
                            indices,
                            texture_id,
                        });
                        indices = Vec::new();
                        vertices = Vec::new();
                        index_offset = 0;
                    }
                    for vertex in base_vertices.iter() {
                        let mut moved_vertex = *vertex;
                        moved_vertex.position += position;
                        vertices.push(moved_vertex);
                    }
                    for index in mesh.indices.iter() {
                        indices.push(index + index_offset);
                    }
                    index_offset += mesh.vertices.len() as u16;
                }
            }
        }
        if !indices.is_empty() {
            result.push(PartialMesh {
                vertices,
                indices,
                texture_id,
            });
        }
        result
    }

    pub fn add_transformed(&mut self, transform: &Mat4, id: usize) {
        self.add_transforms.insert(id, *transform);
        for (i, mesh) in self.base_model.meshes.iter().enumerate() {
            self.combined_model
                .extend(Self::repeat_mesh(mesh, transform, self.repeat, i));
        }
    }

    /** Lazy removal. Meshes are updated when base_transform is called.
     */
    pub fn remove_transformed(&mut self, id: usize) {
        self.add_transforms.remove(&id);
    }

    pub fn draw(&self) {
        let gl;
        unsafe {
            gl = get_internal_gl().quad_gl;
        }
        gl.draw_mode(DrawMode::Triangles);

        // Sort by texture, so we don't sent too many updates to GPU
        let mut draw_order: Vec<usize> = (0..self.combined_model.len()).collect();
        draw_order.sort_by_key(|&i| {
            self.textures[self.combined_model[i].texture_id] as *const Texture2D as usize
        });
        let mut prev_id: usize = 0;
        for i in draw_order {
            let mesh = self.combined_model.get(i).unwrap();
            let texture = self.textures[mesh.texture_id];
            let texture_ptr = texture as *const Texture2D as usize;
            if prev_id != texture_ptr {
                gl.texture(Some(texture));
                prev_id = texture_ptr;
            }
            gl.geometry(&mesh.vertices, &mesh.indices);
        }
    }

    pub fn base_transform(&mut self, transform: Mat4) {
        self.base_transform = transform;
        self.combined_model.clear();
        for (_, add_transform) in self.add_transforms.clone() {
            for (i, mesh) in self.base_model.meshes.iter().enumerate() {
                self.combined_model.extend(Self::repeat_mesh(
                    mesh,
                    &add_transform.mul_mat4(&transform),
                    self.repeat,
                    i,
                ));
            }
        }
    }

    pub fn refresh_transformed(&mut self) {
        self.base_transform(self.base_transform);
    }
}
