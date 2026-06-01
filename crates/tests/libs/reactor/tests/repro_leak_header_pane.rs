//! Minimal reproduction for the memory leak reported in:
//!   - https://github.com/microsoft/windows-rs/issues/4491
//!   - https://github.com/microsoft/windows-rs/issues/4500
//!
//! ROOT CAUSE
//! ==========
//! `Reconciler::unmount()` walks only `children_mirror` via `collect_subtree()`.
//! But widgets that use `header_element()` or `pane_element()` (Expander,
//! TitleBar, SplitView) store those sub-trees in separate maps
//! (`self.header_elements`, `self.pane_elements`) which are **never** walked,
//! unmounted, or destroyed. Their entire subtrees — XAML handles, component
//! instances, event closures — are leaked on every unmount.
//!
//! This affects any page-switching scenario where the unmounted subtree
//! contains an Expander with a complex header, a SplitView with a pane, or
//! a TitleBar. Each navigation cycle leaks more controls.
//!
//! HOW TO RUN
//! ==========
//! From the repo root:
//!
//!   cargo test -p test_reactor --test repro_leak_header_pane
//!
//! Both tests verify the fix works (they pass now that the reconciler
//! correctly unmounts header/pane subtrees):
//!   - `header_element_subtree_leaked_on_unmount`
//!   - `repeated_navigation_accumulates_leaked_controls`

use std::rc::Rc;

use windows_reactor::core::backend::{Op, RecordingBackend};
use windows_reactor::core::element::{Element, Expander};
use windows_reactor::core::reconciler::Reconciler;
use windows_reactor::dsl::text_block;
use windows_reactor::vstack;

fn noop_rerender() -> Rc<dyn Fn()> {
    Rc::new(|| {})
}

/// Count Create and Destroy ops in the recording backend.
fn count_creates_and_destroys(ops: &[Op]) -> (usize, usize) {
    let creates = ops
        .iter()
        .filter(|o| matches!(o, Op::Create { .. }))
        .count();
    let destroys = ops
        .iter()
        .filter(|o| matches!(o, Op::Destroy { .. }))
        .count();
    (creates, destroys)
}

/// Build an Expander with a header_element containing nested widgets.
/// The header_element is stored in `reconciler.header_elements` (NOT in
/// `children_mirror`), so `collect_subtree` never finds it on unmount.
fn make_expander_with_header() -> Element {
    Expander::new(text_block("body content"))
        .header_content(vstack((
            text_block("Header item 1"),
            text_block("Header item 2"),
            text_block("Header item 3"),
        )))
        .expanded(true)
        .into()
}

/// Simulates a page inside a NavigationView that contains an Expander
/// with a complex header — a realistic scenario matching both #4491 and #4500.
fn make_page() -> Element {
    vstack((
        text_block("Page Title"),
        Expander::new(text_block("body content"))
            .header_content(vstack((
                text_block("Header item 1"),
                text_block("Header item 2"),
                text_block("Header item 3"),
            )))
            .expanded(true),
        text_block("Footer"),
    ))
    .into()
}

#[test]
fn header_element_subtree_leaked_on_unmount() {
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let rerender = noop_rerender();

    // 1. Mount an Expander with a complex header_element
    let tree = make_expander_with_header();
    let root_id = reconciler.reconcile(None, &tree, None, Rc::clone(&rerender));
    assert!(root_id.is_some(), "mount should produce a root control");

    // 2. Replace the tree with Empty — this triggers unmount of the root
    let _new_id = reconciler.reconcile(Some(&tree), &Element::Empty, root_id, Rc::clone(&rerender));

    // 3. Check: every Create should have a matching Destroy
    let (creates, destroys) = count_creates_and_destroys(&reconciler.backend.ops);

    println!("Created {creates} controls, destroyed {destroys}");
    println!(
        "LEAKED: {} controls never destroyed (header_element subtree)",
        creates.saturating_sub(destroys)
    );

    // Verifies the fix: all header_element subtree controls are destroyed.
    // Before the fix, creates > destroys (header StackPanel + 3 TextBlocks leaked).
    assert_eq!(
        creates,
        destroys,
        "BUG: {creates} controls created but only {destroys} destroyed. \
         The missing {} are the header_element subtree that was never \
         collected by unmount's collect_subtree().",
        creates.saturating_sub(destroys)
    );
}

#[test]
fn repeated_navigation_accumulates_leaked_controls() {
    // Simulates issue #4491: navigating between pages that contain Expanders.
    // Each "page switch" mounts a new page and unmounts the old one, but the
    // header_element subtrees accumulate indefinitely.
    let mut reconciler = Reconciler::new(RecordingBackend::new());
    let rerender = noop_rerender();

    // Mount-then-unmount in a loop to simulate full page replacements
    // (as happens when switching between different component types in a NavigationView)
    for _cycle in 0..10 {
        let page = make_page();
        let id = reconciler.reconcile(None, &page, None, Rc::clone(&rerender));
        // Simulate navigating away — full unmount
        let _ = reconciler.reconcile(Some(&page), &Element::Empty, id, Rc::clone(&rerender));
    }

    let (creates, destroys) = count_creates_and_destroys(&reconciler.backend.ops);
    let leaked = creates.saturating_sub(destroys);

    println!("After 10 mount/unmount cycles:");
    println!("  Created:   {creates}");
    println!("  Destroyed: {destroys}");
    println!("  LEAKED:    {leaked}");

    // Verifies the fix: no controls leak across navigation cycles.
    // Before the fix, each cycle leaked 4 controls
    // (1 StackPanel + 3 TextBlocks from the Expander header).
    assert_eq!(
        leaked, 0,
        "BUG: {leaked} controls leaked across 10 navigation cycles. \
         Each Expander header_element subtree (4 controls) is leaked per cycle. \
         This directly causes the memory growth reported in #4491 and #4500."
    );
}
