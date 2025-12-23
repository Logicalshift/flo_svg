///
/// How a distance is specified
///
#[derive(Copy, Clone, Debug)]
pub enum Length {
    /// Canvas coordinates
    Canvas(f32),

    /// Points (1/72nd of an inch)
    Points(f32),

    /// Percentage
    Percent(f32),
}

impl Length {
    /// Creates a length in mm
    pub fn mm(mm: f64) -> Length {
        Length::Points((mm / 25.4) as _)
    }

    /// Creates a length in inches
    pub fn inches(inches: f64) -> Length {
        Length::Points((inches / 72.0) as _)
    }

    /// Converts this length to canvas units ('total' can be none if percent lengths cannot be calculated)
    pub fn canvas_units(&self, total: Option<f32>) -> Option<f32> {
        let dpi = 72.0;

        match self {
            Length::Canvas(val)     => Some(*val),
            Length::Points(val)     => Some((val / 72.0) * dpi),
            Length::Percent(val)    => total.map(|total| total * val),
        }
    }
}
