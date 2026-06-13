//! Regression test that asserts every widget variant of `Element`
//! round-trips keyboard accelerator modifiers and that the reconciler
//! emits an `Op::SetKeyboardAccelerators` for them on mount.
//!
//! Background: roadmap item 1b (Keyboard accelerators). Accelerators
//! are plumbed through `Modifiers::keyboard_accelerators` rather than
//! per-widget struct fields, mirroring `Modifiers::accessibility`:
//! every backend `Handle` is castable to `UIElement` and
//! `UIElement.KeyboardAccelerators` is a flat collection that works
//! uniformly across every widget kind. This test enumerates every
//! `Element::*` widget variant to ensure that:
//!
//! 1. the builder method records into `Modifiers::keyboard_accelerators`;
//! 2. the reconciler emits a single `Op::SetKeyboardAccelerators`
//!    carrying the populated list on mount;
//! 3. round-tripping is exact (order preserved, no entry dropped).

use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::Callback;
use windows_reactor::ElementExt;
use windows_reactor::Reconciler;
use windows_reactor::RichText;
use windows_reactor::{
    Border, Button, CheckBox, Color, Element, Grid, GridLength, KeyModifiers, KeyboardAccelerator,
    KeyboardKey, ScrollViewer, StackPanel, TextBlock, TextBox,
};
use windows_reactor::{
    BreadcrumbBar, Canvas, ComboBox, Expander, HyperlinkButton, Image, InfoBadge, InfoBar,
    NavViewItem, NavigationView, NumberBox, PasswordBox, PersonPicture, Pivot, PivotItem,
    ProgressBar, ProgressRing, RadioButton, RadioButtons, Shape, Slider, TabItem, TabView,
    TitleBar, ToggleSwitch,
};
use windows_reactor::{Op, RecordingBackend};

fn one_of_every_widget() -> Vec<(&'static str, Element)> {
    vec![
        ("TextBlock", TextBlock::new("t").into()),
        ("Button", Button::new("b").into()),
        ("StackPanel", StackPanel::vertical().into()),
        ("Border", Border::new(Element::Empty).into()),
        ("CheckBox", CheckBox::new(false).into()),
        ("TextBox", TextBox::new("tf").into()),
        (
            "Grid",
            Grid {
                rows: vec![GridLength::STAR],
                columns: vec![GridLength::STAR],
                ..Grid::default()
            }
            .into(),
        ),
        ("ScrollViewer", ScrollViewer::new(Element::Empty).into()),
        ("ToggleSwitch", ToggleSwitch::new(false).into()),
        ("Slider", Slider::new(0.0).into()),
        ("RadioButton", RadioButton::new("r").into()),
        ("NumberBox", NumberBox::new(0.0).into()),
        ("ProgressBar", ProgressBar::new(50.0).into()),
        ("ProgressRing", ProgressRing::indeterminate().into()),
        ("Expander", Expander::new(Element::Empty).into()),
        ("HyperlinkButton", HyperlinkButton::new("h").into()),
        ("InfoBar", InfoBar::new("i").into()),
        ("InfoBadge", InfoBadge::dot().into()),
        ("PersonPicture", PersonPicture::new().into()),
        (
            "Shape",
            Shape::rectangle().fill(Color::rgb(255, 0, 0)).into(),
        ),
        ("Image", Image::new_with_uri("ms-appx:///x.png").into()),
        (
            "TabView",
            TabView::new([TabItem::new("a", TextBlock::new("x"))]).into(),
        ),
        (
            "NavigationView",
            NavigationView::new([NavViewItem::new("home")], Element::Empty).into(),
        ),
        ("TitleBar", TitleBar::new("title").into()),
        (
            "Pivot",
            Pivot::new([PivotItem::new("a", TextBlock::new("x"))]).into(),
        ),
        ("BreadcrumbBar", BreadcrumbBar::new(["root"]).into()),
        ("PasswordBox", PasswordBox::new().into()),
        ("RadioButtons", RadioButtons::new(["A", "B"]).into()),
        ("ComboBox", ComboBox::new(["A", "B"]).into()),
        ("Canvas", Canvas::new(std::iter::empty::<Element>()).into()),
        ("RichText", RichText::single_paragraph(Vec::new()).into()),
    ]
}

