use super::*;
use core::any::Any;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::{MaybeUninit, forget, transmute_copy};
use core::ptr::NonNull;

/// Debug-only diagnostic helper: reports a `cast` whose `QueryInterface` returned the
/// same interface pointer it started from - i.e. the source already exposes the target
/// interface, so the cast is redundant (use `Deref` or `.into()` instead). Compiled in
/// only in debug builds when `RUSTFLAGS=--cfg windows_cast_diagnostics` is set.
#[cfg(all(debug_assertions, windows_cast_diagnostics))]
#[cold]
#[inline(never)]
fn warn_redundant_cast<T: Interface>(location: &core::panic::Location<'_>) {
    extern crate std;

    std::eprintln!(
        "windows-core: cast::<{}> at {} returned the same interface pointer; the source \
         already exposes this interface (use Deref or .into() instead of .cast())",
        core::any::type_name::<T>(),
        location,
    );
}

/// Provides low-level access to a COM interface vtable.
///
/// # Safety
pub unsafe trait Interface: Sized + Clone {
    #[doc(hidden)]
    type Vtable;

    /// The `GUID` associated with the interface.
    const IID: GUID;

    #[doc(hidden)]
    const UNKNOWN: bool = true;

    /// Returns the interface vtable.
    #[doc(hidden)]
    #[inline(always)]
    fn vtable(&self) -> &Self::Vtable {
        // SAFETY: the implementor of the trait guarantees that `Self` is castable to its vtable
        unsafe { self.assume_vtable::<Self>() }
    }

    /// Views this interface through a compatible vtable.
    ///
    /// # Safety
    ///
    /// This is safe if `T` is an equivalent interface to `Self` or a super interface.
    /// In other words, `T::Vtable` must be equivalent to the beginning of `Self::Vtable`.
    #[doc(hidden)]
    #[inline(always)]
    unsafe fn assume_vtable<T: Interface>(&self) -> &T::Vtable {
        unsafe { &**(self.as_raw() as *mut *mut T::Vtable) }
    }

    /// Returns the borrowed COM interface pointer.
    #[inline(always)]
    fn as_raw(&self) -> *mut c_void {
        // SAFETY: implementors guarantee a pointer representation.
        unsafe { transmute_copy(self) }
    }

    /// Transfers ownership of the COM interface pointer to the caller.
    #[inline(always)]
    fn into_raw(self) -> *mut c_void {
        // SAFETY: implementors guarantee a pointer representation.
        let raw = self.as_raw();
        forget(self);
        raw
    }

    /// Takes ownership of a COM interface pointer.
    ///
    /// # Safety
    ///
    /// `raw` must be owned and point to the vtable for `Self`, beginning with `IUnknown`.
    unsafe fn from_raw(raw: *mut c_void) -> Self {
        unsafe { transmute_copy(&raw) }
    }

    /// Borrows a COM interface pointer.
    ///
    /// # Safety
    ///
    /// `raw` must remain valid and point to the vtable for `Self`, beginning with `IUnknown`.
    #[inline(always)]
    unsafe fn from_raw_borrowed(raw: &*mut c_void) -> Option<&Self> {
        unsafe {
            if raw.is_null() {
                None
            } else {
                Some(transmute_copy(&raw))
            }
        }
    }

