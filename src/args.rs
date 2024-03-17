use clap::{value_parser, Arg, ArgAction, Command};

pub fn app() -> Command {
    Command::new("resup")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("setup").about("Run the setup wizard."),
            Command::new("upscale")
                .about("Upscale image resolution.")
                .args([
                    Arg::new("input")
                        .help("Input files.")
                        .required(true)
                        .default_value("")
                        .value_parser(value_parser!(String)),
                    Arg::new("output")
                        .help("Name of output file.")
                        .last(true)
                        .num_args(1)
                        .default_value("")
                        .value_parser(value_parser!(String)),
                    Arg::new("overwrite")
                        .help("Overwrite file content if file with same name exists.")
                        .long("overwrite")
                        .short('o')
                        .action(ArgAction::SetTrue),
                    Arg::new("showoutput")
                        .help("Show output while upscaling.")
                        .long("show-output")
                        .action(ArgAction::SetTrue),
                ]),
            Command::new("list").about("List available models."),
            Command::new("use").about("Set model to use.").arg(
                Arg::new("model")
                    .help("Model name to set.")
                    .required(false)
                    .num_args(1)
                    .default_value("")
                    .value_parser(value_parser!(String)),
            ),
        ])
}
