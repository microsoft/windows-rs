use std::rc::Rc;

use windows_reactor::Reconciler;
use windows_reactor::{Brush, Color, Element};
use windows_reactor::{ControlKind, Op, Prop, PropValue, RecordingBackend};
use windows_reactor::{HyperlinkButton, Image, ProgressBar, ProgressRing, Shape, Stretch};

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
        .stretch(Stretch::UniformToFill)
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
                prop: Prop::Stretch,
                value: PropValue::I32(v),
                ..
            } if *v == Stretch::UniformToFill.0
        )
    });
    assert!(saw_stretch);
}

#[test]
fn image_default_emits_no_bindings() {
    let el: Element = Image::default().into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::Image);
    // Default Image has no source; Stretch is always emitted (defaults to Uniform).
    let prop_ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::SetProp { .. }))
        .collect();
    assert_eq!(prop_ops.len(), 1);
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
                prop: Prop::Value,
                value: PropValue::F64(50.0),
                ..
            }
        )
    });
    assert!(saw_val);
}

#[test]
fn progress_bar_mounts_with_minimum_maximum_and_indeterminate() {
    let el: Element = ProgressBar {
        value: 25.0,
        minimum: 10.0,
        maximum: 90.0,
        is_indeterminate: true,
        ..ProgressBar::default()
    }
    .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::ProgressBar);

    let mut saw_min = false;
    let mut saw_max = false;
    let mut saw_indeterminate = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Minimum, PropValue::F64(10.0)) => saw_min = true,
                (Prop::Maximum, PropValue::F64(90.0)) => saw_max = true,
                (Prop::IsIndeterminate, PropValue::Bool(true)) => saw_indeterminate = true,
                _ => {}
            }
        }
    }
    assert!(saw_min && saw_max && saw_indeterminate);
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
fn progress_ring_mounts_with_value_range_and_active() {
    let el: Element = ProgressRing {
        value: 42.0,
        minimum: 5.0,
        maximum: 80.0,
        is_indeterminate: false,
        is_active: true,
        ..ProgressRing::default()
    }
    .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::ProgressRing);

    let mut saw_value = false;
    let mut saw_min = false;
    let mut saw_max = false;
    let mut saw_active = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::Value, PropValue::F64(42.0)) => saw_value = true,
                (Prop::Minimum, PropValue::F64(5.0)) => saw_min = true,
                (Prop::Maximum, PropValue::F64(80.0)) => saw_max = true,
                (Prop::IsActive, PropValue::Bool(true)) => saw_active = true,
                _ => {}
            }
        }
    }
    assert!(saw_value && saw_min && saw_max && saw_active);
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
        matches!(op, Op::SetProp { prop: Prop::Content, value: PropValue::Str(s), .. } if s == "Docs")
    });
    assert!(saw_label);
    let saw_uri = r.backend.ops.iter().any(|op| {
        matches!(op, Op::SetProp { prop: Prop::NavigateUri, value: PropValue::Str(s), .. } if s == "https://example.com/docs")
    });
    assert!(saw_uri);
}

fn first_create(r: &Reconciler<RecordingBackend>) -> (ControlKind, windows_reactor::ControlId) {
    r.backend
        .ops
        .iter()
        .find_map(|op| match op {
            Op::Create { id, kind } => Some((*kind, *id)),
            _ => None,
        })
        .expect("no Create op")
}
