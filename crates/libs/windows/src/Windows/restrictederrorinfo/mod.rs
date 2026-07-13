windows_core::imp::define_interface!(ILanguageExceptionErrorInfo, ILanguageExceptionErrorInfo_Vtbl, 0x04a2dbf3_df83_116c_0946_0812abf6e07d);
windows_core::imp::interface_hierarchy!(ILanguageExceptionErrorInfo, windows_core::IUnknown);
impl ILanguageExceptionErrorInfo {
    pub unsafe fn GetLanguageException(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguageException)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLanguageException: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILanguageExceptionErrorInfo_Impl: windows_core::IUnknownImpl {
    fn GetLanguageException(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl ILanguageExceptionErrorInfo_Vtbl {
    pub const fn new<Identity: ILanguageExceptionErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLanguageException<Identity: ILanguageExceptionErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageexception: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILanguageExceptionErrorInfo_Impl::GetLanguageException(this) {
                    Ok(ok__) => {
                        languageexception.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetLanguageException: GetLanguageException::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILanguageExceptionErrorInfo {}
windows_core::imp::define_interface!(ILanguageExceptionErrorInfo2, ILanguageExceptionErrorInfo2_Vtbl, 0x5746e5c4_5b97_424c_b620_2822915734dd);
impl core::ops::Deref for ILanguageExceptionErrorInfo2 {
    type Target = ILanguageExceptionErrorInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ILanguageExceptionErrorInfo2, windows_core::IUnknown, ILanguageExceptionErrorInfo);
impl ILanguageExceptionErrorInfo2 {
    pub unsafe fn GetPreviousLanguageExceptionErrorInfo(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousLanguageExceptionErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CapturePropagationContext<P0>(&self, languageexception: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CapturePropagationContext)(windows_core::Interface::as_raw(self), languageexception.param().abi()) }
    }
    pub unsafe fn GetPropagationContextHead(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropagationContextHead)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo2_Vtbl {
    pub base__: ILanguageExceptionErrorInfo_Vtbl,
    pub GetPreviousLanguageExceptionErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CapturePropagationContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropagationContextHead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILanguageExceptionErrorInfo2_Impl: ILanguageExceptionErrorInfo_Impl {
    fn GetPreviousLanguageExceptionErrorInfo(&self) -> windows_core::Result<ILanguageExceptionErrorInfo2>;
    fn CapturePropagationContext(&self, languageexception: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetPropagationContextHead(&self) -> windows_core::Result<ILanguageExceptionErrorInfo2>;
}
impl ILanguageExceptionErrorInfo2_Vtbl {
    pub const fn new<Identity: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPreviousLanguageExceptionErrorInfo<Identity: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, previouslanguageexceptionerrorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILanguageExceptionErrorInfo2_Impl::GetPreviousLanguageExceptionErrorInfo(this) {
                    Ok(ok__) => {
                        previouslanguageexceptionerrorinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CapturePropagationContext<Identity: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, languageexception: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILanguageExceptionErrorInfo2_Impl::CapturePropagationContext(this, core::mem::transmute_copy(&languageexception)).into()
            }
        }
        unsafe extern "system" fn GetPropagationContextHead<Identity: ILanguageExceptionErrorInfo2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propagatedlanguageexceptionerrorinfohead: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILanguageExceptionErrorInfo2_Impl::GetPropagationContextHead(this) {
                    Ok(ok__) => {
                        propagatedlanguageexceptionerrorinfohead.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ILanguageExceptionErrorInfo_Vtbl::new::<Identity, OFFSET>(),
            GetPreviousLanguageExceptionErrorInfo: GetPreviousLanguageExceptionErrorInfo::<Identity, OFFSET>,
            CapturePropagationContext: CapturePropagationContext::<Identity, OFFSET>,
            GetPropagationContextHead: GetPropagationContextHead::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionErrorInfo2 as windows_core::Interface>::IID || iid == &<ILanguageExceptionErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILanguageExceptionErrorInfo2 {}
windows_core::imp::define_interface!(ILanguageExceptionStackBackTrace, ILanguageExceptionStackBackTrace_Vtbl, 0xcbe53fb5_f967_4258_8d34_42f5e25833de);
windows_core::imp::interface_hierarchy!(ILanguageExceptionStackBackTrace, windows_core::IUnknown);
impl ILanguageExceptionStackBackTrace {
    pub unsafe fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStackBackTrace)(windows_core::Interface::as_raw(self), maxframestocapture, stackbacktrace as _, framescaptured as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionStackBackTrace_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStackBackTrace: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut usize, *mut u32) -> windows_core::HRESULT,
}
pub trait ILanguageExceptionStackBackTrace_Impl: windows_core::IUnknownImpl {
    fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> windows_core::Result<()>;
}
impl ILanguageExceptionStackBackTrace_Vtbl {
    pub const fn new<Identity: ILanguageExceptionStackBackTrace_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStackBackTrace<Identity: ILanguageExceptionStackBackTrace_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILanguageExceptionStackBackTrace_Impl::GetStackBackTrace(this, core::mem::transmute_copy(&maxframestocapture), core::mem::transmute_copy(&stackbacktrace), core::mem::transmute_copy(&framescaptured)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetStackBackTrace: GetStackBackTrace::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionStackBackTrace as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILanguageExceptionStackBackTrace {}
windows_core::imp::define_interface!(ILanguageExceptionTransform, ILanguageExceptionTransform_Vtbl, 0xfeb5a271_a6cd_45ce_880a_696706badc65);
windows_core::imp::interface_hierarchy!(ILanguageExceptionTransform, windows_core::IUnknown);
impl ILanguageExceptionTransform {
    pub unsafe fn GetTransformedRestrictedErrorInfo(&self) -> windows_core::Result<IRestrictedErrorInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformedRestrictedErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTransformedRestrictedErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ILanguageExceptionTransform_Impl: windows_core::IUnknownImpl {
    fn GetTransformedRestrictedErrorInfo(&self) -> windows_core::Result<IRestrictedErrorInfo>;
}
impl ILanguageExceptionTransform_Vtbl {
    pub const fn new<Identity: ILanguageExceptionTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTransformedRestrictedErrorInfo<Identity: ILanguageExceptionTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, restrictederrorinfo: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILanguageExceptionTransform_Impl::GetTransformedRestrictedErrorInfo(this) {
                    Ok(ok__) => {
                        restrictederrorinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTransformedRestrictedErrorInfo: GetTransformedRestrictedErrorInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILanguageExceptionTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILanguageExceptionTransform {}
windows_core::imp::define_interface!(IRestrictedErrorInfo, IRestrictedErrorInfo_Vtbl, 0x82ba7092_4c88_427d_a7bc_16dd93feb67e);
windows_core::imp::interface_hierarchy!(IRestrictedErrorInfo, windows_core::IUnknown);
impl IRestrictedErrorInfo {
    pub unsafe fn GetErrorDetails(&self, description: *mut windows_core::BSTR, error: *mut windows_core::HRESULT, restricteddescription: *mut windows_core::BSTR, capabilitysid: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetErrorDetails)(windows_core::Interface::as_raw(self), core::mem::transmute(description), error as _, core::mem::transmute(restricteddescription), core::mem::transmute(capabilitysid)) }
    }
    pub unsafe fn GetReference(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReference)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedErrorInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetErrorDetails: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetReference: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRestrictedErrorInfo_Impl: windows_core::IUnknownImpl {
    fn GetErrorDetails(&self, description: *mut windows_core::BSTR, error: *mut windows_core::HRESULT, restricteddescription: *mut windows_core::BSTR, capabilitysid: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetReference(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IRestrictedErrorInfo_Vtbl {
    pub const fn new<Identity: IRestrictedErrorInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetErrorDetails<Identity: IRestrictedErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, description: *mut *mut core::ffi::c_void, error: *mut windows_core::HRESULT, restricteddescription: *mut *mut core::ffi::c_void, capabilitysid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRestrictedErrorInfo_Impl::GetErrorDetails(this, core::mem::transmute_copy(&description), core::mem::transmute_copy(&error), core::mem::transmute_copy(&restricteddescription), core::mem::transmute_copy(&capabilitysid)).into()
            }
        }
        unsafe extern "system" fn GetReference<Identity: IRestrictedErrorInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, reference: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRestrictedErrorInfo_Impl::GetReference(this) {
                    Ok(ok__) => {
                        reference.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorDetails: GetErrorDetails::<Identity, OFFSET>,
            GetReference: GetReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRestrictedErrorInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRestrictedErrorInfo {}
