use ast::file::RFile;
use env_logger::fmt::Color;
use log::{Level, LevelFilter};
use std::io::Write;
use crate::pre_processing::pre_processing::pre_process;

mod ast;
mod compile;
mod opcode;
mod parser;
mod pre_processing;

fn init_env_logger() {
    env_logger::builder()
    .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            let mut style = buf.style();
            style
                .set_color(match record.level() {
                    Level::Info => Color::Green,
                    Level::Warn => Color::Yellow,
                    Level::Error => Color::Red,
                    _ => Color::White,
                })
                .set_bold(true);
            writeln!(
                buf,
                "{}",
                style.value(format!("{}: {}", record.level(), record.args(),))
            )
        })
        .init();
}

fn main() {
    init_env_logger();

    let contract_name = "contr".to_owned();
    let input = std::fs::read_to_string("input.mep").unwrap();

    match RFile::new(input.clone()) {
        Ok(meplang_file) => {
            dbg!(&meplang_file);
            match pre_process(&input, meplang_file, contract_name) {
                Ok(res) => {
                    dbg!(res);
                }
                Err(err) => log::error!("Parsing failed:\n{}", err),
            };
        }
        Err(err) => log::error!("Parsing failed:\n{}", err),
    };
}
