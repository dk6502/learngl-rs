pub struct Commands {
  pub should_close: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Commands {
  fn default() -> Self {
    Self {
      should_close: false,
    }
  }
}
