//! Shared harness for the composition-interop examples.
//!
//! Each example is a small reactor app that hosts a lifted
//! `Microsoft.UI.Composition` visual tree via reactor's `composition_host`
//! widget and the [`windows-composition`](windows_composition) crate's
//! `reactor` feature (`CompositionHostExt`). Run one with, e.g.:
//!
//! ```text
//! cargo run -p reactor_composition --example circles
//! ```

use windows_reactor::*;

/// Boots the WinUI runtime and runs a reactor app with the given title.
pub fn run(title: &str, render: fn(&mut RenderCx) -> Element) -> Result<()> {
    bootstrap()?;
    App::new().title(title).render(render)
}
