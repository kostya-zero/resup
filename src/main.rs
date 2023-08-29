use std::{
    path::Path,
    process::exit,
};

use args::app;
use config::{Config, Manager};
use proc::{run_upscale, UpscaleError};
use term::Term;

mod args;
mod config;
mod proc;
mod term;

fn main() {
    if !Manager::exists() {
        Manager::make_default();
    }
    let args = app().get_matches();
    match args.subcommand() {
        Some(("upscale", sub)) => {
            let input = sub.get_one::<String>("input").unwrap().to_string();
            let mut output: String = sub.get_one::<String>("output").unwrap().to_string();
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
            let quite: bool = false;
            Term::message("Preparing to upscale...");
            if quite {
                Term::display_data("Model", &config.get_model());
                Term::display_data("Executable", &config.get_executable_path());
                Term::display_data("Input file", &input);
                Term::display_data("Output file", &output);
                Term::message("Starting...");
            }
            let upscale_result: Result<(), UpscaleError> =
                run_upscale(config, &input, &output, quite);
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
        }
        Some(("model", sub)) => {
            let model_name: String = sub.get_one::<String>("model").unwrap().to_string();
            let mut config: Config = Manager::load();
            if model_name.is_empty() {
                Term::display_data("Current model", &config.get_model());
                if !config.check_model_param_exists() {
                    Term::error("Failed to find model's `.param` file. Check if `.param` file exists in directory with models.");
                }
                if !config.check_model_bin_exists() {
                    Term::error("Failed to find model's `.bin` file. Check if `.bin` file exists in directory with models.");
                }
                exit(0);
            }

            if config.get_model() == model_name {
                Term::warn("Attempt to set same model name.");
                exit(0);
            }

            config.set_model(&model_name);
            Manager::write(config);
            Term::message("Config saved.");
        }
        Some(("models-dir", sub)) => {
            let path: String = sub.get_one::<String>("path").unwrap().to_string();
            let mut config: Config = Manager::load();
            if path.is_empty() {
                Term::display_data(
                    "Current path to directory with models",
                    &config.get_models_path(),
                );
                if !config.check_models_path_exists() {
                    Term::error(
                        "Failed to find directory with models. Please check if path set correctly.",
                    );
                }
                exit(0);
            }

            if config.get_models_path() == path {
                Term::warn("Attempt to set same path to models directory.");
                exit(0);
            }
            config.set_models_path(&path);
            Manager::write(config);
            Term::message("Config saved.");
        }
        Some(("executable", sub)) => {
            let executable: String = sub.get_one::<String>("path").unwrap().to_string();
            let mut config: Config = Manager::load();
            if executable.is_empty() {
                Term::display_data("Current path to executable", &config.get_executable_path());
                if !config.check_executable_exists() {
                    Term::error("Failed to find executable by given path. Please check if path set correctly.");
                }
                exit(0);
            }

            if config.get_executable_path() == executable {
                Term::warn("Attempt to set same path to executable.");
                exit(0);
            }

            config.set_executable_path(&executable);
            Manager::write(config);
            Term::message("Config saved.");
        }
        Some(("config", _sub)) => {
            Term::display_data("Path to config", Manager::get_config_path().as_str());
        }
        _ => Term::error("Unknown command."),
    }
}
