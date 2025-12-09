use std::{error::Error, ffi::CString, path::PathBuf};

use sdl2::{
  EventPump,
  event::Event,
  video::{GLContext, Window},
};

use crate::{
  ecs::commands::Commands,
  resources::{camera::Camera, model::Model, shader::Shader},
};

pub struct App {
  window: Window,
  commands: Commands,
  event_pump: EventPump,
  _context: GLContext,
  camera: Camera,
  shader: Shader,
  model: Model,
  kbd_system: Box<dyn Fn(&mut Commands, Event, &mut Camera)>,
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
    let commands = Commands::default();
    gl::load_with(|symbol| video_subsystem.gl_get_proc_address(symbol) as *const _);
    let camera = Camera::default();

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
      commands,
      event_pump,
      _context: context,
      camera,
      shader,
      model: model2,
      kbd_system: Box::new(|_, _, _| {}),
    });
  }

  pub fn run(self: &mut Self) {
    'running: loop {
      if self.commands.should_close {
        break 'running;
      }
      unsafe {
        gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.camera.update(self.shader.id);
        self.model.draw(self.shader.id);
      }
      self.window.gl_swap_window();
      for event in self.event_pump.poll_iter() {
        (self.kbd_system)(&mut self.commands, event, &mut self.camera)
      }
    }
  }

  pub fn with_kbd_system<F: Fn(&mut Commands, Event, &mut Camera) + 'static>(
    self: Self,
    f: F,
  ) -> Result<Self, Box<dyn Error>> {
    let mut app: App = self;
    app.kbd_system = Box::new(f);
    return Ok(app);
  }
}
