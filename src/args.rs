use clap::{Command, Arg, value_parser};

pub fn app() -> Command {
    Command::new("resup")
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands([
            Command::new("toolchains").about("View list of available toolchains."),
            Command::new("install").about("Install a toolchain.")
            .arg(Arg::new("name")
                    .help("Name of toolchain")
                    .num_args(1)
                    .required(true)
                    .value_parser(value_parser!(String)))
        ])
}
