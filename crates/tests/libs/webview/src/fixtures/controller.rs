//! Controller display-property fixtures: zoom, rasterization scale, visibility.

use crate::harness::Harness;

/// The zoom factor round-trips through the setter and getter.
pub fn zoom_round_trip(harness: &Harness) {
    let controller = harness.controller();
    harness.check(
        "Controller_SetZoom",
        controller.set_zoom_factor(1.5).is_ok(),
    );
    harness.check(
        "Controller_GetZoom",
        (controller.zoom_factor() - 1.5).abs() < 1e-9,
    );
    let _ = controller.set_zoom_factor(1.0);
}

/// The rasterization scale reads back as a positive factor.
pub fn rasterization_scale(harness: &Harness) {
    match harness.controller().rasterization_scale() {
        Ok(scale) => harness.check("Controller_RasterizationScale", scale > 0.0),
        Err(_) => harness.check("Controller_RasterizationScale", false),
    }
}

/// Visibility can be toggled off and back on.
pub fn visibility_toggle(harness: &Harness) {
    let controller = harness.controller();
    harness.check("Controller_Hide", controller.set_visible(false).is_ok());
    harness.check("Controller_Show", controller.set_visible(true).is_ok());
}
