use super::*;

/// The base type shared by every composition brush. A [`Brush`] can be turned
/// into one via [`Brush::as_brush`] to paint a visual or fill a shape.
#[derive(Clone)]
pub struct CompositionBrush(pub(crate) bindings::CompositionBrush);

/// A brush that can paint a [`SpriteVisual`](crate::SpriteVisual), fill a
/// [`CompositionSpriteShape`](crate::CompositionSpriteShape), or serve as the
/// source of a [`CompositionNineGridBrush`].
///
/// This trait is sealed: only the brush types in this crate implement it.
pub trait Brush: Sealed {
    /// Returns this brush as the shared [`CompositionBrush`] base type.
    fn as_brush(&self) -> CompositionBrush;
}

/// A brush that paints with a single solid color.
#[derive(Clone)]
pub struct CompositionColorBrush(pub(crate) bindings::CompositionColorBrush);

impl CompositionColorBrush {
    /// Sets the brush's color.
    pub fn set_color(&self, color: Color) {
        self.0.SetColor(color.0).unwrap();
    }

    /// Returns the brush's color.
    pub fn color(&self) -> Color {
        Color(self.0.Color().unwrap())
    }
}

impl Sealed for CompositionColorBrush {}

impl Brush for CompositionColorBrush {
    fn as_brush(&self) -> CompositionBrush {
        CompositionBrush(self.0.cast().unwrap())
    }
}

/// A brush that stretches its source as a nine-grid: the four corners keep their
/// size, the edges stretch along one axis, and the center stretches (or is left
/// [hollow](Self::set_center_hollow)) to fill the remaining space. Used here to
/// draw a hollow selection border.
#[derive(Clone)]
pub struct CompositionNineGridBrush(pub(crate) bindings::CompositionNineGridBrush);

impl CompositionNineGridBrush {
    /// Sets the left, top, right, and bottom inset widths, in DIPs.
    pub fn set_insets(&self, left: f32, top: f32, right: f32, bottom: f32) {
        self.0
            .SetInsetsWithValues(left, top, right, bottom)
            .unwrap();
    }

    /// Sets whether the center of the grid is left unpainted (hollow).
    pub fn set_center_hollow(&self, hollow: bool) {
        self.0.SetIsCenterHollow(hollow).unwrap();
    }

    /// Sets the brush stretched across the nine-grid.
    pub fn set_source(&self, brush: &impl Brush) {
        self.0.SetSource(&brush.as_brush().0).unwrap();
    }
}

impl Sealed for CompositionNineGridBrush {}

impl Brush for CompositionNineGridBrush {
    fn as_brush(&self) -> CompositionBrush {
        CompositionBrush(self.0.cast().unwrap())
    }
}
