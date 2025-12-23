mod arguments;
mod commands;

use arguments::*;
use commands::*;

use clap::*;

fn main() {
    let parameters = SvgFloCli::parse();

    match &parameters.command {
        SvgFloCommands::Render { svg_file }     => render(&parameters, svg_file),
        SvgFloCommands::Describe { svg_file }   => describe(&parameters, svg_file),
    }
}
