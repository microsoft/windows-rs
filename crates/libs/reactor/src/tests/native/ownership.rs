use super::*;

fn nullable_checked_values_preserve_ownership() {
    let values = Rc::new(RefCell::new(Vec::new()));
    let received = Rc::clone(&values);
    let callback = Callback::new(move |value| received.borrow_mut().push(value));
    let cases: [(View, View); 2] = [
        (
            CheckBox::new()
                .on_is_checked_changed(callback.clone())
                .into(),
            CheckBox::new()
                .is_checked(None)
                .on_is_checked_changed(callback.clone())
                .into(),
        ),
        (
            ToggleButton::new()
                .on_is_checked_changed(callback.clone())
                .into(),
            ToggleButton::new()
                .is_checked(None)
                .on_is_checked_changed(callback)
                .into(),
        ),
    ];
    for (omitted, declared) in cases {
        values.borrow_mut().clear();
        let mut runtime = Runtime::new(WinUiAdapter::default());
        runtime.update(omitted.clone()).unwrap();
        let object = runtime.graph().root().unwrap();
        runtime.adapter().simulate_is_checked(object, None).unwrap();
        assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
        assert_eq!(*values.borrow(), [None]);
        assert!(runtime.update(omitted.clone()).unwrap().is_empty());
        assert!(runtime.graph().properties(object).unwrap().is_empty());

        runtime.update(declared.clone()).unwrap();
        runtime
            .adapter()
            .simulate_is_checked(object, Some(true))
            .unwrap();
        assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
        runtime.update(declared).unwrap();
        assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
        assert_eq!(
            runtime.graph().properties(object).unwrap(),
            [Property {
                id: PropertyId::IsChecked,
                value: PropertyValue::OptionalBool(None),
            }]
        );
        runtime.update(omitted.clone()).unwrap();
        assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
        runtime.adapter().simulate_is_checked(object, None).unwrap();
        assert_eq!(runtime.dispatch_native_events().unwrap(), 1);
        assert!(runtime.update(omitted).unwrap().is_empty());
        assert!(runtime.graph().properties(object).unwrap().is_empty());
        assert_eq!(*values.borrow(), [None, Some(true), None]);
    }
}

fn controls(runtime: &Runtime<WinUiAdapter>) -> [ObjectId; 2] {
    runtime
        .graph()
        .children(runtime.graph().root().unwrap(), RelationId::Children)
        .unwrap()
        .try_into()
        .unwrap()
}

fn assert_values(runtime: &Runtime<WinUiAdapter>, toggle: bool, slider: f64) {
    let objects = controls(runtime);
    assert_eq!(
        runtime.adapter().toggle_switch_state(objects[0]).unwrap(),
        toggle
    );
    assert_eq!(
        runtime.adapter().slider_state(objects[1]).unwrap().2,
        slider
    );
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn native_edits_and_coercion_do_not_claim_undeclared_values() {
    // WinUI supports one Application lifetime per process.
    if std::env::var_os("REACTOR_OWNERSHIP_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::winui::tests::ownership::native_edits_and_coercion_do_not_claim_undeclared_values",
                "--ignored",
                "--nocapture",
            ])
            .env("REACTOR_OWNERSHIP_TEST_CHILD", "1")
            .status()
            .unwrap();
        assert!(status.success());
        return;
    }
    let completed = Rc::new(Cell::new(false));
    let finished = Rc::clone(&completed);
    App::run_with(move |app| {
        nullable_checked_values_preserve_ownership();
        let calls = Rc::new(Cell::new(0));
        let toggle_calls = Rc::clone(&calls);
        let toggle_changed = Callback::new(move |_: bool| toggle_calls.set(toggle_calls.get() + 1));
        let slider_calls = Rc::clone(&calls);
        let slider_changed = Callback::new(move |_: f64| slider_calls.set(slider_calls.get() + 1));
        let view = move |values: Option<(bool, f64)>| {
            let mut toggle = ToggleSwitch::new().on_toggled(toggle_changed.clone());
            let mut slider = Slider::new()
                .minimum(10.0)
                .on_value_changed(slider_changed.clone());
            if let Some((is_on, value)) = values {
                toggle = toggle.is_on(is_on);
                slider = slider.value(value);
            }
            StackPanel::new().children((toggle, slider))
        };
        let mut runtime = Runtime::new(WinUiAdapter::default());
        runtime.update(view(None)).unwrap();
        let root = runtime.graph().root().unwrap();
        let window = runtime.adapter_mut().create_window(root).unwrap();
        window.activate().unwrap();
        let runtime = RefCell::new(runtime);
        let window = RefCell::new(Some(window));
        let phase = Cell::new(0);
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(TimeSpan::try_from(Duration::from_millis(100)).unwrap())?;
        let app = app.clone();
        let tick = timer.Tick(move |sender, _| {
            let mut runtime = runtime.borrow_mut();
            assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
            if phase.get() == 0 {
                assert_values(&runtime, false, 10.0);
                assert!(runtime.update(view(None)).unwrap().is_empty());
                let objects = controls(&runtime);
                runtime
                    .adapter()
                    .set_toggle_switch_is_on(objects[0], true)
                    .unwrap();
                runtime
                    .adapter()
                    .set_slider_value(objects[1], 42.0)
                    .unwrap();
                assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
                assert!(runtime.update(view(None)).unwrap().is_empty());
                assert_values(&runtime, true, 42.0);

                runtime.update(view(Some((false, 20.0)))).unwrap();
                assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
                assert_values(&runtime, false, 20.0);
                runtime
                    .adapter()
                    .set_toggle_switch_is_on(objects[0], true)
                    .unwrap();
                runtime
                    .adapter()
                    .set_slider_value(objects[1], 30.0)
                    .unwrap();
                assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
                runtime.update(view(Some((false, 20.0)))).unwrap();
                assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
                assert_values(&runtime, false, 20.0);

                runtime.update(view(None)).unwrap();
                assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
                assert_values(&runtime, false, 10.0);
                assert!(runtime.update(view(None)).unwrap().is_empty());
                runtime
                    .adapter()
                    .set_toggle_switch_is_on(objects[0], true)
                    .unwrap();
                runtime
                    .adapter()
                    .set_slider_value(objects[1], 50.0)
                    .unwrap();
                assert_eq!(runtime.dispatch_native_events().unwrap(), 2);
                assert!(runtime.update(view(None)).unwrap().is_empty());
                assert_values(&runtime, true, 50.0);

                runtime.update(StackPanel::new()).unwrap();
                runtime.update(view(None)).unwrap();
                assert_ne!(controls(&runtime), objects);
                assert_eq!(runtime.dispatch_native_events().unwrap(), 0);
                assert_values(&runtime, false, 10.0);
                assert!(runtime.update(view(None)).unwrap().is_empty());
                phase.set(1);
            } else {
                assert_eq!(calls.get(), 6);
                sender.ok().unwrap().Stop().unwrap();
                window.borrow_mut().take().unwrap().close().unwrap();
                finished.set(true);
                app.exit().unwrap();
            }
        })?;
        timer.Start()?;
        Ok((timer, tick))
    })
    .unwrap();
    assert!(completed.get());
}
