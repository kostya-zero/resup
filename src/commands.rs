use anyhow::{anyhow, bail, Result};
use std::{fs, path::Path, process::exit};

use home::home_dir;

use crate::{
    config::Config, models::ModelsContainer, print_done, print_error, print_title, terminal::ask,
    upscaler::Upscaler,
};

pub fn setup() -> Result<()> {
    let mut new_config = Config::default();
    loop {
        let exec = ask("Specify the path to the executable file of Real-ESRGAN.");
        let exec_path = Path::new(&exec);
        if !exec_path.exists() {
            print_error!("Path not found!")
        } else if exec_path.is_dir() {
            print_error!("Found directory, expected a file.");
        } else {
            new_config.executable = exec;
            break;
        }
    }

    loop {
        let models = ask("Specify the path to the directory with models");
        let models_path = Path::new(&models);
        if !models_path.exists() {
            print_error!("Path not found!")
        } else if models_path.is_file() {
            print_error!("Found file, expected a directory.");
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
        bail!("failed to create configuration directory");
    }
    Config::write(new_config);
    print_done!("Configuration has been saved.");
    println!("Before starting upscaling, please specify a model that you want to use with 'use' subcommand.");
    Ok(())
}

pub fn upscale(input: String, output: String, overwrite: bool, verbose: bool) -> Result<()> {
    let config = Config::load();

    if !Path::new(&config.executable).exists() {
        bail!("Executable file for Real-ESRGAN is not found. Make sure you have specified a valid path.")
    }

    if config.model.is_empty() {
        bail!("Model is not specified!")
    }

    if !Path::new(&input).exists() {
        bail!("Cannot continue because '{input}' does not exist.")
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
        print_error!(
            "File with name '{output}' already exists. Try a new name or remove this file.",
        );
        exit(1);
    }

    let container = match ModelsContainer::new(&config.models_path) {
        Ok(container) => container,
        Err(err) => {
            bail!("Failed to fetch available models: {:?}", err)
        }
    };

    let current_model = &config.model;

    if !container.get_models().contains(current_model) {
        print_error!("Model {current_model} is not found.");
        exit(1);
    }
    let file_name = Path::new(&input).file_stem().unwrap().to_str().unwrap();

    println!("Upscaling '{file_name}'...");

    let upscaler = Upscaler::new(&config);
    match upscaler.upscale(&input, &output, verbose) {
        Ok(_) => print_done!("Upscale completed!"),
        Err(e) => {
            bail!("{e}")
        }
    }

    Ok(())
}

pub fn list() -> Result<()> {
    let config: Config = Config::load();
    if !Path::new(&config.models_path).exists() {
        print_error!("Failed to find model directory. Check if path set correctly.");
        exit(1);
    }

    let models_path = &config.models_path;
    let container = ModelsContainer::new(models_path)
        .map_err(|err| anyhow!("Failed to fetch available models: {err:?}"))?;

    print_title!("Available models:");
    for i in container.get_models().iter() {
        if *i == config.model {
            println!("{i} (current)");
        } else {
            println!("{i}");
        }
    }

    if !container.get_bad_models().is_empty() {
        println!("Models that are set up incorrectly:");
        for model in container.get_bad_models().iter() {
            println!("{model}");
        }
    }

    Ok(())
}

pub fn use_command(model: String) -> Result<()> {
    let mut config: Config = Config::load();
    config.model = model.to_string();
    Config::write(config);
    print_done!("Config saved.");
    Ok(())
}
