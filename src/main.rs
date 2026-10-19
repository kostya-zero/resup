use clap::Parser;

use crate::{
    cli::{Cli, Command},
    commands::{list, setup, upscale, use_command},
};

mod cli;
mod commands;
mod config;
mod models;
mod term;
mod upscaler;

fn main() {
    let cli = Cli::parse();

    match cli.command {
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
}
