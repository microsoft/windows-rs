use crate::Visual;
use crate::bindings;
use windows_core::Interface;

/// A composition target that hosts a visual tree inside a window.
///
/// Created by
/// [`Compositor::create_desktop_window_target`](crate::Compositor::create_desktop_window_target).
/// Assign the tree's root with [`set_root`](Self::set_root); the target renders
/// its root for as long as it is kept alive.
pub struct DesktopWindowTarget(bindings::CompositionTarget);

impl DesktopWindowTarget {
    pub(crate) fn new(target: bindings::IDesktopWindowTarget) -> Self {
        Self(target.cast().unwrap())
    }

    /// Sets the root visual displayed by the target.
    pub fn set_root(&self, visual: &Visual) {
        self.0.SetRoot(&visual.0).unwrap();
    }

    /// Returns the target's current root visual, if any has been set.
    pub fn root(&self) -> Visual {
        Visual(self.0.Root().unwrap())
    }
}
