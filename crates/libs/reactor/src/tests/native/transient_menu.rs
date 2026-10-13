use super::*;

#[test]
fn application_menu_rejects_duplicate_nested_keys() {
    let menu = [
        MenuItem::item("duplicate", "First"),
        MenuItem::submenu(
            "submenu",
            "Submenu",
            [MenuItem::item("duplicate", "Second")],
        ),
    ];
    assert!(validate_menu_items(&menu, &mut HashSet::new()).is_err());
}

#[test]
fn retired_popup_callbacks_cannot_deliver_to_a_new_popup() {
    let delivered = Rc::new(RefCell::new(Vec::new()));
    let record = Rc::clone(&delivered);
    let callback = Callback::new(move |key| record.borrow_mut().push(key));
    let first_live = Rc::new(Cell::new(true));
    let first = menu_callback(
        Rc::new(RefCell::new(callback.clone())),
        Rc::clone(&first_live),
    );
    first.call("first".into());
    first_live.set(false);
    let second_live = Rc::new(Cell::new(true));
    let second = menu_callback(Rc::new(RefCell::new(callback)), Rc::clone(&second_live));
    first.call("stale".into());
    second.call("second".into());
    second_live.set(false);
    second.call("closed".into());
    assert_eq!(
        *delivered.borrow(),
        [Key::from("first"), Key::from("second")]
    );
}

#[test]
fn popup_callback_replacement_uses_latest_handler_without_holding_a_borrow() {
    let delivered = Rc::new(RefCell::new(Vec::new()));
    let record = Rc::clone(&delivered);
    let callback = Rc::new(RefCell::new(Callback::new(move |key| {
        record.borrow_mut().push((0, key));
    })));
    let live = Rc::new(Cell::new(true));
    let guarded = menu_callback(Rc::clone(&callback), Rc::clone(&live));
    guarded.call("first".into());
    let record = Rc::clone(&delivered);
    let slot = Rc::downgrade(&callback);
    *callback.borrow_mut() = Callback::new(move |key| {
        record.borrow_mut().push((1, key));
        *slot.upgrade().unwrap().borrow_mut() = Callback::new(|_| {});
    });
    guarded.call("second".into());
    live.set(false);
    guarded.call("closed".into());
    assert_eq!(
        *delivered.borrow(),
        [(0, Key::from("first")), (1, Key::from("second"))]
    );
}

fn assert_pending_callback_update(host: &TransientMenuHost) {
    let delivered = Rc::new(RefCell::new(Vec::new()));
    let record = Rc::clone(&delivered);
    let handle = host.handle();
    host.state.borrow_mut().pending =
        Some(Menu::new([MenuItem::item("command", "Command")], |_| {
            panic!("pending menu retained its old callback");
        }));
    let generation = host.state.borrow().generation;
    handle.update_callback(&Callback::new(move |key| record.borrow_mut().push(key)));
    let callback = {
        let state = host.state.borrow();
        assert_eq!(state.generation, generation);
        assert!(state.menu.is_none());
        state.pending.as_ref().unwrap().on_click.clone()
    };
    callback.call("command".into());
    handle.hide().unwrap();
    assert!(!handle.is_open());
    assert_eq!(*delivered.borrow(), [Key::from("command")]);
}

#[test]
fn system_theme_preference_preserves_query_errors() {
    assert_eq!(theme_preference(0, 0).unwrap(), ElementTheme::Dark);
    assert_eq!(theme_preference(0, 1).unwrap(), ElementTheme::Light);
    for missing in [ERROR_FILE_NOT_FOUND, ERROR_PATH_NOT_FOUND] {
        assert_eq!(theme_preference(missing, 0).unwrap(), ElementTheme::Default);
    }
    assert_eq!(
        theme_preference(5, 0).unwrap_err().code(),
        windows_core::WIN32_ERROR(5).to_hresult()
    );
}

