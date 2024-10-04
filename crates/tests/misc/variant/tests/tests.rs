use windows::Foundation::Uri;
use windows::Win32::Foundation::{E_INVALIDARG, TYPE_E_TYPEMISMATCH};
use windows::Win32::System::Com;
use windows::Win32::System::Com::StructuredStorage::PROPVARIANT;
use windows::Win32::System::Variant::*;
use windows_core::*;

#[test]
fn test_variant() -> Result<()> {
    unsafe { Com::CoIncrementMTAUsage()? };

    let empty: VARIANT = VARIANT::default();
    assert!(empty.is_empty());

    let v = VARIANT::default();
    assert!(v.is_empty());
    assert_eq!(v.vt(), VT_EMPTY);

    let v = VARIANT::from(true);
    assert!(!v.is_empty());
    assert_eq!(v.vt(), VT_BOOL);
    assert!(bool::try_from(&v)?);
    let v = VARIANT::from(false);
    assert!(!(bool::try_from(&v)?));
    assert_eq!(VARIANT::from(true), VARIANT::from(true));
    assert_eq!(VARIANT::from(false), VARIANT::from(false));
    assert_ne!(VARIANT::from(true), VARIANT::from(false));

    let v = VARIANT::from(123u8);
    assert_eq!(u16::try_from(&v)?, 123u16);
    assert_eq!(VARIANT::from(123u8), VARIANT::from(123u8));
    assert_ne!(VARIANT::from(123u8), VARIANT::from(0u8));

    let v = VARIANT::from(-123i8);
    assert_eq!(i16::try_from(&v)?, -123i16);
    assert_eq!(VARIANT::from(-123i8), VARIANT::from(-123i8));
    assert_ne!(VARIANT::from(-123i8), VARIANT::from(0i8));

    let v = VARIANT::from(345u16);
    assert_eq!(u16::try_from(&v)?, 345u16);
    assert_eq!(VARIANT::from(345u16), VARIANT::from(345u16));
    assert_ne!(VARIANT::from(345u16), VARIANT::from(0u16));

    let v = VARIANT::from(-345i16);
    assert_eq!(i16::try_from(&v)?, -345i16);
    assert_eq!(VARIANT::from(-345i16), VARIANT::from(-345i16));
    assert_ne!(VARIANT::from(-345i16), VARIANT::from(0i16));

    let v = VARIANT::from(67890u32);
    assert_eq!(u32::try_from(&v)?, 67890u32);
    assert_eq!(VARIANT::from(67890u32), VARIANT::from(67890u32));
    assert_ne!(VARIANT::from(67890u32), VARIANT::from(0u32));

    let v = VARIANT::from(-67890i32);
    assert_eq!(i32::try_from(&v)?, -67890i32);
    assert_eq!(VARIANT::from(-67890i32), VARIANT::from(-67890i32));
    assert_ne!(VARIANT::from(-67890i32), VARIANT::from(0i32));

    let v = VARIANT::from(5294967295u64);
    assert_eq!(u64::try_from(&v)?, 5294967295u64);
    assert_eq!(VARIANT::from(5294967295u64), VARIANT::from(5294967295u64));
    assert_ne!(VARIANT::from(5294967295u64), VARIANT::from(0u64));

    let v = VARIANT::from(-5294967295i64);
    assert_eq!(i64::try_from(&v)?, -5294967295i64);
    assert_eq!(VARIANT::from(-5294967295i64), VARIANT::from(-5294967295i64));
    assert_ne!(VARIANT::from(-5294967295i64), VARIANT::from(0i64));

    let v = VARIANT::from(3.5f32);
    assert_eq!(f64::try_from(&v)?, 3.5f64);
    assert_eq!(VARIANT::from(3.5f32), VARIANT::from(3.5f32));
    assert_ne!(VARIANT::from(3.5f32), VARIANT::from(0.0f32));

    let v = VARIANT::from(3.5f64);
    assert_eq!(f64::try_from(&v)?, 3.5f64);
    assert_eq!(VARIANT::from(3.5f64), VARIANT::from(3.5f64));
    assert_ne!(VARIANT::from(3.5f64), VARIANT::from(0.0f64));

    let unknown: IUnknown = Uri::CreateUri(h!("https://github.com/"))?.into();
    let v = VARIANT::from(unknown);
    let unknown = IUnknown::try_from(&v)?;
    assert_eq!(unknown.cast::<Uri>()?.Domain()?, "github.com");
    assert_eq!(i32::try_from(&v).unwrap_err().code(), TYPE_E_TYPEMISMATCH);
    assert_eq!(
        IUnknown::try_from(&VARIANT::from(3.5f64))
            .unwrap_err()
            .code(),
        TYPE_E_TYPEMISMATCH
    );

    let dispatch: Com::IDispatch =
        unsafe { Com::CoCreateInstance(&Com::Events::CEventSystem, None, Com::CLSCTX_ALL)? };
    let v = VARIANT::from(dispatch);
    let dispatch = Com::IDispatch::try_from(&v)?;
    dispatch.cast::<Com::Events::IEventSystem>()?;
    assert_eq!(i32::try_from(&v).unwrap_err().code(), E_INVALIDARG);
    assert_eq!(
        Com::IDispatch::try_from(&VARIANT::from(3.5f64))
            .unwrap_err()
            .code(),
        TYPE_E_TYPEMISMATCH
    );

    let v = VARIANT::from(BSTR::from("hello"));
    assert_eq!(BSTR::try_from(&v)?, "hello");
    assert_eq!(
        VARIANT::from(BSTR::from("hello")),
        VARIANT::from(BSTR::from("hello"))
    );
    assert_ne!(
        VARIANT::from(BSTR::from("hello")),
        VARIANT::from(BSTR::from("goodbye"))
    );

    let v = VARIANT::from("hello");
    assert_eq!(BSTR::try_from(&v)?, "hello");
    assert_eq!(VARIANT::from("hello"), VARIANT::from("hello"));
    assert_ne!(VARIANT::from("hello"), VARIANT::from("goodbye"));

    let v = VARIANT::from(3.5f64);
    assert_eq!(BSTR::try_from(&v)?, "3.5");

    assert_eq!(format!("{v:?}"), "VARIANT { type: VARENUM(5), value: 3.5 }");
    assert_eq!(format!("{v}"), "3.5");

    let clone = v.clone();
    assert_eq!(v, clone);
    assert_eq!(v, VARIANT::from(3.5f64));
    assert_ne!(v, VARIANT::from(true));

    Ok(())
}

