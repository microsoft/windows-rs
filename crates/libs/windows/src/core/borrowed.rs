#![warn(unsafe_op_in_unsafe_fn)]

/// A logically borrowed type that still retains the in-memory representation of the underlying type.
///
/// Generally a value of type `Borrowed<'a, T>` can be thought of as equivalent to `&'a T` with the only different being that 
/// in-memory it has the exact same layout as `T`. This type is used at the FFI boundary to bridge Rust's ownership semantics
/// and the looser semantics around ownership in the Windows API.
///
/// # Conversion
///
/// If you're looking to convert from a value of type `T` to a `Borrowed<'a, T>` there will generally be a `From` implementation 
/// that achieves this. For example, if you want to convert an `IUnknown` to a `Borrowed<'a, IUnknown>` you simply need to call 
/// `.into()` on that value.
///
/// Many APIs take a params of the form `Into<Borrowed<'a, T>>`. This means you can pass anything that has an `Into` implementation 
/// for `Borrowed<'a, T>`. Generally, if this is safe to do, the `windows` crate provides an implementation.
#[repr(transparent)]
pub struct Borrowed<'a, T> {
	item: core::mem::ManuallyDrop<T>,
    lifetime: core::marker::PhantomData<&'a ()>
}

impl <'a, T> Borrowed<'a, T> {
    /// Create a new `Borrowed` value.
    ///
    /// Normally, it is not necessary to use this function. Generally, there is a `From` implementation
    /// that allows you to call `.into` to safely create a `Borrowed` value.
    ///
	/// # Safety
    ///
    /// It must be safe to alias `T` as long as `T`'s destructor is not run. 
    ///
    /// For example, `IUnknown` is fine to alias as long as the reference count is guaranteed to stay above 0 
    /// until the original `item` is dropped. On the other hand `Box<T>` is not fine to alias and thus passing a `Box` 
    /// to this function would be UB.
    pub unsafe fn new(item: &'a T) -> Self {
        let item = unsafe { core::mem::transmute_copy(item) };
        Self {
            item,
            lifetime: core::marker::PhantomData
        }
    }
}

impl<'a, T: super::Abi> Borrowed<'a, T> {
    pub fn abi(&self) -> T::Abi {
        // TODO: safety
        unsafe { std::mem::transmute_copy(self) }
    }
}

impl<'a, T> std::ops::Deref for Borrowed<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.item
    }
}