fn save_accel() -> KeyboardAccelerator {
    KeyboardAccelerator::new(KeyboardKey::S, KeyModifiers::CONTROL, || {})
}

#[test]
fn every_widget_variant_round_trips_keyboard_accelerators() {
    for (name, el) in one_of_every_widget() {
        let accel = save_accel();
        let labelled = el.keyboard_accelerator(accel.clone());
        let mods = labelled.modifiers().unwrap_or_else(|| {
            panic!("{name}: widget has no modifiers? — keyboard_accelerator should have recorded")
        });
        let list = &mods.keyboard_accelerators;
        assert!(
            !list.is_empty(),
            "{name}: keyboard_accelerator did not record into modifiers"
        );
        assert_eq!(list.len(), 1, "{name}: expected one accelerator");
        assert_eq!(list[0].key, KeyboardKey::S, "{name}: key");
        assert_eq!(
            list[0].modifiers,
            KeyModifiers::CONTROL,
            "{name}: modifiers"
        );
        // Cloned `Callback` is `Rc`-pointer-equal to its source, which
        // is how the diff'er recognises "same accelerator across two
        // renders".
        assert_eq!(list[0].on_invoked, accel.on_invoked);
    }
}

#[test]
fn every_widget_variant_emits_set_keyboard_accelerators_on_mount() {
    for (name, el) in one_of_every_widget() {
        let accel = save_accel();
        let labelled = el.keyboard_accelerator(accel.clone());
        let mut r = Reconciler::new(RecordingBackend::new());
        let id = r
            .reconcile(None, &labelled, None, Rc::new(|| {}))
            .unwrap_or_else(|| panic!("{name}: mount produced no control id"));

        let mut found = false;
        for op in &r.backend.ops {
            if let Op::SetKeyboardAccelerators {
                id: oid,
                accelerators,
            } = op
            {
                if *oid != id {
                    continue;
                }
                assert_eq!(accelerators.len(), 1, "{name}: list length");
                assert_eq!(accelerators[0].key, KeyboardKey::S);
                assert_eq!(accelerators[0].modifiers, KeyModifiers::CONTROL);
                found = true;
            }
        }
        assert!(found, "{name}: missing Op::SetKeyboardAccelerators");
    }
}

#[test]
fn multiple_accelerators_preserve_insertion_order() {
    let a = KeyboardAccelerator::new(KeyboardKey::S, KeyModifiers::CONTROL, || {});
    let b = KeyboardAccelerator::new(KeyboardKey::Escape, KeyModifiers::NONE, || {});
    let c = KeyboardAccelerator::new(
        KeyboardKey::F,
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        || {},
    );
    let el: Element = Button::new("b").into();
    let el = el
        .keyboard_accelerator(a)
        .keyboard_accelerator(b)
        .keyboard_accelerator(c);

    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = r.reconcile(None, &el, None, Rc::new(|| {}));
    let ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetKeyboardAccelerators { accelerators, .. } => Some(accelerators.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(ops.len(), 1);
    assert_eq!(ops[0].len(), 3);
    assert_eq!(ops[0][0].key, KeyboardKey::S);
    assert_eq!(ops[0][1].key, KeyboardKey::Escape);
    assert_eq!(ops[0][2].key, KeyboardKey::F);
    assert_eq!(
        ops[0][2].modifiers,
        KeyModifiers::CONTROL | KeyModifiers::SHIFT
    );
}

#[test]
fn empty_keyboard_accelerators_does_not_emit_op_on_mount() {
    // Sanity check: widgets without any accelerator must not pay for
    // an unnecessary `SetKeyboardAccelerators` op. Matches the
    // "no-op when empty" invariant exercised for accessibility.
    let el: Element = Button::new("b").into();
    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = r.reconcile(None, &el, None, Rc::new(|| {}));
    assert!(
        !r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::SetKeyboardAccelerators { .. })),
        "no SetKeyboardAccelerators op expected for widget without accelerators"
    );
}

