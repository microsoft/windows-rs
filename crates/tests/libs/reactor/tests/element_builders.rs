use windows_reactor::core::element::{Element, GridLength, ScrollBarVisibility, Thickness};
use windows_reactor::dsl::{ElementExt, check_box, scroll_viewer, text_block, text_box};
use windows_reactor::grid;

#[test]
fn check_box_default_state() {
    let c = check_box(true);
    assert!(c.is_checked);
    assert!(c.is_enabled);
    assert!(c.content.is_none());
    assert!(c.on_changed.is_none());
}

#[test]
fn check_box_content_installs_content() {
    let c = check_box(false).content("Accept terms");
    assert_eq!(c.content.as_deref(), Some("Accept terms"));
}

#[test]
fn check_box_on_changed_stores_callback() {
    let c = check_box(false).on_changed(|_v| {});
    assert!(c.on_changed.is_some());
}

#[test]
fn check_box_disabled_clears_flag() {
    let c = check_box(true).enabled(false);
    assert!(!c.is_enabled);
}

#[test]
fn check_box_modifiers_chain_via_element_ext() {
    let c = check_box(false).margin(4.0).with_key("agree");
    assert_eq!(c.modifiers.margin, Some(Thickness::uniform(4.0)));
    assert_eq!(c.key.as_deref(), Some("agree"));
}

#[test]
fn text_field_default_state() {
    let t = text_box("hello");
    assert_eq!(t.value, "hello");
    assert!(t.is_enabled);
    assert!(t.placeholder.is_none());
    assert!(t.header.is_none());
}

#[test]
fn text_field_placeholder_and_header() {
    let t = text_box("").placeholder("Start typing...").header("Notes");
    assert_eq!(t.placeholder.as_deref(), Some("Start typing..."));
    assert_eq!(t.header.as_deref(), Some("Notes"));
}

#[test]
fn text_field_on_changed_installs_handler() {
    let t = text_box("").on_changed(|_v| {});
    assert!(t.on_changed.is_some());
}

#[test]
fn grid_builder_sets_row_and_column_definitions() {
    let g = grid((text_block("a"),))
        .rows([GridLength::Auto, GridLength::STAR])
        .columns([GridLength::Pixel(40.0), GridLength::STAR])
        .row_spacing(4.0)
        .column_spacing(6.0);
    assert_eq!(g.rows.len(), 2);
    assert_eq!(g.columns.len(), 2);
    assert_eq!(g.row_spacing, Some(4.0));
    assert_eq!(g.column_spacing, Some(6.0));
    assert_eq!(g.children.len(), 1);
}

#[test]
fn grid_placement_modifiers() {
    let e: Element = text_block("cell").into();
    let e = e
        .grid_row(1)
        .grid_column(2)
        .grid_row_span(2)
        .grid_column_span(3);

    let p = e.modifiers().unwrap().grid.unwrap();
    assert_eq!(p.row, 1);
    assert_eq!(p.column, 2);
    assert_eq!(p.row_span, 2);
    assert_eq!(p.column_span, 3);

    let c: Element = check_box(true).into();
    let c = c
        .grid_row(3)
        .grid_column(5)
        .grid_row_span(2)
        .grid_column_span(4);
    let p = c.modifiers().unwrap().grid.unwrap();
    assert_eq!(p.row, 3);
    assert_eq!(p.column, 5);
    assert_eq!(p.row_span, 2);
    assert_eq!(p.column_span, 4);
}

#[test]
fn scroll_view_default_visibilities() {
    let s = scroll_viewer(text_block("a"));
    assert_eq!(
        s.horizontal_scroll_bar_visibility,
        ScrollBarVisibility::Disabled
    );
    assert_eq!(s.vertical_scroll_bar_visibility, ScrollBarVisibility::Auto);
}

