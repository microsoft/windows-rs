use super::*;
use bindings::{Grid, HorizontalAlignment, VerticalAlignment, *};

const HOST_SIZE: i32 = 2;

pub(super) struct TransientMenuHost {
    state: Rc<RefCell<TransientMenuState>>,
    _loaded: windows_core::EventRevoker,
}

#[derive(Clone)]
pub(super) struct TransientMenuHandle {
    state: Weak<RefCell<TransientMenuState>>,
}

struct TransientMenuState {
    anchor: FrameworkElement,
    dispatcher: DispatcherQueue,
    host: Window,
    host_app_window: AppWindow,
    host_hwnd: HWND,
    loaded: bool,
    menu: Option<ActiveMenu>,
    pending: Option<Menu>,
    generation: u64,
}

struct ActiveMenu {
    flyout: IFlyoutBase,
    _revokers: Vec<windows_core::EventRevoker>,
    live: Rc<Cell<bool>>,
}

impl Drop for ActiveMenu {
    fn drop(&mut self) {
        self.live.set(false);
    }
}

impl TransientMenuHost {
    pub(super) fn new(dispatcher: DispatcherQueue) -> windows_core::Result<Self> {
        let host = Window::new()?;
        let root = Grid::new()?;
        let anchor = Grid::new()?;
        let anchor_element = anchor.cast::<IFrameworkElement>()?;
        anchor_element.SetWidth(1.0)?;
        anchor_element.SetHeight(1.0)?;
        anchor_element.SetHorizontalAlignment(HorizontalAlignment::Left)?;
        anchor_element.SetVerticalAlignment(VerticalAlignment::Top)?;
        root.cast::<IPanel>()?.Children()?.Append(&anchor)?;
        host.SetContent(&root)?;

        let host_app_window = host.cast::<IWindow2>()?.AppWindow()?;
        host_app_window.SetIsShownInSwitchers(false)?;
        let presenter = host_app_window
            .Presenter()?
            .cast::<IOverlappedPresenter>()?;
        presenter.SetBorderAndTitleBar(false, false)?;
        presenter.SetIsAlwaysOnTop(true)?;

        let mut raw_hwnd = std::ptr::null_mut();
        let host_hwnd;
        unsafe {
            host.cast::<IWindowNative>()?
                .WindowHandle(&mut raw_hwnd)
                .ok()?;
            host_hwnd = raw_hwnd.cast();
            SetLastError(0);
            let style = GetWindowLongW(host_hwnd, GWL_EXSTYLE);
            let error = GetLastError();
            if style == 0 && error != 0 {
                return Err(HRESULT::from(windows_core::WIN32_ERROR(error)).into());
            }
            SetLastError(0);
            let previous = SetWindowLongW(host_hwnd, GWL_EXSTYLE, style | WS_EX_LAYERED);
            let error = GetLastError();
            if previous == 0 && error != 0 {
                return Err(HRESULT::from(windows_core::WIN32_ERROR(error)).into());
            }
            SetLayeredWindowAttributes(host_hwnd, 0, 0, LWA_ALPHA as u32).ok()?;
        }

        let state = Rc::new(RefCell::new(TransientMenuState {
            anchor: anchor.cast()?,
            dispatcher,
            host,
            host_app_window,
            host_hwnd,
            loaded: false,
            menu: None,
            pending: None,
            generation: 0,
        }));
        let loaded_state = Rc::downgrade(&state);
        let loaded = anchor_element.Loaded(move |_, _| {
            let Some(state) = loaded_state.upgrade() else {
                return;
            };
            state.borrow_mut().loaded = true;
            if let Err(error) = show_pending(&state) {
                report_error(error);
            }
        })?;

        Ok(Self {
            state,
            _loaded: loaded,
        })
    }

    pub(super) fn handle(&self) -> TransientMenuHandle {
        TransientMenuHandle {
            state: Rc::downgrade(&self.state),
        }
    }
}

