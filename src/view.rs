use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn render(template: &str, model: &Model) -> String {
    if !Path::exists(Path::new(template)) {
        panic!("Template {} does not exist.", template)
    }

    // let paths = fs::read_dir("./").unwrap();

    // for path in paths {
    //    return path.unwrap().path().display().to_string();
    // }

    let contents: String = fs::read_to_string(template).expect("Could not read provided file.");
    substitute(&contents, &model)
}

fn substitute(template: &str, model: &Model) -> String {
    let mut tokens: Vec<Box<dyn TokenBuffer>> = vec![Box::new(TextBuffer::new())];

    let mut start = false;
    let mut end = false;

    for ch in template.chars() {
        match ch {
            '{' => match start {
                true => {
                    tokens.push(Box::new(ExpressionBuffer::new()));
                    start = false;
                }
                false => start = true,
            },
            '}' => match end {
                true => {
                    tokens.push(Box::new(TextBuffer::new()));
                    end = false;
                }
                false => end = true,
            },
            _ => tokens.last_mut().unwrap().push(ch),
        }
    }

    tokens.iter().fold(String::new(), |mut acum, token| {
        acum.push_str(&token.render());
        acum
    })
}

trait TokenBuffer {
    fn push(&mut self, ch: char) -> ();
    fn render(&self) -> String;
}

struct TextBuffer {
    buffer: String,
}

struct ExpressionBuffer {
    buffer: String,
}

impl ExpressionBuffer {
    fn new() -> Self {
        ExpressionBuffer {
            buffer: String::new(),
        }
    }
}

impl TokenBuffer for ExpressionBuffer {
    fn push(&mut self, ch: char) -> () {
        self.buffer.push(ch);
    }

    fn render(&self) -> String {
        String::from("Expression token")
    }
}

impl TextBuffer {
    fn new() -> Self {
        TextBuffer {
            buffer: String::new(),
        }
    }
}

impl TokenBuffer for TextBuffer {
    fn push(&mut self, ch: char) -> () {
        self.buffer.push(ch);
    }

    fn render(&self) -> String {
        self.buffer.clone()
    }
}

struct Param {
    pub name: String,
    pub value: String,
}

struct ArrayParam {
    name: String,
    pub value: Vec<String>,
}

struct Model {
    params: HashMap<String, String>,
    array_params: HashMap<String, Vec<String>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
            array_params: HashMap::new(),
        }
    }

    pub fn new_with_params(params: Vec<Param>, array_params: Vec<ArrayParam>) -> Self {
        Self {
            params: params.into_iter().map(|x| (x.name, x.value)).collect(),
            array_params: array_params
                .into_iter()
                .map(|x| (x.name, x.value))
                .collect(),
        }
    }

    pub fn add_param(&mut self, name: String, value: String) -> () {
        self.params.insert(name, value);
    }

    pub fn add_array_param(&mut self, name: String, values: Vec<String>) -> () {
        self.array_params.insert(name, values);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta;

    #[test]
    fn array_template_substitution() {
        let model = Model::new_with_params(
            vec![],
            vec![ArrayParam {
                name: String::from("items"),
                value: vec![String::from("one"), String::from("two")],
            }],
        );

        let result = render("./src/templates/comparison.html", &model);

        insta::assert_snapshot!(result)
    }
}
