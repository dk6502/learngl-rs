use asset_importer::Vector3D;
use gl::types::{GLboolean, GLsizeiptr, GLuint};
use itertools::interleave;
use std::{ffi::CString, os::raw::c_void};

pub struct Mesh {
  vertices_texcoords: Vec<Vector3D>,
  vao: u32,
  vbo: u32,
}

impl Mesh {
  pub fn new(vertices: Vec<Vector3D>, texcoords: Vec<Vector3D>) -> Self {
    let data = interleave(vertices, texcoords).collect::<Vec<_>>();
    Mesh {
      vertices_texcoords: data,
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
        data_array.len() as GLsizeiptr,
        data_array.as_ptr() as *const _,
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
      gl::DrawArrays(gl::TRIANGLES, 0, (self.vertices_texcoords.len() * 3) as i32);
    }
  }
}
