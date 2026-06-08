use std::cell::Cell;
use std::rc::Rc;

use windows_reactor::core::element::{Element, TextBlock};
use windows_reactor::core::templated_list::{TemplatedKind, flip_view, grid_view, list_view};

#[test]
fn list_view_builder_produces_templated_list_variant() {
    let e = list_view(vec![1i32, 2, 3], |n, _| TextBlock::new(n.to_string())).build();
    match &e {
        Element::TemplatedList(tl) => {
            assert_eq!(tl.kind, TemplatedKind::ListView);
            assert_eq!(tl.item_count(), 3);
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn build_item_view_invokes_view_builder() {
    let e = list_view(vec!["a".to_string(), "b".to_string()], |s, i| {
        TextBlock::new(format!("{i}:{s}"))
    })
    .build();
    let Element::TemplatedList(tl) = &e else {
        panic!()
    };
    match tl.build_item_view(1) {
        Element::TextBlock(t) => assert_eq!(t.text, "1:b"),
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn same_items_as_is_identity_based() {
    let a = list_view(vec![1i32, 2, 3], |_, _| Element::Empty).build();
    let b = a.clone();
    let c = list_view(vec![1i32, 2, 3], |_, _| Element::Empty).build();
    if let (Element::TemplatedList(a), Element::TemplatedList(b), Element::TemplatedList(c)) =
        (&a, &b, &c)
    {
        assert!(a.same_items_as(b), "clone should share items Rc");
        assert!(!a.same_items_as(c), "freshly-built list allocates a new Rc");
    } else {
        panic!("expected templated list variants");
    }
}

#[test]
fn key_selector_provides_stable_keys() {
    let e = list_view(vec![10i32, 20, 30], |n, _| TextBlock::new(n.to_string()))
        .with_key_selector(|n| format!("k-{n}"))
        .build();
    let Element::TemplatedList(tl) = &e else {
        panic!()
    };
    assert_eq!(tl.item_key(0), Some("k-10".into()));
    assert_eq!(tl.item_key(2), Some("k-30".into()));
    assert!(tl.fully_keyed());
}

#[test]
fn unkeyed_list_collect_keys_falls_back_to_positional() {
    let e = list_view(vec![1i32, 2], |n, _| TextBlock::new(n.to_string())).build();
    let Element::TemplatedList(tl) = &e else {
        panic!()
    };
    assert_eq!(
        tl.collect_keys(),
        vec!["__pos_0".to_string(), "__pos_1".into()]
    );
    assert!(!tl.fully_keyed());
}

#[test]
fn selected_index_defaults_to_negative_one() {
    let e = list_view(vec![1i32], |_, _| Element::Empty).build();
    let Element::TemplatedList(tl) = &e else {
        panic!()
    };
    assert_eq!(tl.selected_index(), -1);
}

#[test]
fn on_selection_changed_forwards_to_callback() {
    let sink = Rc::new(Cell::new(-1_i32));
    let sink_c = Rc::clone(&sink);
    let e = list_view(vec![1i32, 2, 3], |_, _| Element::Empty)
        .on_selection_changed(move |i| sink_c.set(i))
        .build();
    let Element::TemplatedList(tl) = &e else {
        panic!()
    };
    tl.invoke_selection_changed(2);
    assert_eq!(sink.get(), 2);
}

#[test]
fn kinds_are_distinct_by_dsl() {
    let a = list_view(vec![1i32], |_, _| Element::Empty).build();
    let b = grid_view(vec![1i32], |_, _| Element::Empty).build();
    let c = flip_view(vec![1i32], |_, _| Element::Empty).build();
    match (&a, &b, &c) {
        (Element::TemplatedList(a), Element::TemplatedList(b), Element::TemplatedList(c)) => {
            assert_eq!(a.kind, TemplatedKind::ListView);
            assert_eq!(b.kind, TemplatedKind::GridView);
            assert_eq!(c.kind, TemplatedKind::FlipView);
        }
        _ => panic!(),
    }
}
