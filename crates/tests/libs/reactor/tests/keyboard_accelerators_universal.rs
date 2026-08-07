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

use test_reactor::{Op, RecordingBackend};
use windows_reactor::Callback;
use windows_reactor::InputExt;
use windows_reactor::Reconciler;
use windows_reactor::RichTextBlock;
use windows_reactor::{
    Border, Button, CheckBox, Color, Element, Grid, GridLength, KeyboardAccelerator, ScrollViewer,
    StackPanel, TextBlock, TextBox, VirtualKey, VirtualKeyModifiers,
};
use windows_reactor::{
    BreadcrumbBar, Canvas, ComboBox, Expander, HyperlinkButton, Image, InfoBadge, InfoBar,
    NavViewItem, NavigationView, NumberBox, PasswordBox, PersonPicture, Pivot, PivotItem,
    ProgressBar, ProgressRing, RadioButton, RadioButtons, Shape, Slider, TabItem, TabView,
    TitleBar, ToggleSwitch,
};

fn labelled<T: InputExt + Into<Element>>(widget: T) -> (Element, KeyboardAccelerator) {
    let accel = save_accel();
    (widget.keyboard_accelerator(accel.clone()).into(), accel)
}

fn one_of_every_widget() -> Vec<(&'static str, Element, KeyboardAccelerator)> {
    vec![
        ("TextBlock", labelled(TextBlock::new("t"))),
        ("Button", labelled(Button::new("b"))),
        ("StackPanel", labelled(StackPanel::vertical())),
        ("Border", labelled(Border::new(Element::Empty))),
        ("CheckBox", labelled(CheckBox::new(false))),
        ("TextBox", labelled(TextBox::new("tf"))),
        (
            "Grid",
            labelled(Grid {
                rows: vec![GridLength::STAR],
                columns: vec![GridLength::STAR],
                ..Grid::default()
            }),
        ),
        ("ScrollViewer", labelled(ScrollViewer::new(Element::Empty))),
        ("ToggleSwitch", labelled(ToggleSwitch::new(false))),
        ("Slider", labelled(Slider::new(0.0))),
        ("RadioButton", labelled(RadioButton::new("r"))),
        ("NumberBox", labelled(NumberBox::new(0.0))),
        ("ProgressBar", labelled(ProgressBar::new(50.0))),
        ("ProgressRing", labelled(ProgressRing::indeterminate())),
        ("Expander", labelled(Expander::new(Element::Empty))),
        ("HyperlinkButton", labelled(HyperlinkButton::new("h"))),
        ("InfoBar", labelled(InfoBar::new("i"))),
        ("InfoBadge", labelled(InfoBadge::dot())),
        ("PersonPicture", labelled(PersonPicture::new())),
        (
            "Shape",
            labelled(Shape::rectangle().fill(Color::rgb(255, 0, 0))),
        ),
        ("Image", labelled(Image::new_with_uri("ms-appx:///x.png"))),
        (
            "TabView",
            labelled(TabView::new([TabItem::new("a", TextBlock::new("x"))])),
        ),
        (
            "NavigationView",
            labelled(NavigationView::new(
                [NavViewItem::new("home")],
                Element::Empty,
            )),
        ),
        ("TitleBar", labelled(TitleBar::new("title"))),
        (
            "Pivot",
            labelled(Pivot::new([PivotItem::new("a", TextBlock::new("x"))])),
        ),
        ("BreadcrumbBar", labelled(BreadcrumbBar::new(["root"]))),
        ("PasswordBox", labelled(PasswordBox::new())),
        ("RadioButtons", labelled(RadioButtons::new(["A", "B"]))),
        ("ComboBox", labelled(ComboBox::new(["A", "B"]))),
        (
            "Canvas",
            labelled(Canvas::new(std::iter::empty::<Element>())),
        ),
        (
            "RichText",
            labelled(RichTextBlock::single_paragraph(Vec::new())),
        ),
    ]
    .into_iter()
    .map(|(name, (element, accelerator))| (name, element, accelerator))
    .collect()
}

fn save_accel() -> KeyboardAccelerator {
    KeyboardAccelerator::new(VirtualKey::S, VirtualKeyModifiers::Control, || {})
}

#[test]
fn every_widget_variant_round_trips_keyboard_accelerators() {
    for (name, element, accel) in one_of_every_widget() {
        let mods = element.modifiers().unwrap_or_else(|| {
            panic!("{name}: widget has no modifiers? — keyboard_accelerator should have recorded")
        });
        let list = &mods.keyboard_accelerators;
        assert!(
            !list.is_empty(),
            "{name}: keyboard_accelerator did not record into modifiers"
        );
        assert_eq!(list.len(), 1, "{name}: expected one accelerator");
        assert_eq!(list[0].key, VirtualKey::S, "{name}: key");
        assert_eq!(
            list[0].modifiers,
            VirtualKeyModifiers::Control,
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
    for (name, element, _) in one_of_every_widget() {
        let mut r = Reconciler::new(RecordingBackend::new());
        let id = r
            .reconcile(None, &element, None, Rc::new(|| {}))
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
                assert_eq!(accelerators[0].key, VirtualKey::S);
                assert_eq!(accelerators[0].modifiers, VirtualKeyModifiers::Control);
                found = true;
            }
        }
        assert!(found, "{name}: missing Op::SetKeyboardAccelerators");
    }
}

#[test]
fn multiple_accelerators_preserve_insertion_order() {
    let a = KeyboardAccelerator::new(VirtualKey::S, VirtualKeyModifiers::Control, || {});
    let b = KeyboardAccelerator::new(VirtualKey::Escape, VirtualKeyModifiers::None, || {});
    let c = KeyboardAccelerator::new(
        VirtualKey::F,
        VirtualKeyModifiers::Control | VirtualKeyModifiers::Shift,
        || {},
    );
    let el: Element = Button::new("b")
        .keyboard_accelerator(a)
        .keyboard_accelerator(b)
        .keyboard_accelerator(c)
        .into();

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
    assert_eq!(ops[0][0].key, VirtualKey::S);
    assert_eq!(ops[0][1].key, VirtualKey::Escape);
    assert_eq!(ops[0][2].key, VirtualKey::F);
    assert_eq!(
        ops[0][2].modifiers,
        VirtualKeyModifiers::Control | VirtualKeyModifiers::Shift
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
    // Clearing must emit an empty list so the backend clears the WinUI collection.
    let plain: Element = Button::new("b").into();
    let accel_a = KeyboardAccelerator::new(VirtualKey::S, VirtualKeyModifiers::Control, || {});
    let accel_b = KeyboardAccelerator::new(VirtualKey::Escape, VirtualKeyModifiers::None, || {});
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
    assert_eq!(ops[0][0].key, VirtualKey::S);

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
    assert_eq!(ops[0][1].key, VirtualKey::Escape);

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
            key: VirtualKey::S,
            modifiers: VirtualKeyModifiers::Control,
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
    let accel = KeyboardAccelerator::new(VirtualKey::S, VirtualKeyModifiers::Control, move || {
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
