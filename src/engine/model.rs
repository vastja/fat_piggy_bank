use std::collections::binary_heap::Iter;
use std::{collections::HashMap, error::Error, usize};
use std::{fmt, string};

pub struct Param {
    pub name: String,
    pub value: Value,
}

pub struct Model {
    params: HashMap<String, Value>,
    aliases: Vec<Alias>,
}

pub struct Alias {
    pub name: String,
    pub target: String,
    pub identifier: Identifier,
}

pub enum Identifier {
    None,
    Index(usize),
}

pub enum Value {
    Simple(String),
    List(Vec<Value>),
    Complex(Vec<Member>),
}

pub struct Member {
    pub key: String,
    pub value: Value,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Simple(value) => write!(f, "{}", value),
            Value::List(list) => {
                let converted = list
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "[ {} ]", converted)
            }
            Value::Complex(complex) => {
                let converted = complex
                    .iter()
                    .map(|member| format!("{} : {}", member.key.clone(), member.value))
                    .collect::<Vec<String>>()
                    .join(",");
                write!(f, "{{ {} }}", converted)
            }
        }
    }
}

impl Clone for Member {
    fn clone(&self) -> Member {
        Member {
            key: self.key.clone(),
            value: self.value.clone(),
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Value {
        match self {
            Value::Simple(value) => Value::Simple(value.clone()),
            Value::List(list) => Value::List(list.clone()),
            Value::Complex(complex) => Value::Complex(complex.clone()),
        }
    }
}

impl Model {
    pub fn new() -> Self {
        Self {
            params: HashMap::new(),
            aliases: vec![],
        }
    }

    pub fn new_with_params(params: Vec<Param>) -> Self {
        Self {
            params: params.into_iter().map(|x| (x.name, x.value)).collect(),
            aliases: vec![],
        }
    }

    pub fn add_param(&mut self, name: String, value: Value) {
        self.params.insert(name, value);
    }

    fn get_alias(&self, alias_name: &str) -> Option<&Value> {
        let alias: &Alias = self.aliases.iter().find(|&x| x.name == alias_name)?;

        match alias.identifier {
            Identifier::None => self.params.get(&alias.target),
            Identifier::Index(index) => {
                let target_param = self.params.get(&alias.target)?;
                match target_param {
                    Value::List(list) => list.get(index),
                    _ => None,
                }
            }
        }
    }

    pub fn get_param(&self, name: &str) -> Option<&Value> {
        let mut name_parts = name.split('.');
        let first_name = name_parts.next().unwrap();

        let mut param: &Value = match self.params.get(first_name) {
            Some(value) => value,
            None => self.get_alias(first_name)?,
        };

        for next_name in name_parts {
            match param {
                Value::Complex(complex) => {
                    param = &complex.iter().find(|x| x.key == next_name)?.value;
                }
                _ => return None,
            }
        }

        Some(param)
    }

    pub fn set_alias(&mut self, alias: Alias) {
        if let Some(existing_alias) = self.aliases.iter_mut().find(|x| x.name == alias.name) {
            *existing_alias = alias;
        } else {
            self.aliases.push(alias);
        }
    }

    pub fn remove_alias(&mut self, alias: Alias) {
        if let Some(index) = self.aliases.iter_mut().position(|x| x.name == alias.name) {
            self.aliases.remove(index);
        }
    }
}
