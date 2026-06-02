use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::backend::{ControlKind, Event, Op, Prop, PropValue, RecordingBackend};
use windows_reactor::core::element::Element;
use windows_reactor::core::element::{
    BreadcrumbBar, InfoBadge, InfoBar, InfoBarSeverity, NavViewItem, NavViewPaneDisplayMode,
    NavigationView, Pivot, PivotItem, TabItem, TabView, TitleBar,
};
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::dsl::text_block;

fn mount(el: &Element) -> Reconciler<RecordingBackend> {
    let mut r = Reconciler::new(RecordingBackend::new());
    r.reconcile(None, el, None, Rc::new(|| {}));
    r
}

fn count_create_ops(ops: &[Op], k: ControlKind) -> usize {
    ops.iter()
        .filter(|op| matches!(op, Op::Create { kind, .. } if *kind == k))
        .count()
}

#[test]
fn tab_view_mounts_with_headers_and_content() {
    let tabs = TabView::new([
        TabItem::new("Home", text_block("home-content")),
        TabItem::new("Settings", text_block("settings-content")),
    ])
    .selected_index(0);
    let el: Element = tabs.into();
    let r = mount(&el);

    let count_of = |k: ControlKind| {
        r.backend
            .ops
            .iter()
            .filter(|op| matches!(op, Op::Create { kind, .. } if *kind == k))
            .count()
    };
    assert_eq!(count_of(ControlKind::TabView), 1);
    assert_eq!(count_of(ControlKind::TabViewItem), 2);
    assert_eq!(count_of(ControlKind::TextBlock), 2);

    let headers: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter_map(|op| match op {
            Op::SetProp {
                prop: Prop::TabHeader,
                value: PropValue::Str(s),
                ..
            } => Some(s.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(headers, vec!["Home".to_string(), "Settings".to_string()]);
}

#[test]
fn tab_view_selected_index_update_emits_single_set() {
    let tabs_a: Element = TabView::new([TabItem::new("A", text_block("a"))])
        .selected_index(0)
        .into();
    let tabs_b: Element = TabView::new([TabItem::new("A", text_block("a"))])
        .selected_index(1)
        .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &tabs_a, None, Rc::new(|| {})).unwrap();
    r.backend.clear_ops();
    r.reconcile(Some(&tabs_a), &tabs_b, Some(id), Rc::new(|| {}));

    let selected_sets: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::SelectedIndex,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(selected_sets.len(), 1);
}

#[test]
fn tab_view_add_tab_button_visible_prop_emits_set() {
    let el: Element = TabView::new([TabItem::new("A", text_block("a"))])
        .is_add_tab_button_visible(true)
        .into();
    let r = mount(&el);

    let saw_prop = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::IsAddTabButtonVisible,
                value: PropValue::Bool(true),
                ..
            }
        )
    });
    assert!(saw_prop, "IsAddTabButtonVisible prop should be set");
}

#[test]
fn tab_view_add_tab_button_click_attaches_event() {
    let el: Element = TabView::new([TabItem::new("A", text_block("a"))])
        .on_add_tab_button_click(|()| {})
        .into();
    let r = mount(&el);

    let attached = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::AttachEvent {
                event: Event::AddTabButtonClick,
                ..
            }
        )
    });
    assert!(
        attached,
        "AttachEvent for AddTabButtonClick should be emitted"
    );
}

#[test]
fn tab_view_add_tab_button_click_callback_fires() {
    let count = Rc::new(Cell::new(0));
    let count_c = Rc::clone(&count);
    let el: Element = TabView::new([TabItem::new("A", text_block("a"))])
        .on_add_tab_button_click(move |()| count_c.set(count_c.get() + 1))
        .into();
    let r = mount(&el);

    let tv_id = match r.backend.ops.iter().find(|op| {
        matches!(
            op,
            Op::Create {
                kind: ControlKind::TabView,
                ..
            }
        )
    }) {
        Some(Op::Create { id, .. }) => *id,
        _ => panic!("no TabView Create op"),
    };

    r.backend.fire(tv_id, Event::AddTabButtonClick);
    assert_eq!(count.get(), 1);
}

#[test]
fn info_bar_mounts_with_title_message_severity() {
    let el: Element = InfoBar::new("Heads up")
        .message("Something happened")
        .severity(InfoBarSeverity::Warning)
        .into();
    let r = mount(&el);

    let count_of = |k: ControlKind| {
        r.backend
            .ops
            .iter()
            .filter(|op| matches!(op, Op::Create { kind, .. } if *kind == k))
            .count()
    };
    assert_eq!(count_of(ControlKind::InfoBar), 1);

    let mut saw_title = false;
    let mut saw_msg = false;
    let mut saw_sev = false;
    let mut saw_open = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::InfoBarTitle, PropValue::Str(s)) if s == "Heads up" => saw_title = true,
                (Prop::InfoBarMessage, PropValue::Str(s)) if s == "Something happened" => {
                    saw_msg = true;
                }
                (Prop::InfoBarSeverity, PropValue::InfoBarSev(InfoBarSeverity::Warning)) => {
                    saw_sev = true;
                }
                (Prop::InfoBarIsOpen, PropValue::Bool(true)) => saw_open = true,
                _ => {}
            }
        }
    }
    assert!(saw_title);
    assert!(saw_msg);
    assert!(saw_sev);
    assert!(saw_open);
}

