use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(arg_required_else_help = true, subcommand_required = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run the setup wizard.
    Setup,

    /// Upscale image resolution.
    Upscale {
        /// Input file name.
        input: String,

        /// Output file name.
        output: String,

        /// Overwrite the existing output file.
        overwrite: bool,

        /// Show output without progress bar.
        verbose: bool,
    },

    /// List available models.
    List,

    /// Set model to use.
    Use {
        /// Model name to set.
        model: String,
    },
}
