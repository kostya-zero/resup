use std::{fs, path::Path, process::exit};

use args::app;
use config::Config;
use home::home_dir;
use models::ModelsContainer;
use proc::{run_upscale, UpscaleError};
use term::Term;

mod args;
mod config;
mod models;
mod proc;
mod term;

fn check_config() {
    if !Path::new(&Config::get_config_path()).exists() {
        Term::warn(
            "Resup is not configured. Please run `resup setup` to configure the application.",
        );
        exit(1)
    }
}

fn main() {
    let args = app().get_matches();
    match args.subcommand() {
        Some(("setup", _sub)) => {
            let mut new_config = Config::default();
            loop {
                let exec = Term::ask("Specify the path to the executable file of Real-ESRGAN.");
                let exec_path = Path::new(&exec);
                if !exec_path.exists() {
                    Term::error("Executable file not found!")
                } else if exec_path.is_dir() {
                    Term::error("Found directory, not a file.");
                } else {
                    new_config.executable = exec;
                    break;
                }
            }

            loop {
                let models = Term::ask("Specify the path to the directory with models");
                let models_path = Path::new(&models);
                if !models_path.exists() {
                    Term::error("Directory with models not found!")
                } else if models_path.is_file() {
                    Term::error("Found file, not a directory");
                } else {
                    new_config.models_path = models;
                    break;
                }
            }

            new_config.model = String::new();
            let home_path = home_dir().unwrap();
            let _ = fs::create_dir(home_path.join(".config").to_str().unwrap());
            if !Path::new(&Config::get_config_dir()).exists() {
                fs::create_dir(Config::get_config_dir()).unwrap();
            }
            Config::write(new_config);
            Term::done("Configuration has been saved.");
            Term::message("Before starting upscaling, please specify a model that you want to use with 'use' subcommand.");
            exit(0)
        }
        Some(("upscale", sub)) => {
            check_config();
            let input_file = sub.get_one::<String>("input").unwrap();
            if input_file.is_empty() {
                Term::message("Nothing to upscale!");
                exit(1)
            }

            let overwrite: bool = sub.get_flag("overwrite");
            let mut output = sub.get_one::<String>("output").unwrap().clone();

            let config = Config::load();
            if !Path::new(&config.executable).exists() {
                Term::error("Executable file for Real-ESRGAN is not found. Make sure you have specified valid path.");
                exit(1);
            }

            if config.model.is_empty() {
                Term::error("Model is not specified!");
                exit(1)
            }

            if !Path::new(&input_file).exists() {
                Term::error(
                    format!("Cannot continue because '{input_file}' are not exists.").as_str(),
                );
                exit(1)
            }

            if output.is_empty() {
                let file_name = Path::new(input_file)
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                output = file_name.to_string() + "-upscaled.png";
            }

            if Path::new(&output).exists() && !overwrite {
                Term::error(
                    format!(
                        "File with name '{}' already exists. Try new name or remove this file.",
                        &output
                    )
                    .as_str(),
                );
                exit(1);
            }

            let container = ModelsContainer::new(&config.models_path);
            let current_model = config.model.clone();

            if !container.models.iter().any(|m| m.name == current_model) {
                Term::error(format!("Model {} is not found.", current_model).as_str());
                exit(1)
            }

            Term::display_data("Using model", current_model.as_str());
            Term::message(format!("Upscaling '{input_file}'...").as_str());

            match run_upscale(config.clone(), input_file, &output) {
                Ok(_) => Term::done("Upscale completed!"),
                Err(e) => match e {
                    UpscaleError::ExecutableNotFound => {
                        Term::error("Failed to run executable file because it's not found.");
                        exit(1);
                    }
                    UpscaleError::ProcessInterrupted => {
                        Term::error("Process interrupted.");
                        exit(1);
                    }
                    UpscaleError::UnknownError => {
                        Term::error("Upscale failed with unknown reason.");
                        exit(1);
                    }
                    UpscaleError::ModelFilesNotFound => {
                        Term::error("Failed to find `.bin` and `.params` files for model. Check if both files are exists in models directory.");
                    }
                },
            }
            Term::done("Upscale finished successfully.");
        }
        Some(("list", _sub)) => {
            check_config();
            let config: Config = Config::load();
            if !Path::new(&config.models_path).exists() {
                Term::error("Failed to find model directory. Check if path set correctly.");
                exit(1);
            }

            let models_path = config.models_path;
            let container = ModelsContainer::new(models_path.as_str());

            Term::title("Available models:");
            for i in container.models.iter() {
                if i.name == config.model {
                    Term::no_icon_message(format!("{} (current)", i.name).as_str());
                } else {
                    Term::no_icon_message(i.name.as_str());
                }
            }

            if !container.bad_models.is_empty() {
                Term::warn("Models that are set up incorrectly:");
                for model in container.bad_models.iter() {
                    Term::no_icon_message(model);
                }
            }
        }
        Some(("use", sub)) => {
            check_config();
            let model_name: &str = sub.get_one::<String>("model").unwrap().as_str();
            let mut config: Config = Config::load();
            if model_name.is_empty() {
                Term::warn("Model name is note specified. Use `list` subcommand to list all available models.");
                exit(1)
            }

            if config.model == model_name {
                Term::warn("Attempt to set same model name.");
                exit(0);
            }

            config.model = model_name.to_string();
            Config::write(config);
            Term::message("Config saved.");
        }
        _ => Term::error("Unknown command."),
    }
}
