#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISideShowBulkCapabilities_Impl: Sized + ISideShowCapabilities_Impl {
    fn GetCapabilities(&self, in_keycollection: Option<&ISideShowKeyCollection>, inout_pvalues: *mut Option<ISideShowPropVariantCollection>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISideShowBulkCapabilities {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISideShowBulkCapabilities_Vtbl {
    pub const fn new<Identity: ISideShowBulkCapabilities_Impl, const OFFSET: isize>() -> ISideShowBulkCapabilities_Vtbl {
        unsafe extern "system" fn GetCapabilities<Identity: ISideShowBulkCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_keycollection: *mut core::ffi::c_void, inout_pvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowBulkCapabilities_Impl::GetCapabilities(this, windows_core::from_raw_borrowed(&in_keycollection), core::mem::transmute_copy(&inout_pvalues)).into()
        }
        Self { base__: ISideShowCapabilities_Vtbl::new::<Identity, OFFSET>(), GetCapabilities: GetCapabilities::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowBulkCapabilities as windows_core::Interface>::IID || iid == &<ISideShowCapabilities as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISideShowCapabilities_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCapability(&self, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISideShowCapabilities {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISideShowCapabilities_Vtbl {
    pub const fn new<Identity: ISideShowCapabilities_Impl, const OFFSET: isize>() -> ISideShowCapabilities_Vtbl {
        unsafe extern "system" fn GetCapability<Identity: ISideShowCapabilities_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_keycapability: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, inout_pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowCapabilities_Impl::GetCapability(this, core::mem::transmute_copy(&in_keycapability), core::mem::transmute_copy(&inout_pvalue)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCapability: GetCapability::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowCapabilities as windows_core::Interface>::IID
    }
}
pub trait ISideShowCapabilitiesCollection_Impl: Sized + windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetAt(&self, in_dwindex: u32) -> windows_core::Result<ISideShowCapabilities>;
}
impl windows_core::RuntimeName for ISideShowCapabilitiesCollection {}
impl ISideShowCapabilitiesCollection_Vtbl {
    pub const fn new<Identity: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>() -> ISideShowCapabilitiesCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pdwcount: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowCapabilitiesCollection_Impl::GetCount(this) {
                Ok(ok__) => {
                    out_pdwcount.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ISideShowCapabilitiesCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_dwindex: u32, out_ppcapabilities: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowCapabilitiesCollection_Impl::GetAt(this, core::mem::transmute_copy(&in_dwindex)) {
                Ok(ok__) => {
                    out_ppcapabilities.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetAt: GetAt::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowCapabilitiesCollection as windows_core::Interface>::IID
    }
}
pub trait ISideShowContent_Impl: Sized + windows_core::IUnknownImpl {
    fn GetContent(&self, in_picapabilities: Option<&ISideShowCapabilities>, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> windows_core::Result<()>;
    fn ContentId(&self) -> windows_core::Result<u32>;
    fn DifferentiateContent(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for ISideShowContent {}
impl ISideShowContent_Vtbl {
    pub const fn new<Identity: ISideShowContent_Impl, const OFFSET: isize>() -> ISideShowContent_Vtbl {
        unsafe extern "system" fn GetContent<Identity: ISideShowContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_picapabilities: *mut core::ffi::c_void, out_pdwsize: *mut u32, out_ppbdata: *mut *mut u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowContent_Impl::GetContent(this, windows_core::from_raw_borrowed(&in_picapabilities), core::mem::transmute_copy(&out_pdwsize), core::mem::transmute_copy(&out_ppbdata)).into()
        }
        unsafe extern "system" fn ContentId<Identity: ISideShowContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pcontentid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowContent_Impl::ContentId(this) {
                Ok(ok__) => {
                    out_pcontentid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DifferentiateContent<Identity: ISideShowContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pfdifferentiatecontent: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowContent_Impl::DifferentiateContent(this) {
                Ok(ok__) => {
                    out_pfdifferentiatecontent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContent: GetContent::<Identity, OFFSET>,
            ContentId: ContentId::<Identity, OFFSET>,
            DifferentiateContent: DifferentiateContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowContent as windows_core::Interface>::IID
    }
}
pub trait ISideShowContentManager_Impl: Sized + windows_core::IUnknownImpl {
    fn Add(&self, in_picontent: Option<&ISideShowContent>) -> windows_core::Result<()>;
    fn Remove(&self, in_contentid: u32) -> windows_core::Result<()>;
    fn RemoveAll(&self) -> windows_core::Result<()>;
    fn SetEventSink(&self, in_pievents: Option<&ISideShowEvents>) -> windows_core::Result<()>;
    fn GetDeviceCapabilities(&self) -> windows_core::Result<ISideShowCapabilitiesCollection>;
}
impl windows_core::RuntimeName for ISideShowContentManager {}
impl ISideShowContentManager_Vtbl {
    pub const fn new<Identity: ISideShowContentManager_Impl, const OFFSET: isize>() -> ISideShowContentManager_Vtbl {
        unsafe extern "system" fn Add<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_picontent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowContentManager_Impl::Add(this, windows_core::from_raw_borrowed(&in_picontent)).into()
        }
        unsafe extern "system" fn Remove<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_contentid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowContentManager_Impl::Remove(this, core::mem::transmute_copy(&in_contentid)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowContentManager_Impl::RemoveAll(this).into()
        }
        unsafe extern "system" fn SetEventSink<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pievents: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowContentManager_Impl::SetEventSink(this, windows_core::from_raw_borrowed(&in_pievents)).into()
        }
        unsafe extern "system" fn GetDeviceCapabilities<Identity: ISideShowContentManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowContentManager_Impl::GetDeviceCapabilities(this) {
                Ok(ok__) => {
                    out_ppcollection.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveAll: RemoveAll::<Identity, OFFSET>,
            SetEventSink: SetEventSink::<Identity, OFFSET>,
            GetDeviceCapabilities: GetDeviceCapabilities::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowContentManager as windows_core::Interface>::IID
    }
}
pub trait ISideShowEvents_Impl: Sized + windows_core::IUnknownImpl {
    fn ContentMissing(&self, in_contentid: u32) -> windows_core::Result<ISideShowContent>;
    fn ApplicationEvent(&self, in_picapabilities: Option<&ISideShowCapabilities>, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> windows_core::Result<()>;
    fn DeviceAdded(&self, in_pidevice: Option<&ISideShowCapabilities>) -> windows_core::Result<()>;
    fn DeviceRemoved(&self, in_pidevice: Option<&ISideShowCapabilities>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISideShowEvents {}
impl ISideShowEvents_Vtbl {
    pub const fn new<Identity: ISideShowEvents_Impl, const OFFSET: isize>() -> ISideShowEvents_Vtbl {
        unsafe extern "system" fn ContentMissing<Identity: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_contentid: u32, out_ppicontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowEvents_Impl::ContentMissing(this, core::mem::transmute_copy(&in_contentid)) {
                Ok(ok__) => {
                    out_ppicontent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationEvent<Identity: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_picapabilities: *mut core::ffi::c_void, in_dweventid: u32, in_dweventsize: u32, in_pbeventdata: *const u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowEvents_Impl::ApplicationEvent(this, windows_core::from_raw_borrowed(&in_picapabilities), core::mem::transmute_copy(&in_dweventid), core::mem::transmute_copy(&in_dweventsize), core::mem::transmute_copy(&in_pbeventdata)).into()
        }
        unsafe extern "system" fn DeviceAdded<Identity: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pidevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowEvents_Impl::DeviceAdded(this, windows_core::from_raw_borrowed(&in_pidevice)).into()
        }
        unsafe extern "system" fn DeviceRemoved<Identity: ISideShowEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pidevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowEvents_Impl::DeviceRemoved(this, windows_core::from_raw_borrowed(&in_pidevice)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ContentMissing: ContentMissing::<Identity, OFFSET>,
            ApplicationEvent: ApplicationEvent::<Identity, OFFSET>,
            DeviceAdded: DeviceAdded::<Identity, OFFSET>,
            DeviceRemoved: DeviceRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait ISideShowKeyCollection_Impl: Sized + windows_core::IUnknownImpl {
    fn Add(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::Result<()>;
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl windows_core::RuntimeName for ISideShowKeyCollection {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISideShowKeyCollection_Vtbl {
    pub const fn new<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>() -> ISideShowKeyCollection_Vtbl {
        unsafe extern "system" fn Add<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowKeyCollection_Impl::Add(this, core::mem::transmute_copy(&key)).into()
        }
        unsafe extern "system" fn Clear<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowKeyCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn GetAt<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowKeyCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pkey)).into()
        }
        unsafe extern "system" fn GetCount<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowKeyCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ISideShowKeyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowKeyCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowKeyCollection as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
pub trait ISideShowNotification_Impl: Sized + windows_core::IUnknownImpl {
    fn NotificationId(&self) -> windows_core::Result<u32>;
    fn SetNotificationId(&self, in_notificationid: u32) -> windows_core::Result<()>;
    fn Title(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetTitle(&self, in_pwsztitle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Message(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetMessage(&self, in_pwszmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Image(&self) -> windows_core::Result<super::super::UI::WindowsAndMessaging::HICON>;
    fn SetImage(&self, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> windows_core::Result<()>;
    fn ExpirationTime(&self) -> windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn SetExpirationTime(&self, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::RuntimeName for ISideShowNotification {}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl ISideShowNotification_Vtbl {
    pub const fn new<Identity: ISideShowNotification_Impl, const OFFSET: isize>() -> ISideShowNotification_Vtbl {
        unsafe extern "system" fn NotificationId<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_pnotificationid: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowNotification_Impl::NotificationId(this) {
                Ok(ok__) => {
                    out_pnotificationid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNotificationId<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_notificationid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowNotification_Impl::SetNotificationId(this, core::mem::transmute_copy(&in_notificationid)).into()
        }
        unsafe extern "system" fn Title<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ppwsztitle: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowNotification_Impl::Title(this) {
                Ok(ok__) => {
                    out_ppwsztitle.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTitle<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pwsztitle: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowNotification_Impl::SetTitle(this, core::mem::transmute(&in_pwsztitle)).into()
        }
        unsafe extern "system" fn Message<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ppwszmessage: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowNotification_Impl::Message(this) {
                Ok(ok__) => {
                    out_ppwszmessage.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessage<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pwszmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowNotification_Impl::SetMessage(this, core::mem::transmute(&in_pwszmessage)).into()
        }
        unsafe extern "system" fn Image<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_phicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowNotification_Impl::Image(this) {
                Ok(ok__) => {
                    out_phicon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImage<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_hicon: super::super::UI::WindowsAndMessaging::HICON) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowNotification_Impl::SetImage(this, core::mem::transmute_copy(&in_hicon)).into()
        }
        unsafe extern "system" fn ExpirationTime<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, out_ptime: *mut super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowNotification_Impl::ExpirationTime(this) {
                Ok(ok__) => {
                    out_ptime.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExpirationTime<Identity: ISideShowNotification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_ptime: *const super::super::Foundation::SYSTEMTIME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowNotification_Impl::SetExpirationTime(this, core::mem::transmute_copy(&in_ptime)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotificationId: NotificationId::<Identity, OFFSET>,
            SetNotificationId: SetNotificationId::<Identity, OFFSET>,
            Title: Title::<Identity, OFFSET>,
            SetTitle: SetTitle::<Identity, OFFSET>,
            Message: Message::<Identity, OFFSET>,
            SetMessage: SetMessage::<Identity, OFFSET>,
            Image: Image::<Identity, OFFSET>,
            SetImage: SetImage::<Identity, OFFSET>,
            ExpirationTime: ExpirationTime::<Identity, OFFSET>,
            SetExpirationTime: SetExpirationTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowNotification as windows_core::Interface>::IID
    }
}
pub trait ISideShowNotificationManager_Impl: Sized + windows_core::IUnknownImpl {
    fn Show(&self, in_pinotification: Option<&ISideShowNotification>) -> windows_core::Result<()>;
    fn Revoke(&self, in_notificationid: u32) -> windows_core::Result<()>;
    fn RevokeAll(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISideShowNotificationManager {}
impl ISideShowNotificationManager_Vtbl {
    pub const fn new<Identity: ISideShowNotificationManager_Impl, const OFFSET: isize>() -> ISideShowNotificationManager_Vtbl {
        unsafe extern "system" fn Show<Identity: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_pinotification: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowNotificationManager_Impl::Show(this, windows_core::from_raw_borrowed(&in_pinotification)).into()
        }
        unsafe extern "system" fn Revoke<Identity: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_notificationid: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowNotificationManager_Impl::Revoke(this, core::mem::transmute_copy(&in_notificationid)).into()
        }
        unsafe extern "system" fn RevokeAll<Identity: ISideShowNotificationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowNotificationManager_Impl::RevokeAll(this).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Show: Show::<Identity, OFFSET>,
            Revoke: Revoke::<Identity, OFFSET>,
            RevokeAll: RevokeAll::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowNotificationManager as windows_core::Interface>::IID
    }
}
pub trait ISideShowPropVariantCollection_Impl: Sized + windows_core::IUnknownImpl {
    fn Add(&self, pvalue: *const windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn GetAt(&self, dwindex: u32, pvalue: *mut windows_core::PROPVARIANT) -> windows_core::Result<()>;
    fn GetCount(&self, pcelems: *const u32) -> windows_core::Result<()>;
    fn RemoveAt(&self, dwindex: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for ISideShowPropVariantCollection {}
impl ISideShowPropVariantCollection_Vtbl {
    pub const fn new<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>() -> ISideShowPropVariantCollection_Vtbl {
        unsafe extern "system" fn Add<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalue: *const core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowPropVariantCollection_Impl::Add(this, core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn Clear<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowPropVariantCollection_Impl::Clear(this).into()
        }
        unsafe extern "system" fn GetAt<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pvalue: *mut core::mem::MaybeUninit<windows_core::PROPVARIANT>) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowPropVariantCollection_Impl::GetAt(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn GetCount<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcelems: *const u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowPropVariantCollection_Impl::GetCount(this, core::mem::transmute_copy(&pcelems)).into()
        }
        unsafe extern "system" fn RemoveAt<Identity: ISideShowPropVariantCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            ISideShowPropVariantCollection_Impl::RemoveAt(this, core::mem::transmute_copy(&dwindex)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            GetAt: GetAt::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            RemoveAt: RemoveAt::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowPropVariantCollection as windows_core::Interface>::IID
    }
}
pub trait ISideShowSession_Impl: Sized + windows_core::IUnknownImpl {
    fn RegisterContent(&self, in_applicationid: *const windows_core::GUID, in_endpointid: *const windows_core::GUID) -> windows_core::Result<ISideShowContentManager>;
    fn RegisterNotifications(&self, in_applicationid: *const windows_core::GUID) -> windows_core::Result<ISideShowNotificationManager>;
}
impl windows_core::RuntimeName for ISideShowSession {}
impl ISideShowSession_Vtbl {
    pub const fn new<Identity: ISideShowSession_Impl, const OFFSET: isize>() -> ISideShowSession_Vtbl {
        unsafe extern "system" fn RegisterContent<Identity: ISideShowSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_applicationid: *const windows_core::GUID, in_endpointid: *const windows_core::GUID, out_ppicontent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowSession_Impl::RegisterContent(this, core::mem::transmute_copy(&in_applicationid), core::mem::transmute_copy(&in_endpointid)) {
                Ok(ok__) => {
                    out_ppicontent.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterNotifications<Identity: ISideShowSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, in_applicationid: *const windows_core::GUID, out_ppinotification: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match ISideShowSession_Impl::RegisterNotifications(this, core::mem::transmute_copy(&in_applicationid)) {
                Ok(ok__) => {
                    out_ppinotification.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterContent: RegisterContent::<Identity, OFFSET>,
            RegisterNotifications: RegisterNotifications::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISideShowSession as windows_core::Interface>::IID
    }
}
