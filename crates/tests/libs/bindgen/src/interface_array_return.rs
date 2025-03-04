#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_core::imp::define_interface!(
    IDispatch,
    IDispatch_Vtbl,
    0x00020400_0000_0000_c000_000000000046
);
windows_core::imp::interface_hierarchy!(IDispatch, windows_core::IUnknown);
impl IDispatch {
    pub unsafe fn GetTypeInfoCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTypeInfoCount)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub unsafe fn GetIDsOfNames(
        &self,
        riid: *const windows_core::GUID,
        rgsznames: *const windows_core::PCWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> windows_core::Result<()> {
        unsafe {
            (windows_core::Interface::vtable(self).GetIDsOfNames)(
                windows_core::Interface::as_raw(self),
                riid,
                rgsznames,
                cnames,
                lcid,
                rgdispid as _,
            )
            .ok()
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispatch_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTypeInfoCount:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    GetTypeInfo: usize,
    pub GetIDsOfNames: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const windows_core::GUID,
        *const windows_core::PCWSTR,
        u32,
        u32,
        *mut i32,
    ) -> windows_core::HRESULT,
    Invoke: usize,
}
pub trait IDispatch_Impl: windows_core::IUnknownImpl {
    fn GetTypeInfoCount(&self) -> windows_core::Result<u32>;
    fn GetIDsOfNames(
        &self,
        riid: *const windows_core::GUID,
        rgsznames: *const windows_core::PCWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> windows_core::Result<()>;
}
impl IDispatch_Vtbl {
    pub const fn new<Identity: IDispatch_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTypeInfoCount<
            Identity: IDispatch_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            pctinfo: *mut u32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatch_Impl::GetTypeInfoCount(this) {
                    Ok(ok__) => {
                        pctinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: IDispatch_Impl, const OFFSET: isize>(
            this: *mut core::ffi::c_void,
            riid: *const windows_core::GUID,
            rgsznames: *const windows_core::PCWSTR,
            cnames: u32,
            lcid: u32,
            rgdispid: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatch_Impl::GetIDsOfNames(
                    this,
                    core::mem::transmute_copy(&riid),
                    core::mem::transmute_copy(&rgsznames),
                    core::mem::transmute_copy(&cnames),
                    core::mem::transmute_copy(&lcid),
                    core::mem::transmute_copy(&rgdispid),
                )
                .into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, OFFSET>,
            GetTypeInfo: 0,
            GetIDsOfNames: GetIDsOfNames::<Identity, OFFSET>,
            Invoke: 0,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispatch as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDispatch {}
