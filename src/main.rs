extern crate gl;
extern crate nalgebra_glm as glm;

use gl::{DEPTH_TEST, types::*};
use glm::{Vec3, vec3};
use gltf::Gltf;
use obj::{Obj, load_obj};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{ffi::CString, fs::File, io::BufReader, os::raw::c_void, ptr, str};

static VS_SRC: &str = include_str!("v.glsl");
static FS_SRC: &str = include_str!("f.glsl");

#[repr(C)]
struct Vertex {
  x: f32,
  y: f32,
  z: f32,
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
  let (models, _) =
    tobj::load_obj(&obj_file, &tobj::LoadOptions::default()).expect("Failed to load the .obj");
  let model = models.iter().next().expect("Failed to extract mesh");

  let mesh = model.mesh.clone();
  let mut vertices_vec: Vec<Vertex> = vec![];
  let indices_vec = mesh.indices;

  for i in 0..(mesh.positions.len() / 3) {
    vertices_vec.push(Vertex {
      x: mesh.positions[i],
      y: mesh.positions[i + 1],
      z: mesh.positions[i + 2],
    });
  }

  // initialize sdl2
  let sdl = sdl2::init().unwrap();
  let video_subsystem = sdl.video().unwrap();
  let window = video_subsystem
    .window("opengl", 800, 600)
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
  let camera = glm::look_at_rh(&vec3(10.0, 10.0, 10.0), &vec3(0.0, 0.0, 0.0), &Vec3::y());
  let model = glm::identity::<f32, 4>();
  let proj = glm::perspective::<f32>(1.0, glm::half_pi::<f32>() * 0.8, 0.1, 100.0);

  let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
  let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
  let program = link_program(fs, vs);

  // these ints represent the buffers in OpenGL
  let mut ebo = 0;
  let mut vao = 0;
  let mut vbo = 0;

  unsafe {
    gl::Enable(DEPTH_TEST);
    gl::GenVertexArrays(1, &mut vao);
    gl::BindVertexArray(vao);
    gl::GenBuffers(1, &mut vbo);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
      gl::ARRAY_BUFFER,
      size_of_val(&vertices_vec) as GLsizeiptr,
      vertices_vec.as_ptr().cast(),
      gl::STATIC_DRAW,
    );
    gl::GenBuffers(1, &mut ebo);
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
      gl::ELEMENT_ARRAY_BUFFER,
      size_of_val(&indices_vec) as GLsizeiptr,
      indices_vec.as_ptr().cast(),
      gl::STATIC_DRAW,
    );
    gl::UseProgram(program);
    gl::BindFragDataLocation(program, 0, CString::new("out_color").unwrap().as_ptr());
    let pos_attr = gl::GetAttribLocation(program, CString::new("position").unwrap().as_ptr());
    gl::EnableVertexAttribArray(pos_attr as GLuint);
    gl::VertexAttribPointer(
      pos_attr as GLuint,
      3,
      gl::FLOAT,
      gl::FALSE as GLboolean,
      size_of::<Vertex>() as i32,
      0 as *const c_void,
    );
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

      let model_loc = gl::GetUniformLocation(program, CString::new("model").unwrap().as_ptr());
      gl::UniformMatrix4fv(
        model_loc,
        1,
        gl::FALSE as u8,
        &model as *const _ as *const _,
      );
      gl::ClearColor(0.0, 0.0, 1.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      gl::UseProgram(program);

      gl::DrawArrays(gl::ARRAY_BUFFER, 0, vertices_vec.len() as i32);
    }
    window.gl_swap_window();
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit { .. }
        | Event::KeyDown {
          keycode: Some(Keycode::Escape),
          ..
        } => break 'running,
        _ => {}
      }
    }
  }
}
