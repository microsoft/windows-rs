use super::Type;

/// Similar to the Rust Standard Library's `ManuallyDrop` wrapper, the `windows` crate's `ManuallyDrop`
/// makes it easier to work with Win32 structs that lack ownership semantics but nevertheless allow
/// callers to reference such non-blittable fields. Rust's `ManuallyDrop` is not used primarily because
// it makes it very difficult to reference the wrapped value without taking ownership of it.
#[repr(transparent)]
#[derive(Debug)]
pub struct ManuallyDrop<T: Type<T>> {
    abi: T::Abi,
}

impl<T: Type<T>> ManuallyDrop<T> {
    pub fn new(value: &T) -> Self {
        unsafe { std::mem::transmute_copy(value) }
    }

    pub fn abi(&self) -> T::Abi {
        unsafe { std::mem::transmute_copy(&self.abi) }
    }

    pub fn as_ref(&self) -> Option<&T> {
        unsafe { <T as Type<T>>::from_abi_ref(&self.abi) }
    }

    #[track_caller]
    pub fn unwrap(&self) -> &T {
        self.as_ref().unwrap()
    }

    pub fn none() -> Self {
        unsafe { std::mem::zeroed() }
    }
}

impl<T: Type<T>> Clone for ManuallyDrop<T> {
    fn clone(&self) -> Self {
        unsafe { std::mem::transmute_copy(self) }
    }
}

impl<T: Type<T> + PartialEq> Eq for ManuallyDrop<T> {}

impl<T: Type<T> + PartialEq> PartialEq for ManuallyDrop<T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<'a, T: 'a, U> From<&'a U> for ManuallyDrop<T>
where
    T: Type<T>,
    &'a U: Into<&'a T>,
{
    fn from(item: &'a U) -> Self {
        ManuallyDrop::new(item.into())
    }
}

impl<'a, T: Type<T>> From<Option<&'a T>> for ManuallyDrop<T>
where
    &'a T: Into<ManuallyDrop<T>>,
{
    fn from(item: Option<&'a T>) -> Self {
        item.map(|i| i.into()).unwrap_or_else(|| ManuallyDrop::none())
    }
}

macro_rules! primitive_types {
    ($($t:ty),+) => {
        $(
            impl From<$t> for ManuallyDrop<$t>
            {
                fn from(item: $t) -> Self {
                    ManuallyDrop::new(&item)
                }
            }
        )*
    };
}

primitive_types!(bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize, super::PCSTR, super::PCWSTR);
