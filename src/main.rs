use crate::ecs::{app::App, commands::Commands};
use sdl2::{event::Event, keyboard::Keycode};
use std::{error::Error, process::ExitCode};
mod ecs;
mod resources;

fn main() -> Result<(), Box<dyn Error>> {
  let mut app = App::new()?.with_kbd_system(kbd)?;
  app.run();
  return Ok(());
}

fn kbd(commands: &mut Commands, event: Event) {
  match event {
    Event::Quit { .. }
    | Event::KeyDown {
      keycode: Some(Keycode::Escape),
      ..
    } => commands.should_close = true,
    _ => {}
  }
}
