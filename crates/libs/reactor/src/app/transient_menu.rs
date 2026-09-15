use super::*;

const HOST_SIZE: i32 = 2;

pub(super) struct TransientMenuHost {
    state: Rc<RefCell<TransientMenuState>>,
    _loaded: windows_core::EventRevoker,
}

#[derive(Clone)]
pub(super) struct TransientMenuHandle {
    state: Rc<RefCell<TransientMenuState>>,
}

struct TransientMenuState {
    anchor: FrameworkElement,
    app_window: AppWindow,
    dispatcher: DispatcherQueue,
    hwnd: HWND,
    loaded: bool,
    menu: Option<ActiveMenu>,
    pending: Option<Menu>,
    window: Window,
}

struct ActiveMenu {
    flyout: IFlyoutBase,
    _revokers: Vec<windows_core::EventRevoker>,
    #[cfg(feature = "test")]
    first_item: Option<MenuFlyoutItem>,
    #[cfg(feature = "test")]
    opened: bool,
}

impl TransientMenuHost {
    pub(super) fn new(dispatcher: DispatcherQueue) -> windows_core::Result<Self> {
        let window = Window::new()?;
        let root = NativeGrid::new()?;
        let anchor = NativeGrid::new()?;
        let anchor_element = anchor.cast::<IFrameworkElement>()?;
        anchor_element.SetWidth(1.0)?;
        anchor_element.SetHeight(1.0)?;
        anchor_element.SetHorizontalAlignment(NativeHorizontalAlignment::Left)?;
        anchor_element.SetVerticalAlignment(NativeVerticalAlignment::Top)?;
        root.cast::<IPanel>()?.Children()?.Append(&anchor)?;
        window.SetContent(&root)?;

        let app_window = window.cast::<IWindow2>()?.AppWindow()?;
        app_window.SetIsShownInSwitchers(false)?;
        let presenter = app_window.Presenter()?.cast::<IOverlappedPresenter>()?;
        presenter.SetBorderAndTitleBar(false, false)?;
        presenter.SetIsAlwaysOnTop(true)?;

        let mut raw_hwnd = std::ptr::null_mut();
        let hwnd;
        unsafe {
            window
                .cast::<IWindowNative>()?
                .WindowHandle(&mut raw_hwnd)
                .ok()?;
            hwnd = raw_hwnd.cast();
            SetLastError(0);
            let style = GetWindowLongW(hwnd, GWL_EXSTYLE);
            let error = GetLastError();
            if style == 0 && error != 0 {
                return Err(windows_core::HRESULT::from(windows_core::WIN32_ERROR(error)).into());
            }
            SetLastError(0);
            let previous = SetWindowLongW(hwnd, GWL_EXSTYLE, style | WS_EX_LAYERED);
            let error = GetLastError();
            if previous == 0 && error != 0 {
                return Err(windows_core::HRESULT::from(windows_core::WIN32_ERROR(error)).into());
            }
            SetLayeredWindowAttributes(hwnd, 0, 0, LWA_ALPHA as u32).ok()?;
        }

        let state = Rc::new(RefCell::new(TransientMenuState {
            anchor: anchor.cast()?,
            app_window,
            dispatcher,
            hwnd,
            loaded: false,
            menu: None,
            pending: None,
            window,
        }));
        let loaded_state = Rc::downgrade(&state);
        let loaded = anchor_element.Loaded(move |_, _| {
            let Some(state) = loaded_state.upgrade() else {
                return;
            };
            state.borrow_mut().loaded = true;
            if let Err(error) = show_pending(&state) {
                report_fault(error);
            }
        })?;

        Ok(Self {
            state,
            _loaded: loaded,
        })
    }

    pub(super) fn handle(&self) -> TransientMenuHandle {
        TransientMenuHandle {
            state: Rc::clone(&self.state),
        }
    }

    #[cfg(feature = "test")]
    pub(super) fn item_for_test(&self) -> windows_core::Result<MenuFlyoutItem> {
        let state = self.state.borrow();
        let menu = state
            .menu
            .as_ref()
            .ok_or_else(|| windows_core::Error::new(E_FAIL, "live application menu is not open"))?;
        if !menu.opened {
            return Err(windows_core::Error::new(
                E_FAIL,
                "live application menu is not ready",
            ));
        }
        menu.first_item.clone().ok_or_else(|| {
            windows_core::Error::new(E_FAIL, "live application menu has no command item")
        })
    }

    #[cfg(feature = "test")]
    pub(super) fn is_open_for_test(&self) -> bool {
        self.state
            .borrow()
            .menu
            .as_ref()
            .is_some_and(|menu| menu.opened)
    }
}

