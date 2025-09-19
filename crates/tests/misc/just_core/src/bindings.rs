#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[inline]
pub unsafe fn CoGetCallerTID() -> windows_core::Result<u32> {
    windows_core::link!("ole32.dll" "system" fn CoGetCallerTID(lpdwtid : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CoGetCallerTID(&mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn IsCharLowerA(ch: i8) -> windows_core::Result<()> {
    windows_core::link!("user32.dll" "system" fn IsCharLowerA(ch : i8) -> windows_core::BOOL);
    unsafe { IsCharLowerA(ch).ok() }
}
#[inline]
pub unsafe fn SysFreeString(bstrstring: &windows_core::BSTR) {
    windows_core::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : * mut core::ffi::c_void));
    unsafe { SysFreeString(core::mem::transmute_copy(bstrstring)) }
}
windows_core::imp::define_interface!(
    IStringable,
    IStringable_Vtbl,
    0x96369f54_8eb6_48f0_abce_c1b211e627c3
);
impl windows_core::RuntimeType for IStringable {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(
    IStringable,
    windows_core::IUnknown,
    windows_core::IInspectable
);
impl IStringable {
    pub fn ToString(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ToString)(
                windows_core::Interface::as_raw(this),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IStringable {
    const NAME: &'static str = "Windows.Foundation.IStringable";
}
pub trait IStringable_Impl: windows_core::IUnknownImpl {
    fn ToString(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IStringable_Vtbl {
    pub const fn new<Identity: IStringable_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ToString<Identity: IStringable_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            result__: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStringable_Impl::ToString(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IStringable, OFFSET>(),
            ToString: ToString::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStringable as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringable_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
