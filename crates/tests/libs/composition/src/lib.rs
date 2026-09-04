//! Tests for `windows-composition`.

#![cfg(test)]

mod live;

use windows_composition::CompositionColor;

#[test]
fn color_rgb_is_opaque() {
    let c = CompositionColor::rgb(10, 20, 30);
    assert_eq!(c, CompositionColor::rgba(10, 20, 30, 255));
}

#[test]
fn color_rgba_roundtrips_components() {
    assert_eq!(
        CompositionColor::rgba(1, 2, 3, 4),
        CompositionColor::rgba(1, 2, 3, 4)
    );
    assert_ne!(
        CompositionColor::rgba(1, 2, 3, 4),
        CompositionColor::rgba(4, 3, 2, 1)
    );
}
