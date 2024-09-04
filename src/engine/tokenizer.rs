use std::collections::HashSet;

use super::model::Model;

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

pub fn convert_to_blocks(template: &str) -> Vec<Box<dyn Block>> {
    let tokens: Vec<Token> = tokenize(template);
    let mut blocks: Vec<Box<dyn Block>> = vec![];
    // Todo - insert into scopes
    for token in tokens {
        let block: Box<dyn Block> = match token.kind {
            Kind::Text => Box::new(TextBlock {
                buffer: token.buffer.clone(),
            }),
            Kind::Expression => parse_expression(&token),
        };
        blocks.push(block);
    }
    blocks
}

fn parse_expression(token: &Token) -> Box<dyn Block> {
    if let Some(end) = EndBlock::from(&token) {
        return Box::new(end);
    }

    if let Some(for_each_block) = ForEachBlock::from(&token) {
        return Box::new(for_each_block);
    }

    Box::new(VariableBlock {
        variable_name: token.buffer.clone(),
    })
}

pub trait Block {
    fn render(&self) -> String;
}

trait Scope {
    fn add_block(&mut self, block: Box<dyn Block>) -> ();
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
    fn render(&self) -> String {
        self.inner_blocks
            .iter()
            .fold(String::new(), |mut acum, block| {
                acum.push_str(&block.render());
                acum
            })
    }
}

impl Scope for AnonymousBlock {
    fn add_block(&mut self, block: Box<dyn Block>) -> () {
        self.inner_blocks.push(block);
    }
}

struct TextBlock {
    buffer: String,
}

impl Block for TextBlock {
    fn render(&self) -> String {
        self.buffer.clone()
    }
}

struct ForEachBlock {
    array_name: String,
    alias: String,
    inner_blocks: Vec<Box<dyn Block>>,
}

impl Block for ForEachBlock {
    fn render(&self) -> String {
        self.inner_blocks
            .iter()
            .fold(String::new(), |mut acum, block| {
                acum.push_str(&block.render());
                acum
            })
    }
}

impl Scope for ForEachBlock {
    fn add_block(&mut self, block: Box<dyn Block>) -> () {
        self.inner_blocks.push(block)
    }
}

impl ForEachBlock {
    pub fn from(token: &Token) -> Option<Self> {
        let header: String = token.buffer.clone();
        let parts: Option<[&str; 4]> = ForEachBlock::parse_header(&header);

        let (array_name, alias) = match parts {
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
            true => Some(parts.try_into().unwrap()),
            false => None,
        }
    }
}

struct VariableBlock {
    variable_name: String,
}

impl Block for VariableBlock {
    fn render(&self) -> String {
        format!("variable: {}", self.variable_name)
    }
}

struct EndBlock {}

impl Block for EndBlock {
    fn render(&self) -> String {
        String::from("end")
    }
}

impl EndBlock {
    fn from(token: &Token) -> Option<Self> {
        match token.buffer.trim().to_lowercase().as_str() {
            "end" => Some(EndBlock {}),
            _ => None,
        }
    }
}
