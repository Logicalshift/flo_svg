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
