use crate::CompositionColorBrush;
use crate::bindings;
use windows_core::{Interface, Result};
use windows_numerics::{Vector2, Vector3};

/// The base type for every composition visual: a node in the visual tree with a
/// position, size, opacity, and visibility. Concrete visuals (`SpriteVisual`,
/// `ContainerVisual`) deref to `Visual`, so these operations are available on
/// them directly.
pub struct Visual(pub(crate) bindings::Visual);

impl Visual {
    /// Sets the visual's offset from its parent, in DIPs.
    pub fn set_offset(&self, x: f32, y: f32, z: f32) -> Result<()> {
        self.0.SetOffset(Vector3 { x, y, z })
    }

    /// Returns the visual's offset from its parent.
    pub fn offset(&self) -> Result<Vector3> {
        self.0.Offset()
    }

    /// Sets the visual's size, in DIPs.
    pub fn set_size(&self, width: f32, height: f32) -> Result<()> {
        self.0.SetSize(Vector2 {
            x: width,
            y: height,
        })
    }

    /// Returns the visual's size.
    pub fn size(&self) -> Result<Vector2> {
        self.0.Size()
    }

    /// Sets the visual's opacity in the range `0.0..=1.0`.
    pub fn set_opacity(&self, opacity: f32) -> Result<()> {
        self.0.SetOpacity(opacity)
    }

    /// Returns the visual's opacity.
    pub fn opacity(&self) -> Result<f32> {
        self.0.Opacity()
    }

    /// Sets whether the visual (and its subtree) is rendered.
    pub fn set_visible(&self, visible: bool) -> Result<()> {
        self.0.SetIsVisible(visible)
    }

    /// Returns whether the visual is visible.
    pub fn is_visible(&self) -> Result<bool> {
        self.0.IsVisible()
    }
}

/// A visual that hosts a child visual tree via its [`children`](Self::children).
pub struct ContainerVisual {
    visual: Visual,
    container: bindings::ContainerVisual,
}

impl ContainerVisual {
    pub(crate) fn new(container: bindings::ContainerVisual) -> Result<Self> {
        Ok(Self {
            visual: Visual(container.cast()?),
            container,
        })
    }

    /// Returns the collection of child visuals.
    pub fn children(&self) -> Result<VisualCollection> {
        Ok(VisualCollection(self.container.Children()?))
    }
}

impl core::ops::Deref for ContainerVisual {
    type Target = Visual;
    fn deref(&self) -> &Visual {
        &self.visual
    }
}

/// A visual that paints its bounds with a [`CompositionColorBrush`] (and, in
/// future, surface and effect brushes). Also a container.
pub struct SpriteVisual {
    visual: Visual,
    sprite: bindings::SpriteVisual,
}

impl SpriteVisual {
    pub(crate) fn new(sprite: bindings::SpriteVisual) -> Result<Self> {
        Ok(Self {
            visual: Visual(sprite.cast()?),
            sprite,
        })
    }

    /// Sets the brush that fills the visual's bounds.
    pub fn set_brush(&self, brush: &CompositionColorBrush) -> Result<()> {
        self.sprite.SetBrush(&brush.0)
    }
}

impl core::ops::Deref for SpriteVisual {
    type Target = Visual;
    fn deref(&self) -> &Visual {
        &self.visual
    }
}

/// An ordered collection of child visuals under a [`ContainerVisual`].
pub struct VisualCollection(pub(crate) bindings::VisualCollection);

impl VisualCollection {
    /// Returns the number of visuals in the collection.
    pub fn count(&self) -> Result<i32> {
        self.0.Count()
    }

    /// Inserts a visual at the top of the z-order (drawn last, on top).
    pub fn insert_at_top(&self, visual: &Visual) -> Result<()> {
        self.0.InsertAtTop(&visual.0)
    }

    /// Inserts a visual at the bottom of the z-order (drawn first, behind).
    pub fn insert_at_bottom(&self, visual: &Visual) -> Result<()> {
        self.0.InsertAtBottom(&visual.0)
    }

    /// Removes a visual from the collection.
    pub fn remove(&self, visual: &Visual) -> Result<()> {
        self.0.Remove(&visual.0)
    }

    /// Removes every visual from the collection.
    pub fn remove_all(&self) -> Result<()> {
        self.0.RemoveAll()
    }
}
