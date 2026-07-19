//! Tests for `windows-composition`. See `docs/crates/windows-composition.md`.
//!
//! Runtime tests that build a live `Compositor` are deferred until a standalone
//! construction path (dispatcher queue + WinAppSDK bootstrap) lands; a live
//! compositor otherwise requires a hosting element supplied by the reactor seam.
//! For now these tests cover the pure-value surface.

#![cfg(test)]

use windows_composition::Color;

#[test]
fn color_rgb_is_opaque() {
    let c = Color::rgb(10, 20, 30);
    assert_eq!(c, Color::rgba(10, 20, 30, 255));
}

#[test]
fn color_rgba_roundtrips_components() {
    assert_eq!(Color::rgba(1, 2, 3, 4), Color::rgba(1, 2, 3, 4));
    assert_ne!(Color::rgba(1, 2, 3, 4), Color::rgba(4, 3, 2, 1));
}