#[test]
fn update_emits_set_keyboard_accelerators_when_modifiers_change() {
    // Add → Change → Clear. Each transition must emit exactly one
    // `SetKeyboardAccelerators` op carrying the new list (or the
    // empty list when cleared, so the WinUI backend can clear the
    // `UIElement.KeyboardAccelerators` collection).
    let plain: Element = Button::new("b").into();
    let accel_a = KeyboardAccelerator::new(KeyboardKey::S, KeyModifiers::CONTROL, || {});
    let accel_b = KeyboardAccelerator::new(KeyboardKey::Escape, KeyModifiers::NONE, || {});
    let labelled: Element = Button::new("b")
        .keyboard_accelerator(accel_a.clone())
        .into();
    let relabelled: Element = Button::new("b")
        .keyboard_accelerator(accel_a)
        .keyboard_accelerator(accel_b)
        .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &plain, None, Rc::new(|| {}))
        .expect("mount");
    assert!(
        !r.backend
            .ops
            .iter()
            .any(|op| matches!(op, Op::SetKeyboardAccelerators { .. })),
        "no op expected on initial mount without accelerators"
    );

    // Add an accelerator — update path should set it.
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&plain), &labelled, Some(id), Rc::new(|| {}));
    let ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetKeyboardAccelerators { accelerators, .. } => Some(accelerators.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(ops.len(), 1, "expected one SetKeyboardAccelerators on add");
    assert_eq!(ops[0].len(), 1);
    assert_eq!(ops[0][0].key, KeyboardKey::S);

    // Append a second accelerator — update path should re-set the
    // full list.
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&labelled), &relabelled, Some(id), Rc::new(|| {}));
    let ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetKeyboardAccelerators { accelerators, .. } => Some(accelerators.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(ops.len(), 1, "expected one op on append");
    assert_eq!(ops[0].len(), 2);
    assert_eq!(ops[0][1].key, KeyboardKey::Escape);

    // Clear all accelerators — update path should emit an empty
    // list so the backend can clear the WinUI collection.
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&relabelled), &plain, Some(id), Rc::new(|| {}));
    let ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetKeyboardAccelerators { accelerators, .. } => Some(accelerators.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(ops.len(), 1, "expected one op on clear");
    assert!(ops[0].is_empty(), "cleared payload should be empty list");
}

#[test]
fn stable_callback_identity_does_not_redundantly_emit_op() {
    // If the user returns the same `Callback<()>` (same `Rc`) across
    // two renders with no other change, the diff'er should treat the
    // accelerator list as unchanged and *not* emit a redundant op.
    // This mirrors how `accessibility` diffs by `PartialEq`.
    let cb = Callback::<()>::new(|()| {});
    let mk = || {
        Button::new("b").keyboard_accelerator(KeyboardAccelerator {
            key: KeyboardKey::S,
            modifiers: KeyModifiers::CONTROL,
            on_invoked: cb.clone(),
        })
    };
    let first: Element = mk().into();
    let second: Element = mk().into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r
        .reconcile(None, &first, None, Rc::new(|| {}))
        .expect("mount");
    r.backend.clear_ops();
    let _ = r.reconcile(Some(&first), &second, Some(id), Rc::new(|| {}));
    let ops: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::SetKeyboardAccelerators { .. }))
        .collect();
    assert!(
        ops.is_empty(),
        "expected no SetKeyboardAccelerators op when callback identity is stable, got {ops:?}"
    );
}

#[test]
fn callback_is_the_one_invoked_on_accelerator_fire() {
    // The `Callback<()>` round-trips through `Op::SetKeyboardAccelerators`
    // and invoking it from the captured payload runs the user's logic.
    // This is the closest we can come on the platform-agnostic
    // RecordingBackend to "the user pressed Ctrl+S".
    let fired = Rc::new(Cell::new(0_i32));
    let fired_c = Rc::clone(&fired);
    let accel = KeyboardAccelerator::new(KeyboardKey::S, KeyModifiers::CONTROL, move || {
        fired_c.set(fired_c.get() + 1);
    });
    let el: Element = Button::new("b").keyboard_accelerator(accel).into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let _ = r.reconcile(None, &el, None, Rc::new(|| {}));
    for op in &r.backend.ops {
        if let Op::SetKeyboardAccelerators { accelerators, .. } = op {
            for a in accelerators {
                a.on_invoked.invoke(());
            }
        }
    }
    assert_eq!(fired.get(), 1);
}
