#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

mod challenges;
use challenges::*;

use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use log::{info, debug, error};
use simple_logger;

fn main() {
    simple_logger::init_with_level(log::Level::Debug).unwrap();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid cmd line arguments count");
    }
    let input_file_name = args[1].clone();
    debug!("Input file name is \"{}\"", input_file_name);

    let input_file = File::open(input_file_name).expect("Failed opening input file");
    let reader = BufReader::new(input_file);
    let mut input_data: Vec<String> = vec![];

    for line in reader.lines() {
        input_data.push(line.unwrap());
    }

    challenge05::solve(input_data.clone());
}
