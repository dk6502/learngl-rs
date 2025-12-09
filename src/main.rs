use std::process::ExitCode;

use crate::ecs::app::App;

mod ecs;
mod resources;

fn main() -> ExitCode {
  let Ok(mut app) = App::new() else {
    return ExitCode::FAILURE;
  };
  app.run();
  return ExitCode::SUCCESS;
}
