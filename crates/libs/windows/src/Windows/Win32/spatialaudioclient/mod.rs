pub type AudioObjectType = u32;
pub const AudioObjectType_BackCenter: AudioObjectType = 131072;
pub const AudioObjectType_BackLeft: AudioObjectType = 128;
pub const AudioObjectType_BackRight: AudioObjectType = 256;
pub const AudioObjectType_BottomBackLeft: AudioObjectType = 32768;
pub const AudioObjectType_BottomBackRight: AudioObjectType = 65536;
pub const AudioObjectType_BottomFrontLeft: AudioObjectType = 8192;
pub const AudioObjectType_BottomFrontRight: AudioObjectType = 16384;
pub const AudioObjectType_Dynamic: AudioObjectType = 1;
pub const AudioObjectType_FrontCenter: AudioObjectType = 8;
pub const AudioObjectType_FrontLeft: AudioObjectType = 2;
pub const AudioObjectType_FrontRight: AudioObjectType = 4;
pub const AudioObjectType_LowFrequency: AudioObjectType = 16;
pub const AudioObjectType_None: AudioObjectType = 0;
pub const AudioObjectType_SideLeft: AudioObjectType = 32;
pub const AudioObjectType_SideRight: AudioObjectType = 64;
pub const AudioObjectType_StereoLeft: AudioObjectType = 262144;
pub const AudioObjectType_StereoRight: AudioObjectType = 524288;
pub const AudioObjectType_TopBackLeft: AudioObjectType = 2048;
pub const AudioObjectType_TopBackRight: AudioObjectType = 4096;
pub const AudioObjectType_TopFrontLeft: AudioObjectType = 512;
pub const AudioObjectType_TopFrontRight: AudioObjectType = 1024;
windows_core::imp::define_interface!(IAudioFormatEnumerator, IAudioFormatEnumerator_Vtbl, 0xdcdaa858_895a_4a22_a5eb_67bda506096d);
windows_core::imp::interface_hierarchy!(IAudioFormatEnumerator, windows_core::IUnknown);
impl IAudioFormatEnumerator {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_mmeapi")]
    pub unsafe fn GetFormat(&self, index: u32) -> windows_core::Result<*mut super::mmeapi::WAVEFORMATEX> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFormat)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFormatEnumerator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_mmeapi")]
    pub GetFormat: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_mmeapi"))]
    GetFormat: usize,
}
#[cfg(feature = "Win32_mmeapi")]
pub trait IAudioFormatEnumerator_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetFormat(&self, index: u32) -> windows_core::Result<*mut super::mmeapi::WAVEFORMATEX>;
}
#[cfg(feature = "Win32_mmeapi")]
impl IAudioFormatEnumerator_Vtbl {
    pub const fn new<Identity: IAudioFormatEnumerator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IAudioFormatEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioFormatEnumerator_Impl::GetCount(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFormat<Identity: IAudioFormatEnumerator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, format: *mut *mut super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioFormatEnumerator_Impl::GetFormat(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        format.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCount: GetCount::<Identity, OFFSET>, GetFormat: GetFormat::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioFormatEnumerator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_mmeapi")]
impl windows_core::RuntimeName for IAudioFormatEnumerator {}
windows_core::imp::define_interface!(ISpatialAudioClient, ISpatialAudioClient_Vtbl, 0xbbf8e066_aaaa_49be_9a4d_fd2a858ea27f);
windows_core::imp::interface_hierarchy!(ISpatialAudioClient, windows_core::IUnknown);
impl ISpatialAudioClient {
    pub unsafe fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStaticObjectPosition)(windows_core::Interface::as_raw(self), r#type, x as _, y as _, z as _) }
    }
    pub unsafe fn GetNativeStaticObjectTypeMask(&self) -> windows_core::Result<AudioObjectType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNativeStaticObjectTypeMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaxDynamicObjectCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxDynamicObjectCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSupportedAudioObjectFormatEnumerator(&self) -> windows_core::Result<IAudioFormatEnumerator> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedAudioObjectFormatEnumerator)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_mmeapi")]
    pub unsafe fn GetMaxFrameCount(&self, objectformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxFrameCount)(windows_core::Interface::as_raw(self), objectformat, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_mmeapi")]
    pub unsafe fn IsAudioObjectFormatSupported(&self, objectformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsAudioObjectFormatSupported)(windows_core::Interface::as_raw(self), objectformat) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const windows_core::GUID, auxiliaryinfo: Option<*const super::propidlbase::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsSpatialAudioStreamAvailable)(windows_core::Interface::as_raw(self), streamuuid, auxiliaryinfo.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn ActivateSpatialAudioStream<T>(&self, activationparams: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ActivateSpatialAudioStream)(windows_core::Interface::as_raw(self), core::mem::transmute(activationparams), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStaticObjectPosition: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut f32, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetNativeStaticObjectTypeMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioObjectType) -> windows_core::HRESULT,
    pub GetMaxDynamicObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSupportedAudioObjectFormatEnumerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_mmeapi")]
    pub GetMaxFrameCount: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::mmeapi::WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_mmeapi"))]
    GetMaxFrameCount: usize,
    #[cfg(feature = "Win32_mmeapi")]
    pub IsAudioObjectFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_mmeapi"))]
    IsAudioObjectFormatSupported: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub IsSpatialAudioStreamAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    IsSpatialAudioStreamAvailable: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub ActivateSpatialAudioStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::propidlbase::PROPVARIANT, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    ActivateSpatialAudioStream: usize,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmeapi", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISpatialAudioClient_Impl: windows_core::IUnknownImpl {
    fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> windows_core::Result<()>;
    fn GetNativeStaticObjectTypeMask(&self) -> windows_core::Result<AudioObjectType>;
    fn GetMaxDynamicObjectCount(&self) -> windows_core::Result<u32>;
    fn GetSupportedAudioObjectFormatEnumerator(&self) -> windows_core::Result<IAudioFormatEnumerator>;
    fn GetMaxFrameCount(&self, objectformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::Result<u32>;
    fn IsAudioObjectFormatSupported(&self, objectformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::Result<()>;
    fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const windows_core::GUID, auxiliaryinfo: *const super::propidlbase::PROPVARIANT) -> windows_core::Result<()>;
    fn ActivateSpatialAudioStream(&self, activationparams: *const super::propidlbase::PROPVARIANT, riid: *const windows_core::GUID, stream: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmeapi", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISpatialAudioClient_Vtbl {
    pub const fn new<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStaticObjectPosition<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioClient_Impl::GetStaticObjectPosition(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z)).into()
            }
        }
        unsafe extern "system" fn GetNativeStaticObjectTypeMask<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mask: *mut AudioObjectType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient_Impl::GetNativeStaticObjectTypeMask(this) {
                    Ok(ok__) => {
                        mask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxDynamicObjectCount<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient_Impl::GetMaxDynamicObjectCount(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedAudioObjectFormatEnumerator<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enumerator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient_Impl::GetSupportedAudioObjectFormatEnumerator(this) {
                    Ok(ok__) => {
                        enumerator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxFrameCount<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectformat: *const super::mmeapi::WAVEFORMATEX, framecountperbuffer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient_Impl::GetMaxFrameCount(this, core::mem::transmute_copy(&objectformat)) {
                    Ok(ok__) => {
                        framecountperbuffer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsAudioObjectFormatSupported<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, objectformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioClient_Impl::IsAudioObjectFormatSupported(this, core::mem::transmute_copy(&objectformat)).into()
            }
        }
        unsafe extern "system" fn IsSpatialAudioStreamAvailable<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamuuid: *const windows_core::GUID, auxiliaryinfo: *const super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioClient_Impl::IsSpatialAudioStreamAvailable(this, core::mem::transmute_copy(&streamuuid), core::mem::transmute_copy(&auxiliaryinfo)).into()
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioStream<Identity: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activationparams: *const super::propidlbase::PROPVARIANT, riid: *const windows_core::GUID, stream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioClient_Impl::ActivateSpatialAudioStream(this, core::mem::transmute_copy(&activationparams), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&stream)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStaticObjectPosition: GetStaticObjectPosition::<Identity, OFFSET>,
            GetNativeStaticObjectTypeMask: GetNativeStaticObjectTypeMask::<Identity, OFFSET>,
            GetMaxDynamicObjectCount: GetMaxDynamicObjectCount::<Identity, OFFSET>,
            GetSupportedAudioObjectFormatEnumerator: GetSupportedAudioObjectFormatEnumerator::<Identity, OFFSET>,
            GetMaxFrameCount: GetMaxFrameCount::<Identity, OFFSET>,
            IsAudioObjectFormatSupported: IsAudioObjectFormatSupported::<Identity, OFFSET>,
            IsSpatialAudioStreamAvailable: IsSpatialAudioStreamAvailable::<Identity, OFFSET>,
            ActivateSpatialAudioStream: ActivateSpatialAudioStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioClient as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_mmeapi", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISpatialAudioClient {}
windows_core::imp::define_interface!(ISpatialAudioClient2, ISpatialAudioClient2_Vtbl, 0xcaabe452_a66a_4bee_a93e_e320463f6a53);
impl core::ops::Deref for ISpatialAudioClient2 {
    type Target = ISpatialAudioClient;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioClient2, windows_core::IUnknown, ISpatialAudioClient);
impl ISpatialAudioClient2 {
    #[cfg(feature = "Win32_audiosessiontypes")]
    pub unsafe fn IsOffloadCapable(&self, category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOffloadCapable)(windows_core::Interface::as_raw(self), category, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi"))]
    pub unsafe fn GetMaxFrameCountForCategory(&self, category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY, offloadenabled: bool, objectformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxFrameCountForCategory)(windows_core::Interface::as_raw(self), category, offloadenabled.into(), objectformat, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioClient2_Vtbl {
    pub base__: ISpatialAudioClient_Vtbl,
    #[cfg(feature = "Win32_audiosessiontypes")]
    pub IsOffloadCapable: unsafe extern "system" fn(*mut core::ffi::c_void, super::audiosessiontypes::AUDIO_STREAM_CATEGORY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_audiosessiontypes"))]
    IsOffloadCapable: usize,
    #[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi"))]
    pub GetMaxFrameCountForCategory: unsafe extern "system" fn(*mut core::ffi::c_void, super::audiosessiontypes::AUDIO_STREAM_CATEGORY, windows_core::BOOL, *const super::mmeapi::WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi")))]
    GetMaxFrameCountForCategory: usize,
}
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_minwindef", feature = "Win32_mmeapi", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISpatialAudioClient2_Impl: ISpatialAudioClient_Impl {
    fn IsOffloadCapable(&self, category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY) -> windows_core::Result<windows_core::BOOL>;
    fn GetMaxFrameCountForCategory(&self, category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY, offloadenabled: windows_core::BOOL, objectformat: *const super::mmeapi::WAVEFORMATEX) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_minwindef", feature = "Win32_mmeapi", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISpatialAudioClient2_Vtbl {
    pub const fn new<Identity: ISpatialAudioClient2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsOffloadCapable<Identity: ISpatialAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient2_Impl::IsOffloadCapable(this, core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        isoffloadcapable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaxFrameCountForCategory<Identity: ISpatialAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY, offloadenabled: windows_core::BOOL, objectformat: *const super::mmeapi::WAVEFORMATEX, framecountperbuffer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioClient2_Impl::GetMaxFrameCountForCategory(this, core::mem::transmute_copy(&category), core::mem::transmute_copy(&offloadenabled), core::mem::transmute_copy(&objectformat)) {
                    Ok(ok__) => {
                        framecountperbuffer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISpatialAudioClient_Vtbl::new::<Identity, OFFSET>(),
            IsOffloadCapable: IsOffloadCapable::<Identity, OFFSET>,
            GetMaxFrameCountForCategory: GetMaxFrameCountForCategory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioClient2 as windows_core::Interface>::IID || iid == &<ISpatialAudioClient as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_minwindef", feature = "Win32_mmeapi", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISpatialAudioClient2 {}
windows_core::imp::define_interface!(ISpatialAudioObject, ISpatialAudioObject_Vtbl, 0xdde28967_521b_46e5_8f00_bd6f2bc8ab1d);
impl core::ops::Deref for ISpatialAudioObject {
    type Target = ISpatialAudioObjectBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObject, windows_core::IUnknown, ISpatialAudioObjectBase);
impl ISpatialAudioObject {
    pub unsafe fn SetPosition(&self, x: f32, y: f32, z: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), x, y, z) }
    }
    pub unsafe fn SetVolume(&self, volume: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), volume) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObject_Vtbl {
    pub base__: ISpatialAudioObjectBase_Vtbl,
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObject_Impl: ISpatialAudioObjectBase_Impl {
    fn SetPosition(&self, x: f32, y: f32, z: f32) -> windows_core::Result<()>;
    fn SetVolume(&self, volume: f32) -> windows_core::Result<()>;
}
impl ISpatialAudioObject_Vtbl {
    pub const fn new<Identity: ISpatialAudioObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPosition<Identity: ISpatialAudioObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObject_Impl::SetPosition(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z)).into()
            }
        }
        unsafe extern "system" fn SetVolume<Identity: ISpatialAudioObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volume: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObject_Impl::SetVolume(this, core::mem::transmute_copy(&volume)).into()
            }
        }
        Self {
            base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, OFFSET>(),
            SetPosition: SetPosition::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObject as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioObject {}
windows_core::imp::define_interface!(ISpatialAudioObjectBase, ISpatialAudioObjectBase_Vtbl, 0xcce0b8f2_8d4d_4efb_a8cf_3d6ecf1c30e0);
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectBase, windows_core::IUnknown);
impl ISpatialAudioObjectBase {
    pub unsafe fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), buffer as _, bufferlength as _) }
    }
    pub unsafe fn SetEndOfStream(&self, framecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEndOfStream)(windows_core::Interface::as_raw(self), framecount) }
    }
    pub unsafe fn IsActive(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsActive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAudioObjectType(&self) -> windows_core::Result<AudioObjectType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioObjectType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAudioObjectType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AudioObjectType) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectBase_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> windows_core::Result<()>;
    fn SetEndOfStream(&self, framecount: u32) -> windows_core::Result<()>;
    fn IsActive(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetAudioObjectType(&self) -> windows_core::Result<AudioObjectType>;
}
impl ISpatialAudioObjectBase_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut *mut u8, bufferlength: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectBase_Impl::GetBuffer(this, core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&bufferlength)).into()
            }
        }
        unsafe extern "system" fn SetEndOfStream<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, framecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectBase_Impl::SetEndOfStream(this, core::mem::transmute_copy(&framecount)).into()
            }
        }
        unsafe extern "system" fn IsActive<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isactive: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectBase_Impl::IsActive(this) {
                    Ok(ok__) => {
                        isactive.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAudioObjectType<Identity: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioobjecttype: *mut AudioObjectType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectBase_Impl::GetAudioObjectType(this) {
                    Ok(ok__) => {
                        audioobjecttype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            SetEndOfStream: SetEndOfStream::<Identity, OFFSET>,
            IsActive: IsActive::<Identity, OFFSET>,
            GetAudioObjectType: GetAudioObjectType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioObjectBase {}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStream, ISpatialAudioObjectRenderStream_Vtbl, 0xbab5f473_b423_477b_85f5_b5a332a04153);
impl core::ops::Deref for ISpatialAudioObjectRenderStream {
    type Target = ISpatialAudioObjectRenderStreamBase;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStream, windows_core::IUnknown, ISpatialAudioObjectRenderStreamBase);
impl ISpatialAudioObjectRenderStream {
    pub unsafe fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObject> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActivateSpatialAudioObject)(windows_core::Interface::as_raw(self), r#type, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStream_Vtbl {
    pub base__: ISpatialAudioObjectRenderStreamBase_Vtbl,
    pub ActivateSpatialAudioObject: unsafe extern "system" fn(*mut core::ffi::c_void, AudioObjectType, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectRenderStream_Impl: ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> windows_core::Result<ISpatialAudioObject>;
}
impl ISpatialAudioObjectRenderStream_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectRenderStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ActivateSpatialAudioObject<Identity: ISpatialAudioObjectRenderStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectRenderStream_Impl::ActivateSpatialAudioObject(this, core::mem::transmute_copy(&r#type)) {
                    Ok(ok__) => {
                        audioobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, OFFSET>(),
            ActivateSpatialAudioObject: ActivateSpatialAudioObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStream as windows_core::Interface>::IID || iid == &<ISpatialAudioObjectRenderStreamBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStream {}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStreamBase, ISpatialAudioObjectRenderStreamBase_Vtbl, 0xfeaaf403_c1d8_450d_aa05_e0ccee7502a8);
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamBase, windows_core::IUnknown);
impl ISpatialAudioObjectRenderStreamBase {
    pub unsafe fn GetAvailableDynamicObjectCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAvailableDynamicObjectCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetService<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn Start(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BeginUpdatingAudioObjects)(windows_core::Interface::as_raw(self), availabledynamicobjectcount as _, framecountperbuffer as _) }
    }
    pub unsafe fn EndUpdatingAudioObjects(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EndUpdatingAudioObjects)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamBase_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAvailableDynamicObjectCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BeginUpdatingAudioObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub EndUpdatingAudioObjects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectRenderStreamBase_Impl: windows_core::IUnknownImpl {
    fn GetAvailableDynamicObjectCount(&self) -> windows_core::Result<u32>;
    fn GetService(&self, riid: *const windows_core::GUID, service: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> windows_core::Result<()>;
    fn EndUpdatingAudioObjects(&self) -> windows_core::Result<()>;
}
impl ISpatialAudioObjectRenderStreamBase_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAvailableDynamicObjectCount<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpatialAudioObjectRenderStreamBase_Impl::GetAvailableDynamicObjectCount(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetService<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, service: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::GetService(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&service)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn BeginUpdatingAudioObjects<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::BeginUpdatingAudioObjects(this, core::mem::transmute_copy(&availabledynamicobjectcount), core::mem::transmute_copy(&framecountperbuffer)).into()
            }
        }
        unsafe extern "system" fn EndUpdatingAudioObjects<Identity: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamBase_Impl::EndUpdatingAudioObjects(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAvailableDynamicObjectCount: GetAvailableDynamicObjectCount::<Identity, OFFSET>,
            GetService: GetService::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            BeginUpdatingAudioObjects: BeginUpdatingAudioObjects::<Identity, OFFSET>,
            EndUpdatingAudioObjects: EndUpdatingAudioObjects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamBase as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamBase {}
windows_core::imp::define_interface!(ISpatialAudioObjectRenderStreamNotify, ISpatialAudioObjectRenderStreamNotify_Vtbl, 0xdddf83e6_68d7_4c70_883f_a1836afb4a50);
windows_core::imp::interface_hierarchy!(ISpatialAudioObjectRenderStreamNotify, windows_core::IUnknown);
impl ISpatialAudioObjectRenderStreamNotify {
    pub unsafe fn OnAvailableDynamicObjectCountChange<P0>(&self, sender: P0, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISpatialAudioObjectRenderStreamBase>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnAvailableDynamicObjectCountChange)(windows_core::Interface::as_raw(self), sender.param().abi(), hnscompliancedeadlinetime, availabledynamicobjectcountchange) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAvailableDynamicObjectCountChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i64, u32) -> windows_core::HRESULT,
}
pub trait ISpatialAudioObjectRenderStreamNotify_Impl: windows_core::IUnknownImpl {
    fn OnAvailableDynamicObjectCountChange(&self, sender: windows_core::Ref<ISpatialAudioObjectRenderStreamBase>, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> windows_core::Result<()>;
}
impl ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub const fn new<Identity: ISpatialAudioObjectRenderStreamNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAvailableDynamicObjectCountChange<Identity: ISpatialAudioObjectRenderStreamNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISpatialAudioObjectRenderStreamNotify_Impl::OnAvailableDynamicObjectCountChange(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&hnscompliancedeadlinetime), core::mem::transmute_copy(&availabledynamicobjectcountchange)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAvailableDynamicObjectCountChange: OnAvailableDynamicObjectCountChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpatialAudioObjectRenderStreamNotify {}
pub type SPATIAL_AUDIO_STREAM_OPTIONS = u32;
pub const SPATIAL_AUDIO_STREAM_OPTIONS_NONE: SPATIAL_AUDIO_STREAM_OPTIONS = 0;
pub const SPATIAL_AUDIO_STREAM_OPTIONS_OFFLOAD: SPATIAL_AUDIO_STREAM_OPTIONS = 1;
pub const SPTLAUDCLNT_E_DESTROYED: i32 = -2004287232;
pub const SPTLAUDCLNT_E_ERRORS_IN_OBJECT_CALLS: i32 = -2004287227;
pub const SPTLAUDCLNT_E_INTERNAL: i32 = -2004287219;
pub const SPTLAUDCLNT_E_INVALID_LICENSE: i32 = -2004287224;
pub const SPTLAUDCLNT_E_METADATA_FORMAT_NOT_SUPPORTED: i32 = -2004287226;
pub const SPTLAUDCLNT_E_NO_MORE_OBJECTS: i32 = -2004287229;
pub const SPTLAUDCLNT_E_OBJECT_ALREADY_ACTIVE: i32 = -2004287220;
pub const SPTLAUDCLNT_E_OUT_OF_ORDER: i32 = -2004287231;
pub const SPTLAUDCLNT_E_PROPERTY_NOT_SUPPORTED: i32 = -2004287228;
pub const SPTLAUDCLNT_E_RESOURCES_INVALIDATED: i32 = -2004287230;
pub const SPTLAUDCLNT_E_STATIC_OBJECT_NOT_AVAILABLE: i32 = -2004287221;
pub const SPTLAUDCLNT_E_STREAM_NOT_AVAILABLE: i32 = -2004287225;
pub const SPTLAUDCLNT_E_STREAM_NOT_STOPPED: i32 = -2004287222;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SpatialAudioClientActivationParams {
    pub tracingContextId: windows_core::GUID,
    pub appId: windows_core::GUID,
    pub majorVersion: i32,
    pub minorVersion1: i32,
    pub minorVersion2: i32,
    pub minorVersion3: i32,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi", feature = "Win32_winnt"))]
pub struct SpatialAudioObjectRenderStreamActivationParams {
    pub ObjectFormat: *const super::mmeapi::WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::winnt::HANDLE,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
}
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi", feature = "Win32_winnt"))]
impl Default for SpatialAudioObjectRenderStreamActivationParams {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi", feature = "Win32_winnt"))]
pub struct SpatialAudioObjectRenderStreamActivationParams2 {
    pub ObjectFormat: *const super::mmeapi::WAVEFORMATEX,
    pub StaticObjectTypeMask: AudioObjectType,
    pub MinDynamicObjectCount: u32,
    pub MaxDynamicObjectCount: u32,
    pub Category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY,
    pub EventHandle: super::winnt::HANDLE,
    pub NotifyObject: core::mem::ManuallyDrop<Option<ISpatialAudioObjectRenderStreamNotify>>,
    pub Options: SPATIAL_AUDIO_STREAM_OPTIONS,
}
#[cfg(all(feature = "Win32_audiosessiontypes", feature = "Win32_mmeapi", feature = "Win32_winnt"))]
impl Default for SpatialAudioObjectRenderStreamActivationParams2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
