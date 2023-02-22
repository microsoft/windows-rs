use super::*;

pub enum InParam<T: Type<T>> {
    Owned(T),
    Borrowed(T::Abi),
}

impl<T: Type<T>> InParam<T> {
    pub fn abi(&self) -> T::Abi {
        unsafe {
            match self {
                Self::Owned(item) => std::mem::transmute_copy(item),
                Self::Borrowed(borrowed) => std::mem::transmute_copy(borrowed),
            }
        }
    }
}

impl<'a, T, U: Type<U>> From<&'a T> for InParam<U>
where
    &'a T: Into<ManuallyDrop<U>>,
{
    fn from(borrowed: &'a T) -> Self {
        unsafe { InParam::Borrowed(std::mem::transmute_copy(borrowed)) }
    }
}

impl<'a, T: Type<T>> From<Option<&'a T>> for InParam<T>
where
    &'a T: Into<ManuallyDrop<T>>,
{
    fn from(item: Option<&'a T>) -> Self {
        unsafe { InParam::Borrowed(std::mem::transmute_copy(&item)) }
    }
}

macro_rules! primitive_types {
    ($($t:ty),+) => {
        $(
            impl From<$t> for InParam<$t> {
                fn from(item: $t) -> Self {
                    Self::Owned(item)
                }
            }
        )*
    };
}

primitive_types!(bool, i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, usize, isize, super::PCSTR, super::PCWSTR);