#[test]
fn scroll_view_builder_flips_visibilities() {
    let s = scroll_viewer(text_block("a"))
        .horizontal_scroll_bar_visibility(ScrollBarVisibility::Visible)
        .vertical_scroll_bar_visibility(ScrollBarVisibility::Hidden);
    assert_eq!(
        s.horizontal_scroll_bar_visibility,
        ScrollBarVisibility::Visible
    );
    assert_eq!(
        s.vertical_scroll_bar_visibility,
        ScrollBarVisibility::Hidden
    );
}

#[test]
fn min_max_width_and_height_chain_through_element_ext() {
    let c = check_box(false)
        .min_width(10.0)
        .max_width(200.0)
        .min_height(8.0)
        .max_height(150.0);
    assert_eq!(c.modifiers.min_width, Some(10.0));
    assert_eq!(c.modifiers.max_width, Some(200.0));
    assert_eq!(c.modifiers.min_height, Some(8.0));
    assert_eq!(c.modifiers.max_height, Some(150.0));
}

mod mount {
    use super::*;
    use std::rc::Rc;
    use windows_reactor::core::backend::{Op, Prop, PropValue, RecordingBackend};
    use windows_reactor::core::reconciler::Reconciler;

    fn mount_one(el: Element) -> Reconciler<RecordingBackend> {
        let mut r = Reconciler::new(RecordingBackend::new());
        r.reconcile(None, &el, None, Rc::new(|| {}));
        r
    }

    #[test]
    fn mount_check_box_records_is_checked_prop() {
        let r = mount_one(check_box(true).into());
        let sets: Vec<_> = r
            .backend
            .ops
            .iter()
            .filter_map(|o| match o {
                Op::SetProp { prop, value, .. } => Some((*prop, value.clone())),
                _ => None,
            })
            .collect();
        assert!(
            sets.iter()
                .any(|(p, v)| *p == Prop::IsChecked && matches!(v, PropValue::Bool(true)))
        );
    }

    #[test]
    fn mount_text_field_records_value_and_placeholder() {
        let t = text_box("draft").placeholder("…");
        let r = mount_one(t.into());
        let sets: Vec<_> = r
            .backend
            .ops
            .iter()
            .filter_map(|o| match o {
                Op::SetProp { prop, value, .. } => Some((*prop, value.clone())),
                _ => None,
            })
            .collect();
        assert!(sets.iter().any(|(p, v)| matches!(
            (p, v),
            (Prop::Value, PropValue::Str(s)) if s == "draft"
        )));
        assert!(sets.iter().any(|(p, v)| matches!(
            (p, v),
            (Prop::PlaceholderText, PropValue::Str(s)) if s == "…"
        )));
    }

    #[test]
    fn mount_grid_appends_children_and_applies_placement() {
        let g = grid((check_box(true).grid_row(1).grid_column(2),))
            .rows([GridLength::STAR, GridLength::STAR])
            .columns([GridLength::STAR, GridLength::STAR, GridLength::STAR]);
        let r = mount_one(g.into());
        let creates = r
            .backend
            .ops
            .iter()
            .filter(|o| matches!(o, Op::Create { .. }))
            .count();
        assert_eq!(creates, 2, "Grid + one CheckBox child");

        let row_set = r.backend.ops.iter().any(|o| {
            matches!(
                o,
                Op::SetProp {
                    prop: Prop::AttachedGridRow,
                    value: PropValue::I32(1),
                    ..
                }
            )
        });
        let col_set = r.backend.ops.iter().any(|o| {
            matches!(
                o,
                Op::SetProp {
                    prop: Prop::AttachedGridColumn,
                    value: PropValue::I32(2),
                    ..
                }
            )
        });
        assert!(row_set, "Grid.Row attached prop on child");
        assert!(col_set, "Grid.Column attached prop on child");
    }

    #[test]
    fn mount_scroll_view_attaches_child() {
        let s = scroll_viewer(text_block("content"));
        let r = mount_one(s.into());
        let appends = r
            .backend
            .ops
            .iter()
            .filter(|o| matches!(o, Op::AppendChild { .. }))
            .count();
        assert_eq!(appends, 1);
    }
}

