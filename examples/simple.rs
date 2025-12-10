extern crate nalgebra_glm as glm;

use motor::{
  components::{model::Model, name::Name},
  ecs::{app::App, commands::Commands, world::World},
  resources::camera::Camera,
};
use sdl2::{event::Event, keyboard::Keycode};
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::new()?
    .with_startup_system(startup)
    .with_kbd_system(kbd);
  app.run();
  return Ok(());
}

fn startup(_: &mut Commands, world: &mut World) {
  world.spawn(None, Some(Name::new("Dylan")));
  world.spawn(
    Some(
      Model::new(PathBuf::from("models/Dust 2/Dust2.obj"))
        .with_rotate((270 as f32).to_radians(), &glm::Vec3::new(1.0, 0.0, 0.0)),
    ),
    Some(Name::new("de_dust2")),
  );
  world.spawn(
    Some(
      Model::new(PathBuf::from("models/sakuya/InuSakuyaS.obj"))
        .with_scale(&glm::Vec3::new(0.1, 0.1, 0.1))
        .with_translate(&glm::Vec3::new(50.0, 0.0, 0.0))
        .with_rotate((42 as f32).to_radians(), &glm::Vec3::new(0.0, 1.0, 0.0)),
    ),
    None,
  );
  world.spawn(
    Some(Model::new(PathBuf::from("models/suzanne.obj"))),
    Some(Name("Suzanne")),
  );
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
