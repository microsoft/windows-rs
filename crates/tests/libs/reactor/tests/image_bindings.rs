use windows_reactor::{Binding, Image, Prop, PropValue, Widget};

fn image_source(image: &Image) -> Option<PropValue> {
    image
        .bindings()
        .into_iter()
        .find_map(|binding| match binding {
            Binding::Prop(Prop::ImageSource, value) => Some(value),
            _ => None,
        })
}

#[test]
fn new_emits_uri_image_source() {
    let image = Image::new_with_uri("file:///pic.png");
    assert_eq!(
        image_source(&image),
        Some(PropValue::Str("file:///pic.png".into()))
    );
}

#[test]
fn default_emits_no_image_source() {
    let image = Image::default();
    assert_eq!(image_source(&image), None);
    assert!(
        !image
            .bindings()
            .iter()
            .any(|binding| matches!(binding, Binding::Prop(Prop::ImageSource, ..)))
    );
}

#[test]
fn on_mounted_stores_callback() {
    // An on_mounted callback makes the element differ from one without it.
    use windows_reactor::Element;
    let with_cb: Element = Image::new_with_uri("file:///pic.png")
        .on_mounted(|_| {})
        .into();
    let without_cb: Element = Image::new_with_uri("file:///pic.png").into();
    assert_ne!(with_cb, without_cb);
}

#[test]
fn on_mounted_fires_once_at_mount() {
    // The `Image` widget must dispatch its `on_mounted` handle at mount, proving
    // the `Widget::on_mounted_callback` override is wired for the DPI-scale hook.
    use std::cell::Cell;
    use std::rc::Rc;
    use test_reactor::RecordingBackend;
    use windows_reactor::{Element, Reconciler};

    let fired = Rc::new(Cell::new(0));
    let fired2 = fired.clone();
    let image: Element = Image::new_with_uri("file:///pic.png")
        .on_mounted(move |_| fired2.set(fired2.get() + 1))
        .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    r.reconcile(None, &image, None, Rc::new(|| {}))
        .expect("image mounts");

    assert_eq!(fired.get(), 1, "on_mounted fires exactly once at mount");
}
