use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::content_access as content_probe;
use super::*;
use crate::winui::container::tests as container_probe;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_CONTAINER_PROPERTIES_FIXTURE";

fn count_callback<T>(callbacks: &Rc<Cell<usize>>) -> impl Fn(T) + 'static {
    let callbacks = Rc::clone(callbacks);
    move |_| callbacks.set(callbacks.get() + 1)
}

#[test]
#[ignore = "requires the Windows App Runtime"]
fn scrolling_and_panel_properties_update_and_reset() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::container_properties::container_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn container_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let callbacks = Rc::new(Cell::new(0usize));
    let callbacks_for_render = Rc::clone(&callbacks);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let close = open.clone();
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new(
                    "Container properties fixture",
                    content(phase.try_value().unwrap(), &callbacks_for_render),
                    move || {
                        close.set(false);
                    },
                )
                .build(),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        assert_phase(reactor.engine().runtime(), 0);
        assert_eq!(callbacks.get(), 0);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 1);
        assert_eq!(callbacks.get(), 0);
        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), 2);
        assert_eq!(callbacks.get(), 0);
        let split = only_node(reactor.engine().runtime(), NativeKind::SplitView);
        let expander = only_node(reactor.engine().runtime(), NativeKind::Expander);
        container_probe::set_split_view_open(reactor.engine().runtime(), split, false)?;
        container_probe::set_expander(reactor.engine().runtime(), expander, true)?;
        assert!(container_probe::split_view(reactor.engine().runtime(), split)?.1);
        assert!(!container_probe::expander(
            reactor.engine().runtime(),
            expander
        )?);
        reactor.pump();
        assert_eq!(callbacks.get(), 0);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn content(phase: usize, callbacks: &Rc<Cell<usize>>) -> Element {
    let count_empty = |callbacks: &Rc<Cell<usize>>| {
        let callbacks = Rc::clone(callbacks);
        move || callbacks.set(callbacks.get() + 1)
    };

    let viewer =
        ScrollViewer::new(text_block("scroll viewer")).on_view_changed(count_callback(callbacks));
    let scroll =
        ScrollView::new(text_block("scroll view")).on_view_changed(count_callback(callbacks));
    let viewbox = Viewbox::new(text_block("viewbox"));
    let split = SplitView::new(
        text_block("split content"),
        text_block("split pane"),
        count_empty(callbacks),
    );
    let expander = Expander::new(
        text_block("header"),
        text_block("content"),
        count_callback(callbacks),
    );
    let (viewer, scroll, viewbox, split, expander) = match phase {
        1 => (
            viewer
                .horizontal_scroll_bar_visibility(ScrollBarVisibility::Visible)
                .vertical_scroll_bar_visibility(ScrollBarVisibility::Hidden),
            scroll
                .horizontal_scroll_bar_visibility(ScrollViewBarVisibility::Visible)
                .vertical_scroll_bar_visibility(ScrollViewBarVisibility::Hidden)
                .content_orientation(ScrollOrientation::Both),
            viewbox.stretch(Stretch::UniformToFill),
            split
                .display_mode(SplitViewDisplayMode::CompactOverlay)
                .is_pane_open(false)
                .open_pane_length(280.0)
                .compact_pane_length(40.0),
            expander.expanded(true),
        ),
        2 => (
            viewer,
            scroll,
            viewbox,
            SplitView::display(text_block("split content"), text_block("split pane")),
            Expander::display(text_block("header"), text_block("content")),
        ),
        _ => (viewer, scroll, viewbox, split, expander),
    };

    StackPanel::new([
        viewer.build(),
        scroll.build(),
        viewbox.build(),
        split.build(),
        expander.build(),
    ])
    .build()
}

fn only_node(runtime: &WinUiRuntime, kind: NativeKind) -> NodeId {
    let nodes = RuntimeProbe::new(runtime).nodes(kind);
    assert_eq!(nodes.len(), 1, "expected one {kind:?} node");
    nodes[0]
}

fn assert_phase(runtime: &WinUiRuntime, phase: usize) {
    let viewer = only_node(runtime, NativeKind::ScrollViewer);
    let scroll = only_node(runtime, NativeKind::ScrollView);
    let viewbox = only_node(runtime, NativeKind::Viewbox);
    let split = only_node(runtime, NativeKind::SplitView);
    let expander = only_node(runtime, NativeKind::Expander);
    if phase == 1 {
        assert_eq!(
            container_probe::scroll_viewer(runtime, viewer).unwrap(),
            (ScrollBarVisibility::Visible, ScrollBarVisibility::Hidden)
        );
        assert_eq!(
            container_probe::scroll_view(runtime, scroll).unwrap(),
            (
                ScrollViewBarVisibility::Visible,
                ScrollViewBarVisibility::Hidden,
                ScrollOrientation::Both,
            )
        );
        assert_eq!(
            content_probe::viewbox_stretch(runtime, viewbox).unwrap(),
            Stretch::UniformToFill
        );
        assert_eq!(
            container_probe::split_view(runtime, split).unwrap(),
            (SplitViewDisplayMode::CompactOverlay, false, 280.0, 40.0)
        );
        assert!(container_probe::expander(runtime, expander).unwrap());
    } else {
        assert_eq!(
            container_probe::scroll_viewer(runtime, viewer).unwrap(),
            (ScrollBarVisibility::Disabled, ScrollBarVisibility::Auto)
        );
        assert_eq!(
            container_probe::scroll_view(runtime, scroll).unwrap(),
            (
                ScrollViewBarVisibility::Auto,
                ScrollViewBarVisibility::Auto,
                ScrollOrientation::Vertical,
            )
        );
        assert_eq!(
            content_probe::viewbox_stretch(runtime, viewbox).unwrap(),
            Stretch::Uniform
        );
        assert_eq!(
            container_probe::split_view(runtime, split).unwrap(),
            (SplitViewDisplayMode::Inline, true, 320.0, 48.0)
        );
        assert!(!container_probe::expander(runtime, expander).unwrap());
    }
}
