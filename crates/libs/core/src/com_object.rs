use crate::{IUnknown, IUnknownImpl, Interface, InterfaceRef};
use alloc::boxed::Box;
use core::any::Any;
use core::borrow::Borrow;
use core::ops::Deref;
use core::ptr::NonNull;

/// Identifies types that can be placed in [`ComObject`].
///
/// This is an implementation detail of the Windows crates. The `#[implement]` macro generates
/// implementations so user code can use [`ComObject<T>`] instead of `ComObject<T_Impl>`.
pub trait ComObjectInner: Sized {
    /// The generated `<foo>_Impl` type (aka the "boxed" or "outer" type).
    type Outer: IUnknownImpl<Impl = Self>;

    /// Moves an instance of this type into a new `ComObject` box and returns it.
    ///
    /// # Safety
    ///
    /// Safe Rust code must not own a generated "outer" COM object type (e.g. `<foo>_Impl`),
    /// because it carries a reference count and methods that can destroy the object. The
    /// `#[implement]` macro constructs that value, places it on the heap, and returns only a
    /// `ComObject` reference.
    fn into_object(self) -> ComObject<Self>;
}

/// Describes the COM interfaces implemented by a specific COM object.
///
/// This is an implementation detail generated on "outer" types such as `MyApp_Impl`.
pub trait ComObjectInterface<I: Interface> {
    /// Gets a borrowed interface that is implemented by `T`.
    fn as_interface_ref(&self) -> InterfaceRef<'_, I>;
}

/// A counted pointer to a heap-allocated type that implements COM interfaces.
///
/// This type lets you place an object onto the heap and query for COM interfaces without
/// losing the safe reference to the implementation object.
///
/// Because the pointer inside is non-null, `Option<ComObject<T>>` is the same size as a
/// single pointer.
///
/// # Safety
///
/// The contained `ptr` is an owned, reference-counted pointer to a _pinned_
/// `Pin<Box<T::Outer>>`. The implementation does not currently use `Pin<T>` directly but is
/// careful not to expose unsafe semantics to safe code; callers of unsafe functions on
/// [`ComObject`] must preserve these invariants.
#[repr(transparent)]
pub struct ComObject<T: ComObjectInner> {
    ptr: NonNull<T::Outer>,
}

impl<T: ComObjectInner> ComObject<T> {
    /// Allocates a heap cell (box) and moves `value` into it. Returns a counted pointer to `value`.
    pub fn new(value: T) -> Self {
        T::into_object(value)
    }

    /// Creates a new `ComObject` from an existing boxed instance.
    ///
    /// # Safety
    ///
    /// `ptr` must point to a valid, heap-allocated `T::Outer` (typically from
    /// `Box::into_raw(Box::new(...))`).
    ///
    /// The pointed-to box must have a reference count greater than zero.
    ///
    /// This takes ownership of the existing pointer; it does not call `AddRef`. The reference
    /// count must accurately reflect all outstanding references to the box, including `ptr`.
    pub unsafe fn from_raw(ptr: NonNull<T::Outer>) -> Self {
        Self { ptr }
    }

    /// Gets a reference to the shared object stored in the box.
    ///
    /// [`ComObject`] also implements [`Deref`], so it is often more convenient to deref
    /// directly. Use this method when the [`Deref`] impl is inconvenient.
    #[inline(always)]
    pub fn get(&self) -> &T {
        self.get_box().get_impl()
    }

    #[inline(always)]
    fn get_box(&self) -> &T::Outer {
        unsafe { self.ptr.as_ref() }
    }

    // `&mut T::Outer` is not exposed because replacing the outer object would also replace its
    // reference count.

    /// Gets a mutable reference to the object stored in the box, if the reference count is
    /// exactly 1. Returns `None` if there are multiple references.
    #[inline(always)]
    pub fn get_mut(&mut self) -> Option<&mut T> {
        if self.is_reference_count_one() {
            // SAFETY: returning `&mut T::Outer` would allow replacing the refcounted box.
            unsafe { Some(self.ptr.as_mut().get_impl_mut()) }
        } else {
            None
        }
    }

    /// If this object has only a single reference (i.e. this [`ComObject`] is the only
    /// reference to the heap allocation), extracts the inner `T` and frees the heap allocation.
    /// Returns `Err(self)` if there is more than one reference.
    #[inline(always)]
    pub fn take(self) -> Result<T, Self> {
        if self.is_reference_count_one() {
            let outer_box: Box<T::Outer> = unsafe { core::mem::transmute(self) };
            Ok(outer_box.into_inner())
        } else {
            Err(self)
        }
    }

    /// Casts to the given interface type.
    ///
    /// This always performs a `QueryInterface`, even if `T` is known to implement `I`. If you
    /// know that `T` implements `I`, use [`Self::as_interface`] or [`Self::to_interface`]
    /// instead to avoid the dynamic `QueryInterface` call.
    #[inline(always)]
    pub fn cast<I: Interface>(&self) -> windows_core::Result<I>
    where
        T::Outer: ComObjectInterface<IUnknown>,
    {
        let unknown = self.as_interface::<IUnknown>();
        unknown.cast()
    }

    /// Gets a borrowed reference to an interface that is implemented by `T`.
    ///
    /// The returned reference is not `AddRef`ed; call [`InterfaceRef::to_owned`] to obtain
    /// an owned reference.
    #[inline(always)]
    pub fn as_interface<I: Interface>(&self) -> InterfaceRef<'_, I>
    where
        T::Outer: ComObjectInterface<I>,
    {
        self.get_box().as_interface_ref()
    }

