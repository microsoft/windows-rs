pub trait ISideShowBulkCapabilitiesImpl: Sized + ISideShowCapabilitiesImpl {
    fn GetCapabilities();
}
impl ::windows::core::RuntimeName for ISideShowBulkCapabilities {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowBulkCapabilities";
}
impl ISideShowBulkCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowBulkCapabilitiesImpl, const OFFSET: isize>() -> ISideShowBulkCapabilitiesVtbl {
        unsafe extern "system" fn GetCapabilities<Impl: ISideShowBulkCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_keycollection: ::windows::core::RawPtr, inout_pvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapabilities(&*(&in_keycollection as *const <ISideShowKeyCollection as ::windows::core::Abi>::Abi as *const <ISideShowKeyCollection as ::windows::core::DefaultType>::DefaultType), &*(&inout_pvalues as *const <ISideShowPropVariantCollection as ::windows::core::Abi>::Abi as *const <ISideShowPropVariantCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowBulkCapabilities>, ::windows::core::GetTrustLevel, GetCapabilities::<Impl, OFFSET>)
    }
}
pub trait ISideShowCapabilitiesImpl: Sized {
    fn GetCapability();
}
impl ::windows::core::RuntimeName for ISideShowCapabilities {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowCapabilities";
}
impl ISideShowCapabilitiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowCapabilitiesImpl, const OFFSET: isize>() -> ISideShowCapabilitiesVtbl {
        unsafe extern "system" fn GetCapability<Impl: ISideShowCapabilitiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCapability(&*(&in_keycapability as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&inout_pvalue as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowCapabilities>, ::windows::core::GetTrustLevel, GetCapability::<Impl, OFFSET>)
    }
}
pub trait ISideShowCapabilitiesCollectionImpl: Sized {
    fn GetCount();
    fn GetAt();
}
impl ::windows::core::RuntimeName for ISideShowCapabilitiesCollection {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowCapabilitiesCollection";
}
impl ISideShowCapabilitiesCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowCapabilitiesCollectionImpl, const OFFSET: isize>() -> ISideShowCapabilitiesCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: ISideShowCapabilitiesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&out_pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: ISideShowCapabilitiesCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_dwindex: u32, out_ppcapabilities: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(in_dwindex, ::core::mem::transmute_copy(&out_ppcapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowCapabilitiesCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>)
    }
}
pub trait ISideShowContentImpl: Sized {
    fn GetContent();
    fn ContentId();
    fn DifferentiateContent();
}
impl ::windows::core::RuntimeName for ISideShowContent {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowContent";
}
impl ISideShowContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowContentImpl, const OFFSET: isize>() -> ISideShowContentVtbl {
        unsafe extern "system" fn GetContent<Impl: ISideShowContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_picapabilities: ::windows::core::RawPtr, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContent(&*(&in_picapabilities as *const <ISideShowCapabilities as ::windows::core::Abi>::Abi as *const <ISideShowCapabilities as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&out_pdwsize), ::core::mem::transmute_copy(&out_ppbdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentId<Impl: ISideShowContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pcontentid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentId(::core::mem::transmute_copy(&out_pcontentid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DifferentiateContent<Impl: ISideShowContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DifferentiateContent(::core::mem::transmute_copy(&out_pfdifferentiatecontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowContent>, ::windows::core::GetTrustLevel, GetContent::<Impl, OFFSET>, ContentId::<Impl, OFFSET>, DifferentiateContent::<Impl, OFFSET>)
    }
}
pub trait ISideShowContentManagerImpl: Sized {
    fn Add();
    fn Remove();
    fn RemoveAll();
    fn SetEventSink();
    fn GetDeviceCapabilities();
}
impl ::windows::core::RuntimeName for ISideShowContentManager {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowContentManager";
}
impl ISideShowContentManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowContentManagerImpl, const OFFSET: isize>() -> ISideShowContentManagerVtbl {
        unsafe extern "system" fn Add<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_picontent: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&in_picontent as *const <ISideShowContent as ::windows::core::Abi>::Abi as *const <ISideShowContent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_contentid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(in_contentid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAll<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventSink<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pievents: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventSink(&*(&in_pievents as *const <ISideShowEvents as ::windows::core::Abi>::Abi as *const <ISideShowEvents as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceCapabilities<Impl: ISideShowContentManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceCapabilities(::core::mem::transmute_copy(&out_ppcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowContentManager>, ::windows::core::GetTrustLevel, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, RemoveAll::<Impl, OFFSET>, SetEventSink::<Impl, OFFSET>, GetDeviceCapabilities::<Impl, OFFSET>)
    }
}
pub trait ISideShowEventsImpl: Sized {
    fn ContentMissing();
    fn ApplicationEvent();
    fn DeviceAdded();
    fn DeviceRemoved();
}
impl ::windows::core::RuntimeName for ISideShowEvents {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowEvents";
}
impl ISideShowEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowEventsImpl, const OFFSET: isize>() -> ISideShowEventsVtbl {
        unsafe extern "system" fn ContentMissing<Impl: ISideShowEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_contentid: u32, out_ppicontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentMissing(in_contentid, ::core::mem::transmute_copy(&out_ppicontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationEvent<Impl: ISideShowEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_picapabilities: ::windows::core::RawPtr, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationEvent(&*(&in_picapabilities as *const <ISideShowCapabilities as ::windows::core::Abi>::Abi as *const <ISideShowCapabilities as ::windows::core::DefaultType>::DefaultType), in_dweventid, in_dweventsize, in_pbeventdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceAdded<Impl: ISideShowEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pidevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceAdded(&*(&in_pidevice as *const <ISideShowCapabilities as ::windows::core::Abi>::Abi as *const <ISideShowCapabilities as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceRemoved<Impl: ISideShowEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pidevice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceRemoved(&*(&in_pidevice as *const <ISideShowCapabilities as ::windows::core::Abi>::Abi as *const <ISideShowCapabilities as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowEvents>, ::windows::core::GetTrustLevel, ContentMissing::<Impl, OFFSET>, ApplicationEvent::<Impl, OFFSET>, DeviceAdded::<Impl, OFFSET>, DeviceRemoved::<Impl, OFFSET>)
    }
}
pub trait ISideShowKeyCollectionImpl: Sized {
    fn Add();
    fn Clear();
    fn GetAt();
    fn GetCount();
    fn RemoveAt();
}
impl ::windows::core::RuntimeName for ISideShowKeyCollection {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowKeyCollection";
}
impl ISideShowKeyCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>() -> ISideShowKeyCollectionVtbl {
        unsafe extern "system" fn Add<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(dwindex, &*(&pkey as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(pcelems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: ISideShowKeyCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAt(dwindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowKeyCollection>, ::windows::core::GetTrustLevel, Add::<Impl, OFFSET>, Clear::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, GetCount::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for ISideShowNotification {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowNotification";
}
impl ISideShowNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowNotificationImpl, const OFFSET: isize>() -> ISideShowNotificationVtbl {
        unsafe extern "system" fn NotificationId<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_pnotificationid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotificationId(::core::mem::transmute_copy(&out_pnotificationid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationId<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNotificationId(in_notificationid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Title<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ppwsztitle: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Title(::core::mem::transmute_copy(&out_ppwsztitle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pwsztitle: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTitle(&*(&in_pwsztitle as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Message<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ppwszmessage: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Message(::core::mem::transmute_copy(&out_ppwszmessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessage<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pwszmessage: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMessage(&*(&in_pwszmessage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Image<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Image(::core::mem::transmute_copy(&out_phicon)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetImage(&*(&in_hicon as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::Abi>::Abi as *const <super::super::UI::WindowsAndMessaging::HICON as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpirationTime<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpirationTime(::core::mem::transmute_copy(&out_ptime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Impl: ISideShowNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExpirationTime(&*(&in_ptime as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::Abi>::Abi as *const <super::super::Foundation::SYSTEMTIME as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ISideShowNotification>,
            ::windows::core::GetTrustLevel,
            NotificationId::<Impl, OFFSET>,
            SetNotificationId::<Impl, OFFSET>,
            Title::<Impl, OFFSET>,
            SetTitle::<Impl, OFFSET>,
            Message::<Impl, OFFSET>,
            SetMessage::<Impl, OFFSET>,
            Image::<Impl, OFFSET>,
            SetImage::<Impl, OFFSET>,
            ExpirationTime::<Impl, OFFSET>,
            SetExpirationTime::<Impl, OFFSET>,
        )
    }
}
pub trait ISideShowNotificationManagerImpl: Sized {
    fn Show();
    fn Revoke();
    fn RevokeAll();
}
impl ::windows::core::RuntimeName for ISideShowNotificationManager {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowNotificationManager";
}
impl ISideShowNotificationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowNotificationManagerImpl, const OFFSET: isize>() -> ISideShowNotificationManagerVtbl {
        unsafe extern "system" fn Show<Impl: ISideShowNotificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_pinotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Show(&*(&in_pinotification as *const <ISideShowNotification as ::windows::core::Abi>::Abi as *const <ISideShowNotification as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revoke<Impl: ISideShowNotificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_notificationid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Revoke(in_notificationid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeAll<Impl: ISideShowNotificationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RevokeAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowNotificationManager>, ::windows::core::GetTrustLevel, Show::<Impl, OFFSET>, Revoke::<Impl, OFFSET>, RevokeAll::<Impl, OFFSET>)
    }
}
pub trait ISideShowPropVariantCollectionImpl: Sized {
    fn Add();
    fn Clear();
    fn GetAt();
    fn GetCount();
    fn RemoveAt();
}
impl ::windows::core::RuntimeName for ISideShowPropVariantCollection {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowPropVariantCollection";
}
impl ISideShowPropVariantCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>() -> ISideShowPropVariantCollectionVtbl {
        unsafe extern "system" fn Add<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalue: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&pvalue as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pvalue: *mut super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(dwindex, &*(&pvalue as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcelems: *const u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(pcelems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAt<Impl: ISideShowPropVariantCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAt(dwindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowPropVariantCollection>, ::windows::core::GetTrustLevel, Add::<Impl, OFFSET>, Clear::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, GetCount::<Impl, OFFSET>, RemoveAt::<Impl, OFFSET>)
    }
}
pub trait ISideShowSessionImpl: Sized {
    fn RegisterContent();
    fn RegisterNotifications();
}
impl ::windows::core::RuntimeName for ISideShowSession {
    const NAME: &'static str = "Windows.Win32.System.SideShow.ISideShowSession";
}
impl ISideShowSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISideShowSessionImpl, const OFFSET: isize>() -> ISideShowSessionVtbl {
        unsafe extern "system" fn RegisterContent<Impl: ISideShowSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_applicationid: &::windows::core::GUID, in_endpointid: &::windows::core::GUID, out_ppicontent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterContent(&*(&in_applicationid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&in_endpointid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&out_ppicontent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterNotifications<Impl: ISideShowSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, in_applicationid: &::windows::core::GUID, out_ppinotification: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterNotifications(&*(&in_applicationid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&out_ppinotification)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISideShowSession>, ::windows::core::GetTrustLevel, RegisterContent::<Impl, OFFSET>, RegisterNotifications::<Impl, OFFSET>)
    }
}
