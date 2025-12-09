use asset_importer::Vector3D;
use gl::types::{GLboolean, GLuint};
use itertools::interleave;
use std::{ffi::CString, os::raw::c_void};

pub struct Mesh {
  pub texture_id: usize,
  vertices_texcoords: Vec<Vector3D>,
  index_data: Vec<u32>,
  vao: u32,
  vbo: u32,
  ebo: u32,
}

impl Mesh {
  pub fn new(
    texture_id: usize,
    vertices: Vec<Vector3D>,
    texcoords: Vec<Vector3D>,
    indices: Vec<u32>,
  ) -> Self {
    let data = interleave(vertices, texcoords).collect::<Vec<_>>();
    Mesh {
      texture_id: texture_id,
      index_data: indices,
      vertices_texcoords: data,
      ebo: 0,
      vao: 0,
      vbo: 0,
    }
  }
  pub unsafe fn load(self: &mut Self, program: u32) {
    unsafe {
      let (_, data_array, _) = self.vertices_texcoords.align_to::<u8>();
      gl::GenVertexArrays(1, &mut self.vao);
      gl::BindVertexArray(self.vao);
      gl::GenBuffers(1, &mut self.vbo);
      gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
      gl::BufferData(
        gl::ARRAY_BUFFER,
        (data_array.len() * std::mem::size_of::<u8>()) as isize,
        data_array.as_ptr() as *const _,
        gl::STATIC_DRAW,
      );
      gl::GenBuffers(1, &mut self.ebo);
      gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
      gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (self.index_data.len() * std::mem::size_of::<u32>()) as isize,
        self.index_data.as_ptr() as *const _,
        gl::STATIC_DRAW,
      );
      let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
      gl::EnableVertexAttribArray(pos_attr as GLuint);
      gl::VertexAttribPointer(
        pos_attr as GLuint,
        3,
        gl::FLOAT,
        gl::FALSE as GLboolean,
        (size_of::<Vector3D>() * 2) as i32,
        0 as *const c_void,
      );
      let texcoords_attr =
        gl::GetAttribLocation(program, CString::new("texcoords").unwrap().as_ptr());
      gl::EnableVertexAttribArray(texcoords_attr as GLuint);
      gl::VertexAttribPointer(
        texcoords_attr as GLuint,
        3,
        gl::FLOAT,
        gl::FALSE as GLboolean,
        (size_of::<Vector3D>() * 2) as i32,
        size_of::<Vector3D>() as *const c_void,
      );
    }
  }

  pub unsafe fn draw(self: &mut Self) {
    unsafe {
      gl::BindVertexArray(self.vao);
      gl::DrawElements(
        gl::TRIANGLES,
        (self.vertices_texcoords.len() * 3) as i32,
        gl::UNSIGNED_INT,
        0 as *const _,
      );
    }
  }
}
