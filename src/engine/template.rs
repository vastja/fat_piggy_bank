use super::model;
use super::tokenizer;

pub struct Template {
    template: Box<dyn tokenizer::Scope>,
}

impl Template {
    pub fn new(template: &str) -> Self {
        Template {
            template: tokenizer::convert_to_blocks(template),
        }
    }

    pub fn substitute(&self, model: &model::Model) -> String {
        self.template.render()
    }
}
