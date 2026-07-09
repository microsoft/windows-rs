#[cfg(feature = "Win32_mfobjects")]
#[inline]
pub unsafe fn MFCreateVirtualCamera<P3, P4>(r#type: MFVirtualCameraType, lifetime: MFVirtualCameraLifetime, access: MFVirtualCameraAccess, friendlyname: P3, sourceid: P4, categories: Option<&[windows_core::GUID]>) -> windows_core::Result<IMFVirtualCamera>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateVirtualCamera(r#type : MFVirtualCameraType, lifetime : MFVirtualCameraLifetime, access : MFVirtualCameraAccess, friendlyname : windows_core::PCWSTR, sourceid : windows_core::PCWSTR, categories : *const windows_core::GUID, categorycount : u32, virtualcamera : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateVirtualCamera(r#type, lifetime, access, friendlyname.param().abi(), sourceid.param().abi(), core::mem::transmute(categories.map_or(core::ptr::null(), |slice| slice.as_ptr())), categories.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFIsVirtualCameraTypeSupported(r#type: MFVirtualCameraType) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("mfsensorgroup.dll" "system" fn MFIsVirtualCameraTypeSupported(r#type : MFVirtualCameraType, supported : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFIsVirtualCameraTypeSupported(r#type, &mut result__).map(|| result__)
    }
}
windows_core::imp::define_interface!(IMFCameraSyncObject, IMFCameraSyncObject_Vtbl, 0x6338b23a_3042_49d2_a3ea_ec0fed815407);
windows_core::imp::interface_hierarchy!(IMFCameraSyncObject, windows_core::IUnknown);
impl IMFCameraSyncObject {
    pub unsafe fn WaitOnSignal(&self, timeoutinms: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).WaitOnSignal)(windows_core::Interface::as_raw(self), timeoutinms) }
    }
    pub unsafe fn Shutdown(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCameraSyncObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub WaitOnSignal: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMFCameraSyncObject_Impl: windows_core::IUnknownImpl {
    fn WaitOnSignal(&self, timeoutinms: u32) -> windows_core::Result<()>;
    fn Shutdown(&self);
}
impl IMFCameraSyncObject_Vtbl {
    pub const fn new<Identity: IMFCameraSyncObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WaitOnSignal<Identity: IMFCameraSyncObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinms: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraSyncObject_Impl::WaitOnSignal(this, core::mem::transmute_copy(&timeoutinms)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFCameraSyncObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraSyncObject_Impl::Shutdown(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            WaitOnSignal: WaitOnSignal::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCameraSyncObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFCameraSyncObject {}
#[cfg(feature = "Win32_mfobjects")]
windows_core::imp::define_interface!(IMFVirtualCamera, IMFVirtualCamera_Vtbl, 0x1c08a864_ef6c_4c75_af59_5f2d68da9563);
#[cfg(feature = "Win32_mfobjects")]
impl core::ops::Deref for IMFVirtualCamera {
    type Target = super::mfobjects::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "Win32_mfobjects")]
windows_core::imp::interface_hierarchy!(IMFVirtualCamera, windows_core::IUnknown, super::mfobjects::IMFAttributes);
#[cfg(feature = "Win32_mfobjects")]
impl IMFVirtualCamera {
    pub unsafe fn AddDeviceSourceInfo<P0>(&self, devicesourceinfo: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDeviceSourceInfo)(windows_core::Interface::as_raw(self), devicesourceinfo.param().abi()) }
    }
    #[cfg(feature = "Win32_devpropdef")]
    pub unsafe fn AddProperty(&self, pkey: *const super::devpropdef::DEVPROPKEY, r#type: super::devpropdef::DEVPROPTYPE, pbdata: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddProperty)(windows_core::Interface::as_raw(self), pkey, r#type, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap()) }
    }
    pub unsafe fn AddRegistryEntry<P0, P1>(&self, entryname: P0, subkeypath: P1, dwregtype: u32, pbdata: &[u8]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRegistryEntry)(windows_core::Interface::as_raw(self), entryname.param().abi(), subkeypath.param().abi(), dwregtype, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap()) }
    }
    pub unsafe fn Start<P0>(&self, pcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), pcallback.param().abi()) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Remove(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_mfidl")]
    pub unsafe fn GetMediaSource(&self) -> windows_core::Result<super::mfidl::IMFMediaSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SendCameraProperty(&self, propertyset: *const windows_core::GUID, propertyid: u32, propertyflags: u32, propertypayload: Option<*mut core::ffi::c_void>, propertypayloadlength: u32, data: Option<*mut core::ffi::c_void>, datalength: u32, datawritten: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendCameraProperty)(windows_core::Interface::as_raw(self), propertyset, propertyid, propertyflags, propertypayload.unwrap_or(core::mem::zeroed()) as _, propertypayloadlength, data.unwrap_or(core::mem::zeroed()) as _, datalength, datawritten as _) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateSyncEvent(&self, kseventset: *const windows_core::GUID, kseventid: u32, kseventflags: u32, eventhandle: super::winnt::HANDLE) -> windows_core::Result<IMFCameraSyncObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSyncEvent)(windows_core::Interface::as_raw(self), kseventset, kseventid, kseventflags, eventhandle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn CreateSyncSemaphore(&self, kseventset: *const windows_core::GUID, kseventid: u32, kseventflags: u32, semaphorehandle: super::winnt::HANDLE, semaphoreadjustment: i32) -> windows_core::Result<IMFCameraSyncObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSyncSemaphore)(windows_core::Interface::as_raw(self), kseventset, kseventid, kseventflags, semaphorehandle, semaphoreadjustment, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "Win32_mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFVirtualCamera_Vtbl {
    pub base__: super::mfobjects::IMFAttributes_Vtbl,
    pub AddDeviceSourceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_devpropdef")]
    pub AddProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::devpropdef::DEVPROPKEY, super::devpropdef::DEVPROPTYPE, *const u8, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_devpropdef"))]
    AddProperty: usize,
    pub AddRegistryEntry: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *const u8, u32) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_mfidl")]
    pub GetMediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_mfidl"))]
    GetMediaSource: usize,
    pub SendCameraProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, *mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub CreateSyncEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, super::winnt::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateSyncEvent: usize,
    #[cfg(feature = "Win32_winnt")]
    pub CreateSyncSemaphore: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, u32, super::winnt::HANDLE, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    CreateSyncSemaphore: usize,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_devpropdef", feature = "Win32_mfidl", feature = "Win32_mfobjects", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IMFVirtualCamera_Impl: super::mfobjects::IMFAttributes_Impl {
    fn AddDeviceSourceInfo(&self, devicesourceinfo: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddProperty(&self, pkey: *const super::devpropdef::DEVPROPKEY, r#type: super::devpropdef::DEVPROPTYPE, pbdata: *const u8, cbdata: u32) -> windows_core::Result<()>;
    fn AddRegistryEntry(&self, entryname: &windows_core::PCWSTR, subkeypath: &windows_core::PCWSTR, dwregtype: u32, pbdata: *const u8, cbdata: u32) -> windows_core::Result<()>;
    fn Start(&self, pcallback: windows_core::Ref<super::mfobjects::IMFAsyncCallback>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Remove(&self) -> windows_core::Result<()>;
    fn GetMediaSource(&self) -> windows_core::Result<super::mfidl::IMFMediaSource>;
    fn SendCameraProperty(&self, propertyset: *const windows_core::GUID, propertyid: u32, propertyflags: u32, propertypayload: *mut core::ffi::c_void, propertypayloadlength: u32, data: *mut core::ffi::c_void, datalength: u32, datawritten: *mut u32) -> windows_core::Result<()>;
    fn CreateSyncEvent(&self, kseventset: *const windows_core::GUID, kseventid: u32, kseventflags: u32, eventhandle: super::winnt::HANDLE) -> windows_core::Result<IMFCameraSyncObject>;
    fn CreateSyncSemaphore(&self, kseventset: *const windows_core::GUID, kseventid: u32, kseventflags: u32, semaphorehandle: super::winnt::HANDLE, semaphoreadjustment: i32) -> windows_core::Result<IMFCameraSyncObject>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_devpropdef", feature = "Win32_mfidl", feature = "Win32_mfobjects", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IMFVirtualCamera_Vtbl {
    pub const fn new<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddDeviceSourceInfo<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicesourceinfo: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::AddDeviceSourceInfo(this, core::mem::transmute(&devicesourceinfo)).into()
            }
        }
        unsafe extern "system" fn AddProperty<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *const super::devpropdef::DEVPROPKEY, r#type: super::devpropdef::DEVPROPTYPE, pbdata: *const u8, cbdata: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::AddProperty(this, core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn AddRegistryEntry<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entryname: windows_core::PCWSTR, subkeypath: windows_core::PCWSTR, dwregtype: u32, pbdata: *const u8, cbdata: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::AddRegistryEntry(this, core::mem::transmute(&entryname), core::mem::transmute(&subkeypath), core::mem::transmute_copy(&dwregtype), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::Start(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::Remove(this).into()
            }
        }
        unsafe extern "system" fn GetMediaSource<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediasource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVirtualCamera_Impl::GetMediaSource(this) {
                    Ok(ok__) => {
                        ppmediasource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SendCameraProperty<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyset: *const windows_core::GUID, propertyid: u32, propertyflags: u32, propertypayload: *mut core::ffi::c_void, propertypayloadlength: u32, data: *mut core::ffi::c_void, datalength: u32, datawritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::SendCameraProperty(this, core::mem::transmute_copy(&propertyset), core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&propertyflags), core::mem::transmute_copy(&propertypayload), core::mem::transmute_copy(&propertypayloadlength), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&datawritten)).into()
            }
        }
        unsafe extern "system" fn CreateSyncEvent<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kseventset: *const windows_core::GUID, kseventid: u32, kseventflags: u32, eventhandle: super::winnt::HANDLE, camerasyncobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVirtualCamera_Impl::CreateSyncEvent(this, core::mem::transmute_copy(&kseventset), core::mem::transmute_copy(&kseventid), core::mem::transmute_copy(&kseventflags), core::mem::transmute_copy(&eventhandle)) {
                    Ok(ok__) => {
                        camerasyncobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateSyncSemaphore<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kseventset: *const windows_core::GUID, kseventid: u32, kseventflags: u32, semaphorehandle: super::winnt::HANDLE, semaphoreadjustment: i32, camerasyncobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVirtualCamera_Impl::CreateSyncSemaphore(this, core::mem::transmute_copy(&kseventset), core::mem::transmute_copy(&kseventid), core::mem::transmute_copy(&kseventflags), core::mem::transmute_copy(&semaphorehandle), core::mem::transmute_copy(&semaphoreadjustment)) {
                    Ok(ok__) => {
                        camerasyncobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::Shutdown(this).into()
            }
        }
        Self {
            base__: super::mfobjects::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            AddDeviceSourceInfo: AddDeviceSourceInfo::<Identity, OFFSET>,
            AddProperty: AddProperty::<Identity, OFFSET>,
            AddRegistryEntry: AddRegistryEntry::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetMediaSource: GetMediaSource::<Identity, OFFSET>,
            SendCameraProperty: SendCameraProperty::<Identity, OFFSET>,
            CreateSyncEvent: CreateSyncEvent::<Identity, OFFSET>,
            CreateSyncSemaphore: CreateSyncSemaphore::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVirtualCamera as windows_core::Interface>::IID || iid == &<super::mfobjects::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_devpropdef", feature = "Win32_mfidl", feature = "Win32_mfobjects", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_winnt", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IMFVirtualCamera {}
pub type MFVirtualCameraAccess = i32;
pub const MFVirtualCameraAccess_AllUsers: MFVirtualCameraAccess = 1;
pub const MFVirtualCameraAccess_CurrentUser: MFVirtualCameraAccess = 0;
pub type MFVirtualCameraLifetime = i32;
pub const MFVirtualCameraLifetime_Session: MFVirtualCameraLifetime = 0;
pub const MFVirtualCameraLifetime_System: MFVirtualCameraLifetime = 1;
pub type MFVirtualCameraType = i32;
pub const MFVirtualCameraType_SoftwareCameraSource: MFVirtualCameraType = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMFVirtualCameraAccess(pub *mut MFVirtualCameraAccess);
impl PMFVirtualCameraAccess {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMFVirtualCameraAccess {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMFVirtualCameraLifetime(pub *mut MFVirtualCameraLifetime);
impl PMFVirtualCameraLifetime {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMFVirtualCameraLifetime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMFVirtualCameraType(pub *mut MFVirtualCameraType);
impl PMFVirtualCameraType {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMFVirtualCameraType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
