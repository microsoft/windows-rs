//! Tests for `windows-composition`. See `docs/crates/windows-composition.md`.
//!
//! The `color` tests below cover the pure-value surface. The `live` module builds
//! a real `Compositor` on a dispatcher queue created on the test thread and
//! exercises the visual, brush, shape, and animation wrappers headlessly —
//! composition objects are constructed synchronously, so no window or message
//! pump is required. Hosting a tree in a window is exercised by the runnable
//! `composition/standalone` sample.

#![cfg(test)]

mod live;

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
