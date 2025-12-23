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

            Event::Tag("defs", Type::Start, _attributes) => { todo!("defs") }
            Event::Tag("desc", Type::Start, _attributes) => { todo!("desc") }
            Event::Tag("title", Type::Start, _attributes) => { todo!("title") }
            Event::Tag("symbol", Type::Start, _attributes) => { todo!("symbol") }
            Event::Tag("use", Type::Start, _attributes) => { todo!("use") }
            Event::Tag("image", Type::Start, _attributes) => { todo!("image") }
            Event::Tag("switch", Type::Start, _attributes) => { todo!("switch") }

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

                let mut style = Style::from_attributes(&attributes);

                if let Some(css_style) = attributes.get("style") {
                    let css_style = Style::from_style_value(css_style);

                    style = css_style.merge(&style);
                }

                style.render_path(drawing);
            }

            Event::Tag("rect", Type::Start, _attributes) => { todo!("rect") }
            Event::Tag("circle", Type::Start, _attributes) => { todo!("circle") }
            Event::Tag("ellipse", Type::Start, _attributes) => { todo!("ellipse") }
            Event::Tag("line", Type::Start, _attributes) => { todo!("line") }
            Event::Tag("polyline", Type::Start, _attributes) => { todo!("polyline") }
            Event::Tag("polygon", Type::Start, _attributes) => { todo!("polygon") }

            Event::Tag("text", Type::Start, _attributes) => { todo!("text") }
            Event::Tag("tspan", Type::Start, _attributes) => { todo!("tspan") }
            Event::Tag("tref", Type::Start, _attributes) => { todo!("tref") }
            Event::Tag("textPath", Type::Start, _attributes) => { todo!("textPath") }
            Event::Tag("altGlyph", Type::Start, _attributes) => { todo!("altGlyph") }
            Event::Tag("altGlyphDef", Type::Start, _attributes) => { todo!("altGlyphDef") }
            Event::Tag("altGlyphItem", Type::Start, _attributes) => { todo!("altGlyphItem") }
            Event::Tag("glyphRef", Type::Start, _attributes) => { todo!("glyphRef") }

            Event::Tag("marker", Type::Start, _attributes) => { todo!("marker") }

            Event::Tag("color-profile", Type::Start, _attributes) => { todo!("color-profile") }

            Event::Tag("clipPath", Type::Start, _attributes) => { todo!("clipPath") }

            Event::Tag("filter", Type::Start, _attributes) => { todo!("filter") }

            Event::Tag("font", Type::Start, _attributes) => { todo!("font") }
            Event::Tag("glyph", Type::Start, _attributes) => { todo!("glyph") }
            Event::Tag("missing-glyph", Type::Start, _attributes) => { todo!("missing-glyph") }
            Event::Tag("hkern", Type::Start, _attributes) => { todo!("hkern") }
            Event::Tag("vkern", Type::Start, _attributes) => { todo!("vkern") }
            Event::Tag("font-face", Type::Start, _attributes) => { todo!("font-face") }
            Event::Tag("font-face-src", Type::Start, _attributes) => { todo!("font-face-src") }
            Event::Tag("font-face-uri", Type::Start, _attributes) => { todo!("font-face-uri") }
            Event::Tag("font-face-format", Type::Start, _attributes) => { todo!("font-face-format") }
            Event::Tag("font-face-name", Type::Start, _attributes) => { todo!("font-face-name") }

            _ => { }
        }
    }
}
