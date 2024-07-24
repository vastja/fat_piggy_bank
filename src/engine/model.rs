use std::collections::HashMap;

pub struct Param {
    pub name: String,
    pub value: String,
}

pub struct ArrayParam {
    pub name: String,
    pub value: Vec<String>,
}

pub struct Model {
    arams: HashMap<String, String>,
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
