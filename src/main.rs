use ast::file::MeplangFile;
use env_logger::fmt::Color;
use log::Level;
use std::io::Write;

mod opcode;
mod parser;
mod ast;
mod pre_processing;
mod compile;

fn main() {
    env_logger::builder()
        .format(|buf, record| {
            let mut style = buf.style();
            style.set_color(
                match record.level() {
                    Level::Info => Color::Green,
                    Level::Warn => Color::Yellow,
                    Level::Error => Color::Red,
                    _ => Color::White
                }
            ).set_bold(true);
            writeln!(buf, "{}", style.value(format!(
                "{}: {}",
                record.level(), record.args(),
            )))
        })
        .init();

    let meplang_file = MeplangFile::new(
        "input=".to_owned(),
        std::fs::read_to_string("input.mep").unwrap()
    );

    match meplang_file {
        Ok(res) => {dbg!(res);},
        Err(err) => log::error!("Parsing failed:\n{}", err),
    };
}
