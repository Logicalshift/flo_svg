use super::style::*;

use flo_canvas::*;

use svg::node::element::path::*;
use svg::node::element::tag::*;
use svg::parser;
use svgtypes::{Transform};

///
/// The state of an SVG -> flo_canvas reader
///
pub struct SvgReaderState {

}

impl Default for SvgReaderState {
    fn default() -> Self {
        SvgReaderState { 
        }
    }
}

impl SvgReaderState {
    ///
    /// Accepts an SVG parser event and updates this state accordingly
    ///
    pub fn accept(&mut self, event: parser::Event, drawing: &mut Vec<Draw>) {
        use parser::{Event};

        match event {
            Event::Tag("svg", Type::Start, _attributes) => {
            },

            Event::Tag("g", Type::Start, attributes) => {
                drawing.push_state();

                if let Some(transform) = attributes.get("transform") {
                    let transform: Transform            = transform.parse().unwrap();
                    let Transform{ a, b, c, d, e, f }   = transform;
                    let transform                       = Transform2D([[a as _, c as _, e as _], [b as _, d as _, f as _], [0.0, 0.0, 1.0]]);

                    drawing.transform(transform);
                }
            },

            Event::Tag("g", Type::End, _attributes) => {
                drawing.pop_state();
            },

            Event::Tag("path", Type::Start, attributes) |
            Event::Tag("path", Type::Empty, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();

                drawing.new_path();

                for command in data.iter() {
                    match &command {
                        Command::Move(Position::Absolute, param)        => drawing.move_to(param[0] as _, param[1] as _),
                        Command::Line(Position::Absolute, param)        => drawing.line_to(param[0] as _, param[1] as _),
                        Command::CubicCurve(Position::Absolute, param)  => drawing.bezier_curve_to(param[4] as _, param[5] as _, param[0] as _, param[1] as _, param[2] as _, param[3] as _),
                        Command::Close                                  => drawing.close_path(),
                        _                                               => { println!("Other: {:?}", command); }
                    }
                }

                if let Some(style) = attributes.get("style") {
                    let style = Style::from_style_value(style);
                    style.render_path(drawing);
                }
            }
            _ => { }
        }
    }
}
