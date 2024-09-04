use super::model;
use super::tokenizer;

pub struct Template {
    template: Vec<Box<dyn tokenizer::Block>>,
}

impl Template {
    pub fn new(template: &str) -> Self {
        Template {
            template: tokenizer::convert_to_blocks(template),
        }
    }

    pub fn substitute(&self, model: &model::Model) -> String {
        self.template.iter().fold(String::new(), |mut acum, token| {
            acum.push_str(&token.render());
            acum
        })
    }
}
