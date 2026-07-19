use crate::Color;
use crate::bindings;
use windows_core::Result;

/// A brush that paints a visual with a single solid color.
pub struct CompositionColorBrush(pub(crate) bindings::CompositionColorBrush);

impl CompositionColorBrush {
    /// Sets the brush's color.
    pub fn set_color(&self, color: Color) -> Result<()> {
        self.0.SetColor(color.0)
    }

    /// Returns the brush's color.
    pub fn color(&self) -> Result<Color> {
        Ok(Color(self.0.Color()?))
    }
}
