use clap::{value_parser, Arg, ArgAction, Command};

pub fn app() -> Command {
    Command::new("resup")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("upscale")
                .about("Upscale image resolution.")
                .args([
                    Arg::new("input")
                        .help("Input file")
                        .required(true)
                        .num_args(1)
                        .default_value("")
                        .value_parser(value_parser!(String)),
                    Arg::new("output")
                        .help("Name of output file.")
                        .required(false)
                        .num_args(1)
                        .default_value("")
                        .value_parser(value_parser!(String)),
                    Arg::new("overwrite")
                        .help("Overwrite file content if file with same name exists.")
                        .long("overwrite")
                        .short('o')
                        .action(ArgAction::SetTrue),
                    Arg::new("quite")
                        .help("Run upscale without configuration info and Real-ESRGAN output.")
                        .short('q')
                        .long("quite")
                        .action(ArgAction::SetTrue),
                ]),
            Command::new("list").about("List available models."),
            Command::new("executable")
                .about("Set path to executable")
                .arg(
                    Arg::new("path")
                        .help("Path to executable.")
                        .required(false)
                        .num_args(1)
                        .default_value("")
                        .value_parser(value_parser!(String)),
                ),
            Command::new("config").about("Get path to config file."),
            Command::new("model")
                .about("Set or get current model.")
                .arg(
                    Arg::new("model")
                        .help("Model name to set.")
                        .required(false)
                        .num_args(1)
                        .default_value("")
                        .value_parser(value_parser!(String)),
                ),
            Command::new("models-dir")
                .about("Set or get current models directory.")
                .arg(
                    Arg::new("path")
                        .help("Path to directory with models.")
                        .required(false)
                        .num_args(1)
                        .default_value("")
                        .value_parser(value_parser!(String)),
                ),
        ])
}
