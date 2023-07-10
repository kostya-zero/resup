use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("resup")
        .about("A CLI front-end for Real-ESRGAN ncnn Vulkan image upscaler written in Rust.")
        .version("0.1.0")
        .author("Konstantin Zhigaylo")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommands([Command::new("upscale").about("Upscale image").args([
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
                .default_value("")
                .value_parser(clap::value_parser!(String)),
            Arg::new("model")
                .short('m')
                .long("model")
                .help("Which model to use.")
                .default_value("realesrgan-x4plus")
                .value_parser(clap::value_parser!(String)),
            Arg::new("models")
                .short('M')
                .long("models")
                .help("Path to models directory.")
                .default_value("")
                .value_parser(clap::value_parser!(String)),
            Arg::new("executable")
                .short('e')
                .long("executable")
                .help("Set path to executable.")
                .required(false)
                .default_value("")
                .value_parser(clap::value_parser!(String)),
        ])])
}
