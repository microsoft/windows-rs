/// Similar to the Rust Standard Library's `ManuallyDrop` wrapper, the `windows` crate's `ManuallyDrop`
/// makes it easier to work with Win32 structs that lack ownership semantics but nevertheless allow
/// callers to reference such non-blittable fields. Rust's `ManuallyDrop` is not used primarily because
// it makes it very difficult to reference the wrapped value without taking ownership of it.
#[repr(transparent)]
#[derive(Debug)]
pub struct ManuallyDrop<T: super::Abi> {
    abi: T::Abi,
}

impl<T: super::Abi> ManuallyDrop<T> {
    pub fn new(value: &T) -> Self {
        Self { abi: value.abi() }
    }

    pub fn abi(&self) -> T::Abi {
        unsafe { std::mem::transmute_copy(&self.abi) }
    }

    pub fn as_ref(&self) -> Option<&T> {
        unsafe { <T as super::Abi>::from_abi_ref(&self.abi).ok() }
    }

    #[track_caller]
    pub fn unwrap(&self) -> &T {
        self.as_ref().unwrap()
    }

    pub fn none() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl<T: super::Abi> Clone for ManuallyDrop<T> {
    fn clone(&self) -> Self {
        unsafe { std::mem::transmute_copy(self) }
    }
}

impl<T: super::Abi + PartialEq> Eq for ManuallyDrop<T> {}

impl<T: super::Abi + PartialEq> PartialEq for ManuallyDrop<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}
