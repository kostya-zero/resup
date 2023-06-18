use std::{path::Path, process::exit, env};
use clap::{Command, Arg};
use img::{ImageInformation, Img};

use crate::{proc::Proc, term::Term};

mod img;
mod proc;
mod term;

fn app() -> Command {
    Command::new("resup")
        .about("A CLI front-end for Real-ESRGAN ncnn Vulkan image upscaler written in Rust.")
        .version("0.1.0")
        .author("Konstantin Zhigaylo")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommands([
                     Command::new("upscale")
                     .about("Upscale image")
                     .args([
                           Arg::new("input")
                           .short('i')
                           .long("input")
                           .help("Input file to upscale.")
                           .required(true)
                           .value_parser(clap::value_parser!(String)),

                           Arg::new("output")
                           .short('o')
                           .long("output")
                           .help("Name of the final output.")
                           .required(true)
                           .value_parser(clap::value_parser!(String)),

                           Arg::new("model")
                               .short('m')
                               .long("model")
                               .help("Model to use (photo or anime).")
                               .required(true)
                               .value_parser(clap::value_parser!(String)),

                           Arg::new("executable")
                              .short('e')
                              .long("executable")
                              .help("Set path to executable.")
                              .required(false)
                              .default_value("")
                              .value_parser(clap::value_parser!(String))

                     ])
        ])
}

fn main() {
    let args = app().get_matches();
    match args.subcommand() {
        Some(("upscale", sub)) => {
            let input: String = sub.get_one::<String>("input").expect("Error").to_string();
            let output: String = sub.get_one::<String>("output").expect("Error").to_string();
            let model: String = sub.get_one::<String>("model").expect("Error").to_string();
            let mut executable: String = sub.get_one::<String>("executable").expect("Error").to_string();

            if !executable.is_empty() && !Path::new(executable.as_str()).exists() {
                Term::fatal("Cannot find given path to executable.");
                exit(1);
            }

            if executable.is_empty() {
                if env::consts::OS == "windows" {
                    executable = "realesrgan-ncnn-vulkan.exe".to_string();
                }
                else {
                    executable = "realesrgan-ncnn-vulkan".to_string();
                }
            }

            if !(model == "photo" || model == "anime") {
                Term::fatal("Unknown mode name specified. Mode name can be `photo` or `anime`.");
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

            Term::info("Gethering image information, it might take a little bit...");
            
            let image_info: ImageInformation = Img::get_image_meta(input.clone());
            println!("Initial size of image: {}x{}", image_info.width.to_owned(), image_info.height);
            let multi_width = image_info.width * 4;
            let multi_height = image_info.height * 4;
            println!("Final image resolution: {}x{}", multi_width, multi_height);

            Term::info("Calling realesrgan-ncnn-vulkan with arguments...");
            let result = Proc::upscale(input, output, model, executable);
            if !result {
                Term::fatal("Upscale failed!");
                exit(1);
            }

            Term::info("Your image are ready!");
        },
        _ => Term::fatal("Unknown command!")
    }
}
