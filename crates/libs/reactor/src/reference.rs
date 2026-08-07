use crate::{RenderCx, TextBoxHandle};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use windows_core::IInspectable;

pub(crate) mod sealed {
    use windows_core::IInspectable;

    pub trait ElementHandle: Clone + 'static {
        fn from_native(native: IInspectable) -> Self;
    }
}

/// A typed reference to a mounted native element.
///
/// The reference is populated after mount and cleared before the native element is destroyed.
/// Clone the reference freely; every clone observes the same mounted target.
pub struct ElementRef<T: ElementHandle> {
    target: Rc<Target<T>>,
}

impl<T: ElementHandle> ElementRef<T> {
    pub fn new() -> Self {
        Self {
            target: Rc::new(Target {
                current: RefCell::new(None),
            }),
        }
    }

    /// Returns the mounted handle, or `None` before mount and after unmount.
    pub fn current(&self) -> Option<T> {
        self.target.current.borrow().clone()
    }

    pub fn is_mounted(&self) -> bool {
        self.target.current.borrow().is_some()
    }

    pub(crate) fn binding(&self) -> NativeElementRef {
        NativeElementRef(self.target.clone())
    }
}

impl<T: ElementHandle> Clone for ElementRef<T> {
    fn clone(&self) -> Self {
        Self {
            target: self.target.clone(),
        }
    }
}

impl<T: ElementHandle> Default for ElementRef<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: ElementHandle> fmt::Debug for ElementRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ElementRef")
            .field("mounted", &self.is_mounted())
            .finish()
    }
}

impl<T: ElementHandle> PartialEq for ElementRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.target, &other.target)
    }
}

impl<T: ElementHandle> Eq for ElementRef<T> {}

/// A native handle type supported by [`ElementRef`].
///
/// This trait is sealed. Reactor defines the handle type for each reference-capable widget.
pub trait ElementHandle: sealed::ElementHandle {}

impl<T: sealed::ElementHandle> ElementHandle for T {}

trait NativeElementRefTarget {
    fn set_native(&self, native: Option<IInspectable>);
}

struct Target<T: ElementHandle> {
    current: RefCell<Option<T>>,
}

impl<T: ElementHandle> NativeElementRefTarget for Target<T> {
    fn set_native(&self, native: Option<IInspectable>) {
        *self.current.borrow_mut() = native.map(T::from_native);
    }
}

#[derive(Clone)]
pub(crate) struct NativeElementRef(Rc<dyn NativeElementRefTarget>);

impl NativeElementRef {
    pub(crate) fn set_native(&self, native: Option<IInspectable>) {
        self.0.set_native(native);
    }
}

impl fmt::Debug for NativeElementRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("NativeElementRef")
    }
}

impl PartialEq for NativeElementRef {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}

impl RenderCx {
    /// Returns an identity-stable typed element reference for this hook slot.
    pub fn use_element_ref<T: ElementHandle>(&mut self) -> ElementRef<T> {
        self.use_memo((), ElementRef::new)
    }
}

impl ElementRef<TextBoxHandle> {
    /// Requests programmatic focus.
    ///
    /// Returns `Ok(false)` when the TextBox is not mounted or WinUI rejects the request.
    pub fn focus(&self) -> windows_core::Result<bool> {
        match self.current() {
            Some(handle) => handle.focus(),
            None => Ok(false),
        }
    }
}
