//! Reconciler stress fixtures: edge cases for the keyed/positional child
//! diffing algorithm. These test scenarios that are more complex than the
//! basic "reverse a list" covered in reconciler::keyed_list.

use windows_core::Interface;

use windows_reactor::Element;
use windows_reactor::{ElementExt, button, text_block};

use crate::fixtures::reconciler::{FixtureFuture, cc};
use crate::harness::Harness;

use windows_reactor::vstack;

/// Remove an item from the middle of a keyed list and verify surrounding
/// items remain in correct order and identity-preserved.
pub fn keyed_middle_removal(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (removed, set) = cx.use_state(false);
            let items: Vec<&str> = if removed {
                vec!["A", "C", "D", "E"]
            } else {
                vec!["A", "B", "C", "D", "E"]
            };
            let kids: Vec<Element> = items
                .into_iter()
                .map(|s| text_block(s).with_key(s).into())
                .collect();
            vstack((
                button("Remove_B").on_click(move || set.call(true)),
                vstack(kids).spacing(4.0),
            ))
            .into()
        }));
        h.render().await;

        // Capture identity of "C" before removal
        let c_before = h.find_text("C");
        h.check(
            "KeyedStress_MiddleRemoval_AllPresent",
            h.find_text("A").is_some()
                && h.find_text("B").is_some()
                && h.find_text("C").is_some()
                && h.find_text("D").is_some()
                && h.find_text("E").is_some(),
        );

        let _ = h.click_button("Remove_B");
        h.render().await;

        h.check(
            "KeyedStress_MiddleRemoval_BGone",
            h.find_text("B").is_none(),
        );
        h.check(
            "KeyedStress_MiddleRemoval_OthersPresent",
            h.find_text("A").is_some()
                && h.find_text("C").is_some()
                && h.find_text("D").is_some()
                && h.find_text("E").is_some(),
        );

        // "C" should be identity-preserved (not recreated)
        let c_after = h.find_text("C");
        let same = match (c_before, c_after) {
            (Some(a), Some(b)) => identity_eq(&a, &b),
            _ => false,
        };
        h.check("KeyedStress_MiddleRemoval_IdentityPreserved", same);
    })
}

/// Insert a new item into the middle of a keyed list.
pub fn keyed_middle_insert(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (inserted, set) = cx.use_state(false);
            let items: Vec<&str> = if inserted {
                vec!["A", "B", "X", "C", "D"]
            } else {
                vec!["A", "B", "C", "D"]
            };
            let kids: Vec<Element> = items
                .into_iter()
                .map(|s| text_block(s).with_key(s).into())
                .collect();
            vstack((
                button("Insert_X").on_click(move || set.call(true)),
                vstack(kids).spacing(4.0),
            ))
            .into()
        }));
        h.render().await;

        let c_before = h.find_text("C");
        h.check(
            "KeyedStress_MiddleInsert_Initial",
            h.find_text("A").is_some()
                && h.find_text("B").is_some()
                && h.find_text("C").is_some()
                && h.find_text("D").is_some()
                && h.find_text("X").is_none(),
        );

        let _ = h.click_button("Insert_X");
        h.render().await;

        h.check(
            "KeyedStress_MiddleInsert_XAppeared",
            h.find_text("X").is_some(),
        );
        h.check(
            "KeyedStress_MiddleInsert_AllPresent",
            h.find_text("A").is_some()
                && h.find_text("B").is_some()
                && h.find_text("C").is_some()
                && h.find_text("D").is_some(),
        );

        // "C" should be identity-preserved
        let c_after = h.find_text("C");
        let same = match (c_before, c_after) {
            (Some(a), Some(b)) => identity_eq(&a, &b),
            _ => false,
        };
        h.check("KeyedStress_MiddleInsert_IdentityPreserved", same);
    })
}

/// Complex reorder: move first to last, and last to first simultaneously.
pub fn keyed_complex_reorder(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (swapped, set) = cx.use_state(false);
            let items: Vec<&str> = if swapped {
                vec!["E", "B", "C", "D", "A"]
            } else {
                vec!["A", "B", "C", "D", "E"]
            };
            let kids: Vec<Element> = items
                .into_iter()
                .map(|s| text_block(s).with_key(s).into())
                .collect();
            vstack((
                button("Swap").on_click(move || set.call(true)),
                vstack(kids).spacing(4.0),
            ))
            .into()
        }));
        h.render().await;

        let a_before = h.find_text("A");
        let e_before = h.find_text("E");
        h.check(
            "KeyedStress_ComplexReorder_Initial",
            a_before.is_some() && e_before.is_some(),
        );

        let _ = h.click_button("Swap");
        h.render().await;

        h.check(
            "KeyedStress_ComplexReorder_AllPresent",
            h.find_text("A").is_some()
                && h.find_text("B").is_some()
                && h.find_text("C").is_some()
                && h.find_text("D").is_some()
                && h.find_text("E").is_some(),
        );

        // Both A and E should be identity-preserved (reordered, not recreated)
        let a_after = h.find_text("A");
        let e_after = h.find_text("E");
        let a_same = match (a_before, a_after) {
            (Some(a), Some(b)) => identity_eq(&a, &b),
            _ => false,
        };
        let e_same = match (e_before, e_after) {
            (Some(a), Some(b)) => identity_eq(&a, &b),
            _ => false,
        };
        h.check("KeyedStress_ComplexReorder_APreserved", a_same);
        h.check("KeyedStress_ComplexReorder_EPreserved", e_same);
    })
}

