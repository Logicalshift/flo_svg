use clap::*;

///
/// The subcommands for the main 'svgflo' command
///
#[derive(Subcommand)]
pub enum SvgFloCommands {
    /// Renders a SVG file to a bitmap file (PNG by default)
    Render {
        /// The file to render
        svg_file: String,
    },

    /// Converts a SVG file to a list of instructions that can be used to render it
    Describe {
        svg_file: String,
    },
}