#[test]
fn test_propvariant() -> Result<()> {
    unsafe { Com::CoIncrementMTAUsage()? };

    let empty: PROPVARIANT = PROPVARIANT::default();
    assert!(empty.is_empty());

    let v = PROPVARIANT::default();
    assert!(v.is_empty());
    assert_eq!(v.vt(), VT_EMPTY);

    let v = PROPVARIANT::from(true);
    assert!(!v.is_empty());
    assert_eq!(v.vt(), VT_BOOL);
    assert!(bool::try_from(&v)?);
    let v = PROPVARIANT::from(false);
    assert!(!(bool::try_from(&v)?));
    assert_eq!(PROPVARIANT::from(true), PROPVARIANT::from(true));
    assert_eq!(PROPVARIANT::from(false), PROPVARIANT::from(false));
    assert_ne!(PROPVARIANT::from(true), PROPVARIANT::from(false));

    let v = PROPVARIANT::from(123u8);
    assert_eq!(u16::try_from(&v)?, 123u16);
    assert_eq!(PROPVARIANT::from(123u8), PROPVARIANT::from(123u8));
    assert_ne!(PROPVARIANT::from(123u8), PROPVARIANT::from(0u8));

    let v = PROPVARIANT::from(-123i8);
    assert_eq!(i16::try_from(&v)?, -123i16);
    assert_eq!(PROPVARIANT::from(-123i8), PROPVARIANT::from(-123i8));
    assert_ne!(PROPVARIANT::from(-123i8), PROPVARIANT::from(0i8));

    let v = PROPVARIANT::from(345u16);
    assert_eq!(u16::try_from(&v)?, 345u16);
    assert_eq!(PROPVARIANT::from(345u16), PROPVARIANT::from(345u16));
    assert_ne!(PROPVARIANT::from(345u16), PROPVARIANT::from(0u16));

    let v = PROPVARIANT::from(-345i16);
    assert_eq!(i16::try_from(&v)?, -345i16);
    assert_eq!(PROPVARIANT::from(-345i16), PROPVARIANT::from(-345i16));
    assert_ne!(PROPVARIANT::from(-345i16), PROPVARIANT::from(0i16));

    let v = PROPVARIANT::from(67890u32);
    assert_eq!(u32::try_from(&v)?, 67890u32);
    assert_eq!(PROPVARIANT::from(67890u32), PROPVARIANT::from(67890u32));
    assert_ne!(PROPVARIANT::from(67890u32), PROPVARIANT::from(0u32));

    let v = PROPVARIANT::from(-67890i32);
    assert_eq!(i32::try_from(&v)?, -67890i32);
    assert_eq!(PROPVARIANT::from(-67890i32), PROPVARIANT::from(-67890i32));
    assert_ne!(PROPVARIANT::from(-67890i32), PROPVARIANT::from(0i32));

    let v = PROPVARIANT::from(5294967295u64);
    assert_eq!(u64::try_from(&v)?, 5294967295u64);
    assert_eq!(
        PROPVARIANT::from(5294967295u64),
        PROPVARIANT::from(5294967295u64)
    );
    assert_ne!(PROPVARIANT::from(5294967295u64), PROPVARIANT::from(0u64));

    let v = PROPVARIANT::from(-5294967295i64);
    assert_eq!(i64::try_from(&v)?, -5294967295i64);
    assert_eq!(
        PROPVARIANT::from(-5294967295i64),
        PROPVARIANT::from(-5294967295i64)
    );
    assert_ne!(PROPVARIANT::from(-5294967295i64), PROPVARIANT::from(0i64));

    let v = PROPVARIANT::from(3.5f32);
    assert_eq!(f64::try_from(&v)?, 3.5f64);
    assert_eq!(PROPVARIANT::from(3.5f32), PROPVARIANT::from(3.5f32));
    assert_ne!(PROPVARIANT::from(3.5f32), PROPVARIANT::from(0.0f32));

    let v = PROPVARIANT::from(3.5f64);
    assert_eq!(f64::try_from(&v)?, 3.5f64);
    assert_eq!(PROPVARIANT::from(3.5f64), PROPVARIANT::from(3.5f64));
    assert_ne!(PROPVARIANT::from(3.5f64), PROPVARIANT::from(0.0f64));

    let unknown: IUnknown = Uri::CreateUri(h!("https://github.com/"))?.into();
    let v = PROPVARIANT::from(unknown);
    let unknown = IUnknown::try_from(&v)?;
    assert_eq!(unknown.cast::<Uri>()?.Domain()?, "github.com");
    assert_eq!(i32::try_from(&v).unwrap_err().code(), TYPE_E_TYPEMISMATCH);
    assert_eq!(
        IUnknown::try_from(&PROPVARIANT::from(3.5f64))
            .unwrap_err()
            .code(),
        TYPE_E_TYPEMISMATCH
    );

    let dispatch: Com::IDispatch =
        unsafe { Com::CoCreateInstance(&Com::Events::CEventSystem, None, Com::CLSCTX_ALL)? };
    let v = PROPVARIANT::from(dispatch);
    let dispatch = Com::IDispatch::try_from(&v)?;
    dispatch.cast::<Com::Events::IEventSystem>()?;
    assert_eq!(i32::try_from(&v).unwrap_err().code(), E_INVALIDARG);
    assert_eq!(
        Com::IDispatch::try_from(&PROPVARIANT::from(3.5f64))
            .unwrap_err()
            .code(),
        TYPE_E_TYPEMISMATCH
    );

    let v = PROPVARIANT::from(BSTR::from("hello"));
    assert_eq!(BSTR::try_from(&v)?, "hello");
    assert_eq!(
        PROPVARIANT::from(BSTR::from("hello")),
        PROPVARIANT::from(BSTR::from("hello"))
    );
    assert_ne!(
        PROPVARIANT::from(BSTR::from("hello")),
        PROPVARIANT::from(BSTR::from("goodbye"))
    );

    let v = PROPVARIANT::from("hello");
    assert_eq!(BSTR::try_from(&v)?, "hello");
    assert_eq!(PROPVARIANT::from("hello"), PROPVARIANT::from("hello"));
    assert_ne!(PROPVARIANT::from("hello"), PROPVARIANT::from("goodbye"));

    let v = PROPVARIANT::from(3.5f64);
    assert_eq!(BSTR::try_from(&v)?, "3.5");

    assert_eq!(
        format!("{v:?}"),
        "PROPVARIANT { type: VARENUM(5), value: 3.5 }"
    );
    assert_eq!(format!("{v}"), "3.5");

    let clone = v.clone();
    assert_eq!(v, clone);
    assert_eq!(v, PROPVARIANT::from(3.5f64));
    assert_ne!(v, PROPVARIANT::from(true));

    Ok(())
}
