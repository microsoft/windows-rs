use super::*;

#[doc(hidden)]
pub trait RuntimeType: Type<Self> {
    const SIGNATURE: imp::ConstBuffer;
    const NAME: imp::ConstBuffer = imp::ConstBuffer::new();
}

macro_rules! primitives {
    ($(($t:ty, $s:literal, $name:literal)),+) => {
        $(
            impl RuntimeType for $t {
                const SIGNATURE: imp::ConstBuffer = imp::ConstBuffer::from_slice($s);
                const NAME: imp::ConstBuffer = imp::ConstBuffer::from_slice($name);
            }
        )*
    };
}

primitives! {
    (bool, b"b1", b"Boolean"),
    (i8, b"i1", b"Int8"),
    (u8, b"u1", b"UInt8"),
    (i16, b"i2", b"Int16"),
    (u16, b"u2", b"UInt16"),
    (i32, b"i4", b"Int32"),
    (u32, b"u4", b"UInt32"),
    (i64, b"i8", b"Int64"),
    (u64, b"u8", b"UInt64"),
    (f32, b"f4", b"Single"),
    (f64, b"f8", b"Double")
}
