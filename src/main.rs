use ast::file::MeplangFile;
use primitive_types::H256;

mod opcode;
mod parser;
mod ast;
mod pre_processing;
mod compile;

fn main() {
    let meplang_file = MeplangFile::new(
        "input=".to_owned(),
        std::fs::read_to_string("input.mep").unwrap()
    );

    match meplang_file {
        Ok(res) => {dbg!(res);},
        Err(err) => {println!("{}", err);},
    };
}
