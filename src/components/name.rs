pub struct Name(pub &'static str);

impl Name {
  pub fn new(name: &'static str) -> Self {
    Self(name)
  }
}
