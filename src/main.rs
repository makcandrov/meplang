use meplang::*;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

const HELP_MESSAGE: &str = "\
Usage: meplang <COMMAND>\n\n\
Commands:\n\
\tcompile: Compile a Meplang file into EVM bytecode.\n\
\tversion: Print version information.\n\
";

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let mut args = std::env::args();
    args.next();

    let Some(mode) = args.next() else {
        println!("Meplang - An EVM low-level language.\n\n{}", HELP_MESSAGE);
        return;
    };

    match mode.as_str() {
        "version" => println!("Meplang version: {}", env!("CARGO_PKG_VERSION")),
        "compile" => {
            let mut contract = Option::<String>::None;
            let mut input_file = Option::<String>::None;
            let mut output_file = Option::<String>::None;
            let mut settings = Option::<CompilerSettings>::None;

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "-c" | "-contract" => {
                        let Some(next) = args.next() else {
                            tracing::error!("Expected an argument after `{}`.", arg);
                            return;
                        };
                        if contract.replace(next).is_some() {
                            tracing::error!("Contract name specified multiple times.");
                            return;
                        }
                    },
                    "-i" | "-input" => {
                        let Some(next) = args.next() else {
                            tracing::error!("Expected an argument after `{}`.", arg);
                            return;
                        };
                        if input_file.replace(next).is_some() {
                            tracing::error!("Input path specified multiple times.");
                            return;
                        }
                    },
                    "-o" | "-output" => {
                        let Some(next) = args.next() else {
                            tracing::error!("Expected an argument after `{}`.", arg);
                            return;
                        };
                        if output_file.replace(next).is_some() {
                            tracing::error!("Output path specified multiple times.");
                            return;
                        }
                    },
                    "-s" | "-settings" => {
                        let Some(next) = args.next() else {
                            tracing::error!("Expected an argument after `{}`.", arg);
                            return;
                        };
                        let decoded: CompilerSettings = match serde_json::from_str(&next) {
                            Ok(decoded) => decoded,
                            Err(err) => {
                                tracing::error!("Unable to decode compiler settings: {}", err);
                                return;
                            },
                        };
                        if settings.replace(decoded).is_some() {
                            tracing::error!("Compiler settings specified multiple times.");
                            return;
                        }
                    },
                    _ => {
                        tracing::error!("Unexpected argument `{}`.", &arg);
                        return;
                    },
                }
            }

            let Some(contract) = contract else {
                tracing::error!("Expected a contract name (-contract <CONTRACT_NAME>).");
                return;
            };

            let Some(input_file) = input_file else {
                tracing::error!("Expected an input file (-input <CONTRACT_NAME>).");
                return;
            };

            match compile_file(input_file.as_str(), contract.as_str(), settings.unwrap_or_default()) {
                Ok(artifacts) => {
                    if let Some(output_file) = output_file {
                        match std::fs::write(&output_file, serde_json::to_string_pretty(&artifacts).unwrap()) {
                            Ok(()) => {
                                println!(
                                    "Contract `{}` bytecode written in the file `{}`.",
                                    contract, output_file
                                );
                                return;
                            },
                            Err(err) => {
                                tracing::error!("{}", err);
                                return;
                            },
                        }
                    } else {
                        println!(
                            "Contract `{}` bytecode: {}",
                            contract,
                            format!("0x{}", hex::encode(artifacts.main_bytecode()))
                        );
                        return;
                    }
                },
                Err(err) => {
                    tracing::error!("{}", err);
                    return;
                },
            }
        },
        _ => tracing::error!("Unexpected command `{}`", mode),
    }
}
