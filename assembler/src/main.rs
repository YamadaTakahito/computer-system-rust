extern crate assembler;

use std::fs::File;
use std::io::{BufRead, BufReader};

use assembler::lexer::hello;

fn main() {
    let reader = BufReader::new(File::open("files/add/Add.asm").expect("file not found"));

    for line in reader.lines() {
        println!("{}", line.unwrap())
    }

    hello::hello();
}

