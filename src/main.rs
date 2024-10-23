use engine::model;
use importer::{FileImporter, Importer};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::{env, fs};
use view::render;

mod engine;
mod importer;
mod view;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: fat_piggy_bank base-expense-file current-expense-file")
    }

    let report: String = generate_report(&args[1], &args[2], &FileImporter {});
    // Todo
    fs::write("generated_report.html", report);
}

fn generate_report<T: Importer>(baseline: &str, current: &str, importer: &T) -> String {
    let baseline_expenses: Vec<CostItem> = load_expenses(baseline, importer);
    let current_expenses: Vec<CostItem> = load_expenses(current, importer);

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

    render("templates/report.html", &mut model)
}

fn load_expenses<T: Importer>(path: &str, importer: &T) -> Vec<CostItem> {
    let contents: String = importer.load(path).expect("Could not load expense.");
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

fn get_items(lines: Vec<&str>, tag_col_name: &str, amount_col_name: &str) -> Vec<CostItem> {
    let mut header = lines[0].split(",");
    let tag_index: usize = header.position(|x| x.trim() == tag_col_name).unwrap();
    let amount_index: usize = header.position(|x| x.trim() == amount_col_name).unwrap();

    let mut items = Vec::new();
    for line in lines.iter().skip(1) {
        // Todo - properly split csv file line
        let parts: Vec<&str> = vec![];
        let item = CostItem {
            tag: parts[tag_index].trim().to_string(),
            amount: parts[amount_index]
                .trim()
                .parse()
                .expect(parts[amount_index]),
        };
        items.push(item);
    }
    items
}

fn compare_expenses(baseline: Vec<CostItem>, current: Vec<CostItem>) -> Vec<CostItem> {
    let mut comparison: Vec<CostItem> = vec![];
    for expense in baseline.iter() {
        let current_expense: Decimal = match current.iter().find(|x| x.tag == expense.tag) {
            Some(item) => item.amount,
            None => dec!(0),
        };
        comparison.push(CostItem::new(
            &expense.tag,
            current_expense - expense.amount,
        ));
    }

    let new_expenses = current
        .iter()
        .filter(|x| !baseline.iter().any(|y| y.tag == x.tag));
    for expense in new_expenses {
        comparison.push(expense.clone());
    }

    comparison.sort_by(|x, y| y.amount.cmp(&x.amount));
    comparison
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn array_template_substitution() {
        let header = "Datum a čas,Kategorie,Částka v měně účtu";
        let baseline_rows = format!("{}\n{}", header, "7/1/2024,Koloniál,\"200,00\"");
        let current_rows = format!("{}\n{}", header, "8/1/2024,Koloniál,\"400,00\"");
        let mut mem_importer = MemoryImporter::new();
        mem_importer.set("baseline", &baseline_rows);
        mem_importer.set("current", &current_rows);

        let result: String = super::generate_report("baseline", "current", &mem_importer);
        insta::assert_snapshot!(result)
    }

    struct MemoryImporter<'a> {
        memory: HashMap<&'a str, &'a str>,
    }

    impl<'a> MemoryImporter<'a> {
        fn new() -> Self {
            MemoryImporter {
                memory: HashMap::new(),
            }
        }

        fn set(&mut self, uri: &'a str, data: &'a str) {
            self.memory.insert(uri, data);
        }
    }

    impl<'a> Importer for MemoryImporter<'a> {
        fn load(&self, uri: &str) -> Result<String, std::io::Error> {
            match self.memory.get(uri) {
                Some(data) => Ok(data.to_string()),
                None => Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Data not set for uri.",
                )),
            }
        }
    }
}
