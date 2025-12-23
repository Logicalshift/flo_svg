///
/// Describes a SVG document
///
#[derive(Clone, Debug)]
pub struct SvgDocument {
    /// The region to display for this document
    pub (crate) viewbox: Option<((f32, f32), (f32, f32))>,

    /// x-axis coordinate of the region where the SVG is placed
    pub (crate) x: Option<f32>,

    /// y-axis coordinate of the region where the SVG is placed
    pub (crate) y: Option<f32>,

    /// Width of the svg
    pub (crate) width: Option<f32>,

    /// Height of the svg
    pub (crate) height: Option<f32>,
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
