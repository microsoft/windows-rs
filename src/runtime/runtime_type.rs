use super::*;

#[doc(hidden)]
pub unsafe trait RuntimeType: Abi + Clone + PartialEq {
    const SIGNATURE: ConstBuffer;
}

macro_rules! primitive_runtime_types {
    ($(($t:ty, $s:literal)),+) => {
        $(
            unsafe impl RuntimeType for $t {
                const SIGNATURE: ConstBuffer = ConstBuffer::from_slice($s);
            }
            unsafe impl Abi for $t {
                type Abi = Self;
                type DefaultType = Self;
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

unsafe impl Abi for isize {
    type Abi = Self;
    type DefaultType = Self;
}

unsafe impl Abi for usize {
    type Abi = Self;
    type DefaultType = Self;
}
