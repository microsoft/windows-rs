#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISideShowBulkCapabilitiesImpl: Sized + ISideShowCapabilitiesImpl {
    fn GetCapabilities();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISideShowBulkCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowBulkCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowBulkCapabilitiesVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: ISideShowBulkCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_keycollection: ::windows::core::RawPtr, inout_pvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCapability::<Impl, IMPL_OFFSET>, GetCapabilities::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowBulkCapabilities as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISideShowCapabilitiesImpl: Sized {
    fn GetCapability();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISideShowCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowCapabilitiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowCapabilitiesVtbl {
        unsafe extern "system" fn GetCapability<Impl: ISideShowCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCapability::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowCapabilities as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowCapabilitiesCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl ISideShowCapabilitiesCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowCapabilitiesCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowCapabilitiesCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: ISideShowCapabilitiesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: ISideShowCapabilitiesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_dwindex: u32, out_ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowCapabilitiesCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISideShowContentImpl: Sized {
    fn GetContent();
    fn ContentId();
    fn DifferentiateContent();
}
#[cfg(feature = "Win32_Foundation")]
impl ISideShowContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowContentVtbl {
        unsafe extern "system" fn GetContent<Impl: ISideShowContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_picapabilities: ::windows::core::RawPtr, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContentId<Impl: ISideShowContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DifferentiateContent<Impl: ISideShowContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetContent::<Impl, IMPL_OFFSET>, ContentId::<Impl, IMPL_OFFSET>, DifferentiateContent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowContent as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowContentManagerImpl: Sized {
    fn Add();
    fn Remove();
    fn RemoveAll();
    fn SetEventSink();
    fn GetDeviceCapabilities();
}
impl ISideShowContentManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowContentManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowContentManagerVtbl {
        unsafe extern "system" fn Add<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_picontent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_contentid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAll<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventSink<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pievents: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceCapabilities<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, RemoveAll::<Impl, IMPL_OFFSET>, SetEventSink::<Impl, IMPL_OFFSET>, GetDeviceCapabilities::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowContentManager as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowEventsImpl: Sized {
    fn ContentMissing();
    fn ApplicationEvent();
    fn DeviceAdded();
    fn DeviceRemoved();
}
impl ISideShowEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowEventsVtbl {
        unsafe extern "system" fn ContentMissing<Impl: ISideShowEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_contentid: u32, out_ppicontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplicationEvent<Impl: ISideShowEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_picapabilities: ::windows::core::RawPtr, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceAdded<Impl: ISideShowEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pidevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceRemoved<Impl: ISideShowEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pidevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ContentMissing::<Impl, IMPL_OFFSET>, ApplicationEvent::<Impl, IMPL_OFFSET>, DeviceAdded::<Impl, IMPL_OFFSET>, DeviceRemoved::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISideShowKeyCollectionImpl: Sized {
    fn Add();
    fn Clear();
    fn GetAt();
    fn GetCount();
    fn RemoveAt();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISideShowKeyCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowKeyCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowKeyCollectionVtbl {
        unsafe extern "system" fn Add<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Add::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowKeyCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait ISideShowNotificationImpl: Sized {
    fn NotificationId();
    fn SetNotificationId();
    fn Title();
    fn SetTitle();
    fn Message();
    fn SetMessage();
    fn Image();
    fn SetImage();
    fn ExpirationTime();
    fn SetExpirationTime();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
impl ISideShowNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowNotificationVtbl {
        unsafe extern "system" fn NotificationId<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pnotificationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetNotificationId<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Title<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ppwsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTitle<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pwsztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Message<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ppwszmessage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMessage<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pwszmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Image<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetImage<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpirationTime<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExpirationTime<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            NotificationId::<Impl, IMPL_OFFSET>,
            SetNotificationId::<Impl, IMPL_OFFSET>,
            Title::<Impl, IMPL_OFFSET>,
            SetTitle::<Impl, IMPL_OFFSET>,
            Message::<Impl, IMPL_OFFSET>,
            SetMessage::<Impl, IMPL_OFFSET>,
            Image::<Impl, IMPL_OFFSET>,
            SetImage::<Impl, IMPL_OFFSET>,
            ExpirationTime::<Impl, IMPL_OFFSET>,
            SetExpirationTime::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowNotification as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowNotificationManagerImpl: Sized {
    fn Show();
    fn Revoke();
    fn RevokeAll();
}
impl ISideShowNotificationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowNotificationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowNotificationManagerVtbl {
        unsafe extern "system" fn Show<Impl: ISideShowNotificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pinotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Revoke<Impl: ISideShowNotificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevokeAll<Impl: ISideShowNotificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Show::<Impl, IMPL_OFFSET>, Revoke::<Impl, IMPL_OFFSET>, RevokeAll::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowNotificationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISideShowPropVariantCollectionImpl: Sized {
    fn Add();
    fn Clear();
    fn GetAt();
    fn GetCount();
    fn RemoveAt();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISideShowPropVariantCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowPropVariantCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowPropVariantCollectionVtbl {
        unsafe extern "system" fn Add<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAt<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Add::<Impl, IMPL_OFFSET>, Clear::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, RemoveAt::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowPropVariantCollection as ::windows::core::Interface>::IID
    }
}
pub trait ISideShowSessionImpl: Sized {
    fn RegisterContent();
    fn RegisterNotifications();
}
impl ISideShowSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISideShowSessionVtbl {
        unsafe extern "system" fn RegisterContent<Impl: ISideShowSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows::core::GUID, in_endpointid: *const ::windows::core::GUID, out_ppicontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterNotifications<Impl: ISideShowSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_applicationid: *const ::windows::core::GUID, out_ppinotification: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterContent::<Impl, IMPL_OFFSET>, RegisterNotifications::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISideShowSession as ::windows::core::Interface>::IID
    }
}
