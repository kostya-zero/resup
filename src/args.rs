use clap::{Command, Arg, value_parser};

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
                .value_parser(value_parser!(String))
            ]),

            Command::new("executable")
            .about("Set path to executable")
            .arg(
                Arg::new("path")
                .help("Path to executable.")
                .required(false)
                .num_args(1)
                .default_value("")
                .value_parser(value_parser!(String))
            )
        ])
}
