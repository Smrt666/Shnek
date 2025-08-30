use image::ImageReader;
use macroquad::color::WHITE;
use macroquad::math::{vec4, Mat4};
use macroquad::models::{draw_mesh, Mesh, Vertex};
use macroquad::prelude::get_internal_gl;
use macroquad::texture::{FilterMode, Texture2D};
use std::path::Path;
use tobj::{load_obj, Material};

pub struct Model3D {
    pub meshes: Vec<Mesh>,
}

impl Model3D {
    pub fn from_file(path: &str) -> Model3D {
        let (mut models, materials) =
            load_obj(path, &tobj::GPU_LOAD_OPTIONS).expect("failed to load model file");
        let materials = materials.expect("failed to load materials file");
        let material_path = Path::new(path).parent().unwrap();

        let mut meshes = Vec::new();
        // Merge meshes with the same texture into one, so fewer calls to GPU are required.
        // Could backfire with large models, but we will not be using those anyway.
        models.sort_by_key(|m| m.mesh.material_id.expect("no material id"));
        for model in models.iter() {
            let mat_id = model.mesh.material_id.unwrap();
            let material = &materials[mat_id];
            let mut mesh = obj_to_mesh(model, load_diffuse(material_path, material));
            if meshes.len() <= mat_id {
                meshes.push(mesh);
            } else {
                let start_index = meshes[mat_id].vertices.len() as u16;
                mesh.indices.iter_mut().for_each(|idx| *idx += start_index);
                meshes[mat_id].vertices.extend(mesh.vertices.iter());
                meshes[mat_id].indices.extend(mesh.indices.iter());
            }
        }
        Model3D { meshes }
    }

    pub fn draw_meshes(&self, model_matrix: Mat4) {
        unsafe {
            let gl = get_internal_gl().quad_gl;
            gl.push_model_matrix(model_matrix);
            for mesh in self.meshes.iter() {
                draw_mesh(mesh);
            }
            gl.pop_model_matrix();
        }
    }
}

pub fn load_diffuse(material_path: &Path, material: &Material) -> Texture2D {
    let texture = material
        .diffuse_texture
        .clone()
        .expect("no diffuse texture specified");
    let file_name = Path::new(material_path).join(Path::new(&texture));
    // There are functions for loading textures directly from file, but we had some problems with that.
    let image = ImageReader::open(file_name)
        .expect("failed to load image file")
        .decode()
        .expect("failed to decode image");

    Texture2D::from_rgba8(
        image.width() as u16,
        image.height() as u16,
        &image.to_rgba8(),
    )
}

pub fn obj_to_mesh(model: &tobj::Model, texture: Texture2D) -> Mesh {
    let mut vertices: Vec<Vertex> = Vec::new();
    let max_index = *model
        .mesh
        .indices
        .iter()
        .max()
        .expect("No mesh indices found");

    for i in 0..=max_index {
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
            _ => vec4(0., 0., 0., 0.),
        };

        let scale = 20.0;
        // I think (not sure, but it looked like that) that color from texture is multiplied by color,
        // so set it to white to get the same color as in texture.
        // Be careful: v coordinate is inverted.
        let mut vertex = Vertex::new(
            x * scale + 50.,
            y * scale + 50.,
            z * scale + 50.,
            *u,
            1. - v,
            WHITE,
        );
        vertex.normal = normal;
        vertices.push(vertex);
    }

    let indices = model
        .mesh
        .indices
        .iter()
        .map(|i| *i as u16)
        .collect::<Vec<u16>>();
    // Minecraft style pixelated textures. The Linear option blurs the texture.
    texture.set_filter(FilterMode::Nearest);
    Mesh {
        vertices,
        indices,
        texture: Some(texture),
    }
}
