use std::process::ExitCode;

use clap::Parser;

use crate::{
    cli::{Cli, Command},
    commands::{list, setup, upscale, use_command},
};

mod cli;
mod commands;
mod config;
mod models;
mod terminal;
mod upscaler;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Setup => setup(),
        Command::Upscale {
            input,
            output,
            overwrite,
            verbose,
        } => upscale(input, output, overwrite, verbose),
        Command::List => list(),
        Command::Use { model } => use_command(model),
    };

    if let Err(e) = result {
        print_error!("{e}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
