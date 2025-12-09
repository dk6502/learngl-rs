use std::{error::Error, ffi::CString, path::PathBuf};

use sdl2::{
  EventPump,
  event::Event,
  keyboard::Keycode,
  video::{GLContext, Window},
};

use crate::resources::{camera::Camera, model::Model, shader::Shader};

pub struct App {
  window: Window,
  event_pump: EventPump,
  context: GLContext,
  camera: Camera,
  shader: Shader,
  model: Model,
}

impl App {
  pub fn new() -> Result<Self, Box<dyn Error>> {
    let sdl = sdl2::init()?;
    let video_subsystem = sdl.video()?;
    let window = video_subsystem
      .window("opengl", 600, 600)
      .opengl()
      .position_centered()
      .build()?;

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
    gl_attr.set_context_flags().debug().set();

    let context = window.gl_create_context()?;
    let _ = window.gl_make_current(&context);
    gl::load_with(|symbol| video_subsystem.gl_get_proc_address(symbol) as *const _);
    let mut camera = Camera::default();

    let mut model2 = Model::new(PathBuf::from("models/Dust 2/Dust2.obj")).expect("Should work!");

    let shader = Shader::new("src/v.glsl", "src/f.glsl").expect("Should compile");

    unsafe {
      gl::Enable(gl::DEPTH_TEST);
      gl::Enable(gl::DEBUG_OUTPUT);

      gl::BindFragDataLocation(shader.id, 0, CString::new("out_color").unwrap().as_ptr());
      gl::UseProgram(shader.id);

      model2.load(shader.id);

      gl::Viewport(0, 0, 600, 600);
    }

    let event_pump = sdl.event_pump()?;

    return Ok(App {
      window,
      event_pump,
      context,
      camera,
      shader,
      model: model2,
    });
  }

  pub fn run(self: &mut Self) {
    'running: loop {
      unsafe {
        self.camera.update();
        self.camera.update_uniforms(self.shader.id);
        self.model.load_uniforms(self.shader.id);
        gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.model.draw(self.shader.id);
      }
      self.window.gl_swap_window();
      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit { .. }
          | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
          } => break 'running,
          Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
          } => self.camera.move_local_z(1.0),
          Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
          } => self.camera.move_local_z(-1.0),
          Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
          } => self.camera.rotate_local_y(1.0),
          Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
          } => self.camera.rotate_local_y(-1.0),
          _ => {}
        }
      }
    }
  }
}