#[test]
fn info_badge_numeric_sets_value() {
    let el: Element = InfoBadge::numeric(5).into();
    let r = mount(&el);
    let got_value = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::InfoBadgeValue,
                value: PropValue::I32(5),
                ..
            }
        )
    });
    assert!(got_value);
}

#[test]
fn info_badge_dot_uses_negative_sentinel() {
    let el: Element = InfoBadge::dot().into();
    let r = mount(&el);
    let got_value = r.backend.ops.iter().any(|op| {
        matches!(
            op,
            Op::SetProp {
                prop: Prop::InfoBadgeValue,
                value: PropValue::I32(-1),
                ..
            }
        )
    });
    assert!(got_value);
}
#[test]
fn navigation_view_mounts_with_menu_items_and_content() {
    let nv: Element = NavigationView::new(
        [
            NavViewItem::header("MAIN"),
            NavViewItem::new("Home").tag("home"),
            NavViewItem::new("Settings").tag("settings"),
        ],
        text_block("home-page"),
    )
    .selected_tag("home")
    .pane_open(true)
    .pane_display_mode(NavViewPaneDisplayMode::Left)
    .into();
    let r = mount(&nv);

    assert_eq!(
        count_create_ops(&r.backend.ops, ControlKind::NavigationView),
        1
    );

    assert_eq!(count_create_ops(&r.backend.ops, ControlKind::TextBlock), 1);

    let mut saw_menu = false;
    let mut saw_tag = false;
    let mut saw_pane_mode = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::NavMenuItems, PropValue::NavMenuItems(items)) => {
                    saw_menu = items.len() == 3 && items[0].is_header && items[1].content == "Home";
                }
                (Prop::NavSelectedTag, PropValue::Str(s)) if s == "home" => saw_tag = true,
                (Prop::PaneDisplayMode, PropValue::NavPaneDisplayMode(m))
                    if *m == NavViewPaneDisplayMode::Left =>
                {
                    saw_pane_mode = true;
                }
                _ => {}
            }
        }
    }
    assert!(saw_menu, "menu items prop not set");
    assert!(saw_tag, "selected tag prop not set");
    assert!(saw_pane_mode, "pane display mode prop not set");
}

#[test]
fn navigation_view_selection_changed_callback_fires() {
    let observed: Rc<std::cell::RefCell<Option<String>>> = Rc::new(std::cell::RefCell::new(None));
    let observed_c = Rc::clone(&observed);

    let nv: Element =
        NavigationView::new([NavViewItem::new("Home").tag("home")], text_block("page"))
            .on_selection_changed(move |tag| *observed_c.borrow_mut() = Some(tag))
            .into();
    let r = mount(&nv);

    let nv_id = match r.backend.ops.iter().find(|op| {
        matches!(
            op,
            Op::Create {
                kind: ControlKind::NavigationView,
                ..
            }
        )
    }) {
        Some(Op::Create { id, .. }) => *id,
        _ => panic!("no NavigationView Create op"),
    };

    r.backend
        .fire_string(nv_id, Event::NavSelectionChanged, "settings".into());
    assert_eq!(*observed.borrow(), Some("settings".to_string()));
}

#[test]
fn navigation_view_back_requested_fires_zero_arg() {
    let count = Rc::new(Cell::new(0));
    let count_c = Rc::clone(&count);
    let nv: Element = NavigationView::new([NavViewItem::new("Home")], text_block("page"))
        .back_enabled(true)
        .on_back_requested(move || count_c.set(count_c.get() + 1))
        .into();
    let r = mount(&nv);

    let nv_id = match r.backend.ops.iter().find(|op| {
        matches!(
            op,
            Op::Create {
                kind: ControlKind::NavigationView,
                ..
            }
        )
    }) {
        Some(Op::Create { id, .. }) => *id,
        _ => panic!("no NavigationView Create op"),
    };

    r.backend.fire(nv_id, Event::NavBackRequested);
    assert_eq!(count.get(), 1);
}

