use super::*;
use bindings::{Grid, HorizontalAlignment, VerticalAlignment, *};

const HOST_SIZE: i32 = 2;

#[derive(Clone, Copy, PartialEq)]
pub(super) enum MenuTheme {
    Application,
    System,
}

pub(super) struct TransientMenuHost {
    state: Rc<RefCell<TransientMenuState>>,
    _loaded: windows_core::EventRevoker,
}

#[derive(Clone)]
pub(super) struct TransientMenuHandle {
    state: Weak<RefCell<TransientMenuState>>,
}

struct TransientMenuState {
    theme: Rc<MenuThemeState>,
    dispatcher: DispatcherQueue,
    _source: DesktopWindowXamlSource, // Keep this member above `host` to ensure proper drop order
    host: windows_window::Window,
    loaded: bool,
    menu: Option<ActiveMenu>,
    pending: Option<Menu>,
    generation: u64,
}

struct MenuThemeState {
    anchor: FrameworkElement,
    mode: MenuTheme,
    applied: Cell<ElementTheme>,
    refresh_pending: Cell<bool>,
    #[cfg(test)]
    reader: RefCell<Option<Rc<dyn Fn() -> windows_core::Result<ElementTheme>>>>,
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
    pub(super) fn new(dispatcher: DispatcherQueue, theme: MenuTheme) -> windows_core::Result<Self> {
        let _coordinates = PhysicalCoordinates::enter()?;
        let root = Grid::new()?;
        let anchor = Grid::new()?;
        let anchor_element = anchor.cast::<IFrameworkElement>()?;
        anchor_element.SetWidth(1.0)?;
        anchor_element.SetHeight(1.0)?;
        anchor_element.SetHorizontalAlignment(HorizontalAlignment::Left)?;
        anchor_element.SetVerticalAlignment(VerticalAlignment::Top)?;
        root.cast::<IPanel>()?.Children()?.Append(&anchor)?;
        let theme = Rc::new(MenuThemeState {
            anchor: anchor.cast()?,
            mode: theme,
            applied: Cell::new(ElementTheme::Default),
            refresh_pending: Cell::new(false),
            #[cfg(test)]
            reader: RefCell::new(None),
        });
        let message_theme = Rc::downgrade(&theme);
        let message_dispatcher = dispatcher.clone();
        let host = windows_window::Window::new("TransientMenuHost")
            .style(WS_POPUP)
            .ex_style(WS_EX_LAYERED as u32 | WS_EX_TOOLWINDOW as u32 | WS_EX_TOPMOST as u32)
            .size(HOST_SIZE, HOST_SIZE)
            .visible(true)
            .quit_on_close(false)
            .on_message(move |_, message, _, _| {
                if matches!(message as i32, WM_SETTINGCHANGE | WM_THEMECHANGED)
                    && let Some(theme) = message_theme.upgrade()
                    && let Err(error) = theme.schedule(&message_dispatcher)
                {
                    report_error(error);
                }
                None
            })
            .create()?;

        let hwnd = host.hwnd();
        unsafe {
            SetLayeredWindowAttributes(hwnd.cast(), 0, 0, LWA_ALPHA as u32).ok()?;
        }

        let source = DesktopWindowXamlSource::new()?;
        source.Initialize(WindowId { value: hwnd as u64 })?;
        source
            .cast::<IDesktopWindowXamlSource2>()?
            .SetShouldConstrainPopupsToWorkArea(false)?;
        source.SetContent(&root)?;
        source
            .SiteBridge()?
            .cast::<IDesktopChildSiteBridge>()?
            .SetResizePolicy(ContentSizePolicy::ResizeContentToParentWindow)?;

        let state = Rc::new(RefCell::new(TransientMenuState {
            theme,
            dispatcher,
            _source: source,
            host,
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

fn system_theme() -> windows_core::Result<ElementTheme> {
    let mut value = 0u32;
    let mut size = size_of::<u32>() as u32;
    let status = unsafe {
        RegGetValueW(
            HKEY_CURRENT_USER,
            windows_core::w!("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize"),
            windows_core::w!("SystemUsesLightTheme"),
            RRF_RT_REG_DWORD as u32,
            std::ptr::null_mut(),
            (&mut value as *mut u32).cast(),
            &mut size,
        )
    };
    theme_preference(status, value)
}

fn theme_preference(status: i32, value: u32) -> windows_core::Result<ElementTheme> {
    match status {
        0 => Ok(if value == 0 {
            ElementTheme::Dark
        } else {
            ElementTheme::Light
        }),
        ERROR_FILE_NOT_FOUND | ERROR_PATH_NOT_FOUND => Ok(ElementTheme::Default),
        _ => Err(windows_core::WIN32_ERROR(status as u32).into()),
    }
}

impl MenuThemeState {
    fn read(&self) -> windows_core::Result<ElementTheme> {
        #[cfg(test)]
        if let Some(reader) = self.reader.borrow().clone() {
            return reader();
        }
        system_theme()
    }

    fn refresh(&self) -> windows_core::Result<()> {
        if self.mode != MenuTheme::System {
            return Ok(());
        }
        let theme = self.read()?;
        // Record the in-flight theme before native code can reenter. Setter failures terminate
        // the application rather than retrying.
        if self.applied.replace(theme) != theme {
            self.anchor.SetRequestedTheme(theme)?;
        }
        Ok(())
    }

    fn schedule(self: &Rc<Self>, dispatcher: &DispatcherQueue) -> windows_core::Result<()> {
        if self.mode != MenuTheme::System || self.refresh_pending.replace(true) {
            return Ok(());
        }
        let target = Rc::downgrade(self);
        let refresh = DispatcherQueueHandler::new(move || {
            if let Some(theme) = target.upgrade() {
                // A notification during refresh must be able to enqueue the next refresh.
                theme.refresh_pending.set(false);
                if let Err(error) = theme.refresh() {
                    report_error(error);
                }
            }
        });
        match dispatcher.TryEnqueueWithPriority(DispatcherQueuePriority::Normal, &refresh) {
            Ok(true) => Ok(()),
            result => {
                self.refresh_pending.set(false);
                match result {
                    Ok(_) => Err(windows_core::Error::new(
                        E_FAIL,
                        "dispatcher rejected menu theme refresh",
                    )),
                    Err(error) => Err(error),
                }
            }
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
        let active = {
            let mut state = state.borrow_mut();
            state.generation = state.generation.wrapping_add(1);
            state.pending = None;
            state.menu.take()
        };
        if let Some(active) = active {
            let flyout = active.flyout.clone();
            drop(active);
            flyout.Hide()?;
        }
        Ok(())
    }

    pub(super) fn show(&self, position: ScreenPoint, menu: Menu) -> windows_core::Result<()> {
        let mut keys = HashSet::new();
        validate_menu_items(&menu.items, &mut keys)?;
        let state = self.state.upgrade().ok_or_else(|| {
            windows_core::Error::new(E_FAIL, "the application menu host is unavailable")
        })?;
        {
            let mut state = state.borrow_mut();
            if state.pending.is_some() || state.menu.is_some() {
                return Err(windows_core::Error::new(
                    E_FAIL,
                    "an application menu is already open",
                ));
            }
            position_host(&state.host, position)?;
            state.generation = state.generation.wrapping_add(1);
            state.pending = Some(menu);
        }
        show_pending(&state)
    }
}

fn position_host(host: &windows_window::Window, position: ScreenPoint) -> windows_core::Result<()> {
    let _coordinates = PhysicalCoordinates::enter()?;
    let hwnd: HWND = host.hwnd().cast();
    unsafe {
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            position.x,
            position.y,
            HOST_SIZE,
            HOST_SIZE,
            SWP_NOACTIVATE as u32,
        )
        .ok()
    }
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
    let (theme, dispatcher, host_hwnd, menu, generation) = {
        let mut state = state.borrow_mut();
        if !state.loaded || state.menu.is_some() {
            return Ok(());
        }
        let Some(menu) = state.pending.take() else {
            return Ok(());
        };
        (
            Rc::clone(&state.theme),
            state.dispatcher.clone(),
            state.host.hwnd().cast(),
            menu,
            state.generation,
        )
    };

    let result = (|| {
        theme.refresh()?;
        let anchor = &theme.anchor;
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
        flyout_base.ShowAt(anchor)
    })();
    if result.is_err() {
        let mut state = state.borrow_mut();
        state.menu = None;
    }
    result
}

#[cfg(test)]
#[path = "../tests/native/transient_menu.rs"]
mod tests;
