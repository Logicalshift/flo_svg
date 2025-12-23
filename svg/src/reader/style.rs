use flo_canvas::*;

use svg::node::{Value};
use simplecss::*;
use css_color_parser::Color as CssColor;

use std::collections::{HashMap};

///
/// How the shape should be filled in
///
#[derive(Clone, Debug)]
pub enum FillStyle {
    Color(Color),
}

///
/// How the line outlining the current path should be drawn
///
#[derive(Clone, Debug)]
pub enum StrokeStyle {
    Solid(Color)
}

///
/// The style properties that can be applied to a SVG path
///
#[derive(Clone, Debug)]
pub struct Style {
    fill:           Option<FillStyle>,
    stroke:         Option<StrokeStyle>,
    line_width:     Option<f32>,
    fill_opacity:   Option<f32>,
    stroke_opacity: Option<f32>,
}

impl Style {
    ///
    /// Parses a style from a
    ///
    pub fn from_style_value(style: &str) -> Self {
        let style       = format!("path {{ {} }}", style);
        let style       = StyleSheet::parse(&style);
        let rule        = &style.rules[0];

        Self::from_rule(&rule)
    }

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

    ///
    /// Creates a style from the attributes attached to a tag
    ///
    pub fn from_attributes(attributes: &HashMap<String, Value>) -> Self {
        let mut fill            = None;
        let mut stroke          = None;
        let mut line_width      = None;
        let mut fill_opacity    = None;
        let mut stroke_opacity  = None;

        for attr in attributes.iter() {
            match (attr.0.as_str(), attr.1) {
                ("font", _)                 => { }
                ("font-family", _)          => { }
                ("font-size", _)            => { }
                ("font-size-adjust", _)     => { }
                ("font-stretch", _)         => { }
                ("font-style", _)           => { }
                ("font-variant", _)         => { }
                ("font-weight", _)          => { }

                ("direction", _)            => { }
                ("letter-spacing", _)       => { }
                ("text-decoration", _)      => { }
                ("unicode-bidi", _)         => { }
                ("word-spacing", _)         => { }

                ("clip-path", _)            => { }
                ("clip-rule", _)            => { }
                ("mask", _)                 => { }
                ("opacity", _)              => { }

                ("enable-background", _)    => { }
                ("filter", _)               => { }
                ("flood-color", _)          => { }
                ("flood-opacity", _)        => { }
                ("lighting-color", _)       => { }

                ("stop-color", _)           => { }
                ("stop-opacity", _)         => { }

                ("color-interpolation", _)          => { }
                ("color-interpolation-filters", _)  => { }
                ("color-profile", _)                => { }
                ("color-rendering", _)              => { }
                ("fill", _)                         => { }
                ("fill-opacity", _)                 => { }
                ("fill-rule", _)                    => { }
                ("image-rendering", _)              => { }
                ("marker", _)                       => { }
                ("marker-end", _)                   => { }
                ("marker-mid", _)                   => { }
                ("marker-start", _)                 => { }
                ("shape-rendering", _)              => { }
                ("stroke", _)                       => { }
                ("stroke-dasharray", _)             => { }
                ("stroke-dashoffset", _)            => { }
                ("stroke-linecap", _)               => { }
                ("stroke-linejoin", _)              => { }
                ("stroke-miterlimit", _)            => { }
                ("stroke-opacity", _)               => { }
                ("text-rendering", _)               => { }

                ("alignment-baseline", _)               => { }
                ("baseline-shift", _)                   => { }
                ("dominant-baseline", _)                => { }
                ("glyph-orientation-horizontal", _)     => { }
                ("glyph-orientation-vertical", _)       => { }
                ("kerning", _)                          => { }
                ("text-anchor", _)                      => { }
                ("writing-mode", _)                     => { }

                _ => { }
            }
        }

        Self {
            fill, stroke, line_width, fill_opacity, stroke_opacity
        }
    }

    ///
    /// Merges this style with the 'merge_with' style (properties from merge_with will override the properties in this style)
    ///
    pub fn merge(&self, merge_with: &Self) -> Self {
        Self {
            fill:           merge_with.fill.as_ref().or(self.fill.as_ref()).cloned(),
            stroke:         merge_with.stroke.as_ref().or(self.stroke.as_ref()).cloned(),
            line_width:     merge_with.line_width.or(self.line_width),
            fill_opacity:   merge_with.fill_opacity.or(self.fill_opacity),
            stroke_opacity: merge_with.stroke_opacity.or(self.stroke_opacity),
        }
    }

    ///
    /// Given a drawing context with a path defined in it, renders a shape using this style
    ///
    pub fn render_path(&self, drawing: &mut impl GraphicsContext) {
        // Fill the shape first
        match (&self.fill, &self.fill_opacity) {
            (None, _) => { }

            (Some(FillStyle::Color(fill)), None) => {
                drawing.fill_color(*fill);
                drawing.fill();
            }

            (Some(FillStyle::Color(fill)), Some(opacity)) => {
                let (r, g, b, _a)   = fill.to_rgba_components();
                let fill            = Color::Rgba(r, g, b, *opacity);

                drawing.fill_color(fill);
                drawing.fill();
            }
        }

        // Draw the stroke next
        if self.stroke.is_some() {
            match &self.line_width {
                None        => { drawing.line_width(1.0); }
                Some(width) => { drawing.line_width(*width); }
            }

            match (&self.stroke, &self.stroke_opacity) {
                (None, _) => { }

                (Some(StrokeStyle::Solid(color)), None) => {
                    drawing.stroke_color(*color);
                    drawing.stroke();
                }

                (Some(StrokeStyle::Solid(color)), Some(opacity)) => {
                    let (r, g, b, _a)   = color.to_rgba_components();
                    let color           = Color::Rgba(r, g, b, *opacity);

                    drawing.stroke_color(color);
                    drawing.stroke();
                }
            }
        }
    }
}
