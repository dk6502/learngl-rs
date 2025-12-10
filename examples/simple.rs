extern crate nalgebra_glm as glm;

use glm::Vec3;
use hecs::World;
use motor::{
  ecs::{app::App, commands::Commands},
  resources::{camera::Camera, model::Model},
};
use sdl2::{event::Event, keyboard::Keycode};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::new()?
    .with_startup_system(startup)
    .with_update_system(kbd);
  app.run();
  return Ok(());
}

fn startup(_: &mut Commands, world: &mut World) {
  let _ = world.spawn((
    Model::new(PathBuf::from("models/arctic/TERRORIST_Arctic Avenger.obj"))
      .with_translate(&Vec3::new(0.0, 0.0, -20.0)),
  ));
  let _ =
    world
      .spawn((Model::new(PathBuf::from("models/suzanne.obj"))
        .with_translate(&Vec3::new(-5.0, 0.0, -20.0)),));
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
    } => camera.rotate_local_y(5.0),
    Event::KeyDown {
      keycode: Some(Keycode::A),
      ..
    } => camera.rotate_local_y(-5.0),
    _ => {}
  }
}
