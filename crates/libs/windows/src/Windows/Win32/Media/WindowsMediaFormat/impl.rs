pub trait IAMWMBufferPassImpl: Sized {
    fn SetNotify(&mut self, pcallback: ::core::option::Option<IAMWMBufferPassCallback>) -> ::windows::core::Result<()>;
}
impl IAMWMBufferPassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAMWMBufferPassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAMWMBufferPassVtbl {
        unsafe extern "system" fn SetNotify<Impl: IAMWMBufferPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotify(::core::mem::transmute(&pcallback)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetNotify: SetNotify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAMWMBufferPass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IAMWMBufferPassCallbackImpl: Sized {
    fn Notify(&mut self, pnssbuffer3: ::core::option::Option<INSSBuffer3>, ppin: ::core::option::Option<super::DirectShow::IPin>, prtstart: *const i64, prtend: *const i64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IAMWMBufferPassCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAMWMBufferPassCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAMWMBufferPassCallbackVtbl {
        unsafe extern "system" fn Notify<Impl: IAMWMBufferPassCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnssbuffer3: ::windows::core::RawPtr, ppin: ::windows::core::RawPtr, prtstart: *const i64, prtend: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute(&pnssbuffer3), ::core::mem::transmute(&ppin), ::core::mem::transmute_copy(&prtstart), ::core::mem::transmute_copy(&prtend)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Notify: Notify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAMWMBufferPassCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INSNetSourceCreatorImpl: Sized {
    fn Initialize(&mut self) -> ::windows::core::Result<()>;
    fn CreateNetSource(&mut self, pszstreamname: super::super::Foundation::PWSTR, pmonitor: ::core::option::Option<::windows::core::IUnknown>, pdata: *const u8, pusercontext: ::core::option::Option<::windows::core::IUnknown>, pcallback: ::core::option::Option<::windows::core::IUnknown>, qwcontext: u64) -> ::windows::core::Result<()>;
    fn GetNetSourceProperties(&mut self, pszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetNetSourceSharedNamespace(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetNetSourceAdminInterface(&mut self, pszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetNumProtocolsSupported(&mut self) -> ::windows::core::Result<u32>;
    fn GetProtocolName(&mut self, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INSNetSourceCreatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSNetSourceCreatorVtbl {
        unsafe extern "system" fn Initialize<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize().into()
        }
        unsafe extern "system" fn CreateNetSource<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateNetSource(::core::mem::transmute_copy(&pszstreamname), ::core::mem::transmute(&pmonitor), ::core::mem::transmute_copy(&pdata), ::core::mem::transmute(&pusercontext), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&qwcontext)).into()
        }
        unsafe extern "system" fn GetNetSourceProperties<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetSourceProperties(::core::mem::transmute_copy(&pszstreamname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppropertiesnode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceSharedNamespace<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetSourceSharedNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsharednamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceAdminInterface<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetSourceAdminInterface(::core::mem::transmute_copy(&pszstreamname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumProtocolsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pcprotocols = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocolName<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProtocolName(::core::mem::transmute_copy(&dwprotocolnum), ::core::mem::transmute_copy(&pwszprotocolname), ::core::mem::transmute_copy(&pcchprotocolname)).into()
        }
        unsafe extern "system" fn Shutdown<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            CreateNetSource: CreateNetSource::<Impl, IMPL_OFFSET>,
            GetNetSourceProperties: GetNetSourceProperties::<Impl, IMPL_OFFSET>,
            GetNetSourceSharedNamespace: GetNetSourceSharedNamespace::<Impl, IMPL_OFFSET>,
            GetNetSourceAdminInterface: GetNetSourceAdminInterface::<Impl, IMPL_OFFSET>,
            GetNumProtocolsSupported: GetNumProtocolsSupported::<Impl, IMPL_OFFSET>,
            GetProtocolName: GetProtocolName::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSNetSourceCreator as ::windows::core::Interface>::IID
    }
}
pub trait INSSBufferImpl: Sized {
    fn GetLength(&mut self) -> ::windows::core::Result<u32>;
    fn SetLength(&mut self, dwlength: u32) -> ::windows::core::Result<()>;
    fn GetMaxLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetBuffer(&mut self) -> ::windows::core::Result<*mut u8>;
    fn GetBufferAndLength(&mut self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()>;
}
impl INSSBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSSBufferVtbl {
        unsafe extern "system" fn GetLength<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLength(::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetMaxLength<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuffer<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdwbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferAndLength(::core::mem::transmute_copy(&ppdwbuffer), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLength: GetLength::<Impl, IMPL_OFFSET>,
            SetLength: SetLength::<Impl, IMPL_OFFSET>,
            GetMaxLength: GetMaxLength::<Impl, IMPL_OFFSET>,
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer as ::windows::core::Interface>::IID
    }
}
pub trait INSSBuffer2Impl: Sized + INSSBufferImpl {
    fn GetSampleProperties(&mut self, cbproperties: u32) -> ::windows::core::Result<u8>;
    fn SetSampleProperties(&mut self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()>;
}
impl INSSBuffer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSSBuffer2Vtbl {
        unsafe extern "system" fn GetSampleProperties<Impl: INSSBuffer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSampleProperties(::core::mem::transmute_copy(&cbproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleProperties<Impl: INSSBuffer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSampleProperties(::core::mem::transmute_copy(&cbproperties), ::core::mem::transmute_copy(&pbproperties)).into()
        }
        Self {
            base: INSSBufferVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSampleProperties: GetSampleProperties::<Impl, IMPL_OFFSET>,
            SetSampleProperties: SetSampleProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer2 as ::windows::core::Interface>::IID
    }
}
pub trait INSSBuffer3Impl: Sized + INSSBufferImpl + INSSBuffer2Impl {
    fn SetProperty(&mut self, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()>;
}
impl INSSBuffer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSSBuffer3Vtbl {
        unsafe extern "system" fn SetProperty<Impl: INSSBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&guidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&dwbufferpropertysize)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: INSSBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&guidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&pdwbufferpropertysize)).into()
        }
        Self {
            base: INSSBuffer2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer3 as ::windows::core::Interface>::IID
    }
}
pub trait INSSBuffer4Impl: Sized + INSSBufferImpl + INSSBuffer2Impl + INSSBuffer3Impl {
    fn GetPropertyCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetPropertyByIndex(&mut self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()>;
}
impl INSSBuffer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSSBuffer4Vtbl {
        unsafe extern "system" fn GetPropertyCount<Impl: INSSBuffer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbufferproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyByIndex<Impl: INSSBuffer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyByIndex(::core::mem::transmute_copy(&dwbufferpropertyindex), ::core::mem::transmute_copy(&pguidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&pdwbufferpropertysize)).into()
        }
        Self {
            base: INSSBuffer3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Impl, IMPL_OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer4 as ::windows::core::Interface>::IID
    }
}
pub trait IWMAddressAccessImpl: Sized {
    fn GetAccessEntryCount(&mut self, aetype: WM_AETYPE) -> ::windows::core::Result<u32>;
    fn GetAccessEntry(&mut self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY>;
    fn AddAccessEntry(&mut self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()>;
    fn RemoveAccessEntry(&mut self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()>;
}
impl IWMAddressAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMAddressAccessVtbl {
        unsafe extern "system" fn GetAccessEntryCount<Impl: IWMAddressAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessEntryCount(::core::mem::transmute_copy(&aetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcentries = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessEntry<Impl: IWMAddressAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAccessEntry(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum)) {
                ::core::result::Result::Ok(ok__) => {
                    *paddraccessentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccessEntry<Impl: IWMAddressAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAccessEntry(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&paddraccessentry)).into()
        }
        unsafe extern "system" fn RemoveAccessEntry<Impl: IWMAddressAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAccessEntry(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAccessEntryCount: GetAccessEntryCount::<Impl, IMPL_OFFSET>,
            GetAccessEntry: GetAccessEntry::<Impl, IMPL_OFFSET>,
            AddAccessEntry: AddAccessEntry::<Impl, IMPL_OFFSET>,
            RemoveAccessEntry: RemoveAccessEntry::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMAddressAccess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMAddressAccess2Impl: Sized + IWMAddressAccessImpl {
    fn GetAccessEntryEx(&mut self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddAccessEntryEx(&mut self, aetype: WM_AETYPE, bstraddress: super::super::Foundation::BSTR, bstrmask: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMAddressAccess2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMAddressAccess2Vtbl {
        unsafe extern "system" fn GetAccessEntryEx<Impl: IWMAddressAccess2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAccessEntryEx(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum), ::core::mem::transmute_copy(&pbstraddress), ::core::mem::transmute_copy(&pbstrmask)).into()
        }
        unsafe extern "system" fn AddAccessEntryEx<Impl: IWMAddressAccess2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAccessEntryEx(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&bstraddress), ::core::mem::transmute_copy(&bstrmask)).into()
        }
        Self {
            base: IWMAddressAccessVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAccessEntryEx: GetAccessEntryEx::<Impl, IMPL_OFFSET>,
            AddAccessEntryEx: AddAccessEntryEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMAddressAccess2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMAuthorizerImpl: Sized {
    fn GetCertCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetCert(&mut self, dwindex: u32) -> ::windows::core::Result<*mut u8>;
    fn GetSharedData(&mut self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8>;
}
impl IWMAuthorizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMAuthorizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMAuthorizerVtbl {
        unsafe extern "system" fn GetCertCount<Impl: IWMAuthorizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCertCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccerts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCert<Impl: IWMAuthorizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCert(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbcertdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedData<Impl: IWMAuthorizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSharedData(::core::mem::transmute_copy(&dwcertindex), ::core::mem::transmute_copy(&pbshareddata), ::core::mem::transmute_copy(&pbcert)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbshareddata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCertCount: GetCertCount::<Impl, IMPL_OFFSET>,
            GetCert: GetCert::<Impl, IMPL_OFFSET>,
            GetSharedData: GetSharedData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMAuthorizer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMBackupRestorePropsImpl: Sized {
    fn GetPropCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetPropByIndex(&mut self, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn GetPropByName(&mut self, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetProp(&mut self, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn RemoveProp(&mut self, pcwszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn RemoveAllProps(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMBackupRestorePropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMBackupRestorePropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMBackupRestorePropsVtbl {
        unsafe extern "system" fn GetPropCount<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropByIndex<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropByIndex(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetPropByName<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropByName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetProp<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProp(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn RemoveProp<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProp(::core::mem::transmute_copy(&pcwszname)).into()
        }
        unsafe extern "system" fn RemoveAllProps<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllProps().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPropCount: GetPropCount::<Impl, IMPL_OFFSET>,
            GetPropByIndex: GetPropByIndex::<Impl, IMPL_OFFSET>,
            GetPropByName: GetPropByName::<Impl, IMPL_OFFSET>,
            SetProp: SetProp::<Impl, IMPL_OFFSET>,
            RemoveProp: RemoveProp::<Impl, IMPL_OFFSET>,
            RemoveAllProps: RemoveAllProps::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMBackupRestoreProps as ::windows::core::Interface>::IID
    }
}
pub trait IWMBandwidthSharingImpl: Sized + IWMStreamListImpl {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetType(&mut self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetBandwidth(&mut self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::Result<()>;
    fn SetBandwidth(&mut self, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::Result<()>;
}
impl IWMBandwidthSharingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMBandwidthSharingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMBandwidthSharingVtbl {
        unsafe extern "system" fn GetType<Impl: IWMBandwidthSharingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IWMBandwidthSharingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&guidtype)).into()
        }
        unsafe extern "system" fn GetBandwidth<Impl: IWMBandwidthSharingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBandwidth(::core::mem::transmute_copy(&pdwbitrate), ::core::mem::transmute_copy(&pmsbufferwindow)).into()
        }
        unsafe extern "system" fn SetBandwidth<Impl: IWMBandwidthSharingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBandwidth(::core::mem::transmute_copy(&dwbitrate), ::core::mem::transmute_copy(&msbufferwindow)).into()
        }
        Self {
            base: IWMStreamListVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            GetBandwidth: GetBandwidth::<Impl, IMPL_OFFSET>,
            SetBandwidth: SetBandwidth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMBandwidthSharing as ::windows::core::Interface>::IID
    }
}
pub trait IWMClientConnectionsImpl: Sized {
    fn GetClientCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetClientProperties(&mut self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES>;
}
impl IWMClientConnectionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMClientConnectionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMClientConnectionsVtbl {
        unsafe extern "system" fn GetClientCount<Impl: IWMClientConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientProperties<Impl: IWMClientConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientProperties(::core::mem::transmute_copy(&dwclientnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pclientproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClientCount: GetClientCount::<Impl, IMPL_OFFSET>,
            GetClientProperties: GetClientProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMClientConnections as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMClientConnections2Impl: Sized + IWMClientConnectionsImpl {
    fn GetClientInfo(&mut self, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMClientConnections2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMClientConnections2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMClientConnections2Vtbl {
        unsafe extern "system" fn GetClientInfo<Impl: IWMClientConnections2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetClientInfo(::core::mem::transmute_copy(&dwclientnum), ::core::mem::transmute_copy(&pwsznetworkaddress), ::core::mem::transmute_copy(&pcchnetworkaddress), ::core::mem::transmute_copy(&pwszport), ::core::mem::transmute_copy(&pcchport), ::core::mem::transmute_copy(&pwszdnsname), ::core::mem::transmute_copy(&pcchdnsname)).into()
        }
        Self { base: IWMClientConnectionsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetClientInfo: GetClientInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMClientConnections2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait IWMCodecAMVideoAcceleratorImpl: Sized {
    fn SetAcceleratorInterface(&mut self, piamva: ::core::option::Option<super::DirectShow::IAMVideoAccelerator>) -> ::windows::core::Result<()>;
    fn NegotiateConnection(&mut self, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
    fn SetPlayerNotify(&mut self, phook: ::core::option::Option<IWMPlayerTimestampHook>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl IWMCodecAMVideoAcceleratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecAMVideoAcceleratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecAMVideoAcceleratorVtbl {
        unsafe extern "system" fn SetAcceleratorInterface<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleratorInterface(::core::mem::transmute(&piamva)).into()
        }
        unsafe extern "system" fn NegotiateConnection<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NegotiateConnection(::core::mem::transmute_copy(&pmediatype)).into()
        }
        unsafe extern "system" fn SetPlayerNotify<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayerNotify(::core::mem::transmute(&phook)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAcceleratorInterface: SetAcceleratorInterface::<Impl, IMPL_OFFSET>,
            NegotiateConnection: NegotiateConnection::<Impl, IMPL_OFFSET>,
            SetPlayerNotify: SetPlayerNotify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecAMVideoAccelerator as ::windows::core::Interface>::IID
    }
}
pub trait IWMCodecInfoImpl: Sized {
    fn GetCodecInfoCount(&mut self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn GetCodecFormatCount(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32>;
    fn GetCodecFormat(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig>;
}
impl IWMCodecInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecInfoVtbl {
        unsafe extern "system" fn GetCodecInfoCount<Impl: IWMCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodecInfoCount(::core::mem::transmute_copy(&guidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pccodecs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormatCount<Impl: IWMCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodecFormatCount(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormat<Impl: IWMCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodecFormat(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppistreamconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Impl, IMPL_OFFSET>,
            GetCodecFormatCount: GetCodecFormatCount::<Impl, IMPL_OFFSET>,
            GetCodecFormat: GetCodecFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMCodecInfo2Impl: Sized + IWMCodecInfoImpl {
    fn GetCodecName(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()>;
    fn GetCodecFormatDesc(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMCodecInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecInfo2Vtbl {
        unsafe extern "system" fn GetCodecName<Impl: IWMCodecInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCodecName(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetCodecFormatDesc<Impl: IWMCodecInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCodecFormatDesc(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex), ::core::mem::transmute_copy(&ppistreamconfig), ::core::mem::transmute_copy(&wszdesc), ::core::mem::transmute_copy(&pcchdesc)).into()
        }
        Self {
            base: IWMCodecInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCodecName: GetCodecName::<Impl, IMPL_OFFSET>,
            GetCodecFormatDesc: GetCodecFormatDesc::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMCodecInfo3Impl: Sized + IWMCodecInfoImpl + IWMCodecInfo2Impl {
    fn GetCodecFormatProp(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetCodecProp(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn SetCodecEnumerationSetting(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn GetCodecEnumerationSetting(&mut self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMCodecInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecInfo3Vtbl {
        unsafe extern "system" fn GetCodecFormatProp<Impl: IWMCodecInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCodecFormatProp(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn GetCodecProp<Impl: IWMCodecInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCodecProp(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn SetCodecEnumerationSetting<Impl: IWMCodecInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCodecEnumerationSetting(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetCodecEnumerationSetting<Impl: IWMCodecInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCodecEnumerationSetting(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self {
            base: IWMCodecInfo2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCodecFormatProp: GetCodecFormatProp::<Impl, IMPL_OFFSET>,
            GetCodecProp: GetCodecProp::<Impl, IMPL_OFFSET>,
            SetCodecEnumerationSetting: SetCodecEnumerationSetting::<Impl, IMPL_OFFSET>,
            GetCodecEnumerationSetting: GetCodecEnumerationSetting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait IWMCodecVideoAcceleratorImpl: Sized {
    fn NegotiateConnection(&mut self, piamva: ::core::option::Option<super::DirectShow::IAMVideoAccelerator>, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::Result<()>;
    fn SetPlayerNotify(&mut self, phook: ::core::option::Option<IWMPlayerTimestampHook>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl IWMCodecVideoAcceleratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecVideoAcceleratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecVideoAcceleratorVtbl {
        unsafe extern "system" fn NegotiateConnection<Impl: IWMCodecVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NegotiateConnection(::core::mem::transmute(&piamva), ::core::mem::transmute_copy(&pmediatype)).into()
        }
        unsafe extern "system" fn SetPlayerNotify<Impl: IWMCodecVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayerNotify(::core::mem::transmute(&phook)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NegotiateConnection: NegotiateConnection::<Impl, IMPL_OFFSET>,
            SetPlayerNotify: SetPlayerNotify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecVideoAccelerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMCredentialCallbackImpl: Sized {
    fn AcquireCredentials(&mut self, pwszrealm: super::super::Foundation::PWSTR, pwszsite: super::super::Foundation::PWSTR, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMCredentialCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCredentialCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCredentialCallbackVtbl {
        unsafe extern "system" fn AcquireCredentials<Impl: IWMCredentialCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrealm: super::super::Foundation::PWSTR, pwszsite: super::super::Foundation::PWSTR, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireCredentials(::core::mem::transmute_copy(&pwszrealm), ::core::mem::transmute_copy(&pwszsite), ::core::mem::transmute_copy(&pwszuser), ::core::mem::transmute_copy(&cchuser), ::core::mem::transmute_copy(&pwszpassword), ::core::mem::transmute_copy(&cchpassword), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AcquireCredentials: AcquireCredentials::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCredentialCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMEditorImpl: Sized {
    fn GetDRMProperty(&mut self, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMEditorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMEditorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMEditorVtbl {
        unsafe extern "system" fn GetDRMProperty<Impl: IWMDRMEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDRMProperty(::core::mem::transmute_copy(&pwstrname), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDRMProperty: GetDRMProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMEditor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMMessageParserImpl: Sized {
    fn ParseRegistrationReqMsg(&mut self, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::Result<()>;
    fn ParseLicenseRequestMsg(&mut self, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMMessageParserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMMessageParserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMMessageParserVtbl {
        unsafe extern "system" fn ParseRegistrationReqMsg<Impl: IWMDRMMessageParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseRegistrationReqMsg(::core::mem::transmute_copy(&pbregistrationreqmsg), ::core::mem::transmute_copy(&cbregistrationreqmsg), ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber)).into()
        }
        unsafe extern "system" fn ParseLicenseRequestMsg<Impl: IWMDRMMessageParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ParseLicenseRequestMsg(::core::mem::transmute_copy(&pblicenserequestmsg), ::core::mem::transmute_copy(&cblicenserequestmsg), ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber), ::core::mem::transmute_copy(&pbstraction)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ParseRegistrationReqMsg: ParseRegistrationReqMsg::<Impl, IMPL_OFFSET>,
            ParseLicenseRequestMsg: ParseLicenseRequestMsg::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMMessageParser as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReaderImpl: Sized {
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
impl IWMDRMReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMReaderVtbl {
        unsafe extern "system" fn AcquireLicense<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireLicense(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CancelLicenseAcquisition<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelLicenseAcquisition().into()
        }
        unsafe extern "system" fn Individualize<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Individualize(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CancelIndividualization<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelIndividualization().into()
        }
        unsafe extern "system" fn MonitorLicenseAcquisition<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MonitorLicenseAcquisition().into()
        }
        unsafe extern "system" fn CancelMonitorLicenseAcquisition<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelMonitorLicenseAcquisition().into()
        }
        unsafe extern "system" fn SetDRMProperty<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDRMProperty(::core::mem::transmute_copy(&pwstrname), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetDRMProperty<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDRMProperty(::core::mem::transmute_copy(&pwstrname), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AcquireLicense: AcquireLicense::<Impl, IMPL_OFFSET>,
            CancelLicenseAcquisition: CancelLicenseAcquisition::<Impl, IMPL_OFFSET>,
            Individualize: Individualize::<Impl, IMPL_OFFSET>,
            CancelIndividualization: CancelIndividualization::<Impl, IMPL_OFFSET>,
            MonitorLicenseAcquisition: MonitorLicenseAcquisition::<Impl, IMPL_OFFSET>,
            CancelMonitorLicenseAcquisition: CancelMonitorLicenseAcquisition::<Impl, IMPL_OFFSET>,
            SetDRMProperty: SetDRMProperty::<Impl, IMPL_OFFSET>,
            GetDRMProperty: GetDRMProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReader2Impl: Sized + IWMDRMReaderImpl {
    fn SetEvaluateOutputLevelLicenses(&mut self, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPlayOutputLevels(&mut self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()>;
    fn GetCopyOutputLevels(&mut self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()>;
    fn TryNextLicense(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMReader2Vtbl {
        unsafe extern "system" fn SetEvaluateOutputLevelLicenses<Impl: IWMDRMReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEvaluateOutputLevelLicenses(::core::mem::transmute_copy(&fevaluate)).into()
        }
        unsafe extern "system" fn GetPlayOutputLevels<Impl: IWMDRMReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPlayOutputLevels(::core::mem::transmute_copy(&pplayopl), ::core::mem::transmute_copy(&pcblength), ::core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
        }
        unsafe extern "system" fn GetCopyOutputLevels<Impl: IWMDRMReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCopyOutputLevels(::core::mem::transmute_copy(&pcopyopl), ::core::mem::transmute_copy(&pcblength), ::core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
        }
        unsafe extern "system" fn TryNextLicense<Impl: IWMDRMReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TryNextLicense().into()
        }
        Self {
            base: IWMDRMReaderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetEvaluateOutputLevelLicenses: SetEvaluateOutputLevelLicenses::<Impl, IMPL_OFFSET>,
            GetPlayOutputLevels: GetPlayOutputLevels::<Impl, IMPL_OFFSET>,
            GetCopyOutputLevels: GetCopyOutputLevels::<Impl, IMPL_OFFSET>,
            TryNextLicense: TryNextLicense::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReader3Impl: Sized + IWMDRMReaderImpl + IWMDRMReader2Impl {
    fn GetInclusionList(&mut self, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReader3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMReader3Vtbl {
        unsafe extern "system" fn GetInclusionList<Impl: IWMDRMReader3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInclusionList(::core::mem::transmute_copy(&ppguids), ::core::mem::transmute_copy(&pcguids)).into()
        }
        Self { base: IWMDRMReader2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetInclusionList: GetInclusionList::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMReader3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDRMTranscryptionManagerImpl: Sized {
    fn CreateTranscryptor(&mut self) -> ::windows::core::Result<IWMDRMTranscryptor>;
}
impl IWMDRMTranscryptionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMTranscryptionManagerVtbl {
        unsafe extern "system" fn CreateTranscryptor<Impl: IWMDRMTranscryptionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptranscryptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTranscryptor() {
                ::core::result::Result::Ok(ok__) => {
                    *pptranscryptor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateTranscryptor: CreateTranscryptor::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMTranscryptorImpl: Sized {
    fn Initialize(&mut self, bstrfilename: super::super::Foundation::BSTR, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: ::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Seek(&mut self, hnstime: u64) -> ::windows::core::Result<()>;
    fn Read(&mut self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMTranscryptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMTranscryptorVtbl {
        unsafe extern "system" fn Initialize<Impl: IWMDRMTranscryptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&pblicenserequestmsg), ::core::mem::transmute_copy(&cblicenserequestmsg), ::core::mem::transmute_copy(&pplicenseresponsemsg), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Seek<Impl: IWMDRMTranscryptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Seek(::core::mem::transmute_copy(&hnstime)).into()
        }
        unsafe extern "system" fn Read<Impl: IWMDRMTranscryptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&pcbdata)).into()
        }
        unsafe extern "system" fn Close<Impl: IWMDRMTranscryptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Seek: Seek::<Impl, IMPL_OFFSET>,
            Read: Read::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMTranscryptor2Impl: Sized + IWMDRMTranscryptorImpl {
    fn SeekEx(&mut self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ZeroAdjustTimestamps(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSeekStartTime(&mut self) -> ::windows::core::Result<u64>;
    fn GetDuration(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMTranscryptor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMTranscryptor2Vtbl {
        unsafe extern "system" fn SeekEx<Impl: IWMDRMTranscryptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SeekEx(::core::mem::transmute_copy(&cnsstarttime), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&flrate), ::core::mem::transmute_copy(&fincludefileheader)).into()
        }
        unsafe extern "system" fn ZeroAdjustTimestamps<Impl: IWMDRMTranscryptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ZeroAdjustTimestamps(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetSeekStartTime<Impl: IWMDRMTranscryptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSeekStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnstime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuration<Impl: IWMDRMTranscryptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMDRMTranscryptorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SeekEx: SeekEx::<Impl, IMPL_OFFSET>,
            ZeroAdjustTimestamps: ZeroAdjustTimestamps::<Impl, IMPL_OFFSET>,
            GetSeekStartTime: GetSeekStartTime::<Impl, IMPL_OFFSET>,
            GetDuration: GetDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriterImpl: Sized {
    fn GenerateKeySeed(&mut self, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()>;
    fn GenerateKeyID(&mut self, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()>;
    fn GenerateSigningKeyPair(&mut self, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()>;
    fn SetDRMAttribute(&mut self, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMWriterVtbl {
        unsafe extern "system" fn GenerateKeySeed<Impl: IWMDRMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateKeySeed(::core::mem::transmute_copy(&pwszkeyseed), ::core::mem::transmute_copy(&pcwchlength)).into()
        }
        unsafe extern "system" fn GenerateKeyID<Impl: IWMDRMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateKeyID(::core::mem::transmute_copy(&pwszkeyid), ::core::mem::transmute_copy(&pcwchlength)).into()
        }
        unsafe extern "system" fn GenerateSigningKeyPair<Impl: IWMDRMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GenerateSigningKeyPair(::core::mem::transmute_copy(&pwszprivkey), ::core::mem::transmute_copy(&pcwchprivkeylength), ::core::mem::transmute_copy(&pwszpubkey), ::core::mem::transmute_copy(&pcwchpubkeylength)).into()
        }
        unsafe extern "system" fn SetDRMAttribute<Impl: IWMDRMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDRMAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GenerateKeySeed: GenerateKeySeed::<Impl, IMPL_OFFSET>,
            GenerateKeyID: GenerateKeyID::<Impl, IMPL_OFFSET>,
            GenerateSigningKeyPair: GenerateSigningKeyPair::<Impl, IMPL_OFFSET>,
            SetDRMAttribute: SetDRMAttribute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter2Impl: Sized + IWMDRMWriterImpl {
    fn SetWMDRMNetEncryption(&mut self, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMWriter2Vtbl {
        unsafe extern "system" fn SetWMDRMNetEncryption<Impl: IWMDRMWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWMDRMNetEncryption(::core::mem::transmute_copy(&fsamplesencrypted), ::core::mem::transmute_copy(&pbkeyid), ::core::mem::transmute_copy(&cbkeyid)).into()
        }
        Self { base: IWMDRMWriterVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetWMDRMNetEncryption: SetWMDRMNetEncryption::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter3Impl: Sized + IWMDRMWriterImpl + IWMDRMWriter2Impl {
    fn SetProtectStreamSamples(&mut self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMWriter3Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Impl: IWMDRMWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectStreamSamples(::core::mem::transmute_copy(&pimportinitstruct)).into()
        }
        Self {
            base: IWMDRMWriter2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProtectStreamSamples: SetProtectStreamSamples::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDeviceRegistrationImpl: Sized {
    fn RegisterDevice(&mut self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::Result<IWMRegisteredDevice>;
    fn UnregisterDevice(&mut self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::Result<()>;
    fn GetRegistrationStats(&mut self, dwregistertype: u32) -> ::windows::core::Result<u32>;
    fn GetFirstRegisteredDevice(&mut self, dwregistertype: u32) -> ::windows::core::Result<IWMRegisteredDevice>;
    fn GetNextRegisteredDevice(&mut self) -> ::windows::core::Result<IWMRegisteredDevice>;
    fn GetRegisteredDeviceByID(&mut self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::Result<IWMRegisteredDevice>;
}
impl IWMDeviceRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDeviceRegistrationVtbl {
        unsafe extern "system" fn RegisterDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDevice(::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&serialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterDevice(::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&serialnumber)).into()
        }
        unsafe extern "system" fn GetRegistrationStats<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegistrationStats(::core::mem::transmute_copy(&dwregistertype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcregistereddevices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstRegisteredDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFirstRegisteredDevice(::core::mem::transmute_copy(&dwregistertype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextRegisteredDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextRegisteredDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredDeviceByID<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRegisteredDeviceByID(::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&serialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterDevice: RegisterDevice::<Impl, IMPL_OFFSET>,
            UnregisterDevice: UnregisterDevice::<Impl, IMPL_OFFSET>,
            GetRegistrationStats: GetRegistrationStats::<Impl, IMPL_OFFSET>,
            GetFirstRegisteredDevice: GetFirstRegisteredDevice::<Impl, IMPL_OFFSET>,
            GetNextRegisteredDevice: GetNextRegisteredDevice::<Impl, IMPL_OFFSET>,
            GetRegisteredDeviceByID: GetRegisteredDeviceByID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDeviceRegistration as ::windows::core::Interface>::IID
    }
}
pub trait IWMGetSecureChannelImpl: Sized {
    fn GetPeerSecureChannelInterface(&mut self) -> ::windows::core::Result<IWMSecureChannel>;
}
impl IWMGetSecureChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMGetSecureChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMGetSecureChannelVtbl {
        unsafe extern "system" fn GetPeerSecureChannelInterface<Impl: IWMGetSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppeer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPeerSecureChannelInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *pppeer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPeerSecureChannelInterface: GetPeerSecureChannelInterface::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMGetSecureChannel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMHeaderInfoImpl: Sized {
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
impl IWMHeaderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMHeaderInfoVtbl {
        unsafe extern "system" fn GetAttributeCount<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeCount(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributeByIndex(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwstreamnum), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetAttributeByName<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributeByName(::core::mem::transmute_copy(&pwstreamnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetAttribute<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetMarkerCount<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMarkerCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcmarkers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarker<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMarker(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszmarkername), ::core::mem::transmute_copy(&pcchmarkernamelen), ::core::mem::transmute_copy(&pcnsmarkertime)).into()
        }
        unsafe extern "system" fn AddMarker<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMarker(::core::mem::transmute_copy(&pwszmarkername), ::core::mem::transmute_copy(&cnsmarkertime)).into()
        }
        unsafe extern "system" fn RemoveMarker<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMarker(::core::mem::transmute_copy(&windex)).into()
        }
        unsafe extern "system" fn GetScriptCount<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScriptCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcscripts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScript<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetScript(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwsztype), ::core::mem::transmute_copy(&pcchtypelen), ::core::mem::transmute_copy(&pwszcommand), ::core::mem::transmute_copy(&pcchcommandlen), ::core::mem::transmute_copy(&pcnsscripttime)).into()
        }
        unsafe extern "system" fn AddScript<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddScript(::core::mem::transmute_copy(&pwsztype), ::core::mem::transmute_copy(&pwszcommand), ::core::mem::transmute_copy(&cnsscripttime)).into()
        }
        unsafe extern "system" fn RemoveScript<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveScript(::core::mem::transmute_copy(&windex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAttributeCount: GetAttributeCount::<Impl, IMPL_OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Impl, IMPL_OFFSET>,
            GetAttributeByName: GetAttributeByName::<Impl, IMPL_OFFSET>,
            SetAttribute: SetAttribute::<Impl, IMPL_OFFSET>,
            GetMarkerCount: GetMarkerCount::<Impl, IMPL_OFFSET>,
            GetMarker: GetMarker::<Impl, IMPL_OFFSET>,
            AddMarker: AddMarker::<Impl, IMPL_OFFSET>,
            RemoveMarker: RemoveMarker::<Impl, IMPL_OFFSET>,
            GetScriptCount: GetScriptCount::<Impl, IMPL_OFFSET>,
            GetScript: GetScript::<Impl, IMPL_OFFSET>,
            AddScript: AddScript::<Impl, IMPL_OFFSET>,
            RemoveScript: RemoveScript::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMHeaderInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMHeaderInfo2Impl: Sized + IWMHeaderInfoImpl {
    fn GetCodecInfoCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetCodecInfo(&mut self, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMHeaderInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMHeaderInfo2Vtbl {
        unsafe extern "system" fn GetCodecInfoCount<Impl: IWMHeaderInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCodecInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pccodecinfos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecInfo<Impl: IWMHeaderInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCodecInfo(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchdescription), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pcodectype), ::core::mem::transmute_copy(&pcbcodecinfo), ::core::mem::transmute_copy(&pbcodecinfo)).into()
        }
        Self {
            base: IWMHeaderInfoVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Impl, IMPL_OFFSET>,
            GetCodecInfo: GetCodecInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMHeaderInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMHeaderInfo3Impl: Sized + IWMHeaderInfoImpl + IWMHeaderInfo2Impl {
    fn GetAttributeCountEx(&mut self, wstreamnum: u16) -> ::windows::core::Result<u16>;
    fn GetAttributeIndices(&mut self, wstreamnum: u16, pwszname: super::super::Foundation::PWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::Result<()>;
    fn GetAttributeByIndexEx(&mut self, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::Result<()>;
    fn ModifyAttribute(&mut self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::Result<()>;
    fn AddAttribute(&mut self, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::Result<()>;
    fn DeleteAttribute(&mut self, wstreamnum: u16, windex: u16) -> ::windows::core::Result<()>;
    fn AddCodecInfo(&mut self, pwszname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMHeaderInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMHeaderInfo3Vtbl {
        unsafe extern "system" fn GetAttributeCountEx<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeCountEx(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeIndices<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: super::super::Foundation::PWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributeIndices(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwlangindex), ::core::mem::transmute_copy(&pwindices), ::core::mem::transmute_copy(&pwcount)).into()
        }
        unsafe extern "system" fn GetAttributeByIndexEx<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributeByIndexEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pwlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwdatalength)).into()
        }
        unsafe extern "system" fn ModifyAttribute<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ModifyAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&wlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn AddAttribute<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pwindex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&wlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn DeleteAttribute<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex)).into()
        }
        unsafe extern "system" fn AddCodecInfo<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddCodecInfo(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&codectype), ::core::mem::transmute_copy(&cbcodecinfo), ::core::mem::transmute_copy(&pbcodecinfo)).into()
        }
        Self {
            base: IWMHeaderInfo2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAttributeCountEx: GetAttributeCountEx::<Impl, IMPL_OFFSET>,
            GetAttributeIndices: GetAttributeIndices::<Impl, IMPL_OFFSET>,
            GetAttributeByIndexEx: GetAttributeByIndexEx::<Impl, IMPL_OFFSET>,
            ModifyAttribute: ModifyAttribute::<Impl, IMPL_OFFSET>,
            AddAttribute: AddAttribute::<Impl, IMPL_OFFSET>,
            DeleteAttribute: DeleteAttribute::<Impl, IMPL_OFFSET>,
            AddCodecInfo: AddCodecInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMHeaderInfo3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMIStreamPropsImpl: Sized {
    fn GetProperty(&mut self, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMIStreamPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIStreamPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMIStreamPropsVtbl {
        unsafe extern "system" fn GetProperty<Impl: IWMIStreamPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetProperty: GetProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIStreamProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMImageInfoImpl: Sized {
    fn GetImageCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetImage(&mut self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMImageInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMImageInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMImageInfoVtbl {
        unsafe extern "system" fn GetImageCount<Impl: IWMImageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetImageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcimages = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImage<Impl: IWMImageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetImage(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pcchmimetype), ::core::mem::transmute_copy(&pwszmimetype), ::core::mem::transmute_copy(&pcchdescription), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pimagetype), ::core::mem::transmute_copy(&pcbimagedata), ::core::mem::transmute_copy(&pbimagedata)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetImageCount: GetImageCount::<Impl, IMPL_OFFSET>,
            GetImage: GetImage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMImageInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMIndexerImpl: Sized {
    fn StartIndexing(&mut self, pwszurl: super::super::Foundation::PWSTR, pcallback: ::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMIndexerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIndexerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMIndexerVtbl {
        unsafe extern "system" fn StartIndexing<Impl: IWMIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartIndexing(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Cancel<Impl: IWMIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            StartIndexing: StartIndexing::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIndexer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMIndexer2Impl: Sized + IWMIndexerImpl {
    fn Configure(&mut self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMIndexer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIndexer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMIndexer2Vtbl {
        unsafe extern "system" fn Configure<Impl: IWMIndexer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Configure(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&nindexertype), ::core::mem::transmute_copy(&pvinterval), ::core::mem::transmute_copy(&pvindextype)).into()
        }
        Self { base: IWMIndexerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Configure: Configure::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIndexer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMInputMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetConnectionName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
    fn GetGroupName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMInputMediaPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMInputMediaPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMInputMediaPropsVtbl {
        unsafe extern "system" fn GetConnectionName<Impl: IWMInputMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConnectionName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetGroupName<Impl: IWMInputMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGroupName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        Self {
            base: IWMMediaPropsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetConnectionName: GetConnectionName::<Impl, IMPL_OFFSET>,
            GetGroupName: GetGroupName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMInputMediaProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMLanguageListImpl: Sized {
    fn GetLanguageCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetLanguageDetails(&mut self, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()>;
    fn AddLanguageByRFC1766String(&mut self, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::core::Result<u16>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMLanguageListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLanguageListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMLanguageListVtbl {
        unsafe extern "system" fn GetLanguageCount<Impl: IWMLanguageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguageCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageDetails<Impl: IWMLanguageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLanguageDetails(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn AddLanguageByRFC1766String<Impl: IWMLanguageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pwindex: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddLanguageByRFC1766String(::core::mem::transmute_copy(&pwszlanguagestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLanguageCount: GetLanguageCount::<Impl, IMPL_OFFSET>,
            GetLanguageDetails: GetLanguageDetails::<Impl, IMPL_OFFSET>,
            AddLanguageByRFC1766String: AddLanguageByRFC1766String::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLanguageList as ::windows::core::Interface>::IID
    }
}
pub trait IWMLicenseBackupImpl: Sized {
    fn BackupLicenses(&mut self, dwflags: u32, pcallback: ::core::option::Option<IWMStatusCallback>) -> ::windows::core::Result<()>;
    fn CancelLicenseBackup(&mut self) -> ::windows::core::Result<()>;
}
impl IWMLicenseBackupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseBackupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMLicenseBackupVtbl {
        unsafe extern "system" fn BackupLicenses<Impl: IWMLicenseBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackupLicenses(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn CancelLicenseBackup<Impl: IWMLicenseBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelLicenseBackup().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BackupLicenses: BackupLicenses::<Impl, IMPL_OFFSET>,
            CancelLicenseBackup: CancelLicenseBackup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLicenseBackup as ::windows::core::Interface>::IID
    }
}
pub trait IWMLicenseRestoreImpl: Sized {
    fn RestoreLicenses(&mut self, dwflags: u32, pcallback: ::core::option::Option<IWMStatusCallback>) -> ::windows::core::Result<()>;
    fn CancelLicenseRestore(&mut self) -> ::windows::core::Result<()>;
}
impl IWMLicenseRestoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRestoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMLicenseRestoreVtbl {
        unsafe extern "system" fn RestoreLicenses<Impl: IWMLicenseRestoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreLicenses(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pcallback)).into()
        }
        unsafe extern "system" fn CancelLicenseRestore<Impl: IWMLicenseRestoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelLicenseRestore().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RestoreLicenses: RestoreLicenses::<Impl, IMPL_OFFSET>,
            CancelLicenseRestore: CancelLicenseRestore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLicenseRestore as ::windows::core::Interface>::IID
    }
}
pub trait IWMLicenseRevocationAgentImpl: Sized {
    fn GetLRBChallenge(&mut self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::Result<()>;
    fn ProcessLRB(&mut self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::Result<()>;
}
impl IWMLicenseRevocationAgentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRevocationAgentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMLicenseRevocationAgentVtbl {
        unsafe extern "system" fn GetLRBChallenge<Impl: IWMLicenseRevocationAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLRBChallenge(::core::mem::transmute_copy(&pmachineid), ::core::mem::transmute_copy(&dwmachineidlength), ::core::mem::transmute_copy(&pchallenge), ::core::mem::transmute_copy(&dwchallengelength), ::core::mem::transmute_copy(&pchallengeoutput), ::core::mem::transmute_copy(&pdwchallengeoutputlength)).into()
        }
        unsafe extern "system" fn ProcessLRB<Impl: IWMLicenseRevocationAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessLRB(::core::mem::transmute_copy(&psignedlrb), ::core::mem::transmute_copy(&dwsignedlrblength), ::core::mem::transmute_copy(&psignedack), ::core::mem::transmute_copy(&pdwsignedacklength)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLRBChallenge: GetLRBChallenge::<Impl, IMPL_OFFSET>,
            ProcessLRB: ProcessLRB::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLicenseRevocationAgent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMediaPropsImpl: Sized {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetMediaType(&mut self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()>;
    fn SetMediaType(&mut self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMediaPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMediaPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMediaPropsVtbl {
        unsafe extern "system" fn GetType<Impl: IWMMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMediaType<Impl: IWMMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMediaType(::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pcbtype)).into()
        }
        unsafe extern "system" fn SetMediaType<Impl: IWMMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaType(::core::mem::transmute_copy(&ptype)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetMediaType: GetMediaType::<Impl, IMPL_OFFSET>,
            SetMediaType: SetMediaType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMediaProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMetadataEditorImpl: Sized {
    fn Open(&mut self, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMetadataEditorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMetadataEditorVtbl {
        unsafe extern "system" fn Open<Impl: IWMMetadataEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        unsafe extern "system" fn Close<Impl: IWMMetadataEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Flush<Impl: IWMMetadataEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMetadataEditor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMetadataEditor2Impl: Sized + IWMMetadataEditorImpl {
    fn OpenEx(&mut self, pwszfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMetadataEditor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMetadataEditor2Vtbl {
        unsafe extern "system" fn OpenEx<Impl: IWMMetadataEditor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenEx(::core::mem::transmute_copy(&pwszfilename), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&dwsharemode)).into()
        }
        Self { base: IWMMetadataEditorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OpenEx: OpenEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMetadataEditor2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMMutualExclusionImpl: Sized + IWMStreamListImpl {
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetType(&mut self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IWMMutualExclusionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMutualExclusionVtbl {
        unsafe extern "system" fn GetType<Impl: IWMMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IWMMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&guidtype)).into()
        }
        Self {
            base: IWMStreamListVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMutualExclusion as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMutualExclusion2Impl: Sized + IWMStreamListImpl + IWMMutualExclusionImpl {
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
impl IWMMutualExclusion2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMutualExclusion2Vtbl {
        unsafe extern "system" fn GetName<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn SetName<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn GetRecordCount<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pwrecordcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRecord().into()
        }
        unsafe extern "system" fn RemoveRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveRecord(::core::mem::transmute_copy(&wrecordnumber)).into()
        }
        unsafe extern "system" fn GetRecordName<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRecordName(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwszrecordname), ::core::mem::transmute_copy(&pcchrecordname)).into()
        }
        unsafe extern "system" fn SetRecordName<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecordName(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwszrecordname)).into()
        }
        unsafe extern "system" fn GetStreamsForRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreamsForRecord(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwstreamnumarray), ::core::mem::transmute_copy(&pcstreams)).into()
        }
        unsafe extern "system" fn AddStreamForRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStreamForRecord(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&wstreamnumber)).into()
        }
        unsafe extern "system" fn RemoveStreamForRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStreamForRecord(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&wstreamnumber)).into()
        }
        Self {
            base: IWMMutualExclusionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            GetRecordCount: GetRecordCount::<Impl, IMPL_OFFSET>,
            AddRecord: AddRecord::<Impl, IMPL_OFFSET>,
            RemoveRecord: RemoveRecord::<Impl, IMPL_OFFSET>,
            GetRecordName: GetRecordName::<Impl, IMPL_OFFSET>,
            SetRecordName: SetRecordName::<Impl, IMPL_OFFSET>,
            GetStreamsForRecord: GetStreamsForRecord::<Impl, IMPL_OFFSET>,
            AddStreamForRecord: AddStreamForRecord::<Impl, IMPL_OFFSET>,
            RemoveStreamForRecord: RemoveStreamForRecord::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMutualExclusion2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMOutputMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetStreamGroupName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
    fn GetConnectionName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMOutputMediaPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMOutputMediaPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMOutputMediaPropsVtbl {
        unsafe extern "system" fn GetStreamGroupName<Impl: IWMOutputMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreamGroupName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetConnectionName<Impl: IWMOutputMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConnectionName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        Self {
            base: IWMMediaPropsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStreamGroupName: GetStreamGroupName::<Impl, IMPL_OFFSET>,
            GetConnectionName: GetConnectionName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMOutputMediaProps as ::windows::core::Interface>::IID
    }
}
pub trait IWMPacketSizeImpl: Sized {
    fn GetMaxPacketSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxPacketSize(&mut self, dwmaxpacketsize: u32) -> ::windows::core::Result<()>;
}
impl IWMPacketSizeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSizeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPacketSizeVtbl {
        unsafe extern "system" fn GetMaxPacketSize<Impl: IWMPacketSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxpacketsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPacketSize<Impl: IWMPacketSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPacketSize(::core::mem::transmute_copy(&dwmaxpacketsize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMaxPacketSize: GetMaxPacketSize::<Impl, IMPL_OFFSET>,
            SetMaxPacketSize: SetMaxPacketSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPacketSize as ::windows::core::Interface>::IID
    }
}
pub trait IWMPacketSize2Impl: Sized + IWMPacketSizeImpl {
    fn GetMinPacketSize(&mut self) -> ::windows::core::Result<u32>;
    fn SetMinPacketSize(&mut self, dwminpacketsize: u32) -> ::windows::core::Result<()>;
}
impl IWMPacketSize2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSize2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPacketSize2Vtbl {
        unsafe extern "system" fn GetMinPacketSize<Impl: IWMPacketSize2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwminpacketsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPacketSize<Impl: IWMPacketSize2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinPacketSize(::core::mem::transmute_copy(&dwminpacketsize)).into()
        }
        Self {
            base: IWMPacketSizeVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMinPacketSize: GetMinPacketSize::<Impl, IMPL_OFFSET>,
            SetMinPacketSize: SetMinPacketSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPacketSize2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMPlayerHookImpl: Sized {
    fn PreDecode(&mut self) -> ::windows::core::Result<()>;
}
impl IWMPlayerHookVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPlayerHookImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPlayerHookVtbl {
        unsafe extern "system" fn PreDecode<Impl: IWMPlayerHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreDecode().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), PreDecode: PreDecode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPlayerHook as ::windows::core::Interface>::IID
    }
}
pub trait IWMPlayerTimestampHookImpl: Sized {
    fn MapTimestamp(&mut self, rtin: i64) -> ::windows::core::Result<i64>;
}
impl IWMPlayerTimestampHookVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPlayerTimestampHookImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPlayerTimestampHookVtbl {
        unsafe extern "system" fn MapTimestamp<Impl: IWMPlayerTimestampHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapTimestamp(::core::mem::transmute_copy(&rtin)) {
                ::core::result::Result::Ok(ok__) => {
                    *prtout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), MapTimestamp: MapTimestamp::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPlayerTimestampHook as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfileImpl: Sized {
    fn GetVersion(&mut self) -> ::windows::core::Result<WMT_VERSION>;
    fn GetName(&mut self, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()>;
    fn SetName(&mut self, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetDescription(&mut self, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()>;
    fn SetDescription(&mut self, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetStreamCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetStream(&mut self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig>;
    fn GetStreamByNumber(&mut self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig>;
    fn RemoveStream(&mut self, pconfig: ::core::option::Option<IWMStreamConfig>) -> ::windows::core::Result<()>;
    fn RemoveStreamByNumber(&mut self, wstreamnum: u16) -> ::windows::core::Result<()>;
    fn AddStream(&mut self, pconfig: ::core::option::Option<IWMStreamConfig>) -> ::windows::core::Result<()>;
    fn ReconfigStream(&mut self, pconfig: ::core::option::Option<IWMStreamConfig>) -> ::windows::core::Result<()>;
    fn CreateNewStream(&mut self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig>;
    fn GetMutualExclusionCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetMutualExclusion(&mut self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion>;
    fn RemoveMutualExclusion(&mut self, pme: ::core::option::Option<IWMMutualExclusion>) -> ::windows::core::Result<()>;
    fn AddMutualExclusion(&mut self, pme: ::core::option::Option<IWMMutualExclusion>) -> ::windows::core::Result<()>;
    fn CreateNewMutualExclusion(&mut self) -> ::windows::core::Result<IWMMutualExclusion>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfileVtbl {
        unsafe extern "system" fn GetVersion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn SetName<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&pwszname)).into()
        }
        unsafe extern "system" fn GetDescription<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDescription(::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pcchdescription)).into()
        }
        unsafe extern "system" fn SetDescription<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&pwszdescription)).into()
        }
        unsafe extern "system" fn GetStreamCount<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcstreams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStream(::core::mem::transmute_copy(&dwstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamByNumber<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamByNumber(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute(&pconfig)).into()
        }
        unsafe extern "system" fn RemoveStreamByNumber<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStreamByNumber(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn AddStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStream(::core::mem::transmute(&pconfig)).into()
        }
        unsafe extern "system" fn ReconfigStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReconfigStream(::core::mem::transmute(&pconfig)).into()
        }
        unsafe extern "system" fn CreateNewStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewStream(::core::mem::transmute_copy(&guidstreamtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusionCount<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMutualExclusionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcme = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMutualExclusion(::core::mem::transmute_copy(&dwmeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppme = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMutualExclusion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveMutualExclusion(::core::mem::transmute(&pme)).into()
        }
        unsafe extern "system" fn AddMutualExclusion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMutualExclusion(::core::mem::transmute(&pme)).into()
        }
        unsafe extern "system" fn CreateNewMutualExclusion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewMutualExclusion() {
                ::core::result::Result::Ok(ok__) => {
                    *ppme = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetVersion: GetVersion::<Impl, IMPL_OFFSET>,
            GetName: GetName::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            GetStreamCount: GetStreamCount::<Impl, IMPL_OFFSET>,
            GetStream: GetStream::<Impl, IMPL_OFFSET>,
            GetStreamByNumber: GetStreamByNumber::<Impl, IMPL_OFFSET>,
            RemoveStream: RemoveStream::<Impl, IMPL_OFFSET>,
            RemoveStreamByNumber: RemoveStreamByNumber::<Impl, IMPL_OFFSET>,
            AddStream: AddStream::<Impl, IMPL_OFFSET>,
            ReconfigStream: ReconfigStream::<Impl, IMPL_OFFSET>,
            CreateNewStream: CreateNewStream::<Impl, IMPL_OFFSET>,
            GetMutualExclusionCount: GetMutualExclusionCount::<Impl, IMPL_OFFSET>,
            GetMutualExclusion: GetMutualExclusion::<Impl, IMPL_OFFSET>,
            RemoveMutualExclusion: RemoveMutualExclusion::<Impl, IMPL_OFFSET>,
            AddMutualExclusion: AddMutualExclusion::<Impl, IMPL_OFFSET>,
            CreateNewMutualExclusion: CreateNewMutualExclusion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfile2Impl: Sized + IWMProfileImpl {
    fn GetProfileID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfile2Vtbl {
        unsafe extern "system" fn GetProfileID<Impl: IWMProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProfileID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWMProfileVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetProfileID: GetProfileID::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfile3Impl: Sized + IWMProfileImpl + IWMProfile2Impl {
    fn GetStorageFormat(&mut self) -> ::windows::core::Result<WMT_STORAGE_FORMAT>;
    fn SetStorageFormat(&mut self, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::Result<()>;
    fn GetBandwidthSharingCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetBandwidthSharing(&mut self, dwbsindex: u32) -> ::windows::core::Result<IWMBandwidthSharing>;
    fn RemoveBandwidthSharing(&mut self, pbs: ::core::option::Option<IWMBandwidthSharing>) -> ::windows::core::Result<()>;
    fn AddBandwidthSharing(&mut self, pbs: ::core::option::Option<IWMBandwidthSharing>) -> ::windows::core::Result<()>;
    fn CreateNewBandwidthSharing(&mut self) -> ::windows::core::Result<IWMBandwidthSharing>;
    fn GetStreamPrioritization(&mut self) -> ::windows::core::Result<IWMStreamPrioritization>;
    fn SetStreamPrioritization(&mut self, psp: ::core::option::Option<IWMStreamPrioritization>) -> ::windows::core::Result<()>;
    fn RemoveStreamPrioritization(&mut self) -> ::windows::core::Result<()>;
    fn CreateNewStreamPrioritization(&mut self) -> ::windows::core::Result<IWMStreamPrioritization>;
    fn GetExpectedPacketCount(&mut self, msduration: u64) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfile3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfile3Vtbl {
        unsafe extern "system" fn GetStorageFormat<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *pnstorageformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageFormat<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStorageFormat(::core::mem::transmute_copy(&nstorageformat)).into()
        }
        unsafe extern "system" fn GetBandwidthSharingCount<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBandwidthSharingCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBandwidthSharing(::core::mem::transmute_copy(&dwbsindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveBandwidthSharing(::core::mem::transmute(&pbs)).into()
        }
        unsafe extern "system" fn AddBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddBandwidthSharing(::core::mem::transmute(&pbs)).into()
        }
        unsafe extern "system" fn CreateNewBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewBandwidthSharing() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamPrioritization() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamPrioritization(::core::mem::transmute(&psp)).into()
        }
        unsafe extern "system" fn RemoveStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStreamPrioritization().into()
        }
        unsafe extern "system" fn CreateNewStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNewStreamPrioritization() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpectedPacketCount<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExpectedPacketCount(::core::mem::transmute_copy(&msduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcpackets = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMProfile2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStorageFormat: GetStorageFormat::<Impl, IMPL_OFFSET>,
            SetStorageFormat: SetStorageFormat::<Impl, IMPL_OFFSET>,
            GetBandwidthSharingCount: GetBandwidthSharingCount::<Impl, IMPL_OFFSET>,
            GetBandwidthSharing: GetBandwidthSharing::<Impl, IMPL_OFFSET>,
            RemoveBandwidthSharing: RemoveBandwidthSharing::<Impl, IMPL_OFFSET>,
            AddBandwidthSharing: AddBandwidthSharing::<Impl, IMPL_OFFSET>,
            CreateNewBandwidthSharing: CreateNewBandwidthSharing::<Impl, IMPL_OFFSET>,
            GetStreamPrioritization: GetStreamPrioritization::<Impl, IMPL_OFFSET>,
            SetStreamPrioritization: SetStreamPrioritization::<Impl, IMPL_OFFSET>,
            RemoveStreamPrioritization: RemoveStreamPrioritization::<Impl, IMPL_OFFSET>,
            CreateNewStreamPrioritization: CreateNewStreamPrioritization::<Impl, IMPL_OFFSET>,
            GetExpectedPacketCount: GetExpectedPacketCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfile3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfileManagerImpl: Sized {
    fn CreateEmptyProfile(&mut self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile>;
    fn LoadProfileByID(&mut self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile>;
    fn LoadProfileByData(&mut self, pwszprofile: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWMProfile>;
    fn SaveProfile(&mut self, piwmprofile: ::core::option::Option<IWMProfile>, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetSystemProfileCount(&mut self) -> ::windows::core::Result<u32>;
    fn LoadSystemProfile(&mut self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfileManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfileManagerVtbl {
        unsafe extern "system" fn CreateEmptyProfile<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateEmptyProfile(::core::mem::transmute_copy(&dwversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByID<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadProfileByID(::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByData<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprofile: super::super::Foundation::PWSTR, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadProfileByData(::core::mem::transmute_copy(&pwszprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveProfile<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmprofile: ::windows::core::RawPtr, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveProfile(::core::mem::transmute(&piwmprofile), ::core::mem::transmute_copy(&pwszprofile), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSystemProfileCount<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSystemProfileCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcprofiles = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSystemProfile<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadSystemProfile(::core::mem::transmute_copy(&dwprofileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprofile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateEmptyProfile: CreateEmptyProfile::<Impl, IMPL_OFFSET>,
            LoadProfileByID: LoadProfileByID::<Impl, IMPL_OFFSET>,
            LoadProfileByData: LoadProfileByData::<Impl, IMPL_OFFSET>,
            SaveProfile: SaveProfile::<Impl, IMPL_OFFSET>,
            GetSystemProfileCount: GetSystemProfileCount::<Impl, IMPL_OFFSET>,
            LoadSystemProfile: LoadSystemProfile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfileManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfileManager2Impl: Sized + IWMProfileManagerImpl {
    fn GetSystemProfileVersion(&mut self, pdwversion: *mut WMT_VERSION) -> ::windows::core::Result<()>;
    fn SetSystemProfileVersion(&mut self, dwversion: WMT_VERSION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfileManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfileManager2Vtbl {
        unsafe extern "system" fn GetSystemProfileVersion<Impl: IWMProfileManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSystemProfileVersion(::core::mem::transmute_copy(&pdwversion)).into()
        }
        unsafe extern "system" fn SetSystemProfileVersion<Impl: IWMProfileManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSystemProfileVersion(::core::mem::transmute_copy(&dwversion)).into()
        }
        Self {
            base: IWMProfileManagerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSystemProfileVersion: GetSystemProfileVersion::<Impl, IMPL_OFFSET>,
            SetSystemProfileVersion: SetSystemProfileVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfileManager2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMProfileManagerLanguageImpl: Sized {
    fn GetUserLanguageID(&mut self, wlangid: *mut u16) -> ::windows::core::Result<()>;
    fn SetUserLanguageID(&mut self, wlangid: u16) -> ::windows::core::Result<()>;
}
impl IWMProfileManagerLanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManagerLanguageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfileManagerLanguageVtbl {
        unsafe extern "system" fn GetUserLanguageID<Impl: IWMProfileManagerLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUserLanguageID(::core::mem::transmute_copy(&wlangid)).into()
        }
        unsafe extern "system" fn SetUserLanguageID<Impl: IWMProfileManagerLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserLanguageID(::core::mem::transmute_copy(&wlangid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetUserLanguageID: GetUserLanguageID::<Impl, IMPL_OFFSET>,
            SetUserLanguageID: SetUserLanguageID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfileManagerLanguage as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMPropertyVaultImpl: Sized {
    fn GetPropertyCount(&mut self, pdwcount: *const u32) -> ::windows::core::Result<()>;
    fn GetPropertyByName(&mut self, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn SetProperty(&mut self, pszname: super::super::Foundation::PWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn GetPropertyByIndex(&mut self, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn CopyPropertiesFrom(&mut self, piwmpropertyvault: ::core::option::Option<IWMPropertyVault>) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPropertyVaultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPropertyVaultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPropertyVaultVtbl {
        unsafe extern "system" fn GetPropertyCount<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyCount(::core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn GetPropertyByName<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyByName(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn SetProperty<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetPropertyByIndex<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPropertyByIndex(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pdwnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn CopyPropertiesFrom<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpropertyvault: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyPropertiesFrom(::core::mem::transmute(&piwmpropertyvault)).into()
        }
        unsafe extern "system" fn Clear<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Impl, IMPL_OFFSET>,
            GetPropertyByName: GetPropertyByName::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Impl, IMPL_OFFSET>,
            CopyPropertiesFrom: CopyPropertiesFrom::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPropertyVault as ::windows::core::Interface>::IID
    }
}
pub trait IWMProximityDetectionImpl: Sized {
    fn StartDetection(&mut self, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: ::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMProximityDetectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProximityDetectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProximityDetectionVtbl {
        unsafe extern "system" fn StartDetection<Impl: IWMProximityDetectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartDetection(::core::mem::transmute_copy(&pbregistrationmsg), ::core::mem::transmute_copy(&cbregistrationmsg), ::core::mem::transmute_copy(&pblocaladdress), ::core::mem::transmute_copy(&cblocaladdress), ::core::mem::transmute_copy(&dwextraportsallowed), ::core::mem::transmute_copy(&ppregistrationresponsemsg), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), StartDetection: StartDetection::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProximityDetection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderImpl: Sized {
    fn Open(&mut self, pwszurl: super::super::Foundation::PWSTR, pcallback: ::core::option::Option<IWMReaderCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn GetOutputCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetOutputProps(&mut self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(&mut self, dwoutputnum: u32, poutput: ::core::option::Option<IWMOutputMediaProps>) -> ::windows::core::Result<()>;
    fn GetOutputFormatCount(&mut self, dwoutputnumber: u32) -> ::windows::core::Result<u32>;
    fn GetOutputFormat(&mut self, dwoutputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn Start(&mut self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Pause(&mut self) -> ::windows::core::Result<()>;
    fn Resume(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderVtbl {
        unsafe extern "system" fn Open<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Close<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn GetOutputCount<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcoutputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputProps(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputProps(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetOutputFormatCount<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputFormatCount(::core::mem::transmute_copy(&dwoutputnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputFormat(::core::mem::transmute_copy(&dwoutputnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&cnsstart), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Stop<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Pause<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Pause().into()
        }
        unsafe extern "system" fn Resume<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Resume().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            GetOutputCount: GetOutputCount::<Impl, IMPL_OFFSET>,
            GetOutputProps: GetOutputProps::<Impl, IMPL_OFFSET>,
            SetOutputProps: SetOutputProps::<Impl, IMPL_OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Impl, IMPL_OFFSET>,
            GetOutputFormat: GetOutputFormat::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Pause: Pause::<Impl, IMPL_OFFSET>,
            Resume: Resume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderAcceleratorImpl: Sized {
    fn GetCodecInterface(&mut self, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Notify(&mut self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderAcceleratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAcceleratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAcceleratorVtbl {
        unsafe extern "system" fn GetCodecInterface<Impl: IWMReaderAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCodecInterface(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvcodecinterface)).into()
        }
        unsafe extern "system" fn Notify<Impl: IWMReaderAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Notify(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&psubtype)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCodecInterface: GetCodecInterface::<Impl, IMPL_OFFSET>,
            Notify: Notify::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAccelerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderAdvancedImpl: Sized {
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
impl IWMReaderAdvancedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvancedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvancedVtbl {
        unsafe extern "system" fn SetUserProvidedClock<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserProvidedClock(::core::mem::transmute_copy(&fuserclock)).into()
        }
        unsafe extern "system" fn GetUserProvidedClock<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUserProvidedClock() {
                ::core::result::Result::Ok(ok__) => {
                    *pfuserclock = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeliverTime<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeliverTime(::core::mem::transmute_copy(&cnstime)).into()
        }
        unsafe extern "system" fn SetManualStreamSelection<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManualStreamSelection(::core::mem::transmute_copy(&fselection)).into()
        }
        unsafe extern "system" fn GetManualStreamSelection<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetManualStreamSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pfselection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamsSelected<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamsSelected(::core::mem::transmute_copy(&cstreamcount), ::core::mem::transmute_copy(&pwstreamnumbers), ::core::mem::transmute_copy(&pselections)).into()
        }
        unsafe extern "system" fn GetStreamSelected<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamSelected(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pselection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveSelectionCallbacks<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReceiveSelectionCallbacks(::core::mem::transmute_copy(&fgetcallbacks)).into()
        }
        unsafe extern "system" fn GetReceiveSelectionCallbacks<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReceiveSelectionCallbacks() {
                ::core::result::Result::Ok(ok__) => {
                    *pfgetcallbacks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveStreamSamples<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReceiveStreamSamples(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&freceivestreamsamples)).into()
        }
        unsafe extern "system" fn GetReceiveStreamSamples<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReceiveStreamSamples(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfreceivestreamsamples = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForOutput<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfallocate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllocateForStream(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForStream<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllocateForStream(::core::mem::transmute_copy(&dwsreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfallocate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatistics(::core::mem::transmute_copy(&pstatistics)).into()
        }
        unsafe extern "system" fn SetClientInfo<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientInfo(::core::mem::transmute_copy(&pclientinfo)).into()
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxOutputSampleSize(::core::mem::transmute_copy(&dwoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxStreamSampleSize(::core::mem::transmute_copy(&wstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyLateDelivery<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyLateDelivery(::core::mem::transmute_copy(&cnslateness)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetUserProvidedClock: SetUserProvidedClock::<Impl, IMPL_OFFSET>,
            GetUserProvidedClock: GetUserProvidedClock::<Impl, IMPL_OFFSET>,
            DeliverTime: DeliverTime::<Impl, IMPL_OFFSET>,
            SetManualStreamSelection: SetManualStreamSelection::<Impl, IMPL_OFFSET>,
            GetManualStreamSelection: GetManualStreamSelection::<Impl, IMPL_OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Impl, IMPL_OFFSET>,
            GetStreamSelected: GetStreamSelected::<Impl, IMPL_OFFSET>,
            SetReceiveSelectionCallbacks: SetReceiveSelectionCallbacks::<Impl, IMPL_OFFSET>,
            GetReceiveSelectionCallbacks: GetReceiveSelectionCallbacks::<Impl, IMPL_OFFSET>,
            SetReceiveStreamSamples: SetReceiveStreamSamples::<Impl, IMPL_OFFSET>,
            GetReceiveStreamSamples: GetReceiveStreamSamples::<Impl, IMPL_OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Impl, IMPL_OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Impl, IMPL_OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Impl, IMPL_OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Impl, IMPL_OFFSET>,
            GetStatistics: GetStatistics::<Impl, IMPL_OFFSET>,
            SetClientInfo: SetClientInfo::<Impl, IMPL_OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Impl, IMPL_OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Impl, IMPL_OFFSET>,
            NotifyLateDelivery: NotifyLateDelivery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced2Impl: Sized + IWMReaderAdvancedImpl {
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
    fn OpenStream(&mut self, pstream: ::core::option::Option<super::super::System::Com::IStream>, pcallback: ::core::option::Option<IWMReaderCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced2Vtbl {
        unsafe extern "system" fn SetPlayMode<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn GetPlayMode<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPlayMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferProgress<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferProgress(::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pcnsbuffering)).into()
        }
        unsafe extern "system" fn GetDownloadProgress<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDownloadProgress(::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pqwbytesdownloaded), ::core::mem::transmute_copy(&pcnsdownload)).into()
        }
        unsafe extern "system" fn GetSaveAsProgress<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSaveAsProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwpercent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveFileAs<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveFileAs(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        unsafe extern "system" fn GetProtocolName<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProtocolName(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pcchprotocol)).into()
        }
        unsafe extern "system" fn StartAtMarker<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAtMarker(::core::mem::transmute_copy(&wmarkerindex), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn GetOutputSetting<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetOutputSetting<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn Preroll<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Preroll(::core::mem::transmute_copy(&cnsstart), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate)).into()
        }
        unsafe extern "system" fn SetLogClientID<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogClientID(::core::mem::transmute_copy(&flogclientid)).into()
        }
        unsafe extern "system" fn GetLogClientID<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogClientID() {
                ::core::result::Result::Ok(ok__) => {
                    *pflogclientid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopBuffering<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopBuffering().into()
        }
        unsafe extern "system" fn OpenStream<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenStream(::core::mem::transmute(&pstream), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: IWMReaderAdvancedVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPlayMode: SetPlayMode::<Impl, IMPL_OFFSET>,
            GetPlayMode: GetPlayMode::<Impl, IMPL_OFFSET>,
            GetBufferProgress: GetBufferProgress::<Impl, IMPL_OFFSET>,
            GetDownloadProgress: GetDownloadProgress::<Impl, IMPL_OFFSET>,
            GetSaveAsProgress: GetSaveAsProgress::<Impl, IMPL_OFFSET>,
            SaveFileAs: SaveFileAs::<Impl, IMPL_OFFSET>,
            GetProtocolName: GetProtocolName::<Impl, IMPL_OFFSET>,
            StartAtMarker: StartAtMarker::<Impl, IMPL_OFFSET>,
            GetOutputSetting: GetOutputSetting::<Impl, IMPL_OFFSET>,
            SetOutputSetting: SetOutputSetting::<Impl, IMPL_OFFSET>,
            Preroll: Preroll::<Impl, IMPL_OFFSET>,
            SetLogClientID: SetLogClientID::<Impl, IMPL_OFFSET>,
            GetLogClientID: GetLogClientID::<Impl, IMPL_OFFSET>,
            StopBuffering: StopBuffering::<Impl, IMPL_OFFSET>,
            OpenStream: OpenStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced3Impl: Sized + IWMReaderAdvancedImpl + IWMReaderAdvanced2Impl {
    fn StopNetStreaming(&mut self) -> ::windows::core::Result<()>;
    fn StartAtPosition(&mut self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced3Vtbl {
        unsafe extern "system" fn StopNetStreaming<Impl: IWMReaderAdvanced3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopNetStreaming().into()
        }
        unsafe extern "system" fn StartAtPosition<Impl: IWMReaderAdvanced3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartAtPosition(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pvoffsetstart), ::core::mem::transmute_copy(&pvduration), ::core::mem::transmute_copy(&dwoffsetformat), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: IWMReaderAdvanced2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StopNetStreaming: StopNetStreaming::<Impl, IMPL_OFFSET>,
            StartAtPosition: StartAtPosition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced4Impl: Sized + IWMReaderAdvancedImpl + IWMReaderAdvanced2Impl + IWMReaderAdvanced3Impl {
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
impl IWMReaderAdvanced4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced4Vtbl {
        unsafe extern "system" fn GetLanguageCount<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLanguageCount(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwlanguagecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguage<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLanguage(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&wlanguage), ::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn GetMaxSpeedFactor<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxSpeedFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *pdblfactor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUsingFastCache<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUsingFastCache() {
                ::core::result::Result::Ok(ok__) => {
                    *pfusingfastcache = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLogParam<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLogParam(::core::mem::transmute_copy(&wsznamespace), ::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&wszvalue)).into()
        }
        unsafe extern "system" fn SendLogParams<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendLogParams().into()
        }
        unsafe extern "system" fn CanSaveFileAs<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanSaveFileAs() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcansave = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelSaveFileAs<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelSaveFileAs().into()
        }
        unsafe extern "system" fn GetURL<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetURL(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into()
        }
        Self {
            base: IWMReaderAdvanced3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLanguageCount: GetLanguageCount::<Impl, IMPL_OFFSET>,
            GetLanguage: GetLanguage::<Impl, IMPL_OFFSET>,
            GetMaxSpeedFactor: GetMaxSpeedFactor::<Impl, IMPL_OFFSET>,
            IsUsingFastCache: IsUsingFastCache::<Impl, IMPL_OFFSET>,
            AddLogParam: AddLogParam::<Impl, IMPL_OFFSET>,
            SendLogParams: SendLogParams::<Impl, IMPL_OFFSET>,
            CanSaveFileAs: CanSaveFileAs::<Impl, IMPL_OFFSET>,
            CancelSaveFileAs: CancelSaveFileAs::<Impl, IMPL_OFFSET>,
            GetURL: GetURL::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced5Impl: Sized + IWMReaderAdvancedImpl + IWMReaderAdvanced2Impl + IWMReaderAdvanced3Impl + IWMReaderAdvanced4Impl {
    fn SetPlayerHook(&mut self, dwoutputnum: u32, phook: ::core::option::Option<IWMPlayerHook>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced5Vtbl {
        unsafe extern "system" fn SetPlayerHook<Impl: IWMReaderAdvanced5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlayerHook(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&phook)).into()
        }
        Self { base: IWMReaderAdvanced4Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetPlayerHook: SetPlayerHook::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced6Impl: Sized + IWMReaderAdvancedImpl + IWMReaderAdvanced2Impl + IWMReaderAdvanced3Impl + IWMReaderAdvanced4Impl + IWMReaderAdvanced5Impl {
    fn SetProtectStreamSamples(&mut self, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced6Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Impl: IWMReaderAdvanced6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProtectStreamSamples(::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&dwcertificatetype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbinitializationvector), ::core::mem::transmute_copy(&pcbinitializationvector)).into()
        }
        Self {
            base: IWMReaderAdvanced5Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProtectStreamSamples: SetProtectStreamSamples::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced6 as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderAllocatorExImpl: Sized {
    fn AllocateForStreamEx(&mut self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForOutputEx(&mut self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMReaderAllocatorExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAllocatorExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAllocatorExVtbl {
        unsafe extern "system" fn AllocateForStreamEx<Impl: IWMReaderAllocatorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllocateForStreamEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForOutputEx<Impl: IWMReaderAllocatorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllocateForOutputEx(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AllocateForStreamEx: AllocateForStreamEx::<Impl, IMPL_OFFSET>,
            AllocateForOutputEx: AllocateForOutputEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAllocatorEx as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderCallbackImpl: Sized + IWMStatusCallbackImpl {
    fn OnSample(&mut self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMReaderCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderCallbackVtbl {
        unsafe extern "system" fn OnSample<Impl: IWMReaderCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSample(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base: IWMStatusCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnSample: OnSample::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderCallbackAdvancedImpl: Sized {
    fn OnStreamSample(&mut self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OnTime(&mut self, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OnStreamSelection(&mut self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OnOutputPropsChanged(&mut self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForStream(&mut self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForOutput(&mut self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderCallbackAdvancedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackAdvancedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderCallbackAdvancedVtbl {
        unsafe extern "system" fn OnStreamSample<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStreamSample(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnTime<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTime(::core::mem::transmute_copy(&cnscurrenttime), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnStreamSelection<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStreamSelection(::core::mem::transmute_copy(&wstreamcount), ::core::mem::transmute_copy(&pstreamnumbers), ::core::mem::transmute_copy(&pselections), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnOutputPropsChanged<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnOutputPropsChanged(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pmediatype), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForStream<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllocateForStream(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForOutput<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllocateForOutput(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStreamSample: OnStreamSample::<Impl, IMPL_OFFSET>,
            OnTime: OnTime::<Impl, IMPL_OFFSET>,
            OnStreamSelection: OnStreamSelection::<Impl, IMPL_OFFSET>,
            OnOutputPropsChanged: OnOutputPropsChanged::<Impl, IMPL_OFFSET>,
            AllocateForStream: AllocateForStream::<Impl, IMPL_OFFSET>,
            AllocateForOutput: AllocateForOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderCallbackAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderNetworkConfigImpl: Sized {
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
impl IWMReaderNetworkConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderNetworkConfigVtbl {
        unsafe extern "system" fn GetBufferingTime<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsbufferingtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferingTime<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferingTime(::core::mem::transmute_copy(&cnsbufferingtime)).into()
        }
        unsafe extern "system" fn GetUDPPortRanges<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetUDPPortRanges(::core::mem::transmute_copy(&prangearray), ::core::mem::transmute_copy(&pcranges)).into()
        }
        unsafe extern "system" fn SetUDPPortRanges<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUDPPortRanges(::core::mem::transmute_copy(&prangearray), ::core::mem::transmute_copy(&cranges)).into()
        }
        unsafe extern "system" fn GetProxySettings<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProxySettings(::core::mem::transmute_copy(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproxysetting = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxySettings(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&proxysetting)).into()
        }
        unsafe extern "system" fn GetProxyHostName<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProxyHostName(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pwszhostname), ::core::mem::transmute_copy(&pcchhostname)).into()
        }
        unsafe extern "system" fn SetProxyHostName<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyHostName(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pwszhostname)).into()
        }
        unsafe extern "system" fn GetProxyPort<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProxyPort(::core::mem::transmute_copy(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyPort<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, dwport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyPort(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&dwport)).into()
        }
        unsafe extern "system" fn GetProxyExceptionList<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProxyExceptionList(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pwszexceptionlist), ::core::mem::transmute_copy(&pcchexceptionlist)).into()
        }
        unsafe extern "system" fn SetProxyExceptionList<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyExceptionList(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pwszexceptionlist)).into()
        }
        unsafe extern "system" fn GetProxyBypassForLocal<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProxyBypassForLocal(::core::mem::transmute_copy(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfbypassforlocal = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyBypassForLocal<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxyBypassForLocal(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&fbypassforlocal)).into()
        }
        unsafe extern "system" fn GetForceRerunAutoProxyDetection<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetForceRerunAutoProxyDetection() {
                ::core::result::Result::Ok(ok__) => {
                    *pfforcererundetection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceRerunAutoProxyDetection<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceRerunAutoProxyDetection(::core::mem::transmute_copy(&fforcererundetection)).into()
        }
        unsafe extern "system" fn GetEnableMulticast<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnableMulticast() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablemulticast = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableMulticast<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableMulticast(::core::mem::transmute_copy(&fenablemulticast)).into()
        }
        unsafe extern "system" fn GetEnableHTTP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnableHTTP() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablehttp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableHTTP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableHTTP(::core::mem::transmute_copy(&fenablehttp)).into()
        }
        unsafe extern "system" fn GetEnableUDP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnableUDP() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenableudp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableUDP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableUDP(::core::mem::transmute_copy(&fenableudp)).into()
        }
        unsafe extern "system" fn GetEnableTCP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnableTCP() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabletcp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTCP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableTCP(::core::mem::transmute_copy(&fenabletcp)).into()
        }
        unsafe extern "system" fn ResetProtocolRollover<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetProtocolRollover().into()
        }
        unsafe extern "system" fn GetConnectionBandwidth<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectionBandwidth() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconnectionbandwidth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionBandwidth<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectionBandwidth(::core::mem::transmute_copy(&dwconnectionbandwidth)).into()
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumProtocolsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pcprotocols = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedProtocolName<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSupportedProtocolName(::core::mem::transmute_copy(&dwprotocolnum), ::core::mem::transmute_copy(&pwszprotocolname), ::core::mem::transmute_copy(&pcchprotocolname)).into()
        }
        unsafe extern "system" fn AddLoggingUrl<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddLoggingUrl(::core::mem::transmute_copy(&pwszurl)).into()
        }
        unsafe extern "system" fn GetLoggingUrl<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLoggingUrl(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into()
        }
        unsafe extern "system" fn GetLoggingUrlCount<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLoggingUrlCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwurlcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetLoggingUrlList<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetLoggingUrlList().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBufferingTime: GetBufferingTime::<Impl, IMPL_OFFSET>,
            SetBufferingTime: SetBufferingTime::<Impl, IMPL_OFFSET>,
            GetUDPPortRanges: GetUDPPortRanges::<Impl, IMPL_OFFSET>,
            SetUDPPortRanges: SetUDPPortRanges::<Impl, IMPL_OFFSET>,
            GetProxySettings: GetProxySettings::<Impl, IMPL_OFFSET>,
            SetProxySettings: SetProxySettings::<Impl, IMPL_OFFSET>,
            GetProxyHostName: GetProxyHostName::<Impl, IMPL_OFFSET>,
            SetProxyHostName: SetProxyHostName::<Impl, IMPL_OFFSET>,
            GetProxyPort: GetProxyPort::<Impl, IMPL_OFFSET>,
            SetProxyPort: SetProxyPort::<Impl, IMPL_OFFSET>,
            GetProxyExceptionList: GetProxyExceptionList::<Impl, IMPL_OFFSET>,
            SetProxyExceptionList: SetProxyExceptionList::<Impl, IMPL_OFFSET>,
            GetProxyBypassForLocal: GetProxyBypassForLocal::<Impl, IMPL_OFFSET>,
            SetProxyBypassForLocal: SetProxyBypassForLocal::<Impl, IMPL_OFFSET>,
            GetForceRerunAutoProxyDetection: GetForceRerunAutoProxyDetection::<Impl, IMPL_OFFSET>,
            SetForceRerunAutoProxyDetection: SetForceRerunAutoProxyDetection::<Impl, IMPL_OFFSET>,
            GetEnableMulticast: GetEnableMulticast::<Impl, IMPL_OFFSET>,
            SetEnableMulticast: SetEnableMulticast::<Impl, IMPL_OFFSET>,
            GetEnableHTTP: GetEnableHTTP::<Impl, IMPL_OFFSET>,
            SetEnableHTTP: SetEnableHTTP::<Impl, IMPL_OFFSET>,
            GetEnableUDP: GetEnableUDP::<Impl, IMPL_OFFSET>,
            SetEnableUDP: SetEnableUDP::<Impl, IMPL_OFFSET>,
            GetEnableTCP: GetEnableTCP::<Impl, IMPL_OFFSET>,
            SetEnableTCP: SetEnableTCP::<Impl, IMPL_OFFSET>,
            ResetProtocolRollover: ResetProtocolRollover::<Impl, IMPL_OFFSET>,
            GetConnectionBandwidth: GetConnectionBandwidth::<Impl, IMPL_OFFSET>,
            SetConnectionBandwidth: SetConnectionBandwidth::<Impl, IMPL_OFFSET>,
            GetNumProtocolsSupported: GetNumProtocolsSupported::<Impl, IMPL_OFFSET>,
            GetSupportedProtocolName: GetSupportedProtocolName::<Impl, IMPL_OFFSET>,
            AddLoggingUrl: AddLoggingUrl::<Impl, IMPL_OFFSET>,
            GetLoggingUrl: GetLoggingUrl::<Impl, IMPL_OFFSET>,
            GetLoggingUrlCount: GetLoggingUrlCount::<Impl, IMPL_OFFSET>,
            ResetLoggingUrlList: ResetLoggingUrlList::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderNetworkConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderNetworkConfig2Impl: Sized + IWMReaderNetworkConfigImpl {
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
impl IWMReaderNetworkConfig2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderNetworkConfig2Vtbl {
        unsafe extern "system" fn GetEnableContentCaching<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnableContentCaching() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablecontentcaching = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableContentCaching<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableContentCaching(::core::mem::transmute_copy(&fenablecontentcaching)).into()
        }
        unsafe extern "system" fn GetEnableFastCache<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnableFastCache() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablefastcache = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableFastCache<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableFastCache(::core::mem::transmute_copy(&fenablefastcache)).into()
        }
        unsafe extern "system" fn GetAcceleratedStreamingDuration<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAcceleratedStreamingDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsaccelduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceleratedStreamingDuration<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAcceleratedStreamingDuration(::core::mem::transmute_copy(&cnsaccelduration)).into()
        }
        unsafe extern "system" fn GetAutoReconnectLimit<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoReconnectLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwautoreconnectlimit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnectLimit<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoReconnectLimit(::core::mem::transmute_copy(&dwautoreconnectlimit)).into()
        }
        unsafe extern "system" fn GetEnableResends<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnableResends() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenableresends = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableResends<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableResends(::core::mem::transmute_copy(&fenableresends)).into()
        }
        unsafe extern "system" fn GetEnableThinning<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnableThinning() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenablethinning = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableThinning<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnableThinning(::core::mem::transmute_copy(&fenablethinning)).into()
        }
        unsafe extern "system" fn GetMaxNetPacketSize<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxNetPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxnetpacketsize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMReaderNetworkConfigVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetEnableContentCaching: GetEnableContentCaching::<Impl, IMPL_OFFSET>,
            SetEnableContentCaching: SetEnableContentCaching::<Impl, IMPL_OFFSET>,
            GetEnableFastCache: GetEnableFastCache::<Impl, IMPL_OFFSET>,
            SetEnableFastCache: SetEnableFastCache::<Impl, IMPL_OFFSET>,
            GetAcceleratedStreamingDuration: GetAcceleratedStreamingDuration::<Impl, IMPL_OFFSET>,
            SetAcceleratedStreamingDuration: SetAcceleratedStreamingDuration::<Impl, IMPL_OFFSET>,
            GetAutoReconnectLimit: GetAutoReconnectLimit::<Impl, IMPL_OFFSET>,
            SetAutoReconnectLimit: SetAutoReconnectLimit::<Impl, IMPL_OFFSET>,
            GetEnableResends: GetEnableResends::<Impl, IMPL_OFFSET>,
            SetEnableResends: SetEnableResends::<Impl, IMPL_OFFSET>,
            GetEnableThinning: GetEnableThinning::<Impl, IMPL_OFFSET>,
            SetEnableThinning: SetEnableThinning::<Impl, IMPL_OFFSET>,
            GetMaxNetPacketSize: GetMaxNetPacketSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderNetworkConfig2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderPlaylistBurnImpl: Sized {
    fn InitPlaylistBurn(&mut self, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: ::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetInitResults(&mut self, cfiles: u32) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn EndPlaylistBurn(&mut self, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderPlaylistBurnVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderPlaylistBurnImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderPlaylistBurnVtbl {
        unsafe extern "system" fn InitPlaylistBurn<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitPlaylistBurn(::core::mem::transmute_copy(&cfiles), ::core::mem::transmute_copy(&ppwszfilenames), ::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn GetInitResults<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInitResults(::core::mem::transmute_copy(&cfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *phrstati = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn EndPlaylistBurn<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndPlaylistBurn(::core::mem::transmute_copy(&hrburnresult)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            InitPlaylistBurn: InitPlaylistBurn::<Impl, IMPL_OFFSET>,
            GetInitResults: GetInitResults::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            EndPlaylistBurn: EndPlaylistBurn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderPlaylistBurn as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderStreamClockImpl: Sized {
    fn GetTime(&mut self, pcnsnow: *const u64) -> ::windows::core::Result<()>;
    fn SetTimer(&mut self, cnswhen: u64, pvparam: *const ::core::ffi::c_void) -> ::windows::core::Result<u32>;
    fn KillTimer(&mut self, dwtimerid: u32) -> ::windows::core::Result<()>;
}
impl IWMReaderStreamClockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderStreamClockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderStreamClockVtbl {
        unsafe extern "system" fn GetTime<Impl: IWMReaderStreamClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTime(::core::mem::transmute_copy(&pcnsnow)).into()
        }
        unsafe extern "system" fn SetTimer<Impl: IWMReaderStreamClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTimer(::core::mem::transmute_copy(&cnswhen), ::core::mem::transmute_copy(&pvparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwtimerid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KillTimer<Impl: IWMReaderStreamClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KillTimer(::core::mem::transmute_copy(&dwtimerid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTime: GetTime::<Impl, IMPL_OFFSET>,
            SetTimer: SetTimer::<Impl, IMPL_OFFSET>,
            KillTimer: KillTimer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderStreamClock as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderTimecodeImpl: Sized {
    fn GetTimecodeRangeCount(&mut self, wstreamnum: u16) -> ::windows::core::Result<u16>;
    fn GetTimecodeRangeBounds(&mut self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::Result<()>;
}
impl IWMReaderTimecodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderTimecodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderTimecodeVtbl {
        unsafe extern "system" fn GetTimecodeRangeCount<Impl: IWMReaderTimecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimecodeRangeCount(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwrangecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimecodeRangeBounds<Impl: IWMReaderTimecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTimecodeRangeBounds(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&wrangenum), ::core::mem::transmute_copy(&pstarttimecode), ::core::mem::transmute_copy(&pendtimecode)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTimecodeRangeCount: GetTimecodeRangeCount::<Impl, IMPL_OFFSET>,
            GetTimecodeRangeBounds: GetTimecodeRangeBounds::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderTimecode as ::windows::core::Interface>::IID
    }
}
pub trait IWMReaderTypeNegotiationImpl: Sized {
    fn TryOutputProps(&mut self, dwoutputnum: u32, poutput: ::core::option::Option<IWMOutputMediaProps>) -> ::windows::core::Result<()>;
}
impl IWMReaderTypeNegotiationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderTypeNegotiationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderTypeNegotiationVtbl {
        unsafe extern "system" fn TryOutputProps<Impl: IWMReaderTypeNegotiationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TryOutputProps(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&poutput)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TryOutputProps: TryOutputProps::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderTypeNegotiation as ::windows::core::Interface>::IID
    }
}
pub trait IWMRegisterCallbackImpl: Sized {
    fn Advise(&mut self, pcallback: ::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Unadvise(&mut self, pcallback: ::core::option::Option<IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMRegisterCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisterCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMRegisterCallbackVtbl {
        unsafe extern "system" fn Advise<Impl: IWMRegisterCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Advise(::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Unadvise<Impl: IWMRegisterCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unadvise(::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Advise: Advise::<Impl, IMPL_OFFSET>,
            Unadvise: Unadvise::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMRegisterCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMRegisteredDeviceImpl: Sized {
    fn GetDeviceSerialNumber(&mut self) -> ::windows::core::Result<DRM_VAL16>;
    fn GetDeviceCertificate(&mut self) -> ::windows::core::Result<INSSBuffer>;
    fn GetDeviceType(&mut self) -> ::windows::core::Result<u32>;
    fn GetAttributeCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAttributeByIndex(&mut self, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetAttributeByName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAttributeByName(&mut self, bstrname: super::super::Foundation::BSTR, bstrvalue: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Approve(&mut self, fapprove: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsValid(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsApproved(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsWmdrmCompliant(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOpened(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMRegisteredDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMRegisteredDeviceVtbl {
        unsafe extern "system" fn GetDeviceSerialNumber<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceSerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pserialnumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCertificate<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertificate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcertificate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceType<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeCount<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcattributes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAttributeByIndex(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrvalue)).into()
        }
        unsafe extern "system" fn GetAttributeByName<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAttributeByName(::core::mem::transmute_copy(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeByName<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAttributeByName(::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrvalue)).into()
        }
        unsafe extern "system" fn Approve<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Approve(::core::mem::transmute_copy(&fapprove)).into()
        }
        unsafe extern "system" fn IsValid<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *pfvalid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApproved<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsApproved() {
                ::core::result::Result::Ok(ok__) => {
                    *pfapproved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWmdrmCompliant<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWmdrmCompliant() {
                ::core::result::Result::Ok(ok__) => {
                    *pfcompliant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpened<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOpened() {
                ::core::result::Result::Ok(ok__) => {
                    *pfopened = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn Close<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDeviceSerialNumber: GetDeviceSerialNumber::<Impl, IMPL_OFFSET>,
            GetDeviceCertificate: GetDeviceCertificate::<Impl, IMPL_OFFSET>,
            GetDeviceType: GetDeviceType::<Impl, IMPL_OFFSET>,
            GetAttributeCount: GetAttributeCount::<Impl, IMPL_OFFSET>,
            GetAttributeByIndex: GetAttributeByIndex::<Impl, IMPL_OFFSET>,
            GetAttributeByName: GetAttributeByName::<Impl, IMPL_OFFSET>,
            SetAttributeByName: SetAttributeByName::<Impl, IMPL_OFFSET>,
            Approve: Approve::<Impl, IMPL_OFFSET>,
            IsValid: IsValid::<Impl, IMPL_OFFSET>,
            IsApproved: IsApproved::<Impl, IMPL_OFFSET>,
            IsWmdrmCompliant: IsWmdrmCompliant::<Impl, IMPL_OFFSET>,
            IsOpened: IsOpened::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMRegisteredDevice as ::windows::core::Interface>::IID
    }
}
pub trait IWMSBufferAllocatorImpl: Sized {
    fn AllocateBuffer(&mut self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer>;
    fn AllocatePageSizeBuffer(&mut self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer>;
}
impl IWMSBufferAllocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSBufferAllocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSBufferAllocatorVtbl {
        unsafe extern "system" fn AllocateBuffer<Impl: IWMSBufferAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateBuffer(::core::mem::transmute_copy(&dwmaxbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocatePageSizeBuffer<Impl: IWMSBufferAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocatePageSizeBuffer(::core::mem::transmute_copy(&dwmaxbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AllocateBuffer: AllocateBuffer::<Impl, IMPL_OFFSET>,
            AllocatePageSizeBuffer: AllocatePageSizeBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSBufferAllocator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSourceImpl: Sized {
    fn Initialize(&mut self, psharednamespace: ::core::option::Option<::windows::core::IUnknown>, pnamespacenode: ::core::option::Option<::windows::core::IUnknown>, pnetsourcecreator: ::core::option::Option<INSNetSourceCreator>, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetSourceCreator(&mut self) -> ::windows::core::Result<INSNetSourceCreator>;
    fn SetCredentials(&mut self, bstrrealm: super::super::Foundation::BSTR, bstrname: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCredentials(&mut self, bstrrealm: super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DeleteCredentials(&mut self, bstrrealm: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetCredentialFlags(&mut self) -> ::windows::core::Result<u32>;
    fn SetCredentialFlags(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn FindProxyForURL(&mut self, bstrprotocol: super::super::Foundation::BSTR, bstrhost: super::super::Foundation::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()>;
    fn RegisterProxyFailure(&mut self, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::Result<()>;
    fn ShutdownProxyContext(&mut self, dwproxycontext: u32) -> ::windows::core::Result<()>;
    fn IsUsingIE(&mut self, dwproxycontext: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSInternalAdminNetSourceVtbl {
        unsafe extern "system" fn Initialize<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: ::windows::core::RawPtr, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&psharednamespace), ::core::mem::transmute(&pnamespacenode), ::core::mem::transmute(&pnetsourcecreator), ::core::mem::transmute_copy(&fembeddedinserver)).into()
        }
        unsafe extern "system" fn GetNetSourceCreator<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetSourceCreator() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetsourcecreator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentials(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood)).into()
        }
        unsafe extern "system" fn GetCredentials<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCredentials(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        unsafe extern "system" fn DeleteCredentials<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteCredentials(::core::mem::transmute_copy(&bstrrealm)).into()
        }
        unsafe extern "system" fn GetCredentialFlags<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCredentialFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *lpdwflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialFlags<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentialFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindProxyForURL<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindProxyForURL(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&bstrhost), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pdwproxycontext)).into()
        }
        unsafe extern "system" fn RegisterProxyFailure<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterProxyFailure(::core::mem::transmute_copy(&hrparam), ::core::mem::transmute_copy(&dwproxycontext)).into()
        }
        unsafe extern "system" fn ShutdownProxyContext<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutdownProxyContext(::core::mem::transmute_copy(&dwproxycontext)).into()
        }
        unsafe extern "system" fn IsUsingIE<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUsingIE(::core::mem::transmute_copy(&dwproxycontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfisusingie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetNetSourceCreator: GetNetSourceCreator::<Impl, IMPL_OFFSET>,
            SetCredentials: SetCredentials::<Impl, IMPL_OFFSET>,
            GetCredentials: GetCredentials::<Impl, IMPL_OFFSET>,
            DeleteCredentials: DeleteCredentials::<Impl, IMPL_OFFSET>,
            GetCredentialFlags: GetCredentialFlags::<Impl, IMPL_OFFSET>,
            SetCredentialFlags: SetCredentialFlags::<Impl, IMPL_OFFSET>,
            FindProxyForURL: FindProxyForURL::<Impl, IMPL_OFFSET>,
            RegisterProxyFailure: RegisterProxyFailure::<Impl, IMPL_OFFSET>,
            ShutdownProxyContext: ShutdownProxyContext::<Impl, IMPL_OFFSET>,
            IsUsingIE: IsUsingIE::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource2Impl: Sized {
    fn SetCredentialsEx(&mut self, bstrrealm: super::super::Foundation::BSTR, bstrurl: super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCredentialsEx(&mut self, bstrrealm: super::super::Foundation::BSTR, bstrurl: super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DeleteCredentialsEx(&mut self, bstrrealm: super::super::Foundation::BSTR, bstrurl: super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FindProxyForURLEx(&mut self, bstrprotocol: super::super::Foundation::BSTR, bstrhost: super::super::Foundation::BSTR, bstrurl: super::super::Foundation::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSInternalAdminNetSource2Vtbl {
        unsafe extern "system" fn SetCredentialsEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentialsEx(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood)).into()
        }
        unsafe extern "system" fn GetCredentialsEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCredentialsEx(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&pdwurlpolicy), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        unsafe extern "system" fn DeleteCredentialsEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteCredentialsEx(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy)).into()
        }
        unsafe extern "system" fn FindProxyForURLEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindProxyForURLEx(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&bstrhost), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pdwproxycontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetCredentialsEx: SetCredentialsEx::<Impl, IMPL_OFFSET>,
            GetCredentialsEx: GetCredentialsEx::<Impl, IMPL_OFFSET>,
            DeleteCredentialsEx: DeleteCredentialsEx::<Impl, IMPL_OFFSET>,
            FindProxyForURLEx: FindProxyForURLEx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource3Impl: Sized + IWMSInternalAdminNetSource2Impl {
    fn GetNetSourceCreator2(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn FindProxyForURLEx2(&mut self, bstrprotocol: super::super::Foundation::BSTR, bstrhost: super::super::Foundation::BSTR, bstrurl: super::super::Foundation::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::Result<()>;
    fn RegisterProxyFailure2(&mut self, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::Result<()>;
    fn ShutdownProxyContext2(&mut self, qwproxycontext: u64) -> ::windows::core::Result<()>;
    fn IsUsingIE2(&mut self, qwproxycontext: u64) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetCredentialsEx2(&mut self, bstrrealm: super::super::Foundation::BSTR, bstrurl: super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCredentialsEx2(&mut self, bstrrealm: super::super::Foundation::BSTR, bstrurl: super::super::Foundation::BSTR, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSInternalAdminNetSource3Vtbl {
        unsafe extern "system" fn GetNetSourceCreator2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetSourceCreator2() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnetsourcecreator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindProxyForURLEx2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindProxyForURLEx2(::core::mem::transmute_copy(&bstrprotocol), ::core::mem::transmute_copy(&bstrhost), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pqwproxycontext)).into()
        }
        unsafe extern "system" fn RegisterProxyFailure2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterProxyFailure2(::core::mem::transmute_copy(&hrparam), ::core::mem::transmute_copy(&qwproxycontext)).into()
        }
        unsafe extern "system" fn ShutdownProxyContext2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutdownProxyContext2(::core::mem::transmute_copy(&qwproxycontext)).into()
        }
        unsafe extern "system" fn IsUsingIE2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUsingIE2(::core::mem::transmute_copy(&qwproxycontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfisusingie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialsEx2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCredentialsEx2(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&bstrname), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood), ::core::mem::transmute_copy(&fcleartextauthentication)).into()
        }
        unsafe extern "system" fn GetCredentialsEx2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCredentialsEx2(::core::mem::transmute_copy(&bstrrealm), ::core::mem::transmute_copy(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&fcleartextauthentication), ::core::mem::transmute_copy(&pdwurlpolicy), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        Self {
            base: IWMSInternalAdminNetSource2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetNetSourceCreator2: GetNetSourceCreator2::<Impl, IMPL_OFFSET>,
            FindProxyForURLEx2: FindProxyForURLEx2::<Impl, IMPL_OFFSET>,
            RegisterProxyFailure2: RegisterProxyFailure2::<Impl, IMPL_OFFSET>,
            ShutdownProxyContext2: ShutdownProxyContext2::<Impl, IMPL_OFFSET>,
            IsUsingIE2: IsUsingIE2::<Impl, IMPL_OFFSET>,
            SetCredentialsEx2: SetCredentialsEx2::<Impl, IMPL_OFFSET>,
            GetCredentialsEx2: GetCredentialsEx2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSecureChannelImpl: Sized + IWMAuthorizerImpl {
    fn WMSC_AddCertificate(&mut self, pcert: ::core::option::Option<IWMAuthorizer>) -> ::windows::core::Result<()>;
    fn WMSC_AddSignature(&mut self, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::Result<()>;
    fn WMSC_Connect(&mut self, potherside: ::core::option::Option<IWMSecureChannel>) -> ::windows::core::Result<()>;
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
impl IWMSecureChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSecureChannelVtbl {
        unsafe extern "system" fn WMSC_AddCertificate<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcert: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_AddCertificate(::core::mem::transmute(&pcert)).into()
        }
        unsafe extern "system" fn WMSC_AddSignature<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_AddSignature(::core::mem::transmute_copy(&pbcertsig), ::core::mem::transmute_copy(&cbcertsig)).into()
        }
        unsafe extern "system" fn WMSC_Connect<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, potherside: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_Connect(::core::mem::transmute(&potherside)).into()
        }
        unsafe extern "system" fn WMSC_IsConnected<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WMSC_IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Disconnect<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_Disconnect().into()
        }
        unsafe extern "system" fn WMSC_GetValidCertificate<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_GetValidCertificate(::core::mem::transmute_copy(&ppbcertificate), ::core::mem::transmute_copy(&pdwsignature)).into()
        }
        unsafe extern "system" fn WMSC_Encrypt<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_Encrypt(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn WMSC_Decrypt<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_Decrypt(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn WMSC_Lock<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_Lock().into()
        }
        unsafe extern "system" fn WMSC_Unlock<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_Unlock().into()
        }
        unsafe extern "system" fn WMSC_SetSharedData<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WMSC_SetSharedData(::core::mem::transmute_copy(&dwcertindex), ::core::mem::transmute_copy(&pbshareddata)).into()
        }
        Self {
            base: IWMAuthorizerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WMSC_AddCertificate: WMSC_AddCertificate::<Impl, IMPL_OFFSET>,
            WMSC_AddSignature: WMSC_AddSignature::<Impl, IMPL_OFFSET>,
            WMSC_Connect: WMSC_Connect::<Impl, IMPL_OFFSET>,
            WMSC_IsConnected: WMSC_IsConnected::<Impl, IMPL_OFFSET>,
            WMSC_Disconnect: WMSC_Disconnect::<Impl, IMPL_OFFSET>,
            WMSC_GetValidCertificate: WMSC_GetValidCertificate::<Impl, IMPL_OFFSET>,
            WMSC_Encrypt: WMSC_Encrypt::<Impl, IMPL_OFFSET>,
            WMSC_Decrypt: WMSC_Decrypt::<Impl, IMPL_OFFSET>,
            WMSC_Lock: WMSC_Lock::<Impl, IMPL_OFFSET>,
            WMSC_Unlock: WMSC_Unlock::<Impl, IMPL_OFFSET>,
            WMSC_SetSharedData: WMSC_SetSharedData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSecureChannel as ::windows::core::Interface>::IID
    }
}
pub trait IWMStatusCallbackImpl: Sized {
    fn OnStatus(&mut self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMStatusCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStatusCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStatusCallbackVtbl {
        unsafe extern "system" fn OnStatus<Impl: IWMStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStatus(::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnStatus: OnStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStatusCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamConfigImpl: Sized {
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
impl IWMStreamConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamConfigVtbl {
        unsafe extern "system" fn GetStreamType<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamType() {
                ::core::result::Result::Ok(ok__) => {
                    *pguidstreamtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumber<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *pwstreamnum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamNumber<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamNumber(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn GetStreamName<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreamName(::core::mem::transmute_copy(&pwszstreamname), ::core::mem::transmute_copy(&pcchstreamname)).into()
        }
        unsafe extern "system" fn SetStreamName<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamName(::core::mem::transmute_copy(&pwszstreamname)).into()
        }
        unsafe extern "system" fn GetConnectionName<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConnectionName(::core::mem::transmute_copy(&pwszinputname), ::core::mem::transmute_copy(&pcchinputname)).into()
        }
        unsafe extern "system" fn SetConnectionName<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConnectionName(::core::mem::transmute_copy(&pwszinputname)).into()
        }
        unsafe extern "system" fn GetBitrate<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwbitrate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitrate<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBitrate(::core::mem::transmute_copy(&pdwbitrate)).into()
        }
        unsafe extern "system" fn GetBufferWindow<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferWindow() {
                ::core::result::Result::Ok(ok__) => {
                    *pmsbufferwindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferWindow<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferWindow(::core::mem::transmute_copy(&msbufferwindow)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreamType: GetStreamType::<Impl, IMPL_OFFSET>,
            GetStreamNumber: GetStreamNumber::<Impl, IMPL_OFFSET>,
            SetStreamNumber: SetStreamNumber::<Impl, IMPL_OFFSET>,
            GetStreamName: GetStreamName::<Impl, IMPL_OFFSET>,
            SetStreamName: SetStreamName::<Impl, IMPL_OFFSET>,
            GetConnectionName: GetConnectionName::<Impl, IMPL_OFFSET>,
            SetConnectionName: SetConnectionName::<Impl, IMPL_OFFSET>,
            GetBitrate: GetBitrate::<Impl, IMPL_OFFSET>,
            SetBitrate: SetBitrate::<Impl, IMPL_OFFSET>,
            GetBufferWindow: GetBufferWindow::<Impl, IMPL_OFFSET>,
            SetBufferWindow: SetBufferWindow::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamConfig2Impl: Sized + IWMStreamConfigImpl {
    fn GetTransportType(&mut self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE>;
    fn SetTransportType(&mut self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()>;
    fn AddDataUnitExtension(&mut self, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::Result<()>;
    fn GetDataUnitExtensionCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetDataUnitExtension(&mut self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()>;
    fn RemoveAllDataUnitExtensions(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamConfig2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamConfig2Vtbl {
        unsafe extern "system" fn GetTransportType<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransportType() {
                ::core::result::Result::Ok(ok__) => {
                    *pntransporttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportType<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransportType(::core::mem::transmute_copy(&ntransporttype)).into()
        }
        unsafe extern "system" fn AddDataUnitExtension<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDataUnitExtension(::core::mem::transmute_copy(&guidextensionsystemid), ::core::mem::transmute_copy(&cbextensiondatasize), ::core::mem::transmute_copy(&pbextensionsysteminfo), ::core::mem::transmute_copy(&cbextensionsysteminfo)).into()
        }
        unsafe extern "system" fn GetDataUnitExtensionCount<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataUnitExtensionCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcdataunitextensions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataUnitExtension<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDataUnitExtension(::core::mem::transmute_copy(&wdataunitextensionnumber), ::core::mem::transmute_copy(&pguidextensionsystemid), ::core::mem::transmute_copy(&pcbextensiondatasize), ::core::mem::transmute_copy(&pbextensionsysteminfo), ::core::mem::transmute_copy(&pcbextensionsysteminfo)).into()
        }
        unsafe extern "system" fn RemoveAllDataUnitExtensions<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAllDataUnitExtensions().into()
        }
        Self {
            base: IWMStreamConfigVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetTransportType: GetTransportType::<Impl, IMPL_OFFSET>,
            SetTransportType: SetTransportType::<Impl, IMPL_OFFSET>,
            AddDataUnitExtension: AddDataUnitExtension::<Impl, IMPL_OFFSET>,
            GetDataUnitExtensionCount: GetDataUnitExtensionCount::<Impl, IMPL_OFFSET>,
            GetDataUnitExtension: GetDataUnitExtension::<Impl, IMPL_OFFSET>,
            RemoveAllDataUnitExtensions: RemoveAllDataUnitExtensions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamConfig2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamConfig3Impl: Sized + IWMStreamConfigImpl + IWMStreamConfig2Impl {
    fn GetLanguage(&mut self, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()>;
    fn SetLanguage(&mut self, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamConfig3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamConfig3Vtbl {
        unsafe extern "system" fn GetLanguage<Impl: IWMStreamConfig3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLanguage(::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn SetLanguage<Impl: IWMStreamConfig3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLanguage(::core::mem::transmute_copy(&pwszlanguagestring)).into()
        }
        Self {
            base: IWMStreamConfig2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetLanguage: GetLanguage::<Impl, IMPL_OFFSET>,
            SetLanguage: SetLanguage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamConfig3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMStreamListImpl: Sized {
    fn GetStreams(&mut self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()>;
    fn AddStream(&mut self, wstreamnum: u16) -> ::windows::core::Result<()>;
    fn RemoveStream(&mut self, wstreamnum: u16) -> ::windows::core::Result<()>;
}
impl IWMStreamListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamListVtbl {
        unsafe extern "system" fn GetStreams<Impl: IWMStreamListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStreams(::core::mem::transmute_copy(&pwstreamnumarray), ::core::mem::transmute_copy(&pcstreams)).into()
        }
        unsafe extern "system" fn AddStream<Impl: IWMStreamListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStream(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn RemoveStream<Impl: IWMStreamListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStreams: GetStreams::<Impl, IMPL_OFFSET>,
            AddStream: AddStream::<Impl, IMPL_OFFSET>,
            RemoveStream: RemoveStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamList as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamPrioritizationImpl: Sized {
    fn GetPriorityRecords(&mut self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::Result<()>;
    fn SetPriorityRecords(&mut self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamPrioritizationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamPrioritizationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamPrioritizationVtbl {
        unsafe extern "system" fn GetPriorityRecords<Impl: IWMStreamPrioritizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPriorityRecords(::core::mem::transmute_copy(&precordarray), ::core::mem::transmute_copy(&pcrecords)).into()
        }
        unsafe extern "system" fn SetPriorityRecords<Impl: IWMStreamPrioritizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPriorityRecords(::core::mem::transmute_copy(&precordarray), ::core::mem::transmute_copy(&crecords)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPriorityRecords: GetPriorityRecords::<Impl, IMPL_OFFSET>,
            SetPriorityRecords: SetPriorityRecords::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamPrioritization as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMSyncReaderImpl: Sized {
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
    fn SetOutputProps(&mut self, dwoutputnum: u32, poutput: ::core::option::Option<IWMOutputMediaProps>) -> ::windows::core::Result<()>;
    fn GetOutputFormatCount(&mut self, dwoutputnum: u32) -> ::windows::core::Result<u32>;
    fn GetOutputFormat(&mut self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn GetOutputNumberForStream(&mut self, wstreamnum: u16) -> ::windows::core::Result<u32>;
    fn GetStreamNumberForOutput(&mut self, dwoutputnum: u32) -> ::windows::core::Result<u16>;
    fn GetMaxOutputSampleSize(&mut self, dwoutput: u32) -> ::windows::core::Result<u32>;
    fn GetMaxStreamSampleSize(&mut self, wstream: u16) -> ::windows::core::Result<u32>;
    fn OpenStream(&mut self, pstream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMSyncReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSyncReaderVtbl {
        unsafe extern "system" fn Open<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        unsafe extern "system" fn Close<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn SetRange<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRange(::core::mem::transmute_copy(&cnsstarttime), ::core::mem::transmute_copy(&cnsduration)).into()
        }
        unsafe extern "system" fn SetRangeByFrame<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRangeByFrame(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&qwframenumber), ::core::mem::transmute_copy(&cframestoread)).into()
        }
        unsafe extern "system" fn GetNextSample<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut ::windows::core::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextSample(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&ppsample), ::core::mem::transmute_copy(&pcnssampletime), ::core::mem::transmute_copy(&pcnsduration), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwoutputnum), ::core::mem::transmute_copy(&pwstreamnum)).into()
        }
        unsafe extern "system" fn SetStreamsSelected<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStreamsSelected(::core::mem::transmute_copy(&cstreamcount), ::core::mem::transmute_copy(&pwstreamnumbers), ::core::mem::transmute_copy(&pselections)).into()
        }
        unsafe extern "system" fn GetStreamSelected<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamSelected(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pselection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadStreamSamples<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReadStreamSamples(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&fcompressed)).into()
        }
        unsafe extern "system" fn GetReadStreamSamples<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReadStreamSamples(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfcompressed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSetting<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetOutputSetting<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetOutputCount<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcoutputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputProps(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputProps(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetOutputFormatCount<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputFormatCount(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputFormat(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&dwformatnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputNumberForStream<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutputNumberForStream(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwoutputnum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumberForOutput<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamNumberForOutput(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pwstreamnum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxOutputSampleSize(::core::mem::transmute_copy(&dwoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxStreamSampleSize(::core::mem::transmute_copy(&wstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcbmax = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenStream(::core::mem::transmute(&pstream)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            SetRange: SetRange::<Impl, IMPL_OFFSET>,
            SetRangeByFrame: SetRangeByFrame::<Impl, IMPL_OFFSET>,
            GetNextSample: GetNextSample::<Impl, IMPL_OFFSET>,
            SetStreamsSelected: SetStreamsSelected::<Impl, IMPL_OFFSET>,
            GetStreamSelected: GetStreamSelected::<Impl, IMPL_OFFSET>,
            SetReadStreamSamples: SetReadStreamSamples::<Impl, IMPL_OFFSET>,
            GetReadStreamSamples: GetReadStreamSamples::<Impl, IMPL_OFFSET>,
            GetOutputSetting: GetOutputSetting::<Impl, IMPL_OFFSET>,
            SetOutputSetting: SetOutputSetting::<Impl, IMPL_OFFSET>,
            GetOutputCount: GetOutputCount::<Impl, IMPL_OFFSET>,
            GetOutputProps: GetOutputProps::<Impl, IMPL_OFFSET>,
            SetOutputProps: SetOutputProps::<Impl, IMPL_OFFSET>,
            GetOutputFormatCount: GetOutputFormatCount::<Impl, IMPL_OFFSET>,
            GetOutputFormat: GetOutputFormat::<Impl, IMPL_OFFSET>,
            GetOutputNumberForStream: GetOutputNumberForStream::<Impl, IMPL_OFFSET>,
            GetStreamNumberForOutput: GetStreamNumberForOutput::<Impl, IMPL_OFFSET>,
            GetMaxOutputSampleSize: GetMaxOutputSampleSize::<Impl, IMPL_OFFSET>,
            GetMaxStreamSampleSize: GetMaxStreamSampleSize::<Impl, IMPL_OFFSET>,
            OpenStream: OpenStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSyncReader as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMSyncReader2Impl: Sized + IWMSyncReaderImpl {
    fn SetRangeByTimecode(&mut self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::Result<()>;
    fn SetRangeByFrameEx(&mut self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<u64>;
    fn SetAllocateForOutput(&mut self, dwoutputnum: u32, pallocator: ::core::option::Option<IWMReaderAllocatorEx>) -> ::windows::core::Result<()>;
    fn GetAllocateForOutput(&mut self, dwoutputnum: u32) -> ::windows::core::Result<IWMReaderAllocatorEx>;
    fn SetAllocateForStream(&mut self, wstreamnum: u16, pallocator: ::core::option::Option<IWMReaderAllocatorEx>) -> ::windows::core::Result<()>;
    fn GetAllocateForStream(&mut self, dwsreamnum: u16) -> ::windows::core::Result<IWMReaderAllocatorEx>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMSyncReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSyncReader2Vtbl {
        unsafe extern "system" fn SetRangeByTimecode<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRangeByTimecode(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pstart), ::core::mem::transmute_copy(&pend)).into()
        }
        unsafe extern "system" fn SetRangeByFrameEx<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRangeByFrameEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&qwframenumber), ::core::mem::transmute_copy(&cframestoread)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsstarttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pallocator)).into()
        }
        unsafe extern "system" fn GetAllocateForOutput<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppallocator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllocateForStream(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pallocator)).into()
        }
        unsafe extern "system" fn GetAllocateForStream<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllocateForStream(::core::mem::transmute_copy(&dwsreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppallocator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMSyncReaderVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRangeByTimecode: SetRangeByTimecode::<Impl, IMPL_OFFSET>,
            SetRangeByFrameEx: SetRangeByFrameEx::<Impl, IMPL_OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Impl, IMPL_OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Impl, IMPL_OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Impl, IMPL_OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSyncReader2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMVideoMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetMaxKeyFrameSpacing(&mut self) -> ::windows::core::Result<i64>;
    fn SetMaxKeyFrameSpacing(&mut self, lltime: i64) -> ::windows::core::Result<()>;
    fn GetQuality(&mut self) -> ::windows::core::Result<u32>;
    fn SetQuality(&mut self, dwquality: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMVideoMediaPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoMediaPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMVideoMediaPropsVtbl {
        unsafe extern "system" fn GetMaxKeyFrameSpacing<Impl: IWMVideoMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxKeyFrameSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    *plltime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxKeyFrameSpacing<Impl: IWMVideoMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxKeyFrameSpacing(::core::mem::transmute_copy(&lltime)).into()
        }
        unsafe extern "system" fn GetQuality<Impl: IWMVideoMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwquality = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Impl: IWMVideoMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuality(::core::mem::transmute_copy(&dwquality)).into()
        }
        Self {
            base: IWMMediaPropsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMaxKeyFrameSpacing: GetMaxKeyFrameSpacing::<Impl, IMPL_OFFSET>,
            SetMaxKeyFrameSpacing: SetMaxKeyFrameSpacing::<Impl, IMPL_OFFSET>,
            GetQuality: GetQuality::<Impl, IMPL_OFFSET>,
            SetQuality: SetQuality::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMVideoMediaProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWatermarkInfoImpl: Sized {
    fn GetWatermarkEntryCount(&mut self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> ::windows::core::Result<u32>;
    fn GetWatermarkEntry(&mut self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32) -> ::windows::core::Result<WMT_WATERMARK_ENTRY>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWatermarkInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWatermarkInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWatermarkInfoVtbl {
        unsafe extern "system" fn GetWatermarkEntryCount<Impl: IWMWatermarkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatermarkEntryCount(::core::mem::transmute_copy(&wmettype)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatermarkEntry<Impl: IWMWatermarkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWatermarkEntry(::core::mem::transmute_copy(&wmettype), ::core::mem::transmute_copy(&dwentrynum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pentry = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetWatermarkEntryCount: GetWatermarkEntryCount::<Impl, IMPL_OFFSET>,
            GetWatermarkEntry: GetWatermarkEntry::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWatermarkInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterImpl: Sized {
    fn SetProfileByID(&mut self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetProfile(&mut self, pprofile: ::core::option::Option<IWMProfile>) -> ::windows::core::Result<()>;
    fn SetOutputFilename(&mut self, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetInputCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetInputProps(&mut self, dwinputnum: u32) -> ::windows::core::Result<IWMInputMediaProps>;
    fn SetInputProps(&mut self, dwinputnum: u32, pinput: ::core::option::Option<IWMInputMediaProps>) -> ::windows::core::Result<()>;
    fn GetInputFormatCount(&mut self, dwinputnumber: u32) -> ::windows::core::Result<u32>;
    fn GetInputFormat(&mut self, dwinputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMInputMediaProps>;
    fn BeginWriting(&mut self) -> ::windows::core::Result<()>;
    fn EndWriting(&mut self) -> ::windows::core::Result<()>;
    fn AllocateSample(&mut self, dwsamplesize: u32) -> ::windows::core::Result<INSSBuffer>;
    fn WriteSample(&mut self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn Flush(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterVtbl {
        unsafe extern "system" fn SetProfileByID<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProfileByID(::core::mem::transmute_copy(&guidprofile)).into()
        }
        unsafe extern "system" fn SetProfile<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProfile(::core::mem::transmute(&pprofile)).into()
        }
        unsafe extern "system" fn SetOutputFilename<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutputFilename(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        unsafe extern "system" fn GetInputCount<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcinputs = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputProps<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputProps(::core::mem::transmute_copy(&dwinputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputProps<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputProps(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute(&pinput)).into()
        }
        unsafe extern "system" fn GetInputFormatCount<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputFormatCount(::core::mem::transmute_copy(&dwinputnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputFormat<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInputFormat(::core::mem::transmute_copy(&dwinputnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginWriting<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginWriting().into()
        }
        unsafe extern "system" fn EndWriting<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndWriting().into()
        }
        unsafe extern "system" fn AllocateSample<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateSample(::core::mem::transmute_copy(&dwsamplesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsample = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSample<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteSample(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample)).into()
        }
        unsafe extern "system" fn Flush<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Flush().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetProfileByID: SetProfileByID::<Impl, IMPL_OFFSET>,
            SetProfile: SetProfile::<Impl, IMPL_OFFSET>,
            SetOutputFilename: SetOutputFilename::<Impl, IMPL_OFFSET>,
            GetInputCount: GetInputCount::<Impl, IMPL_OFFSET>,
            GetInputProps: GetInputProps::<Impl, IMPL_OFFSET>,
            SetInputProps: SetInputProps::<Impl, IMPL_OFFSET>,
            GetInputFormatCount: GetInputFormatCount::<Impl, IMPL_OFFSET>,
            GetInputFormat: GetInputFormat::<Impl, IMPL_OFFSET>,
            BeginWriting: BeginWriting::<Impl, IMPL_OFFSET>,
            EndWriting: EndWriting::<Impl, IMPL_OFFSET>,
            AllocateSample: AllocateSample::<Impl, IMPL_OFFSET>,
            WriteSample: WriteSample::<Impl, IMPL_OFFSET>,
            Flush: Flush::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvancedImpl: Sized {
    fn GetSinkCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetSink(&mut self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink>;
    fn AddSink(&mut self, psink: ::core::option::Option<IWMWriterSink>) -> ::windows::core::Result<()>;
    fn RemoveSink(&mut self, psink: ::core::option::Option<IWMWriterSink>) -> ::windows::core::Result<()>;
    fn WriteStreamSample(&mut self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn SetLiveSource(&mut self, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsRealTime(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetWriterTime(&mut self) -> ::windows::core::Result<u64>;
    fn GetStatistics(&mut self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS>;
    fn SetSyncTolerance(&mut self, mswindow: u32) -> ::windows::core::Result<()>;
    fn GetSyncTolerance(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvancedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvancedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterAdvancedVtbl {
        unsafe extern "system" fn GetSinkCount<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSinkCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcsinks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSink<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSink(::core::mem::transmute_copy(&dwsinknum)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsink = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSink<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSink(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn RemoveSink<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSink(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn WriteStreamSample<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteStreamSample(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&mssamplesendtime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample)).into()
        }
        unsafe extern "system" fn SetLiveSource<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLiveSource(::core::mem::transmute_copy(&fislivesource)).into()
        }
        unsafe extern "system" fn IsRealTime<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrealtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriterTime<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWriterTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnscurrenttime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatistics(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSyncTolerance<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSyncTolerance(::core::mem::transmute_copy(&mswindow)).into()
        }
        unsafe extern "system" fn GetSyncTolerance<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSyncTolerance() {
                ::core::result::Result::Ok(ok__) => {
                    *pmswindow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSinkCount: GetSinkCount::<Impl, IMPL_OFFSET>,
            GetSink: GetSink::<Impl, IMPL_OFFSET>,
            AddSink: AddSink::<Impl, IMPL_OFFSET>,
            RemoveSink: RemoveSink::<Impl, IMPL_OFFSET>,
            WriteStreamSample: WriteStreamSample::<Impl, IMPL_OFFSET>,
            SetLiveSource: SetLiveSource::<Impl, IMPL_OFFSET>,
            IsRealTime: IsRealTime::<Impl, IMPL_OFFSET>,
            GetWriterTime: GetWriterTime::<Impl, IMPL_OFFSET>,
            GetStatistics: GetStatistics::<Impl, IMPL_OFFSET>,
            SetSyncTolerance: SetSyncTolerance::<Impl, IMPL_OFFSET>,
            GetSyncTolerance: GetSyncTolerance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterAdvanced as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced2Impl: Sized + IWMWriterAdvancedImpl {
    fn GetInputSetting(&mut self, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetInputSetting(&mut self, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterAdvanced2Vtbl {
        unsafe extern "system" fn GetInputSetting<Impl: IWMWriterAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInputSetting(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetInputSetting<Impl: IWMWriterAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInputSetting(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        Self {
            base: IWMWriterAdvancedVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetInputSetting: GetInputSetting::<Impl, IMPL_OFFSET>,
            SetInputSetting: SetInputSetting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterAdvanced2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced3Impl: Sized + IWMWriterAdvancedImpl + IWMWriterAdvanced2Impl {
    fn GetStatisticsEx(&mut self, wstreamnum: u16) -> ::windows::core::Result<WM_WRITER_STATISTICS_EX>;
    fn SetNonBlocking(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterAdvanced3Vtbl {
        unsafe extern "system" fn GetStatisticsEx<Impl: IWMWriterAdvanced3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatisticsEx(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNonBlocking<Impl: IWMWriterAdvanced3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNonBlocking().into()
        }
        Self {
            base: IWMWriterAdvanced2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetStatisticsEx: GetStatisticsEx::<Impl, IMPL_OFFSET>,
            SetNonBlocking: SetNonBlocking::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterAdvanced3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSinkImpl: Sized + IWMWriterSinkImpl {
    fn Open(&mut self, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterFileSinkVtbl {
        unsafe extern "system" fn Open<Impl: IWMWriterFileSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pwszfilename)).into()
        }
        Self { base: IWMWriterSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Open: Open::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterFileSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink2Impl: Sized + IWMWriterSinkImpl + IWMWriterFileSinkImpl {
    fn Start(&mut self, cnsstarttime: u64) -> ::windows::core::Result<()>;
    fn Stop(&mut self, cnsstoptime: u64) -> ::windows::core::Result<()>;
    fn IsStopped(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetFileDuration(&mut self) -> ::windows::core::Result<u64>;
    fn GetFileSize(&mut self) -> ::windows::core::Result<u64>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn IsClosed(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterFileSink2Vtbl {
        unsafe extern "system" fn Start<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&cnsstarttime)).into()
        }
        unsafe extern "system" fn Stop<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop(::core::mem::transmute_copy(&cnsstoptime)).into()
        }
        unsafe extern "system" fn IsStopped<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStopped() {
                ::core::result::Result::Ok(ok__) => {
                    *pfstopped = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileDuration<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *pcnsduration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pcbfile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn IsClosed<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *pfclosed = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWMWriterFileSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            IsStopped: IsStopped::<Impl, IMPL_OFFSET>,
            GetFileDuration: GetFileDuration::<Impl, IMPL_OFFSET>,
            GetFileSize: GetFileSize::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            IsClosed: IsClosed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterFileSink2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink3Impl: Sized + IWMWriterSinkImpl + IWMWriterFileSinkImpl + IWMWriterFileSink2Impl {
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
impl IWMWriterFileSink3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterFileSink3Vtbl {
        unsafe extern "system" fn SetAutoIndexing<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAutoIndexing(::core::mem::transmute_copy(&fdoautoindexing)).into()
        }
        unsafe extern "system" fn GetAutoIndexing<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAutoIndexing() {
                ::core::result::Result::Ok(ok__) => {
                    *pfautoindexing = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlStream<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetControlStream(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&fshouldcontrolstartandstop)).into()
        }
        unsafe extern "system" fn GetMode<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwfilesinkmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnitEx<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataUnitEx(::core::mem::transmute_copy(&pfilesinkdataunit)).into()
        }
        unsafe extern "system" fn SetUnbufferedIO<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUnbufferedIO(::core::mem::transmute_copy(&funbufferedio), ::core::mem::transmute_copy(&frestrictmemusage)).into()
        }
        unsafe extern "system" fn GetUnbufferedIO<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUnbufferedIO() {
                ::core::result::Result::Ok(ok__) => {
                    *pfunbufferedio = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteOperations<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CompleteOperations().into()
        }
        Self {
            base: IWMWriterFileSink2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetAutoIndexing: SetAutoIndexing::<Impl, IMPL_OFFSET>,
            GetAutoIndexing: GetAutoIndexing::<Impl, IMPL_OFFSET>,
            SetControlStream: SetControlStream::<Impl, IMPL_OFFSET>,
            GetMode: GetMode::<Impl, IMPL_OFFSET>,
            OnDataUnitEx: OnDataUnitEx::<Impl, IMPL_OFFSET>,
            SetUnbufferedIO: SetUnbufferedIO::<Impl, IMPL_OFFSET>,
            GetUnbufferedIO: GetUnbufferedIO::<Impl, IMPL_OFFSET>,
            CompleteOperations: CompleteOperations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterFileSink3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterNetworkSinkImpl: Sized + IWMWriterSinkImpl {
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
impl IWMWriterNetworkSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterNetworkSinkVtbl {
        unsafe extern "system" fn SetMaximumClients<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumClients(::core::mem::transmute_copy(&dwmaxclients)).into()
        }
        unsafe extern "system" fn GetMaximumClients<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumClients() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxclients = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProtocol<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNetworkProtocol(::core::mem::transmute_copy(&protocol)).into()
        }
        unsafe extern "system" fn GetNetworkProtocol<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNetworkProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    *pprotocol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostURL<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetHostURL(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into()
        }
        unsafe extern "system" fn Open<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute_copy(&pdwportnum)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn Close<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: IWMWriterSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMaximumClients: SetMaximumClients::<Impl, IMPL_OFFSET>,
            GetMaximumClients: GetMaximumClients::<Impl, IMPL_OFFSET>,
            SetNetworkProtocol: SetNetworkProtocol::<Impl, IMPL_OFFSET>,
            GetNetworkProtocol: GetNetworkProtocol::<Impl, IMPL_OFFSET>,
            GetHostURL: GetHostURL::<Impl, IMPL_OFFSET>,
            Open: Open::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterNetworkSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterPostViewImpl: Sized {
    fn SetPostViewCallback(&mut self, pcallback: ::core::option::Option<IWMWriterPostViewCallback>, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetReceivePostViewSamples(&mut self, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReceivePostViewSamples(&mut self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetPostViewProps(&mut self, wstreamnumber: u16) -> ::windows::core::Result<IWMMediaProps>;
    fn SetPostViewProps(&mut self, wstreamnumber: u16, poutput: ::core::option::Option<IWMMediaProps>) -> ::windows::core::Result<()>;
    fn GetPostViewFormatCount(&mut self, wstreamnumber: u16) -> ::windows::core::Result<u32>;
    fn GetPostViewFormat(&mut self, wstreamnumber: u16, dwformatnumber: u32) -> ::windows::core::Result<IWMMediaProps>;
    fn SetAllocateForPostView(&mut self, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateForPostView(&mut self, wstreamnumber: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterPostViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterPostViewVtbl {
        unsafe extern "system" fn SetPostViewCallback<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostViewCallback(::core::mem::transmute(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn SetReceivePostViewSamples<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetReceivePostViewSamples(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&freceivepostviewsamples)).into()
        }
        unsafe extern "system" fn GetReceivePostViewSamples<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetReceivePostViewSamples(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfreceivepostviewsamples = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewProps<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPostViewProps(::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostViewProps<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostViewProps(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute(&poutput)).into()
        }
        unsafe extern "system" fn GetPostViewFormatCount<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPostViewFormatCount(::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcformats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewFormat<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPostViewFormat(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppprops = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForPostView<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllocateForPostView(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForPostView<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllocateForPostView(::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfallocate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPostViewCallback: SetPostViewCallback::<Impl, IMPL_OFFSET>,
            SetReceivePostViewSamples: SetReceivePostViewSamples::<Impl, IMPL_OFFSET>,
            GetReceivePostViewSamples: GetReceivePostViewSamples::<Impl, IMPL_OFFSET>,
            GetPostViewProps: GetPostViewProps::<Impl, IMPL_OFFSET>,
            SetPostViewProps: SetPostViewProps::<Impl, IMPL_OFFSET>,
            GetPostViewFormatCount: GetPostViewFormatCount::<Impl, IMPL_OFFSET>,
            GetPostViewFormat: GetPostViewFormat::<Impl, IMPL_OFFSET>,
            SetAllocateForPostView: SetAllocateForPostView::<Impl, IMPL_OFFSET>,
            GetAllocateForPostView: GetAllocateForPostView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPostView as ::windows::core::Interface>::IID
    }
}
pub trait IWMWriterPostViewCallbackImpl: Sized + IWMStatusCallbackImpl {
    fn OnPostViewSample(&mut self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForPostView(&mut self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IWMWriterPostViewCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostViewCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterPostViewCallbackVtbl {
        unsafe extern "system" fn OnPostViewSample<Impl: IWMWriterPostViewCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPostViewSample(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForPostView<Impl: IWMWriterPostViewCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllocateForPostView(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base: IWMStatusCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OnPostViewSample: OnPostViewSample::<Impl, IMPL_OFFSET>,
            AllocateForPostView: AllocateForPostView::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPostViewCallback as ::windows::core::Interface>::IID
    }
}
pub trait IWMWriterPreprocessImpl: Sized {
    fn GetMaxPreprocessingPasses(&mut self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<u32>;
    fn SetNumPreprocessingPasses(&mut self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::Result<()>;
    fn BeginPreprocessingPass(&mut self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn PreprocessSample(&mut self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn EndPreprocessingPass(&mut self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()>;
}
impl IWMWriterPreprocessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPreprocessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterPreprocessVtbl {
        unsafe extern "system" fn GetMaxPreprocessingPasses<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxPreprocessingPasses(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxnumpasses = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumPreprocessingPasses<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNumPreprocessingPasses(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwnumpasses)).into()
        }
        unsafe extern "system" fn BeginPreprocessingPass<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginPreprocessingPass(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn PreprocessSample<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PreprocessSample(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&psample)).into()
        }
        unsafe extern "system" fn EndPreprocessingPass<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndPreprocessingPass(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMaxPreprocessingPasses: GetMaxPreprocessingPasses::<Impl, IMPL_OFFSET>,
            SetNumPreprocessingPasses: SetNumPreprocessingPasses::<Impl, IMPL_OFFSET>,
            BeginPreprocessingPass: BeginPreprocessingPass::<Impl, IMPL_OFFSET>,
            PreprocessSample: PreprocessSample::<Impl, IMPL_OFFSET>,
            EndPreprocessingPass: EndPreprocessingPass::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPreprocess as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterPushSinkImpl: Sized + IWMWriterSinkImpl {
    fn Connect(&mut self, pwszurl: super::super::Foundation::PWSTR, pwsztemplateurl: super::super::Foundation::PWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn EndSession(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterPushSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPushSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterPushSinkVtbl {
        unsafe extern "system" fn Connect<Impl: IWMWriterPushSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pwsztemplateurl: super::super::Foundation::PWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pwsztemplateurl), ::core::mem::transmute_copy(&fautodestroy)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IWMWriterPushSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn EndSession<Impl: IWMWriterPushSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndSession().into()
        }
        Self {
            base: IWMWriterSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Connect: Connect::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            EndSession: EndSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPushSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterSinkImpl: Sized {
    fn OnHeader(&mut self, pheader: ::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn IsRealTime(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn AllocateDataUnit(&mut self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer>;
    fn OnDataUnit(&mut self, pdataunit: ::core::option::Option<INSSBuffer>) -> ::windows::core::Result<()>;
    fn OnEndWriting(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterSinkVtbl {
        unsafe extern "system" fn OnHeader<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnHeader(::core::mem::transmute(&pheader)).into()
        }
        unsafe extern "system" fn IsRealTime<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pfrealtime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateDataUnit<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocateDataUnit(::core::mem::transmute_copy(&cbdataunit)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdataunit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnit<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataUnit(::core::mem::transmute(&pdataunit)).into()
        }
        unsafe extern "system" fn OnEndWriting<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEndWriting().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnHeader: OnHeader::<Impl, IMPL_OFFSET>,
            IsRealTime: IsRealTime::<Impl, IMPL_OFFSET>,
            AllocateDataUnit: AllocateDataUnit::<Impl, IMPL_OFFSET>,
            OnDataUnit: OnDataUnit::<Impl, IMPL_OFFSET>,
            OnEndWriting: OnEndWriting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterSink as ::windows::core::Interface>::IID
    }
}
