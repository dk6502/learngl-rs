use std::{error::Error, ffi::CString};

use hecs::World;
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
  world: World,
  #[allow(clippy::type_complexity)]
  update_systems: Vec<Box<dyn Fn(&mut Commands, Event, &mut Camera)>>,
  #[allow(clippy::type_complexity)]
  startup_systems: Vec<Box<dyn Fn(&mut Commands, &mut World)>>,
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

    let shader = Shader::new("src/v.glsl", "src/f.glsl").expect("Should compile");

    let world = World::new();
    unsafe {
      gl::Enable(gl::DEPTH_TEST);
      gl::Enable(gl::DEBUG_OUTPUT);

      gl::BindFragDataLocation(shader.id, 0, CString::new("out_color").unwrap().as_ptr());
      gl::UseProgram(shader.id);

      gl::Viewport(0, 0, 600, 600);
    }

    let event_pump = sdl.event_pump()?;

    Ok(App {
      window,
      commands,
      event_pump,
      _context: context,
      world,
      camera,
      shader,
      update_systems: vec![],
      startup_systems: vec![],
    })
  }

  pub fn run(&mut self) {
    for function in self.startup_systems.iter() {
      function(&mut self.commands, &mut self.world)
    }
    unsafe {
      for (_, model) in self.world.query_mut::<&mut Model>() {
        model.load(self.shader.id);
      }
    }
    'running: loop {
      if self.commands.should_close {
        break 'running;
      }
      unsafe {
        gl::ClearColor(0.0, 0.0, 1.0, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        self.camera.update(self.shader.id);
        for (_, model) in self.world.query_mut::<&mut Model>() {
          model.draw(self.shader.id);
        }
      }
      self.window.gl_swap_window();
      for event in self.event_pump.poll_iter() {
        for system in self.update_systems.iter() {
          system(&mut self.commands, event.clone(), &mut self.camera)
        }
      }
    }
  }

  pub fn with_startup_system<F: Fn(&mut Commands, &mut World) + 'static>(self, f: F) -> Self {
    let mut app = self;
    app.startup_systems.push(Box::new(f));
    app
  }

  pub fn with_update_system<F: Fn(&mut Commands, Event, &mut Camera) + 'static>(
    self,
    f: F,
  ) -> Self {
    let mut app: App = self;
    app.update_systems.push(Box::new(f));
    app
  }
}
