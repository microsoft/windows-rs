pub trait IAMWMBufferPassImpl: Sized {
    fn SetNotify();
}
impl IAMWMBufferPassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAMWMBufferPassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAMWMBufferPassVtbl {
        unsafe extern "system" fn SetNotify<Impl: IAMWMBufferPassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetNotify: SetNotify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAMWMBufferPass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_DirectShow")]
pub trait IAMWMBufferPassCallbackImpl: Sized {
    fn Notify();
}
#[cfg(feature = "Win32_Media_DirectShow")]
impl IAMWMBufferPassCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAMWMBufferPassCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAMWMBufferPassCallbackVtbl {
        unsafe extern "system" fn Notify<Impl: IAMWMBufferPassCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnssbuffer3: ::windows::core::RawPtr, ppin: ::windows::core::RawPtr, prtstart: *const i64, prtend: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Notify: Notify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAMWMBufferPassCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait INSNetSourceCreatorImpl: Sized {
    fn Initialize();
    fn CreateNetSource();
    fn GetNetSourceProperties();
    fn GetNetSourceSharedNamespace();
    fn GetNetSourceAdminInterface();
    fn GetNumProtocolsSupported();
    fn GetProtocolName();
    fn Shutdown();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl INSNetSourceCreatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSNetSourceCreatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSNetSourceCreatorVtbl {
        unsafe extern "system" fn Initialize<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNetSource<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pmonitor: *mut ::core::ffi::c_void, pdata: *const u8, pusercontext: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, qwcontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetSourceProperties<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pppropertiesnode: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetSourceSharedNamespace<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsharednamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetSourceAdminInterface<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszstreamname: super::super::Foundation::PWSTR, pval: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtocolName<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: INSNetSourceCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetLength();
    fn SetLength();
    fn GetMaxLength();
    fn GetBuffer();
    fn GetBufferAndLength();
}
impl INSSBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSSBufferVtbl {
        unsafe extern "system" fn GetLength<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLength<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxLength<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBuffer<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferAndLength<Impl: INSSBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdwbuffer: *mut *mut u8, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetSampleProperties();
    fn SetSampleProperties();
}
impl INSSBuffer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSSBuffer2Vtbl {
        unsafe extern "system" fn GetSampleProperties<Impl: INSSBuffer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSampleProperties<Impl: INSSBuffer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbproperties: u32, pbproperties: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetProperty();
    fn GetProperty();
}
impl INSSBuffer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSSBuffer3Vtbl {
        unsafe extern "system" fn SetProperty<Impl: INSSBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *const ::core::ffi::c_void, dwbufferpropertysize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: INSSBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidbufferproperty: ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetPropertyCount();
    fn GetPropertyByIndex();
}
impl INSSBuffer4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INSSBuffer4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INSSBuffer4Vtbl {
        unsafe extern "system" fn GetPropertyCount<Impl: INSSBuffer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbufferproperties: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyByIndex<Impl: INSSBuffer4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbufferpropertyindex: u32, pguidbufferproperty: *mut ::windows::core::GUID, pvbufferproperty: *mut ::core::ffi::c_void, pdwbufferpropertysize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetAccessEntryCount();
    fn GetAccessEntry();
    fn AddAccessEntry();
    fn RemoveAccessEntry();
}
impl IWMAddressAccessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMAddressAccessVtbl {
        unsafe extern "system" fn GetAccessEntryCount<Impl: IWMAddressAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, pcentries: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAccessEntry<Impl: IWMAddressAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, paddraccessentry: *mut WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAccessEntry<Impl: IWMAddressAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, paddraccessentry: *const WM_ADDRESS_ACCESSENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAccessEntry<Impl: IWMAddressAccessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetAccessEntryEx();
    fn AddAccessEntryEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMAddressAccess2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMAddressAccess2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMAddressAccess2Vtbl {
        unsafe extern "system" fn GetAccessEntryEx<Impl: IWMAddressAccess2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, dwentrynum: u32, pbstraddress: *mut super::super::Foundation::BSTR, pbstrmask: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAccessEntryEx<Impl: IWMAddressAccess2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aetype: WM_AETYPE, bstraddress: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetCertCount();
    fn GetCert();
    fn GetSharedData();
}
impl IWMAuthorizerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMAuthorizerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMAuthorizerVtbl {
        unsafe extern "system" fn GetCertCount<Impl: IWMAuthorizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccerts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCert<Impl: IWMAuthorizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppbcertdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSharedData<Impl: IWMAuthorizerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8, pbcert: *const u8, ppbshareddata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetPropCount();
    fn GetPropByIndex();
    fn GetPropByName();
    fn SetProp();
    fn RemoveProp();
    fn RemoveAllProps();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMBackupRestorePropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMBackupRestorePropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMBackupRestorePropsVtbl {
        unsafe extern "system" fn GetPropCount<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprops: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropByIndex<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropByName<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProp<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveProp<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllProps<Impl: IWMBackupRestorePropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetType();
    fn SetType();
    fn GetBandwidth();
    fn SetBandwidth();
}
impl IWMBandwidthSharingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMBandwidthSharingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMBandwidthSharingVtbl {
        unsafe extern "system" fn GetType<Impl: IWMBandwidthSharingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetType<Impl: IWMBandwidthSharingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBandwidth<Impl: IWMBandwidthSharingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBandwidth<Impl: IWMBandwidthSharingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbitrate: u32, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetClientCount();
    fn GetClientProperties();
}
impl IWMClientConnectionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMClientConnectionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMClientConnectionsVtbl {
        unsafe extern "system" fn GetClientCount<Impl: IWMClientConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClientProperties<Impl: IWMClientConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pclientproperties: *mut WM_CLIENT_PROPERTIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetClientInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMClientConnections2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMClientConnections2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMClientConnections2Vtbl {
        unsafe extern "system" fn GetClientInfo<Impl: IWMClientConnections2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientnum: u32, pwsznetworkaddress: super::super::Foundation::PWSTR, pcchnetworkaddress: *mut u32, pwszport: super::super::Foundation::PWSTR, pcchport: *mut u32, pwszdnsname: super::super::Foundation::PWSTR, pcchdnsname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMClientConnectionsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetClientInfo: GetClientInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMClientConnections2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
pub trait IWMCodecAMVideoAcceleratorImpl: Sized {
    fn SetAcceleratorInterface();
    fn NegotiateConnection();
    fn SetPlayerNotify();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl IWMCodecAMVideoAcceleratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecAMVideoAcceleratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecAMVideoAcceleratorVtbl {
        unsafe extern "system" fn SetAcceleratorInterface<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NegotiateConnection<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlayerNotify<Impl: IWMCodecAMVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetCodecInfoCount();
    fn GetCodecFormatCount();
    fn GetCodecFormat();
}
impl IWMCodecInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecInfoVtbl {
        unsafe extern "system" fn GetCodecInfoCount<Impl: IWMCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, pccodecs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecFormatCount<Impl: IWMCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pcformat: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecFormat<Impl: IWMCodecInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetCodecName();
    fn GetCodecFormatDesc();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMCodecInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecInfo2Vtbl {
        unsafe extern "system" fn GetCodecName<Impl: IWMCodecInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, wszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecFormatDesc<Impl: IWMCodecInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, ppistreamconfig: *mut ::windows::core::RawPtr, wszdesc: super::super::Foundation::PWSTR, pcchdesc: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetCodecFormatProp();
    fn GetCodecProp();
    fn SetCodecEnumerationSetting();
    fn GetCodecEnumerationSetting();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMCodecInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecInfo3Vtbl {
        unsafe extern "system" fn GetCodecFormatProp<Impl: IWMCodecInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, dwformatindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecProp<Impl: IWMCodecInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCodecEnumerationSetting<Impl: IWMCodecInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecEnumerationSetting<Impl: IWMCodecInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID, dwcodecindex: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn NegotiateConnection();
    fn SetPlayerNotify();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_DirectShow"))]
impl IWMCodecVideoAcceleratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCodecVideoAcceleratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCodecVideoAcceleratorVtbl {
        unsafe extern "system" fn NegotiateConnection<Impl: IWMCodecVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piamva: ::windows::core::RawPtr, pmediatype: *const super::DirectShow::AM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlayerNotify<Impl: IWMCodecVideoAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn AcquireCredentials();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMCredentialCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMCredentialCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMCredentialCallbackVtbl {
        unsafe extern "system" fn AcquireCredentials<Impl: IWMCredentialCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszrealm: super::super::Foundation::PWSTR, pwszsite: super::super::Foundation::PWSTR, pwszuser: super::super::Foundation::PWSTR, cchuser: u32, pwszpassword: super::super::Foundation::PWSTR, cchpassword: u32, hrstatus: ::windows::core::HRESULT, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AcquireCredentials: AcquireCredentials::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMCredentialCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMEditorImpl: Sized {
    fn GetDRMProperty();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMEditorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMEditorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMEditorVtbl {
        unsafe extern "system" fn GetDRMProperty<Impl: IWMDRMEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDRMProperty: GetDRMProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMEditor as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMMessageParserImpl: Sized {
    fn ParseRegistrationReqMsg();
    fn ParseLicenseRequestMsg();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMMessageParserVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMMessageParserImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMMessageParserVtbl {
        unsafe extern "system" fn ParseRegistrationReqMsg<Impl: IWMDRMMessageParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistrationreqmsg: *const u8, cbregistrationreqmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParseLicenseRequestMsg<Impl: IWMDRMMessageParserImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pblicenserequestmsg: *const u8, cblicenserequestmsg: u32, ppdevicecert: *mut ::windows::core::RawPtr, pdeviceserialnumber: *mut DRM_VAL16, pbstraction: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn AcquireLicense();
    fn CancelLicenseAcquisition();
    fn Individualize();
    fn CancelIndividualization();
    fn MonitorLicenseAcquisition();
    fn CancelMonitorLicenseAcquisition();
    fn SetDRMProperty();
    fn GetDRMProperty();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMReaderVtbl {
        unsafe extern "system" fn AcquireLicense<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelLicenseAcquisition<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Individualize<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelIndividualization<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MonitorLicenseAcquisition<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelMonitorLicenseAcquisition<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDRMProperty<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDRMProperty<Impl: IWMDRMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrname: super::super::Foundation::PWSTR, pdwtype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetEvaluateOutputLevelLicenses();
    fn GetPlayOutputLevels();
    fn GetCopyOutputLevels();
    fn TryNextLicense();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMReader2Vtbl {
        unsafe extern "system" fn SetEvaluateOutputLevelLicenses<Impl: IWMDRMReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fevaluate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPlayOutputLevels<Impl: IWMDRMReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplayopl: *mut DRM_PLAY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCopyOutputLevels<Impl: IWMDRMReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcopyopl: *mut DRM_COPY_OPL, pcblength: *mut u32, pdwminappcompliancelevel: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TryNextLicense<Impl: IWMDRMReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetInclusionList();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMReader3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMReader3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMReader3Vtbl {
        unsafe extern "system" fn GetInclusionList<Impl: IWMDRMReader3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppguids: *mut *mut ::windows::core::GUID, pcguids: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMDRMReader2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetInclusionList: GetInclusionList::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMReader3 as ::windows::core::Interface>::IID
    }
}
pub trait IWMDRMTranscryptionManagerImpl: Sized {
    fn CreateTranscryptor();
}
impl IWMDRMTranscryptionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptionManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMTranscryptionManagerVtbl {
        unsafe extern "system" fn CreateTranscryptor<Impl: IWMDRMTranscryptionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptranscryptor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateTranscryptor: CreateTranscryptor::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMTranscryptionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMTranscryptorImpl: Sized {
    fn Initialize();
    fn Seek();
    fn Read();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMTranscryptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMTranscryptorVtbl {
        unsafe extern "system" fn Initialize<Impl: IWMDRMTranscryptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pblicenserequestmsg: *mut u8, cblicenserequestmsg: u32, pplicenseresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Seek<Impl: IWMDRMTranscryptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Read<Impl: IWMDRMTranscryptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, pcbdata: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWMDRMTranscryptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SeekEx();
    fn ZeroAdjustTimestamps();
    fn GetSeekStartTime();
    fn GetDuration();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMTranscryptor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMTranscryptor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMTranscryptor2Vtbl {
        unsafe extern "system" fn SeekEx<Impl: IWMDRMTranscryptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: u64, flrate: f32, fincludefileheader: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ZeroAdjustTimestamps<Impl: IWMDRMTranscryptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSeekStartTime<Impl: IWMDRMTranscryptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnstime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDuration<Impl: IWMDRMTranscryptor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GenerateKeySeed();
    fn GenerateKeyID();
    fn GenerateSigningKeyPair();
    fn SetDRMAttribute();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMWriterVtbl {
        unsafe extern "system" fn GenerateKeySeed<Impl: IWMDRMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszkeyseed: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateKeyID<Impl: IWMDRMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszkeyid: super::super::Foundation::PWSTR, pcwchlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GenerateSigningKeyPair<Impl: IWMDRMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprivkey: super::super::Foundation::PWSTR, pcwchprivkeylength: *mut u32, pwszpubkey: super::super::Foundation::PWSTR, pcwchpubkeylength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDRMAttribute<Impl: IWMDRMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetWMDRMNetEncryption();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMWriter2Vtbl {
        unsafe extern "system" fn SetWMDRMNetEncryption<Impl: IWMDRMWriter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsamplesencrypted: super::super::Foundation::BOOL, pbkeyid: *const u8, cbkeyid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMDRMWriterVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetWMDRMNetEncryption: SetWMDRMNetEncryption::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMDRMWriter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMDRMWriter3Impl: Sized + IWMDRMWriterImpl + IWMDRMWriter2Impl {
    fn SetProtectStreamSamples();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMDRMWriter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDRMWriter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDRMWriter3Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Impl: IWMDRMWriter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pimportinitstruct: *const WMDRM_IMPORT_INIT_STRUCT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn RegisterDevice();
    fn UnregisterDevice();
    fn GetRegistrationStats();
    fn GetFirstRegisteredDevice();
    fn GetNextRegisteredDevice();
    fn GetRegisteredDeviceByID();
}
impl IWMDeviceRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMDeviceRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMDeviceRegistrationVtbl {
        unsafe extern "system" fn RegisterDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegistrationStats<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pcregistereddevices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFirstRegisteredDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextRegisteredDevice<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRegisteredDeviceByID<Impl: IWMDeviceRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregistertype: u32, pbcertificate: *const u8, cbcertificate: u32, serialnumber: DRM_VAL16, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetPeerSecureChannelInterface();
}
impl IWMGetSecureChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMGetSecureChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMGetSecureChannelVtbl {
        unsafe extern "system" fn GetPeerSecureChannelInterface<Impl: IWMGetSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppeer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetAttributeCount();
    fn GetAttributeByIndex();
    fn GetAttributeByName();
    fn SetAttribute();
    fn GetMarkerCount();
    fn GetMarker();
    fn AddMarker();
    fn RemoveMarker();
    fn GetScriptCount();
    fn GetScript();
    fn AddScript();
    fn RemoveScript();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMHeaderInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMHeaderInfoVtbl {
        unsafe extern "system" fn GetAttributeCount<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeByIndex<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwstreamnum: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeByName<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttribute<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMarkerCount<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcmarkers: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMarker<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszmarkername: super::super::Foundation::PWSTR, pcchmarkernamelen: *mut u16, pcnsmarkertime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMarker<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmarkername: super::super::Foundation::PWSTR, cnsmarkertime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveMarker<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScriptCount<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcscripts: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScript<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwsztype: super::super::Foundation::PWSTR, pcchtypelen: *mut u16, pwszcommand: super::super::Foundation::PWSTR, pcchcommandlen: *mut u16, pcnsscripttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddScript<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsztype: super::super::Foundation::PWSTR, pwszcommand: super::super::Foundation::PWSTR, cnsscripttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveScript<Impl: IWMHeaderInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetCodecInfoCount();
    fn GetCodecInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMHeaderInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMHeaderInfo2Vtbl {
        unsafe extern "system" fn GetCodecInfoCount<Impl: IWMHeaderInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pccodecinfos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCodecInfo<Impl: IWMHeaderInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u32, pcchname: *mut u16, pwszname: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pcodectype: *mut WMT_CODEC_INFO_TYPE, pcbcodecinfo: *mut u16, pbcodecinfo: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetAttributeCountEx();
    fn GetAttributeIndices();
    fn GetAttributeByIndexEx();
    fn ModifyAttribute();
    fn AddAttribute();
    fn DeleteAttribute();
    fn AddCodecInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMHeaderInfo3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMHeaderInfo3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMHeaderInfo3Vtbl {
        unsafe extern "system" fn GetAttributeCountEx<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pcattributes: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeIndices<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwszname: super::super::Foundation::PWSTR, pwlangindex: *const u16, pwindices: *mut u16, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeByIndexEx<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, pwszname: super::super::Foundation::PWSTR, pwnamelen: *mut u16, ptype: *mut WMT_ATTR_DATATYPE, pwlangindex: *mut u16, pvalue: *mut u8, pdwdatalength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ModifyAttribute<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAttribute<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pszname: super::super::Foundation::PWSTR, pwindex: *mut u16, r#type: WMT_ATTR_DATATYPE, wlangindex: u16, pvalue: *const u8, dwlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAttribute<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, windex: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddCodecInfo<Impl: IWMHeaderInfo3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pwszdescription: super::super::Foundation::PWSTR, codectype: WMT_CODEC_INFO_TYPE, cbcodecinfo: u16, pbcodecinfo: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetProperty();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMIStreamPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIStreamPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMIStreamPropsVtbl {
        unsafe extern "system" fn GetProperty<Impl: IWMIStreamPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetProperty: GetProperty::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIStreamProps as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMImageInfoImpl: Sized {
    fn GetImageCount();
    fn GetImage();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMImageInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMImageInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMImageInfoVtbl {
        unsafe extern "system" fn GetImageCount<Impl: IWMImageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcimages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImage<Impl: IWMImageInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u32, pcchmimetype: *mut u16, pwszmimetype: super::super::Foundation::PWSTR, pcchdescription: *mut u16, pwszdescription: super::super::Foundation::PWSTR, pimagetype: *mut u16, pcbimagedata: *mut u32, pbimagedata: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn StartIndexing();
    fn Cancel();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMIndexerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIndexerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMIndexerVtbl {
        unsafe extern "system" fn StartIndexing<Impl: IWMIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IWMIndexerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Configure();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMIndexer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMIndexer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMIndexer2Vtbl {
        unsafe extern "system" fn Configure<Impl: IWMIndexer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, nindexertype: WMT_INDEXER_TYPE, pvinterval: *const ::core::ffi::c_void, pvindextype: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMIndexerVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Configure: Configure::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMIndexer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMInputMediaPropsImpl: Sized + IWMMediaPropsImpl {
    fn GetConnectionName();
    fn GetGroupName();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMInputMediaPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMInputMediaPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMInputMediaPropsVtbl {
        unsafe extern "system" fn GetConnectionName<Impl: IWMInputMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGroupName<Impl: IWMInputMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetLanguageCount();
    fn GetLanguageDetails();
    fn AddLanguageByRFC1766String();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMLanguageListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLanguageListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMLanguageListVtbl {
        unsafe extern "system" fn GetLanguageCount<Impl: IWMLanguageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLanguageDetails<Impl: IWMLanguageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windex: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddLanguageByRFC1766String<Impl: IWMLanguageListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pwindex: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn BackupLicenses();
    fn CancelLicenseBackup();
}
impl IWMLicenseBackupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseBackupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMLicenseBackupVtbl {
        unsafe extern "system" fn BackupLicenses<Impl: IWMLicenseBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelLicenseBackup<Impl: IWMLicenseBackupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn RestoreLicenses();
    fn CancelLicenseRestore();
}
impl IWMLicenseRestoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRestoreImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMLicenseRestoreVtbl {
        unsafe extern "system" fn RestoreLicenses<Impl: IWMLicenseRestoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelLicenseRestore<Impl: IWMLicenseRestoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetLRBChallenge();
    fn ProcessLRB();
}
impl IWMLicenseRevocationAgentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMLicenseRevocationAgentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMLicenseRevocationAgentVtbl {
        unsafe extern "system" fn GetLRBChallenge<Impl: IWMLicenseRevocationAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmachineid: *const u8, dwmachineidlength: u32, pchallenge: *const u8, dwchallengelength: u32, pchallengeoutput: *mut u8, pdwchallengeoutputlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessLRB<Impl: IWMLicenseRevocationAgentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psignedlrb: *const u8, dwsignedlrblength: u32, psignedack: *mut u8, pdwsignedacklength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetType();
    fn GetMediaType();
    fn SetMediaType();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMediaPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMediaPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMediaPropsVtbl {
        unsafe extern "system" fn GetType<Impl: IWMMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMediaType<Impl: IWMMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut WM_MEDIA_TYPE, pcbtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMediaType<Impl: IWMMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Open();
    fn Close();
    fn Flush();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMetadataEditorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMetadataEditorVtbl {
        unsafe extern "system" fn Open<Impl: IWMMetadataEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWMMetadataEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IWMMetadataEditorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn OpenEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMetadataEditor2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMetadataEditor2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMetadataEditor2Vtbl {
        unsafe extern "system" fn OpenEx<Impl: IWMMetadataEditor2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwsharemode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMMetadataEditorVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OpenEx: OpenEx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMMetadataEditor2 as ::windows::core::Interface>::IID
    }
}
pub trait IWMMutualExclusionImpl: Sized + IWMStreamListImpl {
    fn GetType();
    fn SetType();
}
impl IWMMutualExclusionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMutualExclusionVtbl {
        unsafe extern "system" fn GetType<Impl: IWMMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetType<Impl: IWMMutualExclusionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidtype: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetName();
    fn SetName();
    fn GetRecordCount();
    fn AddRecord();
    fn RemoveRecord();
    fn GetRecordName();
    fn SetRecordName();
    fn GetStreamsForRecord();
    fn AddStreamForRecord();
    fn RemoveStreamForRecord();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMMutualExclusion2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMMutualExclusion2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMMutualExclusion2Vtbl {
        unsafe extern "system" fn GetName<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecordCount<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwrecordcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecordName<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR, pcchrecordname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRecordName<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwszrecordname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamsForRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStreamForRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStreamForRecord<Impl: IWMMutualExclusion2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wrecordnumber: u16, wstreamnumber: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetStreamGroupName();
    fn GetConnectionName();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMOutputMediaPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMOutputMediaPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMOutputMediaPropsVtbl {
        unsafe extern "system" fn GetStreamGroupName<Impl: IWMOutputMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectionName<Impl: IWMOutputMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetMaxPacketSize();
    fn SetMaxPacketSize();
}
impl IWMPacketSizeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSizeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPacketSizeVtbl {
        unsafe extern "system" fn GetMaxPacketSize<Impl: IWMPacketSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxPacketSize<Impl: IWMPacketSizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetMinPacketSize();
    fn SetMinPacketSize();
}
impl IWMPacketSize2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPacketSize2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPacketSize2Vtbl {
        unsafe extern "system" fn GetMinPacketSize<Impl: IWMPacketSize2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinPacketSize<Impl: IWMPacketSize2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminpacketsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn PreDecode();
}
impl IWMPlayerHookVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPlayerHookImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPlayerHookVtbl {
        unsafe extern "system" fn PreDecode<Impl: IWMPlayerHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), PreDecode: PreDecode::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPlayerHook as ::windows::core::Interface>::IID
    }
}
pub trait IWMPlayerTimestampHookImpl: Sized {
    fn MapTimestamp();
}
impl IWMPlayerTimestampHookVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPlayerTimestampHookImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPlayerTimestampHookVtbl {
        unsafe extern "system" fn MapTimestamp<Impl: IWMPlayerTimestampHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rtin: i64, prtout: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), MapTimestamp: MapTimestamp::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMPlayerTimestampHook as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfileImpl: Sized {
    fn GetVersion();
    fn GetName();
    fn SetName();
    fn GetDescription();
    fn SetDescription();
    fn GetStreamCount();
    fn GetStream();
    fn GetStreamByNumber();
    fn RemoveStream();
    fn RemoveStreamByNumber();
    fn AddStream();
    fn ReconfigStream();
    fn CreateNewStream();
    fn GetMutualExclusionCount();
    fn GetMutualExclusion();
    fn RemoveMutualExclusion();
    fn AddMutualExclusion();
    fn CreateNewMutualExclusion();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfileVtbl {
        unsafe extern "system" fn GetVersion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetName<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR, pcchdescription: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamCount<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcstreams: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstreamindex: u32, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamByNumber<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStreamByNumber<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReconfigStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconfig: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNewStream<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidstreamtype: *const ::windows::core::GUID, ppconfig: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMutualExclusionCount<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcme: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMutualExclusion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmeindex: u32, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveMutualExclusion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMutualExclusion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pme: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNewMutualExclusion<Impl: IWMProfileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppme: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetProfileID();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfile2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfile2Vtbl {
        unsafe extern "system" fn GetProfileID<Impl: IWMProfile2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMProfileVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetProfileID: GetProfileID::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProfile2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMProfile3Impl: Sized + IWMProfileImpl + IWMProfile2Impl {
    fn GetStorageFormat();
    fn SetStorageFormat();
    fn GetBandwidthSharingCount();
    fn GetBandwidthSharing();
    fn RemoveBandwidthSharing();
    fn AddBandwidthSharing();
    fn CreateNewBandwidthSharing();
    fn GetStreamPrioritization();
    fn SetStreamPrioritization();
    fn RemoveStreamPrioritization();
    fn CreateNewStreamPrioritization();
    fn GetExpectedPacketCount();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfile3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfile3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfile3Vtbl {
        unsafe extern "system" fn GetStorageFormat<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstorageformat: *mut WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStorageFormat<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nstorageformat: WMT_STORAGE_FORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBandwidthSharingCount<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwbsindex: u32, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNewBandwidthSharing<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbs: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateNewStreamPrioritization<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExpectedPacketCount<Impl: IWMProfile3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msduration: u64, pcpackets: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn CreateEmptyProfile();
    fn LoadProfileByID();
    fn LoadProfileByData();
    fn SaveProfile();
    fn GetSystemProfileCount();
    fn LoadSystemProfile();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfileManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfileManagerVtbl {
        unsafe extern "system" fn CreateEmptyProfile<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadProfileByID<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadProfileByData<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprofile: super::super::Foundation::PWSTR, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveProfile<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmprofile: ::windows::core::RawPtr, pwszprofile: super::super::Foundation::PWSTR, pdwlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSystemProfileCount<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprofiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LoadSystemProfile<Impl: IWMProfileManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprofileindex: u32, ppprofile: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetSystemProfileVersion();
    fn SetSystemProfileVersion();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMProfileManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManager2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfileManager2Vtbl {
        unsafe extern "system" fn GetSystemProfileVersion<Impl: IWMProfileManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSystemProfileVersion<Impl: IWMProfileManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwversion: WMT_VERSION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetUserLanguageID();
    fn SetUserLanguageID();
}
impl IWMProfileManagerLanguageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProfileManagerLanguageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProfileManagerLanguageVtbl {
        unsafe extern "system" fn GetUserLanguageID<Impl: IWMProfileManagerLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wlangid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserLanguageID<Impl: IWMProfileManagerLanguageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wlangid: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetPropertyCount();
    fn GetPropertyByName();
    fn SetProperty();
    fn GetPropertyByIndex();
    fn CopyPropertiesFrom();
    fn Clear();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMPropertyVaultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMPropertyVaultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMPropertyVaultVtbl {
        unsafe extern "system" fn GetPropertyCount<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyByName<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszname: super::super::Foundation::PWSTR, ptype: WMT_ATTR_DATATYPE, pvalue: *const u8, dwsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyByIndex<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pszname: super::super::Foundation::PWSTR, pdwnamelen: *mut u32, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pdwsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyPropertiesFrom<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piwmpropertyvault: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: IWMPropertyVaultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn StartDetection();
}
impl IWMProximityDetectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMProximityDetectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMProximityDetectionVtbl {
        unsafe extern "system" fn StartDetection<Impl: IWMProximityDetectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbregistrationmsg: *const u8, cbregistrationmsg: u32, pblocaladdress: *const u8, cblocaladdress: u32, dwextraportsallowed: u32, ppregistrationresponsemsg: *mut ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), StartDetection: StartDetection::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMProximityDetection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderImpl: Sized {
    fn Open();
    fn Close();
    fn GetOutputCount();
    fn GetOutputProps();
    fn SetOutputProps();
    fn GetOutputFormatCount();
    fn GetOutputFormat();
    fn Start();
    fn Stop();
    fn Pause();
    fn Resume();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderVtbl {
        unsafe extern "system" fn Open<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputCount<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputProps<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputProps<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputFormatCount<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputFormat<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnumber: u32, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Start<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Pause<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IWMReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetCodecInterface();
    fn Notify();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderAcceleratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAcceleratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAcceleratorVtbl {
        unsafe extern "system" fn GetCodecInterface<Impl: IWMReaderAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, riid: *const ::windows::core::GUID, ppvcodecinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Notify<Impl: IWMReaderAcceleratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, psubtype: *const WM_MEDIA_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetUserProvidedClock();
    fn GetUserProvidedClock();
    fn DeliverTime();
    fn SetManualStreamSelection();
    fn GetManualStreamSelection();
    fn SetStreamsSelected();
    fn GetStreamSelected();
    fn SetReceiveSelectionCallbacks();
    fn GetReceiveSelectionCallbacks();
    fn SetReceiveStreamSamples();
    fn GetReceiveStreamSamples();
    fn SetAllocateForOutput();
    fn GetAllocateForOutput();
    fn SetAllocateForStream();
    fn GetAllocateForStream();
    fn GetStatistics();
    fn SetClientInfo();
    fn GetMaxOutputSampleSize();
    fn GetMaxStreamSampleSize();
    fn NotifyLateDelivery();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderAdvancedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvancedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvancedVtbl {
        unsafe extern "system" fn SetUserProvidedClock<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fuserclock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserProvidedClock<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuserclock: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeliverTime<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnstime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetManualStreamSelection<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fselection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetManualStreamSelection<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfselection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamsSelected<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamSelected<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReceiveSelectionCallbacks<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fgetcallbacks: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReceiveSelectionCallbacks<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfgetcallbacks: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReceiveStreamSamples<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivestreamsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReceiveStreamSamples<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivestreamsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllocateForOutput<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocateForOutput<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllocateForStream<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocateForStream<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatistics<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatistics: *mut WM_READER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientInfo<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclientinfo: *const WM_READER_CLIENTINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyLateDelivery<Impl: IWMReaderAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnslateness: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetPlayMode();
    fn GetPlayMode();
    fn GetBufferProgress();
    fn GetDownloadProgress();
    fn GetSaveAsProgress();
    fn SaveFileAs();
    fn GetProtocolName();
    fn StartAtMarker();
    fn GetOutputSetting();
    fn SetOutputSetting();
    fn Preroll();
    fn SetLogClientID();
    fn GetLogClientID();
    fn StopBuffering();
    fn OpenStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced2Vtbl {
        unsafe extern "system" fn SetPlayMode<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mode: WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPlayMode<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmode: *mut WMT_PLAY_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferProgress<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pcnsbuffering: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDownloadProgress<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32, pqwbytesdownloaded: *mut u64, pcnsdownload: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSaveAsProgress<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpercent: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveFileAs<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProtocolName<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pcchprotocol: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartAtMarker<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmarkerindex: u16, cnsduration: u64, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputSetting<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputSetting<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Preroll<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstart: u64, cnsduration: u64, frate: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLogClientID<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flogclientid: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLogClientID<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflogclientid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopBuffering<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenStream<Impl: IWMReaderAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn StopNetStreaming();
    fn StartAtPosition();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced3Vtbl {
        unsafe extern "system" fn StopNetStreaming<Impl: IWMReaderAdvanced3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartAtPosition<Impl: IWMReaderAdvanced3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pvoffsetstart: *const ::core::ffi::c_void, pvduration: *const ::core::ffi::c_void, dwoffsetformat: WMT_OFFSET_FORMAT, frate: f32, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetLanguageCount();
    fn GetLanguage();
    fn GetMaxSpeedFactor();
    fn IsUsingFastCache();
    fn AddLogParam();
    fn SendLogParams();
    fn CanSaveFileAs();
    fn CancelSaveFileAs();
    fn GetURL();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced4Vtbl {
        unsafe extern "system" fn GetLanguageCount<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwlanguagecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLanguage<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, wlanguage: u16, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxSpeedFactor<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdblfactor: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUsingFastCache<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfusingfastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddLogParam<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wsznamespace: super::super::Foundation::PWSTR, wszname: super::super::Foundation::PWSTR, wszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendLogParams<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanSaveFileAs<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcansave: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelSaveFileAs<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetURL<Impl: IWMReaderAdvanced4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetPlayerHook();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced5Vtbl {
        unsafe extern "system" fn SetPlayerHook<Impl: IWMReaderAdvanced5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, phook: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMReaderAdvanced4Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetPlayerHook: SetPlayerHook::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderAdvanced5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IWMReaderAdvanced6Impl: Sized + IWMReaderAdvancedImpl + IWMReaderAdvanced2Impl + IWMReaderAdvanced3Impl + IWMReaderAdvanced4Impl + IWMReaderAdvanced5Impl {
    fn SetProtectStreamSamples();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMReaderAdvanced6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAdvanced6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAdvanced6Vtbl {
        unsafe extern "system" fn SetProtectStreamSamples<Impl: IWMReaderAdvanced6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcertificate: *const u8, cbcertificate: u32, dwcertificatetype: u32, dwflags: u32, pbinitializationvector: *mut u8, pcbinitializationvector: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn AllocateForStreamEx();
    fn AllocateForOutputEx();
}
impl IWMReaderAllocatorExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderAllocatorExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderAllocatorExVtbl {
        unsafe extern "system" fn AllocateForStreamEx<Impl: IWMReaderAllocatorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllocateForOutputEx<Impl: IWMReaderAllocatorExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, dwflags: u32, cnssampletime: u64, cnssampleduration: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn OnSample();
}
impl IWMReaderCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderCallbackVtbl {
        unsafe extern "system" fn OnSample<Impl: IWMReaderCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMStatusCallbackVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), OnSample: OnSample::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMReaderCallbackAdvancedImpl: Sized {
    fn OnStreamSample();
    fn OnTime();
    fn OnStreamSelection();
    fn OnOutputPropsChanged();
    fn AllocateForStream();
    fn AllocateForOutput();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderCallbackAdvancedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderCallbackAdvancedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderCallbackAdvancedVtbl {
        unsafe extern "system" fn OnStreamSample<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTime<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnscurrenttime: u64, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnStreamSelection<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamcount: u16, pstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnOutputPropsChanged<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pmediatype: *const WM_MEDIA_TYPE, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllocateForStream<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllocateForOutput<Impl: IWMReaderCallbackAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetBufferingTime();
    fn SetBufferingTime();
    fn GetUDPPortRanges();
    fn SetUDPPortRanges();
    fn GetProxySettings();
    fn SetProxySettings();
    fn GetProxyHostName();
    fn SetProxyHostName();
    fn GetProxyPort();
    fn SetProxyPort();
    fn GetProxyExceptionList();
    fn SetProxyExceptionList();
    fn GetProxyBypassForLocal();
    fn SetProxyBypassForLocal();
    fn GetForceRerunAutoProxyDetection();
    fn SetForceRerunAutoProxyDetection();
    fn GetEnableMulticast();
    fn SetEnableMulticast();
    fn GetEnableHTTP();
    fn SetEnableHTTP();
    fn GetEnableUDP();
    fn SetEnableUDP();
    fn GetEnableTCP();
    fn SetEnableTCP();
    fn ResetProtocolRollover();
    fn GetConnectionBandwidth();
    fn SetConnectionBandwidth();
    fn GetNumProtocolsSupported();
    fn GetSupportedProtocolName();
    fn AddLoggingUrl();
    fn GetLoggingUrl();
    fn GetLoggingUrlCount();
    fn ResetLoggingUrlList();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderNetworkConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderNetworkConfigVtbl {
        unsafe extern "system" fn GetBufferingTime<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsbufferingtime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBufferingTime<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsbufferingtime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUDPPortRanges<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangearray: *mut WM_PORT_NUMBER_RANGE, pcranges: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUDPPortRanges<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prangearray: *const WM_PORT_NUMBER_RANGE, cranges: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProxySettings<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pproxysetting: *mut WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProxySettings<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, proxysetting: WMT_PROXY_SETTINGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProxyHostName<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR, pcchhostname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProxyHostName<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszhostname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProxyPort<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProxyPort<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, dwport: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProxyExceptionList<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR, pcchexceptionlist: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProxyExceptionList<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pwszexceptionlist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProxyBypassForLocal<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, pfbypassforlocal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProxyBypassForLocal<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszprotocol: super::super::Foundation::PWSTR, fbypassforlocal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetForceRerunAutoProxyDetection<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfforcererundetection: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetForceRerunAutoProxyDetection<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforcererundetection: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnableMulticast<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablemulticast: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableMulticast<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablemulticast: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnableHTTP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablehttp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableHTTP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablehttp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnableUDP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableudp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableUDP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableudp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnableTCP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabletcp: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableTCP<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabletcp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetProtocolRollover<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectionBandwidth<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconnectionbandwidth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectionBandwidth<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnectionbandwidth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumProtocolsSupported<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcprotocols: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedProtocolName<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprotocolnum: u32, pwszprotocolname: super::super::Foundation::PWSTR, pcchprotocolname: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddLoggingUrl<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLoggingUrl<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLoggingUrlCount<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwurlcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetLoggingUrlList<Impl: IWMReaderNetworkConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetEnableContentCaching();
    fn SetEnableContentCaching();
    fn GetEnableFastCache();
    fn SetEnableFastCache();
    fn GetAcceleratedStreamingDuration();
    fn SetAcceleratedStreamingDuration();
    fn GetAutoReconnectLimit();
    fn SetAutoReconnectLimit();
    fn GetEnableResends();
    fn SetEnableResends();
    fn GetEnableThinning();
    fn SetEnableThinning();
    fn GetMaxNetPacketSize();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderNetworkConfig2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderNetworkConfig2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderNetworkConfig2Vtbl {
        unsafe extern "system" fn GetEnableContentCaching<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablecontentcaching: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableContentCaching<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablecontentcaching: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnableFastCache<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablefastcache: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableFastCache<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablefastcache: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAcceleratedStreamingDuration<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsaccelduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAcceleratedStreamingDuration<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsaccelduration: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutoReconnectLimit<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwautoreconnectlimit: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAutoReconnectLimit<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwautoreconnectlimit: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnableResends<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenableresends: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableResends<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenableresends: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEnableThinning<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenablethinning: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnableThinning<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenablethinning: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxNetPacketSize<Impl: IWMReaderNetworkConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxnetpacketsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn InitPlaylistBurn();
    fn GetInitResults();
    fn Cancel();
    fn EndPlaylistBurn();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMReaderPlaylistBurnVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderPlaylistBurnImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderPlaylistBurnVtbl {
        unsafe extern "system" fn InitPlaylistBurn<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfiles: u32, ppwszfilenames: *const super::super::Foundation::PWSTR, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInitResults<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cfiles: u32, phrstati: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndPlaylistBurn<Impl: IWMReaderPlaylistBurnImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrburnresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetTime();
    fn SetTimer();
    fn KillTimer();
}
impl IWMReaderStreamClockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderStreamClockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderStreamClockVtbl {
        unsafe extern "system" fn GetTime<Impl: IWMReaderStreamClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsnow: *const u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimer<Impl: IWMReaderStreamClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnswhen: u64, pvparam: *const ::core::ffi::c_void, pdwtimerid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn KillTimer<Impl: IWMReaderStreamClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimerid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetTimecodeRangeCount();
    fn GetTimecodeRangeBounds();
}
impl IWMReaderTimecodeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderTimecodeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderTimecodeVtbl {
        unsafe extern "system" fn GetTimecodeRangeCount<Impl: IWMReaderTimecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pwrangecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimecodeRangeBounds<Impl: IWMReaderTimecodeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, wrangenum: u16, pstarttimecode: *mut u32, pendtimecode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn TryOutputProps();
}
impl IWMReaderTypeNegotiationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMReaderTypeNegotiationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMReaderTypeNegotiationVtbl {
        unsafe extern "system" fn TryOutputProps<Impl: IWMReaderTypeNegotiationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TryOutputProps: TryOutputProps::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMReaderTypeNegotiation as ::windows::core::Interface>::IID
    }
}
pub trait IWMRegisterCallbackImpl: Sized {
    fn Advise();
    fn Unadvise();
}
impl IWMRegisterCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisterCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMRegisterCallbackVtbl {
        unsafe extern "system" fn Advise<Impl: IWMRegisterCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IWMRegisterCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetDeviceSerialNumber();
    fn GetDeviceCertificate();
    fn GetDeviceType();
    fn GetAttributeCount();
    fn GetAttributeByIndex();
    fn GetAttributeByName();
    fn SetAttributeByName();
    fn Approve();
    fn IsValid();
    fn IsApproved();
    fn IsWmdrmCompliant();
    fn IsOpened();
    fn Open();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMRegisteredDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMRegisteredDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMRegisteredDeviceVtbl {
        unsafe extern "system" fn GetDeviceSerialNumber<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pserialnumber: *mut DRM_VAL16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceCertificate<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertificate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceType<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeCount<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcattributes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeByIndex<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pbstrname: *mut super::super::Foundation::BSTR, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeByName<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAttributeByName<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Approve<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fapprove: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsValid<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsApproved<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfapproved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWmdrmCompliant<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfcompliant: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsOpened<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfopened: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWMRegisteredDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn AllocateBuffer();
    fn AllocatePageSizeBuffer();
}
impl IWMSBufferAllocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSBufferAllocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSBufferAllocatorVtbl {
        unsafe extern "system" fn AllocateBuffer<Impl: IWMSBufferAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllocatePageSizeBuffer<Impl: IWMSBufferAllocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxbuffersize: u32, ppbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Initialize();
    fn GetNetSourceCreator();
    fn SetCredentials();
    fn GetCredentials();
    fn DeleteCredentials();
    fn GetCredentialFlags();
    fn SetCredentialFlags();
    fn FindProxyForURL();
    fn RegisterProxyFailure();
    fn ShutdownProxyContext();
    fn IsUsingIE();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSInternalAdminNetSourceVtbl {
        unsafe extern "system" fn Initialize<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharednamespace: *mut ::core::ffi::c_void, pnamespacenode: *mut ::core::ffi::c_void, pnetsourcecreator: ::windows::core::RawPtr, fembeddedinserver: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetSourceCreator<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCredentials<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCredentials<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteCredentials<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCredentialFlags<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCredentialFlags<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindProxyForURL<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterProxyFailure<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShutdownProxyContext<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUsingIE<Impl: IWMSInternalAdminNetSourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproxycontext: u32, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetCredentialsEx();
    fn GetCredentialsEx();
    fn DeleteCredentialsEx();
    fn FindProxyForURLEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSInternalAdminNetSource2Vtbl {
        unsafe extern "system" fn SetCredentialsEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCredentialsEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteCredentialsEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindProxyForURLEx<Impl: IWMSInternalAdminNetSource2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pdwproxycontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetNetSourceCreator2();
    fn FindProxyForURLEx2();
    fn RegisterProxyFailure2();
    fn ShutdownProxyContext2();
    fn IsUsingIE2();
    fn SetCredentialsEx2();
    fn GetCredentialsEx2();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSInternalAdminNetSource3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSInternalAdminNetSource3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSInternalAdminNetSource3Vtbl {
        unsafe extern "system" fn GetNetSourceCreator2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetsourcecreator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindProxyForURLEx2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprotocol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrhost: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfproxyenabled: *mut super::super::Foundation::BOOL, pbstrproxyserver: *mut super::super::Foundation::BSTR, pdwproxyport: *mut u32, pqwproxycontext: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterProxyFailure2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrparam: ::windows::core::HRESULT, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShutdownProxyContext2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUsingIE2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, qwproxycontext: u64, pfisusingie: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCredentialsEx2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fpersist: super::super::Foundation::BOOL, fconfirmedgood: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCredentialsEx2<Impl: IWMSInternalAdminNetSource3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrealm: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fproxy: super::super::Foundation::BOOL, fcleartextauthentication: super::super::Foundation::BOOL, pdwurlpolicy: *mut NETSOURCE_URLCREDPOLICY_SETTINGS, pbstrname: *mut super::super::Foundation::BSTR, pbstrpassword: *mut super::super::Foundation::BSTR, pfconfirmedgood: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn WMSC_AddCertificate();
    fn WMSC_AddSignature();
    fn WMSC_Connect();
    fn WMSC_IsConnected();
    fn WMSC_Disconnect();
    fn WMSC_GetValidCertificate();
    fn WMSC_Encrypt();
    fn WMSC_Decrypt();
    fn WMSC_Lock();
    fn WMSC_Unlock();
    fn WMSC_SetSharedData();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMSecureChannelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSecureChannelImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSecureChannelVtbl {
        unsafe extern "system" fn WMSC_AddCertificate<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcert: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_AddSignature<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcertsig: *const u8, cbcertsig: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_Connect<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, potherside: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_IsConnected<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_Disconnect<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_GetValidCertificate<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbcertificate: *mut *mut u8, pdwsignature: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_Encrypt<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_Decrypt<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *const u8, cbdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_Lock<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_Unlock<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WMSC_SetSharedData<Impl: IWMSecureChannelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcertindex: u32, pbshareddata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn OnStatus();
}
impl IWMStatusCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStatusCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStatusCallbackVtbl {
        unsafe extern "system" fn OnStatus<Impl: IWMStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: WMT_STATUS, hr: ::windows::core::HRESULT, dwtype: WMT_ATTR_DATATYPE, pvalue: *const u8, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnStatus: OnStatus::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMStatusCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMStreamConfigImpl: Sized {
    fn GetStreamType();
    fn GetStreamNumber();
    fn SetStreamNumber();
    fn GetStreamName();
    fn SetStreamName();
    fn GetConnectionName();
    fn SetConnectionName();
    fn GetBitrate();
    fn SetBitrate();
    fn GetBufferWindow();
    fn SetBufferWindow();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamConfigVtbl {
        unsafe extern "system" fn GetStreamType<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidstreamtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamNumber<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamNumber<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamName<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR, pcchstreamname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamName<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszstreamname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectionName<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR, pcchinputname: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConnectionName<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinputname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBitrate<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBitrate<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwbitrate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBufferWindow<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsbufferwindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBufferWindow<Impl: IWMStreamConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msbufferwindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetTransportType();
    fn SetTransportType();
    fn AddDataUnitExtension();
    fn GetDataUnitExtensionCount();
    fn GetDataUnitExtension();
    fn RemoveAllDataUnitExtensions();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamConfig2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamConfig2Vtbl {
        unsafe extern "system" fn GetTransportType<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntransporttype: *mut WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransportType<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ntransporttype: WMT_TRANSPORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDataUnitExtension<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidextensionsystemid: ::windows::core::GUID, cbextensiondatasize: u16, pbextensionsysteminfo: *const u8, cbextensionsysteminfo: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataUnitExtensionCount<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdataunitextensions: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataUnitExtension<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wdataunitextensionnumber: u16, pguidextensionsystemid: *mut ::windows::core::GUID, pcbextensiondatasize: *mut u16, pbextensionsysteminfo: *mut u8, pcbextensionsysteminfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAllDataUnitExtensions<Impl: IWMStreamConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetLanguage();
    fn SetLanguage();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamConfig3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamConfig3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamConfig3Vtbl {
        unsafe extern "system" fn GetLanguage<Impl: IWMStreamConfig3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR, pcchlanguagestringlength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLanguage<Impl: IWMStreamConfig3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszlanguagestring: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetStreams();
    fn AddStream();
    fn RemoveStream();
}
impl IWMStreamListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamListVtbl {
        unsafe extern "system" fn GetStreams<Impl: IWMStreamListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstreamnumarray: *mut u16, pcstreams: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStream<Impl: IWMStreamListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStream<Impl: IWMStreamListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetPriorityRecords();
    fn SetPriorityRecords();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMStreamPrioritizationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMStreamPrioritizationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMStreamPrioritizationVtbl {
        unsafe extern "system" fn GetPriorityRecords<Impl: IWMStreamPrioritizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordarray: *mut WM_STREAM_PRIORITY_RECORD, pcrecords: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriorityRecords<Impl: IWMStreamPrioritizationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precordarray: *const WM_STREAM_PRIORITY_RECORD, crecords: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Open();
    fn Close();
    fn SetRange();
    fn SetRangeByFrame();
    fn GetNextSample();
    fn SetStreamsSelected();
    fn GetStreamSelected();
    fn SetReadStreamSamples();
    fn GetReadStreamSamples();
    fn GetOutputSetting();
    fn SetOutputSetting();
    fn GetOutputCount();
    fn GetOutputProps();
    fn SetOutputProps();
    fn GetOutputFormatCount();
    fn GetOutputFormat();
    fn GetOutputNumberForStream();
    fn GetStreamNumberForOutput();
    fn GetMaxOutputSampleSize();
    fn GetMaxStreamSampleSize();
    fn OpenStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMSyncReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSyncReaderVtbl {
        unsafe extern "system" fn Open<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRange<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64, cnsduration: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRangeByFrame<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextSample<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, ppsample: *mut ::windows::core::RawPtr, pcnssampletime: *mut u64, pcnsduration: *mut u64, pdwflags: *mut u32, pdwoutputnum: *mut u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStreamsSelected<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cstreamcount: u16, pwstreamnumbers: *const u16, pselections: *const WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamSelected<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pselection: *mut WMT_STREAM_SELECTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReadStreamSamples<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, fcompressed: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReadStreamSamples<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfcompressed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputSetting<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputSetting<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputCount<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcoutputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputProps<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputProps<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputFormatCount<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputFormat<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, dwformatnum: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputNumberForStream<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pdwoutputnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStreamNumberForOutput<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pwstreamnum: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxOutputSampleSize<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutput: u32, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxStreamSampleSize<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstream: u16, pcbmax: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenStream<Impl: IWMSyncReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetRangeByTimecode();
    fn SetRangeByFrameEx();
    fn SetAllocateForOutput();
    fn GetAllocateForOutput();
    fn SetAllocateForStream();
    fn GetAllocateForStream();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IWMSyncReader2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMSyncReader2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMSyncReader2Vtbl {
        unsafe extern "system" fn SetRangeByTimecode<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstart: *const WMT_TIMECODE_EXTENSION_DATA, pend: *const WMT_TIMECODE_EXTENSION_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRangeByFrameEx<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, qwframenumber: u64, cframestoread: i64, pcnsstarttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllocateForOutput<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocateForOutput<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoutputnum: u32, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllocateForStream<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pallocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocateForStream<Impl: IWMSyncReader2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsreamnum: u16, ppallocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetMaxKeyFrameSpacing();
    fn SetMaxKeyFrameSpacing();
    fn GetQuality();
    fn SetQuality();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMVideoMediaPropsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMVideoMediaPropsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMVideoMediaPropsVtbl {
        unsafe extern "system" fn GetMaxKeyFrameSpacing<Impl: IWMVideoMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plltime: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxKeyFrameSpacing<Impl: IWMVideoMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lltime: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuality<Impl: IWMVideoMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwquality: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuality<Impl: IWMVideoMediaPropsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwquality: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetWatermarkEntryCount();
    fn GetWatermarkEntry();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWatermarkInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWatermarkInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWatermarkInfoVtbl {
        unsafe extern "system" fn GetWatermarkEntryCount<Impl: IWMWatermarkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWatermarkEntry<Impl: IWMWatermarkInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wmettype: WMT_WATERMARK_ENTRY_TYPE, dwentrynum: u32, pentry: *mut WMT_WATERMARK_ENTRY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetProfileByID();
    fn SetProfile();
    fn SetOutputFilename();
    fn GetInputCount();
    fn GetInputProps();
    fn SetInputProps();
    fn GetInputFormatCount();
    fn GetInputFormat();
    fn BeginWriting();
    fn EndWriting();
    fn AllocateSample();
    fn WriteSample();
    fn Flush();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterVtbl {
        unsafe extern "system" fn SetProfileByID<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidprofile: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProfile<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprofile: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOutputFilename<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputCount<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinputs: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputProps<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, ppinput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputProps<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pinput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputFormatCount<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInputFormat<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnumber: u32, dwformatnumber: u32, pprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginWriting<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndWriting<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllocateSample<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsamplesize: u32, ppsample: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteSample<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Flush<Impl: IWMWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetSinkCount();
    fn GetSink();
    fn AddSink();
    fn RemoveSink();
    fn WriteStreamSample();
    fn SetLiveSource();
    fn IsRealTime();
    fn GetWriterTime();
    fn GetStatistics();
    fn SetSyncTolerance();
    fn GetSyncTolerance();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvancedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvancedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterAdvancedVtbl {
        unsafe extern "system" fn GetSinkCount<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcsinks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSink<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsinknum: u32, ppsink: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSink<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveSink<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteStreamSample<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cnssampletime: u64, mssamplesendtime: u32, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLiveSource<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fislivesource: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRealTime<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWriterTime<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnscurrenttime: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatistics<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSyncTolerance<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mswindow: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSyncTolerance<Impl: IWMWriterAdvancedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmswindow: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetInputSetting();
    fn SetInputSetting();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterAdvanced2Vtbl {
        unsafe extern "system" fn GetInputSetting<Impl: IWMWriterAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, ptype: *mut WMT_ATTR_DATATYPE, pvalue: *mut u8, pcblength: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInputSetting<Impl: IWMWriterAdvanced2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, pszname: super::super::Foundation::PWSTR, r#type: WMT_ATTR_DATATYPE, pvalue: *const u8, cblength: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetStatisticsEx();
    fn SetNonBlocking();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterAdvanced3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterAdvanced3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterAdvanced3Vtbl {
        unsafe extern "system" fn GetStatisticsEx<Impl: IWMWriterAdvanced3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pstats: *mut WM_WRITER_STATISTICS_EX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNonBlocking<Impl: IWMWriterAdvanced3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Open();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterFileSinkVtbl {
        unsafe extern "system" fn Open<Impl: IWMWriterFileSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IWMWriterSinkVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Open: Open::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWMWriterFileSink as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWMWriterFileSink2Impl: Sized + IWMWriterSinkImpl + IWMWriterFileSinkImpl {
    fn Start();
    fn Stop();
    fn IsStopped();
    fn GetFileDuration();
    fn GetFileSize();
    fn Close();
    fn IsClosed();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterFileSink2Vtbl {
        unsafe extern "system" fn Start<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstarttime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cnsstoptime: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsStopped<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfstopped: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileDuration<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcnsduration: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFileSize<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbfile: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsClosed<Impl: IWMWriterFileSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfclosed: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetAutoIndexing();
    fn GetAutoIndexing();
    fn SetControlStream();
    fn GetMode();
    fn OnDataUnitEx();
    fn SetUnbufferedIO();
    fn GetUnbufferedIO();
    fn CompleteOperations();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterFileSink3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterFileSink3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterFileSink3Vtbl {
        unsafe extern "system" fn SetAutoIndexing<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fdoautoindexing: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAutoIndexing<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfautoindexing: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetControlStream<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fshouldcontrolstartandstop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMode<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfilesinkmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDataUnitEx<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfilesinkdataunit: *const WMT_FILESINK_DATA_UNIT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUnbufferedIO<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, funbufferedio: super::super::Foundation::BOOL, frestrictmemusage: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUnbufferedIO<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfunbufferedio: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CompleteOperations<Impl: IWMWriterFileSink3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetMaximumClients();
    fn GetMaximumClients();
    fn SetNetworkProtocol();
    fn GetNetworkProtocol();
    fn GetHostURL();
    fn Open();
    fn Disconnect();
    fn Close();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterNetworkSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterNetworkSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterNetworkSinkVtbl {
        unsafe extern "system" fn SetMaximumClients<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxclients: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumClients<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxclients: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNetworkProtocol<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protocol: WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNetworkProtocol<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprotocol: *mut WMT_NET_PROTOCOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHostURL<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Open<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwportnum: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IWMWriterNetworkSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn SetPostViewCallback();
    fn SetReceivePostViewSamples();
    fn GetReceivePostViewSamples();
    fn GetPostViewProps();
    fn SetPostViewProps();
    fn GetPostViewFormatCount();
    fn GetPostViewFormat();
    fn SetAllocateForPostView();
    fn GetAllocateForPostView();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterPostViewVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostViewImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterPostViewVtbl {
        unsafe extern "system" fn SetPostViewCallback<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pvcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetReceivePostViewSamples<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, freceivepostviewsamples: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetReceivePostViewSamples<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, pfreceivepostviewsamples: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPostViewProps<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostViewProps<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, poutput: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPostViewFormatCount<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pcformats: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPostViewFormat<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, dwformatnumber: u32, ppprops: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllocateForPostView<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, fallocate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllocateForPostView<Impl: IWMWriterPostViewImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, pfallocate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn OnPostViewSample();
    fn AllocateForPostView();
}
impl IWMWriterPostViewCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPostViewCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterPostViewCallbackVtbl {
        unsafe extern "system" fn OnPostViewSample<Impl: IWMWriterPostViewCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnumber: u16, cnssampletime: u64, cnssampleduration: u64, dwflags: u32, psample: ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllocateForPostView<Impl: IWMWriterPostViewCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wstreamnum: u16, cbbuffer: u32, ppbuffer: *mut ::windows::core::RawPtr, pvcontext: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn GetMaxPreprocessingPasses();
    fn SetNumPreprocessingPasses();
    fn BeginPreprocessingPass();
    fn PreprocessSample();
    fn EndPreprocessingPass();
}
impl IWMWriterPreprocessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPreprocessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterPreprocessVtbl {
        unsafe extern "system" fn GetMaxPreprocessingPasses<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, pdwmaxnumpasses: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNumPreprocessingPasses<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32, dwnumpasses: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginPreprocessingPass<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreprocessSample<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, cnssampletime: u64, dwflags: u32, psample: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndPreprocessingPass<Impl: IWMWriterPreprocessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinputnum: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn Connect();
    fn Disconnect();
    fn EndSession();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterPushSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterPushSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterPushSinkVtbl {
        unsafe extern "system" fn Connect<Impl: IWMWriterPushSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::Foundation::PWSTR, pwsztemplateurl: super::super::Foundation::PWSTR, fautodestroy: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IWMWriterPushSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndSession<Impl: IWMWriterPushSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
    fn OnHeader();
    fn IsRealTime();
    fn AllocateDataUnit();
    fn OnDataUnit();
    fn OnEndWriting();
}
#[cfg(feature = "Win32_Foundation")]
impl IWMWriterSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWMWriterSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWMWriterSinkVtbl {
        unsafe extern "system" fn OnHeader<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRealTime<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfrealtime: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllocateDataUnit<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbdataunit: u32, ppdataunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDataUnit<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataunit: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEndWriting<Impl: IWMWriterSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
