use super::app::{ScreenPoint, report_error};
use super::bindings::*;
use crate::{Key, Menu, MenuItem};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use windows_core::Interface;

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
}

impl TransientMenuHost {
    pub(super) fn new(dispatcher: DispatcherQueue) -> windows_core::Result<Self> {
        let window = Window::new()?;
        let root = Grid::new()?;
        let anchor = Grid::new()?;
        let anchor_element = anchor.cast::<IFrameworkElement>()?;
        anchor_element.SetWidth(1.0)?;
        anchor_element.SetHeight(1.0)?;
        anchor_element.SetHorizontalAlignment(HorizontalAlignment::Left)?;
        anchor_element.SetVerticalAlignment(VerticalAlignment::Top)?;
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
    pub(super) fn show(&self, position: ScreenPoint, menu: Menu) -> windows_core::Result<()> {
        let mut keys = HashSet::new();
        validate_menu_items(&menu.items, &mut keys)?;
        let state = self.state.upgrade().ok_or_else(|| {
            windows_core::Error::new(E_FAIL, "the application menu host is unavailable")
        })?;
        let window = {
            let mut state = state.borrow_mut();
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
            let mut state = state.borrow_mut();
            state.pending = None;
            _ = state.app_window.Hide();
            return Err(error);
        }
        show_pending(&state)
    }
}

fn validate_menu_items(items: &[MenuItem], keys: &mut HashSet<Key>) -> windows_core::Result<()> {
    for item in items {
        let (key, children) = match item {
            MenuItem::Item { key, .. } | MenuItem::Separator { key } => (key, None),
            MenuItem::Submenu { key, items, .. } => (key, Some(items.as_slice())),
        };
        if !keys.insert(key.clone()) {
            return Err(windows_core::Error::new(
                windows_core::HRESULT(0x80070057u32 as i32),
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
            drop(active._revokers);
            _ = active.flyout.Hide();
        }
        state.pending = None;
        _ = state.window.Close();
    }
}

fn build_menu_items(
    items: &[MenuItem],
    output: &windows_collections::IVector<MenuFlyoutItemBase>,
    revokers: &mut Vec<windows_core::EventRevoker>,
    callback: &crate::Callback<Key>,
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

        let mut revokers = Vec::new();
        build_menu_items(&menu.items, &flyout.Items()?, &mut revokers, &menu.on_click)?;

        let closed_state = Rc::downgrade(state);
        revokers.push(flyout_base.Closed(move |_, _| {
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
        });
        flyout_base.ShowAt(&anchor)
    })();
    if result.is_err() {
        state.borrow_mut().menu = None;
        _ = app_window.Hide();
    }
    result
}

#[cfg(test)]
mod tests {
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
}
