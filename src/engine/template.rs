use super::model;
use super::tokenizer;

pub struct Template {
    template: Vec<Box<dyn tokenizer::TokenBuffer>>,
}

impl Template {
    pub fn new(template: &str) -> Self {
        Template {
            template: tokenizer::tokenize(template),
        }
    }

    pub fn substitute(&self, model: &model::Model) -> String {
        self.template.iter().fold(String::new(), |mut acum, token| {
            acum.push_str(&token.render());
            acum
        })
    }
}
