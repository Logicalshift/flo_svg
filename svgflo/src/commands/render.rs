use crate::arguments::*;

use flo_svg::*;
use flo_render_software::*;

use std::fs::*;
use std::io::*;

use serde::*;

///
/// Renders a SVG file to a PNG file
///
pub fn render(arguments: &SvgFloCli, svg_file: &String) {
    // Set up the output file
    // TODO: file error handling
    let is_terminal;

    let mut output: Box<dyn Write> = if let Some(output_file) = &arguments.output {
        let output_file     = File::create_new(output_file).unwrap();
        is_terminal         = output_file.is_terminal();
        let output_writer   = BufWriter::new(output_file);

        Box::new(output_writer)
    } else {
        let stdout  = stdout();
        is_terminal = stdout.is_terminal();

        Box::new(stdout)
    };

    // Read the svg file
    let svg_file = File::open(svg_file).unwrap();
    let svg_file = BufReader::new(svg_file);
    let svg_file = svg_file.bytes().collect::<Result<Vec<u8>>>().unwrap();
    let svg_file = String::from_utf8_lossy(&svg_file);

    // Parse the svg file
    let mut drawing = vec![];
    parse_svg(svg_file, &mut drawing).unwrap();
}
