#[cfg(all(feature = "mfobjects", feature = "shared"))]
#[inline]
pub unsafe fn MFCreateVirtualCamera<P3, P4>(r#type: MFVirtualCameraType, lifetime: MFVirtualCameraLifetime, access: MFVirtualCameraAccess, friendlyname: P3, sourceid: P4, categories: Option<&[windows_core::GUID]>) -> windows_core::Result<IMFVirtualCamera>
where
    P3: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateVirtualCamera(r#type : MFVirtualCameraType, lifetime : MFVirtualCameraLifetime, access : MFVirtualCameraAccess, friendlyname : windows_core::PCWSTR, sourceid : windows_core::PCWSTR, categories : *const windows_core::GUID, categorycount : super::ULONG, virtualcamera : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateVirtualCamera(r#type, lifetime, access, friendlyname.param().abi(), sourceid.param().abi(), categories.map_or(core::ptr::null(), |slice| slice.as_ptr()), categories.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFIsVirtualCameraTypeSupported(r#type: MFVirtualCameraType, supported: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("mfsensorgroup.dll" "system" fn MFIsVirtualCameraTypeSupported(r#type : MFVirtualCameraType, supported : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { MFIsVirtualCameraTypeSupported(r#type, supported as _) }
}
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_IsVirtualCamera: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 3 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_IsWindowsCameraEffectAvailable: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 4 };
#[cfg(feature = "devpropdef")]
pub const DEVPKEY_DeviceInterface_VirtualCameraAssociatedCameras: super::DEVPROPKEY = super::DEVPROPKEY { fmtid: windows_core::GUID::from_u128(0x6edc630d_c2e3_43b7_b2d1_20525a1af120), pid: 5 };
windows_core::imp::define_interface!(IMFCameraSyncObject, IMFCameraSyncObject_Vtbl, 0x6338b23a_3042_49d2_a3ea_ec0fed815407);
windows_core::imp::interface_hierarchy!(IMFCameraSyncObject, windows_core::IUnknown);
impl IMFCameraSyncObject {
    #[cfg(feature = "shared")]
    pub unsafe fn WaitOnSignal(&self, timeoutinms: super::DWORD) -> windows_core::HRESULT {
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
    #[cfg(feature = "shared")]
    pub WaitOnSignal: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    WaitOnSignal: usize,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "shared")]
pub trait IMFCameraSyncObject_Impl: windows_core::IUnknownImpl {
    fn WaitOnSignal(&self, timeoutinms: super::DWORD) -> windows_core::Result<()>;
    fn Shutdown(&self);
}
#[cfg(feature = "shared")]
impl IMFCameraSyncObject_Vtbl {
    pub const fn new<Identity: IMFCameraSyncObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WaitOnSignal<Identity: IMFCameraSyncObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeoutinms: super::DWORD) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IMFCameraSyncObject {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFVirtualCamera, IMFVirtualCamera_Vtbl, 0x1c08a864_ef6c_4c75_af59_5f2d68da9563);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFVirtualCamera {
    type Target = super::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFVirtualCamera, windows_core::IUnknown, super::IMFAttributes);
