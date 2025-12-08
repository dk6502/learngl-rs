extern crate gl;
extern crate nalgebra_glm as glm;

use glm::{Vec3, vec3};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::{ffi::CString, path::PathBuf};

use crate::resources::{model::Model, shader::Shader};

mod resources;

fn main() {
  // open the WaveFront .obj file and make it into an OpenGL-compatible array of vertices and indices
  let obj_file = PathBuf::from("models/de_dust2/InuSakuyaS.obj");

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
  gl_attr.set_context_version(4, 5);
  gl_attr.set_context_flags().debug().set();

  let Ok(context) = window.gl_create_context() else {
    std::println!("Could not create openGL context");
    return;
  };
  let _ = window.gl_make_current(&context);

  gl::load_with(|symbol| video_subsystem.gl_get_proc_address(symbol) as *const _);

  // these are the variables for the 3d camera
  let mut camera = glm::look_at_rh(&vec3(0.0, 0.0, 40.0), &vec3(0.0, 0.0, 0.0), &Vec3::y());
  camera = glm::translate(&camera, &vec3(0.0, -10.0, 0.0));
  let proj = glm::perspective::<f32>(1.0, glm::half_pi::<f32>() * 0.8, 0.1, 1000.0);

  let mut model = Model::new(PathBuf::from(obj_file)).expect("Should work!");
  let shader = Shader::new("src/v.glsl", "src/f.glsl").expect("Should compile");

  unsafe {
    gl::Enable(gl::DEPTH_TEST);
    gl::Enable(gl::DEBUG_OUTPUT);

    gl::BindFragDataLocation(shader.id, 0, CString::new("out_color").unwrap().as_ptr());
    model.load(shader.id);
    gl::UseProgram(shader.id);

    gl::Viewport(0, 0, 600, 600);
  }
  let mut event_pump = sdl.event_pump().unwrap();
  'running: loop {
    unsafe {
      let view_loc = gl::GetUniformLocation(shader.id, CString::new("view").unwrap().as_ptr());
      gl::UniformMatrix4fv(
        view_loc,
        1,
        gl::FALSE as u8,
        &camera as *const _ as *const _,
      );
      let proj_loc = gl::GetUniformLocation(shader.id, CString::new("proj").unwrap().as_ptr());
      gl::UniformMatrix4fv(proj_loc, 1, gl::FALSE as u8, &proj as *const _ as *const _);

      model.load_uniforms(shader.id);
      gl::ClearColor(0.0, 0.0, 1.0, 1.0);
      gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
      gl::UseProgram(shader.id);
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
        } => camera = glm::rotate_y(&camera, 1.0),
        Event::KeyDown {
          keycode: Some(Keycode::A),
          ..
        } => camera = glm::rotate_y(&camera, -1.0),
        _ => {}
      }
    }
  }
}