    /// Queries the object for another interface.
    #[cfg_attr(all(debug_assertions, windows_cast_diagnostics), track_caller)]
    #[inline(always)]
    fn cast<T: Interface>(&self) -> Result<T> {
        // SAFETY: `result` is valid for writing an interface pointer, and casting the result
        // pointer as `T` on success is safe because we use the `IID` tied to `T`, which the
        // implementor of `Interface` has guaranteed is correct.
        unsafe {
            // If `query()` fails, propagate the failure to the caller and ignore the contents
            // of `result` (which will _not_ be dropped, because `MaybeUninit` intentionally
            // does not drop its contents). This guards against COM implementations that store
            // non-null values in `result` but still return `E_NOINTERFACE`.
            let mut result = MaybeUninit::<Option<T>>::zeroed();
            self.query(&T::IID, result.as_mut_ptr() as _).ok()?;

            // `query()` succeeded; still double-check that the output pointer is non-null.
            if let Some(obj) = result.assume_init() {
                #[cfg(all(debug_assertions, windows_cast_diagnostics))]
                if core::ptr::eq(self.as_raw(), obj.as_raw()) {
                    warn_redundant_cast::<T>(core::panic::Location::caller());
                }
                Ok(obj)
            } else {
                Err(imp::E_POINTER.into())
            }
        }
    }

    /// Returns the generated outer Rust implementation as [`&dyn Any`].
    ///
    /// Applications should use
    /// [`Interface::cast_object_ref`] or [`Interface::cast_object`] instead.
    ///
    /// The reference points to the generated outer type, such as `MyApp_Impl`, not `MyApp`.
    ///
    /// Returns `Err(E_NOINTERFACE)` if the object is not a Rust object, not `T`, or contains
    /// non-static lifetimes.
    ///
    /// # Safety
    ///
    /// This uses a private `QueryInterface` protocol identified by `DYNAMIC_CAST_IID`. The
    /// implementation writes a two-pointer `&dyn Any`, not a COM interface pointer, to the output.
    ///
    /// The protocol does not call `AddRef`. The returned reference must not outlive `self`, and
    /// only `#[implement]`-generated objects may recognize this IID.
    #[inline(always)]
    fn cast_to_any<T>(&self) -> Result<&dyn Any>
    where
        T: ComObjectInner,
        T::Outer: Any + 'static + IUnknownImpl<Impl = T>,
    {
        unsafe {
            let mut any_ref_arg: MaybeUninit<&dyn Any> = MaybeUninit::zeroed();
            self.query(
                &DYNAMIC_CAST_IID,
                any_ref_arg.as_mut_ptr() as *mut *mut c_void,
            )
            .ok()?;
            Ok(any_ref_arg.assume_init())
        }
    }

    /// Returns `true` if the given COM interface refers to an implementation of `T`.
    ///
    /// Returns `false` if the object is not a Rust object, not `T`, or contains non-static
    /// lifetimes.
    #[inline(always)]
    fn is_object<T>(&self) -> bool
    where
        T: ComObjectInner,
        T::Outer: Any + 'static + IUnknownImpl<Impl = T>,
    {
        if let Ok(any) = self.cast_to_any::<T>() {
            any.is::<T::Outer>()
        } else {
            false
        }
    }

    /// Returns a borrowed reference to the generated outer Rust implementation.
    ///
    /// Returns `Err(E_NOINTERFACE)` if the object is not a Rust object, not `T`, or contains
    /// non-static lifetimes.
    ///
    /// The returned value is borrowed; use [`Interface::cast_object`] for an owned (counted)
    /// reference.
    #[inline(always)]
    fn cast_object_ref<T>(&self) -> Result<&T::Outer>
    where
        T: ComObjectInner,
        T::Outer: Any + 'static + IUnknownImpl<Impl = T>,
    {
        let any: &dyn Any = self.cast_to_any::<T>()?;
        if let Some(outer) = any.downcast_ref::<T::Outer>() {
            Ok(outer)
        } else {
            Err(imp::E_NOINTERFACE.into())
        }
    }

    /// Returns an owned reference to the generated outer Rust implementation.
    ///
    /// Returns `Err(E_NOINTERFACE)` if the object is not a Rust object, not `T`, or contains
    /// non-static lifetimes.
    ///
    /// The returned value is an owned (counted) reference: this function calls `AddRef`. Use
    /// [`Interface::cast_object_ref`] to avoid `AddRef` / `Release` overhead if you do not need
    /// ownership.
    #[inline(always)]
    fn cast_object<T>(&self) -> Result<ComObject<T>>
    where
        T: ComObjectInner,
        T::Outer: Any + 'static + IUnknownImpl<Impl = T>,
    {
        let object_ref = self.cast_object_ref::<T>()?;
        Ok(object_ref.to_object())
    }

