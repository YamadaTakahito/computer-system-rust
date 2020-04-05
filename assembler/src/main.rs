extern crate assembler;
extern crate regex;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use assembler::lexer::parser;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let filename = args.first().unwrap();
    let reader = BufReader::new(File::open(filename).expect("file not found"));
}

