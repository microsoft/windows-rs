#[cfg(feature = "windef")]
#[inline]
pub unsafe fn MFPCreateMediaPlayer<P0, P3>(pwszurl: P0, fstartplayback: bool, creationoptions: Option<MFP_CREATION_OPTIONS>, pcallback: P3, hwnd: Option<super::HWND>, ppmediaplayer: Option<*mut Option<IMFPMediaPlayer>>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<IMFPMediaPlayerCallback>,
{
    windows_core::link!("mfplay.dll" "system" fn MFPCreateMediaPlayer(pwszurl : windows_core::PCWSTR, fstartplayback : windows_core::BOOL, creationoptions : MFP_CREATION_OPTIONS, pcallback : *mut core::ffi::c_void, hwnd : super::HWND, ppmediaplayer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFPCreateMediaPlayer(pwszurl.param().abi(), fstartplayback.into(), creationoptions.unwrap_or(core::mem::zeroed()) as _, pcallback.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _, ppmediaplayer.unwrap_or(core::mem::zeroed()) as _) }
}
windows_core::imp::define_interface!(IMFPMediaItem, IMFPMediaItem_Vtbl, 0x90eb3e6b_ecbf_45cc_b1da_c6fe3ea70d57);
windows_core::imp::interface_hierarchy!(IMFPMediaItem, windows_core::IUnknown);
impl IMFPMediaItem {
    pub unsafe fn GetMediaPlayer(&self) -> windows_core::Result<IMFPMediaPlayer> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaPlayer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetURL(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetObject(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetUserData(&self) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUserData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUserData(&self, dwuserdata: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUserData)(windows_core::Interface::as_raw(self), dwuserdata) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetStartStopPosition(&self, pguidstartpositiontype: Option<*mut windows_core::GUID>, pvstartvalue: Option<*mut super::PROPVARIANT>, pguidstoppositiontype: Option<*mut windows_core::GUID>, pvstopvalue: Option<*mut super::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStartStopPosition)(windows_core::Interface::as_raw(self), pguidstartpositiontype.unwrap_or(core::mem::zeroed()) as _, pvstartvalue.unwrap_or(core::mem::zeroed()) as _, pguidstoppositiontype.unwrap_or(core::mem::zeroed()) as _, pvstopvalue.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetStartStopPosition(&self, pguidstartpositiontype: Option<*const windows_core::GUID>, pvstartvalue: Option<*const super::PROPVARIANT>, pguidstoppositiontype: Option<*const windows_core::GUID>, pvstopvalue: Option<*const super::PROPVARIANT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartStopPosition)(windows_core::Interface::as_raw(self), pguidstartpositiontype.unwrap_or(core::mem::zeroed()) as _, pvstartvalue.unwrap_or(core::mem::zeroed()) as _, pguidstoppositiontype.unwrap_or(core::mem::zeroed()) as _, pvstopvalue.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn HasVideo(&self, pfhasvideo: Option<*mut windows_core::BOOL>, pfselected: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HasVideo)(windows_core::Interface::as_raw(self), pfhasvideo.unwrap_or(core::mem::zeroed()) as _, pfselected.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn HasAudio(&self, pfhasaudio: Option<*mut windows_core::BOOL>, pfselected: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HasAudio)(windows_core::Interface::as_raw(self), pfhasaudio.unwrap_or(core::mem::zeroed()) as _, pfselected.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn IsProtected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsProtected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetDuration(&self, guidpositiontype: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self), guidpositiontype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetNumberOfStreams(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfStreams)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStreamSelection(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStreamSelection(&self, dwstreamindex: u32, fenabled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, fenabled.into()) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetStreamAttribute(&self, dwstreamindex: u32, guidmfattribute: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamAttribute)(windows_core::Interface::as_raw(self), dwstreamindex, guidmfattribute, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPresentationAttribute(&self, guidmfattribute: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPresentationAttribute)(windows_core::Interface::as_raw(self), guidmfattribute, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCharacteristics(&self) -> windows_core::Result<MFP_MEDIAITEM_CHARACTERISTICS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStreamSink<P1>(&self, dwstreamindex: u32, pmediasink: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSink)(windows_core::Interface::as_raw(self), dwstreamindex, pmediasink.param().abi()) }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn GetMetadata(&self) -> windows_core::Result<super::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadata)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPMediaItem_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMediaPlayer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUserData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize) -> windows_core::HRESULT,
    pub SetUserData: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetStartStopPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID, *mut super::PROPVARIANT, *mut windows_core::GUID, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetStartStopPosition: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetStartStopPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::PROPVARIANT, *const windows_core::GUID, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetStartStopPosition: usize,
    pub HasVideo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub HasAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetDuration: usize,
    pub GetNumberOfStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetStreamAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetStreamAttribute: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPresentationAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetPresentationAttribute: usize,
    pub GetCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFP_MEDIAITEM_CHARACTERISTICS) -> windows_core::HRESULT,
    pub SetStreamSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "propsys")]
    pub GetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    GetMetadata: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "propsys", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFPMediaItem_Impl: windows_core::IUnknownImpl {
    fn GetMediaPlayer(&self) -> windows_core::Result<IMFPMediaPlayer>;
    fn GetURL(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetObject(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetUserData(&self) -> windows_core::Result<usize>;
    fn SetUserData(&self, dwuserdata: usize) -> windows_core::Result<()>;
    fn GetStartStopPosition(&self, pguidstartpositiontype: *mut windows_core::GUID, pvstartvalue: *mut super::PROPVARIANT, pguidstoppositiontype: *mut windows_core::GUID, pvstopvalue: *mut super::PROPVARIANT) -> windows_core::Result<()>;
    fn SetStartStopPosition(&self, pguidstartpositiontype: *const windows_core::GUID, pvstartvalue: *const super::PROPVARIANT, pguidstoppositiontype: *const windows_core::GUID, pvstopvalue: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn HasVideo(&self, pfhasvideo: *mut windows_core::BOOL, pfselected: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn HasAudio(&self, pfhasaudio: *mut windows_core::BOOL, pfselected: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn IsProtected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetDuration(&self, guidpositiontype: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT>;
    fn GetNumberOfStreams(&self) -> windows_core::Result<u32>;
    fn GetStreamSelection(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL>;
    fn SetStreamSelection(&self, dwstreamindex: u32, fenabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetStreamAttribute(&self, dwstreamindex: u32, guidmfattribute: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT>;
    fn GetPresentationAttribute(&self, guidmfattribute: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT>;
    fn GetCharacteristics(&self) -> windows_core::Result<MFP_MEDIAITEM_CHARACTERISTICS>;
    fn SetStreamSink(&self, dwstreamindex: u32, pmediasink: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetMetadata(&self) -> windows_core::Result<super::IPropertyStore>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "propsys", feature = "wtypes", feature = "wtypesbase"))]
impl IMFPMediaItem_Vtbl {
    pub const fn new<Identity: IMFPMediaItem_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMediaPlayer<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediaplayer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetMediaPlayer(this) {
                    Ok(ok__) => {
                        ppmediaplayer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetURL<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetURL(this) {
                    Ok(ok__) => {
                        ppwszurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObject<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppiunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetObject(this) {
                    Ok(ok__) => {
                        ppiunknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUserData<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwuserdata: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetUserData(this) {
                    Ok(ok__) => {
                        pdwuserdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUserData<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwuserdata: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaItem_Impl::SetUserData(this, core::mem::transmute_copy(&dwuserdata)).into()
            }
        }
        unsafe extern "system" fn GetStartStopPosition<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidstartpositiontype: *mut windows_core::GUID, pvstartvalue: *mut super::PROPVARIANT, pguidstoppositiontype: *mut windows_core::GUID, pvstopvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaItem_Impl::GetStartStopPosition(this, core::mem::transmute_copy(&pguidstartpositiontype), core::mem::transmute_copy(&pvstartvalue), core::mem::transmute_copy(&pguidstoppositiontype), core::mem::transmute_copy(&pvstopvalue)).into()
            }
        }
        unsafe extern "system" fn SetStartStopPosition<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidstartpositiontype: *const windows_core::GUID, pvstartvalue: *const super::PROPVARIANT, pguidstoppositiontype: *const windows_core::GUID, pvstopvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaItem_Impl::SetStartStopPosition(this, core::mem::transmute_copy(&pguidstartpositiontype), core::mem::transmute_copy(&pvstartvalue), core::mem::transmute_copy(&pguidstoppositiontype), core::mem::transmute_copy(&pvstopvalue)).into()
            }
        }
        unsafe extern "system" fn HasVideo<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasvideo: *mut windows_core::BOOL, pfselected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaItem_Impl::HasVideo(this, core::mem::transmute_copy(&pfhasvideo), core::mem::transmute_copy(&pfselected)).into()
            }
        }
        unsafe extern "system" fn HasAudio<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfhasaudio: *mut windows_core::BOOL, pfselected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaItem_Impl::HasAudio(this, core::mem::transmute_copy(&pfhasaudio), core::mem::transmute_copy(&pfselected)).into()
            }
        }
        unsafe extern "system" fn IsProtected<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfprotected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::IsProtected(this) {
                    Ok(ok__) => {
                        pfprotected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDuration<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpositiontype: *const windows_core::GUID, pvdurationvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetDuration(this, core::mem::transmute_copy(&guidpositiontype)) {
                    Ok(ok__) => {
                        pvdurationvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberOfStreams<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstreamcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetNumberOfStreams(this) {
                    Ok(ok__) => {
                        pdwstreamcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamSelection<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetStreamSelection(this, core::mem::transmute_copy(&dwstreamindex)) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamSelection<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, fenabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaItem_Impl::SetStreamSelection(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&fenabled)).into()
            }
        }
        unsafe extern "system" fn GetStreamAttribute<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, guidmfattribute: *const windows_core::GUID, pvvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetStreamAttribute(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&guidmfattribute)) {
                    Ok(ok__) => {
                        pvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresentationAttribute<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmfattribute: *const windows_core::GUID, pvvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetPresentationAttribute(this, core::mem::transmute_copy(&guidmfattribute)) {
                    Ok(ok__) => {
                        pvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCharacteristics<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcharacteristics: *mut MFP_MEDIAITEM_CHARACTERISTICS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetCharacteristics(this) {
                    Ok(ok__) => {
                        pcharacteristics.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamSink<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, pmediasink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaItem_Impl::SetStreamSink(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&pmediasink)).into()
            }
        }
        unsafe extern "system" fn GetMetadata<Identity: IMFPMediaItem_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmetadatastore: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaItem_Impl::GetMetadata(this) {
                    Ok(ok__) => {
                        ppmetadatastore.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMediaPlayer: GetMediaPlayer::<Identity, OFFSET>,
            GetURL: GetURL::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetUserData: GetUserData::<Identity, OFFSET>,
            SetUserData: SetUserData::<Identity, OFFSET>,
            GetStartStopPosition: GetStartStopPosition::<Identity, OFFSET>,
            SetStartStopPosition: SetStartStopPosition::<Identity, OFFSET>,
            HasVideo: HasVideo::<Identity, OFFSET>,
            HasAudio: HasAudio::<Identity, OFFSET>,
            IsProtected: IsProtected::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            GetNumberOfStreams: GetNumberOfStreams::<Identity, OFFSET>,
            GetStreamSelection: GetStreamSelection::<Identity, OFFSET>,
            SetStreamSelection: SetStreamSelection::<Identity, OFFSET>,
            GetStreamAttribute: GetStreamAttribute::<Identity, OFFSET>,
            GetPresentationAttribute: GetPresentationAttribute::<Identity, OFFSET>,
            GetCharacteristics: GetCharacteristics::<Identity, OFFSET>,
            SetStreamSink: SetStreamSink::<Identity, OFFSET>,
            GetMetadata: GetMetadata::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPMediaItem as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "propsys", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFPMediaItem {}
windows_core::imp::define_interface!(IMFPMediaPlayer, IMFPMediaPlayer_Vtbl, 0xa714590a_58af_430a_85bf_44f5ec838d85);
windows_core::imp::interface_hierarchy!(IMFPMediaPlayer, windows_core::IUnknown);
impl IMFPMediaPlayer {
    pub unsafe fn Play(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FrameStep(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FrameStep)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetPosition(&self, guidpositiontype: *const windows_core::GUID, pvpositionvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPosition)(windows_core::Interface::as_raw(self), guidpositiontype, pvpositionvalue) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPosition(&self, guidpositiontype: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), guidpositiontype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetDuration(&self, guidpositiontype: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self), guidpositiontype, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRate(&self, flrate: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRate)(windows_core::Interface::as_raw(self), flrate) }
    }
    pub unsafe fn GetRate(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSupportedRates(&self, fforwarddirection: bool, pflslowestrate: *mut f32, pflfastestrate: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedRates)(windows_core::Interface::as_raw(self), fforwarddirection.into(), pflslowestrate as _, pflfastestrate as _) }
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<MFP_MEDIAPLAYER_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateMediaItemFromURL<P0>(&self, pwszurl: P0, fsync: bool, dwuserdata: usize, ppmediaitem: Option<*mut Option<IMFPMediaItem>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateMediaItemFromURL)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), fsync.into(), dwuserdata, ppmediaitem.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn CreateMediaItemFromObject<P0>(&self, piunknownobj: P0, fsync: bool, dwuserdata: usize, ppmediaitem: Option<*mut Option<IMFPMediaItem>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateMediaItemFromObject)(windows_core::Interface::as_raw(self), piunknownobj.param().abi(), fsync.into(), dwuserdata, ppmediaitem.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetMediaItem<P0>(&self, pimfpmediaitem: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPMediaItem>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMediaItem)(windows_core::Interface::as_raw(self), pimfpmediaitem.param().abi()) }
    }
    pub unsafe fn ClearMediaItem(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearMediaItem)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMediaItem(&self) -> windows_core::Result<IMFPMediaItem> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetVolume(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetVolume(&self, flvolume: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), flvolume) }
    }
    pub unsafe fn GetBalance(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBalance)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBalance(&self, flbalance: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBalance)(windows_core::Interface::as_raw(self), flbalance) }
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMute(&self, fmute: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), fmute.into()) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetNativeVideoSize(&self, pszvideo: Option<*mut super::SIZE>, pszarvideo: Option<*mut super::SIZE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNativeVideoSize)(windows_core::Interface::as_raw(self), pszvideo.unwrap_or(core::mem::zeroed()) as _, pszarvideo.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetIdealVideoSize(&self, pszmin: Option<*mut super::SIZE>, pszmax: Option<*mut super::SIZE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIdealVideoSize)(windows_core::Interface::as_raw(self), pszmin.unwrap_or(core::mem::zeroed()) as _, pszmax.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn SetVideoSourceRect(&self, pnrcsource: *const super::MFVideoNormalizedRect) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVideoSourceRect)(windows_core::Interface::as_raw(self), pnrcsource) }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn GetVideoSourceRect(&self) -> windows_core::Result<super::MFVideoNormalizedRect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoSourceRect)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAspectRatioMode(&self, dwaspectratiomode: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAspectRatioMode)(windows_core::Interface::as_raw(self), dwaspectratiomode) }
    }
    pub unsafe fn GetAspectRatioMode(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAspectRatioMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetVideoWindow(&self) -> windows_core::Result<super::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UpdateVideo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateVideo)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetBorderColor(&self, clr: super::COLORREF) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBorderColor)(windows_core::Interface::as_raw(self), clr) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetBorderColor(&self) -> windows_core::Result<super::COLORREF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBorderColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertEffect<P0>(&self, peffect: P0, foptional: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEffect)(windows_core::Interface::as_raw(self), peffect.param().abi(), foptional.into()) }
    }
    pub unsafe fn RemoveEffect<P0>(&self, peffect: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveEffect)(windows_core::Interface::as_raw(self), peffect.param().abi()) }
    }
    pub unsafe fn RemoveAllEffects(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllEffects)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPMediaPlayer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FrameStep: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetPosition: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetPosition: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetDuration: usize,
    pub SetRate: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub GetSupportedRates: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut f32, *mut f32) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFP_MEDIAPLAYER_STATE) -> windows_core::HRESULT,
    pub CreateMediaItemFromURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMediaItemFromObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, usize, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMediaItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearMediaItem: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMediaItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetBalance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetBalance: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetNativeVideoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SIZE, *mut super::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetNativeVideoSize: usize,
    #[cfg(feature = "windef")]
    pub GetIdealVideoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SIZE, *mut super::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetIdealVideoSize: usize,
    #[cfg(feature = "mfidl")]
    pub SetVideoSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MFVideoNormalizedRect) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    SetVideoSourceRect: usize,
    #[cfg(feature = "mfidl")]
    pub GetVideoSourceRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MFVideoNormalizedRect) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    GetVideoSourceRect: usize,
    pub SetAspectRatioMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAspectRatioMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetVideoWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetVideoWindow: usize,
    pub UpdateVideo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetBorderColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetBorderColor: usize,
    #[cfg(feature = "windef")]
    pub GetBorderColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetBorderColor: usize,
    pub InsertEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub RemoveEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfidl", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFPMediaPlayer_Impl: windows_core::IUnknownImpl {
    fn Play(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn FrameStep(&self) -> windows_core::Result<()>;
    fn SetPosition(&self, guidpositiontype: *const windows_core::GUID, pvpositionvalue: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn GetPosition(&self, guidpositiontype: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT>;
    fn GetDuration(&self, guidpositiontype: *const windows_core::GUID) -> windows_core::Result<super::PROPVARIANT>;
    fn SetRate(&self, flrate: f32) -> windows_core::Result<()>;
    fn GetRate(&self) -> windows_core::Result<f32>;
    fn GetSupportedRates(&self, fforwarddirection: windows_core::BOOL, pflslowestrate: *mut f32, pflfastestrate: *mut f32) -> windows_core::Result<()>;
    fn GetState(&self) -> windows_core::Result<MFP_MEDIAPLAYER_STATE>;
    fn CreateMediaItemFromURL(&self, pwszurl: &windows_core::PCWSTR, fsync: windows_core::BOOL, dwuserdata: usize, ppmediaitem: windows_core::OutRef<IMFPMediaItem>) -> windows_core::Result<()>;
    fn CreateMediaItemFromObject(&self, piunknownobj: windows_core::Ref<windows_core::IUnknown>, fsync: windows_core::BOOL, dwuserdata: usize, ppmediaitem: windows_core::OutRef<IMFPMediaItem>) -> windows_core::Result<()>;
    fn SetMediaItem(&self, pimfpmediaitem: windows_core::Ref<IMFPMediaItem>) -> windows_core::Result<()>;
    fn ClearMediaItem(&self) -> windows_core::Result<()>;
    fn GetMediaItem(&self) -> windows_core::Result<IMFPMediaItem>;
    fn GetVolume(&self) -> windows_core::Result<f32>;
    fn SetVolume(&self, flvolume: f32) -> windows_core::Result<()>;
    fn GetBalance(&self) -> windows_core::Result<f32>;
    fn SetBalance(&self, flbalance: f32) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetMute(&self, fmute: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetNativeVideoSize(&self, pszvideo: *mut super::SIZE, pszarvideo: *mut super::SIZE) -> windows_core::Result<()>;
    fn GetIdealVideoSize(&self, pszmin: *mut super::SIZE, pszmax: *mut super::SIZE) -> windows_core::Result<()>;
    fn SetVideoSourceRect(&self, pnrcsource: *const super::MFVideoNormalizedRect) -> windows_core::Result<()>;
    fn GetVideoSourceRect(&self) -> windows_core::Result<super::MFVideoNormalizedRect>;
    fn SetAspectRatioMode(&self, dwaspectratiomode: u32) -> windows_core::Result<()>;
    fn GetAspectRatioMode(&self) -> windows_core::Result<u32>;
    fn GetVideoWindow(&self) -> windows_core::Result<super::HWND>;
    fn UpdateVideo(&self) -> windows_core::Result<()>;
    fn SetBorderColor(&self, clr: super::COLORREF) -> windows_core::Result<()>;
    fn GetBorderColor(&self) -> windows_core::Result<super::COLORREF>;
    fn InsertEffect(&self, peffect: windows_core::Ref<windows_core::IUnknown>, foptional: windows_core::BOOL) -> windows_core::Result<()>;
    fn RemoveEffect(&self, peffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveAllEffects(&self) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IMFPMediaPlayer_Vtbl {
    pub const fn new<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Play<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::Play(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn FrameStep<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::FrameStep(this).into()
            }
        }
        unsafe extern "system" fn SetPosition<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpositiontype: *const windows_core::GUID, pvpositionvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::SetPosition(this, core::mem::transmute_copy(&guidpositiontype), core::mem::transmute_copy(&pvpositionvalue)).into()
            }
        }
        unsafe extern "system" fn GetPosition<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpositiontype: *const windows_core::GUID, pvpositionvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetPosition(this, core::mem::transmute_copy(&guidpositiontype)) {
                    Ok(ok__) => {
                        pvpositionvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDuration<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidpositiontype: *const windows_core::GUID, pvdurationvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetDuration(this, core::mem::transmute_copy(&guidpositiontype)) {
                    Ok(ok__) => {
                        pvdurationvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRate<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flrate: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::SetRate(this, core::mem::transmute_copy(&flrate)).into()
            }
        }
        unsafe extern "system" fn GetRate<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflrate: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetRate(this) {
                    Ok(ok__) => {
                        pflrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedRates<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fforwarddirection: windows_core::BOOL, pflslowestrate: *mut f32, pflfastestrate: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::GetSupportedRates(this, core::mem::transmute_copy(&fforwarddirection), core::mem::transmute_copy(&pflslowestrate), core::mem::transmute_copy(&pflfastestrate)).into()
            }
        }
        unsafe extern "system" fn GetState<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pestate: *mut MFP_MEDIAPLAYER_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetState(this) {
                    Ok(ok__) => {
                        pestate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMediaItemFromURL<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, fsync: windows_core::BOOL, dwuserdata: usize, ppmediaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::CreateMediaItemFromURL(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&fsync), core::mem::transmute_copy(&dwuserdata), core::mem::transmute_copy(&ppmediaitem)).into()
            }
        }
        unsafe extern "system" fn CreateMediaItemFromObject<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piunknownobj: *mut core::ffi::c_void, fsync: windows_core::BOOL, dwuserdata: usize, ppmediaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::CreateMediaItemFromObject(this, core::mem::transmute_copy(&piunknownobj), core::mem::transmute_copy(&fsync), core::mem::transmute_copy(&dwuserdata), core::mem::transmute_copy(&ppmediaitem)).into()
            }
        }
        unsafe extern "system" fn SetMediaItem<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimfpmediaitem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::SetMediaItem(this, core::mem::transmute_copy(&pimfpmediaitem)).into()
            }
        }
        unsafe extern "system" fn ClearMediaItem<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::ClearMediaItem(this).into()
            }
        }
        unsafe extern "system" fn GetMediaItem<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppimfpmediaitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetMediaItem(this) {
                    Ok(ok__) => {
                        ppimfpmediaitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVolume<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflvolume: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetVolume(this) {
                    Ok(ok__) => {
                        pflvolume.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVolume<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flvolume: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::SetVolume(this, core::mem::transmute_copy(&flvolume)).into()
            }
        }
        unsafe extern "system" fn GetBalance<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflbalance: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetBalance(this) {
                    Ok(ok__) => {
                        pflbalance.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBalance<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flbalance: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::SetBalance(this, core::mem::transmute_copy(&flbalance)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfmute: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetMute(this) {
                    Ok(ok__) => {
                        pfmute.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMute<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fmute: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::SetMute(this, core::mem::transmute_copy(&fmute)).into()
            }
        }
        unsafe extern "system" fn GetNativeVideoSize<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvideo: *mut super::SIZE, pszarvideo: *mut super::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::GetNativeVideoSize(this, core::mem::transmute_copy(&pszvideo), core::mem::transmute_copy(&pszarvideo)).into()
            }
        }
        unsafe extern "system" fn GetIdealVideoSize<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmin: *mut super::SIZE, pszmax: *mut super::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::GetIdealVideoSize(this, core::mem::transmute_copy(&pszmin), core::mem::transmute_copy(&pszmax)).into()
            }
        }
        unsafe extern "system" fn SetVideoSourceRect<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnrcsource: *const super::MFVideoNormalizedRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::SetVideoSourceRect(this, core::mem::transmute_copy(&pnrcsource)).into()
            }
        }
        unsafe extern "system" fn GetVideoSourceRect<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnrcsource: *mut super::MFVideoNormalizedRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetVideoSourceRect(this) {
                    Ok(ok__) => {
                        pnrcsource.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAspectRatioMode<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspectratiomode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::SetAspectRatioMode(this, core::mem::transmute_copy(&dwaspectratiomode)).into()
            }
        }
        unsafe extern "system" fn GetAspectRatioMode<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwaspectratiomode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetAspectRatioMode(this) {
                    Ok(ok__) => {
                        pdwaspectratiomode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVideoWindow<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwndvideo: *mut super::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetVideoWindow(this) {
                    Ok(ok__) => {
                        phwndvideo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateVideo<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::UpdateVideo(this).into()
            }
        }
        unsafe extern "system" fn SetBorderColor<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clr: super::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::SetBorderColor(this, core::mem::transmute_copy(&clr)).into()
            }
        }
        unsafe extern "system" fn GetBorderColor<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclr: *mut super::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPMediaPlayer_Impl::GetBorderColor(this) {
                    Ok(ok__) => {
                        pclr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertEffect<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void, foptional: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::InsertEffect(this, core::mem::transmute_copy(&peffect), core::mem::transmute_copy(&foptional)).into()
            }
        }
        unsafe extern "system" fn RemoveEffect<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::RemoveEffect(this, core::mem::transmute_copy(&peffect)).into()
            }
        }
        unsafe extern "system" fn RemoveAllEffects<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::RemoveAllEffects(this).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFPMediaPlayer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayer_Impl::Shutdown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Play: Play::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            FrameStep: FrameStep::<Identity, OFFSET>,
            SetPosition: SetPosition::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            SetRate: SetRate::<Identity, OFFSET>,
            GetRate: GetRate::<Identity, OFFSET>,
            GetSupportedRates: GetSupportedRates::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            CreateMediaItemFromURL: CreateMediaItemFromURL::<Identity, OFFSET>,
            CreateMediaItemFromObject: CreateMediaItemFromObject::<Identity, OFFSET>,
            SetMediaItem: SetMediaItem::<Identity, OFFSET>,
            ClearMediaItem: ClearMediaItem::<Identity, OFFSET>,
            GetMediaItem: GetMediaItem::<Identity, OFFSET>,
            GetVolume: GetVolume::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
            GetBalance: GetBalance::<Identity, OFFSET>,
            SetBalance: SetBalance::<Identity, OFFSET>,
            GetMute: GetMute::<Identity, OFFSET>,
            SetMute: SetMute::<Identity, OFFSET>,
            GetNativeVideoSize: GetNativeVideoSize::<Identity, OFFSET>,
            GetIdealVideoSize: GetIdealVideoSize::<Identity, OFFSET>,
            SetVideoSourceRect: SetVideoSourceRect::<Identity, OFFSET>,
            GetVideoSourceRect: GetVideoSourceRect::<Identity, OFFSET>,
            SetAspectRatioMode: SetAspectRatioMode::<Identity, OFFSET>,
            GetAspectRatioMode: GetAspectRatioMode::<Identity, OFFSET>,
            GetVideoWindow: GetVideoWindow::<Identity, OFFSET>,
            UpdateVideo: UpdateVideo::<Identity, OFFSET>,
            SetBorderColor: SetBorderColor::<Identity, OFFSET>,
            GetBorderColor: GetBorderColor::<Identity, OFFSET>,
            InsertEffect: InsertEffect::<Identity, OFFSET>,
            RemoveEffect: RemoveEffect::<Identity, OFFSET>,
            RemoveAllEffects: RemoveAllEffects::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPMediaPlayer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFPMediaPlayer {}
