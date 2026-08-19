use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

use super::navigation_access as navigation_probe;
use super::*;

const FIXTURE_ENV: &str = "WINDOWS_REACTOR_NAVIGATION_PROPERTIES_FIXTURE";

#[test]
#[ignore = "requires the Windows App Runtime"]
fn navigation_view_updates_native_properties_and_selection() {
    let output = test_reactor_support::run_test_process(
        "winui::tests::navigation_properties::navigation_properties_fixture",
        &[(FIXTURE_ENV, "run")],
        Duration::from_secs(30),
    )
    .unwrap();
    test_reactor_support::assert_success(output);
}

#[test]
fn navigation_properties_fixture() {
    if std::env::var_os(FIXTURE_ENV).is_none() {
        return;
    }

    bootstrap().unwrap();
    let phase_state = Rc::new(RefCell::new(None::<State<bool>>));
    let publish_phase_state = Rc::clone(&phase_state);
    let root = component(move |cx| {
        let changed = cx.use_state(|| false);
        let selected = cx.use_state(|| Some(10_u64));
        publish_phase_state.borrow_mut().replace(changed.clone());
        let set_selected = selected.clone();
        let items = if changed.value() {
            [
                NavigationItem::new(20, "Settings").icon(Icon::symbol(IconSymbol::SETTINGS)),
                NavigationItem::new(10, "Home").icon(Icon::symbol(IconSymbol::HOME)),
            ]
        } else {
            [
                NavigationItem::new(10, "Home").icon(Icon::symbol(IconSymbol::HOME)),
                NavigationItem::new(20, "Settings").icon(Icon::symbol(IconSymbol::SETTINGS)),
            ]
        };
        NavigationView::new(items, text_block("content"), move |key| {
            set_selected.set(key);
        })
        .selected_key(selected.value())
        .pane_title(if changed.value() {
            "Changed"
        } else {
            "Initial"
        })
        .settings_visible(!changed.value())
        .pane_toggle_visible(changed.value())
        .pane_open(!changed.value(), |_| {})
        .open_pane_length(if changed.value() { 280.0 } else { 240.0 })
        .pane_display_mode(if changed.value() {
            NavigationPaneDisplayMode::Top
        } else {
            NavigationPaneDisplayMode::Left
        })
        .pane_footer(text_block("footer"))
        .build()
    });

    run_app_fixture(root, move |reactor| {
        let navigation =
            RuntimeProbe::new(reactor.engine().runtime()).nodes(NativeKind::NavigationView)[0];
        assert_eq!(
            navigation_probe::properties(reactor.engine().runtime(), navigation).unwrap(),
            (false, true, false, 240.0, "Initial".into(), 1, 2, Some(10))
        );

        navigation_probe::select(reactor.engine().runtime(), navigation, 20).unwrap();
        reactor.pump();
        assert_eq!(
            navigation_probe::properties(reactor.engine().runtime(), navigation)
                .unwrap()
                .7,
            Some(20)
        );
        assert!(phase_state.borrow().as_ref().unwrap().try_set(true));
        reactor.pump();
        assert_eq!(
            navigation_probe::properties(reactor.engine().runtime(), navigation).unwrap(),
            (false, false, true, 280.0, "Changed".into(), 2, 2, Some(20))
        );
        Ok(())
    })
    .unwrap();
}
