///
/// Describes a SVG document
///
#[derive(Clone, Debug)]
pub struct SvgDocument {
    /// The region to display for this document
    pub (crate) viewbox: Option<((f32, f32), (f32, f32))>,
}

impl Default for SvgDocument {
    fn default() -> Self {
        Self {
            viewbox: None,
        }
    }
}
