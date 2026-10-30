#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFSpatialAudioObjectBuffer, IMFSpatialAudioObjectBuffer_Vtbl, 0xd396ec8c_605e_4249_978d_72ad1c312872);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFSpatialAudioObjectBuffer {
    type Target = super::IMFMediaBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFSpatialAudioObjectBuffer, windows_core::IUnknown, super::IMFMediaBuffer);
#[cfg(feature = "mfobjects")]
impl IMFSpatialAudioObjectBuffer {
    #[cfg(feature = "shared")]
    pub unsafe fn SetID(&self, u32id: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetID)(windows_core::Interface::as_raw(self), u32id) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetID(&self, pu32id: *mut super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), pu32id as _) }
    }
    #[cfg(feature = "spatialaudioclient")]
    pub unsafe fn SetType(&self, r#type: super::AudioObjectType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type) }
    }
    #[cfg(feature = "spatialaudioclient")]
    pub unsafe fn GetType(&self, ptype: *mut super::AudioObjectType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), ptype as _) }
    }
    #[cfg(feature = "spatialaudiometadata")]
    pub unsafe fn GetMetadataItems(&self) -> windows_core::Result<super::ISpatialAudioMetadataItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFSpatialAudioObjectBuffer_Vtbl {
    pub base__: super::IMFMediaBuffer_Vtbl,
    #[cfg(feature = "shared")]
    pub SetID: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetID: usize,
    #[cfg(feature = "shared")]
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetID: usize,
    #[cfg(feature = "spatialaudioclient")]
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, super::AudioObjectType) -> windows_core::HRESULT,
    #[cfg(not(feature = "spatialaudioclient"))]
    SetType: usize,
    #[cfg(feature = "spatialaudioclient")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::AudioObjectType) -> windows_core::HRESULT,
    #[cfg(not(feature = "spatialaudioclient"))]
    GetType: usize,
    #[cfg(feature = "spatialaudiometadata")]
    pub GetMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "spatialaudiometadata"))]
    GetMetadataItems: usize,
}
#[cfg(all(feature = "mfobjects", feature = "shared", feature = "spatialaudioclient", feature = "spatialaudiometadata"))]
pub trait IMFSpatialAudioObjectBuffer_Impl: super::IMFMediaBuffer_Impl {
    fn SetID(&self, u32id: super::UINT32) -> windows_core::Result<()>;
    fn GetID(&self, pu32id: *mut super::UINT32) -> windows_core::Result<()>;
    fn SetType(&self, r#type: super::AudioObjectType) -> windows_core::Result<()>;
    fn GetType(&self, ptype: *mut super::AudioObjectType) -> windows_core::Result<()>;
    fn GetMetadataItems(&self) -> windows_core::Result<super::ISpatialAudioMetadataItems>;
}
#[cfg(all(feature = "mfobjects", feature = "shared", feature = "spatialaudioclient", feature = "spatialaudiometadata"))]
impl IMFSpatialAudioObjectBuffer_Vtbl {
    pub const fn new<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetID<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32id: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSpatialAudioObjectBuffer_Impl::SetID(this, core::mem::transmute_copy(&u32id)).into()
            }
        }
        unsafe extern "system" fn GetID<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu32id: *mut super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSpatialAudioObjectBuffer_Impl::GetID(this, core::mem::transmute_copy(&pu32id)).into()
            }
        }
        unsafe extern "system" fn SetType<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::AudioObjectType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSpatialAudioObjectBuffer_Impl::SetType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut super::AudioObjectType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSpatialAudioObjectBuffer_Impl::GetType(this, core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn GetMetadataItems<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmetadataitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSpatialAudioObjectBuffer_Impl::GetMetadataItems(this) {
                    Ok(ok__) => {
                        ppmetadataitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppmetadataitems.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: super::IMFMediaBuffer_Vtbl::new::<Identity, OFFSET>(),
            SetID: SetID::<Identity, OFFSET>,
            GetID: GetID::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetMetadataItems: GetMetadataItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSpatialAudioObjectBuffer as windows_core::Interface>::IID || iid == &<super::IMFMediaBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "shared", feature = "spatialaudioclient", feature = "spatialaudiometadata"))]
impl windows_core::RuntimeName for IMFSpatialAudioObjectBuffer {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFSpatialAudioSample, IMFSpatialAudioSample_Vtbl, 0xabf28a9b_3393_4290_ba79_5ffc46d986b2);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFSpatialAudioSample {
    type Target = super::IMFSample;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFSpatialAudioSample, windows_core::IUnknown, super::IMFAttributes, super::IMFSample);
#[cfg(feature = "mfobjects")]
impl IMFSpatialAudioSample {
    #[cfg(feature = "shared")]
    pub unsafe fn GetObjectCount(&self, pdwobjectcount: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetObjectCount)(windows_core::Interface::as_raw(self), pdwobjectcount as _) }
    }
    pub unsafe fn AddSpatialAudioObject<P0>(&self, paudioobjbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFSpatialAudioObjectBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSpatialAudioObject)(windows_core::Interface::as_raw(self), paudioobjbuffer.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSpatialAudioObjectByIndex(&self, dwindex: super::DWORD) -> windows_core::Result<IMFSpatialAudioObjectBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpatialAudioObjectByIndex)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFSpatialAudioSample_Vtbl {
    pub base__: super::IMFSample_Vtbl,
    #[cfg(feature = "shared")]
    pub GetObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetObjectCount: usize,
    pub AddSpatialAudioObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetSpatialAudioObjectByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSpatialAudioObjectByIndex: usize,
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSpatialAudioSample_Impl: super::IMFSample_Impl {
    fn GetObjectCount(&self, pdwobjectcount: *mut super::DWORD) -> windows_core::Result<()>;
    fn AddSpatialAudioObject(&self, paudioobjbuffer: windows_core::Ref<IMFSpatialAudioObjectBuffer>) -> windows_core::Result<()>;
    fn GetSpatialAudioObjectByIndex(&self, dwindex: super::DWORD) -> windows_core::Result<IMFSpatialAudioObjectBuffer>;
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSpatialAudioSample_Vtbl {
    pub const fn new<Identity: IMFSpatialAudioSample_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectCount<Identity: IMFSpatialAudioSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwobjectcount: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSpatialAudioSample_Impl::GetObjectCount(this, core::mem::transmute_copy(&pdwobjectcount)).into()
            }
        }
        unsafe extern "system" fn AddSpatialAudioObject<Identity: IMFSpatialAudioSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paudioobjbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSpatialAudioSample_Impl::AddSpatialAudioObject(this, core::mem::transmute_copy(&paudioobjbuffer)).into()
            }
        }
        unsafe extern "system" fn GetSpatialAudioObjectByIndex<Identity: IMFSpatialAudioSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: super::DWORD, ppaudioobjbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSpatialAudioSample_Impl::GetSpatialAudioObjectByIndex(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppaudioobjbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => {
                        ppaudioobjbuffer.write(core::mem::zeroed());
                        err.into()
                    }
                }
            }
        }
        Self {
            base__: super::IMFSample_Vtbl::new::<Identity, OFFSET>(),
            GetObjectCount: GetObjectCount::<Identity, OFFSET>,
            AddSpatialAudioObject: AddSpatialAudioObject::<Identity, OFFSET>,
            GetSpatialAudioObjectByIndex: GetSpatialAudioObjectByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSpatialAudioSample as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID || iid == &<super::IMFSample as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSpatialAudioSample {}
