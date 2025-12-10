use crate::{components::name::Name, resources::model::Model};

pub struct World {
  pub(crate) model_components: Vec<Option<Model>>,
  pub(crate) name_components: Vec<Option<Name>>,
}
impl World {
  pub fn spawn(self: &mut Self, model: Option<Model>, name: Option<Name>) {
    self.model_components.push(model);
    self.name_components.push(name);
  }
}

impl Default for World {
  fn default() -> Self {
    Self {
      model_components: Vec::new(),
      name_components: Vec::new(),
    }
  }
}
