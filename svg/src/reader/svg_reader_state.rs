use super::document::*;
use super::length::*;
use super::style::*;

use flo_canvas::*;

use svg::node::element::path::*;
use svg::node::element::tag::*;
use svg::node::{Value};
use svg::parser;
use svgtypes::{Transform};

use std::collections::{HashMap};

///
/// The state of an SVG -> flo_canvas reader
///
pub struct SvgReaderState {
    document: SvgDocument
}

impl Default for SvgReaderState {
    fn default() -> Self {
        SvgReaderState {
            document: SvgDocument::default(),
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
            Event::Tag("svg", Type::Start, attributes) => {
                self.parse_svg_attributes(&attributes);
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

    ///
    /// The description for this document
    ///
    pub fn document(&self) -> SvgDocument {
        self.document.clone()
    }

    ///
    /// Parses the attributes on the 'svg' tag
    ///
    fn parse_svg_attributes(&mut self, attributes: &HashMap<String, Value>) {
        for (name, value) in attributes.iter() {
            let name = name.as_str();

            match name {
                "viewbox"   => { self.document.viewbox = Self::viewbox(value); }
                "x"         => { self.document.x = Self::length(value); }
                "y"         => { self.document.y = Self::length(value); }
                "width"     => { self.document.width = Self::length(value); }
                "height"    => { self.document.height = Self::length(value); }

                _           =>  { }
            }
        }

        if let Some(viewbox) = attributes.get("viewbox") {
            self.document.viewbox = Self::viewbox(viewbox);
        }
    }

    ///
    /// Parses the viewbox for this document
    ///
    fn viewbox(viewbox: &str) -> Option<((f32, f32), (f32, f32))> {
        let viewbox = viewbox.parse::<svgtypes::ViewBox>().ok()?;

        Some(((viewbox.x as _, viewbox.y as _), ((viewbox.w + viewbox.x) as _, ((viewbox.h + viewbox.y) as _))))
    }

    ///
    /// Converts a length to flo_canvas coordinates
    ///
    pub (super) fn length(length: &str) -> Option<Length> {
        use svgtypes::{LengthUnit};

        // Parse the length
        let length = length.parse::<svgtypes::Length>().ok()?;

        // Result depends on the length unit
        let canvas_length = match length.unit {
            LengthUnit::None    => Some(Length::Canvas(length.number as _)),
            LengthUnit::Em      => todo!(),
            LengthUnit::Ex      => todo!(),
            LengthUnit::Px      => Some(Length::Canvas(length.number as _)),
            LengthUnit::In      => Some(Length::inches(length.number)),
            LengthUnit::Cm      => Some(Length::mm(length.number * 10.0)),
            LengthUnit::Mm      => Some(Length::mm(length.number)),
            LengthUnit::Pt      => Some(Length::Points(length.number as _)),
            LengthUnit::Pc      => todo!(),
            LengthUnit::Percent => todo!(),
        };

        canvas_length
    }
}
