use std::{fs, path::Path, process::exit};

use home::home_dir;

use crate::{config::Config, models::ModelsContainer, term::Term, upscaler::Upscaler};

pub fn setup() {
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
    if !Path::new(&Config::get_config_dir()).exists()
        && fs::create_dir(Config::get_config_dir()).is_err()
    {
        Term::error("Unable to create configuration directory");
        exit(1);
    }
    Config::write(new_config);
    Term::done("Configuration has been saved.");
    Term::message("Before starting upscaling, please specify a model that you want to use with 'use' subcommand.");
    exit(0)
}

pub fn upscale(input: String, output: String, overwrite: bool, verbose: bool) {
    let config = Config::load();

    if !Path::new(&config.executable).exists() {
        Term::error("Executable file for Real-ESRGAN is not found. Make sure you have specified a valid path.");
        exit(1);
    }

    if config.model.is_empty() {
        Term::error("Model is not specified!");
        exit(1);
    }

    if !Path::new(&input).exists() {
        Term::error(&format!(
            "Cannot continue because '{input}' does not exist."
        ));
        exit(1);
    }

    let output = if output.is_empty() {
        let file_stem = Path::new(&input)
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        format!("{}-upscaled.png", file_stem)
    } else {
        output
    };

    if Path::new(&output).exists() && !overwrite {
        Term::error(
            format!(
                "File with name '{}' already exists. Try a new name or remove this file.",
                output
            )
            .as_str(),
        );
        exit(1);
    }

    let container = match ModelsContainer::new(&config.models_path) {
        Ok(container) => container,
        Err(err) => {
            eprintln!("Failed to fetch available models: {:?}", err);
            exit(1);
        }
    };

    let current_model = &config.model;

    if !container.get_models().contains(current_model) {
        Term::error(&format!("Model {} is not found.", current_model));
        exit(1);
    }
    let file_name = Path::new(&input).file_stem().unwrap().to_str().unwrap();

    Term::message(&format!("Upscaling '{file_name}'..."));

    let upscaler = Upscaler::new(&config);
    match upscaler.upscale(&input, &output, verbose) {
        Ok(_) => Term::done("Upscale completed!"),
        Err(e) => {
            Term::error(e.to_string().as_str());
            exit(1);
        }
    }
}

pub fn list() {
    let config: Config = Config::load();
    if !Path::new(&config.models_path).exists() {
        Term::error("Failed to find model directory. Check if path set correctly.");
        exit(1);
    }

    let models_path = &config.models_path;
    let container = ModelsContainer::new(models_path).unwrap_or_else(|err| {
        Term::error(&format!("Failed to fetch available models: {:?}", err));
        std::process::exit(1);
    });

    Term::title("Available models:");
    for i in container.get_models().iter() {
        if *i == config.model {
            Term::no_icon_message(format!("{} (current)", i).as_str());
        } else {
            Term::no_icon_message(i.as_str());
        }
    }

    if !container.get_bad_models().is_empty() {
        Term::warn("Models that are set up incorrectly:");
        for model in container.get_bad_models().iter() {
            Term::no_icon_message(model);
        }
    }
}

pub fn use_command(model: String) {
    let mut config: Config = Config::load();
    if config.model == model {
        Term::warn("Attempt to set same model name.");
        exit(0);
    }

    config.model = model.to_string();
    Config::write(config);
    Term::message("Config saved.");
}
