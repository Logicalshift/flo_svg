use flo_canvas::*;

use simplecss::*;
use css_color_parser::Color as CssColor;

///
/// How the shape should be filled in
///
pub enum FillStyle {
    Color(Color),
}

///
/// How the line outlining the current path should be drawn
///
pub enum StrokeStyle {
    Solid(Color)
}

///
/// The style properties that can be applied to a SVG path
///
pub struct Style {
    fill:           Option<FillStyle>,
    stroke:         Option<StrokeStyle>,
    line_width:     Option<f32>,
    fill_opacity:   Option<f32>,
    stroke_opacity: Option<f32>,
}

impl Style {
    ///
    /// Parses a CSS rule to determine the style of a SVG path
    ///
    pub fn from_rule(rule: &Rule) -> Self {
        let mut fill            = None;
        let mut stroke          = None;
        let mut line_width      = None;
        let mut fill_opacity    = None;
        let mut stroke_opacity  = None;

        for decl in rule.declarations.iter() {
            match decl.name {
                "fill"              => {
                    if decl.value != "none" {
                        let col = decl.value.parse::<CssColor>().unwrap();
                        fill    = Some(FillStyle::Color(Color::Rgba((col.r as f32)/255.0, (col.g as f32)/255.0, (col.b as f32)/255.0, col.a)));
                    } else {
                        fill    = None;
                    }
                }
                "fill-opacity"      => { 
                    fill_opacity = decl.value.parse::<f32>().ok();
                }
                "fill-rule"         => { }

                "stroke"            => { 
                    if decl.value != "none" {
                        let col = decl.value.parse::<CssColor>().unwrap();
                        stroke  = Some(StrokeStyle::Solid(Color::Rgba((col.r as f32)/255.0, (col.g as f32)/255.0, (col.b as f32)/255.0, col.a)));
                    } else {
                        stroke  = None;
                    }
                }
                "stroke-opacity"    => {
                    stroke_opacity = decl.value.parse::<f32>().ok();
                }
                "stroke-width"      => { 
                    if decl.value.ends_with("px") {
                        line_width = decl.value.parse::<f32>().ok();
                    } else {
                        println!("?? {:?}", decl.value);
                    }
                }
                "stroke-linecap"    => { }
                "stroke-miterlimit" => { }


                _ => { println!("Other decl: {:?}", decl.name); }
            }
        }

        Self {
            fill,
            stroke,
            line_width,
            fill_opacity,
            stroke_opacity,
        }
    }
}