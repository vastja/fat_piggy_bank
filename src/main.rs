use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::path::Path;
use std::str::Split;
use std::{env, fs};

mod engine;
mod view;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: fat_piggy_bank base-expense-file current-expense-file")
    }

    let baseline_expenses: Vec<CostItem> = load_expenses(&args[1]);
    let current_expenses: Vec<CostItem> = load_expenses(&args[2]);

    let comparison: Vec<CostItem> = compare_expenses(baseline_expenses, current_expenses);
    for expense_comparison in comparison {
        println!(
            "Difference for '{}' was {} CZK.",
            expense_comparison.tag, expense_comparison.amount
        )
    }
}

fn load_expenses(path: &str) -> Vec<CostItem> {
    if !Path::exists(Path::new(path)) {
        panic!("File {} does not exist.", path)
    }

    let contents: String = fs::read_to_string(path).expect("Could not read provided file.");
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
        let mut parts: Split<&str> = line.split(",");
        let item = CostItem {
            tag: parts.nth(tag_index).unwrap().trim().to_string(),
            amount: parts.nth(amount_index).unwrap().trim().parse().unwrap(),
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
    use rust_decimal_macros::dec;

    #[test]
    fn parse_provided_expense_data() {
        let data = ["tag, amount", "food, 75"].into_iter().collect();

        let items: Vec<CostItem> = get_items(data, "tag", "amount");

        assert_eq!(
            items.contains(&CostItem {
                tag: String::from("food"),
                amount: dec!(75)
            }),
            true
        );
    }

    #[test]
    fn expenses_comparison_order_from_worst_to_best() {
        let base = [
            CostItem::new("food", dec!(75)),
            CostItem::new("home", dec!(1055.6)),
            CostItem::new("other", dec!(100)),
        ]
        .into_iter()
        .collect();

        let current = [
            CostItem::new("food", dec!(140.5)),
            CostItem::new("home", dec!(1000)),
            CostItem::new("sport", dec!(125)),
        ]
        .into_iter()
        .collect();

        let result: Vec<CostItem> = compare_expenses(base, current);

        assert_eq!(result[0], CostItem::new("sport", dec!(125)));
        assert_eq!(result[1], CostItem::new("food", dec!(65.5)));
        assert_eq!(result[2], CostItem::new("home", dec!(-55.6)));
        assert_eq!(result[3], CostItem::new("other", dec!(-100)));
    }
}
