use super::svg_reader_state::*;

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
            let name = decl.name.to_lowercase();
            match name.as_str() {
                "fill"              => { fill = Self::color(decl.value, &None).map(|col| FillStyle::Color(col)); }
                "fill-opacity"      => { fill_opacity = decl.value.parse::<f32>().ok(); }
                "fill-rule"         => { }

                "stroke"            => { stroke = Self::color(decl.value, &None).map(|col| StrokeStyle::Solid(col)); }
                "stroke-opacity"    => { stroke_opacity = decl.value.parse::<f32>().ok(); }
                "stroke-width"      => { line_width = Self::length(decl.value); }
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
        let mut current_color   = None;
        let mut fill            = None;
        let mut stroke          = None;
        let mut line_width      = None;
        let mut fill_opacity    = None;
        let mut stroke_opacity  = None;

        for attr in attributes.iter() {
            let name = attr.0.to_lowercase();

            match (name.as_str(), attr.1) {
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

                ("color", val)                      => { current_color = Self::color(val, &current_color); }
                ("color-interpolation", _)          => { }
                ("color-interpolation-filters", _)  => { }
                ("color-profile", _)                => { }
                ("color-rendering", _)              => { }
                ("fill", val)                       => { fill = Self::color(val, &current_color).map(|color| FillStyle::Color(color)); }
                ("fill-opacity", val)               => { fill_opacity = val.parse::<f32>().ok(); }
                ("fill-rule", _)                    => { }
                ("image-rendering", _)              => { }
                ("marker", _)                       => { }
                ("marker-end", _)                   => { }
                ("marker-mid", _)                   => { }
                ("marker-start", _)                 => { }
                ("shape-rendering", _)              => { }
                ("stroke", val)                     => { stroke = Self::color(val, &current_color).map(|color| StrokeStyle::Solid(color)); }
                ("stroke-dasharray", _)             => { }
                ("stroke-dashoffset", _)            => { }
                ("stroke-linecap", _)               => { }
                ("stroke-linejoin", _)              => { }
                ("stroke-miterlimit", _)            => { }
                ("stroke-opacity", val)             => { stroke_opacity = val.parse::<f32>().ok(); }
                ("stroke-width", val)               => { line_width = Self::length(val); }
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
    /// Changes an SVG value into a colour
    ///
    fn color(value: &str, current_color: &Option<Color>) -> Option<Color> {
        match value {
            "none"          => None,
            "currentColor"  => *current_color,

            _ => {
                let col = value.parse::<CssColor>().unwrap();
                Some(Color::Rgba((col.r as f32)/255.0, (col.g as f32)/255.0, (col.b as f32)/255.0, col.a))
            }
        }
    }

    ///
    /// Interprets a length to flo_canvas units
    ///
    fn length(value: &str) -> Option<f32> {
        let length = SvgReaderState::length(value);

        length.and_then(|length| length.canvas_units(None))
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
