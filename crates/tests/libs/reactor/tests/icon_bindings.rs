use std::rc::Rc;

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Reconciler;
use windows_reactor::{Button, Element, Icon, ImageSource, Symbol};
use windows_reactor::{ControlId, Prop, PropValue};

fn noop_rr() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

fn mount(el: &Element) -> (Reconciler<RecordingBackend>, Option<ControlId>) {
    let mut r = Reconciler::new(RecordingBackend::new());
    let _id = r.reconcile(None, el, None, noop_rr());
    (r, _id)
}

fn icon_value(el: &Element) -> Option<PropValue> {
    let (r, _id) = mount(el);
    r.backend.ops.iter().find_map(|op| match op {
        Op::SetProp {
            prop: Prop::Icon,
            value,
            ..
        } => Some(value.clone()),
        _ => None,
    })
}

#[test]
fn symbol_shorthand_emits_symbol_icon() {
    let el: Element = Button::new("b").icon(Symbol::Home).into();
    assert_eq!(
        icon_value(&el),
        Some(PropValue::Icon(Icon::Symbol(Symbol::Home)))
    );
}

#[test]
fn image_icon_carries_source() {
    let el: Element = Button::new("b")
        .icon(Icon::image("ms-appx:///logo.svg"))
        .into();
    assert_eq!(
        icon_value(&el),
        Some(PropValue::Icon(Icon::Image(ImageSource::uri(
            "ms-appx:///logo.svg"
        ))))
    );
}

#[test]
fn image_source_converts_into_icon() {
    let source = ImageSource::uri("ms-appx:///logo.png");
    let el: Element = Button::new("b").icon(source.clone()).into();
    assert_eq!(icon_value(&el), Some(PropValue::Icon(Icon::Image(source))));
}

#[test]
fn bitmap_icon_carries_native_rendering_mode() {
    let el: Element = Button::new("b")
        .icon(Icon::bitmap_icon("ms-appx:///logo.png", true))
        .into();
    assert_eq!(
        icon_value(&el),
        Some(PropValue::Icon(Icon::Bitmap {
            uri: "ms-appx:///logo.png".into(),
            show_as_monochrome: true,
        }))
    );
}

#[test]
fn font_icon_carries_glyph_and_optional_family() {
    let default_font: Element = Button::new("b").icon(Icon::font("\u{E734}")).into();
    assert_eq!(
        icon_value(&default_font),
        Some(PropValue::Icon(Icon::Font {
            glyph: "\u{E734}".into(),
            family: None,
        }))
    );

    let with_family: Element = Button::new("b")
        .icon(Icon::font_family("\u{E734}", "Segoe Fluent Icons"))
        .into();
    assert_eq!(
        icon_value(&with_family),
        Some(PropValue::Icon(Icon::Font {
            glyph: "\u{E734}".into(),
            family: Some("Segoe Fluent Icons".into()),
        }))
    );
}

#[test]
fn path_icon_carries_geometry_data() {
    let el: Element = Button::new("b")
        .icon(Icon::path("F1 M 16,12 20,2L 20,16 1,16"))
        .into();
    assert_eq!(
        icon_value(&el),
        Some(PropValue::Icon(Icon::Path(
            "F1 M 16,12 20,2L 20,16 1,16".into()
        )))
    );
}

#[test]
fn no_icon_emits_no_icon_prop() {
    let el: Element = Button::new("b").into();
    assert_eq!(icon_value(&el), None);
}
