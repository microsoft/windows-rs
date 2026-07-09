pub const BackgroundCopyManager10_2: windows_core::GUID = windows_core::GUID::from_u128(0x4575438f_a6c8_4976_b0fe_2f26b80d959e);
#[cfg(feature = "Win32_bits2_5")]
windows_core::imp::define_interface!(IBackgroundCopyJobHttpOptions2, IBackgroundCopyJobHttpOptions2_Vtbl, 0xb591a192_a405_4fc3_8323_4c5c542578fc);
#[cfg(feature = "Win32_bits2_5")]
impl core::ops::Deref for IBackgroundCopyJobHttpOptions2 {
    type Target = super::bits2_5::IBackgroundCopyJobHttpOptions;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_bits2_5")]
windows_core::imp::interface_hierarchy!(IBackgroundCopyJobHttpOptions2, windows_core::IUnknown, super::bits2_5::IBackgroundCopyJobHttpOptions);
#[cfg(feature = "Win32_bits2_5")]
impl IBackgroundCopyJobHttpOptions2 {
    pub unsafe fn SetHttpMethod<P0>(&self, method: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetHttpMethod)(windows_core::Interface::as_raw(self), method.param().abi()) }
    }
    pub unsafe fn GetHttpMethod(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHttpMethod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "Win32_bits2_5")]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions2_Vtbl {
    pub base__: super::bits2_5::IBackgroundCopyJobHttpOptions_Vtbl,
    pub SetHttpMethod: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetHttpMethod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits2_5", feature = "Win32_rpcndr"))]
pub trait IBackgroundCopyJobHttpOptions2_Impl: super::bits2_5::IBackgroundCopyJobHttpOptions_Impl {
    fn SetHttpMethod(&self, method: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetHttpMethod(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "Win32_bits2_5", feature = "Win32_rpcndr"))]
impl IBackgroundCopyJobHttpOptions2_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetHttpMethod<Identity: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions2_Impl::SetHttpMethod(this, core::mem::transmute(&method)).into()
            }
        }
        unsafe extern "system" fn GetHttpMethod<Identity: IBackgroundCopyJobHttpOptions2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBackgroundCopyJobHttpOptions2_Impl::GetHttpMethod(this) {
                    Ok(ok__) => {
                        method.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::bits2_5::IBackgroundCopyJobHttpOptions_Vtbl::new::<Identity, OFFSET>(),
            SetHttpMethod: SetHttpMethod::<Identity, OFFSET>,
            GetHttpMethod: GetHttpMethod::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions2 as windows_core::Interface>::IID || iid == &<super::bits2_5::IBackgroundCopyJobHttpOptions as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits2_5", feature = "Win32_rpcndr"))]
impl windows_core::RuntimeName for IBackgroundCopyJobHttpOptions2 {}
