extern crate gl;

use std::os::raw::c_void;
use std::path::PathBuf;
use std::vec;
use std::{error::Error, ffi::CString};

use asset_importer::{ImportBuilder, Vector3D, postprocess::PostProcessSteps};
use asset_importer::{Scene, TextureInfo};
use gl::types::{GLboolean, GLsizei, GLsizeiptr, GLuint};

pub struct Model {
  scene: Scene,
  position_mat4: glm::Mat4,
  vertices: Vec<Vector3D>,
  vao: u32,
  vbo: u32,
  texture: u32,
}

impl Model {
  // Initializes a new model, currently only uses the first mesh
  pub fn new(path: PathBuf) -> Result<Self, Box<dyn Error>> {
    let importer = ImportBuilder::new();
    let scene = importer
      .with_post_process(PostProcessSteps::TRIANGULATE)
      .import_file(path)?;
    let mesh = scene.mesh(0).expect("There is no mesh!");
    let vertices = mesh.vertices();
    let model = Model {
      scene: scene,
      position_mat4: glm::identity(),
      vertices: vertices,
      vao: 0,
      vbo: 0,
      texture: 0,
    };
    return Ok(model);
  }
  // Loads the mesh into OpenGL
  pub unsafe fn load(self: &mut Self, program: u32) {
    unsafe {
      let (_, vertices_array, _) = self.vertices.align_to::<u8>();
      gl::GenVertexArrays(1, &mut self.vao);
      gl::BindVertexArray(self.vao);
      gl::GenBuffers(1, &mut self.vbo);
      gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        vertices_array.len() as GLsizeiptr,
        vertices_array.as_ptr() as *const _,
        gl::STATIC_DRAW,
      );
      let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
      gl::EnableVertexAttribArray(pos_attr as GLuint);
      gl::VertexAttribPointer(
        pos_attr as GLuint,
        3,
        gl::FLOAT,
        gl::FALSE as GLboolean,
        (size_of::<Vector3D>()) as i32,
        0 as *const c_void,
      );
      self.load_texture();
    }
  }
  // Draw the thing
  pub unsafe fn draw(self: &mut Self) {
    unsafe {
      gl::DrawArrays(gl::TRIANGLES, 0, (self.vertices.len() * 3) as i32);
    }
  }
  pub unsafe fn load_uniforms(self: &mut Self, program: u32) {
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
  // Loads an image
  unsafe fn load_texture(self: &mut Self) {
    let mut data: Vec<u8> = Vec::new();
    let mut textures: Vec<TextureInfo> = vec![];
    println!("Step 1");
    for mat in self.scene.materials() {
      for i in 0..mat.texture_count(asset_importer::TextureType::Diffuse) {
        if let Some(color_texture) = mat.texture(asset_importer::TextureType::Diffuse, i) {
          textures.push(color_texture);
        };
      }
    }
    println!("Step 2");
    if let Some(texture) = textures.iter().next_back() {
      println!("{:?}", texture);
      if let Ok(img) = image::open("models/sakuya/textures/color.png") {
        let rgba = img.flipv().to_rgba8();
        let (w, h) = rgba.dimensions();
        data.extend_from_slice(&rgba.into_raw());

        unsafe {
          gl::GenTextures(gl::TEXTURE_2D as GLsizei, &mut self.texture);
          gl::ActiveTexture(gl::TEXTURE0);
          gl::BindTexture(gl::TEXTURE_2D, self.texture);
          gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            w as i32,
            h as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const _,
          );
        }
      };
    };
  }
}
