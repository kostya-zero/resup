use std::{path::Path, process::{exit, Command, Stdio}};

use args::app;
use config::Manager;
use term::Term;

mod args;
mod config;
mod term;

fn main() {
    let args = app().get_matches();
    match args.subcommand() {
        Some(("upscale", sub)) => {
            let input = sub.get_one::<String>("input").expect("Failed to get input file name.").to_string();
            let mut output: String = sub.get_one::<String>("output").expect("Failed to get output file.").to_string();
            if output.is_empty() {
                let file_name = Path::new(&input).file_stem().unwrap().to_str().unwrap().to_string();
                output = file_name + "-upscaled.png";
            }

            if Path::new(&output).exists() {
                Term::error(format!("File with name {} already exists. Try new name or remove this file.", &output).as_str());
                exit(1);
            }

            let config = Manager::load();
            let mut cmd = Command::new(config.upscale.executable);
            cmd.args(vec!["-i", &input, "-o", &output, "-s", "4", "-f", "png", "-m", &config.upscale.models_path, "-n", &config.upscale.model]);
            cmd.stdout(Stdio::inherit());
            cmd.stdin(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
            let process_result = cmd.output();
            match process_result {
                Ok(_) => {
                    Term::message("Upscale completed!");
                },
                Err(_) => {
                    Term::error("Upscale failed.");
                },
            }
        },
       _ => Term::error("Unknown command.")
    }


}
