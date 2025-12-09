pub struct Commands {
  pub should_close: bool,
}

impl Default for Commands {
  fn default() -> Self {
    Self {
      should_close: false,
    }
  }
}
