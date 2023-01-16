use std::collections::HashSet;

use attribute::Attribute;
use block::Block;

mod attribute;
mod block;
mod variable;

extern crate pest;
#[macro_use]
extern crate pest_derive;
use pest::Parser;

#[derive(Parser)]
#[grammar = "meplang.pest"]
pub struct MeplangParser;

fn main() {
    let code = std::fs::read_to_string("input.mep").unwrap();
    let parsed = match MeplangParser::parse(Rule::file, &code) {
        Ok(parsed) => parsed,
        Err(err) => {
            println!("parsing failed: {}", err);
            return;
        },
    };
    // parsed.
    // for item in parsed {
    //     dbg!(item);
    // }
    dbg!(parsed);
}

pub struct Contract {
    pub attributes: Vec<Attribute>,
    pub blocks: Vec<Block>,
}

