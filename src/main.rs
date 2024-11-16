use core::panic;
use engine::model;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::HashMap;
use std::fs;
use std::mem::take;
use std::path::Path;
use std::{env, usize};
use unicode_segmentation::UnicodeSegmentation;
use view::render;

mod engine;
mod view;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: fat_piggy_bank base-expense-file current-expense-file")
    }

    let report: String = generate_report(Path::new(&args[1]), Path::new(&args[2]));
    // Todo
    fs::write("generated_report.html", report);
}

fn generate_report(baseline: &Path, current: &Path) -> String {
    let baseline_expenses: Vec<CostItem> = load_expenses(baseline);
    let current_expenses: Vec<CostItem> = load_expenses(current);

    let comparison: Vec<CostItem> = compare_expenses(baseline_expenses, current_expenses);

    let differences: Vec<model::Value> = comparison
        .iter()
        .map(|x| {
            model::Value::Complex(vec![
                model::Member {
                    key: String::from("tag"),
                    value: model::Value::Simple(x.tag.to_string()),
                },
                model::Member {
                    key: String::from("amount"),
                    value: model::Value::Simple(x.amount.to_string()),
                },
            ])
        })
        .collect();

    let mut model = model::Model::new_with_params(vec![
        model::Param {
            name: String::from("total_difference"),
            value: model::Value::Simple(String::from("N/A")),
        },
        model::Param {
            name: String::from("differences"),
            value: model::Value::List(differences),
        },
    ]);

    render("./src/templates/report.html", &mut model)
}

fn load_expenses(path: &Path) -> Vec<CostItem> {
    let contents: String = fs::read_to_string(path).expect("Could not load expense.");
    let lines: Vec<&str> = contents.lines().skip(1).collect();
    get_items(lines, "Kategorie", "Částka v měně účtu")
}

#[derive(Debug, PartialEq, Clone)]
struct CostItem {
    tag: String,
    amount: Decimal,
}

impl CostItem {
    pub fn new(tag: &str, amount: Decimal) -> Self {
        CostItem {
            tag: tag.to_string(),
            amount: amount,
        }
    }
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

fn get_items(lines: Vec<&str>, tag_col_name: &str, amount_col_name: &str) -> Vec<CostItem> {
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

fn compare_expenses(baseline: Vec<CostItem>, current: Vec<CostItem>) -> Vec<CostItem> {
    let mut comparison = HashMap::<&str, Decimal>::new();

    for expense in baseline.iter() {
        let tag = expense.tag.as_str();
        match comparison.get(tag) {
            Some(value) => comparison.insert(tag, value + expense.amount),
            None => comparison.insert(tag, expense.amount),
        };
    }

    for expense in current.iter() {
        let tag = expense.tag.as_str();
        match comparison.get(tag) {
            Some(value) => comparison.insert(tag, value - expense.amount),
            None => comparison.insert(tag, -expense.amount),
        };
    }

    let mut diff: Vec<CostItem> = comparison
        .into_iter()
        .map(|(key, value)| CostItem::new(key, value))
        .collect();

    diff.sort_by(|x, y| y.amount.cmp(&x.amount));
    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_template_substitution() {
        let result: String = super::generate_report(
            Path::new("./src/test_data/baseline.csv"),
            Path::new("./src/test_data/target.csv"),
        );
        insta::assert_snapshot!(result)
    }
}