#[test]
fn navigation_view_update_emits_only_changed_props() {
    let a: Element =
        NavigationView::new([NavViewItem::new("Home").tag("home")], text_block("page"))
            .selected_tag("home")
            .into();
    let b: Element =
        NavigationView::new([NavViewItem::new("Home").tag("home")], text_block("page"))
            .selected_tag("settings")
            .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &a, None, Rc::new(|| {})).unwrap();
    r.backend.clear_ops();
    r.reconcile(Some(&a), &b, Some(id), Rc::new(|| {}));

    let tag_sets: Vec<_> = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::NavSelectedTag,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(tag_sets.len(), 1);

    let menu_sets = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::NavMenuItems,
                    ..
                }
            )
        })
        .count();
    assert_eq!(menu_sets, 0);
}

#[test]
fn title_bar_mounts_with_title_subtitle_and_button_visibility() {
    let tb: Element = TitleBar::new("MyApp")
        .subtitle("Preview")
        .back_button_visible(true)
        .back_button_enabled(true)
        .pane_toggle_button_visible(true)
        .into();
    let r = mount(&tb);
    assert_eq!(count_create_ops(&r.backend.ops, ControlKind::TitleBar), 1);

    let mut saw_title = false;
    let mut saw_sub = false;
    let mut saw_back_visible = false;
    let mut saw_back_enabled = false;
    let mut saw_pane = false;
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::TitleBarTitle, PropValue::Str(s)) if s == "MyApp" => saw_title = true,
                (Prop::TitleBarSubtitle, PropValue::Str(s)) if s == "Preview" => saw_sub = true,
                (Prop::IsBackButtonVisible, PropValue::Bool(true)) => saw_back_visible = true,
                (Prop::IsBackEnabled, PropValue::Bool(true)) => saw_back_enabled = true,
                (Prop::IsPaneToggleButtonVisible, PropValue::Bool(true)) => saw_pane = true,
                _ => {}
            }
        }
    }
    assert!(saw_title);
    assert!(saw_sub);
    assert!(saw_back_visible);
    assert!(saw_back_enabled);
    assert!(saw_pane);
}

#[test]
fn title_bar_back_and_pane_callbacks_fire_via_recording_backend() {
    let back = Rc::new(Cell::new(0));
    let pane = Rc::new(Cell::new(0));
    let back_c = Rc::clone(&back);
    let pane_c = Rc::clone(&pane);
    let tb: Element = TitleBar::new("MyApp")
        .on_back_requested(move || back_c.set(back_c.get() + 1))
        .on_pane_toggle_requested(move || pane_c.set(pane_c.get() + 1))
        .into();
    let r = mount(&tb);

    let id = match r.backend.ops.iter().find(|op| {
        matches!(
            op,
            Op::Create {
                kind: ControlKind::TitleBar,
                ..
            }
        )
    }) {
        Some(Op::Create { id, .. }) => *id,
        _ => panic!("no TitleBar Create op"),
    };
    r.backend.fire(id, Event::TitleBarBackRequested);
    r.backend.fire(id, Event::TitleBarPaneToggle);
    assert_eq!(back.get(), 1);
    assert_eq!(pane.get(), 1);
}

#[test]
fn title_bar_update_diffs_only_changed_string_fields() {
    let a: Element = TitleBar::new("App").subtitle("v1").into();
    let b: Element = TitleBar::new("App").subtitle("v2").into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &a, None, Rc::new(|| {})).unwrap();
    r.backend.clear_ops();
    r.reconcile(Some(&a), &b, Some(id), Rc::new(|| {}));

    let title_sets = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::TitleBarTitle,
                    ..
                }
            )
        })
        .count();
    let subtitle_sets = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::TitleBarSubtitle,
                    ..
                }
            )
        })
        .count();
    assert_eq!(title_sets, 0, "title unchanged → no set");
    assert_eq!(subtitle_sets, 1, "subtitle changed → exactly one set");
}

#[test]
fn pivot_mounts_with_items_and_content() {
    let p: Element = Pivot::new([
        PivotItem::new("Tab A", text_block("a-page")),
        PivotItem::new("Tab B", text_block("b-page")),
    ])
    .selected_index(1)
    .title("My Pivot")
    .into();
    let r = mount(&p);

    assert_eq!(count_create_ops(&r.backend.ops, ControlKind::Pivot), 1);
    assert_eq!(count_create_ops(&r.backend.ops, ControlKind::PivotItem), 2);
    assert_eq!(count_create_ops(&r.backend.ops, ControlKind::TextBlock), 2);

    let mut saw_title = false;
    let mut saw_idx = false;
    let mut headers = vec![];
    for op in &r.backend.ops {
        if let Op::SetProp { prop, value, .. } = op {
            match (prop, value) {
                (Prop::PivotTitle, PropValue::Str(s)) if s == "My Pivot" => saw_title = true,
                (Prop::SelectedIndex, PropValue::I32(1)) => saw_idx = true,
                (Prop::PivotItemHeader, PropValue::Str(s)) => headers.push(s.clone()),
                _ => {}
            }
        }
    }
    assert!(saw_title);
    assert!(saw_idx);
    assert_eq!(headers, vec!["Tab A".to_string(), "Tab B".to_string()]);
}

