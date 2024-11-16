use std::{env, fs};

use serde_json::Value;

fn to_string(file_name: String) {
    let data = fs::read_to_string(file_name).expect("Error reading file");

    let v: Value = serde_json::from_str(data.as_str()).expect("Error parsing file contents");

    print!("{:?}", v.to_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => to_string(args[1].clone()),
        _ => panic!("Missing JSON file name/path"),
    }
}
