use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::Duration;

use super::*;
use crate::winui::collection::tests as collection_probe;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_SINGLE_SELECTION_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn single_key_selection_feedback_and_restoration() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::single_selection::single_selection_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn single_selection_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let combo_state = Rc::new(RefCell::new(None::<State<Option<u64>>>));
    let radio_state = Rc::new(RefCell::new(None::<State<Option<u64>>>));
    let items_state = Rc::new(RefCell::new(None::<State<SelectorItems>>));
    let version_state = Rc::new(RefCell::new(None::<State<u64>>));
    let accept_state = Rc::new(RefCell::new(None::<State<bool>>));
    let revision_state = Rc::new(RefCell::new(None::<State<u64>>));
    let open_state = Rc::new(RefCell::new(None::<State<bool>>));
    let events = Rc::new(RefCell::new(Vec::new()));
    let publish_combo = Rc::clone(&combo_state);
    let publish_radio = Rc::clone(&radio_state);
    let publish_items = Rc::clone(&items_state);
    let publish_version = Rc::clone(&version_state);
    let publish_accept = Rc::clone(&accept_state);
    let publish_revision = Rc::clone(&revision_state);
    let publish_open = Rc::clone(&open_state);
    let render_events = Rc::clone(&events);
    let root = component(move |cx| {
        let combo = cx.use_state(|| None::<u64>);
        let radio = cx.use_state(|| None::<u64>);
        let items = cx.use_state(|| SelectorItems::new([(10, "Ten"), (20, "Twenty")]));
        let version = cx.use_state(|| 1u64);
        let accept = cx.use_state(|| true);
        let revision = cx.use_state(|| 0u64);
        let open = cx.use_state(|| true);
        *publish_combo.borrow_mut() = Some(combo.clone());
        *publish_radio.borrow_mut() = Some(radio.clone());
        *publish_items.borrow_mut() = Some(items.clone());
        *publish_version.borrow_mut() = Some(version.clone());
        *publish_accept.borrow_mut() = Some(accept.clone());
        *publish_revision.borrow_mut() = Some(revision.clone());
        *publish_open.borrow_mut() = Some(open.clone());
        _ = revision.value();
        let current_items = items.value();
        let current_version = version.value();
        let combo_accept = accept.clone();
        let radio_accept = accept;
        let combo_update = combo.clone();
        let radio_update = radio.clone();
        let combo_events = Rc::clone(&render_events);
        let radio_events = Rc::clone(&render_events);
        let close = open.clone();
        Application::new(if open.value() {
            vec![
                Window::new(
                    "Single selection fixture",
                    StackPanel::new([
                        ComboBox::from_items(current_items.clone(), move |key| {
                            combo_events
                                .borrow_mut()
                                .push(("combo", current_version, key));
                            if combo_accept.try_value() == Some(true) {
                                combo_update.set(key);
                            }
                        })
                        .selected_key(combo.value())
                        .build(),
                        RadioButtons::from_items(current_items, move |key| {
                            radio_events
                                .borrow_mut()
                                .push(("radio", current_version, key));
                            if radio_accept.try_value() == Some(true) {
                                radio_update.set(key);
                            }
                        })
                        .selected_key(radio.value())
                        .build(),
                    ])
                    .build(),
                    move || close.set(false),
                )
                .build(),
            ]
        } else {
            Vec::new()
        })
        .build()
    });

    let outcome = Rc::new(RefCell::new(None::<Result<(), String>>));
    let finish_outcome = Rc::clone(&outcome);
    let timer = Rc::new(RefCell::new(None::<TestTimer>));
    let finish_timer = Rc::clone(&timer);
    run_app_fixture(root, move |reactor| {
        let combo = collection_probe::SingleSelectorProbe::new(
            reactor.engine().runtime(),
            NativeKind::ComboBox,
        );
        let radio = collection_probe::SingleSelectorProbe::new(
            reactor.engine().runtime(),
            NativeKind::RadioButtons,
        );
        assert_eq!(combo.selection()?, None);
        assert_eq!(radio.selection()?, None);

        let combo_value = combo_state.borrow().as_ref().unwrap().clone();
        let radio_value = radio_state.borrow().as_ref().unwrap().clone();
        let items = items_state.borrow().as_ref().unwrap().clone();
        let version = version_state.borrow().as_ref().unwrap().clone();
        let accept = accept_state.borrow().as_ref().unwrap().clone();
        let revision = revision_state.borrow().as_ref().unwrap().clone();
        let close = open_state.borrow().as_ref().unwrap().clone();
        let phase = Rc::new(Cell::new(0_u8));
        let event_count = Rc::new(Cell::new(0_usize));
        let timer_for_tick = Rc::clone(&finish_timer);
        *finish_timer.borrow_mut() = Some(TestTimer::repeating(
            Duration::from_millis(100),
            move || {
                let step = (|| -> Result<bool, String> {
                    let current = phase.get();
                    match current {
                        0 => {
                            combo
                                .set_selection(Some(10))
                                .map_err(|error| error.to_string())?;
                            radio
                                .set_selection(Some(20))
                                .map_err(|error| error.to_string())?;
                        }
                        1 => {
                            let expected = [("combo", 1, Some(10)), ("radio", 1, Some(20))];
                            if events.borrow().as_slice() != expected {
                                return Err(format!(
                                    "accepted selection events were {:?}",
                                    *events.borrow()
                                ));
                            }
                            if !version.try_set(2) {
                                return Err("version state was stale".into());
                            }
                        }
                        2 => {
                            combo
                                .set_selection(None)
                                .map_err(|error| error.to_string())?;
                            radio
                                .set_selection(None)
                                .map_err(|error| error.to_string())?;
                        }
                        3 => {
                            let expected = [
                                ("combo", 1, Some(10)),
                                ("radio", 1, Some(20)),
                                ("combo", 2, None),
                                ("radio", 2, None),
                            ];
                            if events.borrow().as_slice() != expected {
                                return Err(format!(
                                    "cleared selection events were {:?}",
                                    *events.borrow()
                                ));
                            }
                            if !accept.try_set(false) {
                                return Err("accept state was stale".into());
                            }
                        }
                        4 => {
                            combo
                                .set_selection(Some(20))
                                .map_err(|error| error.to_string())?;
                            radio
                                .set_selection(Some(10))
                                .map_err(|error| error.to_string())?;
                        }
                        5 => {
                            if !revision.try_update(|value| *value += 1) {
                                return Err("revision state was stale".into());
                            }
                        }
                        6 => {
                            if combo
                                .selection()
                                .map_err(|error| error.to_string())?
                                .is_some()
                                || radio
                                    .selection()
                                    .map_err(|error| error.to_string())?
                                    .is_some()
                            {
                                return Err("rejected native selection was not restored".into());
                            }
                            event_count.set(events.borrow().len());
                            if !combo_value.try_set(Some(99)) || !radio_value.try_set(Some(99)) {
                                return Err("selection state was stale".into());
                            }
                        }
                        7 => {
                            if combo
                                .selection()
                                .map_err(|error| error.to_string())?
                                .is_some()
                                || radio
                                    .selection()
                                    .map_err(|error| error.to_string())?
                                    .is_some()
                            {
                                return Err(
                                    "missing selection key reached the native controls".into()
                                );
                            }
                            if !items.try_set(SelectorItems::new([
                                (10, "Ten"),
                                (20, "Twenty"),
                                (99, "Ninety-nine"),
                            ])) {
                                return Err("items state was stale".into());
                            }
                        }
                        8 => {
                            if combo.selection().map_err(|error| error.to_string())? != Some(99)
                                || radio.selection().map_err(|error| error.to_string())? != Some(99)
                            {
                                return Err("newly available selection key was not applied".into());
                            }
                            if events.borrow().len() != event_count.get() {
                                return Err("declarative selection writes echoed as events".into());
                            }
                            return Ok(true);
                        }
                        _ => unreachable!(),
                    }
                    phase.set(current + 1);
                    Ok(false)
                })();

                match step {
                    Ok(false) => {}
                    result => {
                        timer_for_tick.borrow_mut().take();
                        *finish_outcome.borrow_mut() = Some(result.map(|_| ()));
                        close.set(false);
                    }
                }
            },
        )?);
        Ok(())
    })
    .unwrap();

    if let Err(error) = outcome.borrow_mut().take().unwrap() {
        panic!("{error}");
    }
}
