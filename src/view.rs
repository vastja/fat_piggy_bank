use std::fs;
use std::path::Path;

use super::engine::model;
use super::engine::template;

pub fn render(template: &str, model: &mut model::Model) -> String {
    if !Path::exists(Path::new(template)) {
        panic!("Template {} does not exist.", template)
    }

    let contents: String = fs::read_to_string(template).expect("Could not read provided file.");
    let template = template::Template::new(&contents);
    template.substitute(model)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_template_substitution() {
        let mut model = model::Model::new_with_params(vec![model::Param {
            name: String::from("items"),
            value: model::Value::List(vec![
                model::Value::Simple(String::from("one")),
                model::Value::Simple(String::from("two")),
            ]),
        }]);

        let result = render("./src/templates/comparison.html", &mut model);

        insta::assert_snapshot!(result)
    }
}
