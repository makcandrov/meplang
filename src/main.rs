use std::collections::HashSet;

use block::Block;

mod attribute;
mod block;
mod variable;

fn main() {
    let code = std::fs::read_to_string("input.mep").unwrap();
    let parsed = MeplangParser::parse(Rule::file, &code);
    dbg!(parsed);
}

#[derive(Debug, Clone)]
pub struct Contract (Vec<Block>);

#[derive(Debug, Copy, Clone)]
pub enum ParsingError {
    Generic,
}

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "meplang.pest"]
pub struct MeplangParser;
