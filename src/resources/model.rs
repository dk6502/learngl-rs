extern crate gl;
extern crate nalgebra_glm as glm;

use std::path::PathBuf;
use std::vec;
use std::{error::Error, ffi::CString};

use crate::resources::texture::Texture;
use asset_importer::{ImportBuilder, postprocess::PostProcessSteps};

use crate::resources::mesh::Mesh;

pub struct Model {
  model_path: PathBuf,
  pub position_mat4: glm::Mat4,
  meshes: Vec<Mesh>,
  textures: Vec<Texture>,
}

impl Model {
  // Initializes a new model, currently only uses the first mesh
  pub fn new(model_path: PathBuf) -> Result<Self, Box<dyn Error>> {
    let importer = ImportBuilder::new();
    let scene = importer
      .with_post_process(PostProcessSteps::TRIANGULATE)
      .import_file(&model_path)?;
    let mut meshes: Vec<Mesh> = vec![];
    let mut textures: Vec<Texture> = vec![];
    for mesh in scene.meshes() {
      let vertices = mesh.vertices();
      let indices: Vec<u32> = mesh
        .faces()
        .flat_map(|f| {
          f.indices()
            .iter()
            .map(|&idx| idx as u32)
            .collect::<Vec<u32>>()
        })
        .collect();
      let texcoords = mesh.texture_coords(0).expect("Should exist");
      let texture = mesh.material_index();
      meshes.push(Mesh::new(
        texture,
        vertices.clone(),
        texcoords.clone(),
        indices,
      ));
    }
    for mat in scene.materials() {
      for i in 0..mat.texture_count(asset_importer::TextureType::Diffuse) {
        textures.push(Texture::new(
          mat
            .texture(asset_importer::TextureType::Diffuse, i)
            .expect("Should work"),
        ));
      }
    }
    let mut mat4 = glm::identity::<f32, 4>();

    mat4 = glm::rotate::<f32>(
      &mat4,
      (270.0 as f32).to_radians(),
      &glm::Vec3::new(1.0, 0.0, 0.0),
    );
    let model = Model {
      model_path: model_path,
      position_mat4: mat4,
      meshes: meshes,
      textures: textures,
    };
    return Ok(model);
  }
  // Loads the mesh into OpenGL
  pub unsafe fn load(self: &mut Self, program: u32) {
    unsafe {
      for mesh in self.meshes.iter_mut() {
        mesh.load(program);
      }
      for texture in self.textures.iter_mut() {
        texture.load(&self.model_path);
      }
    }
  }
  // Draw the thing
  pub unsafe fn draw(self: &mut Self, program: u32) {
    unsafe {
      self.load_uniforms(program);
      gl::UseProgram(program);
      for mesh in self.meshes.iter_mut() {
        self.textures[mesh.texture_id - 1].draw(program);
        mesh.draw();
      }
    }
  }
  // this makes the uniforms
  unsafe fn load_uniforms(self: &mut Self, program: u32) {
    unsafe {
      let model_loc = gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
      gl::UniformMatrix4fv(
        model_loc,
        1,
        gl::FALSE as u8,
        &self.position_mat4 as *const _ as *const _,
      );
    }
  }
}
