use std::{collections::HashMap, usize};

pub struct Param {
    pub name: String,
    pub value: String,
}

pub struct ArrayParam {
    pub name: String,
    pub value: Vec<String>,
}

pub struct Model {
    params: HashMap<String, String>,
    array_params: HashMap<String, Vec<String>>,
    aliases: Vec<Alias>,
}

pub struct Alias {
    name: String,
    target: String,
    identifier: Identifier,
}

pub enum Identifier {
    None,
    Index(usize),
}

impl Model {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
            array_params: HashMap::new(),
            aliases: vec![],
        }
    }

    pub fn new_with_params(params: Vec<Param>, array_params: Vec<ArrayParam>) -> Self {
        Self {
            params: params.into_iter().map(|x| (x.name, x.value)).collect(),
            array_params: array_params
                .into_iter()
                .map(|x| (x.name, x.value))
                .collect(),
            aliases: vec![],
        }
    }

    pub fn add_param(&mut self, name: String, value: String) {
        self.params.insert(name, value);
    }

    pub fn add_array_param(&mut self, name: String, values: Vec<String>) {
        self.array_params.insert(name, values);
    }

    pub fn get_array_param(&self, name: &str) -> Option<&Vec<String>> {
        self.array_params.get(name)
    }

    pub fn get_param(&self, name: &str) -> Option<&String> {
        let param = self.params.get(name);
        if param.is_none() {
            if let Some(alias) = self.aliases.iter().find(|&x| x.name == name) {
                return match alias.identifier {
                    Identifier::None => self.params.get(&alias.target),
                    Identifier::Index(index) => {
                        if let Some(array_param) = self.array_params.get(&alias.target) {
                            return array_param.get(index);
                        }
                        Option::None
                    }
                };
            }
        }
        param
    }

    pub fn set_alias(&mut self, alias: Alias) {
        if let Some(existing_alias) = self.aliases.iter().find(|&x| x.name == alias.name) {
            *existing_alias = alias;
        } else {
            self.aliases.push(alias);
        }
    }

    pub fn remove_alias(&mut self, alias: Alias) {
        if let Some(index) = self.aliases.iter().position(|&x| x.name == alias.name) {
            self.aliases.remove(index);
        }
    }
}
