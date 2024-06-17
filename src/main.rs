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
                    Term::error("Path not found!")
                } else if exec_path.is_dir() {
                    Term::error("Found directory, expected a file.");
                } else {
                    new_config.executable = exec;
                    break;
                }
            }

            loop {
                let models = Term::ask("Specify the path to the directory with models");
                let models_path = Path::new(&models);
                if !models_path.exists() {
                    Term::error("Path not found!")
                } else if models_path.is_file() {
                    Term::error("Found file, expected a directory.");
                } else {
                    new_config.models_path = models;
                    break;
                }
            }

            new_config.model = String::new();
            let home_path = home_dir().unwrap();
            let _ = fs::create_dir(home_path.join(".config").to_str().unwrap());
            if !Path::new(&Config::get_config_dir()).exists() {
                if let Ok(_) = fs::create_dir(Config::get_config_dir()) {

                }
            }
            Config::write(new_config);
            Term::done("Configuration has been saved.");
            Term::message("Before starting upscaling, please specify a model that you want to use with 'use' subcommand.");
            exit(0)
        }
        Some(("upscale", sub)) => {
            check_config();

            let input_file = match sub.get_one::<String>("input") {
                Some(input) if !input.is_empty() => input.clone(),
                _ => {
                    Term::message("Nothing to upscale!");
                    exit(1);
                }
            };
        
            let overwrite = sub.get_flag("overwrite");
            let output = sub.get_one::<String>("output").unwrap_or(&String::new()).clone();
        
            let config = Config::load();
        
            if !Path::new(&config.executable).exists() {
                Term::error("Executable file for Real-ESRGAN is not found. Make sure you have specified a valid path.");
                exit(1);
            }
        
            if config.model.is_empty() {
                Term::error("Model is not specified!");
                exit(1);
            }
        
            if !Path::new(&input_file).exists() {
                Term::error(&format!("Cannot continue because '{input_file}' does not exist."));
                exit(1);
            }
        
            let output = if output.is_empty() {
                let file_stem = Path::new(&input_file).file_stem().unwrap().to_str().unwrap().to_string();
                format!("{}-upscaled.png", file_stem)
            } else {
                output
            };
        
            if Path::new(&output).exists() && !overwrite {
                Term::error(&format!("File with name '{}' already exists. Try a new name or remove this file.", &output));
                exit(1);
            }
        
            let container = match ModelsContainer::new(&config.models_path) {
                Ok(container) => container,
                Err(err) => {
                    eprintln!("Failed to fetch available models: {:?}", err);
                    exit(1);
                }
            };
        
            let current_model = config.model.clone();
        
            if !container.models.iter().any(|m| m.name == current_model) {
                Term::error(&format!("Model {} is not found.", current_model));
                exit(1);
            }

            let verbose = sub.get_flag("verbose");
        
            Term::display_data("Using model", &current_model);
            Term::message(&format!("Upscaling '{input_file}'..."));
        
            match run_upscale(config.clone(), &input_file, &output, verbose) {
                Ok(_) => Term::done("Upscale completed!"),
                Err(e) => {
                    match e {
                        UpscaleError::ExecutableNotFound => Term::error("Failed to run executable file because it's not found."),
                        UpscaleError::ProcessInterrupted => Term::error("Process interrupted."),
                        UpscaleError::UnknownError => Term::error("Upscale failed for an unknown reason."),
                    }
                    exit(1);
                }
            }
        }
        Some(("list", _sub)) => {
            check_config();
            let config: Config = Config::load();
            if !Path::new(&config.models_path).exists() {
                Term::error("Failed to find model directory. Check if path set correctly.");
                exit(1);
            }

            let models_path = &config.models_path;
            let container = ModelsContainer::new(models_path).unwrap_or_else(|err| {
                eprintln!("Failed to fetch available models: {:?}", err);
                std::process::exit(1);
            });

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