#[cfg(feature = "mfobjects")]
impl IMFVirtualCamera {
    pub unsafe fn AddDeviceSourceInfo<P0>(&self, devicesourceinfo: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDeviceSourceInfo)(windows_core::Interface::as_raw(self), devicesourceinfo.param().abi()) }
    }
    #[cfg(all(feature = "devpropdef", feature = "shared"))]
    pub unsafe fn AddProperty(&self, pkey: *const super::DEVPROPKEY, r#type: super::DEVPROPTYPE, pbdata: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddProperty)(windows_core::Interface::as_raw(self), pkey, r#type, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddRegistryEntry<P0, P1>(&self, entryname: P0, subkeypath: P1, dwregtype: super::DWORD, pbdata: &[u8]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddRegistryEntry)(windows_core::Interface::as_raw(self), entryname.param().abi(), subkeypath.param().abi(), dwregtype, core::mem::transmute(pbdata.as_ptr()), pbdata.len().try_into().unwrap()) }
    }
    pub unsafe fn Start<P0>(&self, pcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), pcallback.param().abi()) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Remove(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn GetMediaSource(&self) -> windows_core::Result<super::IMFMediaSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SendCameraProperty(&self, propertyset: *const windows_core::GUID, propertyid: super::ULONG, propertyflags: super::ULONG, propertypayload: Option<*mut core::ffi::c_void>, propertypayloadlength: super::ULONG, data: Option<*mut core::ffi::c_void>, datalength: super::ULONG, datawritten: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendCameraProperty)(windows_core::Interface::as_raw(self), propertyset, propertyid, propertyflags, propertypayload.unwrap_or(core::mem::zeroed()) as _, propertypayloadlength, data.unwrap_or(core::mem::zeroed()) as _, datalength, datawritten as _) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateSyncEvent(&self, kseventset: *const windows_core::GUID, kseventid: super::ULONG, kseventflags: super::ULONG, eventhandle: super::HANDLE) -> windows_core::Result<IMFCameraSyncObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSyncEvent)(windows_core::Interface::as_raw(self), kseventset, kseventid, kseventflags, eventhandle, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateSyncSemaphore(&self, kseventset: *const windows_core::GUID, kseventid: super::ULONG, kseventflags: super::ULONG, semaphorehandle: super::HANDLE, semaphoreadjustment: super::LONG) -> windows_core::Result<IMFCameraSyncObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSyncSemaphore)(windows_core::Interface::as_raw(self), kseventset, kseventid, kseventflags, semaphorehandle, semaphoreadjustment, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFVirtualCamera_Vtbl {
    pub base__: super::IMFAttributes_Vtbl,
    pub AddDeviceSourceInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "devpropdef", feature = "shared"))]
    pub AddProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::DEVPROPKEY, super::DEVPROPTYPE, *const super::BYTE, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "devpropdef", feature = "shared")))]
    AddProperty: usize,
    #[cfg(feature = "shared")]
    pub AddRegistryEntry: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::DWORD, *const super::BYTE, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddRegistryEntry: usize,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfidl")]
    pub GetMediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    GetMediaSource: usize,
    #[cfg(feature = "shared")]
    pub SendCameraProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::ULONG, super::ULONG, *mut core::ffi::c_void, super::ULONG, *mut core::ffi::c_void, super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SendCameraProperty: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateSyncEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::ULONG, super::ULONG, super::HANDLE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateSyncEvent: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub CreateSyncSemaphore: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, super::ULONG, super::ULONG, super::HANDLE, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    CreateSyncSemaphore: usize,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "devpropdef", feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFVirtualCamera_Impl: super::IMFAttributes_Impl {
    fn AddDeviceSourceInfo(&self, devicesourceinfo: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddProperty(&self, pkey: *const super::DEVPROPKEY, r#type: super::DEVPROPTYPE, pbdata: *const super::BYTE, cbdata: super::ULONG) -> windows_core::Result<()>;
    fn AddRegistryEntry(&self, entryname: &windows_core::PCWSTR, subkeypath: &windows_core::PCWSTR, dwregtype: super::DWORD, pbdata: *const super::BYTE, cbdata: super::ULONG) -> windows_core::Result<()>;
    fn Start(&self, pcallback: windows_core::Ref<super::IMFAsyncCallback>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Remove(&self) -> windows_core::Result<()>;
    fn GetMediaSource(&self) -> windows_core::Result<super::IMFMediaSource>;
    fn SendCameraProperty(&self, propertyset: *const windows_core::GUID, propertyid: super::ULONG, propertyflags: super::ULONG, propertypayload: *mut core::ffi::c_void, propertypayloadlength: super::ULONG, data: *mut core::ffi::c_void, datalength: super::ULONG, datawritten: *mut super::ULONG) -> windows_core::Result<()>;
    fn CreateSyncEvent(&self, kseventset: *const windows_core::GUID, kseventid: super::ULONG, kseventflags: super::ULONG, eventhandle: super::HANDLE) -> windows_core::Result<IMFCameraSyncObject>;
    fn CreateSyncSemaphore(&self, kseventset: *const windows_core::GUID, kseventid: super::ULONG, kseventflags: super::ULONG, semaphorehandle: super::HANDLE, semaphoreadjustment: super::LONG) -> windows_core::Result<IMFCameraSyncObject>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "devpropdef", feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFVirtualCamera_Vtbl {
    pub const fn new<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddDeviceSourceInfo<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, devicesourceinfo: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::AddDeviceSourceInfo(this, core::mem::transmute(&devicesourceinfo)).into()
            }
        }
        unsafe extern "system" fn AddProperty<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *const super::DEVPROPKEY, r#type: super::DEVPROPTYPE, pbdata: *const super::BYTE, cbdata: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::AddProperty(this, core::mem::transmute_copy(&pkey), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata)).into()
            }
        }
        unsafe extern "system" fn AddRegistryEntry<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, entryname: windows_core::PCWSTR, subkeypath: windows_core::PCWSTR, dwregtype: super::DWORD, pbdata: *const super::BYTE, cbdata: super::ULONG) -> windows_core::HRESULT {
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
                    Err(err) => {
                        ppmediasource.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn SendCameraProperty<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyset: *const windows_core::GUID, propertyid: super::ULONG, propertyflags: super::ULONG, propertypayload: *mut core::ffi::c_void, propertypayloadlength: super::ULONG, data: *mut core::ffi::c_void, datalength: super::ULONG, datawritten: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVirtualCamera_Impl::SendCameraProperty(this, core::mem::transmute_copy(&propertyset), core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&propertyflags), core::mem::transmute_copy(&propertypayload), core::mem::transmute_copy(&propertypayloadlength), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datalength), core::mem::transmute_copy(&datawritten)).into()
            }
        }
        unsafe extern "system" fn CreateSyncEvent<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kseventset: *const windows_core::GUID, kseventid: super::ULONG, kseventflags: super::ULONG, eventhandle: super::HANDLE, camerasyncobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVirtualCamera_Impl::CreateSyncEvent(this, core::mem::transmute_copy(&kseventset), core::mem::transmute_copy(&kseventid), core::mem::transmute_copy(&kseventflags), core::mem::transmute_copy(&eventhandle)) {
                    Ok(ok__) => {
                        camerasyncobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        camerasyncobject.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        unsafe extern "system" fn CreateSyncSemaphore<Identity: IMFVirtualCamera_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, kseventset: *const windows_core::GUID, kseventid: super::ULONG, kseventflags: super::ULONG, semaphorehandle: super::HANDLE, semaphoreadjustment: super::LONG, camerasyncobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVirtualCamera_Impl::CreateSyncSemaphore(this, core::mem::transmute_copy(&kseventset), core::mem::transmute_copy(&kseventid), core::mem::transmute_copy(&kseventflags), core::mem::transmute_copy(&semaphorehandle), core::mem::transmute_copy(&semaphoreadjustment)) {
                    Ok(ok__) => {
                        camerasyncobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        camerasyncobject.write(core::mem::zeroed());
                        err.into()
                    }
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
            base__: super::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMFVirtualCamera as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "devpropdef", feature = "guiddef", feature = "mfidl", feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFVirtualCamera {}
pub type MFVirtualCameraAccess = i32;
pub const MFVirtualCameraAccess_AllUsers: MFVirtualCameraAccess = 1;
pub const MFVirtualCameraAccess_CurrentUser: MFVirtualCameraAccess = 0;
pub type MFVirtualCameraLifetime = i32;
pub const MFVirtualCameraLifetime_Session: MFVirtualCameraLifetime = 0;
pub const MFVirtualCameraLifetime_System: MFVirtualCameraLifetime = 1;
pub type MFVirtualCameraType = i32;
pub const MFVirtualCameraType_SoftwareCameraSource: MFVirtualCameraType = 0;
pub const MF_DEVSOURCE_ATTRIBUTE_ENABLE_MS_CAMERA_EFFECTS: windows_core::GUID = windows_core::GUID::from_u128(0x28a5531a_57dd_4fd5_aaa7_385abf57d785);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_CUSTOM_EVENT: windows_core::GUID = windows_core::GUID::from_u128(0x6e59489c_47d3_4467_83ef_12d34e871665);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_PIPELINE_SHUTDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x45a81b31_43f8_4e5d_8ce2_22dce026996d);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_INITIALIZE: windows_core::GUID = windows_core::GUID::from_u128(0xe52c4dff_e46d_4d0b_bc75_ddd4c8723f96);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_START: windows_core::GUID = windows_core::GUID::from_u128(0xb1eeb989_b456_4f4a_ae40_079c28e24af8);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_STOP: windows_core::GUID = windows_core::GUID::from_u128(0xb7fe7a61_fe91_415e_8608_d37dedb1a58b);
pub const MF_FRAMESERVER_VCAMEVENT_EXTENDED_SOURCE_UNINITIALIZE: windows_core::GUID = windows_core::GUID::from_u128(0xa0ebaba7_a422_4e33_8401_b37d2800aa67);
pub const MF_VIRTUALCAMERA_ASSOCIATED_CAMERA_SOURCES: windows_core::GUID = windows_core::GUID::from_u128(0x1bb79e7c_5d83_438c_94d8_e5f0df6d3279);
pub const MF_VIRTUALCAMERA_CONFIGURATION_APP_PACKAGE_FAMILY_NAME: windows_core::GUID = windows_core::GUID::from_u128(0x658abe51_8044_462e_97ea_e676fd72055f);
pub const MF_VIRTUALCAMERA_PROVIDE_ASSOCIATED_CAMERA_SOURCES: windows_core::GUID = windows_core::GUID::from_u128(0xf0273718_4a4d_4ac5_a15d_305eb5e90667);
pub type PMFVirtualCameraAccess = *mut MFVirtualCameraAccess;
pub type PMFVirtualCameraLifetime = *mut MFVirtualCameraLifetime;
pub type PMFVirtualCameraType = *mut MFVirtualCameraType;
