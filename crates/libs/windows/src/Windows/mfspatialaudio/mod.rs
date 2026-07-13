#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFSpatialAudioObjectBuffer, IMFSpatialAudioObjectBuffer_Vtbl, 0xd396ec8c_605e_4249_978d_72ad1c312872);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFSpatialAudioObjectBuffer {
    type Target = super::mfobjects::IMFMediaBuffer;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFSpatialAudioObjectBuffer, windows_core::IUnknown, super::mfobjects::IMFMediaBuffer);
#[cfg(feature = "mfobjects")]
impl IMFSpatialAudioObjectBuffer {
    pub unsafe fn SetID(&self, u32id: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetID)(windows_core::Interface::as_raw(self), u32id) }
    }
    pub unsafe fn GetID(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "spatialaudioclient")]
    pub unsafe fn SetType(&self, r#type: super::spatialaudioclient::AudioObjectType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetType)(windows_core::Interface::as_raw(self), r#type) }
    }
    #[cfg(feature = "spatialaudioclient")]
    pub unsafe fn GetType(&self) -> windows_core::Result<super::spatialaudioclient::AudioObjectType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "spatialaudiometadata")]
    pub unsafe fn GetMetadataItems(&self) -> windows_core::Result<super::spatialaudiometadata::ISpatialAudioMetadataItems> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFSpatialAudioObjectBuffer_Vtbl {
    pub base__: super::mfobjects::IMFMediaBuffer_Vtbl,
    pub SetID: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "spatialaudioclient")]
    pub SetType: unsafe extern "system" fn(*mut core::ffi::c_void, super::spatialaudioclient::AudioObjectType) -> windows_core::HRESULT,
    #[cfg(not(feature = "spatialaudioclient"))]
    SetType: usize,
    #[cfg(feature = "spatialaudioclient")]
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::spatialaudioclient::AudioObjectType) -> windows_core::HRESULT,
    #[cfg(not(feature = "spatialaudioclient"))]
    GetType: usize,
    #[cfg(feature = "spatialaudiometadata")]
    pub GetMetadataItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "spatialaudiometadata"))]
    GetMetadataItems: usize,
}
#[cfg(all(feature = "mfobjects", feature = "spatialaudioclient", feature = "spatialaudiometadata"))]
pub trait IMFSpatialAudioObjectBuffer_Impl: super::mfobjects::IMFMediaBuffer_Impl {
    fn SetID(&self, u32id: u32) -> windows_core::Result<()>;
    fn GetID(&self) -> windows_core::Result<u32>;
    fn SetType(&self, r#type: super::spatialaudioclient::AudioObjectType) -> windows_core::Result<()>;
    fn GetType(&self) -> windows_core::Result<super::spatialaudioclient::AudioObjectType>;
    fn GetMetadataItems(&self) -> windows_core::Result<super::spatialaudiometadata::ISpatialAudioMetadataItems>;
}
#[cfg(all(feature = "mfobjects", feature = "spatialaudioclient", feature = "spatialaudiometadata"))]
impl IMFSpatialAudioObjectBuffer_Vtbl {
    pub const fn new<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetID<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, u32id: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSpatialAudioObjectBuffer_Impl::SetID(this, core::mem::transmute_copy(&u32id)).into()
            }
        }
        unsafe extern "system" fn GetID<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu32id: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSpatialAudioObjectBuffer_Impl::GetID(this) {
                    Ok(ok__) => {
                        pu32id.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetType<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: super::spatialaudioclient::AudioObjectType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSpatialAudioObjectBuffer_Impl::SetType(this, core::mem::transmute_copy(&r#type)).into()
            }
        }
        unsafe extern "system" fn GetType<Identity: IMFSpatialAudioObjectBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut super::spatialaudioclient::AudioObjectType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSpatialAudioObjectBuffer_Impl::GetType(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::mfobjects::IMFMediaBuffer_Vtbl::new::<Identity, OFFSET>(),
            SetID: SetID::<Identity, OFFSET>,
            GetID: GetID::<Identity, OFFSET>,
            SetType: SetType::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetMetadataItems: GetMetadataItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSpatialAudioObjectBuffer as windows_core::Interface>::IID || iid == &<super::mfobjects::IMFMediaBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "spatialaudioclient", feature = "spatialaudiometadata"))]
impl windows_core::RuntimeName for IMFSpatialAudioObjectBuffer {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFSpatialAudioSample, IMFSpatialAudioSample_Vtbl, 0xabf28a9b_3393_4290_ba79_5ffc46d986b2);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFSpatialAudioSample {
    type Target = super::mfobjects::IMFSample;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFSpatialAudioSample, windows_core::IUnknown, super::mfobjects::IMFAttributes, super::mfobjects::IMFSample);
#[cfg(feature = "mfobjects")]
impl IMFSpatialAudioSample {
    pub unsafe fn GetObjectCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddSpatialAudioObject<P0>(&self, paudioobjbuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFSpatialAudioObjectBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSpatialAudioObject)(windows_core::Interface::as_raw(self), paudioobjbuffer.param().abi()) }
    }
    pub unsafe fn GetSpatialAudioObjectByIndex(&self, dwindex: u32) -> windows_core::Result<IMFSpatialAudioObjectBuffer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSpatialAudioObjectByIndex)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFSpatialAudioSample_Vtbl {
    pub base__: super::mfobjects::IMFSample_Vtbl,
    pub GetObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub AddSpatialAudioObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSpatialAudioObjectByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSpatialAudioSample_Impl: super::mfobjects::IMFSample_Impl {
    fn GetObjectCount(&self) -> windows_core::Result<u32>;
    fn AddSpatialAudioObject(&self, paudioobjbuffer: windows_core::Ref<IMFSpatialAudioObjectBuffer>) -> windows_core::Result<()>;
    fn GetSpatialAudioObjectByIndex(&self, dwindex: u32) -> windows_core::Result<IMFSpatialAudioObjectBuffer>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSpatialAudioSample_Vtbl {
    pub const fn new<Identity: IMFSpatialAudioSample_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectCount<Identity: IMFSpatialAudioSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwobjectcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSpatialAudioSample_Impl::GetObjectCount(this) {
                    Ok(ok__) => {
                        pdwobjectcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddSpatialAudioObject<Identity: IMFSpatialAudioSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paudioobjbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSpatialAudioSample_Impl::AddSpatialAudioObject(this, core::mem::transmute_copy(&paudioobjbuffer)).into()
            }
        }
        unsafe extern "system" fn GetSpatialAudioObjectByIndex<Identity: IMFSpatialAudioSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppaudioobjbuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSpatialAudioSample_Impl::GetSpatialAudioObjectByIndex(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppaudioobjbuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::mfobjects::IMFSample_Vtbl::new::<Identity, OFFSET>(),
            GetObjectCount: GetObjectCount::<Identity, OFFSET>,
            AddSpatialAudioObject: AddSpatialAudioObject::<Identity, OFFSET>,
            GetSpatialAudioObjectByIndex: GetSpatialAudioObjectByIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSpatialAudioSample as windows_core::Interface>::IID || iid == &<super::mfobjects::IMFAttributes as windows_core::Interface>::IID || iid == &<super::mfobjects::IMFSample as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSpatialAudioSample {}
