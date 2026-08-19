use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::content_access as content_probe;
use super::*;
use crate::winui::collection::tests as collection_probe;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_COLLECTION_SELECTION_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn selection_updates_native_events_and_empty_state() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::collection_selection::collection_selection_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn collection_selection_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<usize>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_open_state = Rc::clone(&open_state);
    let selection_events = Rc::new(Cell::new(0usize));
    let events_for_render = Rc::clone(&selection_events);
    let root = component(move |cx| {
        let phase = cx.use_state(|| 0usize);
        let open = cx.use_state(|| true);
        publish_phase_state.borrow_mut().replace(phase.clone());
        publish_open_state.borrow_mut().replace(open.clone());
        let events = Rc::clone(&events_for_render);
        let close = open.clone();
        let list = list(phase.try_value().unwrap(), move |_| {
            events.set(events.get() + 1);
        });
        Application::new(if open.try_value().unwrap() {
            vec![
                Window::new("Collection selection fixture", list, move || {
                    close.set(false);
                })
                .build(),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let list = collection_probe::collections(reactor.engine().runtime(), false)[0];
        assert_phase(reactor.engine().runtime(), list, 0);
        assert_eq!(selection_events.get(), 0);
        collection_probe::set_selection(
            reactor.engine().runtime(),
            list,
            &CollectionSelection::new([2]),
        )
        .unwrap();
        reactor.pump();
        assert_eq!(selection_events.get(), 1);
        assert_eq!(
            collection_probe::selection(reactor.engine().runtime(), list).unwrap(),
            CollectionSelection::new([2])
        );
        selection_events.set(0);

        assert!(phase_state.borrow().as_ref().unwrap().try_set(1));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), list, 1);
        assert_eq!(selection_events.get(), 0);
        collection_probe::set_selection(
            reactor.engine().runtime(),
            list,
            &CollectionSelection::new([1_000, 1_003]),
        )
        .unwrap();
        reactor.pump();
        assert_eq!(selection_events.get(), 1);
        assert_eq!(
            collection_probe::selection(reactor.engine().runtime(), list).unwrap(),
            CollectionSelection::new([1_000, 1_003])
        );

        assert!(phase_state.borrow().as_ref().unwrap().try_set(2));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), list, 2);
        assert_eq!(selection_events.get(), 1);
        let empty = RuntimeProbe::new(reactor.engine().runtime()).children(list)[0];
        assert_eq!(
            content_probe::text(reactor.engine().runtime(), empty).unwrap(),
            "empty list"
        );

        assert!(phase_state.borrow().as_ref().unwrap().try_set(3));
        reactor.pump();
        assert_phase(reactor.engine().runtime(), list, 3);
        assert!(!RuntimeProbe::new(reactor.engine().runtime()).contains(empty));
        assert_eq!(selection_events.get(), 1);
        assert!(open_state.borrow().as_ref().unwrap().try_set(false));
        reactor.pump();
        Ok(())
    })
    .unwrap();
}

fn list(phase: usize, on_selection: impl Fn(CollectionSelection) + 'static) -> Element {
    let count = match phase {
        0 => 3,
        1 => 5,
        2 => 0,
        _ => 1,
    };
    let mut list = VirtualList::new(count, 300.0, |index| text_block(format!("row {index}")))
        .empty_state(text_block("empty list"))
        .on_item_invoked(|_| {});
    let selection = match phase {
        0 => {
            list = list.selection_mode(SelectionMode::Single);
            CollectionSelection::new([1])
        }
        1 => {
            list = list
                .item_keys(VirtualItemKeys::new((0..5).map(|index| 1_000 + index)))
                .selection_mode(SelectionMode::Extended);
            CollectionSelection::new([1_001, 1_004])
        }
        2 => {
            list = list
                .item_keys(VirtualItemKeys::new(Vec::<u64>::new()))
                .selection_mode(SelectionMode::Extended);
            CollectionSelection::default()
        }
        _ => {
            list = list
                .item_keys(VirtualItemKeys::new([1_000]))
                .selection_mode(SelectionMode::Extended);
            CollectionSelection::default()
        }
    };
    list.selection(selection, on_selection).build()
}

fn assert_phase(runtime: &WinUiRuntime, id: NodeId, phase: usize) {
    assert_eq!(
        collection_probe::item_count(runtime, id).unwrap(),
        match phase {
            0 => 3,
            1 => 5,
            2 => 0,
            _ => 1,
        }
    );
    assert!(collection_probe::item_click_enabled(runtime, id).unwrap());
    match phase {
        0 => {
            assert_eq!(
                collection_probe::selection_mode(runtime, id).unwrap(),
                SelectionMode::Single
            );
            assert_eq!(
                collection_probe::selection(runtime, id).unwrap(),
                CollectionSelection::new([1])
            );
        }
        1 => {
            assert_eq!(
                collection_probe::selection_mode(runtime, id).unwrap(),
                SelectionMode::Extended
            );
            assert_eq!(
                collection_probe::item_keys(runtime, id).unwrap(),
                vec![1_000, 1_001, 1_002, 1_003, 1_004]
            );
            assert_eq!(
                collection_probe::selection(runtime, id).unwrap(),
                CollectionSelection::new([1_001, 1_004])
            );
        }
        2 => assert_eq!(RuntimeProbe::new(runtime).children(id).len(), 1),
        _ => assert_eq!(
            collection_probe::item_keys(runtime, id).unwrap(),
            vec![1_000]
        ),
    }
}
