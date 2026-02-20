#![doc = include_str!("../../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use clap::{Parser, Subcommand};
use meplang::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Version,
    Compile(Compile),
}

#[derive(Debug, Parser)]
struct Compile {
    #[arg(long, short)]
    contract: String,

    #[arg(long, short)]
    input: String,

    #[arg(long, short)]
    output: Option<String>,

    #[arg(long, short)]
    settings: Option<String>,
}

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Version => println!("Meplang version: {}", env!("CARGO_PKG_VERSION")),
        Commands::Compile(cli) => {
            let settings = if let Some(settings) = cli.settings {
                match serde_json::from_str(&settings) {
                    Ok(decoded) => decoded,
                    Err(err) => {
                        eprintln!("Unable to decode compiler settings: {}", err);
                        return;
                    }
                }
            } else {
                CompilerSettings::default()
            };

            match compile_file(cli.input.as_str(), cli.contract.as_str(), settings) {
                Ok(artifacts) => {
                    if let Some(output_file) = cli.output {
                        match std::fs::write(
                            &output_file,
                            serde_json::to_string_pretty(&artifacts).unwrap(),
                        ) {
                            Ok(()) => {
                                println!(
                                    "Contract `{}` bytecode written in the file `{}`.",
                                    cli.contract, output_file
                                );
                            }
                            Err(err) => {
                                eprintln!("{}", err);
                            }
                        }
                    } else {
                        println!(
                            "Contract `{}` bytecode: 0x{}",
                            cli.contract,
                            hex::encode(artifacts.main_bytecode())
                        );
                    }
                }
                Err(err) => {
                    eprintln!("{}", err);
                }
            }
        }
    }
}
