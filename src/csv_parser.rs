use rust_decimal::Decimal;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq, Clone)]
pub struct CostItem {
    pub tag: String,
    pub amount: Decimal,
}

impl CostItem {
    pub fn new(tag: &str, amount: Decimal) -> Self {
        CostItem {
            tag: tag.to_string(),
            amount,
        }
    }
}

pub fn get_items(lines: Vec<&str>, tag_col_name: &str, amount_col_name: &str) -> Vec<CostItem> {
    let header = lines[0].split(',').collect::<Vec<&str>>();
    let tag_index: usize = header
        .iter()
        .position(|x| x.trim() == tag_col_name)
        .unwrap();
    let amount_index: usize = header
        .iter()
        .position(|x| x.trim() == amount_col_name)
        .unwrap();

    let mut items = Vec::new();
    for line in lines.iter().skip(1) {
        let parts: Vec<&str> = parse_csv(line);
        let item = CostItem {
            tag: parts[tag_index].trim().to_string(),
            amount: parts[amount_index]
                .trim()
                .replace(',', ".")
                .parse()
                .expect(parts[amount_index]),
        };
        items.push(item);
    }
    items
}

enum State {
    Initial,
    EscapeStart,
    EscapeEnd,
    Column { start: usize },
    ColumnEscaped { start: usize },
    Separator,
}

fn parse_csv(line: &str) -> Vec<&str> {
    let mut state = State::Initial;
    let mut cols = vec![];
    let mut last_pos = 0;
    let mut last_len = 0;
    for ch in line.graphemes(true).collect::<Vec<&str>>().into_iter() {
        state = match state {
            State::Initial => match ch {
                "," => panic!("First character in row is not expected to be ','"),
                "\"" => State::EscapeStart,
                _ => State::Column { start: last_pos },
            },
            State::EscapeStart => match ch {
                "\"" => {
                    cols.push("");
                    State::EscapeEnd
                }
                _ => State::ColumnEscaped { start: last_pos },
            },
            State::EscapeEnd => match ch {
                "," => State::Separator,
                _ => panic!("After end of escaped sequence has to be a separator"),
            },
            State::Column { start } => match ch {
                "," => {
                    cols.push(&line[start..last_pos]);
                    State::Separator
                }
                _ => state,
            },
            State::ColumnEscaped { start } => match ch {
                "\"" => {
                    cols.push(&line[start..last_pos - last_len]);
                    State::EscapeEnd
                }
                _ => state,
            },
            State::Separator => match ch {
                "," => {
                    cols.push("");
                    state
                }
                "\"" => State::EscapeStart,
                _ => State::Column { start: last_pos },
            },
        };
        last_len = ch.len();
        last_pos += last_len;
    }

    if let State::Column { start } = state {
        cols.push(&line[start..last_pos]);
    }

    cols
}
