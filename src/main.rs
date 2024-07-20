use clap::Parser;
use std::process::ExitCode;
use thiserror::Error;

mod args;

pub use args::CliArgs;

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{e}");

        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn run() -> Result<(), RunError> {
    let _args = CliArgs::try_parse()?;

    todo!()
}

#[derive(Debug, Error)]
enum RunError {
    #[error(transparent)]
    ParseArgsError(#[from] clap::Error),
}
