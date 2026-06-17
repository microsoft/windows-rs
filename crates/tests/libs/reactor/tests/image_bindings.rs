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