    /// Attempts to create a [`Weak`] reference to this object.
    fn downgrade(&self) -> Result<Weak<Self>> {
        self.cast::<imp::IWeakReferenceSource>()
            .map(|source| Weak::downgrade(&source))
    }

    /// Calls `QueryInterface`.
    ///
    /// # Safety
    ///
    /// `interface` must be a non-null, valid pointer for writing an interface pointer.
    #[inline(always)]
    unsafe fn query(&self, iid: *const GUID, interface: *mut *mut c_void) -> HRESULT {
        unsafe {
            if Self::UNKNOWN {
                (self.assume_vtable::<IUnknown>().QueryInterface)(self.as_raw(), iid, interface)
            } else {
                panic!("Non-COM interfaces cannot be queried.")
            }
        }
    }

    /// Borrows this interface without changing its reference count.
    fn to_ref(&self) -> InterfaceRef<'_, Self> {
        InterfaceRef::from_interface(self)
    }
}

/// A borrowed COM interface pointer with the same representation as `I`.
///
/// This type does not adjust the reference count.
#[repr(transparent)]
pub struct InterfaceRef<'a, I>(NonNull<c_void>, PhantomData<&'a I>);

impl<I> Copy for InterfaceRef<'_, I> {}

impl<I> Clone for InterfaceRef<'_, I> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<I: core::fmt::Debug + Interface> core::fmt::Debug for InterfaceRef<'_, I> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <I as core::fmt::Debug>::fmt(&**self, f)
    }
}

impl<I: Interface> InterfaceRef<'_, I> {
    /// Creates an `InterfaceRef` from a raw pointer. _This is extremely dangerous, since there
    /// is no lifetime tracking at all!_
    ///
    /// # Safety
    /// The caller must guarantee that the `'a` lifetime parameter is bound by context to a correct
    /// lifetime.
    #[inline(always)]
    pub unsafe fn from_raw(ptr: NonNull<c_void>) -> Self {
        Self(ptr, PhantomData)
    }

    /// Creates an `InterfaceRef` from an interface reference. This safely associates the lifetime
    /// of the interface reference with the `'a` parameter of `InterfaceRef`. This allows for
    /// lifetime checking _without_ calling AddRef/Release on the underlying lifetime, which can
    /// improve efficiency.
    #[inline(always)]
    pub fn from_interface(interface: &I) -> Self {
        unsafe {
            // SAFETY: new_unchecked() should be valid because Interface::as_raw should always
            // return a non-null pointer.
            Self(NonNull::new_unchecked(interface.as_raw()), PhantomData)
        }
    }

    /// Calls AddRef on the underlying COM interface and returns an "owned" (counted) reference.
    #[inline(always)]
    pub fn to_owned(self) -> I {
        (*self).clone()
    }
}

impl<'a, 'i: 'a, I: Interface> From<&'i I> for InterfaceRef<'a, I> {
    #[inline(always)]
    fn from(interface: &'a I) -> Self {
        InterfaceRef::from_interface(interface)
    }
}

impl<I: Interface> core::ops::Deref for InterfaceRef<'_, I> {
    type Target = I;

    #[inline(always)]
    fn deref(&self) -> &I {
        unsafe { core::mem::transmute(self) }
    }
}

/// This IID identifies a special protocol, used by [`Interface::cast_to_any`]. This is _not_
/// an ordinary COM interface; it uses special lifetime rules and a larger interface pointer.
/// See the comments on [`Interface::cast_to_any`].
#[doc(hidden)]
pub const DYNAMIC_CAST_IID: GUID = GUID::from_u128(0xae49d5cb_143f_431c_874c_2729336e4eca);
