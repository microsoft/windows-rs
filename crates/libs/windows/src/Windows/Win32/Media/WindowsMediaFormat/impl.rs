#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INSNetSourceCreator_Impl: Sized {
    fn Initialize(&self) -> ::windows::core::Result<()>;
    fn CreateNetSource(&self, pszstreamname: &::windows::core::PCWSTR, pmonitor: ::core::option::Option<&::windows::core::IUnknown>, pdata: *const u8, pusercontext: ::core::option::Option<&::windows::core::IUnknown>, pcallback: ::core::option::Option<&::windows::core::IUnknown>, qwcontext: u64) -> ::windows::core::Result<()>;
    fn GetNetSourceProperties(&self, pszstreamname: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetNetSourceSharedNamespace(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetNetSourceAdminInterface(&self, pszstreamname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32>;
    fn GetProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::Result<()>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for INSNetSourceCreator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INSNetSourceCreator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>() -> INSNetSourceCreator_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize().into()
        }
        unsafe extern "system" fn CreateNetSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: ::windows::core::PCWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateNetSource(::core::mem::transmute(&pszstreamname), ::windows::core::from_raw_borrowed(&pmonitor), ::core::mem::transmute_copy(&pdata), ::windows::core::from_raw_borrowed(&pusercontext), ::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&qwcontext)).into()
        }
        unsafe extern "system" fn GetNetSourceProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: ::windows::core::PCWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetSourceProperties(::core::mem::transmute(&pszstreamname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertiesnode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceSharedNamespace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetSourceSharedNamespace() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsharednamespace, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetSourceAdminInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: ::windows::core::PCWSTR, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetSourceAdminInterface(::core::mem::transmute(&pszstreamname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumProtocolsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcprotocols, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProtocolName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProtocolName(::core::mem::transmute_copy(&dwprotocolnum), ::core::mem::transmute_copy(&pwszprotocolname), ::core::mem::transmute_copy(&pcchprotocolname)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSNetSourceCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<INSNetSourceCreator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait INSSBuffer_Impl: Sized {
    fn GetLength(&self) -> ::windows::core::Result<u32>;
    fn SetLength(&self, dwlength: u32) -> ::windows::core::Result<()>;
    fn GetMaxLength(&self) -> ::windows::core::Result<u32>;
    fn GetBuffer(&self) -> ::windows::core::Result<*mut u8>;
    fn GetBufferAndLength(&self, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INSSBuffer {}
impl INSSBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: isize>() -> INSSBuffer_Vtbl {
        unsafe extern "system" fn GetLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLength(::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn GetMaxLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdwbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferAndLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferAndLength(::core::mem::transmute_copy(&ppdwbuffer), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
            GetMaxLength: GetMaxLength::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            GetBufferAndLength: GetBufferAndLength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait INSSBuffer2_Impl: Sized + INSSBuffer_Impl {
    fn GetSampleProperties(&self, cbproperties: u32) -> ::windows::core::Result<u8>;
    fn SetSampleProperties(&self, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INSSBuffer2 {}
impl INSSBuffer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer2_Impl, const OFFSET: isize>() -> INSSBuffer2_Vtbl {
        unsafe extern "system" fn GetSampleProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSampleProperties(::core::mem::transmute_copy(&cbproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSampleProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSampleProperties(::core::mem::transmute_copy(&cbproperties), ::core::mem::transmute_copy(&pbproperties)).into()
        }
        Self {
            base__: INSSBuffer_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSampleProperties: GetSampleProperties::<Identity, Impl, OFFSET>,
            SetSampleProperties: SetSampleProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer2 as ::windows::core::ComInterface>::IID || iid == &<INSSBuffer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait INSSBuffer3_Impl: Sized + INSSBuffer2_Impl {
    fn SetProperty(&self, guidbufferproperty: &::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, guidbufferproperty: &::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INSSBuffer3 {}
impl INSSBuffer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer3_Impl, const OFFSET: isize>() -> INSSBuffer3_Vtbl {
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute(&guidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&dwbufferpropertysize)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute(&guidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&pdwbufferpropertysize)).into()
        }
        Self {
            base__: INSSBuffer2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer3 as ::windows::core::ComInterface>::IID || iid == &<INSSBuffer as ::windows::core::ComInterface>::IID || iid == &<INSSBuffer2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait INSSBuffer4_Impl: Sized + INSSBuffer3_Impl {
    fn GetPropertyCount(&self) -> ::windows::core::Result<u32>;
    fn GetPropertyByIndex(&self, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INSSBuffer4 {}
impl INSSBuffer4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer4_Impl, const OFFSET: isize>() -> INSSBuffer4_Vtbl {
        unsafe extern "system" fn GetPropertyCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbufferproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INSSBuffer4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyByIndex(::core::mem::transmute_copy(&dwbufferpropertyindex), ::core::mem::transmute_copy(&pguidbufferproperty), ::core::mem::transmute_copy(&pvbufferproperty), ::core::mem::transmute_copy(&pdwbufferpropertysize)).into()
        }
        Self {
            base__: INSSBuffer3_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, Impl, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INSSBuffer4 as ::windows::core::ComInterface>::IID || iid == &<INSSBuffer as ::windows::core::ComInterface>::IID || iid == &<INSSBuffer2 as ::windows::core::ComInterface>::IID || iid == &<INSSBuffer3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMAddressAccess_Impl: Sized {
    fn GetAccessEntryCount(&self, aetype: WM_AETYPE) -> ::windows::core::Result<u32>;
    fn GetAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<WM_ADDRESS_ACCESSENTRY>;
    fn AddAccessEntry(&self, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::Result<()>;
    fn RemoveAccessEntry(&self, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMAddressAccess {}
impl IWMAddressAccess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: isize>() -> IWMAddressAccess_Vtbl {
        unsafe extern "system" fn GetAccessEntryCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccessEntryCount(::core::mem::transmute_copy(&aetype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcentries, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAccessEntry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAccessEntry(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(paddraccessentry, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAccessEntry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAccessEntry(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&paddraccessentry)).into()
        }
        unsafe extern "system" fn RemoveAccessEntry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAddressAccess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAccessEntry(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAccessEntryCount: GetAccessEntryCount::<Identity, Impl, OFFSET>,
            GetAccessEntry: GetAccessEntry::<Identity, Impl, OFFSET>,
            AddAccessEntry: AddAccessEntry::<Identity, Impl, OFFSET>,
            RemoveAccessEntry: RemoveAccessEntry::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMAddressAccess as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMAddressAccess2_Impl: Sized + IWMAddressAccess_Impl {
    fn GetAccessEntryEx(&self, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut ::windows::core::BSTR, pbstrmask: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AddAccessEntryEx(&self, aetype: WM_AETYPE, bstraddress: &::windows::core::BSTR, bstrmask: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMAddressAccess2 {}
impl IWMAddressAccess2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAddressAccess2_Impl, const OFFSET: isize>() -> IWMAddressAccess2_Vtbl {
        unsafe extern "system" fn GetAccessEntryEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAddressAccess2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrmask: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAccessEntryEx(::core::mem::transmute_copy(&aetype), ::core::mem::transmute_copy(&dwentrynum), ::core::mem::transmute_copy(&pbstraddress), ::core::mem::transmute_copy(&pbstrmask)).into()
        }
        unsafe extern "system" fn AddAccessEntryEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAddressAccess2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrmask: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAccessEntryEx(::core::mem::transmute_copy(&aetype), ::core::mem::transmute(&bstraddress), ::core::mem::transmute(&bstrmask)).into()
        }
        Self {
            base__: IWMAddressAccess_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAccessEntryEx: GetAccessEntryEx::<Identity, Impl, OFFSET>,
            AddAccessEntryEx: AddAccessEntryEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMAddressAccess2 as ::windows::core::ComInterface>::IID || iid == &<IWMAddressAccess as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMAuthorizer_Impl: Sized {
    fn GetCertCount(&self) -> ::windows::core::Result<u32>;
    fn GetCert(&self, dwindex: u32) -> ::windows::core::Result<*mut u8>;
    fn GetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8) -> ::windows::core::Result<*mut u8>;
}
impl ::windows::core::RuntimeName for IWMAuthorizer {}
impl IWMAuthorizer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAuthorizer_Impl, const OFFSET: isize>() -> IWMAuthorizer_Vtbl {
        unsafe extern "system" fn GetCertCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAuthorizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCertCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccerts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCert<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAuthorizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCert(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbcertdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMAuthorizer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSharedData(::core::mem::transmute_copy(&dwcertindex), ::core::mem::transmute_copy(&pbshareddata), ::core::mem::transmute_copy(&pbcert)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbshareddata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCertCount: GetCertCount::<Identity, Impl, OFFSET>,
            GetCert: GetCert::<Identity, Impl, OFFSET>,
            GetSharedData: GetSharedData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMAuthorizer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMBackupRestoreProps_Impl: Sized {
    fn GetPropCount(&self) -> ::windows::core::Result<u16>;
    fn GetPropByIndex(&self, windex: u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn GetPropByName(&self, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetProp(&self, pszname: &::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn RemoveProp(&self, pcwszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RemoveAllProps(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMBackupRestoreProps {}
impl IWMBackupRestoreProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>() -> IWMBackupRestoreProps_Vtbl {
        unsafe extern "system" fn GetPropCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcprops, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropByIndex(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetPropByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropByName(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetProp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProp(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn RemoveProp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveProp(::core::mem::transmute(&pcwszname)).into()
        }
        unsafe extern "system" fn RemoveAllProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBackupRestoreProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllProps().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropCount: GetPropCount::<Identity, Impl, OFFSET>,
            GetPropByIndex: GetPropByIndex::<Identity, Impl, OFFSET>,
            GetPropByName: GetPropByName::<Identity, Impl, OFFSET>,
            SetProp: SetProp::<Identity, Impl, OFFSET>,
            RemoveProp: RemoveProp::<Identity, Impl, OFFSET>,
            RemoveAllProps: RemoveAllProps::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMBackupRestoreProps as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMBandwidthSharing_Impl: Sized + IWMStreamList_Impl {
    fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetBandwidth(&self, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::Result<()>;
    fn SetBandwidth(&self, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMBandwidthSharing {}
impl IWMBandwidthSharing_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>() -> IWMBandwidthSharing_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&guidtype)).into()
        }
        unsafe extern "system" fn GetBandwidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBandwidth(::core::mem::transmute_copy(&pdwbitrate), ::core::mem::transmute_copy(&pmsbufferwindow)).into()
        }
        unsafe extern "system" fn SetBandwidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMBandwidthSharing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBandwidth(::core::mem::transmute_copy(&dwbitrate), ::core::mem::transmute_copy(&msbufferwindow)).into()
        }
        Self {
            base__: IWMStreamList_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            GetBandwidth: GetBandwidth::<Identity, Impl, OFFSET>,
            SetBandwidth: SetBandwidth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMBandwidthSharing as ::windows::core::ComInterface>::IID || iid == &<IWMStreamList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMClientConnections_Impl: Sized {
    fn GetClientCount(&self) -> ::windows::core::Result<u32>;
    fn GetClientProperties(&self, dwclientnum: u32) -> ::windows::core::Result<WM_CLIENT_PROPERTIES>;
}
impl ::windows::core::RuntimeName for IWMClientConnections {}
impl IWMClientConnections_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMClientConnections_Impl, const OFFSET: isize>() -> IWMClientConnections_Vtbl {
        unsafe extern "system" fn GetClientCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMClientConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClientCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcclients, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClientProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMClientConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClientProperties(::core::mem::transmute_copy(&dwclientnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclientproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientCount: GetClientCount::<Identity, Impl, OFFSET>,
            GetClientProperties: GetClientProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMClientConnections as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMClientConnections2_Impl: Sized + IWMClientConnections_Impl {
    fn GetClientInfo(&self, dwclientnum: u32, pwsznetworkaddress: ::windows::core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows::core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows::core::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMClientConnections2 {}
impl IWMClientConnections2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMClientConnections2_Impl, const OFFSET: isize>() -> IWMClientConnections2_Vtbl {
        unsafe extern "system" fn GetClientInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMClientConnections2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: ::windows::core::PWSTR, pcchnetworkaddress: *mut u32, pwszport: ::windows::core::PWSTR, pcchport: *mut u32, pwszdnsname: ::windows::core::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClientInfo(::core::mem::transmute_copy(&dwclientnum), ::core::mem::transmute_copy(&pwsznetworkaddress), ::core::mem::transmute_copy(&pcchnetworkaddress), ::core::mem::transmute_copy(&pwszport), ::core::mem::transmute_copy(&pcchport), ::core::mem::transmute_copy(&pwszdnsname), ::core::mem::transmute_copy(&pcchdnsname)).into()
        }
        Self { base__: IWMClientConnections_Vtbl::new::<Identity, Impl, OFFSET>(), GetClientInfo: GetClientInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMClientConnections2 as ::windows::core::ComInterface>::IID || iid == &<IWMClientConnections as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMCodecInfo_Impl: Sized {
    fn GetCodecInfoCount(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn GetCodecFormatCount(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32) -> ::windows::core::Result<u32>;
    fn GetCodecFormat(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32) -> ::windows::core::Result<IWMStreamConfig>;
}
impl ::windows::core::RuntimeName for IWMCodecInfo {}
impl IWMCodecInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo_Impl, const OFFSET: isize>() -> IWMCodecInfo_Vtbl {
        unsafe extern "system" fn GetCodecInfoCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCodecInfoCount(::core::mem::transmute_copy(&guidtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccodecs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormatCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCodecFormatCount(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCodecFormat(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppistreamconfig, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Identity, Impl, OFFSET>,
            GetCodecFormatCount: GetCodecFormatCount::<Identity, Impl, OFFSET>,
            GetCodecFormat: GetCodecFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMCodecInfo2_Impl: Sized + IWMCodecInfo_Impl {
    fn GetCodecName(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()>;
    fn GetCodecFormatDesc(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::core::option::Option<IWMStreamConfig>, wszdesc: ::windows::core::PWSTR, pcchdesc: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMCodecInfo2 {}
impl IWMCodecInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo2_Impl, const OFFSET: isize>() -> IWMCodecInfo2_Vtbl {
        unsafe extern "system" fn GetCodecName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodecName(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&wszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetCodecFormatDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut *mut ::core::ffi::c_void, wszdesc: ::windows::core::PWSTR, pcchdesc: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodecFormatDesc(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex), ::core::mem::transmute_copy(&ppistreamconfig), ::core::mem::transmute_copy(&wszdesc), ::core::mem::transmute_copy(&pcchdesc)).into()
        }
        Self {
            base__: IWMCodecInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCodecName: GetCodecName::<Identity, Impl, OFFSET>,
            GetCodecFormatDesc: GetCodecFormatDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecInfo2 as ::windows::core::ComInterface>::IID || iid == &<IWMCodecInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMCodecInfo3_Impl: Sized + IWMCodecInfo2_Impl {
    fn GetCodecFormatProp(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn GetCodecProp(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn SetCodecEnumerationSetting(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: &::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn GetCodecEnumerationSetting(&self, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMCodecInfo3 {}
impl IWMCodecInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>() -> IWMCodecInfo3_Vtbl {
        unsafe extern "system" fn GetCodecFormatProp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodecFormatProp(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute_copy(&dwformatindex), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn GetCodecProp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodecProp(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn SetCodecEnumerationSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCodecEnumerationSetting(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetCodecEnumerationSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCodecInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodecEnumerationSetting(::core::mem::transmute_copy(&guidtype), ::core::mem::transmute_copy(&dwcodecindex), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self {
            base__: IWMCodecInfo2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCodecFormatProp: GetCodecFormatProp::<Identity, Impl, OFFSET>,
            GetCodecProp: GetCodecProp::<Identity, Impl, OFFSET>,
            SetCodecEnumerationSetting: SetCodecEnumerationSetting::<Identity, Impl, OFFSET>,
            GetCodecEnumerationSetting: GetCodecEnumerationSetting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCodecInfo3 as ::windows::core::ComInterface>::IID || iid == &<IWMCodecInfo as ::windows::core::ComInterface>::IID || iid == &<IWMCodecInfo2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMCredentialCallback_Impl: Sized {
    fn AcquireCredentials(&self, pwszrealm: &::windows::core::PCWSTR, pwszsite: &::windows::core::PCWSTR, pwszuser: ::windows::core::PWSTR, cchuser: u32, pwszpassword: ::windows::core::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMCredentialCallback {}
impl IWMCredentialCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCredentialCallback_Impl, const OFFSET: isize>() -> IWMCredentialCallback_Vtbl {
        unsafe extern "system" fn AcquireCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMCredentialCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrealm: ::windows::core::PCWSTR, pwszsite: ::windows::core::PCWSTR, pwszuser: ::windows::core::PWSTR, cchuser: u32, pwszpassword: ::windows::core::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireCredentials(::core::mem::transmute(&pwszrealm), ::core::mem::transmute(&pwszsite), ::core::mem::transmute_copy(&pwszuser), ::core::mem::transmute_copy(&cchuser), ::core::mem::transmute_copy(&pwszpassword), ::core::mem::transmute_copy(&cchpassword), ::core::mem::transmute_copy(&hrstatus), ::core::mem::transmute_copy(&pdwflags)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AcquireCredentials: AcquireCredentials::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCredentialCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMDRMEditor_Impl: Sized {
    fn GetDRMProperty(&self, pwstrname: &::windows::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDRMEditor {}
impl IWMDRMEditor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMEditor_Impl, const OFFSET: isize>() -> IWMDRMEditor_Vtbl {
        unsafe extern "system" fn GetDRMProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: ::windows::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDRMProperty(::core::mem::transmute(&pwstrname), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDRMProperty: GetDRMProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMEditor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMDRMMessageParser_Impl: Sized {
    fn ParseRegistrationReqMsg(&self, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::Result<()>;
    fn ParseLicenseRequestMsg(&self, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::core::option::Option<INSSBuffer>, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDRMMessageParser {}
impl IWMDRMMessageParser_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMMessageParser_Impl, const OFFSET: isize>() -> IWMDRMMessageParser_Vtbl {
        unsafe extern "system" fn ParseRegistrationReqMsg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMMessageParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut *mut ::core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseRegistrationReqMsg(::core::mem::transmute_copy(&pbregistrationreqmsg), ::core::mem::transmute_copy(&cbregistrationreqmsg), ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber)).into()
        }
        unsafe extern "system" fn ParseLicenseRequestMsg<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMMessageParser_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut *mut ::core::ffi::c_void, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseLicenseRequestMsg(::core::mem::transmute_copy(&pblicenserequestmsg), ::core::mem::transmute_copy(&cblicenserequestmsg), ::core::mem::transmute_copy(&ppdevicecert), ::core::mem::transmute_copy(&pdeviceserialnumber), ::core::mem::transmute_copy(&pbstraction)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ParseRegistrationReqMsg: ParseRegistrationReqMsg::<Identity, Impl, OFFSET>,
            ParseLicenseRequestMsg: ParseLicenseRequestMsg::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMMessageParser as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMDRMReader_Impl: Sized {
    fn AcquireLicense(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn CancelLicenseAcquisition(&self) -> ::windows::core::Result<()>;
    fn Individualize(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn CancelIndividualization(&self) -> ::windows::core::Result<()>;
    fn MonitorLicenseAcquisition(&self) -> ::windows::core::Result<()>;
    fn CancelMonitorLicenseAcquisition(&self) -> ::windows::core::Result<()>;
    fn SetDRMProperty(&self, pwstrname: &::windows::core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn GetDRMProperty(&self, pwstrname: &::windows::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDRMReader {}
impl IWMDRMReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: isize>() -> IWMDRMReader_Vtbl {
        unsafe extern "system" fn AcquireLicense<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireLicense(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CancelLicenseAcquisition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelLicenseAcquisition().into()
        }
        unsafe extern "system" fn Individualize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Individualize(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn CancelIndividualization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelIndividualization().into()
        }
        unsafe extern "system" fn MonitorLicenseAcquisition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MonitorLicenseAcquisition().into()
        }
        unsafe extern "system" fn CancelMonitorLicenseAcquisition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelMonitorLicenseAcquisition().into()
        }
        unsafe extern "system" fn SetDRMProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: ::windows::core::PCWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDRMProperty(::core::mem::transmute(&pwstrname), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetDRMProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: ::windows::core::PCWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDRMProperty(::core::mem::transmute(&pwstrname), ::core::mem::transmute_copy(&pdwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMDRMReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReader2_Impl: Sized + IWMDRMReader_Impl {
    fn SetEvaluateOutputLevelLicenses(&self, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPlayOutputLevels(&self, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()>;
    fn GetCopyOutputLevels(&self, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::Result<()>;
    fn TryNextLicense(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMDRMReader2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: isize>() -> IWMDRMReader2_Vtbl {
        unsafe extern "system" fn SetEvaluateOutputLevelLicenses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEvaluateOutputLevelLicenses(::core::mem::transmute_copy(&fevaluate)).into()
        }
        unsafe extern "system" fn GetPlayOutputLevels<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPlayOutputLevels(::core::mem::transmute_copy(&pplayopl), ::core::mem::transmute_copy(&pcblength), ::core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
        }
        unsafe extern "system" fn GetCopyOutputLevels<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCopyOutputLevels(::core::mem::transmute_copy(&pcopyopl), ::core::mem::transmute_copy(&pcblength), ::core::mem::transmute_copy(&pdwminappcompliancelevel)).into()
        }
        unsafe extern "system" fn TryNextLicense<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TryNextLicense().into()
        }
        Self {
            base__: IWMDRMReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetEvaluateOutputLevelLicenses: SetEvaluateOutputLevelLicenses::<Identity, Impl, OFFSET>,
            GetPlayOutputLevels: GetPlayOutputLevels::<Identity, Impl, OFFSET>,
            GetCopyOutputLevels: GetCopyOutputLevels::<Identity, Impl, OFFSET>,
            TryNextLicense: TryNextLicense::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMReader2 as ::windows::core::ComInterface>::IID || iid == &<IWMDRMReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMReader3_Impl: Sized + IWMDRMReader2_Impl {
    fn GetInclusionList(&self, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMDRMReader3 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReader3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader3_Impl, const OFFSET: isize>() -> IWMDRMReader3_Vtbl {
        unsafe extern "system" fn GetInclusionList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMReader3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInclusionList(::core::mem::transmute_copy(&ppguids), ::core::mem::transmute_copy(&pcguids)).into()
        }
        Self { base__: IWMDRMReader2_Vtbl::new::<Identity, Impl, OFFSET>(), GetInclusionList: GetInclusionList::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMReader3 as ::windows::core::ComInterface>::IID || iid == &<IWMDRMReader as ::windows::core::ComInterface>::IID || iid == &<IWMDRMReader2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMDRMTranscryptionManager_Impl: Sized {
    fn CreateTranscryptor(&self) -> ::windows::core::Result<IWMDRMTranscryptor>;
}
impl ::windows::core::RuntimeName for IWMDRMTranscryptionManager {}
impl IWMDRMTranscryptionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptionManager_Impl, const OFFSET: isize>() -> IWMDRMTranscryptionManager_Vtbl {
        unsafe extern "system" fn CreateTranscryptor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptranscryptor: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTranscryptor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptranscryptor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateTranscryptor: CreateTranscryptor::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptionManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMDRMTranscryptor_Impl: Sized {
    fn Initialize(&self, bstrfilename: &::windows::core::BSTR, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Seek(&self, hnstime: u64) -> ::windows::core::Result<()>;
    fn Read(&self, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDRMTranscryptor {}
impl IWMDRMTranscryptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>() -> IWMDRMTranscryptor_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows::core::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&bstrfilename), ::core::mem::transmute_copy(&pblicenserequestmsg), ::core::mem::transmute_copy(&cblicenserequestmsg), ::core::mem::transmute_copy(&pplicenseresponsemsg), ::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Seek(::core::mem::transmute_copy(&hnstime)).into()
        }
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&pcbdata)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            Read: Read::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMTranscryptor2_Impl: Sized + IWMDRMTranscryptor_Impl {
    fn SeekEx(&self, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ZeroAdjustTimestamps(&self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSeekStartTime(&self) -> ::windows::core::Result<u64>;
    fn GetDuration(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMDRMTranscryptor2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMTranscryptor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>() -> IWMDRMTranscryptor2_Vtbl {
        unsafe extern "system" fn SeekEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SeekEx(::core::mem::transmute_copy(&cnsstarttime), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&flrate), ::core::mem::transmute_copy(&fincludefileheader)).into()
        }
        unsafe extern "system" fn ZeroAdjustTimestamps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ZeroAdjustTimestamps(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetSeekStartTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSeekStartTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnstime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMTranscryptor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDuration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsduration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMDRMTranscryptor_Vtbl::new::<Identity, Impl, OFFSET>(),
            SeekEx: SeekEx::<Identity, Impl, OFFSET>,
            ZeroAdjustTimestamps: ZeroAdjustTimestamps::<Identity, Impl, OFFSET>,
            GetSeekStartTime: GetSeekStartTime::<Identity, Impl, OFFSET>,
            GetDuration: GetDuration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptor2 as ::windows::core::ComInterface>::IID || iid == &<IWMDRMTranscryptor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMDRMWriter_Impl: Sized {
    fn GenerateKeySeed(&self, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()>;
    fn GenerateKeyID(&self, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::Result<()>;
    fn GenerateSigningKeyPair(&self, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::Result<()>;
    fn SetDRMAttribute(&self, wstreamnum: u16, pszname: &::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMDRMWriter {}
impl IWMDRMWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: isize>() -> IWMDRMWriter_Vtbl {
        unsafe extern "system" fn GenerateKeySeed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszkeyseed: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateKeySeed(::core::mem::transmute_copy(&pwszkeyseed), ::core::mem::transmute_copy(&pcwchlength)).into()
        }
        unsafe extern "system" fn GenerateKeyID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszkeyid: ::windows::core::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateKeyID(::core::mem::transmute_copy(&pwszkeyid), ::core::mem::transmute_copy(&pcwchlength)).into()
        }
        unsafe extern "system" fn GenerateSigningKeyPair<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprivkey: ::windows::core::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: ::windows::core::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GenerateSigningKeyPair(::core::mem::transmute_copy(&pwszprivkey), ::core::mem::transmute_copy(&pcwchprivkeylength), ::core::mem::transmute_copy(&pwszpubkey), ::core::mem::transmute_copy(&pcwchpubkeylength)).into()
        }
        unsafe extern "system" fn SetDRMAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDRMAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GenerateKeySeed: GenerateKeySeed::<Identity, Impl, OFFSET>,
            GenerateKeyID: GenerateKeyID::<Identity, Impl, OFFSET>,
            GenerateSigningKeyPair: GenerateSigningKeyPair::<Identity, Impl, OFFSET>,
            SetDRMAttribute: SetDRMAttribute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter2_Impl: Sized + IWMDRMWriter_Impl {
    fn SetWMDRMNetEncryption(&self, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMDRMWriter2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMWriter2_Impl, const OFFSET: isize>() -> IWMDRMWriter2_Vtbl {
        unsafe extern "system" fn SetWMDRMNetEncryption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMWriter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWMDRMNetEncryption(::core::mem::transmute_copy(&fsamplesencrypted), ::core::mem::transmute_copy(&pbkeyid), ::core::mem::transmute_copy(&cbkeyid)).into()
        }
        Self { base__: IWMDRMWriter_Vtbl::new::<Identity, Impl, OFFSET>(), SetWMDRMNetEncryption: SetWMDRMNetEncryption::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter2 as ::windows::core::ComInterface>::IID || iid == &<IWMDRMWriter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter3_Impl: Sized + IWMDRMWriter2_Impl {
    fn SetProtectStreamSamples(&self, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMDRMWriter3 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMWriter3_Impl, const OFFSET: isize>() -> IWMDRMWriter3_Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDRMWriter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProtectStreamSamples(::core::mem::transmute_copy(&pimportinitstruct)).into()
        }
        Self { base__: IWMDRMWriter2_Vtbl::new::<Identity, Impl, OFFSET>(), SetProtectStreamSamples: SetProtectStreamSamples::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter3 as ::windows::core::ComInterface>::IID || iid == &<IWMDRMWriter as ::windows::core::ComInterface>::IID || iid == &<IWMDRMWriter2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMDeviceRegistration_Impl: Sized {
    fn RegisterDevice(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> ::windows::core::Result<IWMRegisteredDevice>;
    fn UnregisterDevice(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> ::windows::core::Result<()>;
    fn GetRegistrationStats(&self, dwregistertype: u32) -> ::windows::core::Result<u32>;
    fn GetFirstRegisteredDevice(&self, dwregistertype: u32) -> ::windows::core::Result<IWMRegisteredDevice>;
    fn GetNextRegisteredDevice(&self) -> ::windows::core::Result<IWMRegisteredDevice>;
    fn GetRegisteredDeviceByID(&self, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: &DRM_VAL16) -> ::windows::core::Result<IWMRegisteredDevice>;
}
impl ::windows::core::RuntimeName for IWMDeviceRegistration {}
impl IWMDeviceRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>() -> IWMDeviceRegistration_Vtbl {
        unsafe extern "system" fn RegisterDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterDevice(::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute(&serialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDevice(::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute(&serialnumber)).into()
        }
        unsafe extern "system" fn GetRegistrationStats<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegistrationStats(::core::mem::transmute_copy(&dwregistertype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcregistereddevices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFirstRegisteredDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFirstRegisteredDevice(::core::mem::transmute_copy(&dwregistertype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextRegisteredDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextRegisteredDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRegisteredDeviceByID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMDeviceRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRegisteredDeviceByID(::core::mem::transmute_copy(&dwregistertype), ::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute(&serialnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDevice: RegisterDevice::<Identity, Impl, OFFSET>,
            UnregisterDevice: UnregisterDevice::<Identity, Impl, OFFSET>,
            GetRegistrationStats: GetRegistrationStats::<Identity, Impl, OFFSET>,
            GetFirstRegisteredDevice: GetFirstRegisteredDevice::<Identity, Impl, OFFSET>,
            GetNextRegisteredDevice: GetNextRegisteredDevice::<Identity, Impl, OFFSET>,
            GetRegisteredDeviceByID: GetRegisteredDeviceByID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDeviceRegistration as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMGetSecureChannel_Impl: Sized {
    fn GetPeerSecureChannelInterface(&self) -> ::windows::core::Result<IWMSecureChannel>;
}
impl ::windows::core::RuntimeName for IWMGetSecureChannel {}
impl IWMGetSecureChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMGetSecureChannel_Impl, const OFFSET: isize>() -> IWMGetSecureChannel_Vtbl {
        unsafe extern "system" fn GetPeerSecureChannelInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMGetSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppeer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPeerSecureChannelInterface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppeer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPeerSecureChannelInterface: GetPeerSecureChannelInterface::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMGetSecureChannel as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMHeaderInfo_Impl: Sized {
    fn GetAttributeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16>;
    fn GetAttributeByIndex(&self, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn GetAttributeByName(&self, pwstreamnum: *mut u16, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetAttribute(&self, wstreamnum: u16, pszname: &::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn GetMarkerCount(&self) -> ::windows::core::Result<u16>;
    fn GetMarker(&self, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::Result<()>;
    fn AddMarker(&self, pwszmarkername: &::windows::core::PCWSTR, cnsmarkertime: u64) -> ::windows::core::Result<()>;
    fn RemoveMarker(&self, windex: u16) -> ::windows::core::Result<()>;
    fn GetScriptCount(&self) -> ::windows::core::Result<u16>;
    fn GetScript(&self, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::Result<()>;
    fn AddScript(&self, pwsztype: &::windows::core::PCWSTR, pwszcommand: &::windows::core::PCWSTR, cnsscripttime: u64) -> ::windows::core::Result<()>;
    fn RemoveScript(&self, windex: u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMHeaderInfo {}
impl IWMHeaderInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>() -> IWMHeaderInfo_Vtbl {
        unsafe extern "system" fn GetAttributeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeCount(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcattributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: ::windows::core::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributeByIndex(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwstreamnum), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn GetAttributeByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributeByName(::core::mem::transmute_copy(&pwstreamnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetMarkerCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMarkerCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcmarkers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMarker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: ::windows::core::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMarker(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszmarkername), ::core::mem::transmute_copy(&pcchmarkernamelen), ::core::mem::transmute_copy(&pcnsmarkertime)).into()
        }
        unsafe extern "system" fn AddMarker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmarkername: ::windows::core::PCWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMarker(::core::mem::transmute(&pwszmarkername), ::core::mem::transmute_copy(&cnsmarkertime)).into()
        }
        unsafe extern "system" fn RemoveMarker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveMarker(::core::mem::transmute_copy(&windex)).into()
        }
        unsafe extern "system" fn GetScriptCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScriptCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcscripts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScript<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: ::windows::core::PWSTR, pcchtypelen: *mut u16, pwszcommand: ::windows::core::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetScript(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwsztype), ::core::mem::transmute_copy(&pcchtypelen), ::core::mem::transmute_copy(&pwszcommand), ::core::mem::transmute_copy(&pcchcommandlen), ::core::mem::transmute_copy(&pcnsscripttime)).into()
        }
        unsafe extern "system" fn AddScript<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztype: ::windows::core::PCWSTR, pwszcommand: ::windows::core::PCWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddScript(::core::mem::transmute(&pwsztype), ::core::mem::transmute(&pwszcommand), ::core::mem::transmute_copy(&cnsscripttime)).into()
        }
        unsafe extern "system" fn RemoveScript<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveScript(::core::mem::transmute_copy(&windex)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMHeaderInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMHeaderInfo2_Impl: Sized + IWMHeaderInfo_Impl {
    fn GetCodecInfoCount(&self) -> ::windows::core::Result<u32>;
    fn GetCodecInfo(&self, windex: u32, pcchname: *mut u16, pwszname: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMHeaderInfo2 {}
impl IWMHeaderInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo2_Impl, const OFFSET: isize>() -> IWMHeaderInfo2_Vtbl {
        unsafe extern "system" fn GetCodecInfoCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCodecInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pccodecinfos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCodecInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodecInfo(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pcchname), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchdescription), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pcodectype), ::core::mem::transmute_copy(&pcbcodecinfo), ::core::mem::transmute_copy(&pbcodecinfo)).into()
        }
        Self {
            base__: IWMHeaderInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCodecInfoCount: GetCodecInfoCount::<Identity, Impl, OFFSET>,
            GetCodecInfo: GetCodecInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMHeaderInfo2 as ::windows::core::ComInterface>::IID || iid == &<IWMHeaderInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMHeaderInfo3_Impl: Sized + IWMHeaderInfo2_Impl {
    fn GetAttributeCountEx(&self, wstreamnum: u16) -> ::windows::core::Result<u16>;
    fn GetAttributeIndices(&self, wstreamnum: u16, pwszname: &::windows::core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::Result<()>;
    fn GetAttributeByIndexEx(&self, wstreamnum: u16, windex: u16, pwszname: ::windows::core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::Result<()>;
    fn ModifyAttribute(&self, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::Result<()>;
    fn AddAttribute(&self, wstreamnum: u16, pszname: &::windows::core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::Result<()>;
    fn DeleteAttribute(&self, wstreamnum: u16, windex: u16) -> ::windows::core::Result<()>;
    fn AddCodecInfo(&self, pwszname: &::windows::core::PCWSTR, pwszdescription: &::windows::core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMHeaderInfo3 {}
impl IWMHeaderInfo3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>() -> IWMHeaderInfo3_Vtbl {
        unsafe extern "system" fn GetAttributeCountEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeCountEx(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcattributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeIndices<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: ::windows::core::PCWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributeIndices(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pwszname), ::core::mem::transmute_copy(&pwlangindex), ::core::mem::transmute_copy(&pwindices), ::core::mem::transmute_copy(&pwcount)).into()
        }
        unsafe extern "system" fn GetAttributeByIndexEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: ::windows::core::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributeByIndexEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pwnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pwlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwdatalength)).into()
        }
        unsafe extern "system" fn ModifyAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ModifyAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&wlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn AddAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: ::windows::core::PCWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&pwindex), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&wlangindex), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwlength)).into()
        }
        unsafe extern "system" fn DeleteAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAttribute(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&windex)).into()
        }
        unsafe extern "system" fn AddCodecInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMHeaderInfo3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR, pwszdescription: ::windows::core::PCWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddCodecInfo(::core::mem::transmute(&pwszname), ::core::mem::transmute(&pwszdescription), ::core::mem::transmute_copy(&codectype), ::core::mem::transmute_copy(&cbcodecinfo), ::core::mem::transmute_copy(&pbcodecinfo)).into()
        }
        Self {
            base__: IWMHeaderInfo2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMHeaderInfo3 as ::windows::core::ComInterface>::IID || iid == &<IWMHeaderInfo as ::windows::core::ComInterface>::IID || iid == &<IWMHeaderInfo2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMIStreamProps_Impl: Sized {
    fn GetProperty(&self, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMIStreamProps {}
impl IWMIStreamProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIStreamProps_Impl, const OFFSET: isize>() -> IWMIStreamProps_Vtbl {
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIStreamProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProperty: GetProperty::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIStreamProps as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMImageInfo_Impl: Sized {
    fn GetImageCount(&self) -> ::windows::core::Result<u32>;
    fn GetImage(&self, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMImageInfo {}
impl IWMImageInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMImageInfo_Impl, const OFFSET: isize>() -> IWMImageInfo_Vtbl {
        unsafe extern "system" fn GetImageCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMImageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImageCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcimages, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMImageInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: ::windows::core::PWSTR, pcchdescription: *mut u16, pwszdescription: ::windows::core::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetImage(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pcchmimetype), ::core::mem::transmute_copy(&pwszmimetype), ::core::mem::transmute_copy(&pcchdescription), ::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pimagetype), ::core::mem::transmute_copy(&pcbimagedata), ::core::mem::transmute_copy(&pbimagedata)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetImageCount: GetImageCount::<Identity, Impl, OFFSET>,
            GetImage: GetImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMImageInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMIndexer_Impl: Sized {
    fn StartIndexing(&self, pwszurl: &::windows::core::PCWSTR, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMIndexer {}
impl IWMIndexer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIndexer_Impl, const OFFSET: isize>() -> IWMIndexer_Vtbl {
        unsafe extern "system" fn StartIndexing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIndexer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartIndexing(::core::mem::transmute(&pwszurl), ::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIndexer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartIndexing: StartIndexing::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIndexer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMIndexer2_Impl: Sized + IWMIndexer_Impl {
    fn Configure(&self, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMIndexer2 {}
impl IWMIndexer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIndexer2_Impl, const OFFSET: isize>() -> IWMIndexer2_Vtbl {
        unsafe extern "system" fn Configure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMIndexer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Configure(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&nindexertype), ::core::mem::transmute_copy(&pvinterval), ::core::mem::transmute_copy(&pvindextype)).into()
        }
        Self { base__: IWMIndexer_Vtbl::new::<Identity, Impl, OFFSET>(), Configure: Configure::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIndexer2 as ::windows::core::ComInterface>::IID || iid == &<IWMIndexer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMInputMediaProps_Impl: Sized + IWMMediaProps_Impl {
    fn GetConnectionName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
    fn GetGroupName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMInputMediaProps {}
#[cfg(feature = "Win32_Foundation")]
impl IWMInputMediaProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMInputMediaProps_Impl, const OFFSET: isize>() -> IWMInputMediaProps_Vtbl {
        unsafe extern "system" fn GetConnectionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMInputMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConnectionName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetGroupName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMInputMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGroupName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        Self {
            base__: IWMMediaProps_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetConnectionName: GetConnectionName::<Identity, Impl, OFFSET>,
            GetGroupName: GetGroupName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMInputMediaProps as ::windows::core::ComInterface>::IID || iid == &<IWMMediaProps as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMLanguageList_Impl: Sized {
    fn GetLanguageCount(&self) -> ::windows::core::Result<u16>;
    fn GetLanguageDetails(&self, windex: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()>;
    fn AddLanguageByRFC1766String(&self, pwszlanguagestring: &::windows::core::PCWSTR) -> ::windows::core::Result<u16>;
}
impl ::windows::core::RuntimeName for IWMLanguageList {}
impl IWMLanguageList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLanguageList_Impl, const OFFSET: isize>() -> IWMLanguageList_Vtbl {
        unsafe extern "system" fn GetLanguageCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLanguageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguageCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguageDetails<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLanguageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLanguageDetails(::core::mem::transmute_copy(&windex), ::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn AddLanguageByRFC1766String<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLanguageList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows::core::PCWSTR, pwindex: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddLanguageByRFC1766String(::core::mem::transmute(&pwszlanguagestring)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLanguageCount: GetLanguageCount::<Identity, Impl, OFFSET>,
            GetLanguageDetails: GetLanguageDetails::<Identity, Impl, OFFSET>,
            AddLanguageByRFC1766String: AddLanguageByRFC1766String::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLanguageList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMLicenseBackup_Impl: Sized {
    fn BackupLicenses(&self, dwflags: u32, pcallback: ::core::option::Option<&IWMStatusCallback>) -> ::windows::core::Result<()>;
    fn CancelLicenseBackup(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMLicenseBackup {}
impl IWMLicenseBackup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLicenseBackup_Impl, const OFFSET: isize>() -> IWMLicenseBackup_Vtbl {
        unsafe extern "system" fn BackupLicenses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLicenseBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackupLicenses(::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn CancelLicenseBackup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLicenseBackup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelLicenseBackup().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BackupLicenses: BackupLicenses::<Identity, Impl, OFFSET>,
            CancelLicenseBackup: CancelLicenseBackup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLicenseBackup as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMLicenseRestore_Impl: Sized {
    fn RestoreLicenses(&self, dwflags: u32, pcallback: ::core::option::Option<&IWMStatusCallback>) -> ::windows::core::Result<()>;
    fn CancelLicenseRestore(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMLicenseRestore {}
impl IWMLicenseRestore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLicenseRestore_Impl, const OFFSET: isize>() -> IWMLicenseRestore_Vtbl {
        unsafe extern "system" fn RestoreLicenses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLicenseRestore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreLicenses(::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&pcallback)).into()
        }
        unsafe extern "system" fn CancelLicenseRestore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLicenseRestore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelLicenseRestore().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RestoreLicenses: RestoreLicenses::<Identity, Impl, OFFSET>,
            CancelLicenseRestore: CancelLicenseRestore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLicenseRestore as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMLicenseRevocationAgent_Impl: Sized {
    fn GetLRBChallenge(&self, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::Result<()>;
    fn ProcessLRB(&self, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMLicenseRevocationAgent {}
impl IWMLicenseRevocationAgent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLicenseRevocationAgent_Impl, const OFFSET: isize>() -> IWMLicenseRevocationAgent_Vtbl {
        unsafe extern "system" fn GetLRBChallenge<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLicenseRevocationAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLRBChallenge(::core::mem::transmute_copy(&pmachineid), ::core::mem::transmute_copy(&dwmachineidlength), ::core::mem::transmute_copy(&pchallenge), ::core::mem::transmute_copy(&dwchallengelength), ::core::mem::transmute_copy(&pchallengeoutput), ::core::mem::transmute_copy(&pdwchallengeoutputlength)).into()
        }
        unsafe extern "system" fn ProcessLRB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMLicenseRevocationAgent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessLRB(::core::mem::transmute_copy(&psignedlrb), ::core::mem::transmute_copy(&dwsignedlrblength), ::core::mem::transmute_copy(&psignedack), ::core::mem::transmute_copy(&pdwsignedacklength)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLRBChallenge: GetLRBChallenge::<Identity, Impl, OFFSET>,
            ProcessLRB: ProcessLRB::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMLicenseRevocationAgent as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMMediaProps_Impl: Sized {
    fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetMediaType(&self, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::Result<()>;
    fn SetMediaType(&self, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMMediaProps {}
#[cfg(feature = "Win32_Foundation")]
impl IWMMediaProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMediaProps_Impl, const OFFSET: isize>() -> IWMMediaProps_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMediaType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMediaType(::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pcbtype)).into()
        }
        unsafe extern "system" fn SetMediaType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMediaType(::core::mem::transmute_copy(&ptype)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetMediaType: GetMediaType::<Identity, Impl, OFFSET>,
            SetMediaType: SetMediaType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMediaProps as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMMetadataEditor_Impl: Sized {
    fn Open(&self, pwszfilename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn Flush(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMMetadataEditor {}
impl IWMMetadataEditor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMetadataEditor_Impl, const OFFSET: isize>() -> IWMMetadataEditor_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMetadataEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&pwszfilename)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMetadataEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMetadataEditor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flush().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Flush: Flush::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMetadataEditor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMMetadataEditor2_Impl: Sized + IWMMetadataEditor_Impl {
    fn OpenEx(&self, pwszfilename: &::windows::core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMMetadataEditor2 {}
impl IWMMetadataEditor2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMetadataEditor2_Impl, const OFFSET: isize>() -> IWMMetadataEditor2_Vtbl {
        unsafe extern "system" fn OpenEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMetadataEditor2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenEx(::core::mem::transmute(&pwszfilename), ::core::mem::transmute_copy(&dwdesiredaccess), ::core::mem::transmute_copy(&dwsharemode)).into()
        }
        Self { base__: IWMMetadataEditor_Vtbl::new::<Identity, Impl, OFFSET>(), OpenEx: OpenEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMetadataEditor2 as ::windows::core::ComInterface>::IID || iid == &<IWMMetadataEditor as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMMutualExclusion_Impl: Sized + IWMStreamList_Impl {
    fn GetType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetType(&self, guidtype: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMMutualExclusion {}
impl IWMMutualExclusion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion_Impl, const OFFSET: isize>() -> IWMMutualExclusion_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&guidtype)).into()
        }
        Self {
            base__: IWMStreamList_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMutualExclusion as ::windows::core::ComInterface>::IID || iid == &<IWMStreamList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMMutualExclusion2_Impl: Sized + IWMMutualExclusion_Impl {
    fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
    fn SetName(&self, pwszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetRecordCount(&self) -> ::windows::core::Result<u16>;
    fn AddRecord(&self) -> ::windows::core::Result<()>;
    fn RemoveRecord(&self, wrecordnumber: u16) -> ::windows::core::Result<()>;
    fn GetRecordName(&self, wrecordnumber: u16, pwszrecordname: ::windows::core::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::Result<()>;
    fn SetRecordName(&self, wrecordnumber: u16, pwszrecordname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetStreamsForRecord(&self, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()>;
    fn AddStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()>;
    fn RemoveStreamForRecord(&self, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMMutualExclusion2 {}
impl IWMMutualExclusion2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>() -> IWMMutualExclusion2_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn GetRecordCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecordCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwrecordcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRecord().into()
        }
        unsafe extern "system" fn RemoveRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveRecord(::core::mem::transmute_copy(&wrecordnumber)).into()
        }
        unsafe extern "system" fn GetRecordName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows::core::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRecordName(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwszrecordname), ::core::mem::transmute_copy(&pcchrecordname)).into()
        }
        unsafe extern "system" fn SetRecordName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecordName(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute(&pwszrecordname)).into()
        }
        unsafe extern "system" fn GetStreamsForRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreamsForRecord(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&pwstreamnumarray), ::core::mem::transmute_copy(&pcstreams)).into()
        }
        unsafe extern "system" fn AddStreamForRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStreamForRecord(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&wstreamnumber)).into()
        }
        unsafe extern "system" fn RemoveStreamForRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMMutualExclusion2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStreamForRecord(::core::mem::transmute_copy(&wrecordnumber), ::core::mem::transmute_copy(&wstreamnumber)).into()
        }
        Self {
            base__: IWMMutualExclusion_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMMutualExclusion2 as ::windows::core::ComInterface>::IID || iid == &<IWMStreamList as ::windows::core::ComInterface>::IID || iid == &<IWMMutualExclusion as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMOutputMediaProps_Impl: Sized + IWMMediaProps_Impl {
    fn GetStreamGroupName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
    fn GetConnectionName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMOutputMediaProps {}
#[cfg(feature = "Win32_Foundation")]
impl IWMOutputMediaProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMOutputMediaProps_Impl, const OFFSET: isize>() -> IWMOutputMediaProps_Vtbl {
        unsafe extern "system" fn GetStreamGroupName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMOutputMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreamGroupName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn GetConnectionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMOutputMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConnectionName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        Self {
            base__: IWMMediaProps_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStreamGroupName: GetStreamGroupName::<Identity, Impl, OFFSET>,
            GetConnectionName: GetConnectionName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMOutputMediaProps as ::windows::core::ComInterface>::IID || iid == &<IWMMediaProps as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMPacketSize_Impl: Sized {
    fn GetMaxPacketSize(&self) -> ::windows::core::Result<u32>;
    fn SetMaxPacketSize(&self, dwmaxpacketsize: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPacketSize {}
impl IWMPacketSize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPacketSize_Impl, const OFFSET: isize>() -> IWMPacketSize_Vtbl {
        unsafe extern "system" fn GetMaxPacketSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPacketSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxpacketsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxPacketSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPacketSize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxPacketSize(::core::mem::transmute_copy(&dwmaxpacketsize)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxPacketSize: GetMaxPacketSize::<Identity, Impl, OFFSET>,
            SetMaxPacketSize: SetMaxPacketSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPacketSize as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMPacketSize2_Impl: Sized + IWMPacketSize_Impl {
    fn GetMinPacketSize(&self) -> ::windows::core::Result<u32>;
    fn SetMinPacketSize(&self, dwminpacketsize: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPacketSize2 {}
impl IWMPacketSize2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPacketSize2_Impl, const OFFSET: isize>() -> IWMPacketSize2_Vtbl {
        unsafe extern "system" fn GetMinPacketSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPacketSize2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwminpacketsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPacketSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPacketSize2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinPacketSize(::core::mem::transmute_copy(&dwminpacketsize)).into()
        }
        Self {
            base__: IWMPacketSize_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMinPacketSize: GetMinPacketSize::<Identity, Impl, OFFSET>,
            SetMinPacketSize: SetMinPacketSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPacketSize2 as ::windows::core::ComInterface>::IID || iid == &<IWMPacketSize as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMPlayerHook_Impl: Sized {
    fn PreDecode(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPlayerHook {}
impl IWMPlayerHook_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPlayerHook_Impl, const OFFSET: isize>() -> IWMPlayerHook_Vtbl {
        unsafe extern "system" fn PreDecode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPlayerHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreDecode().into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), PreDecode: PreDecode::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPlayerHook as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMPlayerTimestampHook_Impl: Sized {
    fn MapTimestamp(&self, rtin: i64) -> ::windows::core::Result<i64>;
}
impl ::windows::core::RuntimeName for IWMPlayerTimestampHook {}
impl IWMPlayerTimestampHook_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPlayerTimestampHook_Impl, const OFFSET: isize>() -> IWMPlayerTimestampHook_Vtbl {
        unsafe extern "system" fn MapTimestamp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPlayerTimestampHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MapTimestamp(::core::mem::transmute_copy(&rtin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prtout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), MapTimestamp: MapTimestamp::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPlayerTimestampHook as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMProfile_Impl: Sized {
    fn GetVersion(&self) -> ::windows::core::Result<WMT_VERSION>;
    fn GetName(&self, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::Result<()>;
    fn SetName(&self, pwszname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetDescription(&self, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::Result<()>;
    fn SetDescription(&self, pwszdescription: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetStreamCount(&self) -> ::windows::core::Result<u32>;
    fn GetStream(&self, dwstreamindex: u32) -> ::windows::core::Result<IWMStreamConfig>;
    fn GetStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<IWMStreamConfig>;
    fn RemoveStream(&self, pconfig: ::core::option::Option<&IWMStreamConfig>) -> ::windows::core::Result<()>;
    fn RemoveStreamByNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()>;
    fn AddStream(&self, pconfig: ::core::option::Option<&IWMStreamConfig>) -> ::windows::core::Result<()>;
    fn ReconfigStream(&self, pconfig: ::core::option::Option<&IWMStreamConfig>) -> ::windows::core::Result<()>;
    fn CreateNewStream(&self, guidstreamtype: *const ::windows::core::GUID) -> ::windows::core::Result<IWMStreamConfig>;
    fn GetMutualExclusionCount(&self) -> ::windows::core::Result<u32>;
    fn GetMutualExclusion(&self, dwmeindex: u32) -> ::windows::core::Result<IWMMutualExclusion>;
    fn RemoveMutualExclusion(&self, pme: ::core::option::Option<&IWMMutualExclusion>) -> ::windows::core::Result<()>;
    fn AddMutualExclusion(&self, pme: ::core::option::Option<&IWMMutualExclusion>) -> ::windows::core::Result<()>;
    fn CreateNewMutualExclusion(&self) -> ::windows::core::Result<IWMMutualExclusion>;
}
impl ::windows::core::RuntimeName for IWMProfile {}
impl IWMProfile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>() -> IWMProfile_Vtbl {
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&pwszname), ::core::mem::transmute_copy(&pcchname)).into()
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&pwszname)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdescription: ::windows::core::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDescription(::core::mem::transmute_copy(&pwszdescription), ::core::mem::transmute_copy(&pcchdescription)).into()
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&pwszdescription)).into()
        }
        unsafe extern "system" fn GetStreamCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcstreams, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStream(::core::mem::transmute_copy(&dwstreamindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfig, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamByNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamByNumber(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfig, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStream(::windows::core::from_raw_borrowed(&pconfig)).into()
        }
        unsafe extern "system" fn RemoveStreamByNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStreamByNumber(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn AddStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStream(::windows::core::from_raw_borrowed(&pconfig)).into()
        }
        unsafe extern "system" fn ReconfigStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReconfigStream(::windows::core::from_raw_borrowed(&pconfig)).into()
        }
        unsafe extern "system" fn CreateNewStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateNewStream(::core::mem::transmute_copy(&guidstreamtype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconfig, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMutualExclusionCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcme, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMutualExclusion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMutualExclusion(::core::mem::transmute_copy(&dwmeindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppme, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveMutualExclusion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pme: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveMutualExclusion(::windows::core::from_raw_borrowed(&pme)).into()
        }
        unsafe extern "system" fn AddMutualExclusion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pme: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMutualExclusion(::windows::core::from_raw_borrowed(&pme)).into()
        }
        unsafe extern "system" fn CreateNewMutualExclusion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppme: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateNewMutualExclusion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppme, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMProfile as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMProfile2_Impl: Sized + IWMProfile_Impl {
    fn GetProfileID(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IWMProfile2 {}
impl IWMProfile2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile2_Impl, const OFFSET: isize>() -> IWMProfile2_Vtbl {
        unsafe extern "system" fn GetProfileID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProfileID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IWMProfile_Vtbl::new::<Identity, Impl, OFFSET>(), GetProfileID: GetProfileID::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfile2 as ::windows::core::ComInterface>::IID || iid == &<IWMProfile as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMProfile3_Impl: Sized + IWMProfile2_Impl {
    fn GetStorageFormat(&self) -> ::windows::core::Result<WMT_STORAGE_FORMAT>;
    fn SetStorageFormat(&self, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::Result<()>;
    fn GetBandwidthSharingCount(&self) -> ::windows::core::Result<u32>;
    fn GetBandwidthSharing(&self, dwbsindex: u32) -> ::windows::core::Result<IWMBandwidthSharing>;
    fn RemoveBandwidthSharing(&self, pbs: ::core::option::Option<&IWMBandwidthSharing>) -> ::windows::core::Result<()>;
    fn AddBandwidthSharing(&self, pbs: ::core::option::Option<&IWMBandwidthSharing>) -> ::windows::core::Result<()>;
    fn CreateNewBandwidthSharing(&self) -> ::windows::core::Result<IWMBandwidthSharing>;
    fn GetStreamPrioritization(&self) -> ::windows::core::Result<IWMStreamPrioritization>;
    fn SetStreamPrioritization(&self, psp: ::core::option::Option<&IWMStreamPrioritization>) -> ::windows::core::Result<()>;
    fn RemoveStreamPrioritization(&self) -> ::windows::core::Result<()>;
    fn CreateNewStreamPrioritization(&self) -> ::windows::core::Result<IWMStreamPrioritization>;
    fn GetExpectedPacketCount(&self, msduration: u64) -> ::windows::core::Result<u64>;
}
impl ::windows::core::RuntimeName for IWMProfile3 {}
impl IWMProfile3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>() -> IWMProfile3_Vtbl {
        unsafe extern "system" fn GetStorageFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStorageFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnstorageformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStorageFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStorageFormat(::core::mem::transmute_copy(&nstorageformat)).into()
        }
        unsafe extern "system" fn GetBandwidthSharingCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBandwidthSharingCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBandwidthSharing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBandwidthSharing(::core::mem::transmute_copy(&dwbsindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveBandwidthSharing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveBandwidthSharing(::windows::core::from_raw_borrowed(&pbs)).into()
        }
        unsafe extern "system" fn AddBandwidthSharing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddBandwidthSharing(::windows::core::from_raw_borrowed(&pbs)).into()
        }
        unsafe extern "system" fn CreateNewBandwidthSharing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbs: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateNewBandwidthSharing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamPrioritization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamPrioritization() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamPrioritization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psp: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStreamPrioritization(::windows::core::from_raw_borrowed(&psp)).into()
        }
        unsafe extern "system" fn RemoveStreamPrioritization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStreamPrioritization().into()
        }
        unsafe extern "system" fn CreateNewStreamPrioritization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateNewStreamPrioritization() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExpectedPacketCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfile3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExpectedPacketCount(::core::mem::transmute_copy(&msduration)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcpackets, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMProfile2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMProfile3 as ::windows::core::ComInterface>::IID || iid == &<IWMProfile as ::windows::core::ComInterface>::IID || iid == &<IWMProfile2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMProfileManager_Impl: Sized {
    fn CreateEmptyProfile(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<IWMProfile>;
    fn LoadProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<IWMProfile>;
    fn LoadProfileByData(&self, pwszprofile: &::windows::core::PCWSTR) -> ::windows::core::Result<IWMProfile>;
    fn SaveProfile(&self, piwmprofile: ::core::option::Option<&IWMProfile>, pwszprofile: &::windows::core::PCWSTR, pdwlength: *mut u32) -> ::windows::core::Result<()>;
    fn GetSystemProfileCount(&self) -> ::windows::core::Result<u32>;
    fn LoadSystemProfile(&self, dwprofileindex: u32) -> ::windows::core::Result<IWMProfile>;
}
impl ::windows::core::RuntimeName for IWMProfileManager {}
impl IWMProfileManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: isize>() -> IWMProfileManager_Vtbl {
        unsafe extern "system" fn CreateEmptyProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateEmptyProfile(::core::mem::transmute_copy(&dwversion)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadProfileByID(::core::mem::transmute_copy(&guidprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadProfileByData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprofile: ::windows::core::PCWSTR, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadProfileByData(::core::mem::transmute(&pwszprofile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmprofile: *mut ::core::ffi::c_void, pwszprofile: ::windows::core::PCWSTR, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveProfile(::windows::core::from_raw_borrowed(&piwmprofile), ::core::mem::transmute(&pwszprofile), ::core::mem::transmute_copy(&pdwlength)).into()
        }
        unsafe extern "system" fn GetSystemProfileCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSystemProfileCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcprofiles, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadSystemProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LoadSystemProfile(::core::mem::transmute_copy(&dwprofileindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprofile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateEmptyProfile: CreateEmptyProfile::<Identity, Impl, OFFSET>,
            LoadProfileByID: LoadProfileByID::<Identity, Impl, OFFSET>,
            LoadProfileByData: LoadProfileByData::<Identity, Impl, OFFSET>,
            SaveProfile: SaveProfile::<Identity, Impl, OFFSET>,
            GetSystemProfileCount: GetSystemProfileCount::<Identity, Impl, OFFSET>,
            LoadSystemProfile: LoadSystemProfile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfileManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMProfileManager2_Impl: Sized + IWMProfileManager_Impl {
    fn GetSystemProfileVersion(&self, pdwversion: *mut WMT_VERSION) -> ::windows::core::Result<()>;
    fn SetSystemProfileVersion(&self, dwversion: WMT_VERSION) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMProfileManager2 {}
impl IWMProfileManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager2_Impl, const OFFSET: isize>() -> IWMProfileManager2_Vtbl {
        unsafe extern "system" fn GetSystemProfileVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSystemProfileVersion(::core::mem::transmute_copy(&pdwversion)).into()
        }
        unsafe extern "system" fn SetSystemProfileVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSystemProfileVersion(::core::mem::transmute_copy(&dwversion)).into()
        }
        Self {
            base__: IWMProfileManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSystemProfileVersion: GetSystemProfileVersion::<Identity, Impl, OFFSET>,
            SetSystemProfileVersion: SetSystemProfileVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfileManager2 as ::windows::core::ComInterface>::IID || iid == &<IWMProfileManager as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMProfileManagerLanguage_Impl: Sized {
    fn GetUserLanguageID(&self, wlangid: *mut u16) -> ::windows::core::Result<()>;
    fn SetUserLanguageID(&self, wlangid: u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMProfileManagerLanguage {}
impl IWMProfileManagerLanguage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManagerLanguage_Impl, const OFFSET: isize>() -> IWMProfileManagerLanguage_Vtbl {
        unsafe extern "system" fn GetUserLanguageID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManagerLanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUserLanguageID(::core::mem::transmute_copy(&wlangid)).into()
        }
        unsafe extern "system" fn SetUserLanguageID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProfileManagerLanguage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserLanguageID(::core::mem::transmute_copy(&wlangid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUserLanguageID: GetUserLanguageID::<Identity, Impl, OFFSET>,
            SetUserLanguageID: SetUserLanguageID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfileManagerLanguage as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMPropertyVault_Impl: Sized {
    fn GetPropertyCount(&self, pdwcount: *const u32) -> ::windows::core::Result<()>;
    fn GetPropertyByName(&self, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn SetProperty(&self, pszname: &::windows::core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::Result<()>;
    fn GetPropertyByIndex(&self, dwindex: u32, pszname: ::windows::core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::Result<()>;
    fn CopyPropertiesFrom(&self, piwmpropertyvault: ::core::option::Option<&IWMPropertyVault>) -> ::windows::core::Result<()>;
    fn Clear(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMPropertyVault {}
impl IWMPropertyVault_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: isize>() -> IWMPropertyVault_Vtbl {
        unsafe extern "system" fn GetPropertyCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyCount(::core::mem::transmute_copy(&pdwcount)).into()
        }
        unsafe extern "system" fn GetPropertyByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyByName(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: ::windows::core::PCWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&dwsize)).into()
        }
        unsafe extern "system" fn GetPropertyByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: ::windows::core::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyByIndex(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pszname), ::core::mem::transmute_copy(&pdwnamelen), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pdwsize)).into()
        }
        unsafe extern "system" fn CopyPropertiesFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpropertyvault: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyPropertiesFrom(::windows::core::from_raw_borrowed(&piwmpropertyvault)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMPropertyVault_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyCount: GetPropertyCount::<Identity, Impl, OFFSET>,
            GetPropertyByName: GetPropertyByName::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            GetPropertyByIndex: GetPropertyByIndex::<Identity, Impl, OFFSET>,
            CopyPropertiesFrom: CopyPropertiesFrom::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPropertyVault as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMProximityDetection_Impl: Sized {
    fn StartDetection(&self, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::core::option::Option<INSSBuffer>, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMProximityDetection {}
impl IWMProximityDetection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProximityDetection_Impl, const OFFSET: isize>() -> IWMProximityDetection_Vtbl {
        unsafe extern "system" fn StartDetection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMProximityDetection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartDetection(::core::mem::transmute_copy(&pbregistrationmsg), ::core::mem::transmute_copy(&cbregistrationmsg), ::core::mem::transmute_copy(&pblocaladdress), ::core::mem::transmute_copy(&cblocaladdress), ::core::mem::transmute_copy(&dwextraportsallowed), ::core::mem::transmute_copy(&ppregistrationresponsemsg), ::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), StartDetection: StartDetection::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProximityDetection as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMReader_Impl: Sized {
    fn Open(&self, pwszurl: &::windows::core::PCWSTR, pcallback: ::core::option::Option<&IWMReaderCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn GetOutputCount(&self) -> ::windows::core::Result<u32>;
    fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(&self, dwoutputnum: u32, poutput: ::core::option::Option<&IWMOutputMediaProps>) -> ::windows::core::Result<()>;
    fn GetOutputFormatCount(&self, dwoutputnumber: u32) -> ::windows::core::Result<u32>;
    fn GetOutputFormat(&self, dwoutputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn Start(&self, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMReader {}
impl IWMReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>() -> IWMReader_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&pwszurl), ::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcoutputs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputProps(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputProps(::core::mem::transmute_copy(&dwoutputnum), ::windows::core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputFormatCount(::core::mem::transmute_copy(&dwoutputnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformats, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputFormat(::core::mem::transmute_copy(&dwoutputnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprops, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute_copy(&cnsstart), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Pause<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pause().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderAccelerator_Impl: Sized {
    fn GetCodecInterface(&self, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Notify(&self, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMReaderAccelerator {}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderAccelerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAccelerator_Impl, const OFFSET: isize>() -> IWMReaderAccelerator_Vtbl {
        unsafe extern "system" fn GetCodecInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCodecInterface(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvcodecinterface)).into()
        }
        unsafe extern "system" fn Notify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAccelerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Notify(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&psubtype)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCodecInterface: GetCodecInterface::<Identity, Impl, OFFSET>,
            Notify: Notify::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAccelerator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderAdvanced_Impl: Sized {
    fn SetUserProvidedClock(&self, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetUserProvidedClock(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn DeliverTime(&self, cnstime: u64) -> ::windows::core::Result<()>;
    fn SetManualStreamSelection(&self, fselection: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetManualStreamSelection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()>;
    fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION>;
    fn SetReceiveSelectionCallbacks(&self, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReceiveSelectionCallbacks(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetReceiveStreamSamples(&self, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReceiveStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAllocateForOutput(&self, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetAllocateForStream(&self, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetStatistics(&self, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::Result<()>;
    fn SetClientInfo(&self, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::Result<()>;
    fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32>;
    fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32>;
    fn NotifyLateDelivery(&self, cnslateness: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMReaderAdvanced {}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderAdvanced_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>() -> IWMReaderAdvanced_Vtbl {
        unsafe extern "system" fn SetUserProvidedClock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserProvidedClock(::core::mem::transmute_copy(&fuserclock)).into()
        }
        unsafe extern "system" fn GetUserProvidedClock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserProvidedClock() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfuserclock, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeliverTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeliverTime(::core::mem::transmute_copy(&cnstime)).into()
        }
        unsafe extern "system" fn SetManualStreamSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetManualStreamSelection(::core::mem::transmute_copy(&fselection)).into()
        }
        unsafe extern "system" fn GetManualStreamSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetManualStreamSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfselection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStreamsSelected(::core::mem::transmute_copy(&cstreamcount), ::core::mem::transmute_copy(&pwstreamnumbers), ::core::mem::transmute_copy(&pselections)).into()
        }
        unsafe extern "system" fn GetStreamSelected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamSelected(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pselection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveSelectionCallbacks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReceiveSelectionCallbacks(::core::mem::transmute_copy(&fgetcallbacks)).into()
        }
        unsafe extern "system" fn GetReceiveSelectionCallbacks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReceiveSelectionCallbacks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfgetcallbacks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReceiveStreamSamples<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReceiveStreamSamples(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&freceivestreamsamples)).into()
        }
        unsafe extern "system" fn GetReceiveStreamSamples<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReceiveStreamSamples(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfreceivestreamsamples, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallocate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllocateForStream(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllocateForStream(::core::mem::transmute_copy(&dwsreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallocate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatistics(::core::mem::transmute_copy(&pstatistics)).into()
        }
        unsafe extern "system" fn SetClientInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientInfo(::core::mem::transmute_copy(&pclientinfo)).into()
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxOutputSampleSize(::core::mem::transmute_copy(&dwoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxStreamSampleSize(::core::mem::transmute_copy(&wstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyLateDelivery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyLateDelivery(::core::mem::transmute_copy(&cnslateness)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMReaderAdvanced as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced2_Impl: Sized + IWMReaderAdvanced_Impl {
    fn SetPlayMode(&self, mode: WMT_PLAY_MODE) -> ::windows::core::Result<()>;
    fn GetPlayMode(&self) -> ::windows::core::Result<WMT_PLAY_MODE>;
    fn GetBufferProgress(&self, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::Result<()>;
    fn GetDownloadProgress(&self, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::Result<()>;
    fn GetSaveAsProgress(&self) -> ::windows::core::Result<u32>;
    fn SaveFileAs(&self, pwszfilename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetProtocolName(&self, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::Result<()>;
    fn StartAtMarker(&self, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetOutputSetting(&self, dwoutputnum: u32, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetOutputSetting(&self, dwoutputnum: u32, pszname: &::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn Preroll(&self, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::Result<()>;
    fn SetLogClientID(&self, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLogClientID(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn StopBuffering(&self) -> ::windows::core::Result<()>;
    fn OpenStream(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>, pcallback: ::core::option::Option<&IWMReaderCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMReaderAdvanced2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>() -> IWMReaderAdvanced2_Vtbl {
        unsafe extern "system" fn SetPlayMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlayMode(::core::mem::transmute_copy(&mode)).into()
        }
        unsafe extern "system" fn GetPlayMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPlayMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferProgress(::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pcnsbuffering)).into()
        }
        unsafe extern "system" fn GetDownloadProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDownloadProgress(::core::mem::transmute_copy(&pdwpercent), ::core::mem::transmute_copy(&pqwbytesdownloaded), ::core::mem::transmute_copy(&pcnsdownload)).into()
        }
        unsafe extern "system" fn GetSaveAsProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSaveAsProgress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpercent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveFileAs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveFileAs(::core::mem::transmute(&pwszfilename)).into()
        }
        unsafe extern "system" fn GetProtocolName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProtocolName(::core::mem::transmute_copy(&pwszprotocol), ::core::mem::transmute_copy(&pcchprotocol)).into()
        }
        unsafe extern "system" fn StartAtMarker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartAtMarker(::core::mem::transmute_copy(&wmarkerindex), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn GetOutputSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetOutputSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn Preroll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Preroll(::core::mem::transmute_copy(&cnsstart), ::core::mem::transmute_copy(&cnsduration), ::core::mem::transmute_copy(&frate)).into()
        }
        unsafe extern "system" fn SetLogClientID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogClientID(::core::mem::transmute_copy(&flogclientid)).into()
        }
        unsafe extern "system" fn GetLogClientID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLogClientID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflogclientid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopBuffering<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopBuffering().into()
        }
        unsafe extern "system" fn OpenStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenStream(::windows::core::from_raw_borrowed(&pstream), ::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: IWMReaderAdvanced_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMReaderAdvanced2 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced3_Impl: Sized + IWMReaderAdvanced2_Impl {
    fn StopNetStreaming(&self) -> ::windows::core::Result<()>;
    fn StartAtPosition(&self, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMReaderAdvanced3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced3_Impl, const OFFSET: isize>() -> IWMReaderAdvanced3_Vtbl {
        unsafe extern "system" fn StopNetStreaming<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopNetStreaming().into()
        }
        unsafe extern "system" fn StartAtPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartAtPosition(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pvoffsetstart), ::core::mem::transmute_copy(&pvduration), ::core::mem::transmute_copy(&dwoffsetformat), ::core::mem::transmute_copy(&frate), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: IWMReaderAdvanced2_Vtbl::new::<Identity, Impl, OFFSET>(),
            StopNetStreaming: StopNetStreaming::<Identity, Impl, OFFSET>,
            StartAtPosition: StartAtPosition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced3 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced4_Impl: Sized + IWMReaderAdvanced3_Impl {
    fn GetLanguageCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u16>;
    fn GetLanguage(&self, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()>;
    fn GetMaxSpeedFactor(&self) -> ::windows::core::Result<f64>;
    fn IsUsingFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn AddLogParam(&self, wsznamespace: &::windows::core::PCWSTR, wszname: &::windows::core::PCWSTR, wszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SendLogParams(&self) -> ::windows::core::Result<()>;
    fn CanSaveFileAs(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CancelSaveFileAs(&self) -> ::windows::core::Result<()>;
    fn GetURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMReaderAdvanced4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>() -> IWMReaderAdvanced4_Vtbl {
        unsafe extern "system" fn GetLanguageCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLanguageCount(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwlanguagecount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLanguage(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&wlanguage), ::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn GetMaxSpeedFactor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxSpeedFactor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdblfactor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUsingFastCache<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUsingFastCache() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfusingfastcache, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddLogParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznamespace: ::windows::core::PCWSTR, wszname: ::windows::core::PCWSTR, wszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLogParam(::core::mem::transmute(&wsznamespace), ::core::mem::transmute(&wszname), ::core::mem::transmute(&wszvalue)).into()
        }
        unsafe extern "system" fn SendLogParams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendLogParams().into()
        }
        unsafe extern "system" fn CanSaveFileAs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanSaveFileAs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcansave, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelSaveFileAs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelSaveFileAs().into()
        }
        unsafe extern "system" fn GetURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetURL(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into()
        }
        Self {
            base__: IWMReaderAdvanced3_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMReaderAdvanced4 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced2 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced5_Impl: Sized + IWMReaderAdvanced4_Impl {
    fn SetPlayerHook(&self, dwoutputnum: u32, phook: ::core::option::Option<&IWMPlayerHook>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMReaderAdvanced5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced5_Impl, const OFFSET: isize>() -> IWMReaderAdvanced5_Vtbl {
        unsafe extern "system" fn SetPlayerHook<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlayerHook(::core::mem::transmute_copy(&dwoutputnum), ::windows::core::from_raw_borrowed(&phook)).into()
        }
        Self { base__: IWMReaderAdvanced4_Vtbl::new::<Identity, Impl, OFFSET>(), SetPlayerHook: SetPlayerHook::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced5 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced2 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced3 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced6_Impl: Sized + IWMReaderAdvanced5_Impl {
    fn SetProtectStreamSamples(&self, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMReaderAdvanced6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced6_Impl, const OFFSET: isize>() -> IWMReaderAdvanced6_Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAdvanced6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProtectStreamSamples(::core::mem::transmute_copy(&pbcertificate), ::core::mem::transmute_copy(&cbcertificate), ::core::mem::transmute_copy(&dwcertificatetype), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pbinitializationvector), ::core::mem::transmute_copy(&pcbinitializationvector)).into()
        }
        Self { base__: IWMReaderAdvanced5_Vtbl::new::<Identity, Impl, OFFSET>(), SetProtectStreamSamples: SetProtectStreamSamples::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced6 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced2 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced3 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced4 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderAdvanced5 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMReaderAllocatorEx_Impl: Sized {
    fn AllocateForStreamEx(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForOutputEx(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMReaderAllocatorEx {}
impl IWMReaderAllocatorEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAllocatorEx_Impl, const OFFSET: isize>() -> IWMReaderAllocatorEx_Vtbl {
        unsafe extern "system" fn AllocateForStreamEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAllocatorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllocateForStreamEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForOutputEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderAllocatorEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllocateForOutputEx(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocateForStreamEx: AllocateForStreamEx::<Identity, Impl, OFFSET>,
            AllocateForOutputEx: AllocateForOutputEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAllocatorEx as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMReaderCallback_Impl: Sized + IWMStatusCallback_Impl {
    fn OnSample(&self, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMReaderCallback {}
impl IWMReaderCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderCallback_Impl, const OFFSET: isize>() -> IWMReaderCallback_Vtbl {
        unsafe extern "system" fn OnSample<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSample(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&psample), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base__: IWMStatusCallback_Vtbl::new::<Identity, Impl, OFFSET>(), OnSample: OnSample::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderCallback as ::windows::core::ComInterface>::IID || iid == &<IWMStatusCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderCallbackAdvanced_Impl: Sized {
    fn OnStreamSample(&self, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OnTime(&self, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OnStreamSelection(&self, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OnOutputPropsChanged(&self, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForStream(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForOutput(&self, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMReaderCallbackAdvanced {}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderCallbackAdvanced_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>() -> IWMReaderCallbackAdvanced_Vtbl {
        unsafe extern "system" fn OnStreamSample<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStreamSample(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&psample), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTime(::core::mem::transmute_copy(&cnscurrenttime), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnStreamSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStreamSelection(::core::mem::transmute_copy(&wstreamcount), ::core::mem::transmute_copy(&pstreamnumbers), ::core::mem::transmute_copy(&pselections), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn OnOutputPropsChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnOutputPropsChanged(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&pmediatype), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllocateForStream(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderCallbackAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllocateForOutput(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStreamSample: OnStreamSample::<Identity, Impl, OFFSET>,
            OnTime: OnTime::<Identity, Impl, OFFSET>,
            OnStreamSelection: OnStreamSelection::<Identity, Impl, OFFSET>,
            OnOutputPropsChanged: OnOutputPropsChanged::<Identity, Impl, OFFSET>,
            AllocateForStream: AllocateForStream::<Identity, Impl, OFFSET>,
            AllocateForOutput: AllocateForOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderCallbackAdvanced as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderNetworkConfig_Impl: Sized {
    fn GetBufferingTime(&self) -> ::windows::core::Result<u64>;
    fn SetBufferingTime(&self, cnsbufferingtime: u64) -> ::windows::core::Result<()>;
    fn GetUDPPortRanges(&self, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::Result<()>;
    fn SetUDPPortRanges(&self, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::Result<()>;
    fn GetProxySettings(&self, pwszprotocol: &::windows::core::PCWSTR) -> ::windows::core::Result<WMT_PROXY_SETTINGS>;
    fn SetProxySettings(&self, pwszprotocol: &::windows::core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::Result<()>;
    fn GetProxyHostName(&self, pwszprotocol: &::windows::core::PCWSTR, pwszhostname: ::windows::core::PWSTR, pcchhostname: *mut u32) -> ::windows::core::Result<()>;
    fn SetProxyHostName(&self, pwszprotocol: &::windows::core::PCWSTR, pwszhostname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetProxyPort(&self, pwszprotocol: &::windows::core::PCWSTR) -> ::windows::core::Result<u32>;
    fn SetProxyPort(&self, pwszprotocol: &::windows::core::PCWSTR, dwport: u32) -> ::windows::core::Result<()>;
    fn GetProxyExceptionList(&self, pwszprotocol: &::windows::core::PCWSTR, pwszexceptionlist: ::windows::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::Result<()>;
    fn SetProxyExceptionList(&self, pwszprotocol: &::windows::core::PCWSTR, pwszexceptionlist: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetProxyBypassForLocal(&self, pwszprotocol: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetProxyBypassForLocal(&self, pwszprotocol: &::windows::core::PCWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetForceRerunAutoProxyDetection(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetForceRerunAutoProxyDetection(&self, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableMulticast(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableMulticast(&self, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableHTTP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableHTTP(&self, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableUDP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableUDP(&self, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableTCP(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableTCP(&self, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ResetProtocolRollover(&self) -> ::windows::core::Result<()>;
    fn GetConnectionBandwidth(&self) -> ::windows::core::Result<u32>;
    fn SetConnectionBandwidth(&self, dwconnectionbandwidth: u32) -> ::windows::core::Result<()>;
    fn GetNumProtocolsSupported(&self) -> ::windows::core::Result<u32>;
    fn GetSupportedProtocolName(&self, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::Result<()>;
    fn AddLoggingUrl(&self, pwszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetLoggingUrl(&self, dwindex: u32, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()>;
    fn GetLoggingUrlCount(&self) -> ::windows::core::Result<u32>;
    fn ResetLoggingUrlList(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMReaderNetworkConfig {}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderNetworkConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>() -> IWMReaderNetworkConfig_Vtbl {
        unsafe extern "system" fn GetBufferingTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBufferingTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsbufferingtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferingTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferingTime(::core::mem::transmute_copy(&cnsbufferingtime)).into()
        }
        unsafe extern "system" fn GetUDPPortRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUDPPortRanges(::core::mem::transmute_copy(&prangearray), ::core::mem::transmute_copy(&pcranges)).into()
        }
        unsafe extern "system" fn SetUDPPortRanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUDPPortRanges(::core::mem::transmute_copy(&prangearray), ::core::mem::transmute_copy(&cranges)).into()
        }
        unsafe extern "system" fn GetProxySettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProxySettings(::core::mem::transmute(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproxysetting, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxySettings<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProxySettings(::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&proxysetting)).into()
        }
        unsafe extern "system" fn GetProxyHostName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszhostname: ::windows::core::PWSTR, pcchhostname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProxyHostName(::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&pwszhostname), ::core::mem::transmute_copy(&pcchhostname)).into()
        }
        unsafe extern "system" fn SetProxyHostName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszhostname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProxyHostName(::core::mem::transmute(&pwszprotocol), ::core::mem::transmute(&pwszhostname)).into()
        }
        unsafe extern "system" fn GetProxyPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProxyPort(::core::mem::transmute(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, dwport: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProxyPort(::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&dwport)).into()
        }
        unsafe extern "system" fn GetProxyExceptionList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszexceptionlist: ::windows::core::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProxyExceptionList(::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&pwszexceptionlist), ::core::mem::transmute_copy(&pcchexceptionlist)).into()
        }
        unsafe extern "system" fn SetProxyExceptionList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pwszexceptionlist: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProxyExceptionList(::core::mem::transmute(&pwszprotocol), ::core::mem::transmute(&pwszexceptionlist)).into()
        }
        unsafe extern "system" fn GetProxyBypassForLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProxyBypassForLocal(::core::mem::transmute(&pwszprotocol)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfbypassforlocal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyBypassForLocal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: ::windows::core::PCWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProxyBypassForLocal(::core::mem::transmute(&pwszprotocol), ::core::mem::transmute_copy(&fbypassforlocal)).into()
        }
        unsafe extern "system" fn GetForceRerunAutoProxyDetection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetForceRerunAutoProxyDetection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforcererundetection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceRerunAutoProxyDetection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetForceRerunAutoProxyDetection(::core::mem::transmute_copy(&fforcererundetection)).into()
        }
        unsafe extern "system" fn GetEnableMulticast<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnableMulticast() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablemulticast, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableMulticast<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableMulticast(::core::mem::transmute_copy(&fenablemulticast)).into()
        }
        unsafe extern "system" fn GetEnableHTTP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnableHTTP() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablehttp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableHTTP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableHTTP(::core::mem::transmute_copy(&fenablehttp)).into()
        }
        unsafe extern "system" fn GetEnableUDP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnableUDP() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenableudp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableUDP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableUDP(::core::mem::transmute_copy(&fenableudp)).into()
        }
        unsafe extern "system" fn GetEnableTCP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnableTCP() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabletcp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableTCP<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableTCP(::core::mem::transmute_copy(&fenabletcp)).into()
        }
        unsafe extern "system" fn ResetProtocolRollover<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetProtocolRollover().into()
        }
        unsafe extern "system" fn GetConnectionBandwidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectionBandwidth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnectionbandwidth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConnectionBandwidth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConnectionBandwidth(::core::mem::transmute_copy(&dwconnectionbandwidth)).into()
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNumProtocolsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcprotocols, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedProtocolName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: ::windows::core::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSupportedProtocolName(::core::mem::transmute_copy(&dwprotocolnum), ::core::mem::transmute_copy(&pwszprotocolname), ::core::mem::transmute_copy(&pcchprotocolname)).into()
        }
        unsafe extern "system" fn AddLoggingUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddLoggingUrl(::core::mem::transmute(&pwszurl)).into()
        }
        unsafe extern "system" fn GetLoggingUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLoggingUrl(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into()
        }
        unsafe extern "system" fn GetLoggingUrlCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLoggingUrlCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwurlcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetLoggingUrlList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetLoggingUrlList().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMReaderNetworkConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderNetworkConfig2_Impl: Sized + IWMReaderNetworkConfig_Impl {
    fn GetEnableContentCaching(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableContentCaching(&self, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableFastCache(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableFastCache(&self, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAcceleratedStreamingDuration(&self) -> ::windows::core::Result<u64>;
    fn SetAcceleratedStreamingDuration(&self, cnsaccelduration: u64) -> ::windows::core::Result<()>;
    fn GetAutoReconnectLimit(&self) -> ::windows::core::Result<u32>;
    fn SetAutoReconnectLimit(&self, dwautoreconnectlimit: u32) -> ::windows::core::Result<()>;
    fn GetEnableResends(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableResends(&self, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetEnableThinning(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnableThinning(&self, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMaxNetPacketSize(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMReaderNetworkConfig2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderNetworkConfig2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>() -> IWMReaderNetworkConfig2_Vtbl {
        unsafe extern "system" fn GetEnableContentCaching<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnableContentCaching() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablecontentcaching, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableContentCaching<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableContentCaching(::core::mem::transmute_copy(&fenablecontentcaching)).into()
        }
        unsafe extern "system" fn GetEnableFastCache<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnableFastCache() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablefastcache, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableFastCache<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableFastCache(::core::mem::transmute_copy(&fenablefastcache)).into()
        }
        unsafe extern "system" fn GetAcceleratedStreamingDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAcceleratedStreamingDuration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsaccelduration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAcceleratedStreamingDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAcceleratedStreamingDuration(::core::mem::transmute_copy(&cnsaccelduration)).into()
        }
        unsafe extern "system" fn GetAutoReconnectLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAutoReconnectLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwautoreconnectlimit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAutoReconnectLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoReconnectLimit(::core::mem::transmute_copy(&dwautoreconnectlimit)).into()
        }
        unsafe extern "system" fn GetEnableResends<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnableResends() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenableresends, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableResends<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableResends(::core::mem::transmute_copy(&fenableresends)).into()
        }
        unsafe extern "system" fn GetEnableThinning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnableThinning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenablethinning, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnableThinning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnableThinning(::core::mem::transmute_copy(&fenablethinning)).into()
        }
        unsafe extern "system" fn GetMaxNetPacketSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderNetworkConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxNetPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxnetpacketsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMReaderNetworkConfig_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMReaderNetworkConfig2 as ::windows::core::ComInterface>::IID || iid == &<IWMReaderNetworkConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMReaderPlaylistBurn_Impl: Sized {
    fn InitPlaylistBurn(&self, cfiles: u32, ppwszfilenames: *const ::windows::core::PCWSTR, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetInitResults(&self, cfiles: u32) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn EndPlaylistBurn(&self, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMReaderPlaylistBurn {}
impl IWMReaderPlaylistBurn_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>() -> IWMReaderPlaylistBurn_Vtbl {
        unsafe extern "system" fn InitPlaylistBurn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const ::windows::core::PCWSTR, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitPlaylistBurn(::core::mem::transmute_copy(&cfiles), ::core::mem::transmute_copy(&ppwszfilenames), ::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn GetInitResults<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInitResults(::core::mem::transmute_copy(&cfiles)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phrstati, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn EndPlaylistBurn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderPlaylistBurn_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPlaylistBurn(::core::mem::transmute_copy(&hrburnresult)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitPlaylistBurn: InitPlaylistBurn::<Identity, Impl, OFFSET>,
            GetInitResults: GetInitResults::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            EndPlaylistBurn: EndPlaylistBurn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderPlaylistBurn as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMReaderStreamClock_Impl: Sized {
    fn GetTime(&self, pcnsnow: *const u64) -> ::windows::core::Result<()>;
    fn SetTimer(&self, cnswhen: u64, pvparam: *const ::core::ffi::c_void) -> ::windows::core::Result<u32>;
    fn KillTimer(&self, dwtimerid: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMReaderStreamClock {}
impl IWMReaderStreamClock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderStreamClock_Impl, const OFFSET: isize>() -> IWMReaderStreamClock_Vtbl {
        unsafe extern "system" fn GetTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderStreamClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTime(::core::mem::transmute_copy(&pcnsnow)).into()
        }
        unsafe extern "system" fn SetTimer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderStreamClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetTimer(::core::mem::transmute_copy(&cnswhen), ::core::mem::transmute_copy(&pvparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtimerid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KillTimer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderStreamClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KillTimer(::core::mem::transmute_copy(&dwtimerid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTime: GetTime::<Identity, Impl, OFFSET>,
            SetTimer: SetTimer::<Identity, Impl, OFFSET>,
            KillTimer: KillTimer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderStreamClock as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMReaderTimecode_Impl: Sized {
    fn GetTimecodeRangeCount(&self, wstreamnum: u16) -> ::windows::core::Result<u16>;
    fn GetTimecodeRangeBounds(&self, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMReaderTimecode {}
impl IWMReaderTimecode_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderTimecode_Impl, const OFFSET: isize>() -> IWMReaderTimecode_Vtbl {
        unsafe extern "system" fn GetTimecodeRangeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderTimecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTimecodeRangeCount(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwrangecount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimecodeRangeBounds<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderTimecode_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTimecodeRangeBounds(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&wrangenum), ::core::mem::transmute_copy(&pstarttimecode), ::core::mem::transmute_copy(&pendtimecode)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTimecodeRangeCount: GetTimecodeRangeCount::<Identity, Impl, OFFSET>,
            GetTimecodeRangeBounds: GetTimecodeRangeBounds::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderTimecode as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMReaderTypeNegotiation_Impl: Sized {
    fn TryOutputProps(&self, dwoutputnum: u32, poutput: ::core::option::Option<&IWMOutputMediaProps>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMReaderTypeNegotiation {}
impl IWMReaderTypeNegotiation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderTypeNegotiation_Impl, const OFFSET: isize>() -> IWMReaderTypeNegotiation_Vtbl {
        unsafe extern "system" fn TryOutputProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMReaderTypeNegotiation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TryOutputProps(::core::mem::transmute_copy(&dwoutputnum), ::windows::core::from_raw_borrowed(&poutput)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TryOutputProps: TryOutputProps::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderTypeNegotiation as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMRegisterCallback_Impl: Sized {
    fn Advise(&self, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Unadvise(&self, pcallback: ::core::option::Option<&IWMStatusCallback>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMRegisterCallback {}
impl IWMRegisterCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisterCallback_Impl, const OFFSET: isize>() -> IWMRegisterCallback_Vtbl {
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisterCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Advise(::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisterCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMRegisterCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMRegisteredDevice_Impl: Sized {
    fn GetDeviceSerialNumber(&self) -> ::windows::core::Result<DRM_VAL16>;
    fn GetDeviceCertificate(&self) -> ::windows::core::Result<INSSBuffer>;
    fn GetDeviceType(&self) -> ::windows::core::Result<u32>;
    fn GetAttributeCount(&self) -> ::windows::core::Result<u32>;
    fn GetAttributeByIndex(&self, dwindex: u32, pbstrname: *mut ::windows::core::BSTR, pbstrvalue: *mut ::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetAttributeByName(&self, bstrname: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetAttributeByName(&self, bstrname: &::windows::core::BSTR, bstrvalue: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Approve(&self, fapprove: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsValid(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsApproved(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsWmdrmCompliant(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsOpened(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn Open(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMRegisteredDevice {}
#[cfg(feature = "Win32_Foundation")]
impl IWMRegisteredDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>() -> IWMRegisteredDevice_Vtbl {
        unsafe extern "system" fn GetDeviceSerialNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceSerialNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pserialnumber, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertificate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcertificate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcattributes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeByIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAttributeByIndex(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrvalue)).into()
        }
        unsafe extern "system" fn GetAttributeByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeByName(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAttributeByName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrvalue: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAttributeByName(::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrvalue)).into()
        }
        unsafe extern "system" fn Approve<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Approve(::core::mem::transmute_copy(&fapprove)).into()
        }
        unsafe extern "system" fn IsValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfvalid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApproved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsApproved() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfapproved, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWmdrmCompliant<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWmdrmCompliant() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcompliant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsOpened<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOpened() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfopened, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMRegisteredDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMRegisteredDevice as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMSBufferAllocator_Impl: Sized {
    fn AllocateBuffer(&self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer>;
    fn AllocatePageSizeBuffer(&self, dwmaxbuffersize: u32) -> ::windows::core::Result<INSSBuffer>;
}
impl ::windows::core::RuntimeName for IWMSBufferAllocator {}
impl IWMSBufferAllocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSBufferAllocator_Impl, const OFFSET: isize>() -> IWMSBufferAllocator_Vtbl {
        unsafe extern "system" fn AllocateBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSBufferAllocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllocateBuffer(::core::mem::transmute_copy(&dwmaxbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocatePageSizeBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSBufferAllocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllocatePageSizeBuffer(::core::mem::transmute_copy(&dwmaxbuffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AllocateBuffer: AllocateBuffer::<Identity, Impl, OFFSET>,
            AllocatePageSizeBuffer: AllocatePageSizeBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSBufferAllocator as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource_Impl: Sized {
    fn Initialize(&self, psharednamespace: ::core::option::Option<&::windows::core::IUnknown>, pnamespacenode: ::core::option::Option<&::windows::core::IUnknown>, pnetsourcecreator: ::core::option::Option<&INSNetSourceCreator>, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetNetSourceCreator(&self) -> ::windows::core::Result<INSNetSourceCreator>;
    fn SetCredentials(&self, bstrrealm: &::windows::core::BSTR, bstrname: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCredentials(&self, bstrrealm: &::windows::core::BSTR, pbstrname: *mut ::windows::core::BSTR, pbstrpassword: *mut ::windows::core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DeleteCredentials(&self, bstrrealm: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetCredentialFlags(&self) -> ::windows::core::Result<u32>;
    fn SetCredentialFlags(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn FindProxyForURL(&self, bstrprotocol: &::windows::core::BSTR, bstrhost: &::windows::core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows::core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()>;
    fn RegisterProxyFailure(&self, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::Result<()>;
    fn ShutdownProxyContext(&self, dwproxycontext: u32) -> ::windows::core::Result<()>;
    fn IsUsingIE(&self, dwproxycontext: u32) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMSInternalAdminNetSource {}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>() -> IWMSInternalAdminNetSource_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: *mut ::core::ffi::c_void, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&psharednamespace), ::windows::core::from_raw_borrowed(&pnamespacenode), ::windows::core::from_raw_borrowed(&pnetsourcecreator), ::core::mem::transmute_copy(&fembeddedinserver)).into()
        }
        unsafe extern "system" fn GetNetSourceCreator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetSourceCreator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetsourcecreator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCredentials(::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood)).into()
        }
        unsafe extern "system" fn GetCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCredentials(::core::mem::transmute(&bstrrealm), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        unsafe extern "system" fn DeleteCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteCredentials(::core::mem::transmute(&bstrrealm)).into()
        }
        unsafe extern "system" fn GetCredentialFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCredentialFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpdwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCredentialFlags(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn FindProxyForURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrhost: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindProxyForURL(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&bstrhost), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pdwproxycontext)).into()
        }
        unsafe extern "system" fn RegisterProxyFailure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterProxyFailure(::core::mem::transmute_copy(&hrparam), ::core::mem::transmute_copy(&dwproxycontext)).into()
        }
        unsafe extern "system" fn ShutdownProxyContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownProxyContext(::core::mem::transmute_copy(&dwproxycontext)).into()
        }
        unsafe extern "system" fn IsUsingIE<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUsingIE(::core::mem::transmute_copy(&dwproxycontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisusingie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMSInternalAdminNetSource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource2_Impl: Sized {
    fn SetCredentialsEx(&self, bstrrealm: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCredentialsEx(&self, bstrrealm: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::windows::core::BSTR, pbstrpassword: *mut ::windows::core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DeleteCredentialsEx(&self, bstrrealm: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, fproxy: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FindProxyForURLEx(&self, bstrprotocol: &::windows::core::BSTR, bstrhost: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows::core::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMSInternalAdminNetSource2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>() -> IWMSInternalAdminNetSource2_Vtbl {
        unsafe extern "system" fn SetCredentialsEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCredentialsEx(::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood)).into()
        }
        unsafe extern "system" fn GetCredentialsEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCredentialsEx(::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&pdwurlpolicy), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        unsafe extern "system" fn DeleteCredentialsEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteCredentialsEx(::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy)).into()
        }
        unsafe extern "system" fn FindProxyForURLEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrhost: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindProxyForURLEx(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&bstrhost), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pdwproxycontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCredentialsEx: SetCredentialsEx::<Identity, Impl, OFFSET>,
            GetCredentialsEx: GetCredentialsEx::<Identity, Impl, OFFSET>,
            DeleteCredentialsEx: DeleteCredentialsEx::<Identity, Impl, OFFSET>,
            FindProxyForURLEx: FindProxyForURLEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSInternalAdminNetSource2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSInternalAdminNetSource3_Impl: Sized + IWMSInternalAdminNetSource2_Impl {
    fn GetNetSourceCreator2(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn FindProxyForURLEx2(&self, bstrprotocol: &::windows::core::BSTR, bstrhost: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::windows::core::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::Result<()>;
    fn RegisterProxyFailure2(&self, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::Result<()>;
    fn ShutdownProxyContext2(&self, qwproxycontext: u64) -> ::windows::core::Result<()>;
    fn IsUsingIE2(&self, qwproxycontext: u64) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetCredentialsEx2(&self, bstrrealm: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, fproxy: super::super::Foundation::BOOL, bstrname: &::windows::core::BSTR, bstrpassword: &::windows::core::BSTR, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCredentialsEx2(&self, bstrrealm: &::windows::core::BSTR, bstrurl: &::windows::core::BSTR, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::windows::core::BSTR, pbstrpassword: *mut ::windows::core::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMSInternalAdminNetSource3 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>() -> IWMSInternalAdminNetSource3_Vtbl {
        unsafe extern "system" fn GetNetSourceCreator2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetSourceCreator2() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetsourcecreator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindProxyForURLEx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrhost: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindProxyForURLEx2(::core::mem::transmute(&bstrprotocol), ::core::mem::transmute(&bstrhost), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&pfproxyenabled), ::core::mem::transmute_copy(&pbstrproxyserver), ::core::mem::transmute_copy(&pdwproxyport), ::core::mem::transmute_copy(&pqwproxycontext)).into()
        }
        unsafe extern "system" fn RegisterProxyFailure2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterProxyFailure2(::core::mem::transmute_copy(&hrparam), ::core::mem::transmute_copy(&qwproxycontext)).into()
        }
        unsafe extern "system" fn ShutdownProxyContext2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownProxyContext2(::core::mem::transmute_copy(&qwproxycontext)).into()
        }
        unsafe extern "system" fn IsUsingIE2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUsingIE2(::core::mem::transmute_copy(&qwproxycontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisusingie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCredentialsEx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows::core::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCredentialsEx2(::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute(&bstrname), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&fpersist), ::core::mem::transmute_copy(&fconfirmedgood), ::core::mem::transmute_copy(&fcleartextauthentication)).into()
        }
        unsafe extern "system" fn GetCredentialsEx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSInternalAdminNetSource3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrurl: ::std::mem::MaybeUninit<::windows::core::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrpassword: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCredentialsEx2(::core::mem::transmute(&bstrrealm), ::core::mem::transmute(&bstrurl), ::core::mem::transmute_copy(&fproxy), ::core::mem::transmute_copy(&fcleartextauthentication), ::core::mem::transmute_copy(&pdwurlpolicy), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrpassword), ::core::mem::transmute_copy(&pfconfirmedgood)).into()
        }
        Self {
            base__: IWMSInternalAdminNetSource2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMSInternalAdminNetSource3 as ::windows::core::ComInterface>::IID || iid == &<IWMSInternalAdminNetSource2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMSecureChannel_Impl: Sized + IWMAuthorizer_Impl {
    fn WMSC_AddCertificate(&self, pcert: ::core::option::Option<&IWMAuthorizer>) -> ::windows::core::Result<()>;
    fn WMSC_AddSignature(&self, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::Result<()>;
    fn WMSC_Connect(&self, potherside: ::core::option::Option<&IWMSecureChannel>) -> ::windows::core::Result<()>;
    fn WMSC_IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn WMSC_Disconnect(&self) -> ::windows::core::Result<()>;
    fn WMSC_GetValidCertificate(&self, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::Result<()>;
    fn WMSC_Encrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()>;
    fn WMSC_Decrypt(&self, pbdata: *const u8, cbdata: u32) -> ::windows::core::Result<()>;
    fn WMSC_Lock(&self) -> ::windows::core::Result<()>;
    fn WMSC_Unlock(&self) -> ::windows::core::Result<()>;
    fn WMSC_SetSharedData(&self, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMSecureChannel {}
#[cfg(feature = "Win32_Foundation")]
impl IWMSecureChannel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>() -> IWMSecureChannel_Vtbl {
        unsafe extern "system" fn WMSC_AddCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcert: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_AddCertificate(::windows::core::from_raw_borrowed(&pcert)).into()
        }
        unsafe extern "system" fn WMSC_AddSignature<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_AddSignature(::core::mem::transmute_copy(&pbcertsig), ::core::mem::transmute_copy(&cbcertsig)).into()
        }
        unsafe extern "system" fn WMSC_Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, potherside: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_Connect(::windows::core::from_raw_borrowed(&potherside)).into()
        }
        unsafe extern "system" fn WMSC_IsConnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WMSC_IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisconnected, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WMSC_Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_Disconnect().into()
        }
        unsafe extern "system" fn WMSC_GetValidCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_GetValidCertificate(::core::mem::transmute_copy(&ppbcertificate), ::core::mem::transmute_copy(&pdwsignature)).into()
        }
        unsafe extern "system" fn WMSC_Encrypt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_Encrypt(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn WMSC_Decrypt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_Decrypt(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbdata)).into()
        }
        unsafe extern "system" fn WMSC_Lock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_Lock().into()
        }
        unsafe extern "system" fn WMSC_Unlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_Unlock().into()
        }
        unsafe extern "system" fn WMSC_SetSharedData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSecureChannel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WMSC_SetSharedData(::core::mem::transmute_copy(&dwcertindex), ::core::mem::transmute_copy(&pbshareddata)).into()
        }
        Self {
            base__: IWMAuthorizer_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMSecureChannel as ::windows::core::ComInterface>::IID || iid == &<IWMAuthorizer as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMStatusCallback_Impl: Sized {
    fn OnStatus(&self, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMStatusCallback {}
impl IWMStatusCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStatusCallback_Impl, const OFFSET: isize>() -> IWMStatusCallback_Vtbl {
        unsafe extern "system" fn OnStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStatus(::core::mem::transmute_copy(&status), ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&dwtype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnStatus: OnStatus::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStatusCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMStreamConfig_Impl: Sized {
    fn GetStreamType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetStreamNumber(&self) -> ::windows::core::Result<u16>;
    fn SetStreamNumber(&self, wstreamnum: u16) -> ::windows::core::Result<()>;
    fn GetStreamName(&self, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::Result<()>;
    fn SetStreamName(&self, pwszstreamname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetConnectionName(&self, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::Result<()>;
    fn SetConnectionName(&self, pwszinputname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetBitrate(&self, pdwbitrate: u32) -> ::windows::core::Result<()>;
    fn GetBufferWindow(&self) -> ::windows::core::Result<u32>;
    fn SetBufferWindow(&self, msbufferwindow: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMStreamConfig {}
impl IWMStreamConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>() -> IWMStreamConfig_Vtbl {
        unsafe extern "system" fn GetStreamType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidstreamtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwstreamnum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStreamNumber<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStreamNumber(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn GetStreamName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows::core::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreamName(::core::mem::transmute_copy(&pwszstreamname), ::core::mem::transmute_copy(&pcchstreamname)).into()
        }
        unsafe extern "system" fn SetStreamName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszstreamname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStreamName(::core::mem::transmute(&pwszstreamname)).into()
        }
        unsafe extern "system" fn GetConnectionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinputname: ::windows::core::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConnectionName(::core::mem::transmute_copy(&pwszinputname), ::core::mem::transmute_copy(&pcchinputname)).into()
        }
        unsafe extern "system" fn SetConnectionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinputname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConnectionName(::core::mem::transmute(&pwszinputname)).into()
        }
        unsafe extern "system" fn GetBitrate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBitrate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwbitrate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBitrate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBitrate(::core::mem::transmute_copy(&pdwbitrate)).into()
        }
        unsafe extern "system" fn GetBufferWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBufferWindow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmsbufferwindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferWindow(::core::mem::transmute_copy(&msbufferwindow)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMStreamConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMStreamConfig2_Impl: Sized + IWMStreamConfig_Impl {
    fn GetTransportType(&self) -> ::windows::core::Result<WMT_TRANSPORT_TYPE>;
    fn SetTransportType(&self, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::Result<()>;
    fn AddDataUnitExtension(&self, guidextensionsystemid: &::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::Result<()>;
    fn GetDataUnitExtensionCount(&self) -> ::windows::core::Result<u16>;
    fn GetDataUnitExtension(&self, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::Result<()>;
    fn RemoveAllDataUnitExtensions(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMStreamConfig2 {}
impl IWMStreamConfig2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>() -> IWMStreamConfig2_Vtbl {
        unsafe extern "system" fn GetTransportType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransportType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pntransporttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransportType(::core::mem::transmute_copy(&ntransporttype)).into()
        }
        unsafe extern "system" fn AddDataUnitExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDataUnitExtension(::core::mem::transmute(&guidextensionsystemid), ::core::mem::transmute_copy(&cbextensiondatasize), ::core::mem::transmute_copy(&pbextensionsysteminfo), ::core::mem::transmute_copy(&cbextensionsysteminfo)).into()
        }
        unsafe extern "system" fn GetDataUnitExtensionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataUnitExtensionCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcdataunitextensions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataUnitExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataUnitExtension(::core::mem::transmute_copy(&wdataunitextensionnumber), ::core::mem::transmute_copy(&pguidextensionsystemid), ::core::mem::transmute_copy(&pcbextensiondatasize), ::core::mem::transmute_copy(&pbextensionsysteminfo), ::core::mem::transmute_copy(&pcbextensionsysteminfo)).into()
        }
        unsafe extern "system" fn RemoveAllDataUnitExtensions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAllDataUnitExtensions().into()
        }
        Self {
            base__: IWMStreamConfig_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTransportType: GetTransportType::<Identity, Impl, OFFSET>,
            SetTransportType: SetTransportType::<Identity, Impl, OFFSET>,
            AddDataUnitExtension: AddDataUnitExtension::<Identity, Impl, OFFSET>,
            GetDataUnitExtensionCount: GetDataUnitExtensionCount::<Identity, Impl, OFFSET>,
            GetDataUnitExtension: GetDataUnitExtension::<Identity, Impl, OFFSET>,
            RemoveAllDataUnitExtensions: RemoveAllDataUnitExtensions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamConfig2 as ::windows::core::ComInterface>::IID || iid == &<IWMStreamConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMStreamConfig3_Impl: Sized + IWMStreamConfig2_Impl {
    fn GetLanguage(&self, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::Result<()>;
    fn SetLanguage(&self, pwszlanguagestring: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMStreamConfig3 {}
impl IWMStreamConfig3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig3_Impl, const OFFSET: isize>() -> IWMStreamConfig3_Vtbl {
        unsafe extern "system" fn GetLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows::core::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLanguage(::core::mem::transmute_copy(&pwszlanguagestring), ::core::mem::transmute_copy(&pcchlanguagestringlength)).into()
        }
        unsafe extern "system" fn SetLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamConfig3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLanguage(::core::mem::transmute(&pwszlanguagestring)).into()
        }
        Self {
            base__: IWMStreamConfig2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetLanguage: GetLanguage::<Identity, Impl, OFFSET>,
            SetLanguage: SetLanguage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamConfig3 as ::windows::core::ComInterface>::IID || iid == &<IWMStreamConfig as ::windows::core::ComInterface>::IID || iid == &<IWMStreamConfig2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMStreamList_Impl: Sized {
    fn GetStreams(&self, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::Result<()>;
    fn AddStream(&self, wstreamnum: u16) -> ::windows::core::Result<()>;
    fn RemoveStream(&self, wstreamnum: u16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMStreamList {}
impl IWMStreamList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamList_Impl, const OFFSET: isize>() -> IWMStreamList_Vtbl {
        unsafe extern "system" fn GetStreams<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStreams(::core::mem::transmute_copy(&pwstreamnumarray), ::core::mem::transmute_copy(&pcstreams)).into()
        }
        unsafe extern "system" fn AddStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStream(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStream(::core::mem::transmute_copy(&wstreamnum)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStreams: GetStreams::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamList as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamPrioritization_Impl: Sized {
    fn GetPriorityRecords(&self, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::Result<()>;
    fn SetPriorityRecords(&self, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMStreamPrioritization {}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamPrioritization_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamPrioritization_Impl, const OFFSET: isize>() -> IWMStreamPrioritization_Vtbl {
        unsafe extern "system" fn GetPriorityRecords<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPriorityRecords(::core::mem::transmute_copy(&precordarray), ::core::mem::transmute_copy(&pcrecords)).into()
        }
        unsafe extern "system" fn SetPriorityRecords<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMStreamPrioritization_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriorityRecords(::core::mem::transmute_copy(&precordarray), ::core::mem::transmute_copy(&crecords)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPriorityRecords: GetPriorityRecords::<Identity, Impl, OFFSET>,
            SetPriorityRecords: SetPriorityRecords::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStreamPrioritization as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMSyncReader_Impl: Sized {
    fn Open(&self, pwszfilename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn SetRange(&self, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::Result<()>;
    fn SetRangeByFrame(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<()>;
    fn GetNextSample(&self, wstreamnum: u16, ppsample: *mut ::core::option::Option<INSSBuffer>, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::Result<()>;
    fn SetStreamsSelected(&self, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::Result<()>;
    fn GetStreamSelected(&self, wstreamnum: u16) -> ::windows::core::Result<WMT_STREAM_SELECTION>;
    fn SetReadStreamSamples(&self, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReadStreamSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetOutputSetting(&self, dwoutputnum: u32, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetOutputSetting(&self, dwoutputnum: u32, pszname: &::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
    fn GetOutputCount(&self) -> ::windows::core::Result<u32>;
    fn GetOutputProps(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn SetOutputProps(&self, dwoutputnum: u32, poutput: ::core::option::Option<&IWMOutputMediaProps>) -> ::windows::core::Result<()>;
    fn GetOutputFormatCount(&self, dwoutputnum: u32) -> ::windows::core::Result<u32>;
    fn GetOutputFormat(&self, dwoutputnum: u32, dwformatnum: u32) -> ::windows::core::Result<IWMOutputMediaProps>;
    fn GetOutputNumberForStream(&self, wstreamnum: u16) -> ::windows::core::Result<u32>;
    fn GetStreamNumberForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<u16>;
    fn GetMaxOutputSampleSize(&self, dwoutput: u32) -> ::windows::core::Result<u32>;
    fn GetMaxStreamSampleSize(&self, wstream: u16) -> ::windows::core::Result<u32>;
    fn OpenStream(&self, pstream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMSyncReader {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMSyncReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>() -> IWMSyncReader_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&pwszfilename)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn SetRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRange(::core::mem::transmute_copy(&cnsstarttime), ::core::mem::transmute_copy(&cnsduration)).into()
        }
        unsafe extern "system" fn SetRangeByFrame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRangeByFrame(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&qwframenumber), ::core::mem::transmute_copy(&cframestoread)).into()
        }
        unsafe extern "system" fn GetNextSample<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut *mut ::core::ffi::c_void, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextSample(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&ppsample), ::core::mem::transmute_copy(&pcnssampletime), ::core::mem::transmute_copy(&pcnsduration), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pdwoutputnum), ::core::mem::transmute_copy(&pwstreamnum)).into()
        }
        unsafe extern "system" fn SetStreamsSelected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStreamsSelected(::core::mem::transmute_copy(&cstreamcount), ::core::mem::transmute_copy(&pwstreamnumbers), ::core::mem::transmute_copy(&pselections)).into()
        }
        unsafe extern "system" fn GetStreamSelected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamSelected(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pselection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetReadStreamSamples<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReadStreamSamples(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&fcompressed)).into()
        }
        unsafe extern "system" fn GetReadStreamSamples<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReadStreamSamples(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfcompressed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetOutputSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputSetting(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        unsafe extern "system" fn GetOutputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcoutputs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputProps(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutputProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputProps(::core::mem::transmute_copy(&dwoutputnum), ::windows::core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn GetOutputFormatCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputFormatCount(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformats, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputFormat(::core::mem::transmute_copy(&dwoutputnum), ::core::mem::transmute_copy(&dwformatnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprops, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputNumberForStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutputNumberForStream(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwoutputnum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamNumberForOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamNumberForOutput(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwstreamnum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxOutputSampleSize(::core::mem::transmute_copy(&dwoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxStreamSampleSize(::core::mem::transmute_copy(&wstream)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbmax, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenStream(::windows::core::from_raw_borrowed(&pstream)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMSyncReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMSyncReader2_Impl: Sized + IWMSyncReader_Impl {
    fn SetRangeByTimecode(&self, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::Result<()>;
    fn SetRangeByFrameEx(&self, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::Result<u64>;
    fn SetAllocateForOutput(&self, dwoutputnum: u32, pallocator: ::core::option::Option<&IWMReaderAllocatorEx>) -> ::windows::core::Result<()>;
    fn GetAllocateForOutput(&self, dwoutputnum: u32) -> ::windows::core::Result<IWMReaderAllocatorEx>;
    fn SetAllocateForStream(&self, wstreamnum: u16, pallocator: ::core::option::Option<&IWMReaderAllocatorEx>) -> ::windows::core::Result<()>;
    fn GetAllocateForStream(&self, dwsreamnum: u16) -> ::windows::core::Result<IWMReaderAllocatorEx>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IWMSyncReader2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMSyncReader2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: isize>() -> IWMSyncReader2_Vtbl {
        unsafe extern "system" fn SetRangeByTimecode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRangeByTimecode(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pstart), ::core::mem::transmute_copy(&pend)).into()
        }
        unsafe extern "system" fn SetRangeByFrameEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetRangeByFrameEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&qwframenumber), ::core::mem::transmute_copy(&cframestoread)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsstarttime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum), ::windows::core::from_raw_borrowed(&pallocator)).into()
        }
        unsafe extern "system" fn GetAllocateForOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllocateForOutput(::core::mem::transmute_copy(&dwoutputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppallocator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllocateForStream(::core::mem::transmute_copy(&wstreamnum), ::windows::core::from_raw_borrowed(&pallocator)).into()
        }
        unsafe extern "system" fn GetAllocateForStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMSyncReader2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllocateForStream(::core::mem::transmute_copy(&dwsreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppallocator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMSyncReader_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRangeByTimecode: SetRangeByTimecode::<Identity, Impl, OFFSET>,
            SetRangeByFrameEx: SetRangeByFrameEx::<Identity, Impl, OFFSET>,
            SetAllocateForOutput: SetAllocateForOutput::<Identity, Impl, OFFSET>,
            GetAllocateForOutput: GetAllocateForOutput::<Identity, Impl, OFFSET>,
            SetAllocateForStream: SetAllocateForStream::<Identity, Impl, OFFSET>,
            GetAllocateForStream: GetAllocateForStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMSyncReader2 as ::windows::core::ComInterface>::IID || iid == &<IWMSyncReader as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMVideoMediaProps_Impl: Sized + IWMMediaProps_Impl {
    fn GetMaxKeyFrameSpacing(&self) -> ::windows::core::Result<i64>;
    fn SetMaxKeyFrameSpacing(&self, lltime: i64) -> ::windows::core::Result<()>;
    fn GetQuality(&self) -> ::windows::core::Result<u32>;
    fn SetQuality(&self, dwquality: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMVideoMediaProps {}
#[cfg(feature = "Win32_Foundation")]
impl IWMVideoMediaProps_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>() -> IWMVideoMediaProps_Vtbl {
        unsafe extern "system" fn GetMaxKeyFrameSpacing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxKeyFrameSpacing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plltime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxKeyFrameSpacing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxKeyFrameSpacing(::core::mem::transmute_copy(&lltime)).into()
        }
        unsafe extern "system" fn GetQuality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQuality() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwquality, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMVideoMediaProps_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuality(::core::mem::transmute_copy(&dwquality)).into()
        }
        Self {
            base__: IWMMediaProps_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMaxKeyFrameSpacing: GetMaxKeyFrameSpacing::<Identity, Impl, OFFSET>,
            SetMaxKeyFrameSpacing: SetMaxKeyFrameSpacing::<Identity, Impl, OFFSET>,
            GetQuality: GetQuality::<Identity, Impl, OFFSET>,
            SetQuality: SetQuality::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMVideoMediaProps as ::windows::core::ComInterface>::IID || iid == &<IWMMediaProps as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMWatermarkInfo_Impl: Sized {
    fn GetWatermarkEntryCount(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE) -> ::windows::core::Result<u32>;
    fn GetWatermarkEntry(&self, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMWatermarkInfo {}
impl IWMWatermarkInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWatermarkInfo_Impl, const OFFSET: isize>() -> IWMWatermarkInfo_Vtbl {
        unsafe extern "system" fn GetWatermarkEntryCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWatermarkInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWatermarkEntryCount(::core::mem::transmute_copy(&wmettype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWatermarkEntry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWatermarkInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetWatermarkEntry(::core::mem::transmute_copy(&wmettype), ::core::mem::transmute_copy(&dwentrynum), ::core::mem::transmute_copy(&pentry)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWatermarkEntryCount: GetWatermarkEntryCount::<Identity, Impl, OFFSET>,
            GetWatermarkEntry: GetWatermarkEntry::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWatermarkInfo as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMWriter_Impl: Sized {
    fn SetProfileByID(&self, guidprofile: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetProfile(&self, pprofile: ::core::option::Option<&IWMProfile>) -> ::windows::core::Result<()>;
    fn SetOutputFilename(&self, pwszfilename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetInputCount(&self) -> ::windows::core::Result<u32>;
    fn GetInputProps(&self, dwinputnum: u32) -> ::windows::core::Result<IWMInputMediaProps>;
    fn SetInputProps(&self, dwinputnum: u32, pinput: ::core::option::Option<&IWMInputMediaProps>) -> ::windows::core::Result<()>;
    fn GetInputFormatCount(&self, dwinputnumber: u32) -> ::windows::core::Result<u32>;
    fn GetInputFormat(&self, dwinputnumber: u32, dwformatnumber: u32) -> ::windows::core::Result<IWMInputMediaProps>;
    fn BeginWriting(&self) -> ::windows::core::Result<()>;
    fn EndWriting(&self) -> ::windows::core::Result<()>;
    fn AllocateSample(&self, dwsamplesize: u32) -> ::windows::core::Result<INSSBuffer>;
    fn WriteSample(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>) -> ::windows::core::Result<()>;
    fn Flush(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMWriter {}
impl IWMWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>() -> IWMWriter_Vtbl {
        unsafe extern "system" fn SetProfileByID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProfileByID(::core::mem::transmute_copy(&guidprofile)).into()
        }
        unsafe extern "system" fn SetProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProfile(::windows::core::from_raw_borrowed(&pprofile)).into()
        }
        unsafe extern "system" fn SetOutputFilename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutputFilename(::core::mem::transmute(&pwszfilename)).into()
        }
        unsafe extern "system" fn GetInputCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcinputs, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputProps(::core::mem::transmute_copy(&dwinputnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInputProps(::core::mem::transmute_copy(&dwinputnum), ::windows::core::from_raw_borrowed(&pinput)).into()
        }
        unsafe extern "system" fn GetInputFormatCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputFormatCount(::core::mem::transmute_copy(&dwinputnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformats, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInputFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInputFormat(::core::mem::transmute_copy(&dwinputnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprops, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginWriting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginWriting().into()
        }
        unsafe extern "system" fn EndWriting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndWriting().into()
        }
        unsafe extern "system" fn AllocateSample<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllocateSample(::core::mem::transmute_copy(&dwsamplesize)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsample, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSample<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteSample(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&psample)).into()
        }
        unsafe extern "system" fn Flush<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Flush().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMWriter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced_Impl: Sized {
    fn GetSinkCount(&self) -> ::windows::core::Result<u32>;
    fn GetSink(&self, dwsinknum: u32) -> ::windows::core::Result<IWMWriterSink>;
    fn AddSink(&self, psink: ::core::option::Option<&IWMWriterSink>) -> ::windows::core::Result<()>;
    fn RemoveSink(&self, psink: ::core::option::Option<&IWMWriterSink>) -> ::windows::core::Result<()>;
    fn WriteStreamSample(&self, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>) -> ::windows::core::Result<()>;
    fn SetLiveSource(&self, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetWriterTime(&self) -> ::windows::core::Result<u64>;
    fn GetStatistics(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::Result<()>;
    fn SetSyncTolerance(&self, mswindow: u32) -> ::windows::core::Result<()>;
    fn GetSyncTolerance(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterAdvanced {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>() -> IWMWriterAdvanced_Vtbl {
        unsafe extern "system" fn GetSinkCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSinkCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcsinks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSink(::core::mem::transmute_copy(&dwsinknum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsink, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSink(::windows::core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn RemoveSink<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSink(::windows::core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn WriteStreamSample<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteStreamSample(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&mssamplesendtime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&psample)).into()
        }
        unsafe extern "system" fn SetLiveSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLiveSource(::core::mem::transmute_copy(&fislivesource)).into()
        }
        unsafe extern "system" fn IsRealTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrealtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWriterTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWriterTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnscurrenttime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatistics(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetSyncTolerance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSyncTolerance(::core::mem::transmute_copy(&mswindow)).into()
        }
        unsafe extern "system" fn GetSyncTolerance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSyncTolerance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmswindow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMWriterAdvanced as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced2_Impl: Sized + IWMWriterAdvanced_Impl {
    fn GetInputSetting(&self, dwinputnum: u32, pszname: &::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::Result<()>;
    fn SetInputSetting(&self, dwinputnum: u32, pszname: &::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterAdvanced2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced2_Impl, const OFFSET: isize>() -> IWMWriterAdvanced2_Vtbl {
        unsafe extern "system" fn GetInputSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows::core::PCWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInputSetting(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ptype), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&pcblength)).into()
        }
        unsafe extern "system" fn SetInputSetting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: ::windows::core::PCWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInputSetting(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&pvalue), ::core::mem::transmute_copy(&cblength)).into()
        }
        Self {
            base__: IWMWriterAdvanced_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetInputSetting: GetInputSetting::<Identity, Impl, OFFSET>,
            SetInputSetting: SetInputSetting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterAdvanced2 as ::windows::core::ComInterface>::IID || iid == &<IWMWriterAdvanced as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterAdvanced3_Impl: Sized + IWMWriterAdvanced2_Impl {
    fn GetStatisticsEx(&self, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::Result<()>;
    fn SetNonBlocking(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterAdvanced3 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced3_Impl, const OFFSET: isize>() -> IWMWriterAdvanced3_Vtbl {
        unsafe extern "system" fn GetStatisticsEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatisticsEx(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetNonBlocking<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterAdvanced3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNonBlocking().into()
        }
        Self {
            base__: IWMWriterAdvanced2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetStatisticsEx: GetStatisticsEx::<Identity, Impl, OFFSET>,
            SetNonBlocking: SetNonBlocking::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterAdvanced3 as ::windows::core::ComInterface>::IID || iid == &<IWMWriterAdvanced as ::windows::core::ComInterface>::IID || iid == &<IWMWriterAdvanced2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink_Impl: Sized + IWMWriterSink_Impl {
    fn Open(&self, pwszfilename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterFileSink {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink_Impl, const OFFSET: isize>() -> IWMWriterFileSink_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&pwszfilename)).into()
        }
        Self { base__: IWMWriterSink_Vtbl::new::<Identity, Impl, OFFSET>(), Open: Open::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterFileSink as ::windows::core::ComInterface>::IID || iid == &<IWMWriterSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink2_Impl: Sized + IWMWriterFileSink_Impl {
    fn Start(&self, cnsstarttime: u64) -> ::windows::core::Result<()>;
    fn Stop(&self, cnsstoptime: u64) -> ::windows::core::Result<()>;
    fn IsStopped(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetFileDuration(&self) -> ::windows::core::Result<u64>;
    fn GetFileSize(&self) -> ::windows::core::Result<u64>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterFileSink2 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSink2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>() -> IWMWriterFileSink2_Vtbl {
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute_copy(&cnsstarttime)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop(::core::mem::transmute_copy(&cnsstoptime)).into()
        }
        unsafe extern "system" fn IsStopped<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsStopped() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfstopped, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileDuration() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcnsduration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFileSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFileSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn IsClosed<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsClosed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfclosed, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWMWriterFileSink_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMWriterFileSink2 as ::windows::core::ComInterface>::IID || iid == &<IWMWriterSink as ::windows::core::ComInterface>::IID || iid == &<IWMWriterFileSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink3_Impl: Sized + IWMWriterFileSink2_Impl {
    fn SetAutoIndexing(&self, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAutoIndexing(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetControlStream(&self, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMode(&self) -> ::windows::core::Result<u32>;
    fn OnDataUnitEx(&self, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::Result<()>;
    fn SetUnbufferedIO(&self, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetUnbufferedIO(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CompleteOperations(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterFileSink3 {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSink3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>() -> IWMWriterFileSink3_Vtbl {
        unsafe extern "system" fn SetAutoIndexing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAutoIndexing(::core::mem::transmute_copy(&fdoautoindexing)).into()
        }
        unsafe extern "system" fn GetAutoIndexing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAutoIndexing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfautoindexing, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetControlStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetControlStream(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&fshouldcontrolstartandstop)).into()
        }
        unsafe extern "system" fn GetMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwfilesinkmode, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnitEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataUnitEx(::core::mem::transmute_copy(&pfilesinkdataunit)).into()
        }
        unsafe extern "system" fn SetUnbufferedIO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUnbufferedIO(::core::mem::transmute_copy(&funbufferedio), ::core::mem::transmute_copy(&frestrictmemusage)).into()
        }
        unsafe extern "system" fn GetUnbufferedIO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUnbufferedIO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfunbufferedio, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CompleteOperations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterFileSink3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompleteOperations().into()
        }
        Self {
            base__: IWMWriterFileSink2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMWriterFileSink3 as ::windows::core::ComInterface>::IID || iid == &<IWMWriterSink as ::windows::core::ComInterface>::IID || iid == &<IWMWriterFileSink as ::windows::core::ComInterface>::IID || iid == &<IWMWriterFileSink2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterNetworkSink_Impl: Sized + IWMWriterSink_Impl {
    fn SetMaximumClients(&self, dwmaxclients: u32) -> ::windows::core::Result<()>;
    fn GetMaximumClients(&self) -> ::windows::core::Result<u32>;
    fn SetNetworkProtocol(&self, protocol: WMT_NET_PROTOCOL) -> ::windows::core::Result<()>;
    fn GetNetworkProtocol(&self) -> ::windows::core::Result<WMT_NET_PROTOCOL>;
    fn GetHostURL(&self, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::Result<()>;
    fn Open(&self, pdwportnum: *mut u32) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterNetworkSink {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterNetworkSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>() -> IWMWriterNetworkSink_Vtbl {
        unsafe extern "system" fn SetMaximumClients<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaximumClients(::core::mem::transmute_copy(&dwmaxclients)).into()
        }
        unsafe extern "system" fn GetMaximumClients<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaximumClients() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxclients, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNetworkProtocol<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNetworkProtocol(::core::mem::transmute_copy(&protocol)).into()
        }
        unsafe extern "system" fn GetNetworkProtocol<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkProtocol() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprotocol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostURL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHostURL(::core::mem::transmute_copy(&pwszurl), ::core::mem::transmute_copy(&pcchurl)).into()
        }
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute_copy(&pdwportnum)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect().into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterNetworkSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: IWMWriterSink_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWMWriterNetworkSink as ::windows::core::ComInterface>::IID || iid == &<IWMWriterSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterPostView_Impl: Sized {
    fn SetPostViewCallback(&self, pcallback: ::core::option::Option<&IWMWriterPostViewCallback>, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetReceivePostViewSamples(&self, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetReceivePostViewSamples(&self, wstreamnum: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetPostViewProps(&self, wstreamnumber: u16) -> ::windows::core::Result<IWMMediaProps>;
    fn SetPostViewProps(&self, wstreamnumber: u16, poutput: ::core::option::Option<&IWMMediaProps>) -> ::windows::core::Result<()>;
    fn GetPostViewFormatCount(&self, wstreamnumber: u16) -> ::windows::core::Result<u32>;
    fn GetPostViewFormat(&self, wstreamnumber: u16, dwformatnumber: u32) -> ::windows::core::Result<IWMMediaProps>;
    fn SetAllocateForPostView(&self, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetAllocateForPostView(&self, wstreamnumber: u16) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterPostView {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterPostView_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>() -> IWMWriterPostView_Vtbl {
        unsafe extern "system" fn SetPostViewCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPostViewCallback(::windows::core::from_raw_borrowed(&pcallback), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn SetReceivePostViewSamples<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetReceivePostViewSamples(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&freceivepostviewsamples)).into()
        }
        unsafe extern "system" fn GetReceivePostViewSamples<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetReceivePostViewSamples(::core::mem::transmute_copy(&wstreamnum)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfreceivepostviewsamples, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPostViewProps(::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostViewProps<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPostViewProps(::core::mem::transmute_copy(&wstreamnumber), ::windows::core::from_raw_borrowed(&poutput)).into()
        }
        unsafe extern "system" fn GetPostViewFormatCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPostViewFormatCount(::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcformats, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPostViewFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPostViewFormat(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&dwformatnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppprops, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllocateForPostView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllocateForPostView(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&fallocate)).into()
        }
        unsafe extern "system" fn GetAllocateForPostView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostView_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllocateForPostView(::core::mem::transmute_copy(&wstreamnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallocate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IWMWriterPostView as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMWriterPostViewCallback_Impl: Sized + IWMStatusCallback_Impl {
    fn OnPostViewSample(&self, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn AllocateForPostView(&self, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::core::option::Option<INSSBuffer>, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMWriterPostViewCallback {}
impl IWMWriterPostViewCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostViewCallback_Impl, const OFFSET: isize>() -> IWMWriterPostViewCallback_Vtbl {
        unsafe extern "system" fn OnPostViewSample<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostViewCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPostViewSample(::core::mem::transmute_copy(&wstreamnumber), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&cnssampleduration), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&psample), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        unsafe extern "system" fn AllocateForPostView<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPostViewCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut *mut ::core::ffi::c_void, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllocateForPostView(::core::mem::transmute_copy(&wstreamnum), ::core::mem::transmute_copy(&cbbuffer), ::core::mem::transmute_copy(&ppbuffer), ::core::mem::transmute_copy(&pvcontext)).into()
        }
        Self {
            base__: IWMStatusCallback_Vtbl::new::<Identity, Impl, OFFSET>(),
            OnPostViewSample: OnPostViewSample::<Identity, Impl, OFFSET>,
            AllocateForPostView: AllocateForPostView::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPostViewCallback as ::windows::core::ComInterface>::IID || iid == &<IWMStatusCallback as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"implement\"`*"]
pub trait IWMWriterPreprocess_Impl: Sized {
    fn GetMaxPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<u32>;
    fn SetNumPreprocessingPasses(&self, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::Result<()>;
    fn BeginPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn PreprocessSample(&self, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::core::option::Option<&INSSBuffer>) -> ::windows::core::Result<()>;
    fn EndPreprocessingPass(&self, dwinputnum: u32, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWMWriterPreprocess {}
impl IWMWriterPreprocess_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>() -> IWMWriterPreprocess_Vtbl {
        unsafe extern "system" fn GetMaxPreprocessingPasses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxPreprocessingPasses(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxnumpasses, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNumPreprocessingPasses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNumPreprocessingPasses(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwnumpasses)).into()
        }
        unsafe extern "system" fn BeginPreprocessingPass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginPreprocessingPass(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn PreprocessSample<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreprocessSample(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&cnssampletime), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&psample)).into()
        }
        unsafe extern "system" fn EndPreprocessingPass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPreprocess_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndPreprocessingPass(::core::mem::transmute_copy(&dwinputnum), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaxPreprocessingPasses: GetMaxPreprocessingPasses::<Identity, Impl, OFFSET>,
            SetNumPreprocessingPasses: SetNumPreprocessingPasses::<Identity, Impl, OFFSET>,
            BeginPreprocessingPass: BeginPreprocessingPass::<Identity, Impl, OFFSET>,
            PreprocessSample: PreprocessSample::<Identity, Impl, OFFSET>,
            EndPreprocessingPass: EndPreprocessingPass::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPreprocess as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterPushSink_Impl: Sized + IWMWriterSink_Impl {
    fn Connect(&self, pwszurl: &::windows::core::PCWSTR, pwsztemplateurl: &::windows::core::PCWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn EndSession(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterPushSink {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterPushSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPushSink_Impl, const OFFSET: isize>() -> IWMWriterPushSink_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPushSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: ::windows::core::PCWSTR, pwsztemplateurl: ::windows::core::PCWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect(::core::mem::transmute(&pwszurl), ::core::mem::transmute(&pwsztemplateurl), ::core::mem::transmute_copy(&fautodestroy)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPushSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect().into()
        }
        unsafe extern "system" fn EndSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterPushSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSession().into()
        }
        Self {
            base__: IWMWriterSink_Vtbl::new::<Identity, Impl, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            EndSession: EndSession::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterPushSink as ::windows::core::ComInterface>::IID || iid == &<IWMWriterSink as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_WindowsMediaFormat\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterSink_Impl: Sized {
    fn OnHeader(&self, pheader: ::core::option::Option<&INSSBuffer>) -> ::windows::core::Result<()>;
    fn IsRealTime(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn AllocateDataUnit(&self, cbdataunit: u32) -> ::windows::core::Result<INSSBuffer>;
    fn OnDataUnit(&self, pdataunit: ::core::option::Option<&INSSBuffer>) -> ::windows::core::Result<()>;
    fn OnEndWriting(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IWMWriterSink {}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: isize>() -> IWMWriterSink_Vtbl {
        unsafe extern "system" fn OnHeader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnHeader(::windows::core::from_raw_borrowed(&pheader)).into()
        }
        unsafe extern "system" fn IsRealTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfrealtime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllocateDataUnit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllocateDataUnit(::core::mem::transmute_copy(&cbdataunit)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdataunit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUnit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataunit: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataUnit(::windows::core::from_raw_borrowed(&pdataunit)).into()
        }
        unsafe extern "system" fn OnEndWriting<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWMWriterSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEndWriting().into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnHeader: OnHeader::<Identity, Impl, OFFSET>,
            IsRealTime: IsRealTime::<Identity, Impl, OFFSET>,
            AllocateDataUnit: AllocateDataUnit::<Identity, Impl, OFFSET>,
            OnDataUnit: OnDataUnit::<Identity, Impl, OFFSET>,
            OnEndWriting: OnEndWriting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterSink as ::windows::core::ComInterface>::IID
    }
}
