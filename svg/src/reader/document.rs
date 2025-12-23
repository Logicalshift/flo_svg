use super::length::*;

///
/// Describes a SVG document
///
#[derive(Clone, Debug)]
pub struct SvgDocument {
    /// The region to display for this document
    pub (crate) viewbox: Option<((f32, f32), (f32, f32))>,

    /// x-axis coordinate of the region where the SVG is placed
    pub (crate) x: Option<Length>,

    /// y-axis coordinate of the region where the SVG is placed
    pub (crate) y: Option<Length>,

    /// Width of the svg
    pub (crate) width: Option<Length>,

    /// Height of the svg
    pub (crate) height: Option<Length>,
}

impl Default for SvgDocument {
    fn default() -> Self {
        Self {
            viewbox:    None,
            x:          None,
            y:          None,
            width:      None,
            height:     None,
        }
    }
}

impl SvgDocument {
    #[inline] pub fn viewbox(&self) -> Option<((f32, f32), (f32, f32))> { self.viewbox }
    #[inline] pub fn x(&self) -> Option<Length>                         { self.x }
    #[inline] pub fn y(&self) -> Option<Length>                         { self.x }
    #[inline] pub fn width(&self) -> Option<Length>                     { self.width }
    #[inline] pub fn height(&self) -> Option<Length>                    { self.height }

    ///
    /// Returns the desired render width of this document (in canvas units), if specified
    ///
    pub fn render_width(&self) -> Option<f32> {
        if let Some(viewbox) = self.viewbox {
            Some(viewbox.1.0 - viewbox.0.0)
        } else if let Some(width) = self.width {
            width.canvas_units(None)
        } else {
            None
        }
    }

    ///
    /// Returns the desired render height of this document (in canvas units), if specified
    ///
    pub fn render_height(&self) -> Option<f32> {
        if let Some(viewbox) = self.viewbox {
            Some(viewbox.1.1 - viewbox.0.1)
        } else if let Some(height) = self.height {
            height.canvas_units(None)
        } else {
            None
        }
    }
}