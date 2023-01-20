use crate::pre_processing::pre_processing::pre_process;
use ast::RFile;
use env_logger::fmt::Color;
use log::{Level, LevelFilter};
use std::io::Write;

mod ast;
mod compile;
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

    let contract_name = "SwapperConstructor".to_owned();
    let input = std::fs::read_to_string("input.mep").unwrap();

    let r_file = match RFile::new(input.clone()) {
        Ok(r_file) => r_file,
        Err(err) => {
            log::error!("Parsing failed:\n{}", err);
            return;
        }
    };

    // dbg!(&meplang_file);

    let pre_processed = match pre_process(&input, r_file, contract_name) {
        Ok(pre_processed) => pre_processed,
        Err(err) => {
            log::error!("Pre-processing failed:\n{}", err);
            return;
        }
    };

    // dbg!(&pre_processed);
}
