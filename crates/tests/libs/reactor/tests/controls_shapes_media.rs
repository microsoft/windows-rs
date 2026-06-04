use std::rc::Rc;

use windows_reactor::core::backend::{ControlKind, Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::element::{Brush, Color, Element};
use windows_reactor::core::element::{
    HyperlinkButton, Image, ImageStretch, ProgressBar, ProgressRing, Shape,
};
use windows_reactor::core::reconciler::Reconciler;

fn mount(el: &Element) -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    r.reconcile(None, el, None, Rc::new(|| {}));
    r
}

#[test]
fn rectangle_mounts_with_fill_and_corner_radius() {
    let el: Element = Shape::rectangle()
        .fill_rgb(40, 120, 200)
        .corner_radius(8.0)
        .stroke_thickness(2.0)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::Rectangle);

    let saw_fill = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::Fill,
                value: PropValue::Brush(Brush::Solid(_)),
                ..
            }
        )
    });
    assert!(saw_fill);
    let saw_radius = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::CornerRadius,
                value: PropValue::F64(8.0),
                ..
            }
        )
    });
    assert!(saw_radius);
    let saw_stroke = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::StrokeThickness,
                value: PropValue::F64(2.0),
                ..
            }
        )
    });
    assert!(saw_stroke);
}

#[test]
fn ellipse_maps_to_ellipse_kind() {
    let el: Element = Shape::ellipse().fill(Color::rgb(1, 2, 3)).into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::Ellipse);
}

#[test]
fn line_maps_to_line_kind_and_sets_endpoints() {
    let el: Element = Shape::line(0.0, 0.0, 100.0, 20.0)
        .stroke_thickness(1.5)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::Line);

    let saw_endpoints = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::LineEndpoints,
                ..
            }
        )
    });
    assert!(saw_endpoints);
}

#[test]
fn shape_kind_change_remounts_control() {
    let rect: Element = Shape::rectangle().fill(Color::rgb(0, 128, 255)).into();
    let ell: Element = Shape::ellipse().fill(Color::rgb(0, 128, 255)).into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &rect, None, Rc::new(|| {}))
        .expect("mount Rectangle");
    let (kind0, id0) = first_create(&r);
    assert_eq!(kind0, ControlKind::Rectangle);
    assert_eq!(id0, id);

    let prefix = r.backend.ops.len();
    let new_id = r
        .reconcile(Some(&rect), &ell, Some(id), Rc::new(|| {}))
        .expect("update Rectangle → Ellipse");
    let window = &r.backend.ops[prefix..];

    let ellipse_create = window.iter().any(|op| {
        matches!(
            op,
            Op::Create {
                kind: ControlKind::Ellipse,
                ..
            }
        )
    });
    assert!(
        ellipse_create,
        "audit §7.1.5: Rectangle → Ellipse must mount a fresh Ellipse \
         control; ops in swap window: {window:#?}"
    );

    let _ = new_id;
}

#[test]
fn image_mounts_with_source_and_stretch() {
    let el: Element = Image::new_with_uri("ms-appx:///Assets/logo.png")
        .stretch(ImageStretch::UniformToFill)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::Image);

    let saw_src = r.backend.ops.iter().any(|op| {
        matches!(op, Op::SetProp { prop: Prop::ImageSource, value: PropValue::Str(s), .. } if s == "ms-appx:///Assets/logo.png")
    });
    assert!(saw_src);
    let saw_stretch = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::ImageStretch,
                value: PropValue::ImageStretch(ImageStretch::UniformToFill),
                ..
            }
        )
    });
    assert!(saw_stretch);
}

#[test]
fn progress_bar_determinate_sets_value() {
    let el: Element = ProgressBar::new(50.0).into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::ProgressBar);

    let saw_val = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::NumericValue,
                value: PropValue::F64(50.0),
                ..
            }
        )
    });
    assert!(saw_val);
}

#[test]
fn progress_ring_indeterminate_sets_flag() {
    let el: Element = ProgressRing::indeterminate().into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::ProgressRing);

    let saw = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::IsIndeterminate,
                value: PropValue::Bool(true),
                ..
            }
        )
    });
    assert!(saw);
}

#[test]
fn hyperlink_mounts_with_label_and_uri() {
    let el: Element = HyperlinkButton::new("Docs")
        .navigate_uri("https://example.com/docs")
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::HyperlinkButton);

    let saw_label = r.backend.ops.iter().any(|op| {
        matches!(op, Op::SetProp { prop: Prop::ButtonContent, value: PropValue::Str(s), .. } if s == "Docs")
    });
    assert!(saw_label);
    let saw_uri = r.backend.ops.iter().any(|op| {
        matches!(op, Op::SetProp { prop: Prop::NavigateUri, value: PropValue::Str(s), .. } if s == "https://example.com/docs")
    });
    assert!(saw_uri);
}

fn first_create(
    r: &Reconciler<RecordingBackend>,
) -> (ControlKind, windows_reactor::core::backend::ControlId) {
    r.backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::Create { id, kind } => Some((*kind, *id)),
            _ => None,
        })
        .expect("no Create op")
}
