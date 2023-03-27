use std::{path::Path, process::exit};
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

            if !(model == "photo" || model == "anime") {
                println!("Unknown model name specified. Model name can be `photo` or `anime`.");
                exit(1);
            }

            if !Path::new(&input).exists() {
                println!("Input file not found.");
                exit(1);
            }

            if Path::new(&output).exists() {
                println!("Output file with same name already exists.");
                exit(1);
            }

            Term::info("Gethering image information, it might take a little bit...");
            
            let image_info: ImageInformation = Img::get_image_meta(input.clone());
            println!("Initial size of image: {}x{}", image_info.width.to_owned(), image_info.height.to_string());
            let multi_width = image_info.width * 4;
            let multi_height = image_info.height * 4;
            println!("Final image resolution: {}x{}", multi_width.to_string(), multi_height.to_owned());

            Term::info("Calling realesrgan-ncnn-vulkan with arguments...");
            let result = Proc::upscale(input, output, model);
            if !result {
                println!("Upscale failed!");
                exit(1);
            }

            Term::info("Your image are ready!");
        },
        _ => println!("Unknown command!")
    }
}
