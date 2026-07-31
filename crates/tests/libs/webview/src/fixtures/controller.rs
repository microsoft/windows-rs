use crate::harness::Harness;

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

pub fn visibility_toggle(harness: &Harness) {
    let controller = harness.controller();
    harness.check("Controller_Hide", controller.set_visible(false).is_ok());
    harness.check("Controller_Show", controller.set_visible(true).is_ok());
}

pub fn background_color_round_trip(harness: &Harness) {
    use windows_webview::Color;

    let controller = harness.controller();
    let original = controller.default_background_color();
    let color = Color {
        r: 10,
        g: 20,
        b: 30,
        a: 255,
    };
    harness.check(
        "Controller_SetBackground",
        controller.set_default_background_color(color).is_ok(),
    );
    harness.check(
        "Controller_GetBackground",
        controller.default_background_color().ok() == Some(color),
    );
    if let Ok(original) = original {
        let _ = controller.set_default_background_color(original);
    }
}

pub fn rasterization_scale_round_trip(harness: &Harness) {
    let controller = harness.controller();
    let original = controller.rasterization_scale().unwrap_or(1.0);
    harness.check(
        "Controller_SetRasterization",
        controller.set_rasterization_scale(2.0).is_ok(),
    );
    harness.check(
        "Controller_GetRasterization",
        controller
            .rasterization_scale()
            .is_ok_and(|scale| (scale - 2.0).abs() < 1e-9),
    );
    let _ = controller.set_rasterization_scale(original);
}

pub fn detect_monitor_scale_changes_round_trip(harness: &Harness) {
    let controller = harness.controller();
    let original = controller
        .should_detect_monitor_scale_changes()
        .unwrap_or(true);
    harness.check(
        "Controller_SetDetectScale",
        controller
            .set_should_detect_monitor_scale_changes(false)
            .is_ok(),
    );
    harness.check(
        "Controller_GetDetectScale",
        controller.should_detect_monitor_scale_changes().ok() == Some(false),
    );
    let _ = controller.set_should_detect_monitor_scale_changes(original);
}
