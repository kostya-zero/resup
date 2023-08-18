use std::{
    path::Path,
    process::{exit, Command, Stdio},
};

use args::app;
use config::{Config, Manager};
use term::Term;

mod args;
mod config;
mod term;

fn main() {
    if !Manager::exists() {
        Manager::make_default();
    }
    let args = app().get_matches();
    match args.subcommand() {
        Some(("upscale", sub)) => {
            let input = sub
                .get_one::<String>("input")
                .expect("Failed to get input file name.")
                .to_string();
            let mut output: String = sub
                .get_one::<String>("output")
                .expect("Failed to get output file.")
                .to_string();
            let overwrite: bool = sub.get_flag("overwrite");
            if output.is_empty() {
                let file_name = Path::new(&input)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                output = file_name + "-upscaled.png";
            }

            if Path::new(&output).exists() && !overwrite {
                Term::error(
                    format!(
                        "File with name {} already exists. Try new name or remove this file.",
                        &output
                    )
                    .as_str(),
                );
                exit(1);
            }

            let config = Manager::load();
            Term::message("Preparing to upscale...");
            Term::display_data("Model", &config.upscale.model);
            Term::display_data("Executable", &config.upscale.executable);
            Term::display_data("Input file", &input);
            Term::display_data("Output file", &output);
            let mut cmd = Command::new(config.upscale.executable);
            cmd.args(vec![
                "-i",
                &input,
                "-o",
                &output,
                "-s",
                "4",
                "-f",
                "png",
                "-m",
                &config.upscale.models_path,
                "-n",
                &config.upscale.model,
            ]);
            cmd.stdout(Stdio::inherit());
            cmd.stdin(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
            Term::message("Starting...");
            let process_result = cmd.output();
            match process_result {
                Ok(_) => {
                    Term::done("Upscale completed!");
                }
                Err(i) => {
                    match i.kind() {
                        std::io::ErrorKind::NotFound => {
                            Term::error("Cannot find executable file. Check if path set correctly.");
                            exit(1);
                        },
                        std::io::ErrorKind::Interrupted => {
                            Term::error("Interrupted.");
                            exit(1);
                        }
                        _ => {
                            Term::error("Unknown error ocurs.");
                            exit(1);
                        }
                    }
                }
            }
        }
        Some(("model", sub)) => {
            let model_name: String = sub
                .get_one::<String>("model")
                .expect("Failed to get path variable.")
                .to_string();
            let mut config: Config = Manager::load();
            if model_name.is_empty() {
                Term::display_data("Current model", &config.upscale.model);
                exit(0);
            }

            if config.upscale.model == model_name {
                Term::warn("Attempt to set same model name.");
                exit(0);
            }

            config.upscale.model = model_name;
            Manager::write(config);
            Term::message("Config saved.");
        }
        Some(("models-dir", sub)) => {
            let path: String = sub
                .get_one::<String>("path")
                .expect("Failed to get path variable.")
                .to_string();
            let mut config: Config = Manager::load();
            if path.is_empty() {
                Term::display_data("Current path to directory with models", &config.upscale.models_path);
                exit(0);
            }

            if config.upscale.models_path == path {
                Term::warn("Attempt to set same path to models directory.");
                exit(0);
            }

            config.upscale.models_path = path;
            Manager::write(config);
            Term::message("Config saved.");
        }
        Some(("executable", sub)) => {
            let executable: String = sub
                .get_one::<String>("path")
                .expect("Failed to get path variable.")
                .to_string();
            let mut config: Config = Manager::load();
            if executable.is_empty() {
                Term::display_data("Current path to executable", &config.upscale.executable);
                exit(0);
            }

            if config.upscale.executable == executable {
                Term::warn("Attempt to set same path to executable.");
                exit(0);
            }

            config.upscale.executable = executable;
            Manager::write(config);
            Term::message("Config saved.");
        }
        Some(("config", _sub)) => {
            Term::display_data("Path to config", Manager::get_config_path().as_str());
        }
        _ => Term::error("Unknown command."),
    }
}
