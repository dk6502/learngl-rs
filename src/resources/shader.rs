use gl::types::*;
use std::error::Error;
use std::ffi::CString;
use std::{fs, ptr};

pub struct Shader {
  pub id: u32,
}

impl Shader {
  // Create a new shader object
  pub fn new(vs_path: &'static str, fs_path: &'static str) -> Result<Self, Box<dyn Error>> {
    let vs_src = fs::read_to_string(vs_path)?;
    let fs_src = fs::read_to_string(fs_path)?;
    let vs = compile_shader(&vs_src, gl::VERTEX_SHADER);
    let fs = compile_shader(&fs_src, gl::FRAGMENT_SHADER);
    Ok(Shader {
      id: link_program(vs, fs),
    })
  }
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
  let shader;
  unsafe {
    shader = gl::CreateShader(ty);
    let c_str = CString::new(src.as_bytes()).unwrap();
    gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
    gl::CompileShader(shader);
    let mut status = gl::FALSE as GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    if status != (gl::TRUE as GLint) {
      let mut len = 0;
      let mut outlen = 0;
      gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
      let mut buf = Vec::with_capacity(len as usize);
      gl::GetShaderInfoLog(shader, len, &mut outlen, buf.as_mut_ptr() as *mut GLchar);
      buf.set_len(outlen as usize);

      panic!(
        "{}",
        str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
      );
    }
  };
  shader
}

fn link_program(fs: GLuint, vs: GLuint) -> GLuint {
  unsafe {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vs);
    gl::AttachShader(program, fs);
    gl::LinkProgram(program);
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    if status != (gl::TRUE as GLint) {
      println!("Shader didn't link")
    }
    program
  }
}
