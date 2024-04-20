use std::{
    env, fs,
    path::{Path, PathBuf},
    process::exit,
};

use args::app;
use config::{Config, Manager};
use home::home_dir;
use proc::{run_upscale, UpscaleError};
use term::Term;

mod args;
mod config;
mod proc;
mod term;

fn check_config() {
    if !Manager::exists() {
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
            if env::consts::OS == "windows" {
                Term::warn("Because you are on Windows, replace single backslash with double.");
            }
            loop {
                let exec = Term::ask(
                    "Specify the path to the executable file of 'realeasrgan-ncnn-vulkan'",
                );
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
            if !Path::new(&Manager::get_config_dir()).exists() {
                fs::create_dir(Manager::get_config_dir()).unwrap();
            }
            Manager::write(new_config);
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

            let config = Manager::load();
            Term::message("Preparing to upscale...");
            if config.model.is_empty() {
                Term::error("Model is not specified!");
                exit(1)
            }
            Term::display_data("Using model", config.model.clone().as_str());
            Term::message("Starting...");

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

            let show_output = sub.get_flag("showoutput");

            Term::message(format!("Upscaling '{input_file}'...").as_str());
            let upscale_result: Result<(), UpscaleError> =
                run_upscale(config.clone(), input_file, &output, show_output);

            match upscale_result {
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
                    UpscaleError::ModelsDirectoryNotFound => {
                        Term::error("Failed to find directory with models. Please check if path set correctly.",);
                        exit(1)
                    }
                    UpscaleError::ModelParamNotFound => {
                        Term::error("Failed to find model's `.param` file. Check if `.param` file exists in directory with models.");
                        exit(1)
                    }
                    UpscaleError::ModelBinNotFound => {
                        Term::error("Failed to find model's `.bin` file. Check if `.bin` file exists in directory with models.");
                    }
                },
            }
            Term::done("Upscale finished successfully.");
        }
        Some(("list", _sub)) => {
            check_config();
            let config: Config = Manager::load();
            if !config.check_models_path_exists() {
                Term::error("Failed to find model directory. Check if path set correctly.");
                exit(1);
            }

            let models_path = config.models_path;
            let mut available_models: Vec<String> = Vec::new();
            for entry in fs::read_dir(models_path.clone()).unwrap() {
                let entry: PathBuf = entry.unwrap().path();
                let entry_path: &str = entry.to_str().unwrap();
                let filename: &str = Path::new(entry_path).file_stem().unwrap().to_str().unwrap();
                if available_models.contains(&filename.to_string()) {
                    continue;
                }

                let param_path: PathBuf =
                    Path::new(&models_path).join(filename.to_string() + ".param");
                let bin_path: PathBuf = Path::new(&models_path).join(filename.to_string() + ".bin");

                if param_path.exists() && bin_path.exists() {
                    available_models.push(filename.to_string());
                }
            }

            Term::title("Available models:");
            for i in available_models.iter() {
                if *i == config.model {
                    Term::no_icon_message(format!("{} (current)", i).as_str());
                } else {
                    Term::no_icon_message(i);
                }
            }
        }
        Some(("use", sub)) => {
            check_config();
            let model_name: &str = sub.get_one::<String>("model").unwrap().as_str();
            let mut config: Config = Manager::load();
            if model_name.is_empty() {
                Term::warn("Model name is note specified. Use `list` subcommand to list all available models.");
                exit(1)
            }

            if config.model == model_name {
                Term::warn("Attempt to set same model name.");
                exit(0);
            }

            config.model = model_name.to_string();
            Manager::write(config);
            Term::message("Config saved.");
        }
        _ => Term::error("Unknown command."),
    }
}
