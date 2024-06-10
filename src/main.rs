use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::str::Split;
use std::{env, fs};

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 3 { 
        println!("Usage: fat_piggy_bank base-expense-file current-expense-file")
    }

    let base_expenses : Vec<CostItem> = load_expenses(&args[1]);
    let current_expenses : Vec<CostItem> = load_expenses(&args[2]);

 
}

fn load_expenses(path: &str) -> Vec<CostItem> {
    if !Path::exists(Path::new(path)) {
        panic!("File {} does not exist.", path)
    } 

    let contents : String = fs::read_to_string(path).expect("Could not read provided file.");
    let lines: Vec<&str> = contents.lines().skip(1).collect();
    get_items(lines, "Kategorie", "Částka v měně účtu")
}

#[derive(PartialEq)]
struct CostItem {
    tag : String,
    amount : usize
}

fn get_items(lines : Vec<&str>, tag_col_name: &str, amount_col_name: &str) -> Vec<CostItem> {
    let mut header = lines[0].split(",");
    let tag_index : usize = header.position(|x| x.trim() == tag_col_name).unwrap();
    let amount_index : usize = header.position(|x| x.trim() == amount_col_name).unwrap();

    let mut items = Vec::new();
    for line in lines.iter().skip(1) {
        let mut parts: Split<&str> = line.split(",");
        let item = CostItem {
            tag:  parts.nth(tag_index).unwrap().trim().to_string(),
            amount: parts.nth(amount_index).unwrap().trim().parse().unwrap()
        };
        items.push(item);
    } 
    items
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_provided_expense_data() {
        let data = [ "tag, amount",  "food, 75"].into_iter().collect();

        let items : Vec<CostItem> = get_items(data, "tag", "amount");

        assert_eq!(items.contains(&CostItem { tag : String::from("food"), amount : 75 }), true);
    }
}