#[test]
fn pivot_selection_changed_dispatches_index() {
    let observed = Rc::new(Cell::new(-1_i32));
    let observed_c = Rc::clone(&observed);
    let p: Element = Pivot::new([PivotItem::new("A", text_block("a"))])
        .on_selection_changed(move |i| observed_c.set(i))
        .into();
    let r = mount(&p);
    let id = match r.backend.ops.iter().find(|op| {
        matches!(
            op,
            Op::Create {
                kind: ControlKind::Pivot,
                ..
            }
        )
    }) {
        Some(Op::Create { id, .. }) => *id,
        _ => panic!("no Pivot Create op"),
    };
    r.backend.fire_i32(id, Event::PivotSelectionChanged, 2);
    assert_eq!(observed.get(), 2);
}

#[test]
fn pivot_update_grows_and_shrinks_items() {
    let a: Element = Pivot::new([PivotItem::new("A", text_block("a"))]).into();
    let b: Element = Pivot::new([
        PivotItem::new("A", text_block("a")),
        PivotItem::new("B", text_block("b")),
    ])
    .into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &a, None, Rc::new(|| {})).unwrap();
    r.backend.clear_ops();
    r.reconcile(Some(&a), &b, Some(id), Rc::new(|| {}));

    assert_eq!(count_create_ops(&r.backend.ops, ControlKind::PivotItem), 1);
    assert_eq!(count_create_ops(&r.backend.ops, ControlKind::TextBlock), 1);

    r.backend.clear_ops();
    r.reconcile(Some(&b), &a, Some(id), Rc::new(|| {}));
    let removes = r
        .backend
        .ops
        .iter()
        .filter(|op| matches!(op, Op::RemoveChild { .. }))
        .count();
    assert_eq!(removes, 1, "exactly one tab removed");
}

#[test]
fn breadcrumb_bar_mounts_items_as_string_list() {
    let bc: Element = BreadcrumbBar::new(["Root", "Folder", "File.txt"]).into();
    let r = mount(&bc);
    assert_eq!(
        count_create_ops(&r.backend.ops, ControlKind::BreadcrumbBar),
        1
    );

    let saw_items = r.backend.ops.iter().any(|op| {
        matches!(op, Op::SetProp {
            prop: Prop::BreadcrumbItems,
            value: PropValue::StrList(v),
            ..
        } if v == &vec!["Root".to_string(), "Folder".to_string(), "File.txt".to_string()])
    });
    assert!(saw_items, "items prop not set with the right strings");
}

#[test]
fn breadcrumb_bar_item_clicked_dispatches_index() {
    let observed = Rc::new(Cell::new(-1_i32));
    let observed_c = Rc::clone(&observed);
    let bc: Element = BreadcrumbBar::new(["Root", "Inner"])
        .on_item_clicked(move |i| observed_c.set(i))
        .into();
    let r = mount(&bc);
    let id = match r.backend.ops.iter().find(|op| {
        matches!(
            op,
            Op::Create {
                kind: ControlKind::BreadcrumbBar,
                ..
            }
        )
    }) {
        Some(Op::Create { id, .. }) => *id,
        _ => panic!("no BreadcrumbBar Create op"),
    };
    r.backend.fire_i32(id, Event::BreadcrumbItemClicked, 0);
    assert_eq!(observed.get(), 0);
}

#[test]
fn breadcrumb_bar_update_emits_set_only_when_items_change() {
    let a: Element = BreadcrumbBar::new(["A", "B"]).into();
    let b: Element = BreadcrumbBar::new(["A", "B"]).into();
    let c: Element = BreadcrumbBar::new(["A", "B", "C"]).into();

    let mut r = Reconciler::new(RecordingBackend::new());
    let id = r.reconcile(None, &a, None, Rc::new(|| {})).unwrap();

    r.backend.clear_ops();
    r.reconcile(Some(&a), &b, Some(id), Rc::new(|| {}));
    let no_change_sets = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::BreadcrumbItems,
                    ..
                }
            )
        })
        .count();
    assert_eq!(no_change_sets, 0);

    r.backend.clear_ops();
    r.reconcile(Some(&b), &c, Some(id), Rc::new(|| {}));
    let grew_sets = r
        .backend
        .ops
        .iter()
        .filter(|op| {
            matches!(
                op,
                Op::SetProp {
                    prop: Prop::BreadcrumbItems,
                    ..
                }
            )
        })
        .count();
    assert_eq!(grew_sets, 1);
}
