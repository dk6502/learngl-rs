extern crate gl;
extern crate nalgebra_glm as glm;

use gl::types::*;
use glm::{Vec3, vec3};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{ffi::CString, path::PathBuf, ptr, str};

use crate::model::Model;

mod model;

static VS_SRC: &str = include_str!("v.glsl");
static FS_SRC: &str = include_str!("f.glsl");

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
      println!("shader not valid");
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

fn main() {
  // open the WaveFront .obj file and make it into an OpenGL-compatible array of vertices and indices
  let obj_file = std::env::args()
    .skip(1)
    .next()
    .expect("An obj file is required!");

  // initialize sdl2
  let sdl = sdl2::init().unwrap();
  let video_subsystem = sdl.video().unwrap();
  let window = video_subsystem
    .window("opengl", 600, 600)
    .opengl()
    .position_centered()
    .build()
    .unwrap();

  let gl_attr = video_subsystem.gl_attr();
  gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
  gl_attr.set_context_version(3, 3);
  gl_attr.set_context_flags().debug().set();

  let Ok(context) = window.gl_create_context() else {
    std::println!("Could not create openGL context");
    return;
  };
  let _ = window.gl_make_current(&context);

  gl::load_with(|symbol| video_subsystem.gl_get_proc_address(symbol) as *const _);

  // these are the variables for the 3d camera
  let mut camera = glm::look_at_rh(&vec3(0.0, 0.0, 10.0), &vec3(0.0, 0.0, 0.0), &Vec3::y());
  let proj = glm::perspective::<f32>(1.0, glm::half_pi::<f32>() * 0.8, 0.1, 1000.0);

  let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
  let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
  let program = link_program(fs, vs);

  let mut model = Model::new(PathBuf::from(obj_file)).expect("Should work!");

  unsafe {
    gl::Enable(gl::DEPTH_TEST);
    gl::UseProgram(program);
    gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
    model.load(program);
    gl::Viewport(0, 0, 600, 600);
  }
  let mut event_pump = sdl.event_pump().unwrap();
  'running: loop {
    unsafe {
      let view_loc = gl::GetUniformLocation(program, CString::new("view").unwrap().as_ptr());
      gl::UniformMatrix4fv(
        view_loc,
        1,
        gl::FALSE as u8,
        &camera as *const _ as *const _,
      );
      let proj_loc = gl::GetUniformLocation(program, CString::new("proj").unwrap().as_ptr());
      gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE as u8, &proj as *const _ as *const _);

      model.load_uniforms(program);
      gl::ClearColor(0.0, 0.0, 0.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      gl::UseProgram(program);
      model.draw();
    }
    window.gl_swap_window();
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        Event::KeyDown {
          keycode: Some(Keycode::D),
          ..
        } => camera = glm::rotate_y(&camera, 0.0174532925),
        Event::KeyDown {
          keycode: Some(Keycode::A),
          ..
        } => camera = glm::rotate_y(&camera, -0.0174532925),
        _ => {}
      }
    }
  }
}