/// Insert + remove + reorder in a single update.
pub fn keyed_insert_remove_reorder(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (phase, set) = cx.use_state(0u32);
            // Phase 0: [A, B, C, D]
            // Phase 1: [D, X, A, C]  (remove B, insert X, reorder)
            let items: Vec<&str> = if phase == 0 {
                vec!["A", "B", "C", "D"]
            } else {
                vec!["D", "X", "A", "C"]
            };
            let kids: Vec<Element> = items
                .into_iter()
                .map(|s| text_block(s).with_key(s).into())
                .collect();
            vstack((
                button("Mutate").on_click(move || set.call(1)),
                vstack(kids).spacing(4.0),
            ))
            .into()
        }));
        h.render().await;

        let a_before = h.find_text("A");
        let d_before = h.find_text("D");
        h.check(
            "KeyedStress_InsertRemoveReorder_Initial",
            h.find_text("A").is_some()
                && h.find_text("B").is_some()
                && h.find_text("C").is_some()
                && h.find_text("D").is_some(),
        );

        let _ = h.click_button("Mutate");
        h.render().await;

        h.check(
            "KeyedStress_InsertRemoveReorder_BGone",
            h.find_text("B").is_none(),
        );
        h.check(
            "KeyedStress_InsertRemoveReorder_XAppeared",
            h.find_text("X").is_some(),
        );
        h.check(
            "KeyedStress_InsertRemoveReorder_OthersPresent",
            h.find_text("A").is_some() && h.find_text("C").is_some() && h.find_text("D").is_some(),
        );

        // Identity preservation for surviving nodes
        let a_after = h.find_text("A");
        let d_after = h.find_text("D");
        let a_same = match (a_before, a_after) {
            (Some(a), Some(b)) => identity_eq(&a, &b),
            _ => false,
        };
        let d_same = match (d_before, d_after) {
            (Some(a), Some(b)) => identity_eq(&a, &b),
            _ => false,
        };
        h.check("KeyedStress_InsertRemoveReorder_APreserved", a_same);
        h.check("KeyedStress_InsertRemoveReorder_DPreserved", d_same);
    })
}

/// Large list stress: mount 100 keyed items, then remove every other one.
pub fn keyed_large_list(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (filtered, set) = cx.use_state(false);
            let items: Vec<usize> = if filtered {
                (0..100).filter(|i| i % 2 == 0).collect()
            } else {
                (0..100).collect()
            };
            let kids: Vec<Element> = items
                .into_iter()
                .map(|i| text_block(format!("N{i}")).with_key(format!("k{i}")).into())
                .collect();
            vstack((
                button("Filter").on_click(move || set.call(true)),
                vstack(kids),
            ))
            .into()
        }));
        h.render().await;

        h.check(
            "KeyedStress_LargeList_Initial",
            h.find_text("N0").is_some() && h.find_text("N99").is_some(),
        );

        let n0_before = h.find_text("N0");

        let _ = h.click_button("Filter");
        h.render().await;

        h.check(
            "KeyedStress_LargeList_OddsRemoved",
            h.find_text("N1").is_none() && h.find_text("N99").is_none(),
        );
        h.check(
            "KeyedStress_LargeList_EvensPresent",
            h.find_text("N0").is_some()
                && h.find_text("N50").is_some()
                && h.find_text("N98").is_some(),
        );

        // Identity check on a surviving node
        let n0_after = h.find_text("N0");
        let same = match (n0_before, n0_after) {
            (Some(a), Some(b)) => identity_eq(&a, &b),
            _ => false,
        };
        h.check("KeyedStress_LargeList_IdentityPreserved", same);
    })
}

/// Positional type mismatch: swap a TextBlock for a Button in the same slot
/// (same position, different widget kind). The reconciler must detach the
/// old control and create a new one.
pub fn positional_type_mismatch(h: Harness) -> FixtureFuture {
    Box::pin(async move {
        h.mount(cc(|cx| {
            let (use_button, set) = cx.use_state(false);
            let child: Element = if use_button {
                button("Now a button").into()
            } else {
                text_block("Initially text").into()
            };
            vstack((
                child,
                button("Switch").on_click(move || set.call(!use_button)),
            ))
            .into()
        }));
        h.render().await;

        let tb = h.find_text("Initially text");
        h.check("KeyedStress_TypeMismatch_InitialText", tb.is_some());
        h.check(
            "KeyedStress_TypeMismatch_NoButton",
            h.find_button("Now a button").is_none(),
        );

        let _ = h.click_button("Switch");
        h.render().await;

        h.check(
            "KeyedStress_TypeMismatch_TextGone",
            h.find_text("Initially text").is_none(),
        );
        h.check(
            "KeyedStress_TypeMismatch_ButtonAppeared",
            h.find_button("Now a button").is_some(),
        );
    })
}

fn identity_eq<T: Interface, U: Interface>(a: &T, b: &U) -> bool {
    use windows_core::IUnknown;
    let Ok(ua) = a.cast::<IUnknown>() else {
        return false;
    };
    let Ok(ub) = b.cast::<IUnknown>() else {
        return false;
    };
    ua.as_raw() == ub.as_raw()
}