impl TransientMenuHandle {
    pub(super) fn is_open(&self) -> bool {
        self.state.upgrade().is_some_and(|state| {
            let state = state.borrow();
            state.menu.is_some() || state.pending.is_some()
        })
    }

    pub(super) fn hide(&self) -> windows_core::Result<()> {
        let Some(state) = self.state.upgrade() else {
            return Ok(());
        };
        let (active, host) = {
            let mut state = state.borrow_mut();
            state.generation = state.generation.wrapping_add(1);
            state.pending = None;
            (state.menu.take(), state.host_app_window.clone())
        };
        if let Some(active) = active {
            let flyout = active.flyout.clone();
            drop(active);
            flyout.Hide()?;
        }
        host.Hide()
    }

    pub(super) fn show(&self, position: ScreenPoint, menu: Menu) -> windows_core::Result<()> {
        let mut keys = HashSet::new();
        validate_menu_items(&menu.items, &mut keys)?;
        let state = self.state.upgrade().ok_or_else(|| {
            windows_core::Error::new(E_FAIL, "the application menu host is unavailable")
        })?;
        let host = {
            let mut state = state.borrow_mut();
            if state.pending.is_some() || state.menu.is_some() {
                return Err(windows_core::Error::new(
                    E_FAIL,
                    "an application menu is already open",
                ));
            }
            position_host_anchor(&state.host_app_window, state.host_hwnd, position)?;
            state.generation = state.generation.wrapping_add(1);
            state.pending = Some(menu);
            state.host.clone()
        };

        if let Err(error) = host.Activate() {
            let mut state = state.borrow_mut();
            state.pending = None;
            _ = state.host_app_window.Hide();
            return Err(error);
        }
        show_pending(&state)
    }
}

fn position_host_anchor(
    host: &AppWindow,
    host_hwnd: HWND,
    position: ScreenPoint,
) -> windows_core::Result<()> {
    host.MoveAndResize(RectInt32 {
        x: position.x,
        y: position.y,
        width: HOST_SIZE,
        height: HOST_SIZE,
    })?;

    // MoveAndResize positions outer bounds. Measure and compensate for the client offset.
    let mut client_origin = POINT::default();
    unsafe {
        ClientToScreen(host_hwnd, &mut client_origin).ok()?;
    }
    host.MoveAndResize(RectInt32 {
        x: host_coordinate(position.x, client_origin.x)?,
        y: host_coordinate(position.y, client_origin.y)?,
        width: HOST_SIZE,
        height: HOST_SIZE,
    })
}

fn host_coordinate(requested: i32, client_origin: i32) -> windows_core::Result<i32> {
    let requested = i64::from(requested);
    let client_offset = i64::from(client_origin) - requested;
    (requested - client_offset)
        .try_into()
        .map_err(|_| windows_core::Error::new(E_FAIL, "application menu position is out of range"))
}

pub(super) fn validate_menu_items(
    items: &[MenuItem],
    keys: &mut HashSet<Key>,
) -> windows_core::Result<()> {
    for item in items {
        let (key, children) = match item {
            MenuItem::Item { key, .. } | MenuItem::Separator { key } => (key, None),
            MenuItem::Submenu { key, items, .. } => (key, Some(items.as_slice())),
        };
        if !keys.insert(key.clone()) {
            return Err(windows_core::Error::new(
                HRESULT(0x80070057u32 as i32),
                "application menu keys must be unique",
            ));
        }
        if let Some(children) = children {
            validate_menu_items(children, keys)?;
        }
    }
    Ok(())
}

impl Drop for TransientMenuHost {
    fn drop(&mut self) {
        let mut state = self.state.borrow_mut();
        if let Some(active) = state.menu.take() {
            let flyout = active.flyout.clone();
            drop(active);
            _ = flyout.Hide();
        }
        state.pending = None;
        _ = state.host.Close();
    }
}