windows_core::imp::define_interface!(IMFPMediaPlayerCallback, IMFPMediaPlayerCallback_Vtbl, 0x766c8ffb_5fdb_4fea_a28d_b912996f51bd);
windows_core::imp::interface_hierarchy!(IMFPMediaPlayerCallback, windows_core::IUnknown);
impl IMFPMediaPlayerCallback {
    #[cfg(feature = "propsys")]
    pub unsafe fn OnMediaPlayerEvent(&self, peventheader: *const MFP_EVENT_HEADER) {
        unsafe {
            (windows_core::Interface::vtable(self).OnMediaPlayerEvent)(windows_core::Interface::as_raw(self), peventheader);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPMediaPlayerCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "propsys")]
    pub OnMediaPlayerEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const MFP_EVENT_HEADER),
    #[cfg(not(feature = "propsys"))]
    OnMediaPlayerEvent: usize,
}
#[cfg(feature = "propsys")]
pub trait IMFPMediaPlayerCallback_Impl: windows_core::IUnknownImpl {
    fn OnMediaPlayerEvent(&self, peventheader: *const MFP_EVENT_HEADER);
}
#[cfg(feature = "propsys")]
impl IMFPMediaPlayerCallback_Vtbl {
    pub const fn new<Identity: IMFPMediaPlayerCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnMediaPlayerEvent<Identity: IMFPMediaPlayerCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peventheader: *const MFP_EVENT_HEADER) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMediaPlayerCallback_Impl::OnMediaPlayerEvent(this, core::mem::transmute_copy(&peventheader));
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnMediaPlayerEvent: OnMediaPlayerEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPMediaPlayerCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "propsys")]
impl windows_core::RuntimeName for IMFPMediaPlayerCallback {}
#[repr(C)]
#[cfg(all(feature = "mfidl", feature = "propsys"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_ACQUIRE_USER_CREDENTIAL_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub dwUserData: usize,
    pub fProceedWithAuthentication: windows_core::BOOL,
    pub hrAuthenticationStatus: windows_core::HRESULT,
    pub pwszURL: windows_core::PCWSTR,
    pub pwszSite: windows_core::PCWSTR,
    pub pwszRealm: windows_core::PCWSTR,
    pub pwszPackage: windows_core::PCWSTR,
    pub nRetries: i32,
    pub flags: MFP_CREDENTIAL_FLAGS,
    pub pCredential: core::mem::ManuallyDrop<Option<super::IMFNetCredential>>,
}
pub type MFP_CREATION_OPTIONS = u32;
pub const MFP_CREDENTIAL_CLEAR_TEXT: MFP_CREDENTIAL_FLAGS = 8;
pub const MFP_CREDENTIAL_DO_NOT_CACHE: MFP_CREDENTIAL_FLAGS = 4;
pub type MFP_CREDENTIAL_FLAGS = u32;
pub const MFP_CREDENTIAL_LOGGED_ON_USER: MFP_CREDENTIAL_FLAGS = 32;
pub const MFP_CREDENTIAL_PROMPT: MFP_CREDENTIAL_FLAGS = 1;
pub const MFP_CREDENTIAL_PROXY: MFP_CREDENTIAL_FLAGS = 16;
pub const MFP_CREDENTIAL_SAVE: MFP_CREDENTIAL_FLAGS = 2;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_ERROR_EVENT {
    pub header: MFP_EVENT_HEADER,
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_EVENT_HEADER {
    pub eEventType: MFP_EVENT_TYPE,
    pub hrEvent: windows_core::HRESULT,
    pub pMediaPlayer: core::mem::ManuallyDrop<Option<IMFPMediaPlayer>>,
    pub eState: MFP_MEDIAPLAYER_STATE,
    pub pPropertyStore: core::mem::ManuallyDrop<Option<super::IPropertyStore>>,
}
pub type MFP_EVENT_TYPE = i32;
pub const MFP_EVENT_TYPE_ACQUIRE_USER_CREDENTIAL: MFP_EVENT_TYPE = 12;
pub const MFP_EVENT_TYPE_ERROR: MFP_EVENT_TYPE = 10;
pub const MFP_EVENT_TYPE_FRAME_STEP: MFP_EVENT_TYPE = 7;
pub const MFP_EVENT_TYPE_MEDIAITEM_CLEARED: MFP_EVENT_TYPE = 8;
pub const MFP_EVENT_TYPE_MEDIAITEM_CREATED: MFP_EVENT_TYPE = 5;
pub const MFP_EVENT_TYPE_MEDIAITEM_SET: MFP_EVENT_TYPE = 6;
pub const MFP_EVENT_TYPE_MF: MFP_EVENT_TYPE = 9;
pub const MFP_EVENT_TYPE_PAUSE: MFP_EVENT_TYPE = 1;
pub const MFP_EVENT_TYPE_PLAY: MFP_EVENT_TYPE = 0;
pub const MFP_EVENT_TYPE_PLAYBACK_ENDED: MFP_EVENT_TYPE = 11;
pub const MFP_EVENT_TYPE_POSITION_SET: MFP_EVENT_TYPE = 3;
pub const MFP_EVENT_TYPE_RATE_SET: MFP_EVENT_TYPE = 4;
pub const MFP_EVENT_TYPE_STOP: MFP_EVENT_TYPE = 2;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_FRAME_STEP_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
}
pub const MFP_MEDIAITEM_CAN_PAUSE: MFP_MEDIAITEM_CHARACTERISTICS = 4;
pub const MFP_MEDIAITEM_CAN_SEEK: MFP_MEDIAITEM_CHARACTERISTICS = 2;
pub type MFP_MEDIAITEM_CHARACTERISTICS = u32;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_MEDIAITEM_CLEARED_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_MEDIAITEM_CREATED_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
    pub dwUserData: usize,
}
pub const MFP_MEDIAITEM_HAS_SLOW_SEEK: MFP_MEDIAITEM_CHARACTERISTICS = 8;
pub const MFP_MEDIAITEM_IS_LIVE: MFP_MEDIAITEM_CHARACTERISTICS = 1;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_MEDIAITEM_SET_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
}
pub type MFP_MEDIAPLAYER_STATE = i32;
pub const MFP_MEDIAPLAYER_STATE_EMPTY: MFP_MEDIAPLAYER_STATE = 0;
pub const MFP_MEDIAPLAYER_STATE_PAUSED: MFP_MEDIAPLAYER_STATE = 3;
pub const MFP_MEDIAPLAYER_STATE_PLAYING: MFP_MEDIAPLAYER_STATE = 2;
pub const MFP_MEDIAPLAYER_STATE_SHUTDOWN: MFP_MEDIAPLAYER_STATE = 4;
pub const MFP_MEDIAPLAYER_STATE_STOPPED: MFP_MEDIAPLAYER_STATE = 1;
#[repr(C)]
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_MF_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub MFEventType: super::MediaEventType,
    pub pMFMediaEvent: core::mem::ManuallyDrop<Option<super::IMFMediaEvent>>,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
}
pub const MFP_OPTION_FREE_THREADED_CALLBACK: MFP_CREATION_OPTIONS = 1;
pub const MFP_OPTION_NONE: MFP_CREATION_OPTIONS = 0;
pub const MFP_OPTION_NO_MMCSS: MFP_CREATION_OPTIONS = 2;
pub const MFP_OPTION_NO_REMOTE_DESKTOP_OPTIMIZATION: MFP_CREATION_OPTIONS = 4;
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_PAUSE_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_PLAYBACK_ENDED_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_PLAY_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_POSITION_SET_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MFP_RATE_SET_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
    pub flRate: f32,
}
#[repr(C)]
#[cfg(feature = "propsys")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MFP_STOP_EVENT {
    pub header: MFP_EVENT_HEADER,
    pub pMediaItem: core::mem::ManuallyDrop<Option<IMFPMediaItem>>,
}
