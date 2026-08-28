use super::super::*;
use crate::native::*;

#[test]
fn resource_overrides_mount_update_and_clear() {
    let initial = ResourceOverrides::new()
        .set("ButtonBackground", Color::rgb(178, 34, 34))
        .set("ButtonBorderThemeThickness", Thickness::uniform(0.0));
    let replacement = ResourceOverrides::new()
        .set("ButtonForeground", Color::rgb(255, 255, 255))
        .set("ControlCornerRadius", CornerRadius::uniform(8.0));
    let mut pump = Pump::new(RecordingRuntime::default());
    pump.mount(Button::new().resource_overrides(initial.clone()).into())
        .unwrap();
    let root = pump.root().unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::ButtonResources),
        Some(&PropertyValue::ResourceOverrides(initial))
    );

    pump.update(Button::new().resource_overrides(replacement.clone()).into())
        .unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::ButtonResources),
        Some(&PropertyValue::ResourceOverrides(replacement))
    );

    pump.update(Button::new().into()).unwrap();
    assert_eq!(
        pump.runtime()
            .node(root)
            .unwrap()
            .property(PropertyId::ButtonResources),
        None
    );
}
