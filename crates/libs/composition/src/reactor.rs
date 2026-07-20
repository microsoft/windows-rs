//! The reactor host bridge (feature `reactor`).
//!
//! [`windows-reactor`](windows_reactor)'s `CompositionHost` widget hosts a
//! lifted `Microsoft.UI.Composition` visual tree inside a WinUI element. Its
//! handle exposes only a raw `IInspectable` seam so that reactor need not depend
//! on this crate; the [`CompositionHostExt`] trait layers this crate's safe,
//! typed API on top of that seam.
//!
//! ```ignore
//! use windows_composition::CompositionHostExt;
//! # use windows_reactor::CompositionHostHandle;
//! # fn demo(host: CompositionHostHandle) -> windows_core::Result<()> {
//! let compositor = host.compositor()?;
//! let root = compositor.create_container_visual()?;
//! host.set_child_visual(&root)?;
//! # Ok(())
//! # }
//! ```

use crate::{Compositor, Visual};
use windows_core::Result;

/// Extends reactor's `CompositionHostHandle` with this crate's typed
/// composition API.
///
/// Import this trait to obtain the host element's [`Compositor`] and attach a
/// [`Visual`] tree built with the safe wrappers, instead of working with the
/// raw `IInspectable` seam directly.
pub trait CompositionHostExt {
    /// Returns the [`Compositor`] associated with the host element. Every visual
    /// attached with [`set_child_visual`](Self::set_child_visual) must be
    /// created from it.
    fn compositor(&self) -> Result<Compositor>;

    /// Attaches `visual` as the host element's child visual, replacing any
    /// visual attached previously.
    fn set_child_visual(&self, visual: &Visual) -> Result<()>;
}

impl CompositionHostExt for windows_reactor::CompositionHostHandle {
    fn compositor(&self) -> Result<Compositor> {
        Compositor::from_host(self.compositor_raw()?)
    }

    fn set_child_visual(&self, visual: &Visual) -> Result<()> {
        self.set_child_visual_raw(&visual.as_raw())
    }
}