impl TransientMenuHandle {
    pub(super) fn show(&self, position: ScreenPoint, menu: Menu) -> windows_core::Result<()> {
        let window = {
            let mut state = self.state.borrow_mut();
            if state.pending.is_some() || state.menu.is_some() {
                return Err(windows_core::Error::new(
                    E_FAIL,
                    "an application menu is already open",
                ));
            }
            state.app_window.MoveAndResize(RectInt32 {
                x: position.x,
                y: position.y,
                width: HOST_SIZE,
                height: HOST_SIZE,
            })?;
            state.pending = Some(menu);
            state.window.clone()
        };

        if let Err(error) = window.Activate() {
            let mut state = self.state.borrow_mut();
            state.pending = None;
            _ = state.app_window.Hide();
            return Err(error);
        }
        show_pending(&self.state)
    }
}

impl Drop for TransientMenuHost {
    fn drop(&mut self) {
        let mut state = self.state.borrow_mut();
        if let Some(active) = state.menu.take() {
            let ActiveMenu {
                flyout,
                _revokers: revokers,
                ..
            } = active;
            drop(revokers);
            _ = flyout.Hide();
        }
        state.pending = None;
        _ = state.window.Close();
    }
}

fn show_pending(state: &Rc<RefCell<TransientMenuState>>) -> windows_core::Result<()> {
    let (anchor, app_window, dispatcher, hwnd, menu) = {
        let mut state = state.borrow_mut();
        if !state.loaded || state.menu.is_some() {
            return Ok(());
        }
        let Some(menu) = state.pending.take() else {
            return Ok(());
        };
        (
            state.anchor.clone(),
            state.app_window.clone(),
            state.dispatcher.clone(),
            state.hwnd,
            menu,
        )
    };

    let result = (|| {
        // Windows may deny foreground activation under its focus-stealing policy. The flyout can
        // still open, and notification-area callbacks normally carry foreground permission.
        _ = unsafe { SetForegroundWindow(hwnd) };

        let flyout = MenuFlyout::new()?;
        let flyout_base = flyout.cast::<IFlyoutBase>()?;
        flyout_base.SetPlacement(FlyoutPlacementMode::BottomEdgeAlignedLeft)?;
        flyout_base.SetShouldConstrainToRootBounds(false)?;
        flyout_base.SetXamlRoot(&anchor.cast::<IUIElement>()?.XamlRoot()?)?;

        let callback = menu.on_click;
        let invoke: Rc<dyn Fn(String)> = Rc::new(move |label| {
            _ = callback.call(label);
        });
        let mut revokers = Vec::new();
        build_native_menu_items(&menu.items, &flyout.Items()?, &mut revokers, &invoke)?;

        #[cfg(feature = "test")]
        let first_item = flyout
            .Items()?
            .GetAt(0)
            .ok()
            .and_then(|item| item.cast::<MenuFlyoutItem>().ok());
        #[cfg(feature = "test")]
        let opened_state = Rc::downgrade(state);
        #[cfg(feature = "test")]
        revokers.push(flyout_base.Opened(move |_, _| {
            if let Some(state) = opened_state.upgrade()
                && let Some(menu) = state.borrow_mut().menu.as_mut()
            {
                menu.opened = true;
            }
        })?);

        let closed_state = Rc::downgrade(state);
        let closed = flyout_base.Closed(move |_, _| {
            let Some(state) = closed_state.upgrade() else {
                return;
            };
            let cleanup_state = Rc::downgrade(&state);
            let cleanup = DispatcherQueueHandler::new(move || {
                let Some(state) = cleanup_state.upgrade() else {
                    return;
                };
                let mut state = state.borrow_mut();
                state.menu = None;
                if let Err(error) = state.app_window.Hide() {
                    drop(state);
                    report_fault(error);
                }
            });
            match dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &cleanup) {
                Ok(true) => {}
                Ok(false) => report_fault(windows_core::Error::new(
                    E_FAIL,
                    "dispatcher rejected application menu cleanup",
                )),
                Err(error) => report_fault(error),
            }
        });
        revokers.push(closed?);

        state.borrow_mut().menu = Some(ActiveMenu {
            flyout: flyout_base.clone(),
            _revokers: revokers,
            #[cfg(feature = "test")]
            first_item,
            #[cfg(feature = "test")]
            opened: false,
        });
        flyout_base.ShowAt(&anchor)
    })();
    if result.is_err() {
        state.borrow_mut().menu = None;
        _ = app_window.Hide();
    }
    result
}

fn report_fault(error: windows_core::Error) {
    eprintln!("windows-reactor application menu fault: {error}");
    HOST.with(|host| {
        if let Some(host) = host.borrow_mut().as_mut() {
            host.fault = Some(error);
        }
    });
    exit_ui_thread();
}
