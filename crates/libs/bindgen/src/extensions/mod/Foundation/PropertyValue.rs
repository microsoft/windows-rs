macro_rules! primitive_boxed_type {
    ($(($t:ty, $m:ident)),+) => {
        $(impl std::convert::TryFrom<$t> for windows::core::IInspectable {
            type Error = windows::core::Error;
            fn try_from(value: $t) -> windows::core::Result<Self> {
                PropertyValue::$m(value)
            }
        }
        impl std::convert::TryFrom<windows::core::IInspectable> for $t {
            type Error = windows::core::Error;
            fn try_from(value: windows::core::IInspectable) -> windows::core::Result<Self> {
                <windows::core::IInspectable as windows::core::Interface>::cast::<IReference<$t>>(&value)?.Value()
            }
        }
        impl std::convert::TryFrom<&windows::core::IInspectable> for $t {
            type Error = windows::core::Error;
            fn try_from(value: &windows::core::IInspectable) -> windows::core::Result<Self> {
                <windows::core::IInspectable as windows::core::Interface>::cast::<IReference<$t>>(value)?.Value()
            }
        })*
    };
}
primitive_boxed_type! {
    (bool, CreateBoolean),
    (u8, CreateUInt8),
    (i16, CreateInt16),
    (u16, CreateUInt16),
    (i32, CreateInt32),
    (u32, CreateUInt32),
    (i64, CreateInt64),
    (u64, CreateUInt64),
    (f32, CreateSingle),
    (f64, CreateDouble)
}
impl std::convert::TryFrom<&str> for windows::core::IInspectable {
    type Error = windows::core::Error;
    fn try_from(value: &str) -> windows::core::Result<Self> {
        let value: windows::core::HSTRING = value.into();
        PropertyValue::CreateString(&value)
    }
}
impl std::convert::TryFrom<windows::core::HSTRING> for windows::core::IInspectable {
    type Error = windows::core::Error;
    fn try_from(value: windows::core::HSTRING) -> windows::core::Result<Self> {
        PropertyValue::CreateString(&value)
    }
}
impl std::convert::TryFrom<&windows::core::HSTRING> for windows::core::IInspectable {
    type Error = windows::core::Error;
    fn try_from(value: &windows::core::HSTRING) -> windows::core::Result<Self> {
        PropertyValue::CreateString(value)
    }
}
impl std::convert::TryFrom<windows::core::IInspectable> for windows::core::HSTRING {
    type Error = windows::core::Error;
    fn try_from(value: windows::core::IInspectable) -> windows::core::Result<Self> {
        <windows::core::IInspectable as windows::core::Interface>::cast::<
            IReference<windows::core::HSTRING>,
        >(&value)?
        .Value()
    }
}
impl std::convert::TryFrom<&windows::core::IInspectable> for windows::core::HSTRING {
    type Error = windows::core::Error;
    fn try_from(value: &windows::core::IInspectable) -> windows::core::Result<Self> {
        <windows::core::IInspectable as windows::core::Interface>::cast::<
            IReference<windows::core::HSTRING>,
        >(value)?
        .Value()
    }
}
