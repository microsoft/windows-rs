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
    /// Creates a standalone compositor.
    ///
    /// Because this crate targets the **lifted** `Microsoft.UI.Composition`
    /// stack (a Windows App SDK component, not an OS component), a standalone
    /// compositor has two prerequisites:
    ///
    /// 1. **The Windows App SDK runtime must be bootstrapped** before this call.
    ///    Add [`windows-reactor-setup`] to your `build.rs` (its staging is
    ///    generic Windows App SDK setup despite the reactor-centric name).
    ///    Reactor-hosted apps already satisfy this — there, adopt the element's
    ///    compositor with [`from_raw`](Self::from_raw) instead of `new`.
    /// 2. **A dispatcher queue must exist on the current thread.** Create a
    ///    [`DispatcherQueueController`](crate::DispatcherQueueController) first
    ///    and keep it alive for the compositor's lifetime.
    ///
    /// ```no_run
    /// use windows_composition::{Compositor, DispatcherQueueController};
    ///
    /// let _queue = DispatcherQueueController::create_on_current_thread()?;
    /// let compositor = Compositor::new()?;
    /// # windows_core::Result::Ok(())
    /// ```
    ///
    /// [`windows-reactor-setup`]: https://docs.rs/windows-reactor-setup
    pub fn new() -> Result<Self> {
        Ok(Self(bindings::Compositor::new()?))
    }

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
