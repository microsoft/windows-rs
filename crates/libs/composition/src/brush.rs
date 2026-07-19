use crate::Color;
use crate::bindings;
use crate::sealed::Sealed;
use windows_core::{Interface, Result};

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
    fn as_brush(&self) -> Result<CompositionBrush>;
}

/// A brush that paints with a single solid color.
#[derive(Clone)]
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

impl Sealed for CompositionColorBrush {}

impl Brush for CompositionColorBrush {
    fn as_brush(&self) -> Result<CompositionBrush> {
        Ok(CompositionBrush(self.0.cast()?))
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
    pub fn set_insets(&self, left: f32, top: f32, right: f32, bottom: f32) -> Result<()> {
        self.0.SetInsetsWithValues(left, top, right, bottom)
    }

    /// Sets whether the center of the grid is left unpainted (hollow).
    pub fn set_center_hollow(&self, hollow: bool) -> Result<()> {
        self.0.SetIsCenterHollow(hollow)
    }

    /// Sets the brush stretched across the nine-grid.
    pub fn set_source(&self, brush: &impl Brush) -> Result<()> {
        self.0.SetSource(&brush.as_brush()?.0)
    }
}

impl Sealed for CompositionNineGridBrush {}

impl Brush for CompositionNineGridBrush {
    fn as_brush(&self) -> Result<CompositionBrush> {
        Ok(CompositionBrush(self.0.cast()?))
    }
}
