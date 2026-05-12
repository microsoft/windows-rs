//! Step 1 runtime proof: the foundation runs end-to-end against the *real*
//! `windows_core::imp::implement` module. Mirrors the spike's `runtime.rs`
//! but no longer redefines the foundation locally.

use test_implement_foundation::sample::foundation_path::{Foo, Foo_Impl};
use test_implement_foundation::sample::IValue;
use windows_core::{
    ComObject, ComObjectInner, IInspectable, IUnknown, IUnknownImpl, Interface, GUID,
};

#[test]
fn into_object_returns_refcount_one() {
    let obj: ComObject<Foo> = Foo { x: 42 }.into_object();
    assert!(obj.is_reference_count_one());
}

#[test]
fn deref_reaches_user_fields() {
    let obj: ComObject<Foo> = Foo { x: 7 }.into_object();
    // ComObject<Foo>: Deref<Target = Foo_Impl>; Foo_Impl: Deref<Target = Foo>.
    assert_eq!(obj.x, 7);
}

#[test]
fn add_ref_release_round_trip() {
    let obj: ComObject<Foo> = Foo { x: 1 }.into_object();
    let outer: &Foo_Impl = &obj;
    let after_add = outer.AddRef();
    assert_eq!(after_add, 2);
    // SAFETY: ref count > 0 here (we just AddRef'd).
    let after_release = unsafe { <Foo_Impl as IUnknownImpl>::Release(outer as *const _ as *mut _) };
    assert_eq!(after_release, 1);
    assert!(obj.is_reference_count_one());
}

#[test]
fn query_interface_identity_iunknown() {
    let obj: ComObject<Foo> = Foo { x: 0 }.into_object();
    let outer: &Foo_Impl = &obj;
    let mut iface: *mut core::ffi::c_void = core::ptr::null_mut();
    let hr = unsafe { outer.QueryInterface(&IUnknown::IID, &mut iface) };
    assert_eq!(hr.0, 0);
    assert!(!iface.is_null());
    unsafe {
        drop(IUnknown::from_raw(iface));
    }
}

#[test]
fn query_interface_identity_iinspectable() {
    let obj: ComObject<Foo> = Foo { x: 0 }.into_object();
    let outer: &Foo_Impl = &obj;
    let mut iface: *mut core::ffi::c_void = core::ptr::null_mut();
    let hr = unsafe { outer.QueryInterface(&IInspectable::IID, &mut iface) };
    assert_eq!(hr.0, 0);
    assert!(!iface.is_null());
    unsafe {
        drop(IInspectable::from_raw(iface));
    }
}

#[test]
fn query_interface_declared_ivalue() {
    let obj: ComObject<Foo> = Foo { x: 99 }.into_object();
    let outer: &Foo_Impl = &obj;
    let mut iface: *mut core::ffi::c_void = core::ptr::null_mut();
    let hr = unsafe { outer.QueryInterface(&IValue::IID, &mut iface) };
    assert_eq!(hr.0, 0, "QI(IValue) hr = {:#x}", hr.0 as u32);
    assert!(!iface.is_null());

    let value = unsafe { IValue::from_raw(iface) };
    let got = unsafe { value.get() };
    assert_eq!(got, 99);
}

#[test]
fn query_interface_unknown_iid_returns_e_nointerface() {
    let obj: ComObject<Foo> = Foo { x: 0 }.into_object();
    let outer: &Foo_Impl = &obj;
    let mut iface: *mut core::ffi::c_void = 0xdead_beef as *mut _;
    let unknown_iid = GUID::from_u128(0xdeadbeef_dead_beef_dead_beefdeadbeef);
    let hr = unsafe { outer.QueryInterface(&unknown_iid, &mut iface) };
    // E_NOINTERFACE == 0x80004002
    assert_eq!(hr.0 as u32, 0x80004002);
    assert!(iface.is_null());
}

#[test]
fn from_foo_into_iunknown_round_trips() {
    let unk: IUnknown = Foo { x: 5 }.into();
    let value: IValue = unk.cast().expect("IUnknown should QI to IValue");
    let got = unsafe { value.get() };
    assert_eq!(got, 5);
}
