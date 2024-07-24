pub fn tokenize(template: &str) -> Vec<Box<dyn TokenBuffer>> {
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
    tokens
}

pub trait TokenBuffer {
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
