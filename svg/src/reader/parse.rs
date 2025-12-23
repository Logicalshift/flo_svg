use super::document::*;
use super::svg_reader_state::*;

use std::io::{Result};
use std::path::{Path};
use flo_canvas::*;

///
/// Appends the drawing instructions for rendering the supplied SVG file to the `drawing` vec
///
pub fn parse_svg(svg: impl Into<String>, drawing: &mut Vec<Draw>) -> Result<SvgDocument> {
    // Open the SVG data
    let mut svg = svg.into();
    let events  = svg::read(&mut svg)?;

    // Interpret to a set of drawing instructions
    let mut state = SvgReaderState::default();
    for evt in events {
        state.accept(evt, drawing);
    }

    Ok(state.document())
}

///
/// Reads svg from a file and parses into the instructions for rendering it, stored in the `drawing` vec
///
pub fn read_svg(path: impl AsRef<Path>, drawing: &mut Vec<Draw>) -> Result<SvgDocument> {
    // Open the SVG data
    let mut svg = String::new();
    let events  = svg::open(path, &mut svg)?;

    // Interpret to a set of drawing instructions
    let mut state = SvgReaderState::default();
    for evt in events {
        state.accept(evt, drawing);
    }

    Ok(state.document())
}
