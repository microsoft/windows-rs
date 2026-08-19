use std::rc::Rc;

pub struct Callback<T> {
    inner: Rc<dyn Fn(T)>,
}

impl<T> Callback<T> {
    pub(crate) fn new(callback: impl Fn(T) + 'static) -> Self {
        Self {
            inner: Rc::new(callback),
        }
    }

    pub fn call(&self, value: T) {
        (self.inner)(value);
    }

    pub(crate) fn ptr_eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.inner, &other.inner)
    }
}

impl<T> Clone for Callback<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }
}

impl<T> PartialEq for Callback<T> {
    fn eq(&self, other: &Self) -> bool {
        self.ptr_eq(other)
    }
}

impl<T> Eq for Callback<T> {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VirtualKey(i32);

impl VirtualKey {
    pub const ENTER: Self = Self(13);
    pub const ESCAPE: Self = Self(27);
    pub const SPACE: Self = Self(32);
    pub const LEFT: Self = Self(37);
    pub const UP: Self = Self(38);
    pub const RIGHT: Self = Self(39);
    pub const DOWN: Self = Self(40);
    pub const DELETE: Self = Self(46);
    pub const A: Self = Self(65);
    pub const B: Self = Self(66);
    pub const C: Self = Self(67);
    pub const D: Self = Self(68);
    pub const E: Self = Self(69);
    pub const F: Self = Self(70);
    pub const G: Self = Self(71);
    pub const H: Self = Self(72);
    pub const I: Self = Self(73);
    pub const J: Self = Self(74);
    pub const K: Self = Self(75);
    pub const L: Self = Self(76);
    pub const M: Self = Self(77);
    pub const N: Self = Self(78);
    pub const O: Self = Self(79);
    pub const P: Self = Self(80);
    pub const Q: Self = Self(81);
    pub const R: Self = Self(82);
    pub const S: Self = Self(83);
    pub const T: Self = Self(84);
    pub const U: Self = Self(85);
    pub const V: Self = Self(86);
    pub const W: Self = Self(87);
    pub const X: Self = Self(88);
    pub const Y: Self = Self(89);
    pub const Z: Self = Self(90);
    pub const NUMBER_PAD_0: Self = Self(96);
    pub const NUMBER_PAD_1: Self = Self(97);
    pub const NUMBER_PAD_2: Self = Self(98);
    pub const NUMBER_PAD_3: Self = Self(99);
    pub const NUMBER_PAD_4: Self = Self(100);
    pub const NUMBER_PAD_5: Self = Self(101);
    pub const NUMBER_PAD_6: Self = Self(102);
    pub const NUMBER_PAD_7: Self = Self(103);
    pub const NUMBER_PAD_8: Self = Self(104);
    pub const NUMBER_PAD_9: Self = Self(105);
    pub const MULTIPLY: Self = Self(106);
    pub const ADD: Self = Self(107);
    pub const SUBTRACT: Self = Self(109);
    pub const DECIMAL: Self = Self(110);
    pub const DIVIDE: Self = Self(111);
    pub const F1: Self = Self(112);
    pub const F2: Self = Self(113);
    pub const F3: Self = Self(114);
    pub const F4: Self = Self(115);
    pub const F5: Self = Self(116);
    pub const F6: Self = Self(117);
    pub const F7: Self = Self(118);
    pub const F8: Self = Self(119);
    pub const F9: Self = Self(120);
    pub const F10: Self = Self(121);
    pub const F11: Self = Self(122);
    pub const F12: Self = Self(123);

    pub const fn from_code(code: i32) -> Self {
        Self(code)
    }

    pub const fn code(self) -> i32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct VirtualKeyModifiers(u32);

impl VirtualKeyModifiers {
    pub const NONE: Self = Self(0);
    pub const CONTROL: Self = Self(1);
    pub const MENU: Self = Self(2);
    pub const SHIFT: Self = Self(4);
    pub const WINDOWS: Self = Self(8);

    pub const fn bits(self) -> u32 {
        self.0
    }
}

impl std::ops::BitOr for VirtualKeyModifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for VirtualKeyModifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AutomationHeadingLevel {
    Level1 = 1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
}

#[derive(Clone)]
pub struct KeyboardAccelerator {
    key: VirtualKey,
    modifiers: VirtualKeyModifiers,
    on_invoked: Callback<()>,
}

impl KeyboardAccelerator {
    pub fn new(
        key: VirtualKey,
        modifiers: VirtualKeyModifiers,
        on_invoked: impl Fn() + 'static,
    ) -> Self {
        Self {
            key,
            modifiers,
            on_invoked: Callback::new(move |()| on_invoked()),
        }
    }

    pub const fn key(&self) -> VirtualKey {
        self.key
    }

    pub const fn modifiers(&self) -> VirtualKeyModifiers {
        self.modifiers
    }

    pub(crate) fn invoke(&self) {
        self.on_invoked.call(());
    }
}

impl PartialEq for KeyboardAccelerator {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
            && self.modifiers == other.modifiers
            && self.on_invoked.ptr_eq(&other.on_invoked)
    }
}

impl Eq for KeyboardAccelerator {}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointerEvent {
    pub pointer_id: u32,
    pub x: f32,
    pub y: f32,
    pub window_x: f32,
    pub window_y: f32,
    pub capture_succeeded: bool,
    pub is_left_button_pressed: bool,
    pub is_right_button_pressed: bool,
    pub is_middle_button_pressed: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DropOperation {
    Copy,
    Move,
    Link,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DropFormats(u8);

impl DropFormats {
    pub const TEXT: Self = Self(1 << 0);
    pub const STORAGE_ITEMS: Self = Self(1 << 1);

    pub const fn contains(self, value: Self) -> bool {
        self.0 & value.0 == value.0
    }

    pub const fn intersects(self, value: Self) -> bool {
        self.0 & value.0 != 0
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl std::ops::BitOr for DropFormats {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for DropFormats {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DropTarget {
    operation: DropOperation,
    formats: DropFormats,
}

impl DropTarget {
    pub fn new(operation: DropOperation, formats: DropFormats) -> Self {
        assert!(
            !formats.is_empty(),
            "drop target requires at least one format"
        );
        Self { operation, formats }
    }

    pub const fn operation(self) -> DropOperation {
        self.operation
    }

    pub const fn formats(self) -> DropFormats {
        self.formats
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DroppedItem {
    pub path: String,
    pub name: String,
    pub is_folder: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropEvent {
    pub formats: DropFormats,
    pub text: Option<String>,
    pub storage_items: Box<[DroppedItem]>,
}
