use crate::view::render;

use super::model::{Alias, Identifier, Model};
use core::panic;
use std::fmt::Debug;

fn tokenize(template: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![Token::new(Kind::Text)];

    let mut start = false;
    let mut end = false;

    for ch in template.chars() {
        match ch {
            '{' => match start {
                true => {
                    tokens.push(Token::new(Kind::Expression));
                    start = false;
                }
                false => start = true,
            },
            '}' => match end {
                true => {
                    tokens.push(Token::new(Kind::Text));
                    end = false;
                }
                false => end = true,
            },
            _ => tokens.last_mut().unwrap().buffer.push(ch),
        }
    }
    tokens
}

enum Kind {
    Text,
    Expression,
}

struct Token {
    buffer: String,
    kind: Kind,
}

impl Token {
    fn new(kind: Kind) -> Self {
        Token {
            buffer: String::new(),
            kind,
        }
    }
}

pub fn convert_to_blocks(template: &str) -> Box<dyn Scope> {
    let tokens: Vec<Token> = tokenize(template);

    let global = Scopes::Anonymous(AnonymousBlock::new());
    let mut scope: Vec<Scopes> = vec![global];
    for token in tokens {
        let scope_op = match token.kind {
            Kind::Text => {
                let block = Box::new(TextBlock {
                    buffer: token.buffer.clone(),
                });
                scope.last_mut().unwrap().add_block(block);
                ScopeOperator::Same
            }
            Kind::Expression => parse_expression(&token, scope.last_mut().unwrap()),
        };

        match scope_op {
            ScopeOperator::Same => (),
            ScopeOperator::New(new_scope) => scope.push(new_scope),
            ScopeOperator::End => {
                let child_scope = scope.pop().unwrap();
                let parent_scope = scope.last_mut().unwrap();
                let child_block: Box<dyn Block> = match child_scope {
                    Scopes::Anonymous(anonymous) => Box::new(anonymous),
                    Scopes::ForEach(for_each) => Box::new(for_each),
                };
                parent_scope.add_block(child_block);
            }
        }
    }

    match scope.pop().unwrap() {
        Scopes::Anonymous(global) => Box::new(global),
        _ => panic!("Encountered not global scope."),
    }
}

enum ScopeOperator {
    New(Scopes),
    Same,
    End,
}

enum Scopes {
    Anonymous(AnonymousBlock),
    ForEach(ForEachBlock),
}

impl Scopes {
    fn add_block(&mut self, block: Box<dyn Block>) {
        match self {
            Scopes::Anonymous(anonymous) => anonymous.add_block(block),
            Scopes::ForEach(for_each) => for_each.add_block(block),
        }
    }
}

fn parse_expression(token: &Token, scopes: &mut Scopes) -> ScopeOperator {
    if EndBlock::from(&token).is_some() {
        return ScopeOperator::End;
    }

    if let Some(for_each_block) = ForEachBlock::from(&token) {
        let for_each = for_each_block;
        return ScopeOperator::New(Scopes::ForEach(for_each));
    }

    let block = Box::new(VariableBlock {
        variable_name: token.buffer.clone().trim().to_string(),
    });
    scopes.add_block(block);
    ScopeOperator::Same
}

pub trait Block {
    fn render(&self, model: &mut Model) -> Result<String, String>;
}

pub trait Scope: Block {
    fn add_block(&mut self, block: Box<dyn Block>);
}

struct AnonymousBlock {
    inner_blocks: Vec<Box<dyn Block>>,
}

impl AnonymousBlock {
    fn new() -> Self {
        AnonymousBlock {
            inner_blocks: vec![],
        }
    }
}

impl Block for AnonymousBlock {
    fn render(&self, model: &mut Model) -> Result<String, String> {
        self.inner_blocks
            .iter()
            .try_fold(String::new(), |mut acum, block| {
                acum.push_str(&block.render(model)?);
                Ok(acum)
            })
    }
}

impl Scope for AnonymousBlock {
    fn add_block(&mut self, block: Box<dyn Block>) {
        self.inner_blocks.push(block);
    }
}

struct TextBlock {
    buffer: String,
}

impl Block for TextBlock {
    fn render(&self, model: &mut Model) -> Result<String, String> {
        Result::Ok(self.buffer.clone())
    }
}

struct ForEachBlock {
    array_name: String,
    alias: String,
    inner_blocks: Vec<Box<dyn Block>>,
}

impl Block for ForEachBlock {
    fn render(&self, model: &mut Model) -> Result<String, String> {
        let mut block_render = String::new();
        let array_param = match model.get_array_param(&self.array_name) {
            Some(param) => param.clone(),
            None => {
                return Err(format!(
                    "Array parameter '{}' not found in current scope.",
                    self.array_name
                ))
            }
        };

        for i in 0..array_param.len() {
            model.set_alias(Alias {
                name: self.alias.clone(),
                target: self.array_name.clone(),
                identifier: Identifier::Index(i),
            });
            let inner: Result<String, String> =
                self.inner_blocks
                    .iter()
                    .try_fold(String::new(), |mut acum, block| {
                        acum.push_str(&block.render(model)?);
                        Ok(acum)
                    });
            block_render.push_str(&inner?);
        }
        Ok(block_render)
    }
}

impl Scope for ForEachBlock {
    fn add_block(&mut self, block: Box<dyn Block>) {
        self.inner_blocks.push(block);
    }
}

impl ForEachBlock {
    pub fn from(token: &Token) -> Option<Self> {
        let header: String = token.buffer.clone();
        let parts: Option<[&str; 4]> = ForEachBlock::parse_header(&header);

        let (alias, array_name) = match parts {
            Some(parts) => (parts[1], parts[3]),
            None => return None,
        };

        Some(ForEachBlock {
            array_name: array_name.to_string(),
            alias: alias.to_string(),
            inner_blocks: vec![],
        })
    }

    fn parse_header(header: &str) -> Option<[&str; 4]> {
        let parts: Vec<&str> = header.trim().split_whitespace().collect();
        let is_header = parts.len() == 4
            && parts[0].to_lowercase() == "foreach"
            && parts[2].to_lowercase() == "in";
        match is_header {
            true => {
                let trimmed: Vec<&str> = parts.iter().map(|x| x.trim()).collect();
                Some(trimmed.try_into().unwrap())
            }
            false => None,
        }
    }
}

struct VariableBlock {
    variable_name: String,
}

impl Block for VariableBlock {
    fn render(&self, model: &mut Model) -> Result<String, String> {
        match model.get_param(&self.variable_name) {
            Some(variable) => Ok(variable.clone()),
            None => Err(format!(
                "Variable with name '{}' not found in current scope.",
                self.variable_name
            )),
        }
    }
}

struct EndBlock {}

impl EndBlock {
    fn from(token: &Token) -> Option<Self> {
        match token.buffer.trim().to_lowercase().as_str() {
            "end" => Some(EndBlock {}),
            _ => None,
        }
    }
}
