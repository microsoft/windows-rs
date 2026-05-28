//! Per-element pointer / tap event handlers carried on
//! [`Modifiers::pointer_handlers`](super::Modifiers::pointer_handlers)
//! and applied via [`Backend::set_pointer_handlers`](super::backend::Backend::set_pointer_handlers).
//! These mirror the universal `UIElement` pointer events (`Tapped`,
//! `RightTapped`, `PointerPressed`, `PointerReleased`, `PointerExited`).

use super::callback::Callback;

/// Bundle of per-element pointer / tap callbacks; each slot is
/// individually optional.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct PointerHandlers {
    pub on_tapped: Option<Callback<()>>,
    pub on_right_tapped: Option<Callback<()>>,
    pub on_pointer_pressed: Option<Callback<PointerEventInfo>>,
    pub on_pointer_released: Option<Callback<PointerEventInfo>>,
    pub on_pointer_exited: Option<Callback<()>>,
}

impl PointerHandlers {
    pub fn is_empty(&self) -> bool {
        self.on_tapped.is_none()
            && self.on_right_tapped.is_none()
            && self.on_pointer_pressed.is_none()
            && self.on_pointer_released.is_none()
            && self.on_pointer_exited.is_none()
    }
}

/// Button state captured at a `PointerPressed` / `PointerReleased`
/// callback. Non-mouse pointer kinds report all three as `false`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PointerEventInfo {
    pub is_left_button_pressed: bool,
    pub is_right_button_pressed: bool,
    pub is_middle_button_pressed: bool,
}
