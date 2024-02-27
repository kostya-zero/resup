use std::{
    fs,
    path::{Path, PathBuf},
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
            let quite: bool = sub.get_flag("quite");
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
            if !quite {
                Term::display_data("Model", &config.model);
                Term::display_data("Executable", &config.executable);
                Term::display_data("Input file", &input);
                Term::display_data("Output file", &output);
            }
            Term::message("Starting...");
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
        Some(("list", _sub)) => {
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

                let param_path: String = models_path.clone() + "/" + filename + ".param";
                let bin_path: String = models_path.clone() + "/" + filename + ".bin";
                let param_found: bool = Path::new(&param_path).exists();
                let bin_found: bool = Path::new(&bin_path).exists();

                if param_found && bin_found {
                    available_models.push(filename.to_string());
                }
            }

            Term::message("Available models:");
            for i in available_models.iter() {
                if *i == config.model {
                    Term::no_icon_message(format!("{} (current)", i).as_str());
                } else {
                    Term::no_icon_message(i);
                }
            }
        }
        Some(("model", sub)) => {
            let model_name: String = sub.get_one::<String>("model").unwrap().to_string();
            let mut config: Config = Manager::load();
            if model_name.is_empty() {
                Term::display_data("Current model", &config.model);
                if !config.check_model_param_exists() {
                    Term::error("Failed to find model's `.param` file. Check if `.param` file exists in directory with models.");
                }
                if !config.check_model_bin_exists() {
                    Term::error("Failed to find model's `.bin` file. Check if `.bin` file exists in directory with models.");
                }
                exit(0);
            }

            if config.model == model_name {
                Term::warn("Attempt to set same model name.");
                exit(0);
            }

            config.model = model_name;
            Manager::write(config);
            Term::message("Config saved.");
        }
        Some(("models-dir", sub)) => {
            let path: String = sub.get_one::<String>("path").unwrap().to_string();
            let mut config: Config = Manager::load();
            if path.is_empty() {
                Term::display_data(
                    "Current path to directory with models",
                    &config.models_path,
                );
                if !config.check_models_path_exists() {
                    Term::error(
                        "Failed to find directory with models. Please check if path set correctly.",
                    );
                }
                exit(0);
            }

            if config.models_path == path {
                Term::warn("Attempt to set same path to models directory.");
                exit(0);
            }
            config.models_path = path;
            Manager::write(config);
            Term::message("Config saved.");
        }
        Some(("executable", sub)) => {
            let executable: String = sub.get_one::<String>("path").unwrap().to_string();
            let mut config: Config = Manager::load();
            if executable.is_empty() {
                Term::display_data("Current path to executable", &config.executable);
                if !config.check_executable_exists() {
                    Term::error("Failed to find executable by given path. Please check if path set correctly.");
                }
                exit(0);
            }

            if config.executable == executable {
                Term::warn("Attempt to set same path to executable.");
                exit(0);
            }

            config.executable = executable;
            Manager::write(config);
            Term::message("Config saved.");
        }
        Some(("config", _sub)) => {
            Term::display_data("Path to config", Manager::get_config_path().as_str());
        }
        _ => Term::error("Unknown command."),
    }
}
