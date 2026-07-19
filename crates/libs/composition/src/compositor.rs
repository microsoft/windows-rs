use crate::bindings;
use crate::{Color, CompositionColorBrush, ContainerVisual, SpriteVisual};
use windows_core::{Interface, Result};

/// The composition engine — the factory for visuals and brushes.
///
/// A `Compositor` is obtained by adopting the compositor that owns a hosting
/// element's visual (via the reactor seam); every visual and brush it creates
/// belongs to the same compositor and can only be combined with its siblings.
pub struct Compositor(pub(crate) bindings::Compositor);

impl Compositor {
    /// Adopts a compositor obtained from a hosting element (e.g. the reactor
    /// seam's `element_compositor`). The value must be a
    /// `Microsoft.UI.Composition.Compositor`.
    pub fn from_raw(compositor: windows_core::IInspectable) -> Result<Self> {
        Ok(Self(compositor.cast()?))
    }

    /// Creates an empty container visual that hosts a child visual tree.
    pub fn create_container_visual(&self) -> Result<ContainerVisual> {
        ContainerVisual::new(self.0.CreateContainerVisual()?)
    }

    /// Creates a sprite visual that paints itself with a brush.
    pub fn create_sprite_visual(&self) -> Result<SpriteVisual> {
        SpriteVisual::new(self.0.CreateSpriteVisual()?)
    }

    /// Creates a solid-color brush.
    pub fn create_color_brush(&self, color: Color) -> Result<CompositionColorBrush> {
        Ok(CompositionColorBrush(
            self.0.CreateColorBrushWithColor(color.0)?,
        ))
    }
}
