pub const BackgroundCopyManager10_3: windows_core::GUID = windows_core::GUID::from_u128(0x5fd42ad5_c04e_4d36_adc7_e08ff15737ad);
#[cfg(all(feature = "Win32_bits10_2", feature = "Win32_bits2_5"))]
windows_core::imp::define_interface!(IBackgroundCopyJobHttpOptions3, IBackgroundCopyJobHttpOptions3_Vtbl, 0x8a9263d3_fd4c_4eda_9b28_30132a4d4e3c);
#[cfg(all(feature = "Win32_bits10_2", feature = "Win32_bits2_5"))]
impl core::ops::Deref for IBackgroundCopyJobHttpOptions3 {
    type Target = super::bits10_2::IBackgroundCopyJobHttpOptions2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(all(feature = "Win32_bits10_2", feature = "Win32_bits2_5"))]
windows_core::imp::interface_hierarchy!(IBackgroundCopyJobHttpOptions3, windows_core::IUnknown, super::bits2_5::IBackgroundCopyJobHttpOptions, super::bits10_2::IBackgroundCopyJobHttpOptions2);
#[cfg(all(feature = "Win32_bits10_2", feature = "Win32_bits2_5"))]
impl IBackgroundCopyJobHttpOptions3 {
    pub unsafe fn SetServerCertificateValidationInterface<P0>(&self, certvalidationcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServerCertificateValidationInterface)(windows_core::Interface::as_raw(self), certvalidationcallback.param().abi()) }
    }
    pub unsafe fn MakeCustomHeadersWriteOnly(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MakeCustomHeadersWriteOnly)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(all(feature = "Win32_bits10_2", feature = "Win32_bits2_5"))]
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyJobHttpOptions3_Vtbl {
    pub base__: super::bits10_2::IBackgroundCopyJobHttpOptions2_Vtbl,
    pub SetServerCertificateValidationInterface: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MakeCustomHeadersWriteOnly: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_bits10_2", feature = "Win32_bits2_5", feature = "Win32_rpcndr"))]
pub trait IBackgroundCopyJobHttpOptions3_Impl: super::bits10_2::IBackgroundCopyJobHttpOptions2_Impl {
    fn SetServerCertificateValidationInterface(&self, certvalidationcallback: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn MakeCustomHeadersWriteOnly(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_bits10_2", feature = "Win32_bits2_5", feature = "Win32_rpcndr"))]
impl IBackgroundCopyJobHttpOptions3_Vtbl {
    pub const fn new<Identity: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetServerCertificateValidationInterface<Identity: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, certvalidationcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions3_Impl::SetServerCertificateValidationInterface(this, core::mem::transmute_copy(&certvalidationcallback)).into()
            }
        }
        unsafe extern "system" fn MakeCustomHeadersWriteOnly<Identity: IBackgroundCopyJobHttpOptions3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyJobHttpOptions3_Impl::MakeCustomHeadersWriteOnly(this).into()
            }
        }
        Self {
            base__: super::bits10_2::IBackgroundCopyJobHttpOptions2_Vtbl::new::<Identity, OFFSET>(),
            SetServerCertificateValidationInterface: SetServerCertificateValidationInterface::<Identity, OFFSET>,
            MakeCustomHeadersWriteOnly: MakeCustomHeadersWriteOnly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyJobHttpOptions3 as windows_core::Interface>::IID || iid == &<super::bits2_5::IBackgroundCopyJobHttpOptions as windows_core::Interface>::IID || iid == &<super::bits10_2::IBackgroundCopyJobHttpOptions2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_bits10_2", feature = "Win32_bits2_5", feature = "Win32_rpcndr"))]
impl windows_core::RuntimeName for IBackgroundCopyJobHttpOptions3 {}
windows_core::imp::define_interface!(IBackgroundCopyServerCertificateValidationCallback, IBackgroundCopyServerCertificateValidationCallback_Vtbl, 0x4cec0d02_def7_4158_813a_c32a46945ff7);
windows_core::imp::interface_hierarchy!(IBackgroundCopyServerCertificateValidationCallback, windows_core::IUnknown);
impl IBackgroundCopyServerCertificateValidationCallback {
    #[cfg(feature = "Win32_bits")]
    pub unsafe fn ValidateServerCertificate<P0, P1>(&self, job: P0, file: P1, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::bits::IBackgroundCopyJob>,
        P1: windows_core::Param<super::bits::IBackgroundCopyFile>,
    {
        unsafe { (windows_core::Interface::vtable(self).ValidateServerCertificate)(windows_core::Interface::as_raw(self), job.param().abi(), file.param().abi(), certlength, certdata, certencodingtype, certstorelength, certstoredata) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBackgroundCopyServerCertificateValidationCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_bits")]
    pub ValidateServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, u32, u32, *const u8) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_bits"))]
    ValidateServerCertificate: usize,
}
#[cfg(feature = "Win32_bits")]
pub trait IBackgroundCopyServerCertificateValidationCallback_Impl: windows_core::IUnknownImpl {
    fn ValidateServerCertificate(&self, job: windows_core::Ref<super::bits::IBackgroundCopyJob>, file: windows_core::Ref<super::bits::IBackgroundCopyFile>, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_bits")]
impl IBackgroundCopyServerCertificateValidationCallback_Vtbl {
    pub const fn new<Identity: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ValidateServerCertificate<Identity: IBackgroundCopyServerCertificateValidationCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, job: *mut core::ffi::c_void, file: *mut core::ffi::c_void, certlength: u32, certdata: *const u8, certencodingtype: u32, certstorelength: u32, certstoredata: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBackgroundCopyServerCertificateValidationCallback_Impl::ValidateServerCertificate(this, core::mem::transmute_copy(&job), core::mem::transmute_copy(&file), core::mem::transmute_copy(&certlength), core::mem::transmute_copy(&certdata), core::mem::transmute_copy(&certencodingtype), core::mem::transmute_copy(&certstorelength), core::mem::transmute_copy(&certstoredata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ValidateServerCertificate: ValidateServerCertificate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBackgroundCopyServerCertificateValidationCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_bits")]
impl windows_core::RuntimeName for IBackgroundCopyServerCertificateValidationCallback {}
