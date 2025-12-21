use flo_canvas::*;

use svg::node::element::path::*;
use svg::node::element::tag::*;
use svg::parser;
use svgtypes::{Transform};
use simplecss::*;
use css_color_parser::Color as CssColor;

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
                    let style       = format!("path {{ {} }}", style);
                    let style       = StyleSheet::parse(&style);
                    let rule        = &style.rules[0];

                    let mut fill    = None;
                    let mut stroke  = None;

                    for decl in rule.declarations.iter() {
                        match decl.name {
                            "fill"              => {
                                if decl.value != "none" {
                                    let col = decl.value.parse::<CssColor>().unwrap();
                                    fill    = Some(Color::Rgba((col.r as f32)/255.0, (col.g as f32)/255.0, (col.b as f32)/255.0, col.a));
                                } else {
                                    fill    = None;
                                }
                            }
                            "fill-opacity"      => { 
                                let alpha           = decl.value.parse::<f32>().unwrap();
                                let new_fill        = fill.unwrap_or(Color::Rgba(0.0, 0.0, 0.0, 1.0));
                                let (r, g, b, _a)   = new_fill.to_rgba_components();
                                fill                = Some(Color::Rgba(r, g, b, alpha));
                            }
                            "fill-rule"         => { }

                            "stroke"            => { 
                                if decl.value != "none" {
                                    let col = decl.value.parse::<CssColor>().unwrap();
                                    stroke  = Some(Color::Rgba((col.r as f32)/255.0, (col.g as f32)/255.0, (col.b as f32)/255.0, col.a));
                                } else {
                                    stroke  = None;
                                }
                            }
                            "stroke-opacity"    => { 
                                let alpha           = decl.value.parse::<f32>().unwrap();
                                let new_stroke      = stroke.unwrap_or(Color::Rgba(0.0, 0.0, 0.0, 1.0));
                                let (r, g, b, _a)   = new_stroke.to_rgba_components();
                                stroke              = Some(Color::Rgba(r, g, b, alpha));
                            }
                            "stroke-width"      => { 
                                if decl.value.ends_with("px") {
                                    drawing.line_width(decl.value[0..(decl.value.len()-2)].parse().unwrap());
                                } else {
                                    println!("?? {:?}", decl.value);
                                }
                            }
                            "stroke-linecap"    => { }
                            "stroke-miterlimit" => { }


                            _ => { println!("Other decl: {:?}", decl.name); }
                        }
                    }

                    if let Some(fill) = fill {
                        drawing.fill_color(fill);
                        drawing.fill();
                    }

                    if let Some(stroke) = stroke {
                        drawing.stroke_color(stroke);
                        drawing.stroke();
                    }
                }
            }
            _ => { }
        }
    }
}
