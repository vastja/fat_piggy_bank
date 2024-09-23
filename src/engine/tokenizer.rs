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
        variable_name: token.buffer.clone(),
    });
    scopes.add_block(block);
    ScopeOperator::Same
}

pub trait Block {
    fn render(&self) -> String;
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
    fn add_block(&mut self, block: Box<dyn Block>) {
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
        let header = format!("foreach {} in {}", self.alias, self.array_name);
        let inner = self.inner_blocks.iter().fold(header, |mut acum, block| {
            acum.push_str(&block.render());
            acum
        });
        inner + "end"
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