#[test]
#[ignore = "requires an interactive WinUI desktop"]
fn system_menu_theme_updates_live_and_propagates_errors() {
    // WinUI supports one Application lifetime per process.
    if std::env::var_os("REACTOR_MENU_THEME_TEST_CHILD").is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "native::transient_menu::tests::system_menu_theme_updates_live_and_propagates_errors",
                "--ignored",
                "--nocapture",
            ])
            .env("REACTOR_MENU_THEME_TEST_CHILD", "1")
            .status()
            .unwrap();
        assert!(status.success());
        return;
    }

    fn assert_theme(state: &TransientMenuState, expected: ElementTheme) {
        assert_eq!(state.theme.anchor.ActualTheme().unwrap(), expected);
        let flyout = state
            .menu
            .as_ref()
            .unwrap()
            .flyout
            .cast::<MenuFlyout>()
            .unwrap();
        for index in 0..2 {
            assert_eq!(
                flyout
                    .Items()
                    .unwrap()
                    .GetAt(index)
                    .unwrap()
                    .cast::<IFrameworkElement>()
                    .unwrap()
                    .ActualTheme()
                    .unwrap(),
                expected,
            );
        }
    }

    let completed = Rc::new(Cell::new(false));
    let finished = Rc::clone(&completed);
    let error = App::run_with(move |_| {
        let dispatcher = DispatcherQueue::GetForCurrentThread()?;
        let host = TransientMenuHost::new(dispatcher.clone(), MenuTheme::System)?;
        assert_pending_callback_update(&host);
        let default = TransientMenuHost::new(dispatcher.clone(), MenuTheme::Application)?;
        let default_theme = default.state.borrow().theme.anchor.ActualTheme()?;
        default
            .state
            .borrow()
            .theme
            .reader
            .replace(Some(Rc::new(|| {
                panic!("application menus must not read the taskbar preference")
            })));
        default.state.borrow().theme.refresh()?;
        assert_eq!(
            default.state.borrow().theme.anchor.ActualTheme()?,
            default_theme
        );
        let preference = Rc::new(Cell::new((0, 0)));
        let reads = Rc::new(Cell::new(0));
        let read_preference = Rc::clone(&preference);
        let count_reads = Rc::clone(&reads);
        host.state
            .borrow()
            .theme
            .reader
            .replace(Some(Rc::new(move || {
                count_reads.set(count_reads.get() + 1);
                let (status, value) = read_preference.get();
                theme_preference(status, value)
            })));
        let handle = host.handle();
        let menu = Menu::new(
            [
                MenuItem::item("command", "Command"),
                MenuItem::submenu("submenu", "Submenu", [MenuItem::item("child", "Child")]),
            ],
            |_: Key| {},
        );
        handle.show(ScreenPoint::new(600, 400), menu.clone())?;
        let timer = dispatcher.CreateTimer()?;
        timer.SetInterval(TimeSpan::try_from(Duration::from_millis(300)).unwrap())?;
        timer.SetIsRepeating(true)?;
        let phase = Cell::new(0);
        let scheduled_reads = Cell::new(0);
        let retired_reads = Rc::new(Cell::new(0));
        let invalidated_reads = Rc::new(Cell::new(0));
        let tick = timer.Tick(move |_, _| {
            let result = (|| -> windows_core::Result<()> {
                let state = handle.state.upgrade().unwrap();
                let theme = Rc::clone(&state.borrow().theme);
                match phase.get() {
                    0 => {
                        assert_theme(&state.borrow(), ElementTheme::Dark);
                        preference.set((0, 1));
                        let before = reads.get();
                        scheduled_reads.set(before + 1);
                        // The window procedure must not borrow popup state or run XAML work.
                        let state = state.borrow_mut();
                        unsafe {
                            SendMessageW(state.host.hwnd().cast(), WM_SETTINGCHANGE as u32, 0, 0);
                            SendMessageW(state.host.hwnd().cast(), WM_THEMECHANGED as u32, 0, 0);
                        }
                        assert!(theme.refresh_pending.get());
                        assert_eq!(reads.get(), before);
                    }
                    1 => {
                        assert_theme(&state.borrow(), ElementTheme::Light);
                        assert_eq!(reads.get(), scheduled_reads.get());
                        assert!(!theme.refresh_pending.get());
                        handle.hide()?;
                        preference.set((0, 0));
                        handle.show(ScreenPoint::new(600, 400), menu.clone())?;
                        assert_eq!(theme.applied.get(), ElementTheme::Dark);
                    }
                    2 => {
                        assert_theme(&state.borrow(), ElementTheme::Dark);
                        preference.set((ERROR_FILE_NOT_FOUND, 0));
                        theme.schedule(&dispatcher)?;
                    }
                    3 => {
                        assert_eq!(theme.applied.get(), ElementTheme::Default);
                        assert_theme(&state.borrow(), default_theme);
                        let retiring =
                            TransientMenuHost::new(dispatcher.clone(), MenuTheme::System)?;
                        let count = Rc::clone(&retired_reads);
                        retiring
                            .state
                            .borrow()
                            .theme
                            .reader
                            .replace(Some(Rc::new(move || {
                                count.set(count.get() + 1);
                                Ok(ElementTheme::Light)
                            })));
                        let weak = Rc::downgrade(&retiring.state.borrow().theme);
                        retiring.state.borrow().theme.schedule(&dispatcher)?;
                        drop(retiring);
                        assert!(weak.upgrade().is_none());
                    }
                    4 => {
                        assert_eq!(retired_reads.get(), 0);
                        handle.hide()?;
                        preference.set((5, 0));
                        let error = handle
                            .show(ScreenPoint::new(600, 400), menu.clone())
                            .unwrap_err();
                        assert_eq!(error.code(), windows_core::WIN32_ERROR(5).to_hresult());
                        assert!(!handle.is_open());
                        preference.set((ERROR_FILE_NOT_FOUND, 0));
                        handle.show(ScreenPoint::new(600, 400), menu.clone())?;
                        let count = Rc::clone(&invalidated_reads);
                        let hwnd = state.borrow().host.hwnd();
                        theme.reader.replace(Some(Rc::new(move || {
                            count.set(count.get() + 1);
                            if count.get() == 1 {
                                // Invalidate the snapshot while its refresh is still running.
                                unsafe {
                                    SendMessageW(hwnd.cast(), WM_SETTINGCHANGE as u32, 0, 0);
                                }
                                Ok(ElementTheme::Light)
                            } else {
                                Ok(ElementTheme::Dark)
                            }
                        })));
                        theme.schedule(&dispatcher)?;
                    }
                    5 => {
                        assert_eq!(invalidated_reads.get(), 2);
                        assert_theme(&state.borrow(), ElementTheme::Dark);
                        theme
                            .reader
                            .replace(Some(Rc::new(|| theme_preference(5, 0))));
                        finished.set(true);
                        theme.schedule(&dispatcher)?;
                    }
                    _ => unreachable!(),
                }
                phase.set(phase.get() + 1);
                Ok(())
            })();
            result.unwrap();
        })?;
        timer.Start()?;
        Ok((host, default, timer, tick))
    })
    .unwrap_err();
    assert!(completed.get());
    assert_eq!(error.code(), windows_core::WIN32_ERROR(5).to_hresult());
}
