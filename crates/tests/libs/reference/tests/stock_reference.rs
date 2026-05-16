#![cfg(windows)]
#![expect(non_snake_case)]

use windows_core::*;
use windows_reference::*;

#[test]
fn primitive_i32() -> Result<()> {
    let r = IReference::<i32>::from(42);
    assert_eq!(r.Value()?, 42);
    Ok(())
}

#[test]
fn primitive_bool() -> Result<()> {
    let r = IReference::<bool>::from(true);
    assert!(r.Value()?);

    let r = IReference::<bool>::from(false);
    assert!(!r.Value()?);
    Ok(())
}

#[test]
fn primitive_f64() -> Result<()> {
    let r = IReference::<f64>::from(1.5);
    assert_eq!(r.Value()?, 1.5);
    Ok(())
}

#[test]
fn hstring_value() -> Result<()> {
    let r = IReference::<HSTRING>::from(HSTRING::from("hello"));
    assert_eq!(r.Value()?, HSTRING::from("hello"));
    Ok(())
}

#[test]
fn guid_value() -> Result<()> {
    let g = GUID::from_u128(0x11223344_5566_7788_99AA_BBCCDDEEFF00);
    let r = IReference::<GUID>::from(g);
    assert_eq!(r.Value()?, g);
    Ok(())
}

#[test]
fn use_as_inspectable() -> Result<()> {
    // A boxed `IReference<T>` can be cast to `IInspectable` since `IReference`
    // derives from `IInspectable`.
    let r = IReference::<i32>::from(99);
    let _inspectable: IInspectable = r.cast()?;
    Ok(())
}

#[test]
fn cloning_preserves_value() -> Result<()> {
    let r = IReference::<i32>::from(7);
    let r2 = r.clone();
    assert_eq!(r.Value()?, 7);
    assert_eq!(r2.Value()?, 7);
    Ok(())
}

// ABI conformance: `IReference<T>` derives from `IPropertyValue`, so a boxed reference
// must respond to `QueryInterface(IID_IPropertyValue)` and dispatch the `IPropertyValue`
// vtable correctly. The stock implementation reports `PropertyType::OtherType` for
// `Type()`, returns `false` from `IsNumericScalar()`, and rejects typed accessors with
// `E_NOTIMPL`. This test exercises that contract through the raw `IPropertyValue` vtable.
//
// `windows-reference`'s private `bindings::IPropertyValue` projection is not re-exported,
// so we hand-declare just enough of the vtable here. The IID is taken from
// `Windows.Foundation.IPropertyValue` (4bd682dd-7554-40e9-9a9b-82654ede7e62) and
// `PropertyType::OtherType` has metadata value 13.

const E_NOTIMPL: HRESULT = HRESULT(0x80004001_u32 as _);
const PROPERTY_TYPE_OTHER_TYPE: i32 = 13;

const IID_IPROPERTY_VALUE: GUID = GUID::from_u128(0x4bd682dd_7554_40e9_9a9b_82654ede7e62);

#[repr(C)]
struct IPropertyValueVtblPrefix {
    base__: IInspectable_Vtbl,
    Type: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> HRESULT,
    IsNumericScalar: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> HRESULT,
    GetUInt8: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> HRESULT,
    GetInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> HRESULT,
    GetUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> HRESULT,
    GetInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> HRESULT,
}

#[test]
fn property_value_abi() -> Result<()> {
    let r = IReference::<i32>::from(123);
    let unknown: IUnknown = r.cast()?;

    // QueryInterface for IPropertyValue must succeed: IReference<T> derives from it.
    let pv: *mut core::ffi::c_void = unsafe {
        let mut raw = core::ptr::null_mut();
        Interface::query(&unknown, &IID_IPROPERTY_VALUE, &mut raw).ok()?;
        raw
    };
    assert!(!pv.is_null());

    // The first vtable slot after IInspectable's three methods is `Type`. Read it through
    // the hand-declared vtable prefix to verify the slot dispatches into the stock impl.
    let vtable: &IPropertyValueVtblPrefix =
        unsafe { &**(pv as *const *const IPropertyValueVtblPrefix) };

    // Type() must report `OtherType` (13).
    let mut ty: i32 = 0;
    unsafe { (vtable.Type)(pv, &mut ty).ok()? };
    assert_eq!(ty, PROPERTY_TYPE_OTHER_TYPE);

    // IsNumericScalar() must report `false`.
    let mut numeric: bool = true;
    unsafe { (vtable.IsNumericScalar)(pv, &mut numeric).ok()? };
    assert!(!numeric);

    // Typed accessors must return E_NOTIMPL — callers should use `Value()`.
    let mut u8_out: u8 = 0;
    let mut i16_out: i16 = 0;
    let mut u16_out: u16 = 0;
    let mut i32_out: i32 = 0;
    for hr in unsafe {
        [
            (vtable.GetUInt8)(pv, &mut u8_out),
            (vtable.GetInt16)(pv, &mut i16_out),
            (vtable.GetUInt16)(pv, &mut u16_out),
            (vtable.GetInt32)(pv, &mut i32_out),
        ]
    } {
        assert_eq!(hr, E_NOTIMPL);
    }

    // Release the QI'd reference.
    unsafe { IUnknown::from_raw(pv) };

    Ok(())
}
