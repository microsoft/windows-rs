use std::rc::Rc;

use std::cell::Cell;

use windows_reactor::Element;
use windows_reactor::Reconciler;
use windows_reactor::Thickness;
use windows_reactor::text_block;
use windows_reactor::{
    Border, Expander, Grid, GridLength, ScrollBarVisibility, ScrollView, ScrollViewer,
    ScrollingScrollBarVisibility, SplitView, Stretch, Viewbox,
};
use windows_reactor::{ControlKind, Event, Prop, PropValue};
use windows_reactor::{Op, RecordingBackend};

fn mount(el: &Element) -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    r.reconcile(None, el, None, Rc::new(|| {}));
    r
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

#[test]
fn expander_mounts_with_header_and_collapses_by_default() {
    let el: Element = Expander::new(text_block("hidden")).header("Details").into();
    let r = mount(&el);

    let creates: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::Create {
                    kind: ControlKind::Expander,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(creates.len(), 1);

    let header_set = r.backend.ops.iter().any(|op| {
        matches!(op, Op::SetProp { prop: Prop::Header, value: PropValue::Str(s), .. } if s == "Details")
    });
    assert!(header_set);

    // IsExpanded defaults to false. With always-emit, the binding is always
    // present so that the diff engine can track changes.
    let expanded_set = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::IsExpanded,
                value: PropValue::Bool(false),
                ..
            }
        )
    });
    assert!(expanded_set);
}

#[test]
fn expander_update_diffs_is_expanded() {
    let old: Element = Expander::new(text_block("x")).into();
    let new: Element = Expander::new(text_block("x")).expanded(true).into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &old, None, Rc::new(|| {})).unwrap();
    r.backend.clear_ops();
    r.reconcile(Some(&old), &new, Some(id), Rc::new(|| {}));

    let changes: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::IsExpanded,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(changes.len(), 1);
}

#[test]
fn expander_child_is_mounted_as_inner_control() {
    let el: Element = Expander::new(text_block("inner")).into();
    let r = mount(&el);

    let creates: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::Create { .. }))
        .collect();
    assert_eq!(creates.len(), 2, "expected Expander + its child");
}

#[test]
fn grid_mounts_with_rows_columns_and_spacing() {
    let el: Element = Grid::default()
        .rows(vec![GridLength::Star(1.0)])
        .columns(vec![GridLength::Auto])
        .row_spacing(4.0)
        .column_spacing(8.0)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::Grid);

    let mut saw_rows = false;
    let mut saw_columns = false;
    let mut saw_row_spacing = false;
    let mut saw_column_spacing = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::GridRows, PropValue::GridLengths(v))
                    if v == &vec![GridLength::Star(1.0)] =>
                {
                    saw_rows = true;
                }
                (Prop::GridColumns, PropValue::GridLengths(v)) if v == &vec![GridLength::Auto] => {
                    saw_columns = true;
                }
                (Prop::RowSpacing, PropValue::F64(4.0)) => saw_row_spacing = true,
                (Prop::ColumnSpacing, PropValue::F64(8.0)) => saw_column_spacing = true,
                _ => {}
            }
        }
    }
    assert!(saw_rows && saw_columns && saw_row_spacing && saw_column_spacing);
}

#[test]
fn border_mounts_with_corner_radius_and_border_thickness() {
    let el: Element = Border::new(text_block("inner"))
        .corner_radius(8.0)
        .border_thickness(Thickness::uniform(2.0))
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::Border);

    let mut saw_radius = false;
    let mut saw_thickness = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::CornerRadius, PropValue::F64(8.0)) => saw_radius = true,
                (Prop::BorderThickness, PropValue::Thickness(v))
                    if *v == Thickness::uniform(2.0) =>
                {
                    saw_thickness = true;
                }
                _ => {}
            }
        }
    }
    assert!(saw_radius && saw_thickness);
}

#[test]
fn split_view_mounts_with_pane_lengths_and_open_state() {
    let el: Element = SplitView::new(text_block("content"))
        .pane(text_block("pane"))
        .is_pane_open(true)
        .open_pane_length(300.0)
        .compact_pane_length(48.0)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::SplitView);

    let mut saw_open = false;
    let mut saw_open_len = false;
    let mut saw_compact_len = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::IsPaneOpen, PropValue::Bool(true)) => saw_open = true,
                (Prop::OpenPaneLength, PropValue::F64(300.0)) => saw_open_len = true,
                (Prop::CompactPaneLength, PropValue::F64(48.0)) => saw_compact_len = true,
                _ => {}
            }
        }
    }
    assert!(saw_open && saw_open_len && saw_compact_len);
}

#[test]
fn split_view_pane_closed_event_fires() {
    let fired = Rc::new(Cell::new(0));
    let fired_c = Rc::clone(&fired);
    let el: Element = SplitView::new(text_block("content"))
        .pane(text_block("pane"))
        .on_pane_closed(move || fired_c.set(fired_c.get() + 1))
        .into();
    let r = mount(&el);

    let (_, id) = first_create(&r);
    r.backend.fire(id, Event::PaneClosed);
    assert_eq!(fired.get(), 1);
}

#[test]
fn viewbox_mounts_with_stretch() {
    let el: Element = Viewbox::new(text_block("content"))
        .stretch(Stretch::Fill)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::Viewbox);

    let saw_stretch = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::Stretch,
                value: PropValue::I32(v),
                ..
            } if *v == Stretch::Fill.0
        )
    });
    assert!(saw_stretch);
}

#[test]
fn scroll_view_mounts_with_horizontal_and_vertical_visibility() {
    let el: Element = ScrollView::new(text_block("content"))
        .horizontal_scroll_bar_visibility(ScrollingScrollBarVisibility::Hidden)
        .vertical_scroll_bar_visibility(ScrollingScrollBarVisibility::Visible)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::ScrollView);

    let mut saw_horizontal = false;
    let mut saw_vertical = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::HorizontalScrollBarVisibility, PropValue::I32(v))
                    if *v == ScrollingScrollBarVisibility::Hidden.0 =>
                {
                    saw_horizontal = true;
                }
                (Prop::VerticalScrollBarVisibility, PropValue::I32(v))
                    if *v == ScrollingScrollBarVisibility::Visible.0 =>
                {
                    saw_vertical = true;
                }
                _ => {}
            }
        }
    }
    assert!(saw_horizontal && saw_vertical);
}

#[test]
fn scroll_viewer_mounts_with_horizontal_and_vertical_visibility() {
    let el: Element = ScrollViewer::new(text_block("content"))
        .horizontal_scroll_bar_visibility(ScrollBarVisibility::Hidden)
        .vertical_scroll_bar_visibility(ScrollBarVisibility::Auto)
        .into();
    let r = mount(&el);
    let (kind, _) = first_create(&r);
    assert_eq!(kind, ControlKind::ScrollViewer);

    let mut saw_horizontal = false;
    let mut saw_vertical = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::HorizontalScrollBarVisibility, PropValue::I32(v))
                    if *v == ScrollBarVisibility::Hidden.0 =>
                {
                    saw_horizontal = true;
                }
                (Prop::VerticalScrollBarVisibility, PropValue::I32(v))
                    if *v == ScrollBarVisibility::Auto.0 =>
                {
                    saw_vertical = true;
                }
                _ => {}
            }
        }
    }
    assert!(saw_horizontal && saw_vertical);
}
