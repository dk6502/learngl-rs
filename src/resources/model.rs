extern crate gl;

use std::path::PathBuf;
use std::vec;
use std::{error::Error, ffi::CString};

use crate::resources::texture::Texture;
use asset_importer::{ImportBuilder, Vector3D, postprocess::PostProcessSteps};

use crate::resources::mesh::Mesh;

pub struct Model {
  position_mat4: glm::Mat4,
  meshes: Vec<Mesh>,
  textures: Vec<Texture>,
}

impl Model {
  // Initializes a new model, currently only uses the first mesh
  pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
    let importer = ImportBuilder::new();
    let scene = importer
      .with_post_process(PostProcessSteps::TRIANGULATE | PostProcessSteps::FIND_INVALID_DATA)
      .import_file(&path)?;
    let mut meshes: Vec<Mesh> = vec![];
    let mut textures: Vec<Texture> = vec![];
    for mesh in scene.meshes() {
      let vertices = mesh.vertices();
      if let Some(texcoords) = mesh.texture_coords(0) {
        meshes.push(Mesh::new(vertices, texcoords));
      } else {
        meshes.push(Mesh::new(
          vertices.clone(),
          vec![Vector3D::new(0.0, 0.0, 0.0); vertices.len()],
        ));
      };
    }
    println!("{:?}", scene.has_textures());
    for texture in scene.textures() {
      textures.push(Texture::new(texture));
    }
    let model = Model {
      position_mat4: glm::identity(),
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
        texture.load();
      }
    }
  }
  // Draw the thing
  pub unsafe fn draw(self: &mut Self) {
    unsafe {
      for mesh in self.meshes.iter_mut() {
        mesh.draw();
      }
    }
  }
  // this makes the uniforms
  pub unsafe fn load_uniforms(self: &mut Self, program: u32) {
    unsafe {
      let model_loc = gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
      gl::UniformMatrix4fv(
        model_loc,
        1,
        gl::FALSE as u8,
        &self.position_mat4 as *const _ as *const _,
      );
      let texture_loc = gl::GetUniformLocation(program, CString::new("texture0").unwrap().as_ptr());
      gl::Uniform1i(texture_loc, 0);
    }
  }
}
