use engine::model;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use view::render;

mod csv_parser;
mod engine;
mod view;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: fat_piggy_bank base-expense-file current-expense-file")
    }

    let report: String = generate_report(Path::new(&args[1]), Path::new(&args[2]));
    fs::write("generated_report.html", report).unwrap();
}

fn generate_report(baseline: &Path, current: &Path) -> String {
    let baseline_expenses: Vec<csv_parser::CostItem> = load_expenses(baseline);
    let current_expenses: Vec<csv_parser::CostItem> = load_expenses(current);

    let comparison: Vec<csv_parser::CostItem> =
        compare_expenses(baseline_expenses, current_expenses);

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

fn load_expenses(path: &Path) -> Vec<csv_parser::CostItem> {
    let contents: String = fs::read_to_string(path).expect("Could not load expense.");
    let lines: Vec<&str> = contents.lines().skip(1).collect();
    csv_parser::get_items(lines, "Kategorie", "Částka v měně účtu")
}

fn compare_expenses(
    baseline: Vec<csv_parser::CostItem>,
    current: Vec<csv_parser::CostItem>,
) -> Vec<csv_parser::CostItem> {
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

    let mut diff: Vec<csv_parser::CostItem> = comparison
        .into_iter()
        .map(|(key, value)| csv_parser::CostItem::new(key, value))
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
