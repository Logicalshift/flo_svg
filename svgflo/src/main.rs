mod arguments;

use arguments::*;

use clap::*;

fn main() {
    let parameters = SvgFloCli::parse();
}
