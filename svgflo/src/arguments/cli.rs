use super::commands::*;

use clap::*;

///
/// CLI arguments for svgflo
///
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct SvgFloCli {
    #[command(subcommand)]
    command: SvgFloCommands,

    /// The file where the output should be sent (stdout by default)
    #[arg(short, long)]
    output: Option<String>,
}
