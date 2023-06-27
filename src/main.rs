use crate::term::Term;
use args::cli;
use img::{ImageInformation, Img};
use proc::upscale;
use std::{env, path::Path, process::exit};

mod args;
mod img;
mod proc;
mod term;

fn main() {
    let args = cli().get_matches();
    match args.subcommand() {
        Some(("upscale", sub)) => {
            let input: String = sub
                .get_one::<String>("input")
                .expect("Cannot get argument content.")
                .to_string();
            let output: String = sub
                .get_one::<String>("output")
                .expect("Cannot get argument content.")
                .to_string();
            let model: String = sub
                .get_one::<String>("model")
                .expect("Cannot get argument content.")
                .to_string();
            let models: String = sub
                .get_one::<String>("models")
                .expect("Cannot get argument content.")
                .to_string();
            let mut executable: String = sub
                .get_one::<String>("executable")
                .expect("Error")
                .to_string();

            if !executable.is_empty() && !Path::new(executable.as_str()).exists() {
                Term::fatal("Cannot find given path to executable.");
                exit(1);
            }

            if executable.is_empty() {
                if env::consts::OS == "windows" {
                    executable = "realesrgan-ncnn-vulkan.exe".to_string();
                } else {
                    executable = "realesrgan-ncnn-vulkan".to_string();
                }
            }

            if model.is_empty() {
                Term::fatal("No model specified.");
                exit(1);
            }

            if !Path::new(&input).exists() {
                Term::fatal("Input file not found!");
                exit(1);
            }

            if Path::new(&output).exists() {
                Term::fatal("Output file with same name alerady exists.");
                exit(1);
            }

            Term::work("Gethering image information, it might take a while...");
            let image_info: ImageInformation = Img::get_image_meta(input.clone());
            Term::info(&format!(
                "Intitial size of image: {}x{}",
                image_info.width, image_info.height
            ));
            let multi_width = image_info.width * 4;
            let multi_height = image_info.height * 4;
            Term::info(&format!(
                "Final image resolution: {}x{}",
                multi_width, multi_height
            ));

            Term::work("Calling real-esrgan-ncnn-vulkan with arguments...");
            let result = upscale(input, output, model, executable, models);
            if !result {
                Term::fatal("Upscale failed!");
                exit(1);
            }

            Term::info("Your image are ready!");
        }
        _ => Term::fatal("Unknown command!"),
    }
}
