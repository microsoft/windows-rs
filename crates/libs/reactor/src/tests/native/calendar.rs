use super::*;

fn date(day: i64) -> DateTime {
    DateTime::from_unix_secs(1_767_225_600 + day * 86_400)
}

fn picker(runtime: &Runtime<WinUiAdapter>) -> native::CalendarDatePicker {
    let root = runtime.graph().root().unwrap();
    let Some(Handle::Generated(GeneratedHandle::CalendarDatePicker(picker))) =
        runtime.adapter().handles.get(&root)
    else {
        panic!()
    };
    picker.value.clone()
}

fn assert_native_date(runtime: &Runtime<WinUiAdapter>, expected: Option<DateTime>) {
    match picker(runtime).Date() {
        Ok(date) => assert_eq!(Some(date), expected),
        Err(error) => {
            assert!(error.code().is_ok());
            assert_eq!(expected, None);
        }
    }
}

fn assert_date(runtime: &mut Runtime<WinUiAdapter>, expected: Option<DateTime>) {
    runtime.dispatch_native_events().unwrap();
    assert_native_date(runtime, expected);
    let root = runtime.graph().root().unwrap();
    let observed = runtime
        .graph()
        .properties(root)
        .unwrap()
        .iter()
        .find(|property| property.id == PropertyId::Date);
    assert_eq!(
        observed.unwrap().value,
        PropertyValue::OptionalDateTime(expected)
    );
}

