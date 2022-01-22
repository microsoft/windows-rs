pub trait IAMWMBufferPass_Impl: Sized {
    fn SetNotify(&mut self, pcallback: &::core::option::Option<IAMWMBufferPassCallback>) -> ::windows::core::Result<()>;
}
impl IAMWMBufferPass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAMWMBufferPass_Impl, const OFFSET: isize>() -> IAMWMBufferPass_Vtbl {
        unsafe extern "system" fn SetNotify<Identity: ::windows::core::IUnknownImpl, Impl: IAMWMBufferPass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNotify(::core::mem::transmute(&pcallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetNotify: SetNotify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAMWMBufferPass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IAMWMBufferPassCallback_Impl: Sized {
    fn Notify(&mut self, pnssbuffer3: &::core::option::Option<INSSBuffer3>, ppin: &::core::option::Option<super::DirectShow::IPin>, prtstart: *const i64, prtend: *const i64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IAMWMBufferPassCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAMWMBufferPassCallback_Impl, const OFFSET: isize>() -> IAMWMBufferPassCallback_Vtbl {
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IAMWMBufferPassCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnssbuffer3: ::windows::core::RawPtr, ppin: ::windows::core::RawPtr, prtstart: *const i64, prtend: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute(&pnssbuffer3), ::core::mem::transmute(&ppin), ::core::mem::transmute_copy(&prtstart), ::core::mem::transmute_copy(&prtend)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Notify: Notify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAMWMBufferPassCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INSNetSourceCreator_Impl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn CreateNetSource(&mut self, pszstreamname: super::super::Foundation::PWSTR, pmonitor: &::core::option::Option<::windows::core::IUnknown>, pdata: *const u8, pusercontext: &::core::option::Option<::windows::core::IUnknown>, pcallback: &::core::option::Option<::windows::core::IUnknown>, qwcontext: u64) -> ::windows::core::Result<()>;
    fn GetNetSourceProperties(&mut self, pszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetNetSourceSharedNamespace(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetNetSourceAdminInterface(&mut self, pszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetNumProtocolsSupported(&mut self) -> ::windows::core::Result<u32>;
    fn GetProtocolName(&mut self, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INSNetSourceCreator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>() -> INSNetSourceCreator_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn CreateNetSource<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateNetSource(::core::mem::transmute_copy(&pszstreamname), ::core::mem::transmute(&pmonitor), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute(&pusercontext), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&qwcontext)).into()
        }
        unsafe extern "system" fn GetNetSourceProperties<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetSourceProperties(::core::mem::transmute_copy(&pszstreamname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertiesnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceSharedNamespace<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetSourceSharedNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsharednamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceAdminInterface<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetSourceAdminInterface(::core::mem::transmute_copy(&pszstreamname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumProtocolsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pcprotocols = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocolName<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProtocolName(::core::mem::transmute_copy(&dwprotocolnum), ::core::mem::transmute_copy(&pwszprotocolname), ::core::mem::transmute_copy(&pcchprotocolname)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Shutdown().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            CreateNetSource: CreateNetSource::<Identity, Impl, OFFSET>,
            GetNetSourceProperties: GetNetSourceProperties::<Identity, Impl, OFFSET>,
            GetNetSourceSharedNamespace: GetNetSourceSharedNamespace::<Identity, Impl, OFFSET>,
            GetNetSourceAdminInterface: GetNetSourceAdminInterface::<Identity, Impl, OFFSET>,
            GetNumProtocolsSupported: GetNumProtocolsSupported::<Identity, Impl, OFFSET>,
            GetProtocolName: GetProtocolName::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSNetSourceCreator as ::windows::core::Interface>::IID
    }
}
pub trait INSSBuffer_Impl: Sized {
    fn GetLength(&mut self) -> ::windows::core::Result<u32>;
    fn SetLength(&mut self, dwlength: u32) -> ::windows::core::Result<()>;
    fn GetMaxLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetBuffer(&mut self) -> ::windows::core::Result<*mut u8>;
    fn GetBufferAndLength(&mut self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()>;
}
impl INSSBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer_Impl, const OFFSET: isize>() -> INSSBuffer_Vtbl {
        unsafe extern "system" fn GetLength<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLength(::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetMaxLength<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdwbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferAndLength(::core::mem::transmute_copy(&ppdwbuffer), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer as ::windows::core::Interface>::IID
    }
}
pub trait INSSBuffer2_Impl: Sized + INSSBuffer_Impl {
    fn GetSampleProperties(&mut self, cbproperties: u32) -> ::windows::core::Result<u8>;
    fn SetSampleProperties(&mut self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()>;
}
impl INSSBuffer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer2_Impl, const OFFSET: isize>() -> INSSBuffer2_Vtbl {
        unsafe extern "system" fn GetSampleProperties<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSampleProperties(::core::mem::transmute_copy(&cbproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleProperties<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSampleProperties(::core::mem::transmute_copy(&cbproperties), ::core::mem::transmute_copy(&pbproperties)).into()
        }
        Self {
            base: INSSBuffer_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSampleProperties: GetSampleProperties::<Identity, Impl, OFFSET>,
            SetSampleProperties: SetSampleProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer2 as ::windows::core::Interface>::IID || iid == &<INSSBuffer as ::windows::core::Interface>::IID
    }
}
pub trait INSSBuffer3_Impl: Sized + INSSBuffer_Impl + INSSBuffer2_Impl {
    fn SetProperty(&mut self, guidbufferproperty: &::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, guidbufferproperty: &::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()>;
}
impl INSSBuffer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer3_Impl, const OFFSET: isize>() -> INSSBuffer3_Vtbl {
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&guidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&dwbufferpropertysize)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&guidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&pdwbufferpropertysize)).into()
        }
        Self {
            base: INSSBuffer2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer3 as ::windows::core::Interface>::IID || iid == &<INSSBuffer as ::windows::core::Interface>::IID || iid == &<INSSBuffer2 as ::windows::core::Interface>::IID
    }
}
pub trait INSSBuffer4_Impl: Sized + INSSBuffer_Impl + INSSBuffer2_Impl + INSSBuffer3_Impl {
    fn GetPropertyCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetPropertyByIndex(&mut self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()>;
}
impl INSSBuffer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer4_Impl, const OFFSET: isize>() -> INSSBuffer4_Vtbl {
        unsafe extern "system" fn GetPropertyCount<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertyCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbufferproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyByIndex(::core::mem::transmute_copy(&dwbufferpropertyindex), ::core::mem::transmute_copy(&pguidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&pdwbufferpropertysize)).into()
        }
        Self {
            base: INSSBuffer3_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, Impl, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer4 as ::windows::core::Interface>::IID || iid == &<INSSBuffer as ::windows::core::Interface>::IID || iid == &<INSSBuffer2 as ::windows::core::Interface>::IID || iid == &<INSSBuffer3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMAddressAccess_Impl: Sized {
    fn GetAccessEntryCount(&mut self, aetype: WM_AETYPE) -> ::windows::core::Result<u32>;
    fn GetAccessEntry(&mut self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY>;
    fn AddAccessEntry(&mut self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()>;
    fn RemoveAccessEntry(&mut self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()>;
}
impl IWMAddressAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess_Impl, const OFFSET: isize>() -> IWMAddressAccess_Vtbl {
        unsafe extern "system" fn GetAccessEntryCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAccessEntryCount(::core::mem::transmute_copy(&aetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcentries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessEntry<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAccessEntry(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum)) {
                ::core::result::Result::Ok(ok__) => {
                    *paddraccessentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccessEntry<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAccessEntry(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&paddraccessentry)).into()
        }
        unsafe extern "system" fn RemoveAccessEntry<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAccessEntry(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAccessEntryCount: GetAccessEntryCount::<Identity, Impl, OFFSET>,
            GetAccessEntry: GetAccessEntry::<Identity, Impl, OFFSET>,
            AddAccessEntry: AddAccessEntry::<Identity, Impl, OFFSET>,
            RemoveAccessEntry: RemoveAccessEntry::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMAddressAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMAddressAccess2_Impl: Sized + IWMAddressAccess_Impl {
    fn GetAccessEntryEx(&mut self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddAccessEntryEx(&mut self, aetype: WM_AETYPE, bstraddress: &super::super::Foundation::BSTR, bstrmask: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMAddressAccess2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess2_Impl, const OFFSET: isize>() -> IWMAddressAccess2_Vtbl {
        unsafe extern "system" fn GetAccessEntryEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAccessEntryEx(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum), ::core::mem::transmute_copy(&pbstraddress), ::core::mem::transmute_copy(&pbstrmask)).into()
        }
        unsafe extern "system" fn AddAccessEntryEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAccessEntryEx(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&bstraddress), ::core::mem::transmute_copy(&bstrmask)).into()
        }
        Self {
            base: IWMAddressAccess_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAccessEntryEx: GetAccessEntryEx::<Identity, Impl, OFFSET>,
            AddAccessEntryEx: AddAccessEntryEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMAddressAccess2 as ::windows::core::Interface>::IID || iid == &<IWMAddressAccess as ::windows::core::Interface>::IID
    }
}
pub trait IWMAuthorizer_Impl: Sized {
    fn GetCertCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetCert(&mut self, dwindex: u32) -> ::windows::core::Result<*mut u8>;
    fn GetSharedData(&mut self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8>;
}
impl IWMAuthorizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMAuthorizer_Impl, const OFFSET: isize>() -> IWMAuthorizer_Vtbl {
        unsafe extern "system" fn GetCertCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMAuthorizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCertCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccerts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCert<Identity: ::windows::core::IUnknownImpl, Impl: IWMAuthorizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCert(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbcertdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedData<Identity: ::windows::core::IUnknownImpl, Impl: IWMAuthorizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSharedData(::core::mem::transmute_copy(&dwcertindex), ::core::mem::transmute_copy(&pbshareddata), ::core::mem::transmute_copy(&pbcert)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbshareddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCertCount: GetCertCount::<Identity, Impl, OFFSET>,
            GetCert: GetCert::<Identity, Impl, OFFSET>,
            GetSharedData: GetSharedData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMAuthorizer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMBackupRestoreProps_Impl: Sized {
    fn GetPropCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetPropByIndex(&mut self, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn GetPropByName(&mut self, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetProp(&mut self, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn RemoveProp(&mut self, pcwszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RemoveAllProps(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMBackupRestoreProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>() -> IWMBackupRestoreProps_Vtbl {
        unsafe extern "system" fn GetPropCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropByIndex(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetPropByName<Identity: ::windows::core::IUnknownImpl, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropByName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetProp<Identity: ::windows::core::IUnknownImpl, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProp(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn RemoveProp<Identity: ::windows::core::IUnknownImpl, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveProp(::core::mem::transmute_copy(&pcwszname)).into()
        }
        unsafe extern "system" fn RemoveAllProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAllProps().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPropCount: GetPropCount::<Identity, Impl, OFFSET>,
            GetPropByIndex: GetPropByIndex::<Identity, Impl, OFFSET>,
            GetPropByName: GetPropByName::<Identity, Impl, OFFSET>,
            SetProp: SetProp::<Identity, Impl, OFFSET>,
            RemoveProp: RemoveProp::<Identity, Impl, OFFSET>,
            RemoveAllProps: RemoveAllProps::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMBackupRestoreProps as ::windows::core::Interface>::IID
    }
}
pub trait IWMBandwidthSharing_Impl: Sized + IWMStreamList_Impl {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetType(&mut self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetBandwidth(&mut self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::Result<()>;
    fn SetBandwidth(&mut self, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::Result<()>;
}
impl IWMBandwidthSharing_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>() -> IWMBandwidthSharing_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&guidtype)).into()
        }
        unsafe extern "system" fn GetBandwidth<Identity: ::windows::core::IUnknownImpl, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBandwidth(::core::mem::transmute_copy(&pdwbitrate), ::core::mem::transmute_copy(&pmsbufferwindow)).into()
        }
        unsafe extern "system" fn SetBandwidth<Identity: ::windows::core::IUnknownImpl, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBandwidth(::core::mem::transmute_copy(&dwbitrate), ::core::mem::transmute_copy(&msbufferwindow)).into()
        }
        Self {
            base: IWMStreamList_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            GetBandwidth: GetBandwidth::<Identity, Impl, OFFSET>,
            SetBandwidth: SetBandwidth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMBandwidthSharing as ::windows::core::Interface>::IID || iid == &<IWMStreamList as ::windows::core::Interface>::IID
    }
}
pub trait IWMClientConnections_Impl: Sized {
    fn GetClientCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetClientProperties(&mut self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES>;
}
impl IWMClientConnections_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMClientConnections_Impl, const OFFSET: isize>() -> IWMClientConnections_Vtbl {
        unsafe extern "system" fn GetClientCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMClientConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClientCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientProperties<Identity: ::windows::core::IUnknownImpl, Impl: IWMClientConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClientProperties(::core::mem::transmute_copy(&dwclientnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pclientproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClientCount: GetClientCount::<Identity, Impl, OFFSET>,
            GetClientProperties: GetClientProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMClientConnections as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMClientConnections2_Impl: Sized + IWMClientConnections_Impl {
    fn GetClientInfo(&mut self, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMClientConnections2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMClientConnections2_Impl, const OFFSET: isize>() -> IWMClientConnections2_Vtbl {
        unsafe extern "system" fn GetClientInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMClientConnections2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetClientInfo(::core::mem::transmute_copy(&dwclientnum), ::core::mem::transmute_copy(&pwsznetworkaddress), ::core::mem::transmute_copy(&pcchnetworkaddress), ::core::mem::transmute_copy(&pwszport), ::core::mem::transmute_copy(&pcchport), ::core::mem::transmute_copy(&pwszdnsname), ::core::mem::transmute_copy(&pcchdnsname)).into()
        }
        Self { base: IWMClientConnections_Vtbl::new::<Identity, Impl, OFFSET>(), GetClientInfo: GetClientInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMClientConnections2 as ::windows::core::Interface>::IID || iid == &<IWMClientConnections as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait IWMCodecAMVideoAccelerator_Impl: Sized {
    fn SetAcceleratorInterface(&mut self, piamva: &::core::option::Option<super::DirectShow::IAMVideoAccelerator>) -> ::windows::core::Result<()>;
    fn NegotiateConnection(&mut self, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
    fn SetPlayerNotify(&mut self, phook: &::core::option::Option<IWMPlayerTimestampHook>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl IWMCodecAMVideoAccelerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecAMVideoAccelerator_Impl, const OFFSET: isize>() -> IWMCodecAMVideoAccelerator_Vtbl {
        unsafe extern "system" fn SetAcceleratorInterface<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecAMVideoAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAcceleratorInterface(::core::mem::transmute(&piamva)).into()
        }
        unsafe extern "system" fn NegotiateConnection<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecAMVideoAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NegotiateConnection(::core::mem::transmute_copy(&pmediatype)).into()
        }
        unsafe extern "system" fn SetPlayerNotify<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecAMVideoAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlayerNotify(::core::mem::transmute(&phook)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAcceleratorInterface: SetAcceleratorInterface::<Identity, Impl, OFFSET>,
            NegotiateConnection: NegotiateConnection::<Identity, Impl, OFFSET>,
            SetPlayerNotify: SetPlayerNotify::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecAMVideoAccelerator as ::windows::core::Interface>::IID
    }
}
pub trait IWMCodecInfo_Impl: Sized {
    fn GetCodecInfoCount(&mut self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn GetCodecFormatCount(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32>;
    fn GetCodecFormat(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig>;
}
impl IWMCodecInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo_Impl, const OFFSET: isize>() -> IWMCodecInfo_Vtbl {
        unsafe extern "system" fn GetCodecInfoCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCodecInfoCount(::core::mem::transmute_copy(&guidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pccodecs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormatCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCodecFormatCount(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCodecFormat(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppistreamconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Identity, Impl, OFFSET>,
            GetCodecFormatCount: GetCodecFormatCount::<Identity, Impl, OFFSET>,
            GetCodecFormat: GetCodecFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMCodecInfo2_Impl: Sized + IWMCodecInfo_Impl {
    fn GetCodecName(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()>;
    fn GetCodecFormatDesc(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMCodecInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo2_Impl, const OFFSET: isize>() -> IWMCodecInfo2_Vtbl {
        unsafe extern "system" fn GetCodecName<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCodecName(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetCodecFormatDesc<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCodecFormatDesc(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex), ::core::mem::transmute_copy(&ppistreamconfig), ::core::mem::transmute_copy(&wszdesc), ::core::mem::transmute_copy(&pcchdesc)).into()
        }
        Self {
            base: IWMCodecInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCodecName: GetCodecName::<Identity, Impl, OFFSET>,
            GetCodecFormatDesc: GetCodecFormatDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecInfo2 as ::windows::core::Interface>::IID || iid == &<IWMCodecInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMCodecInfo3_Impl: Sized + IWMCodecInfo_Impl + IWMCodecInfo2_Impl {
    fn GetCodecFormatProp(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetCodecProp(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn SetCodecEnumerationSetting(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn GetCodecEnumerationSetting(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMCodecInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>() -> IWMCodecInfo3_Vtbl {
        unsafe extern "system" fn GetCodecFormatProp<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCodecFormatProp(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn GetCodecProp<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCodecProp(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn SetCodecEnumerationSetting<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCodecEnumerationSetting(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetCodecEnumerationSetting<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCodecEnumerationSetting(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self {
            base: IWMCodecInfo2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCodecFormatProp: GetCodecFormatProp::<Identity, Impl, OFFSET>,
            GetCodecProp: GetCodecProp::<Identity, Impl, OFFSET>,
            SetCodecEnumerationSetting: SetCodecEnumerationSetting::<Identity, Impl, OFFSET>,
            GetCodecEnumerationSetting: GetCodecEnumerationSetting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecInfo3 as ::windows::core::Interface>::IID || iid == &<IWMCodecInfo as ::windows::core::Interface>::IID || iid == &<IWMCodecInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait IWMCodecVideoAccelerator_Impl: Sized {
    fn NegotiateConnection(&mut self, piamva: &::core::option::Option<super::DirectShow::IAMVideoAccelerator>, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
    fn SetPlayerNotify(&mut self, phook: &::core::option::Option<IWMPlayerTimestampHook>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl IWMCodecVideoAccelerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecVideoAccelerator_Impl, const OFFSET: isize>() -> IWMCodecVideoAccelerator_Vtbl {
        unsafe extern "system" fn NegotiateConnection<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecVideoAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NegotiateConnection(::core::mem::transmute(&piamva), ::core::mem::transmute_copy(&pmediatype)).into()
        }
        unsafe extern "system" fn SetPlayerNotify<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecVideoAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlayerNotify(::core::mem::transmute(&phook)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            NegotiateConnection: NegotiateConnection::<Identity, Impl, OFFSET>,
            SetPlayerNotify: SetPlayerNotify::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecVideoAccelerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMCredentialCallback_Impl: Sized {
    fn AcquireCredentials(&mut self, pwszrealm: super::super::Foundation::PWSTR, pwszsite: super::super::Foundation::PWSTR, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMCredentialCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCredentialCallback_Impl, const OFFSET: isize>() -> IWMCredentialCallback_Vtbl {
        unsafe extern "system" fn AcquireCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IWMCredentialCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrealm: super::super::Foundation::PWSTR, pwszsite: super::super::Foundation::PWSTR, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireCredentials(::core::mem::transmute_copy(&pwszrealm), ::core::mem::transmute_copy(&pwszsite), ::core::mem::transmute_copy(&pwszuser), ::core::mem::transmute_copy(&cchuser), ::core::mem::transmute_copy(&pwszpassword), ::core::mem::transmute_copy(&cchpassword), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AcquireCredentials: AcquireCredentials::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCredentialCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMEditor_Impl: Sized {
    fn GetDRMProperty(&mut self, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMEditor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMEditor_Impl, const OFFSET: isize>() -> IWMDRMEditor_Vtbl {
        unsafe extern "system" fn GetDRMProperty<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDRMProperty(::core::mem::transmute_copy(&pwstrname), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDRMProperty: GetDRMProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMEditor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMMessageParser_Impl: Sized {
    fn ParseRegistrationReqMsg(&mut self, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::Result<()>;
    fn ParseLicenseRequestMsg(&mut self, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMMessageParser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMMessageParser_Impl, const OFFSET: isize>() -> IWMDRMMessageParser_Vtbl {
        unsafe extern "system" fn ParseRegistrationReqMsg<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMMessageParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ParseRegistrationReqMsg(::core::mem::transmute_copy(&pbregistrationreqmsg), ::core::mem::transmute_copy(&cbregistrationreqmsg), ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber)).into()
        }
        unsafe extern "system" fn ParseLicenseRequestMsg<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMMessageParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ParseLicenseRequestMsg(::core::mem::transmute_copy(&pblicenserequestmsg), ::core::mem::transmute_copy(&cblicenserequestmsg), ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber), ::core::mem::transmute_copy(&pbstraction)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ParseRegistrationReqMsg: ParseRegistrationReqMsg::<Identity, Impl, OFFSET>,
            ParseLicenseRequestMsg: ParseLicenseRequestMsg::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMMessageParser as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReader_Impl: Sized {
    fn AcquireLicense(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn CancelLicenseAcquisition(&mut self) -> ::windows::core::Result<()>;
    fn Individualize(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn CancelIndividualization(&mut self) -> ::windows::core::Result<()>;
    fn MonitorLicenseAcquisition(&mut self) -> ::windows::core::Result<()>;
    fn CancelMonitorLicenseAcquisition(&mut self) -> ::windows::core::Result<()>;
    fn SetDRMProperty(&mut self, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn GetDRMProperty(&mut self, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader_Impl, const OFFSET: isize>() -> IWMDRMReader_Vtbl {
        unsafe extern "system" fn AcquireLicense<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireLicense(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CancelLicenseAcquisition<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelLicenseAcquisition().into()
        }
        unsafe extern "system" fn Individualize<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Individualize(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CancelIndividualization<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelIndividualization().into()
        }
        unsafe extern "system" fn MonitorLicenseAcquisition<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MonitorLicenseAcquisition().into()
        }
        unsafe extern "system" fn CancelMonitorLicenseAcquisition<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelMonitorLicenseAcquisition().into()
        }
        unsafe extern "system" fn SetDRMProperty<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDRMProperty(::core::mem::transmute_copy(&pwstrname), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetDRMProperty<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDRMProperty(::core::mem::transmute_copy(&pwstrname), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AcquireLicense: AcquireLicense::<Identity, Impl, OFFSET>,
            CancelLicenseAcquisition: CancelLicenseAcquisition::<Identity, Impl, OFFSET>,
            Individualize: Individualize::<Identity, Impl, OFFSET>,
            CancelIndividualization: CancelIndividualization::<Identity, Impl, OFFSET>,
            MonitorLicenseAcquisition: MonitorLicenseAcquisition::<Identity, Impl, OFFSET>,
            CancelMonitorLicenseAcquisition: CancelMonitorLicenseAcquisition::<Identity, Impl, OFFSET>,
            SetDRMProperty: SetDRMProperty::<Identity, Impl, OFFSET>,
            GetDRMProperty: GetDRMProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReader2_Impl: Sized + IWMDRMReader_Impl {
    fn SetEvaluateOutputLevelLicenses(&mut self, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPlayOutputLevels(&mut self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()>;
    fn GetCopyOutputLevels(&mut self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()>;
    fn TryNextLicense(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader2_Impl, const OFFSET: isize>() -> IWMDRMReader2_Vtbl {
        unsafe extern "system" fn SetEvaluateOutputLevelLicenses<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEvaluateOutputLevelLicenses(::core::mem::transmute_copy(&fevaluate)).into()
        }
        unsafe extern "system" fn GetPlayOutputLevels<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPlayOutputLevels(::core::mem::transmute_copy(&pplayopl), ::core::mem::transmute_copy(&pcblength), ::core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
        }
        unsafe extern "system" fn GetCopyOutputLevels<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCopyOutputLevels(::core::mem::transmute_copy(&pcopyopl), ::core::mem::transmute_copy(&pcblength), ::core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
        }
        unsafe extern "system" fn TryNextLicense<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TryNextLicense().into()
        }
        Self {
            base: IWMDRMReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetEvaluateOutputLevelLicenses: SetEvaluateOutputLevelLicenses::<Identity, Impl, OFFSET>,
            GetPlayOutputLevels: GetPlayOutputLevels::<Identity, Impl, OFFSET>,
            GetCopyOutputLevels: GetCopyOutputLevels::<Identity, Impl, OFFSET>,
            TryNextLicense: TryNextLicense::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMReader2 as ::windows::core::Interface>::IID || iid == &<IWMDRMReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReader3_Impl: Sized + IWMDRMReader_Impl + IWMDRMReader2_Impl {
    fn GetInclusionList(&mut self, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReader3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader3_Impl, const OFFSET: isize>() -> IWMDRMReader3_Vtbl {
        unsafe extern "system" fn GetInclusionList<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInclusionList(::core::mem::transmute_copy(&ppguids), ::core::mem::transmute_copy(&pcguids)).into()
        }
        Self { base: IWMDRMReader2_Vtbl::new::<Identity, Impl, OFFSET>(), GetInclusionList: GetInclusionList::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMReader3 as ::windows::core::Interface>::IID || iid == &<IWMDRMReader as ::windows::core::Interface>::IID || iid == &<IWMDRMReader2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDRMTranscryptionManager_Impl: Sized {
    fn CreateTranscryptor(&mut self) -> ::windows::core::Result<IWMDRMTranscryptor>;
}
impl IWMDRMTranscryptionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptionManager_Impl, const OFFSET: isize>() -> IWMDRMTranscryptionManager_Vtbl {
        unsafe extern "system" fn CreateTranscryptor<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptranscryptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTranscryptor() {
                ::core::result::Result::Ok(ok__) => {
                    *pptranscryptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateTranscryptor: CreateTranscryptor::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMTranscryptor_Impl: Sized {
    fn Initialize(&mut self, bstrfilename: &super::super::Foundation::BSTR, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: &::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Seek(&mut self, hnstime: u64) -> ::windows::core::Result<()>;
    fn Read(&mut self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMTranscryptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>() -> IWMDRMTranscryptor_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&pblicenserequestmsg), ::core::mem::transmute_copy(&cblicenserequestmsg), ::core::mem::transmute_copy(&pplicenseresponsemsg), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Seek(::core::mem::transmute_copy(&hnstime)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&pcbdata)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMTranscryptor2_Impl: Sized + IWMDRMTranscryptor_Impl {
    fn SeekEx(&mut self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ZeroAdjustTimestamps(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSeekStartTime(&mut self) -> ::windows::core::Result<u64>;
    fn GetDuration(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMTranscryptor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>() -> IWMDRMTranscryptor2_Vtbl {
        unsafe extern "system" fn SeekEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SeekEx(::core::mem::transmute_copy(&cnsstarttime), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&flrate), ::core::mem::transmute_copy(&fincludefileheader)).into()
        }
        unsafe extern "system" fn ZeroAdjustTimestamps<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ZeroAdjustTimestamps(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetSeekStartTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSeekStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnstime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMDRMTranscryptor_Vtbl::new::<Identity, Impl, OFFSET>(),
            SeekEx: SeekEx::<Identity, Impl, OFFSET>,
            ZeroAdjustTimestamps: ZeroAdjustTimestamps::<Identity, Impl, OFFSET>,
            GetSeekStartTime: GetSeekStartTime::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor2 as ::windows::core::Interface>::IID || iid == &<IWMDRMTranscryptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter_Impl: Sized {
    fn GenerateKeySeed(&mut self, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()>;
    fn GenerateKeyID(&mut self, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()>;
    fn GenerateSigningKeyPair(&mut self, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()>;
    fn SetDRMAttribute(&mut self, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter_Impl, const OFFSET: isize>() -> IWMDRMWriter_Vtbl {
        unsafe extern "system" fn GenerateKeySeed<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateKeySeed(::core::mem::transmute_copy(&pwszkeyseed), ::core::mem::transmute_copy(&pcwchlength)).into()
        }
        unsafe extern "system" fn GenerateKeyID<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateKeyID(::core::mem::transmute_copy(&pwszkeyid), ::core::mem::transmute_copy(&pcwchlength)).into()
        }
        unsafe extern "system" fn GenerateSigningKeyPair<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GenerateSigningKeyPair(::core::mem::transmute_copy(&pwszprivkey), ::core::mem::transmute_copy(&pcwchprivkeylength), ::core::mem::transmute_copy(&pwszpubkey), ::core::mem::transmute_copy(&pcwchpubkeylength)).into()
        }
        unsafe extern "system" fn SetDRMAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDRMAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GenerateKeySeed: GenerateKeySeed::<Identity, Impl, OFFSET>,
            GenerateKeyID: GenerateKeyID::<Identity, Impl, OFFSET>,
            GenerateSigningKeyPair: GenerateSigningKeyPair::<Identity, Impl, OFFSET>,
            SetDRMAttribute: SetDRMAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter2_Impl: Sized + IWMDRMWriter_Impl {
    fn SetWMDRMNetEncryption(&mut self, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter2_Impl, const OFFSET: isize>() -> IWMDRMWriter2_Vtbl {
        unsafe extern "system" fn SetWMDRMNetEncryption<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWMDRMNetEncryption(::core::mem::transmute_copy(&fsamplesencrypted), ::core::mem::transmute_copy(&pbkeyid), ::core::mem::transmute_copy(&cbkeyid)).into()
        }
        Self { base: IWMDRMWriter_Vtbl::new::<Identity, Impl, OFFSET>(), SetWMDRMNetEncryption: SetWMDRMNetEncryption::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter2 as ::windows::core::Interface>::IID || iid == &<IWMDRMWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter3_Impl: Sized + IWMDRMWriter_Impl + IWMDRMWriter2_Impl {
    fn SetProtectStreamSamples(&mut self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter3_Impl, const OFFSET: isize>() -> IWMDRMWriter3_Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProtectStreamSamples(::core::mem::transmute_copy(&pimportinitstruct)).into()
        }
        Self { base: IWMDRMWriter2_Vtbl::new::<Identity, Impl, OFFSET>(), SetProtectStreamSamples: SetProtectStreamSamples::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter3 as ::windows::core::Interface>::IID || iid == &<IWMDRMWriter as ::windows::core::Interface>::IID || iid == &<IWMDRMWriter2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDeviceRegistration_Impl: Sized {
    fn RegisterDevice(&mut self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> ::windows::core::Result<IWMRegisteredDevice>;
    fn UnregisterDevice(&mut self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> ::windows::core::Result<()>;
    fn GetRegistrationStats(&mut self, dwregistertype: u32) -> ::windows::core::Result<u32>;
    fn GetFirstRegisteredDevice(&mut self, dwregistertype: u32) -> ::windows::core::Result<IWMRegisteredDevice>;
    fn GetNextRegisteredDevice(&mut self) -> ::windows::core::Result<IWMRegisteredDevice>;
    fn GetRegisteredDeviceByID(&mut self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> ::windows::core::Result<IWMRegisteredDevice>;
}
impl IWMDeviceRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>() -> IWMDeviceRegistration_Vtbl {
        unsafe extern "system" fn RegisterDevice<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterDevice(::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&serialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterDevice(::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&serialnumber)).into()
        }
        unsafe extern "system" fn GetRegistrationStats<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegistrationStats(::core::mem::transmute_copy(&dwregistertype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcregistereddevices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstRegisteredDevice<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFirstRegisteredDevice(::core::mem::transmute_copy(&dwregistertype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextRegisteredDevice<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNextRegisteredDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredDeviceByID<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRegisteredDeviceByID(::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&serialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterDevice: RegisterDevice::<Identity, Impl, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, Impl, OFFSET>,
            GetRegistrationStats: GetRegistrationStats::<Identity, Impl, OFFSET>,
            GetFirstRegisteredDevice: GetFirstRegisteredDevice::<Identity, Impl, OFFSET>,
            GetNextRegisteredDevice: GetNextRegisteredDevice::<Identity, Impl, OFFSET>,
            GetRegisteredDeviceByID: GetRegisteredDeviceByID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDeviceRegistration as ::windows::core::Interface>::IID
    }
}
pub trait IWMGetSecureChannel_Impl: Sized {
    fn GetPeerSecureChannelInterface(&mut self) -> ::windows::core::Result<IWMSecureChannel>;
}
impl IWMGetSecureChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMGetSecureChannel_Impl, const OFFSET: isize>() -> IWMGetSecureChannel_Vtbl {
        unsafe extern "system" fn GetPeerSecureChannelInterface<Identity: ::windows::core::IUnknownImpl, Impl: IWMGetSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppeer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPeerSecureChannelInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *pppeer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPeerSecureChannelInterface: GetPeerSecureChannelInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMGetSecureChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMHeaderInfo_Impl: Sized {
    fn GetAttributeCount(&mut self, wstreamnum: u16) -> ::windows::core::Result<u16>;
    fn GetAttributeByIndex(&mut self, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn GetAttributeByName(&mut self, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetAttribute(&mut self, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn GetMarkerCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetMarker(&mut self, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()>;
    fn AddMarker(&mut self, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::core::Result<()>;
    fn RemoveMarker(&mut self, windex: u16) -> ::windows::core::Result<()>;
    fn GetScriptCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetScript(&mut self, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()>;
    fn AddScript(&mut self, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::core::Result<()>;
    fn RemoveScript(&mut self, windex: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMHeaderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>() -> IWMHeaderInfo_Vtbl {
        unsafe extern "system" fn GetAttributeCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeCount(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttributeByIndex(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwstreamnum), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetAttributeByName<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttributeByName(::core::mem::transmute_copy(&pwstreamnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetMarkerCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMarkerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcmarkers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarker<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMarker(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszmarkername), ::core::mem::transmute_copy(&pcchmarkernamelen), ::core::mem::transmute_copy(&pcnsmarkertime)).into()
        }
        unsafe extern "system" fn AddMarker<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMarker(::core::mem::transmute_copy(&pwszmarkername), ::core::mem::transmute_copy(&cnsmarkertime)).into()
        }
        unsafe extern "system" fn RemoveMarker<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveMarker(::core::mem::transmute_copy(&windex)).into()
        }
        unsafe extern "system" fn GetScriptCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetScriptCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcscripts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScript<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetScript(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwsztype), ::core::mem::transmute_copy(&pcchtypelen), ::core::mem::transmute_copy(&pwszcommand), ::core::mem::transmute_copy(&pcchcommandlen), ::core::mem::transmute_copy(&pcnsscripttime)).into()
        }
        unsafe extern "system" fn AddScript<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddScript(::core::mem::transmute_copy(&pwsztype), ::core::mem::transmute_copy(&pwszcommand), ::core::mem::transmute_copy(&cnsscripttime)).into()
        }
        unsafe extern "system" fn RemoveScript<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveScript(::core::mem::transmute_copy(&windex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAttributeCount: GetAttributeCount::<Identity, Impl, OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Identity, Impl, OFFSET>,
            GetAttributeByName: GetAttributeByName::<Identity, Impl, OFFSET>,
            SetAttribute: SetAttribute::<Identity, Impl, OFFSET>,
            GetMarkerCount: GetMarkerCount::<Identity, Impl, OFFSET>,
            GetMarker: GetMarker::<Identity, Impl, OFFSET>,
            AddMarker: AddMarker::<Identity, Impl, OFFSET>,
            RemoveMarker: RemoveMarker::<Identity, Impl, OFFSET>,
            GetScriptCount: GetScriptCount::<Identity, Impl, OFFSET>,
            GetScript: GetScript::<Identity, Impl, OFFSET>,
            AddScript: AddScript::<Identity, Impl, OFFSET>,
            RemoveScript: RemoveScript::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMHeaderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMHeaderInfo2_Impl: Sized + IWMHeaderInfo_Impl {
    fn GetCodecInfoCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetCodecInfo(&mut self, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMHeaderInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo2_Impl, const OFFSET: isize>() -> IWMHeaderInfo2_Vtbl {
        unsafe extern "system" fn GetCodecInfoCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCodecInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccodecinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCodecInfo(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchdescription), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pcodectype), ::core::mem::transmute_copy(&pcbcodecinfo), ::core::mem::transmute_copy(&pbcodecinfo)).into()
        }
        Self {
            base: IWMHeaderInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Identity, Impl, OFFSET>,
            GetCodecInfo: GetCodecInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMHeaderInfo2 as ::windows::core::Interface>::IID || iid == &<IWMHeaderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMHeaderInfo3_Impl: Sized + IWMHeaderInfo_Impl + IWMHeaderInfo2_Impl {
    fn GetAttributeCountEx(&mut self, wstreamnum: u16) -> ::windows::core::Result<u16>;
    fn GetAttributeIndices(&mut self, wstreamnum: u16, pwszname: super::super::Foundation::PWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::Result<()>;
    fn GetAttributeByIndexEx(&mut self, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::Result<()>;
    fn ModifyAttribute(&mut self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::Result<()>;
    fn AddAttribute(&mut self, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::Result<()>;
    fn DeleteAttribute(&mut self, wstreamnum: u16, windex: u16) -> ::windows::core::Result<()>;
    fn AddCodecInfo(&mut self, pwszname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMHeaderInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>() -> IWMHeaderInfo3_Vtbl {
        unsafe extern "system" fn GetAttributeCountEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeCountEx(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeIndices<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: super::super::Foundation::PWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttributeIndices(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwlangindex), ::core::mem::transmute_copy(&pwindices), ::core::mem::transmute_copy(&pwcount)).into()
        }
        unsafe extern "system" fn GetAttributeByIndexEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttributeByIndexEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pwlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwdatalength)).into()
        }
        unsafe extern "system" fn ModifyAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ModifyAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&wlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn AddAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pwindex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&wlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn DeleteAttribute<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex)).into()
        }
        unsafe extern "system" fn AddCodecInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddCodecInfo(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&codectype), ::core::mem::transmute_copy(&cbcodecinfo), ::core::mem::transmute_copy(&pbcodecinfo)).into()
        }
        Self {
            base: IWMHeaderInfo2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAttributeCountEx: GetAttributeCountEx::<Identity, Impl, OFFSET>,
            GetAttributeIndices: GetAttributeIndices::<Identity, Impl, OFFSET>,
            GetAttributeByIndexEx: GetAttributeByIndexEx::<Identity, Impl, OFFSET>,
            ModifyAttribute: ModifyAttribute::<Identity, Impl, OFFSET>,
            AddAttribute: AddAttribute::<Identity, Impl, OFFSET>,
            DeleteAttribute: DeleteAttribute::<Identity, Impl, OFFSET>,
            AddCodecInfo: AddCodecInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMHeaderInfo3 as ::windows::core::Interface>::IID || iid == &<IWMHeaderInfo as ::windows::core::Interface>::IID || iid == &<IWMHeaderInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMIStreamProps_Impl: Sized {
    fn GetProperty(&mut self, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMIStreamProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIStreamProps_Impl, const OFFSET: isize>() -> IWMIStreamProps_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IWMIStreamProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetProperty: GetProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIStreamProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMImageInfo_Impl: Sized {
    fn GetImageCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetImage(&mut self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMImageInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMImageInfo_Impl, const OFFSET: isize>() -> IWMImageInfo_Vtbl {
        unsafe extern "system" fn GetImageCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMImageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetImageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcimages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImage<Identity: ::windows::core::IUnknownImpl, Impl: IWMImageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetImage(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pcchmimetype), ::core::mem::transmute_copy(&pwszmimetype), ::core::mem::transmute_copy(&pcchdescription), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pimagetype), ::core::mem::transmute_copy(&pcbimagedata), ::core::mem::transmute_copy(&pbimagedata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetImageCount: GetImageCount::<Identity, Impl, OFFSET>,
            GetImage: GetImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMImageInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMIndexer_Impl: Sized {
    fn StartIndexing(&mut self, pwszurl: super::super::Foundation::PWSTR, pcallback: &::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMIndexer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIndexer_Impl, const OFFSET: isize>() -> IWMIndexer_Vtbl {
        unsafe extern "system" fn StartIndexing<Identity: ::windows::core::IUnknownImpl, Impl: IWMIndexer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartIndexing(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IWMIndexer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            StartIndexing: StartIndexing::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIndexer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMIndexer2_Impl: Sized + IWMIndexer_Impl {
    fn Configure(&mut self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMIndexer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIndexer2_Impl, const OFFSET: isize>() -> IWMIndexer2_Vtbl {
        unsafe extern "system" fn Configure<Identity: ::windows::core::IUnknownImpl, Impl: IWMIndexer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Configure(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&nindexertype), ::core::mem::transmute_copy(&pvinterval), ::core::mem::transmute_copy(&pvindextype)).into()
        }
        Self { base: IWMIndexer_Vtbl::new::<Identity, Impl, OFFSET>(), Configure: Configure::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIndexer2 as ::windows::core::Interface>::IID || iid == &<IWMIndexer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMInputMediaProps_Impl: Sized + IWMMediaProps_Impl {
    fn GetConnectionName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
    fn GetGroupName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMInputMediaProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMInputMediaProps_Impl, const OFFSET: isize>() -> IWMInputMediaProps_Vtbl {
        unsafe extern "system" fn GetConnectionName<Identity: ::windows::core::IUnknownImpl, Impl: IWMInputMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConnectionName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetGroupName<Identity: ::windows::core::IUnknownImpl, Impl: IWMInputMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetGroupName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        Self {
            base: IWMMediaProps_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetConnectionName: GetConnectionName::<Identity, Impl, OFFSET>,
            GetGroupName: GetGroupName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMInputMediaProps as ::windows::core::Interface>::IID || iid == &<IWMMediaProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMLanguageList_Impl: Sized {
    fn GetLanguageCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetLanguageDetails(&mut self, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()>;
    fn AddLanguageByRFC1766String(&mut self, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMLanguageList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLanguageList_Impl, const OFFSET: isize>() -> IWMLanguageList_Vtbl {
        unsafe extern "system" fn GetLanguageCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMLanguageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLanguageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageDetails<Identity: ::windows::core::IUnknownImpl, Impl: IWMLanguageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLanguageDetails(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn AddLanguageByRFC1766String<Identity: ::windows::core::IUnknownImpl, Impl: IWMLanguageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pwindex: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddLanguageByRFC1766String(::core::mem::transmute_copy(&pwszlanguagestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLanguageCount: GetLanguageCount::<Identity, Impl, OFFSET>,
            GetLanguageDetails: GetLanguageDetails::<Identity, Impl, OFFSET>,
            AddLanguageByRFC1766String: AddLanguageByRFC1766String::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLanguageList as ::windows::core::Interface>::IID
    }
}
pub trait IWMLicenseBackup_Impl: Sized {
    fn BackupLicenses(&mut self, dwflags: u32, pcallback: &::core::option::Option<IWMStatusCallback>) -> ::windows::core::Result<()>;
    fn CancelLicenseBackup(&mut self) -> ::windows::core::Result<()>;
}
impl IWMLicenseBackup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseBackup_Impl, const OFFSET: isize>() -> IWMLicenseBackup_Vtbl {
        unsafe extern "system" fn BackupLicenses<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BackupLicenses(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn CancelLicenseBackup<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelLicenseBackup().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BackupLicenses: BackupLicenses::<Identity, Impl, OFFSET>,
            CancelLicenseBackup: CancelLicenseBackup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLicenseBackup as ::windows::core::Interface>::IID
    }
}
pub trait IWMLicenseRestore_Impl: Sized {
    fn RestoreLicenses(&mut self, dwflags: u32, pcallback: &::core::option::Option<IWMStatusCallback>) -> ::windows::core::Result<()>;
    fn CancelLicenseRestore(&mut self) -> ::windows::core::Result<()>;
}
impl IWMLicenseRestore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRestore_Impl, const OFFSET: isize>() -> IWMLicenseRestore_Vtbl {
        unsafe extern "system" fn RestoreLicenses<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRestore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RestoreLicenses(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn CancelLicenseRestore<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRestore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelLicenseRestore().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RestoreLicenses: RestoreLicenses::<Identity, Impl, OFFSET>,
            CancelLicenseRestore: CancelLicenseRestore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLicenseRestore as ::windows::core::Interface>::IID
    }
}
pub trait IWMLicenseRevocationAgent_Impl: Sized {
    fn GetLRBChallenge(&mut self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::Result<()>;
    fn ProcessLRB(&mut self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::Result<()>;
}
impl IWMLicenseRevocationAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRevocationAgent_Impl, const OFFSET: isize>() -> IWMLicenseRevocationAgent_Vtbl {
        unsafe extern "system" fn GetLRBChallenge<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRevocationAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLRBChallenge(::core::mem::transmute_copy(&pmachineid), ::core::mem::transmute_copy(&dwmachineidlength), ::core::mem::transmute_copy(&pchallenge), ::core::mem::transmute_copy(&dwchallengelength), ::core::mem::transmute_copy(&pchallengeoutput), ::core::mem::transmute_copy(&pdwchallengeoutputlength)).into()
        }
        unsafe extern "system" fn ProcessLRB<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRevocationAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessLRB(::core::mem::transmute_copy(&psignedlrb), ::core::mem::transmute_copy(&dwsignedlrblength), ::core::mem::transmute_copy(&psignedack), ::core::mem::transmute_copy(&pdwsignedacklength)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLRBChallenge: GetLRBChallenge::<Identity, Impl, OFFSET>,
            ProcessLRB: ProcessLRB::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLicenseRevocationAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMediaProps_Impl: Sized {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetMediaType(&mut self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()>;
    fn SetMediaType(&mut self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMediaProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMediaProps_Impl, const OFFSET: isize>() -> IWMMediaProps_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IWMMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMediaType<Identity: ::windows::core::IUnknownImpl, Impl: IWMMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMediaType(::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pcbtype)).into()
        }
        unsafe extern "system" fn SetMediaType<Identity: ::windows::core::IUnknownImpl, Impl: IWMMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMediaType(::core::mem::transmute_copy(&ptype)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetMediaType: GetMediaType::<Identity, Impl, OFFSET>,
            SetMediaType: SetMediaType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMediaProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMetadataEditor_Impl: Sized {
    fn Open(&mut self, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMetadataEditor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditor_Impl, const OFFSET: isize>() -> IWMMetadataEditor_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMetadataEditor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMetadataEditor2_Impl: Sized + IWMMetadataEditor_Impl {
    fn OpenEx(&mut self, pwszfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMetadataEditor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditor2_Impl, const OFFSET: isize>() -> IWMMetadataEditor2_Vtbl {
        unsafe extern "system" fn OpenEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenEx(::core::mem::transmute_copy(&pwszfilename), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&dwsharemode)).into()
        }
        Self { base: IWMMetadataEditor_Vtbl::new::<Identity, Impl, OFFSET>(), OpenEx: OpenEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMetadataEditor2 as ::windows::core::Interface>::IID || iid == &<IWMMetadataEditor as ::windows::core::Interface>::IID
    }
}
pub trait IWMMutualExclusion_Impl: Sized + IWMStreamList_Impl {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetType(&mut self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IWMMutualExclusion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion_Impl, const OFFSET: isize>() -> IWMMutualExclusion_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&guidtype)).into()
        }
        Self {
            base: IWMStreamList_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMutualExclusion as ::windows::core::Interface>::IID || iid == &<IWMStreamList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMutualExclusion2_Impl: Sized + IWMStreamList_Impl + IWMMutualExclusion_Impl {
    fn GetName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
    fn SetName(&mut self, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetRecordCount(&mut self) -> ::windows::core::Result<u16>;
    fn AddRecord(&mut self) -> ::windows::core::Result<()>;
    fn RemoveRecord(&mut self, wrecordnumber: u16) -> ::windows::core::Result<()>;
    fn GetRecordName(&mut self, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::Result<()>;
    fn SetRecordName(&mut self, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetStreamsForRecord(&mut self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()>;
    fn AddStreamForRecord(&mut self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()>;
    fn RemoveStreamForRecord(&mut self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMutualExclusion2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>() -> IWMMutualExclusion2_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pwrecordcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRecord<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRecord().into()
        }
        unsafe extern "system" fn RemoveRecord<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveRecord(::core::mem::transmute_copy(&wrecordnumber)).into()
        }
        unsafe extern "system" fn GetRecordName<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRecordName(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwszrecordname), ::core::mem::transmute_copy(&pcchrecordname)).into()
        }
        unsafe extern "system" fn SetRecordName<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRecordName(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwszrecordname)).into()
        }
        unsafe extern "system" fn GetStreamsForRecord<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreamsForRecord(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwstreamnumarray), ::core::mem::transmute_copy(&pcstreams)).into()
        }
        unsafe extern "system" fn AddStreamForRecord<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStreamForRecord(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&wstreamnumber)).into()
        }
        unsafe extern "system" fn RemoveStreamForRecord<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStreamForRecord(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&wstreamnumber)).into()
        }
        Self {
            base: IWMMutualExclusion_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetRecordCount: GetRecordCount::<Identity, Impl, OFFSET>,
            AddRecord: AddRecord::<Identity, Impl, OFFSET>,
            RemoveRecord: RemoveRecord::<Identity, Impl, OFFSET>,
            GetRecordName: GetRecordName::<Identity, Impl, OFFSET>,
            SetRecordName: SetRecordName::<Identity, Impl, OFFSET>,
            GetStreamsForRecord: GetStreamsForRecord::<Identity, Impl, OFFSET>,
            AddStreamForRecord: AddStreamForRecord::<Identity, Impl, OFFSET>,
            RemoveStreamForRecord: RemoveStreamForRecord::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMutualExclusion2 as ::windows::core::Interface>::IID || iid == &<IWMStreamList as ::windows::core::Interface>::IID || iid == &<IWMMutualExclusion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMOutputMediaProps_Impl: Sized + IWMMediaProps_Impl {
    fn GetStreamGroupName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
    fn GetConnectionName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMOutputMediaProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMOutputMediaProps_Impl, const OFFSET: isize>() -> IWMOutputMediaProps_Vtbl {
        unsafe extern "system" fn GetStreamGroupName<Identity: ::windows::core::IUnknownImpl, Impl: IWMOutputMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreamGroupName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetConnectionName<Identity: ::windows::core::IUnknownImpl, Impl: IWMOutputMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConnectionName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        Self {
            base: IWMMediaProps_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStreamGroupName: GetStreamGroupName::<Identity, Impl, OFFSET>,
            GetConnectionName: GetConnectionName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMOutputMediaProps as ::windows::core::Interface>::IID || iid == &<IWMMediaProps as ::windows::core::Interface>::IID
    }
}
pub trait IWMPacketSize_Impl: Sized {
    fn GetMaxPacketSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxPacketSize(&mut self, dwmaxpacketsize: u32) -> ::windows::core::Result<()>;
}
impl IWMPacketSize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSize_Impl, const OFFSET: isize>() -> IWMPacketSize_Vtbl {
        unsafe extern "system" fn GetMaxPacketSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxpacketsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPacketSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxPacketSize(::core::mem::transmute_copy(&dwmaxpacketsize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetMaxPacketSize: GetMaxPacketSize::<Identity, Impl, OFFSET>,
            SetMaxPacketSize: SetMaxPacketSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPacketSize as ::windows::core::Interface>::IID
    }
}
pub trait IWMPacketSize2_Impl: Sized + IWMPacketSize_Impl {
    fn GetMinPacketSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetMinPacketSize(&mut self, dwminpacketsize: u32) -> ::windows::core::Result<()>;
}
impl IWMPacketSize2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSize2_Impl, const OFFSET: isize>() -> IWMPacketSize2_Vtbl {
        unsafe extern "system" fn GetMinPacketSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSize2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMinPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwminpacketsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPacketSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSize2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMinPacketSize(::core::mem::transmute_copy(&dwminpacketsize)).into()
        }
        Self {
            base: IWMPacketSize_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMinPacketSize: GetMinPacketSize::<Identity, Impl, OFFSET>,
            SetMinPacketSize: SetMinPacketSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPacketSize2 as ::windows::core::Interface>::IID || iid == &<IWMPacketSize as ::windows::core::Interface>::IID
    }
}
pub trait IWMPlayerHook_Impl: Sized {
    fn PreDecode(&mut self) -> ::windows::core::Result<()>;
}
impl IWMPlayerHook_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPlayerHook_Impl, const OFFSET: isize>() -> IWMPlayerHook_Vtbl {
        unsafe extern "system" fn PreDecode<Identity: ::windows::core::IUnknownImpl, Impl: IWMPlayerHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreDecode().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), PreDecode: PreDecode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPlayerHook as ::windows::core::Interface>::IID
    }
}
pub trait IWMPlayerTimestampHook_Impl: Sized {
    fn MapTimestamp(&mut self, rtin: i64) -> ::windows::core::Result<i64>;
}
impl IWMPlayerTimestampHook_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPlayerTimestampHook_Impl, const OFFSET: isize>() -> IWMPlayerTimestampHook_Vtbl {
        unsafe extern "system" fn MapTimestamp<Identity: ::windows::core::IUnknownImpl, Impl: IWMPlayerTimestampHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MapTimestamp(::core::mem::transmute_copy(&rtin)) {
                ::core::result::Result::Ok(ok__) => {
                    *prtout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), MapTimestamp: MapTimestamp::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPlayerTimestampHook as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfile_Impl: Sized {
    fn GetVersion(&mut self) -> ::windows::core::Result<WMT_VERSION>;
    fn GetName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()>;
    fn SetName(&mut self, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDescription(&mut self, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()>;
    fn SetDescription(&mut self, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetStreamCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetStream(&mut self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig>;
    fn GetStreamByNumber(&mut self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig>;
    fn RemoveStream(&mut self, pconfig: &::core::option::Option<IWMStreamConfig>) -> ::windows::core::Result<()>;
    fn RemoveStreamByNumber(&mut self, wstreamnum: u16) -> ::windows::core::Result<()>;
    fn AddStream(&mut self, pconfig: &::core::option::Option<IWMStreamConfig>) -> ::windows::core::Result<()>;
    fn ReconfigStream(&mut self, pconfig: &::core::option::Option<IWMStreamConfig>) -> ::windows::core::Result<()>;
    fn CreateNewStream(&mut self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig>;
    fn GetMutualExclusionCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetMutualExclusion(&mut self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion>;
    fn RemoveMutualExclusion(&mut self, pme: &::core::option::Option<IWMMutualExclusion>) -> ::windows::core::Result<()>;
    fn AddMutualExclusion(&mut self, pme: &::core::option::Option<IWMMutualExclusion>) -> ::windows::core::Result<()>;
    fn CreateNewMutualExclusion(&mut self) -> ::windows::core::Result<IWMMutualExclusion>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>() -> IWMProfile_Vtbl {
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDescription(::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pcchdescription)).into()
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&pwszdescription)).into()
        }
        unsafe extern "system" fn GetStreamCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcstreams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&dwstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamByNumber<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamByNumber(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute(&pconfig)).into()
        }
        unsafe extern "system" fn RemoveStreamByNumber<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStreamByNumber(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn AddStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStream(::core::mem::transmute(&pconfig)).into()
        }
        unsafe extern "system" fn ReconfigStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReconfigStream(::core::mem::transmute(&pconfig)).into()
        }
        unsafe extern "system" fn CreateNewStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateNewStream(::core::mem::transmute_copy(&guidstreamtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusionCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMutualExclusionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcme = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusion<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMutualExclusion(::core::mem::transmute_copy(&dwmeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppme = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMutualExclusion<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveMutualExclusion(::core::mem::transmute(&pme)).into()
        }
        unsafe extern "system" fn AddMutualExclusion<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMutualExclusion(::core::mem::transmute(&pme)).into()
        }
        unsafe extern "system" fn CreateNewMutualExclusion<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateNewMutualExclusion() {
                ::core::result::Result::Ok(ok__) => {
                    *ppme = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetStreamCount: GetStreamCount::<Identity, Impl, OFFSET>,
            GetStream: GetStream::<Identity, Impl, OFFSET>,
            GetStreamByNumber: GetStreamByNumber::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            RemoveStreamByNumber: RemoveStreamByNumber::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            ReconfigStream: ReconfigStream::<Identity, Impl, OFFSET>,
            CreateNewStream: CreateNewStream::<Identity, Impl, OFFSET>,
            GetMutualExclusionCount: GetMutualExclusionCount::<Identity, Impl, OFFSET>,
            GetMutualExclusion: GetMutualExclusion::<Identity, Impl, OFFSET>,
            RemoveMutualExclusion: RemoveMutualExclusion::<Identity, Impl, OFFSET>,
            AddMutualExclusion: AddMutualExclusion::<Identity, Impl, OFFSET>,
            CreateNewMutualExclusion: CreateNewMutualExclusion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfile2_Impl: Sized + IWMProfile_Impl {
    fn GetProfileID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfile2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile2_Impl, const OFFSET: isize>() -> IWMProfile2_Vtbl {
        unsafe extern "system" fn GetProfileID<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProfileID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWMProfile_Vtbl::new::<Identity, Impl, OFFSET>(), GetProfileID: GetProfileID::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfile2 as ::windows::core::Interface>::IID || iid == &<IWMProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfile3_Impl: Sized + IWMProfile_Impl + IWMProfile2_Impl {
    fn GetStorageFormat(&mut self) -> ::windows::core::Result<WMT_STORAGE_FORMAT>;
    fn SetStorageFormat(&mut self, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::Result<()>;
    fn GetBandwidthSharingCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetBandwidthSharing(&mut self, dwbsindex: u32) -> ::windows::core::Result<IWMBandwidthSharing>;
    fn RemoveBandwidthSharing(&mut self, pbs: &::core::option::Option<IWMBandwidthSharing>) -> ::windows::core::Result<()>;
    fn AddBandwidthSharing(&mut self, pbs: &::core::option::Option<IWMBandwidthSharing>) -> ::windows::core::Result<()>;
    fn CreateNewBandwidthSharing(&mut self) -> ::windows::core::Result<IWMBandwidthSharing>;
    fn GetStreamPrioritization(&mut self) -> ::windows::core::Result<IWMStreamPrioritization>;
    fn SetStreamPrioritization(&mut self, psp: &::core::option::Option<IWMStreamPrioritization>) -> ::windows::core::Result<()>;
    fn RemoveStreamPrioritization(&mut self) -> ::windows::core::Result<()>;
    fn CreateNewStreamPrioritization(&mut self) -> ::windows::core::Result<IWMStreamPrioritization>;
    fn GetExpectedPacketCount(&mut self, msduration: u64) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfile3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>() -> IWMProfile3_Vtbl {
        unsafe extern "system" fn GetStorageFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStorageFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pnstorageformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStorageFormat(::core::mem::transmute_copy(&nstorageformat)).into()
        }
        unsafe extern "system" fn GetBandwidthSharingCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBandwidthSharingCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBandwidthSharing<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBandwidthSharing(::core::mem::transmute_copy(&dwbsindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBandwidthSharing<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveBandwidthSharing(::core::mem::transmute(&pbs)).into()
        }
        unsafe extern "system" fn AddBandwidthSharing<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddBandwidthSharing(::core::mem::transmute(&pbs)).into()
        }
        unsafe extern "system" fn CreateNewBandwidthSharing<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateNewBandwidthSharing() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamPrioritization<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamPrioritization() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamPrioritization<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStreamPrioritization(::core::mem::transmute(&psp)).into()
        }
        unsafe extern "system" fn RemoveStreamPrioritization<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStreamPrioritization().into()
        }
        unsafe extern "system" fn CreateNewStreamPrioritization<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateNewStreamPrioritization() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpectedPacketCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetExpectedPacketCount(::core::mem::transmute_copy(&msduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcpackets = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMProfile2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStorageFormat: GetStorageFormat::<Identity, Impl, OFFSET>,
            SetStorageFormat: SetStorageFormat::<Identity, Impl, OFFSET>,
            GetBandwidthSharingCount: GetBandwidthSharingCount::<Identity, Impl, OFFSET>,
            GetBandwidthSharing: GetBandwidthSharing::<Identity, Impl, OFFSET>,
            RemoveBandwidthSharing: RemoveBandwidthSharing::<Identity, Impl, OFFSET>,
            AddBandwidthSharing: AddBandwidthSharing::<Identity, Impl, OFFSET>,
            CreateNewBandwidthSharing: CreateNewBandwidthSharing::<Identity, Impl, OFFSET>,
            GetStreamPrioritization: GetStreamPrioritization::<Identity, Impl, OFFSET>,
            SetStreamPrioritization: SetStreamPrioritization::<Identity, Impl, OFFSET>,
            RemoveStreamPrioritization: RemoveStreamPrioritization::<Identity, Impl, OFFSET>,
            CreateNewStreamPrioritization: CreateNewStreamPrioritization::<Identity, Impl, OFFSET>,
            GetExpectedPacketCount: GetExpectedPacketCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfile3 as ::windows::core::Interface>::IID || iid == &<IWMProfile as ::windows::core::Interface>::IID || iid == &<IWMProfile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfileManager_Impl: Sized {
    fn CreateEmptyProfile(&mut self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile>;
    fn LoadProfileByID(&mut self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile>;
    fn LoadProfileByData(&mut self, pwszprofile: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWMProfile>;
    fn SaveProfile(&mut self, piwmprofile: &::core::option::Option<IWMProfile>, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetSystemProfileCount(&mut self) -> ::windows::core::Result<u32>;
    fn LoadSystemProfile(&mut self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfileManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager_Impl, const OFFSET: isize>() -> IWMProfileManager_Vtbl {
        unsafe extern "system" fn CreateEmptyProfile<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateEmptyProfile(::core::mem::transmute_copy(&dwversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByID<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadProfileByID(::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByData<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprofile: super::super::Foundation::PWSTR, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadProfileByData(::core::mem::transmute_copy(&pwszprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveProfile<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmprofile: ::windows::core::RawPtr, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveProfile(::core::mem::transmute(&piwmprofile), ::core::mem::transmute_copy(&pwszprofile), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSystemProfileCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSystemProfileCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcprofiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSystemProfile<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LoadSystemProfile(::core::mem::transmute_copy(&dwprofileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateEmptyProfile: CreateEmptyProfile::<Identity, Impl, OFFSET>,
            LoadProfileByID: LoadProfileByID::<Identity, Impl, OFFSET>,
            LoadProfileByData: LoadProfileByData::<Identity, Impl, OFFSET>,
            SaveProfile: SaveProfile::<Identity, Impl, OFFSET>,
            GetSystemProfileCount: GetSystemProfileCount::<Identity, Impl, OFFSET>,
            LoadSystemProfile: LoadSystemProfile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfileManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfileManager2_Impl: Sized + IWMProfileManager_Impl {
    fn GetSystemProfileVersion(&mut self, pdwversion: *mut WMT_VERSION) -> ::windows::core::Result<()>;
    fn SetSystemProfileVersion(&mut self, dwversion: WMT_VERSION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfileManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager2_Impl, const OFFSET: isize>() -> IWMProfileManager2_Vtbl {
        unsafe extern "system" fn GetSystemProfileVersion<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSystemProfileVersion(::core::mem::transmute_copy(&pdwversion)).into()
        }
        unsafe extern "system" fn SetSystemProfileVersion<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSystemProfileVersion(::core::mem::transmute_copy(&dwversion)).into()
        }
        Self {
            base: IWMProfileManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSystemProfileVersion: GetSystemProfileVersion::<Identity, Impl, OFFSET>,
            SetSystemProfileVersion: SetSystemProfileVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfileManager2 as ::windows::core::Interface>::IID || iid == &<IWMProfileManager as ::windows::core::Interface>::IID
    }
}
pub trait IWMProfileManagerLanguage_Impl: Sized {
    fn GetUserLanguageID(&mut self, wlangid: *mut u16) -> ::windows::core::Result<()>;
    fn SetUserLanguageID(&mut self, wlangid: u16) -> ::windows::core::Result<()>;
}
impl IWMProfileManagerLanguage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManagerLanguage_Impl, const OFFSET: isize>() -> IWMProfileManagerLanguage_Vtbl {
        unsafe extern "system" fn GetUserLanguageID<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManagerLanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUserLanguageID(::core::mem::transmute_copy(&wlangid)).into()
        }
        unsafe extern "system" fn SetUserLanguageID<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManagerLanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserLanguageID(::core::mem::transmute_copy(&wlangid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetUserLanguageID: GetUserLanguageID::<Identity, Impl, OFFSET>,
            SetUserLanguageID: SetUserLanguageID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfileManagerLanguage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPropertyVault_Impl: Sized {
    fn GetPropertyCount(&mut self, pdwcount: *const u32) -> ::windows::core::Result<()>;
    fn GetPropertyByName(&mut self, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, pszname: super::super::Foundation::PWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn GetPropertyByIndex(&mut self, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn CopyPropertiesFrom(&mut self, piwmpropertyvault: &::core::option::Option<IWMPropertyVault>) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPropertyVault_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPropertyVault_Impl, const OFFSET: isize>() -> IWMPropertyVault_Vtbl {
        unsafe extern "system" fn GetPropertyCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyCount(::core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn GetPropertyByName<Identity: ::windows::core::IUnknownImpl, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyByName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPropertyByIndex(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pdwnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn CopyPropertiesFrom<Identity: ::windows::core::IUnknownImpl, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpropertyvault: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CopyPropertiesFrom(::core::mem::transmute(&piwmpropertyvault)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, Impl, OFFSET>,
            GetPropertyByName: GetPropertyByName::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, Impl, OFFSET>,
            CopyPropertiesFrom: CopyPropertiesFrom::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPropertyVault as ::windows::core::Interface>::IID
    }
}
pub trait IWMProximityDetection_Impl: Sized {
    fn StartDetection(&mut self, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: &::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMProximityDetection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProximityDetection_Impl, const OFFSET: isize>() -> IWMProximityDetection_Vtbl {
        unsafe extern "system" fn StartDetection<Identity: ::windows::core::IUnknownImpl, Impl: IWMProximityDetection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartDetection(::core::mem::transmute_copy(&pbregistrationmsg), ::core::mem::transmute_copy(&cbregistrationmsg), ::core::mem::transmute_copy(&pblocaladdress), ::core::mem::transmute_copy(&cblocaladdress), ::core::mem::transmute_copy(&dwextraportsallowed), ::core::mem::transmute_copy(&ppregistrationresponsemsg), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), StartDetection: StartDetection::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProximityDetection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReader_Impl: Sized {
    fn Open(&mut self, pwszurl: super::super::Foundation::PWSTR, pcallback: &::core::option::Option<IWMReaderCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn GetOutputCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetOutputProps(&mut self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(&mut self, dwoutputnum: u32, poutput: &::core::option::Option<IWMOutputMediaProps>) -> ::windows::core::Result<()>;
    fn GetOutputFormatCount(&mut self, dwoutputnumber: u32) -> ::windows::core::Result<u32>;
    fn GetOutputFormat(&mut self, dwoutputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn Start(&mut self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>() -> IWMReader_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcoutputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputProps(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputProps(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputFormatCount(::core::mem::transmute_copy(&dwoutputnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputFormat(::core::mem::transmute_copy(&dwoutputnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&cnsstart), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            GetOutputProps: GetOutputProps::<Identity, Impl, OFFSET>,
            SetOutputProps: SetOutputProps::<Identity, Impl, OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Identity, Impl, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Pause: Pause::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderAccelerator_Impl: Sized {
    fn GetCodecInterface(&mut self, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Notify(&mut self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderAccelerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAccelerator_Impl, const OFFSET: isize>() -> IWMReaderAccelerator_Vtbl {
        unsafe extern "system" fn GetCodecInterface<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCodecInterface(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvcodecinterface)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&psubtype)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCodecInterface: GetCodecInterface::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAccelerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderAdvanced_Impl: Sized {
    fn SetUserProvidedClock(&mut self, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetUserProvidedClock(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DeliverTime(&mut self, cnstime: u64) -> ::windows::core::Result<()>;
    fn SetManualStreamSelection(&mut self, fselection: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetManualStreamSelection(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetStreamsSelected(&mut self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()>;
    fn GetStreamSelected(&mut self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION>;
    fn SetReceiveSelectionCallbacks(&mut self, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReceiveSelectionCallbacks(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetReceiveStreamSamples(&mut self, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReceiveStreamSamples(&mut self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAllocateForOutput(&mut self, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateForOutput(&mut self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAllocateForStream(&mut self, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateForStream(&mut self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetStatistics(&mut self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()>;
    fn SetClientInfo(&mut self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()>;
    fn GetMaxOutputSampleSize(&mut self, dwoutput: u32) -> ::windows::core::Result<u32>;
    fn GetMaxStreamSampleSize(&mut self, wstream: u16) -> ::windows::core::Result<u32>;
    fn NotifyLateDelivery(&mut self, cnslateness: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderAdvanced_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>() -> IWMReaderAdvanced_Vtbl {
        unsafe extern "system" fn SetUserProvidedClock<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserProvidedClock(::core::mem::transmute_copy(&fuserclock)).into()
        }
        unsafe extern "system" fn GetUserProvidedClock<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUserProvidedClock() {
                ::core::result::Result::Ok(ok__) => {
                    *pfuserclock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeliverTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeliverTime(::core::mem::transmute_copy(&cnstime)).into()
        }
        unsafe extern "system" fn SetManualStreamSelection<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetManualStreamSelection(::core::mem::transmute_copy(&fselection)).into()
        }
        unsafe extern "system" fn GetManualStreamSelection<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetManualStreamSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pfselection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStreamsSelected(::core::mem::transmute_copy(&cstreamcount), ::core::mem::transmute_copy(&pwstreamnumbers), ::core::mem::transmute_copy(&pselections)).into()
        }
        unsafe extern "system" fn GetStreamSelected<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamSelected(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pselection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveSelectionCallbacks<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReceiveSelectionCallbacks(::core::mem::transmute_copy(&fgetcallbacks)).into()
        }
        unsafe extern "system" fn GetReceiveSelectionCallbacks<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReceiveSelectionCallbacks() {
                ::core::result::Result::Ok(ok__) => {
                    *pfgetcallbacks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveStreamSamples<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReceiveStreamSamples(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&freceivestreamsamples)).into()
        }
        unsafe extern "system" fn GetReceiveStreamSamples<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReceiveStreamSamples(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfreceivestreamsamples = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfallocate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllocateForStream(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllocateForStream(::core::mem::transmute_copy(&dwsreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfallocate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&pstatistics)).into()
        }
        unsafe extern "system" fn SetClientInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetClientInfo(::core::mem::transmute_copy(&pclientinfo)).into()
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxOutputSampleSize(::core::mem::transmute_copy(&dwoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxStreamSampleSize(::core::mem::transmute_copy(&wstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyLateDelivery<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NotifyLateDelivery(::core::mem::transmute_copy(&cnslateness)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetUserProvidedClock: SetUserProvidedClock::<Identity, Impl, OFFSET>,
            GetUserProvidedClock: GetUserProvidedClock::<Identity, Impl, OFFSET>,
            DeliverTime: DeliverTime::<Identity, Impl, OFFSET>,
            SetManualStreamSelection: SetManualStreamSelection::<Identity, Impl, OFFSET>,
            GetManualStreamSelection: GetManualStreamSelection::<Identity, Impl, OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Identity, Impl, OFFSET>,
            GetStreamSelected: GetStreamSelected::<Identity, Impl, OFFSET>,
            SetReceiveSelectionCallbacks: SetReceiveSelectionCallbacks::<Identity, Impl, OFFSET>,
            GetReceiveSelectionCallbacks: GetReceiveSelectionCallbacks::<Identity, Impl, OFFSET>,
            SetReceiveStreamSamples: SetReceiveStreamSamples::<Identity, Impl, OFFSET>,
            GetReceiveStreamSamples: GetReceiveStreamSamples::<Identity, Impl, OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Identity, Impl, OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Identity, Impl, OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Identity, Impl, OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            SetClientInfo: SetClientInfo::<Identity, Impl, OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Identity, Impl, OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Identity, Impl, OFFSET>,
            NotifyLateDelivery: NotifyLateDelivery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced2_Impl: Sized + IWMReaderAdvanced_Impl {
    fn SetPlayMode(&mut self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()>;
    fn GetPlayMode(&mut self) -> ::windows::core::Result<WMT_PLAY_MODE>;
    fn GetBufferProgress(&mut self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()>;
    fn GetDownloadProgress(&mut self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()>;
    fn GetSaveAsProgress(&mut self) -> ::windows::core::Result<u32>;
    fn SaveFileAs(&mut self, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetProtocolName(&mut self, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()>;
    fn StartAtMarker(&mut self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetOutputSetting(&mut self, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetOutputSetting(&mut self, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn Preroll(&mut self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()>;
    fn SetLogClientID(&mut self, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLogClientID(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn StopBuffering(&mut self) -> ::windows::core::Result<()>;
    fn OpenStream(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>, pcallback: &::core::option::Option<IWMReaderCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>() -> IWMReaderAdvanced2_Vtbl {
        unsafe extern "system" fn SetPlayMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlayMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn GetPlayMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPlayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferProgress<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBufferProgress(::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pcnsbuffering)).into()
        }
        unsafe extern "system" fn GetDownloadProgress<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDownloadProgress(::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pqwbytesdownloaded), ::core::mem::transmute_copy(&pcnsdownload)).into()
        }
        unsafe extern "system" fn GetSaveAsProgress<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSaveAsProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwpercent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveFileAs<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SaveFileAs(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        unsafe extern "system" fn GetProtocolName<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProtocolName(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pcchprotocol)).into()
        }
        unsafe extern "system" fn StartAtMarker<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartAtMarker(::core::mem::transmute_copy(&wmarkerindex), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn GetOutputSetting<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetOutputSetting<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn Preroll<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Preroll(::core::mem::transmute_copy(&cnsstart), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate)).into()
        }
        unsafe extern "system" fn SetLogClientID<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLogClientID(::core::mem::transmute_copy(&flogclientid)).into()
        }
        unsafe extern "system" fn GetLogClientID<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLogClientID() {
                ::core::result::Result::Ok(ok__) => {
                    *pflogclientid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopBuffering<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopBuffering().into()
        }
        unsafe extern "system" fn OpenStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenStream(::core::mem::transmute(&pstream), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: IWMReaderAdvanced_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPlayMode: SetPlayMode::<Identity, Impl, OFFSET>,
            GetPlayMode: GetPlayMode::<Identity, Impl, OFFSET>,
            GetBufferProgress: GetBufferProgress::<Identity, Impl, OFFSET>,
            GetDownloadProgress: GetDownloadProgress::<Identity, Impl, OFFSET>,
            GetSaveAsProgress: GetSaveAsProgress::<Identity, Impl, OFFSET>,
            SaveFileAs: SaveFileAs::<Identity, Impl, OFFSET>,
            GetProtocolName: GetProtocolName::<Identity, Impl, OFFSET>,
            StartAtMarker: StartAtMarker::<Identity, Impl, OFFSET>,
            GetOutputSetting: GetOutputSetting::<Identity, Impl, OFFSET>,
            SetOutputSetting: SetOutputSetting::<Identity, Impl, OFFSET>,
            Preroll: Preroll::<Identity, Impl, OFFSET>,
            SetLogClientID: SetLogClientID::<Identity, Impl, OFFSET>,
            GetLogClientID: GetLogClientID::<Identity, Impl, OFFSET>,
            StopBuffering: StopBuffering::<Identity, Impl, OFFSET>,
            OpenStream: OpenStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced2 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced3_Impl: Sized + IWMReaderAdvanced_Impl + IWMReaderAdvanced2_Impl {
    fn StopNetStreaming(&mut self) -> ::windows::core::Result<()>;
    fn StartAtPosition(&mut self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced3_Impl, const OFFSET: isize>() -> IWMReaderAdvanced3_Vtbl {
        unsafe extern "system" fn StopNetStreaming<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StopNetStreaming().into()
        }
        unsafe extern "system" fn StartAtPosition<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).StartAtPosition(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pvoffsetstart), ::core::mem::transmute_copy(&pvduration), ::core::mem::transmute_copy(&dwoffsetformat), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: IWMReaderAdvanced2_Vtbl::new::<Identity, Impl, OFFSET>(),
            StopNetStreaming: StopNetStreaming::<Identity, Impl, OFFSET>,
            StartAtPosition: StartAtPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced3 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced4_Impl: Sized + IWMReaderAdvanced_Impl + IWMReaderAdvanced2_Impl + IWMReaderAdvanced3_Impl {
    fn GetLanguageCount(&mut self, dwoutputnum: u32) -> ::windows::core::Result<u16>;
    fn GetLanguage(&mut self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()>;
    fn GetMaxSpeedFactor(&mut self) -> ::windows::core::Result<f64>;
    fn IsUsingFastCache(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn AddLogParam(&mut self, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SendLogParams(&mut self) -> ::windows::core::Result<()>;
    fn CanSaveFileAs(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CancelSaveFileAs(&mut self) -> ::windows::core::Result<()>;
    fn GetURL(&mut self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>() -> IWMReaderAdvanced4_Vtbl {
        unsafe extern "system" fn GetLanguageCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLanguageCount(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwlanguagecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLanguage(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&wlanguage), ::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn GetMaxSpeedFactor<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxSpeedFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *pdblfactor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUsingFastCache<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUsingFastCache() {
                ::core::result::Result::Ok(ok__) => {
                    *pfusingfastcache = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLogParam<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLogParam(::core::mem::transmute_copy(&wsznamespace), ::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&wszvalue)).into()
        }
        unsafe extern "system" fn SendLogParams<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendLogParams().into()
        }
        unsafe extern "system" fn CanSaveFileAs<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanSaveFileAs() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcansave = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelSaveFileAs<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CancelSaveFileAs().into()
        }
        unsafe extern "system" fn GetURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetURL(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into()
        }
        Self {
            base: IWMReaderAdvanced3_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLanguageCount: GetLanguageCount::<Identity, Impl, OFFSET>,
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            GetMaxSpeedFactor: GetMaxSpeedFactor::<Identity, Impl, OFFSET>,
            IsUsingFastCache: IsUsingFastCache::<Identity, Impl, OFFSET>,
            AddLogParam: AddLogParam::<Identity, Impl, OFFSET>,
            SendLogParams: SendLogParams::<Identity, Impl, OFFSET>,
            CanSaveFileAs: CanSaveFileAs::<Identity, Impl, OFFSET>,
            CancelSaveFileAs: CancelSaveFileAs::<Identity, Impl, OFFSET>,
            GetURL: GetURL::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced4 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced2 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced5_Impl: Sized + IWMReaderAdvanced_Impl + IWMReaderAdvanced2_Impl + IWMReaderAdvanced3_Impl + IWMReaderAdvanced4_Impl {
    fn SetPlayerHook(&mut self, dwoutputnum: u32, phook: &::core::option::Option<IWMPlayerHook>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced5_Impl, const OFFSET: isize>() -> IWMReaderAdvanced5_Vtbl {
        unsafe extern "system" fn SetPlayerHook<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPlayerHook(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&phook)).into()
        }
        Self { base: IWMReaderAdvanced4_Vtbl::new::<Identity, Impl, OFFSET>(), SetPlayerHook: SetPlayerHook::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced5 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced2 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced3 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced6_Impl: Sized + IWMReaderAdvanced_Impl + IWMReaderAdvanced2_Impl + IWMReaderAdvanced3_Impl + IWMReaderAdvanced4_Impl + IWMReaderAdvanced5_Impl {
    fn SetProtectStreamSamples(&mut self, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced6_Impl, const OFFSET: isize>() -> IWMReaderAdvanced6_Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProtectStreamSamples(::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&dwcertificatetype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbinitializationvector), ::core::mem::transmute_copy(&pcbinitializationvector)).into()
        }
        Self { base: IWMReaderAdvanced5_Vtbl::new::<Identity, Impl, OFFSET>(), SetProtectStreamSamples: SetProtectStreamSamples::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced6 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced2 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced3 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced4 as ::windows::core::Interface>::IID || iid == &<IWMReaderAdvanced5 as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderAllocatorEx_Impl: Sized {
    fn AllocateForStreamEx(&mut self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForOutputEx(&mut self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMReaderAllocatorEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAllocatorEx_Impl, const OFFSET: isize>() -> IWMReaderAllocatorEx_Vtbl {
        unsafe extern "system" fn AllocateForStreamEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAllocatorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllocateForStreamEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForOutputEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAllocatorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllocateForOutputEx(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AllocateForStreamEx: AllocateForStreamEx::<Identity, Impl, OFFSET>,
            AllocateForOutputEx: AllocateForOutputEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAllocatorEx as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderCallback_Impl: Sized + IWMStatusCallback_Impl {
    fn OnSample(&mut self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: &::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMReaderCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallback_Impl, const OFFSET: isize>() -> IWMReaderCallback_Vtbl {
        unsafe extern "system" fn OnSample<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSample(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base: IWMStatusCallback_Vtbl::new::<Identity, Impl, OFFSET>(), OnSample: OnSample::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderCallback as ::windows::core::Interface>::IID || iid == &<IWMStatusCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderCallbackAdvanced_Impl: Sized {
    fn OnStreamSample(&mut self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: &::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OnTime(&mut self, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OnStreamSelection(&mut self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OnOutputPropsChanged(&mut self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForStream(&mut self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForOutput(&mut self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderCallbackAdvanced_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>() -> IWMReaderCallbackAdvanced_Vtbl {
        unsafe extern "system" fn OnStreamSample<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStreamSample(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnTime(::core::mem::transmute_copy(&cnscurrenttime), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnStreamSelection<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStreamSelection(::core::mem::transmute_copy(&wstreamcount), ::core::mem::transmute_copy(&pstreamnumbers), ::core::mem::transmute_copy(&pselections), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnOutputPropsChanged<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnOutputPropsChanged(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pmediatype), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllocateForStream(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForOutput<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllocateForOutput(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStreamSample: OnStreamSample::<Identity, Impl, OFFSET>,
            OnTime: OnTime::<Identity, Impl, OFFSET>,
            OnStreamSelection: OnStreamSelection::<Identity, Impl, OFFSET>,
            OnOutputPropsChanged: OnOutputPropsChanged::<Identity, Impl, OFFSET>,
            AllocateForStream: AllocateForStream::<Identity, Impl, OFFSET>,
            AllocateForOutput: AllocateForOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderCallbackAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderNetworkConfig_Impl: Sized {
    fn GetBufferingTime(&mut self) -> ::windows::core::Result<u64>;
    fn SetBufferingTime(&mut self, cnsbufferingtime: u64) -> ::windows::core::Result<()>;
    fn GetUDPPortRanges(&mut self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::Result<()>;
    fn SetUDPPortRanges(&mut self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::Result<()>;
    fn GetProxySettings(&mut self, pwszprotocol: super::super::Foundation::PWSTR) -> ::windows::core::Result<WMT_PROXY_SETTINGS>;
    fn SetProxySettings(&mut self, pwszprotocol: super::super::Foundation::PWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::Result<()>;
    fn GetProxyHostName(&mut self, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::core::Result<()>;
    fn SetProxyHostName(&mut self, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetProxyPort(&mut self, pwszprotocol: super::super::Foundation::PWSTR) -> ::windows::core::Result<u32>;
    fn SetProxyPort(&mut self, pwszprotocol: super::super::Foundation::PWSTR, dwport: u32) -> ::windows::core::Result<()>;
    fn GetProxyExceptionList(&mut self, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::Result<()>;
    fn SetProxyExceptionList(&mut self, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetProxyBypassForLocal(&mut self, pwszprotocol: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetProxyBypassForLocal(&mut self, pwszprotocol: super::super::Foundation::PWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetForceRerunAutoProxyDetection(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetForceRerunAutoProxyDetection(&mut self, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableMulticast(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableMulticast(&mut self, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableHTTP(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableHTTP(&mut self, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableUDP(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableUDP(&mut self, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableTCP(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableTCP(&mut self, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ResetProtocolRollover(&mut self) -> ::windows::core::Result<()>;
    fn GetConnectionBandwidth(&mut self) -> ::windows::core::Result<u32>;
    fn SetConnectionBandwidth(&mut self, dwconnectionbandwidth: u32) -> ::windows::core::Result<()>;
    fn GetNumProtocolsSupported(&mut self) -> ::windows::core::Result<u32>;
    fn GetSupportedProtocolName(&mut self, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::Result<()>;
    fn AddLoggingUrl(&mut self, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetLoggingUrl(&mut self, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()>;
    fn GetLoggingUrlCount(&mut self) -> ::windows::core::Result<u32>;
    fn ResetLoggingUrlList(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderNetworkConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>() -> IWMReaderNetworkConfig_Vtbl {
        unsafe extern "system" fn GetBufferingTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBufferingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsbufferingtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferingTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferingTime(::core::mem::transmute_copy(&cnsbufferingtime)).into()
        }
        unsafe extern "system" fn GetUDPPortRanges<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetUDPPortRanges(::core::mem::transmute_copy(&prangearray), ::core::mem::transmute_copy(&pcranges)).into()
        }
        unsafe extern "system" fn SetUDPPortRanges<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUDPPortRanges(::core::mem::transmute_copy(&prangearray), ::core::mem::transmute_copy(&cranges)).into()
        }
        unsafe extern "system" fn GetProxySettings<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProxySettings(::core::mem::transmute_copy(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproxysetting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxySettings(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&proxysetting)).into()
        }
        unsafe extern "system" fn GetProxyHostName<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProxyHostName(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pwszhostname), ::core::mem::transmute_copy(&pcchhostname)).into()
        }
        unsafe extern "system" fn SetProxyHostName<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxyHostName(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pwszhostname)).into()
        }
        unsafe extern "system" fn GetProxyPort<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProxyPort(::core::mem::transmute_copy(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyPort<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, dwport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxyPort(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&dwport)).into()
        }
        unsafe extern "system" fn GetProxyExceptionList<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetProxyExceptionList(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pwszexceptionlist), ::core::mem::transmute_copy(&pcchexceptionlist)).into()
        }
        unsafe extern "system" fn SetProxyExceptionList<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxyExceptionList(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pwszexceptionlist)).into()
        }
        unsafe extern "system" fn GetProxyBypassForLocal<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProxyBypassForLocal(::core::mem::transmute_copy(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfbypassforlocal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyBypassForLocal<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxyBypassForLocal(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&fbypassforlocal)).into()
        }
        unsafe extern "system" fn GetForceRerunAutoProxyDetection<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetForceRerunAutoProxyDetection() {
                ::core::result::Result::Ok(ok__) => {
                    *pfforcererundetection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceRerunAutoProxyDetection<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetForceRerunAutoProxyDetection(::core::mem::transmute_copy(&fforcererundetection)).into()
        }
        unsafe extern "system" fn GetEnableMulticast<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnableMulticast() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablemulticast = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableMulticast<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableMulticast(::core::mem::transmute_copy(&fenablemulticast)).into()
        }
        unsafe extern "system" fn GetEnableHTTP<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnableHTTP() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablehttp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableHTTP<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableHTTP(::core::mem::transmute_copy(&fenablehttp)).into()
        }
        unsafe extern "system" fn GetEnableUDP<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnableUDP() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenableudp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableUDP<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableUDP(::core::mem::transmute_copy(&fenableudp)).into()
        }
        unsafe extern "system" fn GetEnableTCP<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnableTCP() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabletcp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTCP<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableTCP(::core::mem::transmute_copy(&fenabletcp)).into()
        }
        unsafe extern "system" fn ResetProtocolRollover<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetProtocolRollover().into()
        }
        unsafe extern "system" fn GetConnectionBandwidth<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetConnectionBandwidth() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconnectionbandwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionBandwidth<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnectionBandwidth(::core::mem::transmute_copy(&dwconnectionbandwidth)).into()
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNumProtocolsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pcprotocols = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedProtocolName<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSupportedProtocolName(::core::mem::transmute_copy(&dwprotocolnum), ::core::mem::transmute_copy(&pwszprotocolname), ::core::mem::transmute_copy(&pcchprotocolname)).into()
        }
        unsafe extern "system" fn AddLoggingUrl<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddLoggingUrl(::core::mem::transmute_copy(&pwszurl)).into()
        }
        unsafe extern "system" fn GetLoggingUrl<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLoggingUrl(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into()
        }
        unsafe extern "system" fn GetLoggingUrlCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLoggingUrlCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwurlcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetLoggingUrlList<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetLoggingUrlList().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBufferingTime: GetBufferingTime::<Identity, Impl, OFFSET>,
            SetBufferingTime: SetBufferingTime::<Identity, Impl, OFFSET>,
            GetUDPPortRanges: GetUDPPortRanges::<Identity, Impl, OFFSET>,
            SetUDPPortRanges: SetUDPPortRanges::<Identity, Impl, OFFSET>,
            GetProxySettings: GetProxySettings::<Identity, Impl, OFFSET>,
            SetProxySettings: SetProxySettings::<Identity, Impl, OFFSET>,
            GetProxyHostName: GetProxyHostName::<Identity, Impl, OFFSET>,
            SetProxyHostName: SetProxyHostName::<Identity, Impl, OFFSET>,
            GetProxyPort: GetProxyPort::<Identity, Impl, OFFSET>,
            SetProxyPort: SetProxyPort::<Identity, Impl, OFFSET>,
            GetProxyExceptionList: GetProxyExceptionList::<Identity, Impl, OFFSET>,
            SetProxyExceptionList: SetProxyExceptionList::<Identity, Impl, OFFSET>,
            GetProxyBypassForLocal: GetProxyBypassForLocal::<Identity, Impl, OFFSET>,
            SetProxyBypassForLocal: SetProxyBypassForLocal::<Identity, Impl, OFFSET>,
            GetForceRerunAutoProxyDetection: GetForceRerunAutoProxyDetection::<Identity, Impl, OFFSET>,
            SetForceRerunAutoProxyDetection: SetForceRerunAutoProxyDetection::<Identity, Impl, OFFSET>,
            GetEnableMulticast: GetEnableMulticast::<Identity, Impl, OFFSET>,
            SetEnableMulticast: SetEnableMulticast::<Identity, Impl, OFFSET>,
            GetEnableHTTP: GetEnableHTTP::<Identity, Impl, OFFSET>,
            SetEnableHTTP: SetEnableHTTP::<Identity, Impl, OFFSET>,
            GetEnableUDP: GetEnableUDP::<Identity, Impl, OFFSET>,
            SetEnableUDP: SetEnableUDP::<Identity, Impl, OFFSET>,
            GetEnableTCP: GetEnableTCP::<Identity, Impl, OFFSET>,
            SetEnableTCP: SetEnableTCP::<Identity, Impl, OFFSET>,
            ResetProtocolRollover: ResetProtocolRollover::<Identity, Impl, OFFSET>,
            GetConnectionBandwidth: GetConnectionBandwidth::<Identity, Impl, OFFSET>,
            SetConnectionBandwidth: SetConnectionBandwidth::<Identity, Impl, OFFSET>,
            GetNumProtocolsSupported: GetNumProtocolsSupported::<Identity, Impl, OFFSET>,
            GetSupportedProtocolName: GetSupportedProtocolName::<Identity, Impl, OFFSET>,
            AddLoggingUrl: AddLoggingUrl::<Identity, Impl, OFFSET>,
            GetLoggingUrl: GetLoggingUrl::<Identity, Impl, OFFSET>,
            GetLoggingUrlCount: GetLoggingUrlCount::<Identity, Impl, OFFSET>,
            ResetLoggingUrlList: ResetLoggingUrlList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderNetworkConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderNetworkConfig2_Impl: Sized + IWMReaderNetworkConfig_Impl {
    fn GetEnableContentCaching(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableContentCaching(&mut self, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableFastCache(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableFastCache(&mut self, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAcceleratedStreamingDuration(&mut self) -> ::windows::core::Result<u64>;
    fn SetAcceleratedStreamingDuration(&mut self, cnsaccelduration: u64) -> ::windows::core::Result<()>;
    fn GetAutoReconnectLimit(&mut self) -> ::windows::core::Result<u32>;
    fn SetAutoReconnectLimit(&mut self, dwautoreconnectlimit: u32) -> ::windows::core::Result<()>;
    fn GetEnableResends(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableResends(&mut self, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableThinning(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableThinning(&mut self, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMaxNetPacketSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderNetworkConfig2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>() -> IWMReaderNetworkConfig2_Vtbl {
        unsafe extern "system" fn GetEnableContentCaching<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnableContentCaching() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablecontentcaching = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableContentCaching<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableContentCaching(::core::mem::transmute_copy(&fenablecontentcaching)).into()
        }
        unsafe extern "system" fn GetEnableFastCache<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnableFastCache() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablefastcache = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableFastCache<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableFastCache(::core::mem::transmute_copy(&fenablefastcache)).into()
        }
        unsafe extern "system" fn GetAcceleratedStreamingDuration<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAcceleratedStreamingDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsaccelduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceleratedStreamingDuration<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAcceleratedStreamingDuration(::core::mem::transmute_copy(&cnsaccelduration)).into()
        }
        unsafe extern "system" fn GetAutoReconnectLimit<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAutoReconnectLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwautoreconnectlimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnectLimit<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoReconnectLimit(::core::mem::transmute_copy(&dwautoreconnectlimit)).into()
        }
        unsafe extern "system" fn GetEnableResends<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnableResends() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenableresends = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableResends<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableResends(::core::mem::transmute_copy(&fenableresends)).into()
        }
        unsafe extern "system" fn GetEnableThinning<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEnableThinning() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablethinning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableThinning<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnableThinning(::core::mem::transmute_copy(&fenablethinning)).into()
        }
        unsafe extern "system" fn GetMaxNetPacketSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxNetPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxnetpacketsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMReaderNetworkConfig_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetEnableContentCaching: GetEnableContentCaching::<Identity, Impl, OFFSET>,
            SetEnableContentCaching: SetEnableContentCaching::<Identity, Impl, OFFSET>,
            GetEnableFastCache: GetEnableFastCache::<Identity, Impl, OFFSET>,
            SetEnableFastCache: SetEnableFastCache::<Identity, Impl, OFFSET>,
            GetAcceleratedStreamingDuration: GetAcceleratedStreamingDuration::<Identity, Impl, OFFSET>,
            SetAcceleratedStreamingDuration: SetAcceleratedStreamingDuration::<Identity, Impl, OFFSET>,
            GetAutoReconnectLimit: GetAutoReconnectLimit::<Identity, Impl, OFFSET>,
            SetAutoReconnectLimit: SetAutoReconnectLimit::<Identity, Impl, OFFSET>,
            GetEnableResends: GetEnableResends::<Identity, Impl, OFFSET>,
            SetEnableResends: SetEnableResends::<Identity, Impl, OFFSET>,
            GetEnableThinning: GetEnableThinning::<Identity, Impl, OFFSET>,
            SetEnableThinning: SetEnableThinning::<Identity, Impl, OFFSET>,
            GetMaxNetPacketSize: GetMaxNetPacketSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderNetworkConfig2 as ::windows::core::Interface>::IID || iid == &<IWMReaderNetworkConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderPlaylistBurn_Impl: Sized {
    fn InitPlaylistBurn(&mut self, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: &::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetInitResults(&mut self, cfiles: u32) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn EndPlaylistBurn(&mut self, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderPlaylistBurn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>() -> IWMReaderPlaylistBurn_Vtbl {
        unsafe extern "system" fn InitPlaylistBurn<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InitPlaylistBurn(::core::mem::transmute_copy(&cfiles), ::core::mem::transmute_copy(&ppwszfilenames), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn GetInitResults<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInitResults(::core::mem::transmute_copy(&cfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrstati = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn EndPlaylistBurn<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndPlaylistBurn(::core::mem::transmute_copy(&hrburnresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InitPlaylistBurn: InitPlaylistBurn::<Identity, Impl, OFFSET>,
            GetInitResults: GetInitResults::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            EndPlaylistBurn: EndPlaylistBurn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderPlaylistBurn as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderStreamClock_Impl: Sized {
    fn GetTime(&mut self, pcnsnow: *const u64) -> ::windows::core::Result<()>;
    fn SetTimer(&mut self, cnswhen: u64, pvparam: *const ::core::ffi::c_void) -> ::windows::core::Result<u32>;
    fn KillTimer(&mut self, dwtimerid: u32) -> ::windows::core::Result<()>;
}
impl IWMReaderStreamClock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderStreamClock_Impl, const OFFSET: isize>() -> IWMReaderStreamClock_Vtbl {
        unsafe extern "system" fn GetTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderStreamClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTime(::core::mem::transmute_copy(&pcnsnow)).into()
        }
        unsafe extern "system" fn SetTimer<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderStreamClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetTimer(::core::mem::transmute_copy(&cnswhen), ::core::mem::transmute_copy(&pvparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwtimerid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KillTimer<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderStreamClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KillTimer(::core::mem::transmute_copy(&dwtimerid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTime: GetTime::<Identity, Impl, OFFSET>,
            SetTimer: SetTimer::<Identity, Impl, OFFSET>,
            KillTimer: KillTimer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderStreamClock as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderTimecode_Impl: Sized {
    fn GetTimecodeRangeCount(&mut self, wstreamnum: u16) -> ::windows::core::Result<u16>;
    fn GetTimecodeRangeBounds(&mut self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::Result<()>;
}
impl IWMReaderTimecode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderTimecode_Impl, const OFFSET: isize>() -> IWMReaderTimecode_Vtbl {
        unsafe extern "system" fn GetTimecodeRangeCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderTimecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTimecodeRangeCount(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwrangecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimecodeRangeBounds<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderTimecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetTimecodeRangeBounds(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&wrangenum), ::core::mem::transmute_copy(&pstarttimecode), ::core::mem::transmute_copy(&pendtimecode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTimecodeRangeCount: GetTimecodeRangeCount::<Identity, Impl, OFFSET>,
            GetTimecodeRangeBounds: GetTimecodeRangeBounds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderTimecode as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderTypeNegotiation_Impl: Sized {
    fn TryOutputProps(&mut self, dwoutputnum: u32, poutput: &::core::option::Option<IWMOutputMediaProps>) -> ::windows::core::Result<()>;
}
impl IWMReaderTypeNegotiation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderTypeNegotiation_Impl, const OFFSET: isize>() -> IWMReaderTypeNegotiation_Vtbl {
        unsafe extern "system" fn TryOutputProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderTypeNegotiation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TryOutputProps(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&poutput)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), TryOutputProps: TryOutputProps::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderTypeNegotiation as ::windows::core::Interface>::IID
    }
}
pub trait IWMRegisterCallback_Impl: Sized {
    fn Advise(&mut self, pcallback: &::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Unadvise(&mut self, pcallback: &::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMRegisterCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisterCallback_Impl, const OFFSET: isize>() -> IWMRegisterCallback_Vtbl {
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisterCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Advise(::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisterCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unadvise(::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMRegisterCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMRegisteredDevice_Impl: Sized {
    fn GetDeviceSerialNumber(&mut self) -> ::windows::core::Result<DRM_VAL16>;
    fn GetDeviceCertificate(&mut self) -> ::windows::core::Result<INSSBuffer>;
    fn GetDeviceType(&mut self) -> ::windows::core::Result<u32>;
    fn GetAttributeCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAttributeByIndex(&mut self, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetAttributeByName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAttributeByName(&mut self, bstrname: &super::super::Foundation::BSTR, bstrvalue: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Approve(&mut self, fapprove: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsValid(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsApproved(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsWmdrmCompliant(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOpened(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMRegisteredDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>() -> IWMRegisteredDevice_Vtbl {
        unsafe extern "system" fn GetDeviceSerialNumber<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceSerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pserialnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertificate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcertificate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceType<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAttributeByIndex(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrvalue)).into()
        }
        unsafe extern "system" fn GetAttributeByName<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAttributeByName(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeByName<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAttributeByName(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn Approve<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Approve(::core::mem::transmute_copy(&fapprove)).into()
        }
        unsafe extern "system" fn IsValid<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *pfvalid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApproved<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsApproved() {
                ::core::result::Result::Ok(ok__) => {
                    *pfapproved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWmdrmCompliant<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsWmdrmCompliant() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcompliant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpened<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsOpened() {
                ::core::result::Result::Ok(ok__) => {
                    *pfopened = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDeviceSerialNumber: GetDeviceSerialNumber::<Identity, Impl, OFFSET>,
            GetDeviceCertificate: GetDeviceCertificate::<Identity, Impl, OFFSET>,
            GetDeviceType: GetDeviceType::<Identity, Impl, OFFSET>,
            GetAttributeCount: GetAttributeCount::<Identity, Impl, OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Identity, Impl, OFFSET>,
            GetAttributeByName: GetAttributeByName::<Identity, Impl, OFFSET>,
            SetAttributeByName: SetAttributeByName::<Identity, Impl, OFFSET>,
            Approve: Approve::<Identity, Impl, OFFSET>,
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            IsApproved: IsApproved::<Identity, Impl, OFFSET>,
            IsWmdrmCompliant: IsWmdrmCompliant::<Identity, Impl, OFFSET>,
            IsOpened: IsOpened::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMRegisteredDevice as ::windows::core::Interface>::IID
    }
}
pub trait IWMSBufferAllocator_Impl: Sized {
    fn AllocateBuffer(&mut self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer>;
    fn AllocatePageSizeBuffer(&mut self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer>;
}
impl IWMSBufferAllocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSBufferAllocator_Impl, const OFFSET: isize>() -> IWMSBufferAllocator_Vtbl {
        unsafe extern "system" fn AllocateBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWMSBufferAllocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllocateBuffer(::core::mem::transmute_copy(&dwmaxbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocatePageSizeBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IWMSBufferAllocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllocatePageSizeBuffer(::core::mem::transmute_copy(&dwmaxbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AllocateBuffer: AllocateBuffer::<Identity, Impl, OFFSET>,
            AllocatePageSizeBuffer: AllocatePageSizeBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSBufferAllocator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource_Impl: Sized {
    fn Initialize(&mut self, psharednamespace: &::core::option::Option<::windows::core::IUnknown>, pnamespacenode: &::core::option::Option<::windows::core::IUnknown>, pnetsourcecreator: &::core::option::Option<INSNetSourceCreator>, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetSourceCreator(&mut self) -> ::windows::core::Result<INSNetSourceCreator>;
    fn SetCredentials(&mut self, bstrrealm: &super::super::Foundation::BSTR, bstrname: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCredentials(&mut self, bstrrealm: &super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DeleteCredentials(&mut self, bstrrealm: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetCredentialFlags(&mut self) -> ::windows::core::Result<u32>;
    fn SetCredentialFlags(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn FindProxyForURL(&mut self, bstrprotocol: &super::super::Foundation::BSTR, bstrhost: &super::super::Foundation::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()>;
    fn RegisterProxyFailure(&mut self, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::Result<()>;
    fn ShutdownProxyContext(&mut self, dwproxycontext: u32) -> ::windows::core::Result<()>;
    fn IsUsingIE(&mut self, dwproxycontext: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>() -> IWMSInternalAdminNetSource_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: ::windows::core::RawPtr, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&psharednamespace), ::core::mem::transmute(&pnamespacenode), ::core::mem::transmute(&pnetsourcecreator), ::core::mem::transmute_copy(&fembeddedinserver)).into()
        }
        unsafe extern "system" fn GetNetSourceCreator<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetSourceCreator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetsourcecreator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood)).into()
        }
        unsafe extern "system" fn GetCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCredentials(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        unsafe extern "system" fn DeleteCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteCredentials(::core::mem::transmute_copy(&bstrrealm)).into()
        }
        unsafe extern "system" fn GetCredentialFlags<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCredentialFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *lpdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialFlags<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCredentialFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindProxyForURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindProxyForURL(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&bstrhost), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pdwproxycontext)).into()
        }
        unsafe extern "system" fn RegisterProxyFailure<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterProxyFailure(::core::mem::transmute_copy(&hrparam), ::core::mem::transmute_copy(&dwproxycontext)).into()
        }
        unsafe extern "system" fn ShutdownProxyContext<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShutdownProxyContext(::core::mem::transmute_copy(&dwproxycontext)).into()
        }
        unsafe extern "system" fn IsUsingIE<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUsingIE(::core::mem::transmute_copy(&dwproxycontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfisusingie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetNetSourceCreator: GetNetSourceCreator::<Identity, Impl, OFFSET>,
            SetCredentials: SetCredentials::<Identity, Impl, OFFSET>,
            GetCredentials: GetCredentials::<Identity, Impl, OFFSET>,
            DeleteCredentials: DeleteCredentials::<Identity, Impl, OFFSET>,
            GetCredentialFlags: GetCredentialFlags::<Identity, Impl, OFFSET>,
            SetCredentialFlags: SetCredentialFlags::<Identity, Impl, OFFSET>,
            FindProxyForURL: FindProxyForURL::<Identity, Impl, OFFSET>,
            RegisterProxyFailure: RegisterProxyFailure::<Identity, Impl, OFFSET>,
            ShutdownProxyContext: ShutdownProxyContext::<Identity, Impl, OFFSET>,
            IsUsingIE: IsUsingIE::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource2_Impl: Sized {
    fn SetCredentialsEx(&mut self, bstrrealm: &super::super::Foundation::BSTR, bstrurl: &super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCredentialsEx(&mut self, bstrrealm: &super::super::Foundation::BSTR, bstrurl: &super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DeleteCredentialsEx(&mut self, bstrrealm: &super::super::Foundation::BSTR, bstrurl: &super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FindProxyForURLEx(&mut self, bstrprotocol: &super::super::Foundation::BSTR, bstrhost: &super::super::Foundation::BSTR, bstrurl: &super::super::Foundation::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>() -> IWMSInternalAdminNetSource2_Vtbl {
        unsafe extern "system" fn SetCredentialsEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCredentialsEx(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood)).into()
        }
        unsafe extern "system" fn GetCredentialsEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCredentialsEx(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&pdwurlpolicy), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        unsafe extern "system" fn DeleteCredentialsEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteCredentialsEx(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy)).into()
        }
        unsafe extern "system" fn FindProxyForURLEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindProxyForURLEx(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&bstrhost), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pdwproxycontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetCredentialsEx: SetCredentialsEx::<Identity, Impl, OFFSET>,
            GetCredentialsEx: GetCredentialsEx::<Identity, Impl, OFFSET>,
            DeleteCredentialsEx: DeleteCredentialsEx::<Identity, Impl, OFFSET>,
            FindProxyForURLEx: FindProxyForURLEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource3_Impl: Sized + IWMSInternalAdminNetSource2_Impl {
    fn GetNetSourceCreator2(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn FindProxyForURLEx2(&mut self, bstrprotocol: &super::super::Foundation::BSTR, bstrhost: &super::super::Foundation::BSTR, bstrurl: &super::super::Foundation::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::Result<()>;
    fn RegisterProxyFailure2(&mut self, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::Result<()>;
    fn ShutdownProxyContext2(&mut self, qwproxycontext: u64) -> ::windows::core::Result<()>;
    fn IsUsingIE2(&mut self, qwproxycontext: u64) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetCredentialsEx2(&mut self, bstrrealm: &super::super::Foundation::BSTR, bstrurl: &super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCredentialsEx2(&mut self, bstrrealm: &super::super::Foundation::BSTR, bstrurl: &super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>() -> IWMSInternalAdminNetSource3_Vtbl {
        unsafe extern "system" fn GetNetSourceCreator2<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetSourceCreator2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetsourcecreator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindProxyForURLEx2<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindProxyForURLEx2(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&bstrhost), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pqwproxycontext)).into()
        }
        unsafe extern "system" fn RegisterProxyFailure2<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterProxyFailure2(::core::mem::transmute_copy(&hrparam), ::core::mem::transmute_copy(&qwproxycontext)).into()
        }
        unsafe extern "system" fn ShutdownProxyContext2<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ShutdownProxyContext2(::core::mem::transmute_copy(&qwproxycontext)).into()
        }
        unsafe extern "system" fn IsUsingIE2<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUsingIE2(::core::mem::transmute_copy(&qwproxycontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfisusingie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialsEx2<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCredentialsEx2(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood), ::core::mem::transmute_copy(&fcleartextauthentication)).into()
        }
        unsafe extern "system" fn GetCredentialsEx2<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCredentialsEx2(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&fcleartextauthentication), ::core::mem::transmute_copy(&pdwurlpolicy), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        Self {
            base: IWMSInternalAdminNetSource2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNetSourceCreator2: GetNetSourceCreator2::<Identity, Impl, OFFSET>,
            FindProxyForURLEx2: FindProxyForURLEx2::<Identity, Impl, OFFSET>,
            RegisterProxyFailure2: RegisterProxyFailure2::<Identity, Impl, OFFSET>,
            ShutdownProxyContext2: ShutdownProxyContext2::<Identity, Impl, OFFSET>,
            IsUsingIE2: IsUsingIE2::<Identity, Impl, OFFSET>,
            SetCredentialsEx2: SetCredentialsEx2::<Identity, Impl, OFFSET>,
            GetCredentialsEx2: GetCredentialsEx2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource3 as ::windows::core::Interface>::IID || iid == &<IWMSInternalAdminNetSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSecureChannel_Impl: Sized + IWMAuthorizer_Impl {
    fn WMSC_AddCertificate(&mut self, pcert: &::core::option::Option<IWMAuthorizer>) -> ::windows::core::Result<()>;
    fn WMSC_AddSignature(&mut self, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::Result<()>;
    fn WMSC_Connect(&mut self, potherside: &::core::option::Option<IWMSecureChannel>) -> ::windows::core::Result<()>;
    fn WMSC_IsConnected(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn WMSC_Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn WMSC_GetValidCertificate(&mut self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::Result<()>;
    fn WMSC_Encrypt(&mut self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()>;
    fn WMSC_Decrypt(&mut self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()>;
    fn WMSC_Lock(&mut self) -> ::windows::core::Result<()>;
    fn WMSC_Unlock(&mut self) -> ::windows::core::Result<()>;
    fn WMSC_SetSharedData(&mut self, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSecureChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>() -> IWMSecureChannel_Vtbl {
        unsafe extern "system" fn WMSC_AddCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcert: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_AddCertificate(::core::mem::transmute(&pcert)).into()
        }
        unsafe extern "system" fn WMSC_AddSignature<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_AddSignature(::core::mem::transmute_copy(&pbcertsig), ::core::mem::transmute_copy(&cbcertsig)).into()
        }
        unsafe extern "system" fn WMSC_Connect<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, potherside: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_Connect(::core::mem::transmute(&potherside)).into()
        }
        unsafe extern "system" fn WMSC_IsConnected<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).WMSC_IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_Disconnect().into()
        }
        unsafe extern "system" fn WMSC_GetValidCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_GetValidCertificate(::core::mem::transmute_copy(&ppbcertificate), ::core::mem::transmute_copy(&pdwsignature)).into()
        }
        unsafe extern "system" fn WMSC_Encrypt<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_Encrypt(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn WMSC_Decrypt<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_Decrypt(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn WMSC_Lock<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_Lock().into()
        }
        unsafe extern "system" fn WMSC_Unlock<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_Unlock().into()
        }
        unsafe extern "system" fn WMSC_SetSharedData<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WMSC_SetSharedData(::core::mem::transmute_copy(&dwcertindex), ::core::mem::transmute_copy(&pbshareddata)).into()
        }
        Self {
            base: IWMAuthorizer_Vtbl::new::<Identity, Impl, OFFSET>(),
            WMSC_AddCertificate: WMSC_AddCertificate::<Identity, Impl, OFFSET>,
            WMSC_AddSignature: WMSC_AddSignature::<Identity, Impl, OFFSET>,
            WMSC_Connect: WMSC_Connect::<Identity, Impl, OFFSET>,
            WMSC_IsConnected: WMSC_IsConnected::<Identity, Impl, OFFSET>,
            WMSC_Disconnect: WMSC_Disconnect::<Identity, Impl, OFFSET>,
            WMSC_GetValidCertificate: WMSC_GetValidCertificate::<Identity, Impl, OFFSET>,
            WMSC_Encrypt: WMSC_Encrypt::<Identity, Impl, OFFSET>,
            WMSC_Decrypt: WMSC_Decrypt::<Identity, Impl, OFFSET>,
            WMSC_Lock: WMSC_Lock::<Identity, Impl, OFFSET>,
            WMSC_Unlock: WMSC_Unlock::<Identity, Impl, OFFSET>,
            WMSC_SetSharedData: WMSC_SetSharedData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSecureChannel as ::windows::core::Interface>::IID || iid == &<IWMAuthorizer as ::windows::core::Interface>::IID
    }
}
pub trait IWMStatusCallback_Impl: Sized {
    fn OnStatus(&mut self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMStatusCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStatusCallback_Impl, const OFFSET: isize>() -> IWMStatusCallback_Vtbl {
        unsafe extern "system" fn OnStatus<Identity: ::windows::core::IUnknownImpl, Impl: IWMStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnStatus(::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnStatus: OnStatus::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStatusCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamConfig_Impl: Sized {
    fn GetStreamType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetStreamNumber(&mut self) -> ::windows::core::Result<u16>;
    fn SetStreamNumber(&mut self, wstreamnum: u16) -> ::windows::core::Result<()>;
    fn GetStreamName(&mut self, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()>;
    fn SetStreamName(&mut self, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetConnectionName(&mut self, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()>;
    fn SetConnectionName(&mut self, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetBitrate(&mut self) -> ::windows::core::Result<u32>;
    fn SetBitrate(&mut self, pdwbitrate: u32) -> ::windows::core::Result<()>;
    fn GetBufferWindow(&mut self) -> ::windows::core::Result<u32>;
    fn SetBufferWindow(&mut self, msbufferwindow: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>() -> IWMStreamConfig_Vtbl {
        unsafe extern "system" fn GetStreamType<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidstreamtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumber<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pwstreamnum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamNumber<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStreamNumber(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn GetStreamName<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreamName(::core::mem::transmute_copy(&pwszstreamname), ::core::mem::transmute_copy(&pcchstreamname)).into()
        }
        unsafe extern "system" fn SetStreamName<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStreamName(::core::mem::transmute_copy(&pwszstreamname)).into()
        }
        unsafe extern "system" fn GetConnectionName<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetConnectionName(::core::mem::transmute_copy(&pwszinputname), ::core::mem::transmute_copy(&pcchinputname)).into()
        }
        unsafe extern "system" fn SetConnectionName<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetConnectionName(::core::mem::transmute_copy(&pwszinputname)).into()
        }
        unsafe extern "system" fn GetBitrate<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwbitrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitrate<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBitrate(::core::mem::transmute_copy(&pdwbitrate)).into()
        }
        unsafe extern "system" fn GetBufferWindow<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBufferWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *pmsbufferwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferWindow<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBufferWindow(::core::mem::transmute_copy(&msbufferwindow)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStreamType: GetStreamType::<Identity, Impl, OFFSET>,
            GetStreamNumber: GetStreamNumber::<Identity, Impl, OFFSET>,
            SetStreamNumber: SetStreamNumber::<Identity, Impl, OFFSET>,
            GetStreamName: GetStreamName::<Identity, Impl, OFFSET>,
            SetStreamName: SetStreamName::<Identity, Impl, OFFSET>,
            GetConnectionName: GetConnectionName::<Identity, Impl, OFFSET>,
            SetConnectionName: SetConnectionName::<Identity, Impl, OFFSET>,
            GetBitrate: GetBitrate::<Identity, Impl, OFFSET>,
            SetBitrate: SetBitrate::<Identity, Impl, OFFSET>,
            GetBufferWindow: GetBufferWindow::<Identity, Impl, OFFSET>,
            SetBufferWindow: SetBufferWindow::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamConfig2_Impl: Sized + IWMStreamConfig_Impl {
    fn GetTransportType(&mut self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE>;
    fn SetTransportType(&mut self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()>;
    fn AddDataUnitExtension(&mut self, guidextensionsystemid: &::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::Result<()>;
    fn GetDataUnitExtensionCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetDataUnitExtension(&mut self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()>;
    fn RemoveAllDataUnitExtensions(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamConfig2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>() -> IWMStreamConfig2_Vtbl {
        unsafe extern "system" fn GetTransportType<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTransportType() {
                ::core::result::Result::Ok(ok__) => {
                    *pntransporttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportType<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransportType(::core::mem::transmute_copy(&ntransporttype)).into()
        }
        unsafe extern "system" fn AddDataUnitExtension<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDataUnitExtension(::core::mem::transmute_copy(&guidextensionsystemid), ::core::mem::transmute_copy(&cbextensiondatasize), ::core::mem::transmute_copy(&pbextensionsysteminfo), ::core::mem::transmute_copy(&cbextensionsysteminfo)).into()
        }
        unsafe extern "system" fn GetDataUnitExtensionCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDataUnitExtensionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcdataunitextensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataUnitExtension<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDataUnitExtension(::core::mem::transmute_copy(&wdataunitextensionnumber), ::core::mem::transmute_copy(&pguidextensionsystemid), ::core::mem::transmute_copy(&pcbextensiondatasize), ::core::mem::transmute_copy(&pbextensionsysteminfo), ::core::mem::transmute_copy(&pcbextensionsysteminfo)).into()
        }
        unsafe extern "system" fn RemoveAllDataUnitExtensions<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAllDataUnitExtensions().into()
        }
        Self {
            base: IWMStreamConfig_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTransportType: GetTransportType::<Identity, Impl, OFFSET>,
            SetTransportType: SetTransportType::<Identity, Impl, OFFSET>,
            AddDataUnitExtension: AddDataUnitExtension::<Identity, Impl, OFFSET>,
            GetDataUnitExtensionCount: GetDataUnitExtensionCount::<Identity, Impl, OFFSET>,
            GetDataUnitExtension: GetDataUnitExtension::<Identity, Impl, OFFSET>,
            RemoveAllDataUnitExtensions: RemoveAllDataUnitExtensions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamConfig2 as ::windows::core::Interface>::IID || iid == &<IWMStreamConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamConfig3_Impl: Sized + IWMStreamConfig_Impl + IWMStreamConfig2_Impl {
    fn GetLanguage(&mut self, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()>;
    fn SetLanguage(&mut self, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamConfig3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig3_Impl, const OFFSET: isize>() -> IWMStreamConfig3_Vtbl {
        unsafe extern "system" fn GetLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetLanguage(::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLanguage(::core::mem::transmute_copy(&pwszlanguagestring)).into()
        }
        Self {
            base: IWMStreamConfig2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamConfig3 as ::windows::core::Interface>::IID || iid == &<IWMStreamConfig as ::windows::core::Interface>::IID || iid == &<IWMStreamConfig2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMStreamList_Impl: Sized {
    fn GetStreams(&mut self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()>;
    fn AddStream(&mut self, wstreamnum: u16) -> ::windows::core::Result<()>;
    fn RemoveStream(&mut self, wstreamnum: u16) -> ::windows::core::Result<()>;
}
impl IWMStreamList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamList_Impl, const OFFSET: isize>() -> IWMStreamList_Vtbl {
        unsafe extern "system" fn GetStreams<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStreams(::core::mem::transmute_copy(&pwstreamnumarray), ::core::mem::transmute_copy(&pcstreams)).into()
        }
        unsafe extern "system" fn AddStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStream(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStreams: GetStreams::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamPrioritization_Impl: Sized {
    fn GetPriorityRecords(&mut self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::Result<()>;
    fn SetPriorityRecords(&mut self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamPrioritization_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamPrioritization_Impl, const OFFSET: isize>() -> IWMStreamPrioritization_Vtbl {
        unsafe extern "system" fn GetPriorityRecords<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPriorityRecords(::core::mem::transmute_copy(&precordarray), ::core::mem::transmute_copy(&pcrecords)).into()
        }
        unsafe extern "system" fn SetPriorityRecords<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriorityRecords(::core::mem::transmute_copy(&precordarray), ::core::mem::transmute_copy(&crecords)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPriorityRecords: GetPriorityRecords::<Identity, Impl, OFFSET>,
            SetPriorityRecords: SetPriorityRecords::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamPrioritization as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMSyncReader_Impl: Sized {
    fn Open(&mut self, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn SetRange(&mut self, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::Result<()>;
    fn SetRangeByFrame(&mut self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<()>;
    fn GetNextSample(&mut self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::Result<()>;
    fn SetStreamsSelected(&mut self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()>;
    fn GetStreamSelected(&mut self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION>;
    fn SetReadStreamSamples(&mut self, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReadStreamSamples(&mut self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetOutputSetting(&mut self, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetOutputSetting(&mut self, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn GetOutputCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetOutputProps(&mut self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(&mut self, dwoutputnum: u32, poutput: &::core::option::Option<IWMOutputMediaProps>) -> ::windows::core::Result<()>;
    fn GetOutputFormatCount(&mut self, dwoutputnum: u32) -> ::windows::core::Result<u32>;
    fn GetOutputFormat(&mut self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn GetOutputNumberForStream(&mut self, wstreamnum: u16) -> ::windows::core::Result<u32>;
    fn GetStreamNumberForOutput(&mut self, dwoutputnum: u32) -> ::windows::core::Result<u16>;
    fn GetMaxOutputSampleSize(&mut self, dwoutput: u32) -> ::windows::core::Result<u32>;
    fn GetMaxStreamSampleSize(&mut self, wstream: u16) -> ::windows::core::Result<u32>;
    fn OpenStream(&mut self, pstream: &::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMSyncReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>() -> IWMSyncReader_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn SetRange<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRange(::core::mem::transmute_copy(&cnsstarttime), ::core::mem::transmute_copy(&cnsduration)).into()
        }
        unsafe extern "system" fn SetRangeByFrame<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRangeByFrame(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&qwframenumber), ::core::mem::transmute_copy(&cframestoread)).into()
        }
        unsafe extern "system" fn GetNextSample<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut ::windows::core::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNextSample(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&ppsample), ::core::mem::transmute_copy(&pcnssampletime), ::core::mem::transmute_copy(&pcnsduration), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwoutputnum), ::core::mem::transmute_copy(&pwstreamnum)).into()
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStreamsSelected(::core::mem::transmute_copy(&cstreamcount), ::core::mem::transmute_copy(&pwstreamnumbers), ::core::mem::transmute_copy(&pselections)).into()
        }
        unsafe extern "system" fn GetStreamSelected<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamSelected(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pselection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadStreamSamples<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReadStreamSamples(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&fcompressed)).into()
        }
        unsafe extern "system" fn GetReadStreamSamples<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReadStreamSamples(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfcompressed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSetting<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetOutputSetting<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcoutputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputProps(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputProps(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputFormatCount(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputFormat(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&dwformatnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputNumberForStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutputNumberForStream(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwoutputnum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumberForOutput<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStreamNumberForOutput(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwstreamnum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxOutputSampleSize(::core::mem::transmute_copy(&dwoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxStreamSampleSize(::core::mem::transmute_copy(&wstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OpenStream(::core::mem::transmute(&pstream)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            SetRange: SetRange::<Identity, Impl, OFFSET>,
            SetRangeByFrame: SetRangeByFrame::<Identity, Impl, OFFSET>,
            GetNextSample: GetNextSample::<Identity, Impl, OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Identity, Impl, OFFSET>,
            GetStreamSelected: GetStreamSelected::<Identity, Impl, OFFSET>,
            SetReadStreamSamples: SetReadStreamSamples::<Identity, Impl, OFFSET>,
            GetReadStreamSamples: GetReadStreamSamples::<Identity, Impl, OFFSET>,
            GetOutputSetting: GetOutputSetting::<Identity, Impl, OFFSET>,
            SetOutputSetting: SetOutputSetting::<Identity, Impl, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, Impl, OFFSET>,
            GetOutputProps: GetOutputProps::<Identity, Impl, OFFSET>,
            SetOutputProps: SetOutputProps::<Identity, Impl, OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Identity, Impl, OFFSET>,
            GetOutputFormat: GetOutputFormat::<Identity, Impl, OFFSET>,
            GetOutputNumberForStream: GetOutputNumberForStream::<Identity, Impl, OFFSET>,
            GetStreamNumberForOutput: GetStreamNumberForOutput::<Identity, Impl, OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Identity, Impl, OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Identity, Impl, OFFSET>,
            OpenStream: OpenStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSyncReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMSyncReader2_Impl: Sized + IWMSyncReader_Impl {
    fn SetRangeByTimecode(&mut self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::Result<()>;
    fn SetRangeByFrameEx(&mut self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<u64>;
    fn SetAllocateForOutput(&mut self, dwoutputnum: u32, pallocator: &::core::option::Option<IWMReaderAllocatorEx>) -> ::windows::core::Result<()>;
    fn GetAllocateForOutput(&mut self, dwoutputnum: u32) -> ::windows::core::Result<IWMReaderAllocatorEx>;
    fn SetAllocateForStream(&mut self, wstreamnum: u16, pallocator: &::core::option::Option<IWMReaderAllocatorEx>) -> ::windows::core::Result<()>;
    fn GetAllocateForStream(&mut self, dwsreamnum: u16) -> ::windows::core::Result<IWMReaderAllocatorEx>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMSyncReader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader2_Impl, const OFFSET: isize>() -> IWMSyncReader2_Vtbl {
        unsafe extern "system" fn SetRangeByTimecode<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRangeByTimecode(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pstart), ::core::mem::transmute_copy(&pend)).into()
        }
        unsafe extern "system" fn SetRangeByFrameEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetRangeByFrameEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&qwframenumber), ::core::mem::transmute_copy(&cframestoread)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsstarttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pallocator)).into()
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppallocator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllocateForStream(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pallocator)).into()
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllocateForStream(::core::mem::transmute_copy(&dwsreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppallocator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMSyncReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRangeByTimecode: SetRangeByTimecode::<Identity, Impl, OFFSET>,
            SetRangeByFrameEx: SetRangeByFrameEx::<Identity, Impl, OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Identity, Impl, OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Identity, Impl, OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Identity, Impl, OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSyncReader2 as ::windows::core::Interface>::IID || iid == &<IWMSyncReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMVideoMediaProps_Impl: Sized + IWMMediaProps_Impl {
    fn GetMaxKeyFrameSpacing(&mut self) -> ::windows::core::Result<i64>;
    fn SetMaxKeyFrameSpacing(&mut self, lltime: i64) -> ::windows::core::Result<()>;
    fn GetQuality(&mut self) -> ::windows::core::Result<u32>;
    fn SetQuality(&mut self, dwquality: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMVideoMediaProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>() -> IWMVideoMediaProps_Vtbl {
        unsafe extern "system" fn GetMaxKeyFrameSpacing<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxKeyFrameSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *plltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxKeyFrameSpacing<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxKeyFrameSpacing(::core::mem::transmute_copy(&lltime)).into()
        }
        unsafe extern "system" fn GetQuality<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwquality = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetQuality(::core::mem::transmute_copy(&dwquality)).into()
        }
        Self {
            base: IWMMediaProps_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMaxKeyFrameSpacing: GetMaxKeyFrameSpacing::<Identity, Impl, OFFSET>,
            SetMaxKeyFrameSpacing: SetMaxKeyFrameSpacing::<Identity, Impl, OFFSET>,
            GetQuality: GetQuality::<Identity, Impl, OFFSET>,
            SetQuality: SetQuality::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMVideoMediaProps as ::windows::core::Interface>::IID || iid == &<IWMMediaProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWatermarkInfo_Impl: Sized {
    fn GetWatermarkEntryCount(&mut self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> ::windows::core::Result<u32>;
    fn GetWatermarkEntry(&mut self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32) -> ::windows::core::Result<WMT_WATERMARK_ENTRY>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWatermarkInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWatermarkInfo_Impl, const OFFSET: isize>() -> IWMWatermarkInfo_Vtbl {
        unsafe extern "system" fn GetWatermarkEntryCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMWatermarkInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWatermarkEntryCount(::core::mem::transmute_copy(&wmettype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatermarkEntry<Identity: ::windows::core::IUnknownImpl, Impl: IWMWatermarkInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWatermarkEntry(::core::mem::transmute_copy(&wmettype), ::core::mem::transmute_copy(&dwentrynum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetWatermarkEntryCount: GetWatermarkEntryCount::<Identity, Impl, OFFSET>,
            GetWatermarkEntry: GetWatermarkEntry::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWatermarkInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriter_Impl: Sized {
    fn SetProfileByID(&mut self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetProfile(&mut self, pprofile: &::core::option::Option<IWMProfile>) -> ::windows::core::Result<()>;
    fn SetOutputFilename(&mut self, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetInputCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetInputProps(&mut self, dwinputnum: u32) -> ::windows::core::Result<IWMInputMediaProps>;
    fn SetInputProps(&mut self, dwinputnum: u32, pinput: &::core::option::Option<IWMInputMediaProps>) -> ::windows::core::Result<()>;
    fn GetInputFormatCount(&mut self, dwinputnumber: u32) -> ::windows::core::Result<u32>;
    fn GetInputFormat(&mut self, dwinputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMInputMediaProps>;
    fn BeginWriting(&mut self) -> ::windows::core::Result<()>;
    fn EndWriting(&mut self) -> ::windows::core::Result<()>;
    fn AllocateSample(&mut self, dwsamplesize: u32) -> ::windows::core::Result<INSSBuffer>;
    fn WriteSample(&mut self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: &::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>() -> IWMWriter_Vtbl {
        unsafe extern "system" fn SetProfileByID<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProfileByID(::core::mem::transmute_copy(&guidprofile)).into()
        }
        unsafe extern "system" fn SetProfile<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProfile(::core::mem::transmute(&pprofile)).into()
        }
        unsafe extern "system" fn SetOutputFilename<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutputFilename(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        unsafe extern "system" fn GetInputCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcinputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputProps(::core::mem::transmute_copy(&dwinputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInputProps(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute(&pinput)).into()
        }
        unsafe extern "system" fn GetInputFormatCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputFormatCount(::core::mem::transmute_copy(&dwinputnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInputFormat(::core::mem::transmute_copy(&dwinputnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginWriting<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginWriting().into()
        }
        unsafe extern "system" fn EndWriting<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndWriting().into()
        }
        unsafe extern "system" fn AllocateSample<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllocateSample(::core::mem::transmute_copy(&dwsamplesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsample = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSample<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteSample(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample)).into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetProfileByID: SetProfileByID::<Identity, Impl, OFFSET>,
            SetProfile: SetProfile::<Identity, Impl, OFFSET>,
            SetOutputFilename: SetOutputFilename::<Identity, Impl, OFFSET>,
            GetInputCount: GetInputCount::<Identity, Impl, OFFSET>,
            GetInputProps: GetInputProps::<Identity, Impl, OFFSET>,
            SetInputProps: SetInputProps::<Identity, Impl, OFFSET>,
            GetInputFormatCount: GetInputFormatCount::<Identity, Impl, OFFSET>,
            GetInputFormat: GetInputFormat::<Identity, Impl, OFFSET>,
            BeginWriting: BeginWriting::<Identity, Impl, OFFSET>,
            EndWriting: EndWriting::<Identity, Impl, OFFSET>,
            AllocateSample: AllocateSample::<Identity, Impl, OFFSET>,
            WriteSample: WriteSample::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced_Impl: Sized {
    fn GetSinkCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetSink(&mut self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink>;
    fn AddSink(&mut self, psink: &::core::option::Option<IWMWriterSink>) -> ::windows::core::Result<()>;
    fn RemoveSink(&mut self, psink: &::core::option::Option<IWMWriterSink>) -> ::windows::core::Result<()>;
    fn WriteStreamSample(&mut self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: &::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn SetLiveSource(&mut self, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsRealTime(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetWriterTime(&mut self) -> ::windows::core::Result<u64>;
    fn GetStatistics(&mut self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS>;
    fn SetSyncTolerance(&mut self, mswindow: u32) -> ::windows::core::Result<()>;
    fn GetSyncTolerance(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>() -> IWMWriterAdvanced_Vtbl {
        unsafe extern "system" fn GetSinkCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSinkCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcsinks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSink<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSink(::core::mem::transmute_copy(&dwsinknum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSink<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddSink(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn RemoveSink<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveSink(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn WriteStreamSample<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WriteStreamSample(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&mssamplesendtime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample)).into()
        }
        unsafe extern "system" fn SetLiveSource<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLiveSource(::core::mem::transmute_copy(&fislivesource)).into()
        }
        unsafe extern "system" fn IsRealTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrealtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriterTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWriterTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnscurrenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatistics(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncTolerance<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSyncTolerance(::core::mem::transmute_copy(&mswindow)).into()
        }
        unsafe extern "system" fn GetSyncTolerance<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSyncTolerance() {
                ::core::result::Result::Ok(ok__) => {
                    *pmswindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSinkCount: GetSinkCount::<Identity, Impl, OFFSET>,
            GetSink: GetSink::<Identity, Impl, OFFSET>,
            AddSink: AddSink::<Identity, Impl, OFFSET>,
            RemoveSink: RemoveSink::<Identity, Impl, OFFSET>,
            WriteStreamSample: WriteStreamSample::<Identity, Impl, OFFSET>,
            SetLiveSource: SetLiveSource::<Identity, Impl, OFFSET>,
            IsRealTime: IsRealTime::<Identity, Impl, OFFSET>,
            GetWriterTime: GetWriterTime::<Identity, Impl, OFFSET>,
            GetStatistics: GetStatistics::<Identity, Impl, OFFSET>,
            SetSyncTolerance: SetSyncTolerance::<Identity, Impl, OFFSET>,
            GetSyncTolerance: GetSyncTolerance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced2_Impl: Sized + IWMWriterAdvanced_Impl {
    fn GetInputSetting(&mut self, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetInputSetting(&mut self, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced2_Impl, const OFFSET: isize>() -> IWMWriterAdvanced2_Vtbl {
        unsafe extern "system" fn GetInputSetting<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInputSetting(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetInputSetting<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInputSetting(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        Self {
            base: IWMWriterAdvanced_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetInputSetting: GetInputSetting::<Identity, Impl, OFFSET>,
            SetInputSetting: SetInputSetting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterAdvanced2 as ::windows::core::Interface>::IID || iid == &<IWMWriterAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced3_Impl: Sized + IWMWriterAdvanced_Impl + IWMWriterAdvanced2_Impl {
    fn GetStatisticsEx(&mut self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS_EX>;
    fn SetNonBlocking(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced3_Impl, const OFFSET: isize>() -> IWMWriterAdvanced3_Vtbl {
        unsafe extern "system" fn GetStatisticsEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetStatisticsEx(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonBlocking<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNonBlocking().into()
        }
        Self {
            base: IWMWriterAdvanced2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStatisticsEx: GetStatisticsEx::<Identity, Impl, OFFSET>,
            SetNonBlocking: SetNonBlocking::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterAdvanced3 as ::windows::core::Interface>::IID || iid == &<IWMWriterAdvanced as ::windows::core::Interface>::IID || iid == &<IWMWriterAdvanced2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink_Impl: Sized + IWMWriterSink_Impl {
    fn Open(&mut self, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink_Impl, const OFFSET: isize>() -> IWMWriterFileSink_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        Self { base: IWMWriterSink_Vtbl::new::<Identity, Impl, OFFSET>(), Open: Open::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterFileSink as ::windows::core::Interface>::IID || iid == &<IWMWriterSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink2_Impl: Sized + IWMWriterSink_Impl + IWMWriterFileSink_Impl {
    fn Start(&mut self, cnsstarttime: u64) -> ::windows::core::Result<()>;
    fn Stop(&mut self, cnsstoptime: u64) -> ::windows::core::Result<()>;
    fn IsStopped(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetFileDuration(&mut self) -> ::windows::core::Result<u64>;
    fn GetFileSize(&mut self) -> ::windows::core::Result<u64>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn IsClosed(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSink2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>() -> IWMWriterFileSink2_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&cnsstarttime)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop(::core::mem::transmute_copy(&cnsstoptime)).into()
        }
        unsafe extern "system" fn IsStopped<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsStopped() {
                ::core::result::Result::Ok(ok__) => {
                    *pfstopped = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileDuration<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbfile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn IsClosed<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *pfclosed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMWriterFileSink_Vtbl::new::<Identity, Impl, OFFSET>(),
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            IsStopped: IsStopped::<Identity, Impl, OFFSET>,
            GetFileDuration: GetFileDuration::<Identity, Impl, OFFSET>,
            GetFileSize: GetFileSize::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            IsClosed: IsClosed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterFileSink2 as ::windows::core::Interface>::IID || iid == &<IWMWriterSink as ::windows::core::Interface>::IID || iid == &<IWMWriterFileSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink3_Impl: Sized + IWMWriterSink_Impl + IWMWriterFileSink_Impl + IWMWriterFileSink2_Impl {
    fn SetAutoIndexing(&mut self, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAutoIndexing(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetControlStream(&mut self, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMode(&mut self) -> ::windows::core::Result<u32>;
    fn OnDataUnitEx(&mut self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::Result<()>;
    fn SetUnbufferedIO(&mut self, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetUnbufferedIO(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CompleteOperations(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSink3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>() -> IWMWriterFileSink3_Vtbl {
        unsafe extern "system" fn SetAutoIndexing<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAutoIndexing(::core::mem::transmute_copy(&fdoautoindexing)).into()
        }
        unsafe extern "system" fn GetAutoIndexing<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAutoIndexing() {
                ::core::result::Result::Ok(ok__) => {
                    *pfautoindexing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlStream<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetControlStream(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&fshouldcontrolstartandstop)).into()
        }
        unsafe extern "system" fn GetMode<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwfilesinkmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnitEx<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDataUnitEx(::core::mem::transmute_copy(&pfilesinkdataunit)).into()
        }
        unsafe extern "system" fn SetUnbufferedIO<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUnbufferedIO(::core::mem::transmute_copy(&funbufferedio), ::core::mem::transmute_copy(&frestrictmemusage)).into()
        }
        unsafe extern "system" fn GetUnbufferedIO<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUnbufferedIO() {
                ::core::result::Result::Ok(ok__) => {
                    *pfunbufferedio = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CompleteOperations().into()
        }
        Self {
            base: IWMWriterFileSink2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetAutoIndexing: SetAutoIndexing::<Identity, Impl, OFFSET>,
            GetAutoIndexing: GetAutoIndexing::<Identity, Impl, OFFSET>,
            SetControlStream: SetControlStream::<Identity, Impl, OFFSET>,
            GetMode: GetMode::<Identity, Impl, OFFSET>,
            OnDataUnitEx: OnDataUnitEx::<Identity, Impl, OFFSET>,
            SetUnbufferedIO: SetUnbufferedIO::<Identity, Impl, OFFSET>,
            GetUnbufferedIO: GetUnbufferedIO::<Identity, Impl, OFFSET>,
            CompleteOperations: CompleteOperations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterFileSink3 as ::windows::core::Interface>::IID || iid == &<IWMWriterSink as ::windows::core::Interface>::IID || iid == &<IWMWriterFileSink as ::windows::core::Interface>::IID || iid == &<IWMWriterFileSink2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterNetworkSink_Impl: Sized + IWMWriterSink_Impl {
    fn SetMaximumClients(&mut self, dwmaxclients: u32) -> ::windows::core::Result<()>;
    fn GetMaximumClients(&mut self) -> ::windows::core::Result<u32>;
    fn SetNetworkProtocol(&mut self, protocol: WMT_NET_PROTOCOL) -> ::windows::core::Result<()>;
    fn GetNetworkProtocol(&mut self) -> ::windows::core::Result<WMT_NET_PROTOCOL>;
    fn GetHostURL(&mut self, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()>;
    fn Open(&mut self, pdwportnum: *mut u32) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterNetworkSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>() -> IWMWriterNetworkSink_Vtbl {
        unsafe extern "system" fn SetMaximumClients<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumClients(::core::mem::transmute_copy(&dwmaxclients)).into()
        }
        unsafe extern "system" fn GetMaximumClients<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaximumClients() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProtocol<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNetworkProtocol(::core::mem::transmute_copy(&protocol)).into()
        }
        unsafe extern "system" fn GetNetworkProtocol<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetNetworkProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostURL<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetHostURL(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pdwportnum)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: IWMWriterSink_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMaximumClients: SetMaximumClients::<Identity, Impl, OFFSET>,
            GetMaximumClients: GetMaximumClients::<Identity, Impl, OFFSET>,
            SetNetworkProtocol: SetNetworkProtocol::<Identity, Impl, OFFSET>,
            GetNetworkProtocol: GetNetworkProtocol::<Identity, Impl, OFFSET>,
            GetHostURL: GetHostURL::<Identity, Impl, OFFSET>,
            Open: Open::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterNetworkSink as ::windows::core::Interface>::IID || iid == &<IWMWriterSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterPostView_Impl: Sized {
    fn SetPostViewCallback(&mut self, pcallback: &::core::option::Option<IWMWriterPostViewCallback>, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetReceivePostViewSamples(&mut self, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReceivePostViewSamples(&mut self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetPostViewProps(&mut self, wstreamnumber: u16) -> ::windows::core::Result<IWMMediaProps>;
    fn SetPostViewProps(&mut self, wstreamnumber: u16, poutput: &::core::option::Option<IWMMediaProps>) -> ::windows::core::Result<()>;
    fn GetPostViewFormatCount(&mut self, wstreamnumber: u16) -> ::windows::core::Result<u32>;
    fn GetPostViewFormat(&mut self, wstreamnumber: u16, dwformatnumber: u32) -> ::windows::core::Result<IWMMediaProps>;
    fn SetAllocateForPostView(&mut self, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateForPostView(&mut self, wstreamnumber: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterPostView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>() -> IWMWriterPostView_Vtbl {
        unsafe extern "system" fn SetPostViewCallback<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostViewCallback(::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn SetReceivePostViewSamples<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetReceivePostViewSamples(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&freceivepostviewsamples)).into()
        }
        unsafe extern "system" fn GetReceivePostViewSamples<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetReceivePostViewSamples(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfreceivepostviewsamples = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPostViewProps(::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostViewProps<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPostViewProps(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetPostViewFormatCount<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPostViewFormatCount(::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewFormat<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPostViewFormat(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForPostView<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllocateForPostView(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForPostView<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllocateForPostView(::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfallocate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetPostViewCallback: SetPostViewCallback::<Identity, Impl, OFFSET>,
            SetReceivePostViewSamples: SetReceivePostViewSamples::<Identity, Impl, OFFSET>,
            GetReceivePostViewSamples: GetReceivePostViewSamples::<Identity, Impl, OFFSET>,
            GetPostViewProps: GetPostViewProps::<Identity, Impl, OFFSET>,
            SetPostViewProps: SetPostViewProps::<Identity, Impl, OFFSET>,
            GetPostViewFormatCount: GetPostViewFormatCount::<Identity, Impl, OFFSET>,
            GetPostViewFormat: GetPostViewFormat::<Identity, Impl, OFFSET>,
            SetAllocateForPostView: SetAllocateForPostView::<Identity, Impl, OFFSET>,
            GetAllocateForPostView: GetAllocateForPostView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPostView as ::windows::core::Interface>::IID
    }
}
pub trait IWMWriterPostViewCallback_Impl: Sized + IWMStatusCallback_Impl {
    fn OnPostViewSample(&mut self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: &::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForPostView(&mut self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMWriterPostViewCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostViewCallback_Impl, const OFFSET: isize>() -> IWMWriterPostViewCallback_Vtbl {
        unsafe extern "system" fn OnPostViewSample<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostViewCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnPostViewSample(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForPostView<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostViewCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AllocateForPostView(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: IWMStatusCallback_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnPostViewSample: OnPostViewSample::<Identity, Impl, OFFSET>,
            AllocateForPostView: AllocateForPostView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPostViewCallback as ::windows::core::Interface>::IID || iid == &<IWMStatusCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWMWriterPreprocess_Impl: Sized {
    fn GetMaxPreprocessingPasses(&mut self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<u32>;
    fn SetNumPreprocessingPasses(&mut self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::Result<()>;
    fn BeginPreprocessingPass(&mut self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn PreprocessSample(&mut self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: &::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn EndPreprocessingPass(&mut self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()>;
}
impl IWMWriterPreprocess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>() -> IWMWriterPreprocess_Vtbl {
        unsafe extern "system" fn GetMaxPreprocessingPasses<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaxPreprocessingPasses(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxnumpasses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumPreprocessingPasses<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNumPreprocessingPasses(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwnumpasses)).into()
        }
        unsafe extern "system" fn BeginPreprocessingPass<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginPreprocessingPass(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn PreprocessSample<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PreprocessSample(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample)).into()
        }
        unsafe extern "system" fn EndPreprocessingPass<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndPreprocessingPass(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetMaxPreprocessingPasses: GetMaxPreprocessingPasses::<Identity, Impl, OFFSET>,
            SetNumPreprocessingPasses: SetNumPreprocessingPasses::<Identity, Impl, OFFSET>,
            BeginPreprocessingPass: BeginPreprocessingPass::<Identity, Impl, OFFSET>,
            PreprocessSample: PreprocessSample::<Identity, Impl, OFFSET>,
            EndPreprocessingPass: EndPreprocessingPass::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPreprocess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterPushSink_Impl: Sized + IWMWriterSink_Impl {
    fn Connect(&mut self, pwszurl: super::super::Foundation::PWSTR, pwsztemplateurl: super::super::Foundation::PWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn EndSession(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterPushSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPushSink_Impl, const OFFSET: isize>() -> IWMWriterPushSink_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPushSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pwsztemplateurl: super::super::Foundation::PWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pwsztemplateurl), ::core::mem::transmute_copy(&fautodestroy)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPushSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn EndSession<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPushSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndSession().into()
        }
        Self {
            base: IWMWriterSink_Vtbl::new::<Identity, Impl, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPushSink as ::windows::core::Interface>::IID || iid == &<IWMWriterSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterSink_Impl: Sized {
    fn OnHeader(&mut self, pheader: &::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn IsRealTime(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn AllocateDataUnit(&mut self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer>;
    fn OnDataUnit(&mut self, pdataunit: &::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn OnEndWriting(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterSink_Impl, const OFFSET: isize>() -> IWMWriterSink_Vtbl {
        unsafe extern "system" fn OnHeader<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnHeader(::core::mem::transmute(&pheader)).into()
        }
        unsafe extern "system" fn IsRealTime<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrealtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateDataUnit<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllocateDataUnit(::core::mem::transmute_copy(&cbdataunit)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataunit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnit<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDataUnit(::core::mem::transmute(&pdataunit)).into()
        }
        unsafe extern "system" fn OnEndWriting<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEndWriting().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnHeader: OnHeader::<Identity, Impl, OFFSET>,
            IsRealTime: IsRealTime::<Identity, Impl, OFFSET>,
            AllocateDataUnit: AllocateDataUnit::<Identity, Impl, OFFSET>,
            OnDataUnit: OnDataUnit::<Identity, Impl, OFFSET>,
            OnEndWriting: OnEndWriting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterSink as ::windows::core::Interface>::IID
    }
}
