use crate::arguments::*;

use flo_svg::*;

use std::fs::*;
use std::io::*;

use serde::*;

///
/// Describes the instructions needed to render a SVG file
///
pub fn describe(arguments: &SvgFloCli, svg_file: &String) {
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

    // Convert to JSON, write to the output
    let drawing_json = drawing.serialize(serde_json::value::Serializer).unwrap();

    let drawing_string = if is_terminal {
        format!("{:#}", drawing_json)
    } else {
        drawing_json.to_string()
    };

    // Write to the output
    output.write(drawing_string.as_bytes()).unwrap();
}