fn assert_unowned_date(runtime: &mut Runtime<WinUiAdapter>, expected: Option<DateTime>) {
    runtime.dispatch_native_events().unwrap();
    assert_native_date(runtime, expected);
    let root = runtime.graph().root().unwrap();
    assert!(
        runtime
            .graph()
            .properties(root)
            .unwrap()
            .iter()
            .all(|property| property.id != PropertyId::Date)
    );
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn controlled_calendar_dates() {
    // WinUI supports one Application lifetime per process.
    if std::env::var_os("REACTOR_CALENDAR_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::winui::tests::calendar::controlled_calendar_dates",
                "--ignored",
                "--nocapture",
            ])
            .env("REACTOR_CALENDAR_TEST_CHILD", "1")
            .status()
            .unwrap();
        assert!(status.success());
        return;
    }
    let completed = Rc::new(Cell::new(false));
    let finished = Rc::clone(&completed);
    App::run_with(move |app| {
        let callbacks = Rc::new(RefCell::new(Vec::new()));
        let changed = Rc::clone(&callbacks);
        let callback = Callback::new(move |value| changed.borrow_mut().push(value));
        let view_callback = callback.clone();
        let view = move |min, max, selected| {
            CalendarDatePicker::new()
                .date(selected)
                .max_date(date(max))
                .min_date(date(min))
                .on_date_changed(view_callback.clone())
        };
        let mut runtime = Runtime::new(WinUiAdapter::default());
        runtime
            .update(CalendarDatePicker::new().on_date_changed(callback.clone()))
            .unwrap();
        picker(&runtime).SetDate(Some(date(5)))?;
        assert_unowned_date(&mut runtime, Some(date(5)));
        runtime
            .update(
                CalendarDatePicker::new()
                    .placeholder_text("Choose a date")
                    .on_date_changed(callback.clone()),
            )
            .unwrap();
        assert_unowned_date(&mut runtime, Some(date(5)));
        assert_eq!(*callbacks.borrow(), [Some(date(5))]);
        callbacks.borrow_mut().clear();
        runtime
            .update(
                CalendarDatePicker::new()
                    .date(date(20))
                    .min_date(date(0))
                    .max_date(date(10)),
            )
            .unwrap();
        assert_date(&mut runtime, Some(date(10)));
        runtime.update(view(0, 10, Some(date(5)))).unwrap();
        assert_date(&mut runtime, Some(date(5)));
        let root = runtime.graph().root().unwrap();
        let window = runtime.adapter_mut().create_window(root).unwrap();
        window.activate().unwrap();
        let window = RefCell::new(Some(window));
        let runtime = RefCell::new(runtime);
        let phase = Cell::new(0);
        let timer = native::DispatcherQueue::GetForCurrentThread()?.CreateTimer()?;
        timer.SetInterval(TimeSpan::try_from(Duration::from_millis(100)).unwrap())?;
        let app = app.clone();
        let tick = timer.Tick(move |sender, _| {
            let mut runtime = runtime.borrow_mut();
            match phase.get() {
                0 => {
                    runtime.update(view(0, 10, None)).unwrap();
                    assert_date(&mut runtime, None);
                    runtime.update(view(0, 10, Some(date(3)))).unwrap();
                    assert_date(&mut runtime, Some(date(3)));
                    assert!(callbacks.borrow().is_empty());

                    picker(&runtime).SetDate(Some(date(8))).unwrap();
                    assert_date(&mut runtime, Some(date(8)));
                    picker(&runtime).SetDate(None).unwrap();
                    assert_date(&mut runtime, None);
                    assert_eq!(*callbacks.borrow(), [Some(date(8)), None]);
                    callbacks.borrow_mut().clear();

                    runtime.update(view(0, 10, Some(date(3)))).unwrap();
                    assert_date(&mut runtime, Some(date(3)));
                    runtime.update(view(0, 10, Some(date(20)))).unwrap();
                    assert_date(&mut runtime, Some(date(10)));
                    assert!(
                        runtime
                            .update(view(0, 10, Some(date(10))))
                            .unwrap()
                            .is_empty()
                    );
                    runtime.update(view(0, 10, Some(date(-10)))).unwrap();
                    assert_date(&mut runtime, Some(date(0)));

                    runtime.update(view(0, 10, Some(date(10)))).unwrap();
                    assert_date(&mut runtime, Some(date(10)));
                    runtime.update(view(0, 10, Some(date(20)))).unwrap();
                    assert_date(&mut runtime, Some(date(10)));
                    runtime.update(view(0, 30, Some(date(20)))).unwrap();
                    assert_date(&mut runtime, Some(date(20)));

                    runtime.update(view(0, 10, Some(date(5)))).unwrap();
                    runtime.update(view(7, 10, Some(date(5)))).unwrap();
                    assert_date(&mut runtime, Some(date(5)));
                    runtime.update(view(7, 10, Some(date(8)))).unwrap();
                    assert_date(&mut runtime, Some(date(8)));

                    runtime.update(view(20, 30, Some(date(25)))).unwrap();
                    assert_date(&mut runtime, Some(date(25)));
                    runtime.update(view(-20, -10, Some(date(-15)))).unwrap();
                    assert_date(&mut runtime, Some(date(-15)));
                    for day in [-100_000, 100_000] {
                        let default = native::CalendarDatePicker::new().unwrap();
                        default.SetDate(Some(date(day))).unwrap();
                        runtime
                            .update(
                                CalendarDatePicker::new()
                                    .date(date(day))
                                    .on_date_changed(callback.clone()),
                            )
                            .unwrap();
                        assert_date(&mut runtime, Some(default.Date().unwrap()));
                    }
                    runtime
                        .update(CalendarDatePicker::new().on_date_changed(callback.clone()))
                        .unwrap();
                    assert_unowned_date(&mut runtime, None);
                    let omitted = CalendarDatePicker::new().on_date_changed(callback.clone());
                    assert!(runtime.update(omitted.clone()).unwrap().is_empty());
                    picker(&runtime).SetDate(Some(date(6))).unwrap();
                    assert_unowned_date(&mut runtime, Some(date(6)));
                    assert!(runtime.update(omitted).unwrap().is_empty());
                    assert_unowned_date(&mut runtime, Some(date(6)));
                    assert_eq!(*callbacks.borrow(), [Some(date(6))]);
                    callbacks.borrow_mut().clear();

                    runtime
                        .update(view(0, 10, Some(date(5))).is_calendar_open(true))
                        .unwrap();
                    assert_date(&mut runtime, Some(date(5)));
                }
                1 => {
                    assert_date(&mut runtime, Some(date(5)));
                    runtime
                        .update(view(7, 10, Some(date(5))).is_calendar_open(true))
                        .unwrap();
                    assert_date(&mut runtime, Some(date(5)));
                }
                2 => {
                    assert_date(&mut runtime, Some(date(5)));
                    runtime
                        .update(view(20, 30, Some(date(25))).is_calendar_open(true))
                        .unwrap();
                    assert_date(&mut runtime, Some(date(25)));
                }
                3 => {
                    assert_date(&mut runtime, Some(date(25)));
                    runtime
                        .update(view(-20, -10, Some(date(-15))).is_calendar_open(true))
                        .unwrap();
                    assert_date(&mut runtime, Some(date(-15)));
                }
                4 => {
                    assert_date(&mut runtime, Some(date(-15)));
                    runtime
                        .update(view(-20, -10, None).is_calendar_open(false))
                        .unwrap();
                    assert_date(&mut runtime, None);
                }
                _ => {
                    assert_date(&mut runtime, None);
                    assert!(callbacks.borrow().is_empty(), "{:?}", callbacks.borrow());
                    sender.ok().unwrap().Stop().unwrap();
                    window.borrow_mut().take().unwrap().close().unwrap();
                    finished.set(true);
                    app.exit().unwrap();
                }
            }
            phase.set(phase.get() + 1);
        })?;
        timer.Start()?;
        Ok((timer, tick))
    })
    .unwrap();
    assert!(completed.get());
}