    /// Gets an owned (counted) reference to an interface that is implemented by this [`ComObject`].
    #[inline(always)]
    pub fn to_interface<I: Interface>(&self) -> I
    where
        T::Outer: ComObjectInterface<I>,
    {
        self.as_interface::<I>().to_owned()
    }

    /// Converts `self` into an interface that it implements.
    ///
    /// This does not need to adjust reference counts because `self` is consumed.
    #[inline(always)]
    pub fn into_interface<I: Interface>(self) -> I
    where
        T::Outer: ComObjectInterface<I>,
    {
        unsafe {
            let raw = self.get_box().as_interface_ref().as_raw();
            core::mem::forget(self);
            I::from_raw(raw)
        }
    }

    /// Casts the given COM interface to `&dyn Any`, returning a reference to the "outer"
    /// object (e.g. `MyApp_Impl`), not the inner `MyApp`.
    ///
    /// `T` must be a type annotated with `#[implement]`; this is enforced at compile time by
    /// the generic constraints.
    ///
    /// Returns `Err(E_NOINTERFACE)` if the object is not a Rust object, not `T`, or contains
    /// non-static lifetimes.
    ///
    /// The returned value is an owned (counted) reference: this function calls `AddRef`. If
    /// you do not need an owned reference, use [`Interface::cast_object_ref`] instead to
    /// avoid the `AddRef` / `Release` overhead.
    pub fn cast_from<I>(interface: &I) -> crate::Result<Self>
    where
        I: Interface,
        T::Outer: Any + 'static + IUnknownImpl<Impl = T>,
    {
        interface.cast_object()
    }
}

impl<T: ComObjectInner + Default> Default for ComObject<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: ComObjectInner> Drop for ComObject<T> {
    fn drop(&mut self) {
        unsafe {
            T::Outer::Release(self.ptr.as_ptr());
        }
    }
}

impl<T: ComObjectInner> Clone for ComObject<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        unsafe {
            self.ptr.as_ref().AddRef();
            Self { ptr: self.ptr }
        }
    }
}

impl<T: ComObjectInner> AsRef<T> for ComObject<T> {
    #[inline(always)]
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T: ComObjectInner> Deref for ComObject<T> {
    type Target = T::Outer;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.get_box()
    }
}

// There is no DerefMut implementation because we cannot statically guarantee
// that the reference count is 1, which is a requirement for getting exclusive
// access to the contents of the object. Use get_mut() for dynamically-checked
// exclusive access.

impl<T: ComObjectInner> From<T> for ComObject<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

// Delegate hashing, if implemented.
impl<T: ComObjectInner + core::hash::Hash> core::hash::Hash for ComObject<T> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.get().hash(state);
    }
}

// If T is Send (or Sync) then the ComObject<T> is also Send (or Sync).
// Since the actual object storage is in the heap, the object is never moved.
unsafe impl<T: ComObjectInner + Send> Send for ComObject<T> {}
unsafe impl<T: ComObjectInner + Sync> Sync for ComObject<T> {}

impl<T: ComObjectInner + PartialEq> PartialEq for ComObject<T> {
    fn eq(&self, other: &Self) -> bool {
        let inner_self: &T = self.get();
        let other_self: &T = other.get();
        inner_self == other_self
    }
}

impl<T: ComObjectInner + Eq> Eq for ComObject<T> {}

impl<T: ComObjectInner + PartialOrd> PartialOrd for ComObject<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let inner_self: &T = self.get();
        let other_self: &T = other.get();
        <T as PartialOrd>::partial_cmp(inner_self, other_self)
    }
}

impl<T: ComObjectInner + Ord> Ord for ComObject<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let inner_self: &T = self.get();
        let other_self: &T = other.get();
        <T as Ord>::cmp(inner_self, other_self)
    }
}

impl<T: ComObjectInner + core::fmt::Debug> core::fmt::Debug for ComObject<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <T as core::fmt::Debug>::fmt(self.get(), f)
    }
}

impl<T: ComObjectInner + core::fmt::Display> core::fmt::Display for ComObject<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        <T as core::fmt::Display>::fmt(self.get(), f)
    }
}

impl<T: ComObjectInner> Borrow<T> for ComObject<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

/// Stores a COM object in static memory.
///
/// Its reference count starts at one and tracks owned interface references.
pub struct StaticComObject<T>
where
    T: ComObjectInner,
{
    outer: T::Outer,
}

// IMPORTANT: Do not expose methods that return mutable access to the contents of
// `StaticComObject`. Doing so would violate our safety invariants - for example, a `DerefMut`
// impl would be unsound.
impl<T> StaticComObject<T>
where
    T: ComObjectInner,
{
    /// Wraps `outer` in a `StaticComObject`.
    pub const fn from_outer(outer: T::Outer) -> Self {
        Self { outer }
    }
}

impl<T> StaticComObject<T>
where
    T: ComObjectInner,
{
    /// Gets access to the contained value.
    pub const fn get(&'static self) -> &'static T::Outer {
        &self.outer
    }
}

impl<T> Deref for StaticComObject<T>
where
    T: ComObjectInner,
{
    type Target = T::Outer;

    fn deref(&self) -> &Self::Target {
        &self.outer
    }
}
