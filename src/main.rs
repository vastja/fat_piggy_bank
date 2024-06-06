use std::str::Split;
use std::{env, fs};
use std::path::Path;

fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() != 2 { 
        println!("Usage: fat_piggy_bank input-csv-file-path")
    }
    let path = Path::new(&args[1]);

    let contents : String = fs::read_to_string(path).expect("Could not read provided file.");

    let lines: Vec<&str> = contents.lines().collect();

    let mut header = lines[1].split(",");
    let tag_index : usize = header.position(|x| x == "Kategorie").unwrap();
    let amount_index : usize = header.position(|x| x == "Částka v měně účtu").unwrap();

    for line in lines.iter().skip(2) {
        let mut parts: Split<&str> = line.split(",");
        println!("Tag: {}; amount: {}", parts.nth(tag_index).unwrap(), parts.nth(amount_index).unwrap())
    } 
}