mod update {
    use super::*;
    use std::rc::Rc;
    use windows_reactor::core::backend::{Op, Prop, PropValue, RecordingBackend};
    use windows_reactor::core::reconciler::Reconciler;

    fn reconcile(
        r: &mut Reconciler<RecordingBackend>,
        old: &Element,
        new: &Element,
        existing: Option<windows_reactor::core::backend::ControlId>,
    ) {
        r.reconcile(Some(old), new, existing, Rc::new(|| {}));
    }

    #[test]
    fn checkbox_toggle_emits_is_checked_set() {
        let mut r = Reconciler::new(RecordingBackend::new());
        let a: Element = check_box(false).into();
        let id = r.reconcile(None, &a, None, Rc::new(|| {})).unwrap();
        r.backend.clear_ops();
        let b: Element = check_box(true).into();
        reconcile(&mut r, &a, &b, Some(id));
        let sets = r
            .backend
            .ops
            .iter()
            .filter(|o| {
                matches!(
                    o,
                    Op::SetProp {
                        prop: Prop::IsChecked,
                        ..
                    }
                )
            })
            .count();
        assert_eq!(sets, 1);
    }

    #[test]
    fn text_field_value_change_emits_single_set() {
        let mut r = Reconciler::new(RecordingBackend::new());
        let a: Element = text_box("hi").into();
        let id = r.reconcile(None, &a, None, Rc::new(|| {})).unwrap();
        r.backend.clear_ops();
        let b: Element = text_box("hello").into();
        reconcile(&mut r, &a, &b, Some(id));
        let sets: Vec<_> = r
            .backend
            .ops
            .iter()
            .filter(|o| {
                matches!(
                    o,
                    Op::SetProp {
                        prop: Prop::Value,
                        ..
                    }
                )
            })
            .collect();
        assert_eq!(sets.len(), 1);
        match &sets[0] {
            Op::SetProp {
                value: PropValue::Str(s),
                ..
            } => assert_eq!(s, "hello"),
            _ => unreachable!(),
        }
    }

    #[test]
    fn grid_row_change_emits_new_attached_prop() {
        let mut r = Reconciler::new(RecordingBackend::new());
        let a: Element = grid((check_box(false).grid_row(0),)).into();
        let id = r.reconcile(None, &a, None, Rc::new(|| {})).unwrap();
        r.backend.clear_ops();
        let b: Element = grid((check_box(false).grid_row(3),)).into();
        reconcile(&mut r, &a, &b, Some(id));
        let rows: Vec<_> = r
            .backend
            .ops
            .iter()
            .filter(|o| {
                matches!(
                    o,
                    Op::SetProp {
                        prop: Prop::AttachedGridRow,
                        ..
                    }
                )
            })
            .collect();
        assert_eq!(rows.len(), 1);
        match &rows[0] {
            Op::SetProp {
                value: PropValue::I32(r),
                ..
            } => assert_eq!(*r, 3),
            _ => unreachable!(),
        }
    }

    #[test]
    fn scroll_view_visibility_change_emits_set() {
        let mut r = Reconciler::new(RecordingBackend::new());
        let a: Element = scroll_viewer(text_block("x")).into();
        let id = r.reconcile(None, &a, None, Rc::new(|| {})).unwrap();
        r.backend.clear_ops();
        let b: Element = scroll_viewer(text_block("x"))
            .vertical_scroll_bar_visibility(ScrollBarVisibility::Visible)
            .into();
        reconcile(&mut r, &a, &b, Some(id));
        let sets = r
            .backend
            .ops
            .iter()
            .filter(|o| {
                matches!(
                    o,
                    Op::SetProp {
                        prop: Prop::VerticalScrollBarVisibility,
                        ..
                    }
                )
            })
            .count();
        assert_eq!(sets, 1);
    }
}
