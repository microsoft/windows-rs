use super::*;

pub trait DefaultType: Sized + Clone + PartialEq {
    type DefaultType: Sized + Clone + PartialEq;

    /// # Safety
    unsafe fn from_default(value: &Self::DefaultType) -> Result<Self> {
        let value = value as *const _ as *const Self;
        Ok((*value).clone())
    }
}

impl<T: Interface + Clone + PartialEq> DefaultType for T {
    type DefaultType = Option<T>;

    unsafe fn from_default(value: &Self::DefaultType) -> Result<Self> {
        let value = value as *const _ as *const Option<Self>;

        match &*value {
            Some(value) => Ok(value.clone()),
            None => Err(Error::OK),
        }
    }
}

#[doc(hidden)]
pub unsafe trait RuntimeType: Abi + DefaultType + PartialEq {
    const SIGNATURE: ConstBuffer;
}

macro_rules! primitive_runtime_types {
    ($(($t:ty, $s:literal)),+) => {
        $(
            unsafe impl RuntimeType for $t {
                const SIGNATURE: ConstBuffer = ConstBuffer::from_slice($s);
            }
            impl DefaultType for $t {
                type DefaultType = Self;
            }
            unsafe impl Abi for $t {
                type Abi = Self;
            }
        )*
    };
}

primitive_runtime_types! {
    (bool, b"b1"),
    (i8, b"i1"),
    (u8, b"u1"),
    (i16, b"i2"),
    (u16, b"u2"),
    (i32, b"i4"),
    (u32, b"u4"),
    (i64, b"i8"),
    (u64, b"u8"),
    (f32, b"f4"),
    (f64, b"f8")
}
