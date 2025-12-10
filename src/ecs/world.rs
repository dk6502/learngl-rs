use crate::{components::model::Model, components::name::Name};

#[derive(Default)]
pub struct World {
  pub(crate) model_components: Vec<Option<Model>>,
  pub(crate) name_components: Vec<Option<Name>>,
}

impl World {
  pub fn spawn(&mut self, model: Option<Model>, name: Option<Name>) {
    self.model_components.push(model);
    self.name_components.push(name);
  }
}
