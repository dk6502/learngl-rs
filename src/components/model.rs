extern crate gl;
extern crate nalgebra_glm as glm;

use std::ffi::CString;
use std::path::PathBuf;
use std::vec;

use crate::resources::texture::Texture;
use asset_importer::Vector3D;
use asset_importer::{ImportBuilder, postprocess::PostProcessSteps};
use nalgebra_glm::Vec3;

use crate::resources::mesh::Mesh;

pub struct Model {
  model_path: PathBuf,
  position_mat4: glm::Mat4,
  meshes: Vec<Mesh>,
  textures: Option<Vec<Texture>>,
}

impl Model {
  // Initializes a new model, currently only uses the first mesh
  pub fn new(model_path: PathBuf) -> Self {
    let mut has_texture = true;
    #[allow(clippy::needless_late_init)]
    let textures: Option<Vec<Texture>>;
    let importer = ImportBuilder::new();
    let scene = importer
      .with_post_process(PostProcessSteps::TRIANGULATE)
      .import_file(&model_path)
      .expect("Could not find the model path!");
    let mut meshes: Vec<Mesh> = vec![];
    let mut textures_vec: Vec<Texture> = vec![];
    for mat in scene.materials() {
      for i in 0..mat.texture_count(asset_importer::TextureType::Diffuse) {
        textures_vec.push(Texture::new(
          mat
            .texture(asset_importer::TextureType::Diffuse, i)
            .expect("Should work"),
        ));
      }
    }
    for mesh in scene.meshes() {
      let vertices = mesh.vertices();
      let indices: Vec<u32> = mesh.faces().flat_map(|f| f.indices().to_vec()).collect();
      if let Some(texcoords) = mesh.texture_coords(0) {
        let texture = mesh.material_index();

        meshes.push(Mesh::new(
          texture,
          vertices.clone(),
          texcoords.clone(),
          indices,
        ));
      } else {
        let texture = 1;
        meshes.push(Mesh::new(
          texture,
          vertices.clone(),
          vec![Vector3D::new(0.0, 0.0, 0.0); vertices.len()],
          indices,
        ));
        has_texture = false;
      }
    }
    if has_texture {
      textures = Some(textures_vec)
    } else {
      textures = None;
    }
    let mat4 = glm::identity::<f32, 4>();

    Model {
      model_path,
      position_mat4: mat4,
      meshes,
      textures,
    }
  }

  // rotates the model
  pub fn with_rotate(self, radians: f32, rot: &Vec3) -> Self {
    let mut model = self;
    model.position_mat4 = glm::rotate(&model.position_mat4, radians, rot);
    model
  }

  // scales the model
  pub fn with_scale(self, scale: &Vec3) -> Self {
    let mut model = self;
    model.position_mat4 = glm::scale(&model.position_mat4, scale);
    model
  }

  // translates the model
  pub fn with_translate(self, translate: &Vec3) -> Self {
    let mut model = self;
    model.position_mat4 = glm::translate(&model.position_mat4, translate);
    model
  }
  // Loads the mesh into OpenGL
  pub unsafe fn load(&mut self, program: u32) {
    unsafe {
      for mesh in self.meshes.iter_mut() {
        mesh.load(program);
      }
      if let Some(textures) = &mut self.textures {
        for texture in textures.iter_mut() {
          texture.load(&self.model_path);
        }
      }
    }
  }
  // Draw the thing
  pub unsafe fn draw(&mut self, program: u32) {
    unsafe {
      self.load_uniforms(program);
      gl::UseProgram(program);
      for mesh in self.meshes.iter_mut() {
        if let Some(textures) = &mut self.textures {
          textures[mesh.texture_id - 1].draw(program);
        }
        mesh.draw();
      }
    }
  }
  // this makes the uniforms
  unsafe fn load_uniforms(&mut self, program: u32) {
    unsafe {
      let model_loc = gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
      gl::UniformMatrix4fv(
        model_loc,
        1,
        gl::FALSE,
        &self.position_mat4 as *const _ as *const _,
      );
    }
  }
}
