use windows::Foundation::Uri;
use windows::Win32::Foundation::{E_INVALIDARG, TYPE_E_TYPEMISMATCH};
use windows::Win32::System::Com;
use windows_core::*;

#[test]
fn test_variant() -> Result<()> {
    unsafe { Com::CoIncrementMTAUsage()? };

    let empty: VARIANT = VARIANT::new();
    assert!(empty.is_empty());
    assert!(!empty.is_null());

    let v = VARIANT::default();
    assert!(v.is_empty());
    assert!(!v.is_null());

    assert_eq!(VARIANT::new(), VARIANT::default());

    let v = VARIANT::null();
    assert!(v.is_null());
    assert!(!v.is_empty());
    assert_ne!(v, VARIANT::new());

    let v = VARIANT::from(true);
    assert!(!v.is_empty());
    assert_eq!(bool::try_from(&v)?, true);
    let v = VARIANT::from(false);
    assert_eq!(bool::try_from(&v)?, false);
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

    let v = VARIANT::from("hello".to_owned());
    assert_eq!(BSTR::try_from(&v)?, "hello");
    assert_eq!(
        VARIANT::from("hello".to_owned()),
        VARIANT::from("hello".to_owned())
    );
    assert_ne!(
        VARIANT::from("hello".to_owned()),
        VARIANT::from("goodbye".to_owned())
    );

    let v = VARIANT::from(3.5f64);
    assert_eq!(BSTR::try_from(&v)?, "3.5");

    assert_eq!(format!("{v:?}"), "VARIANT { type: 5, value: 3.5 }");
    assert_eq!(format!("{v}"), "3.5");

    let clone = v.clone();
    assert_eq!(v, clone);
    assert_eq!(v, VARIANT::from(3.5f64));
    assert_ne!(v, VARIANT::from(true));

    let v = VARIANT::try_from(&["abc", "def", "xyz", "123"][..])?;
    assert!(!v.is_empty());
    assert!(!v.is_null());
    assert_eq!(
        VARIANT::try_from(&["abc", "def", "xyz", "123"][..])?,
        VARIANT::try_from(&["abc", "def", "xyz", "123"][..])?
    );
    assert_ne!(
        VARIANT::try_from(&["abc", "def", "xyz", "123"][..])?,
        VARIANT::try_from(&["hello", "world"][..])?
    );

    let v = VARIANT::try_from(
        &[
            "abc".to_owned(),
            "def".to_owned(),
            "xyz".to_owned(),
            "123".to_owned(),
        ][..],
    )?;
    assert!(!v.is_empty());
    assert!(!v.is_null());
    assert_eq!(
        VARIANT::try_from(
            &[
                "abc".to_owned(),
                "def".to_owned(),
                "xyz".to_owned(),
                "123".to_owned(),
            ][..]
        )?,
        VARIANT::try_from(
            &[
                "abc".to_owned(),
                "def".to_owned(),
                "xyz".to_owned(),
                "123".to_owned(),
            ][..]
        )?
    );
    assert_ne!(
        VARIANT::try_from(
            &[
                "abc".to_owned(),
                "def".to_owned(),
                "xyz".to_owned(),
                "123".to_owned(),
            ][..]
        )?,
        VARIANT::try_from(&["hello".to_owned(), "world".to_owned()][..])?
    );

    Ok(())
}

#[test]
fn test_propvariant() -> Result<()> {
    unsafe { Com::CoIncrementMTAUsage()? };

    let empty: PROPVARIANT = PROPVARIANT::new();
    assert!(empty.is_empty());
    assert!(!empty.is_null());

    let v = PROPVARIANT::default();
    assert!(v.is_empty());
    assert!(!v.is_null());

    assert_eq!(PROPVARIANT::new(), PROPVARIANT::default());

    let v = PROPVARIANT::null();
    assert!(v.is_null());
    assert!(!v.is_empty());
    assert_ne!(v, PROPVARIANT::new());

    let v = PROPVARIANT::from(true);
    assert!(!v.is_empty());
    assert_eq!(bool::try_from(&v)?, true);
    let v = PROPVARIANT::from(false);
    assert_eq!(bool::try_from(&v)?, false);
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

    let v = PROPVARIANT::from("hello".to_owned());
    assert_eq!(BSTR::try_from(&v)?, "hello");
    assert_eq!(
        PROPVARIANT::from("hello".to_owned()),
        PROPVARIANT::from("hello".to_owned())
    );
    assert_ne!(
        PROPVARIANT::from("hello".to_owned()),
        PROPVARIANT::from("goodbye".to_owned())
    );

    let v = PROPVARIANT::from(3.5f64);
    assert_eq!(BSTR::try_from(&v)?, "3.5");

    assert_eq!(format!("{v:?}"), "PROPVARIANT { type: 5, value: 3.5 }");
    assert_eq!(format!("{v}"), "3.5");

    let clone = v.clone();
    assert_eq!(v, clone);
    assert_eq!(v, PROPVARIANT::from(3.5f64));
    assert_ne!(v, PROPVARIANT::from(true));

    let v = PROPVARIANT::try_from(&["abc", "def", "xyz", "123"][..])?;
    assert!(!v.is_empty());
    assert!(!v.is_null());
    assert_eq!(
        PROPVARIANT::try_from(&["abc", "def", "xyz", "123"][..])?,
        PROPVARIANT::try_from(&["abc", "def", "xyz", "123"][..])?
    );
    assert_ne!(
        PROPVARIANT::try_from(&["abc", "def", "xyz", "123"][..])?,
        PROPVARIANT::try_from(&["hello", "world"][..])?
    );

    let v = PROPVARIANT::try_from(
        &[
            "abc".to_owned(),
            "def".to_owned(),
            "xyz".to_owned(),
            "123".to_owned(),
        ][..],
    )?;
    assert!(!v.is_empty());
    assert!(!v.is_null());
    assert_eq!(
        PROPVARIANT::try_from(
            &[
                "abc".to_owned(),
                "def".to_owned(),
                "xyz".to_owned(),
                "123".to_owned(),
            ][..]
        )?,
        PROPVARIANT::try_from(
            &[
                "abc".to_owned(),
                "def".to_owned(),
                "xyz".to_owned(),
                "123".to_owned(),
            ][..]
        )?
    );
    assert_ne!(
        PROPVARIANT::try_from(
            &[
                "abc".to_owned(),
                "def".to_owned(),
                "xyz".to_owned(),
                "123".to_owned(),
            ][..]
        )?,
        PROPVARIANT::try_from(&["hello".to_owned(), "world".to_owned()][..])?
    );

    Ok(())
}