fn build_menu_items(
    items: &[MenuItem],
    output: &IVector<MenuFlyoutItemBase>,
    revokers: &mut Vec<windows_core::EventRevoker>,
    callback: &Callback<Key>,
) -> windows_core::Result<()> {
    for entry in items {
        let item: MenuFlyoutItemBase = match entry {
            MenuItem::Item {
                key,
                label,
                enabled,
            } => {
                let item = MenuFlyoutItem::new()?;
                item.SetText(label)?;
                item.cast::<IControl>()?.SetIsEnabled(*enabled)?;
                let callback = callback.clone();
                let key = key.clone();
                revokers.push(item.Click(move |_, _| callback.call(key.clone()))?);
                item.cast()?
            }
            MenuItem::Separator { .. } => MenuFlyoutSeparator::new()?.cast()?,
            MenuItem::Submenu { label, items, .. } => {
                let item = MenuFlyoutSubItem::new()?;
                item.SetText(label)?;
                build_menu_items(items, &item.Items()?, revokers, callback)?;
                item.cast()?
            }
        };
        output.Append(&item)?;
    }
    Ok(())
}

fn menu_callback(callback: Callback<Key>, live: Rc<Cell<bool>>) -> Callback<Key> {
    Callback::new(move |key| {
        if live.get() {
            callback.call(key);
        }
    })
}

fn show_pending(state: &Rc<RefCell<TransientMenuState>>) -> windows_core::Result<()> {
    let (anchor, host_app_window, dispatcher, host_hwnd, menu, generation) = {
        let mut state = state.borrow_mut();
        if !state.loaded || state.menu.is_some() {
            return Ok(());
        }
        let Some(menu) = state.pending.take() else {
            return Ok(());
        };
        (
            state.anchor.clone(),
            state.host_app_window.clone(),
            state.dispatcher.clone(),
            state.host_hwnd,
            menu,
            state.generation,
        )
    };

    let result = (|| {
        // Windows may deny foreground activation under its focus-stealing policy. The flyout can
        // still open, and notification-area callbacks normally carry foreground permission.
        _ = unsafe { SetForegroundWindow(host_hwnd) };

        let flyout = MenuFlyout::new()?;
        let flyout_base = flyout.cast::<IFlyoutBase>()?;
        flyout_base.SetPlacement(FlyoutPlacementMode::BottomEdgeAlignedLeft)?;
        flyout_base.SetShouldConstrainToRootBounds(false)?;
        flyout_base.SetXamlRoot(&anchor.cast::<IUIElement>()?.XamlRoot()?)?;

        let live = Rc::new(Cell::new(true));
        let callback = menu_callback(menu.on_click, Rc::clone(&live));
        let mut revokers = Vec::new();
        build_menu_items(&menu.items, &flyout.Items()?, &mut revokers, &callback)?;

        let closed_state = Rc::downgrade(state);
        let closed_live = Rc::clone(&live);
        revokers.push(flyout_base.Closed(move |_, _| {
            closed_live.set(false);
            let Some(state) = closed_state.upgrade() else {
                return;
            };
            let cleanup_state = Rc::downgrade(&state);
            let cleanup = DispatcherQueueHandler::new(move || {
                let Some(state) = cleanup_state.upgrade() else {
                    return;
                };
                let mut state = state.borrow_mut();
                if state.generation != generation {
                    return;
                }
                state.menu = None;
                if let Err(error) = state.host_app_window.Hide() {
                    drop(state);
                    report_error(error);
                }
            });
            match dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &cleanup) {
                Ok(true) => {}
                Ok(false) => report_error(windows_core::Error::new(
                    E_FAIL,
                    "dispatcher rejected application menu cleanup",
                )),
                Err(error) => report_error(error),
            }
        })?);

        state.borrow_mut().menu = Some(ActiveMenu {
            flyout: flyout_base.clone(),
            _revokers: revokers,
            live,
        });
        flyout_base.ShowAt(&anchor)
    })();
    if result.is_err() {
        state.borrow_mut().menu = None;
        _ = host_app_window.Hide();
    }
    result
}

#[cfg(test)]
#[path = "../tests/native/transient_menu.rs"]
mod tests;
