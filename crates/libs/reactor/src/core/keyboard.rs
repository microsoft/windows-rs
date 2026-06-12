use super::*;

/// A single keyboard shortcut bound to an element via
/// [`Modifiers`](crate::Modifiers)`.keyboard_accelerators`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyboardAccelerator {
    pub key: KeyboardKey,
    pub modifiers: KeyModifiers,
    pub on_invoked: Callback<()>,
}

impl KeyboardAccelerator {
    pub fn new<F: Fn() + 'static>(
        key: KeyboardKey,
        modifiers: KeyModifiers,
        on_invoked: F,
    ) -> Self {
        Self {
            key,
            modifiers,
            on_invoked: Callback::new(move |()| on_invoked()),
        }
    }
}

/// Bit-flags subset of WinRT `VirtualKeyModifiers` (Alt is named `MENU`
/// upstream; this crate keeps the conventional `ALT` name).
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct KeyModifiers(pub u32);

impl KeyModifiers {
    pub const NONE: Self = Self(0);
    pub const CONTROL: Self = Self(1);
    pub const ALT: Self = Self(2);
    pub const SHIFT: Self = Self(4);
    pub const WINDOWS: Self = Self(8);

    pub const fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl core::ops::BitOr for KeyModifiers {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl core::ops::BitOrAssign for KeyModifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum KeyboardKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    NumPad0,
    NumPad1,
    NumPad2,
    NumPad3,
    NumPad4,
    NumPad5,
    NumPad6,
    NumPad7,
    NumPad8,
    NumPad9,
    NumPadAdd,
    NumPadSubtract,
    NumPadMultiply,
    NumPadDivide,
    NumPadDecimal,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Enter,
    Escape,
    Tab,
    Space,
    Backspace,
    Delete,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    Left,
    Right,
    Up,
    Down,
}
