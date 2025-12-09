use crate::{
  ecs::{app::App, commands::Commands},
  resources::camera::Camera,
};
use sdl2::{event::Event, keyboard::Keycode};
use std::error::Error;
mod ecs;
mod resources;

fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::new()?.with_kbd_system(kbd)?;
  app.run();
  return Ok(());
}

fn kbd(commands: &mut Commands, event: Event, camera: &mut Camera) {
  match event {
    Event::Quit { .. }
    | Event::KeyDown {
      keycode: Some(Keycode::Escape),
      ..
    } => commands.should_close = true,
    Event::KeyDown {
      keycode: Some(Keycode::W),
      ..
    } => camera.move_local_z(1.0),
    Event::KeyDown {
      keycode: Some(Keycode::S),
      ..
    } => camera.move_local_z(-1.0),
    Event::KeyDown {
      keycode: Some(Keycode::D),
      ..
    } => camera.rotate_local_y(1.0),
    Event::KeyDown {
      keycode: Some(Keycode::A),
      ..
    } => camera.rotate_local_y(-1.0),
    _ => {}
  }
}
