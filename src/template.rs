use std::collections::HashMap;

pub struct Template {
  template: String,
  variables: HashMap<&'static str, String>,
}

impl Template {
  pub fn new(template: String) -> Self {
    Self {
      template,
      variables: HashMap::new(),
    }
  }

  pub fn add(&mut self, key: &'static str, value: String) { self.variables.insert(key, value); }

  pub fn render(&self) -> String {
    let mut rendered = self.template.clone();

    for (key, value) in &self.variables {
      rendered = rendered.replace(&format!("{{{key}}}"), value);
    }

    rendered
  }
}
