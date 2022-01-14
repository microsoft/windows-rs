#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISideShowBulkCapabilities_Impl: Sized + ISideShowCapabilities_Impl {
    fn GetCapabilities(&mut self, in_keycollection: &::core::option::Option<ISideShowKeyCollection>, inout_pvalues: *mut ::core::option::Option<ISideShowPropVariantCollection>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISideShowBulkCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowBulkCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowBulkCapabilities_Vtbl {
        unsafe extern "system" fn GetCapabilities<Impl: ISideShowBulkCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_keycollection: ::windows::core::RawPtr, inout_pvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapabilities(::core::mem::transmute(&in_keycollection), ::core::mem::transmute_copy(&inout_pvalues)).into()
        }
        Self { base: ISideShowCapabilities_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetCapabilities: GetCapabilities::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowBulkCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISideShowCapabilities_Impl: Sized {
    fn GetCapability(&mut self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISideShowCapabilities_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowCapabilities_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowCapabilities_Vtbl {
        unsafe extern "system" fn GetCapability<Impl: ISideShowCapabilities_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCapability(::core::mem::transmute_copy(&in_keycapability), ::core::mem::transmute_copy(&inout_pvalue)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCapability: GetCapability::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowCapabilities as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowCapabilitiesCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, in_dwindex: u32) -> ::windows::core::Result<ISideShowCapabilities>;
}
impl ISideShowCapabilitiesCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowCapabilitiesCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowCapabilitiesCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *out_pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_dwindex: u32, out_ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&in_dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *out_ppcapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCount: GetCount::<Impl, IMPL_OFFSET>, GetAt: GetAt::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowCapabilitiesCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISideShowContent_Impl: Sized {
    fn GetContent(&mut self, in_picapabilities: &::core::option::Option<ISideShowCapabilities>, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows::core::Result<()>;
    fn ContentId(&mut self) -> ::windows::core::Result<u32>;
    fn DifferentiateContent(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISideShowContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowContent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowContent_Vtbl {
        unsafe extern "system" fn GetContent<Impl: ISideShowContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_picapabilities: ::windows::core::RawPtr, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContent(::core::mem::transmute(&in_picapabilities), ::core::mem::transmute_copy(&out_pdwsize), ::core::mem::transmute_copy(&out_ppbdata)).into()
        }
        unsafe extern "system" fn ContentId<Impl: ISideShowContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentId() {
                ::core::result::Result::Ok(ok__) => {
                    *out_pcontentid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DifferentiateContent<Impl: ISideShowContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DifferentiateContent() {
                ::core::result::Result::Ok(ok__) => {
                    *out_pfdifferentiatecontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetContent: GetContent::<Impl, IMPL_OFFSET>,
            ContentId: ContentId::<Impl, IMPL_OFFSET>,
            DifferentiateContent: DifferentiateContent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowContent as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowContentManager_Impl: Sized {
    fn Add(&mut self, in_picontent: &::core::option::Option<ISideShowContent>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, in_contentid: u32) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
    fn SetEventSink(&mut self, in_pievents: &::core::option::Option<ISideShowEvents>) -> ::windows::core::Result<()>;
    fn GetDeviceCapabilities(&mut self) -> ::windows::core::Result<ISideShowCapabilitiesCollection>;
}
impl ISideShowContentManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowContentManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowContentManager_Vtbl {
        unsafe extern "system" fn Add<Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_picontent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&in_picontent)).into()
        }
        unsafe extern "system" fn Remove<Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_contentid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&in_contentid)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        unsafe extern "system" fn SetEventSink<Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pievents: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventSink(::core::mem::transmute(&in_pievents)).into()
        }
        unsafe extern "system" fn GetDeviceCapabilities<Impl: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *out_ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
            SetEventSink: SetEventSink::<Impl, IMPL_OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowContentManager as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowEvents_Impl: Sized {
    fn ContentMissing(&mut self, in_contentid: u32) -> ::windows::core::Result<ISideShowContent>;
    fn ApplicationEvent(&mut self, in_picapabilities: &::core::option::Option<ISideShowCapabilities>, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows::core::Result<()>;
    fn DeviceAdded(&mut self, in_pidevice: &::core::option::Option<ISideShowCapabilities>) -> ::windows::core::Result<()>;
    fn DeviceRemoved(&mut self, in_pidevice: &::core::option::Option<ISideShowCapabilities>) -> ::windows::core::Result<()>;
}
impl ISideShowEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowEvents_Vtbl {
        unsafe extern "system" fn ContentMissing<Impl: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_contentid: u32, out_ppicontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentMissing(::core::mem::transmute_copy(&in_contentid)) {
                ::core::result::Result::Ok(ok__) => {
                    *out_ppicontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationEvent<Impl: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_picapabilities: ::windows::core::RawPtr, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplicationEvent(::core::mem::transmute(&in_picapabilities), ::core::mem::transmute_copy(&in_dweventid), ::core::mem::transmute_copy(&in_dweventsize), ::core::mem::transmute_copy(&in_pbeventdata)).into()
        }
        unsafe extern "system" fn DeviceAdded<Impl: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pidevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceAdded(::core::mem::transmute(&in_pidevice)).into()
        }
        unsafe extern "system" fn DeviceRemoved<Impl: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pidevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeviceRemoved(::core::mem::transmute(&in_pidevice)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ContentMissing: ContentMissing::<Impl, IMPL_OFFSET>,
            ApplicationEvent: ApplicationEvent::<Impl, IMPL_OFFSET>,
            DeviceAdded: DeviceAdded::<Impl, IMPL_OFFSET>,
            DeviceRemoved: DeviceRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISideShowKeyCollection_Impl: Sized {
    fn Add(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn GetAt(&mut self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
    fn GetCount(&mut self, pcelems: *const u32) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, dwindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISideShowKeyCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowKeyCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowKeyCollection_Vtbl {
        unsafe extern "system" fn Add<Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn Clear<Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn GetAt<Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn GetCount<Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowKeyCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ISideShowNotification_Impl: Sized {
    fn NotificationId(&mut self) -> ::windows::core::Result<u32>;
    fn SetNotificationId(&mut self, in_notificationid: u32) -> ::windows::core::Result<()>;
    fn Title(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetTitle(&mut self, in_pwsztitle: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Message(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetMessage(&mut self, in_pwszmessage: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Image(&mut self) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn SetImage(&mut self, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::Result<()>;
    fn ExpirationTime(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetExpirationTime(&mut self, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ISideShowNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowNotification_Vtbl {
        unsafe extern "system" fn NotificationId<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pnotificationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationId() {
                ::core::result::Result::Ok(ok__) => {
                    *out_pnotificationid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationId<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationId(::core::mem::transmute_copy(&in_notificationid)).into()
        }
        unsafe extern "system" fn Title<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ppwsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title() {
                ::core::result::Result::Ok(ok__) => {
                    *out_ppwsztitle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pwsztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTitle(::core::mem::transmute_copy(&in_pwsztitle)).into()
        }
        unsafe extern "system" fn Message<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ppwszmessage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message() {
                ::core::result::Result::Ok(ok__) => {
                    *out_ppwszmessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessage<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pwszmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessage(::core::mem::transmute_copy(&in_pwszmessage)).into()
        }
        unsafe extern "system" fn Image<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image() {
                ::core::result::Result::Ok(ok__) => {
                    *out_phicon = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetImage(::core::mem::transmute_copy(&in_hicon)).into()
        }
        unsafe extern "system" fn ExpirationTime<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *out_ptime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExpirationTime(::core::mem::transmute_copy(&in_ptime)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            NotificationId: NotificationId::<Impl, IMPL_OFFSET>,
            SetNotificationId: SetNotificationId::<Impl, IMPL_OFFSET>,
            Title: Title::<Impl, IMPL_OFFSET>,
            SetTitle: SetTitle::<Impl, IMPL_OFFSET>,
            Message: Message::<Impl, IMPL_OFFSET>,
            SetMessage: SetMessage::<Impl, IMPL_OFFSET>,
            Image: Image::<Impl, IMPL_OFFSET>,
            SetImage: SetImage::<Impl, IMPL_OFFSET>,
            ExpirationTime: ExpirationTime::<Impl, IMPL_OFFSET>,
            SetExpirationTime: SetExpirationTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowNotification as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowNotificationManager_Impl: Sized {
    fn Show(&mut self, in_pinotification: &::core::option::Option<ISideShowNotification>) -> ::windows::core::Result<()>;
    fn Revoke(&mut self, in_notificationid: u32) -> ::windows::core::Result<()>;
    fn RevokeAll(&mut self) -> ::windows::core::Result<()>;
}
impl ISideShowNotificationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowNotificationManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowNotificationManager_Vtbl {
        unsafe extern "system" fn Show<Impl: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pinotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Show(::core::mem::transmute(&in_pinotification)).into()
        }
        unsafe extern "system" fn Revoke<Impl: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Revoke(::core::mem::transmute_copy(&in_notificationid)).into()
        }
        unsafe extern "system" fn RevokeAll<Impl: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RevokeAll().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Show: Show::<Impl, IMPL_OFFSET>,
            Revoke: Revoke::<Impl, IMPL_OFFSET>,
            RevokeAll: RevokeAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowNotificationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISideShowPropVariantCollection_Impl: Sized {
    fn Add(&mut self, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
    fn GetAt(&mut self, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn GetCount(&mut self, pcelems: *const u32) -> ::windows::core::Result<()>;
    fn RemoveAt(&mut self, dwindex: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISideShowPropVariantCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowPropVariantCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowPropVariantCollection_Vtbl {
        unsafe extern "system" fn Add<Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Clear<Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        unsafe extern "system" fn GetAt<Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAt(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetCount<Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCount(::core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn RemoveAt<Impl: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAt(::core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            RemoveAt: RemoveAt::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowPropVariantCollection as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowSession_Impl: Sized {
    fn RegisterContent(&mut self, in_applicationid: *const ::windows::core::GUID, in_endpointid: *const ::windows::core::GUID) -> ::windows::core::Result<ISideShowContentManager>;
    fn RegisterNotifications(&mut self, in_applicationid: *const ::windows::core::GUID) -> ::windows::core::Result<ISideShowNotificationManager>;
}
impl ISideShowSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowSession_Vtbl {
        unsafe extern "system" fn RegisterContent<Impl: ISideShowSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows::core::GUID, in_endpointid: *const ::windows::core::GUID, out_ppicontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterContent(::core::mem::transmute_copy(&in_applicationid), ::core::mem::transmute_copy(&in_endpointid)) {
                ::core::result::Result::Ok(ok__) => {
                    *out_ppicontent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterNotifications<Impl: ISideShowSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows::core::GUID, out_ppinotification: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterNotifications(::core::mem::transmute_copy(&in_applicationid)) {
                ::core::result::Result::Ok(ok__) => {
                    *out_ppinotification = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterContent: RegisterContent::<Impl, IMPL_OFFSET>,
            RegisterNotifications: RegisterNotifications::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowSession as ::windows::core::Interface>::IID
    }
}
