windows_core::imp::define_interface!(IAudioSourceProvider, IAudioSourceProvider_Vtbl, 0xebbaf249_afc2_4582_91c6_b60df2e84954);
windows_core::imp::interface_hierarchy!(IAudioSourceProvider, windows_core::IUnknown);
impl IAudioSourceProvider {
    pub unsafe fn ProvideInput(&self, dwsamplecount: u32, pdwchannelcount: *mut u32, pinterleavedaudiodata: Option<*mut f32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProvideInput)(windows_core::Interface::as_raw(self), dwsamplecount, pdwchannelcount as _, pinterleavedaudiodata.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioSourceProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProvideInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioSourceProvider_Impl: windows_core::IUnknownImpl {
    fn ProvideInput(&self, dwsamplecount: u32, pdwchannelcount: *mut u32, pinterleavedaudiodata: *mut f32) -> windows_core::Result<()>;
}
impl IAudioSourceProvider_Vtbl {
    pub const fn new<Identity: IAudioSourceProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProvideInput<Identity: IAudioSourceProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsamplecount: u32, pdwchannelcount: *mut u32, pinterleavedaudiodata: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioSourceProvider_Impl::ProvideInput(this, core::mem::transmute_copy(&dwsamplecount), core::mem::transmute_copy(&pdwchannelcount), core::mem::transmute_copy(&pinterleavedaudiodata)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ProvideInput: ProvideInput::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioSourceProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioSourceProvider {}
windows_core::imp::define_interface!(IMFBufferListNotify, IMFBufferListNotify_Vtbl, 0x24cd47f7_81d8_4785_adb2_af697a963cd2);
windows_core::imp::interface_hierarchy!(IMFBufferListNotify, windows_core::IUnknown);
impl IMFBufferListNotify {
    pub unsafe fn OnAddSourceBuffer(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnAddSourceBuffer)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnRemoveSourceBuffer(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnRemoveSourceBuffer)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFBufferListNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAddSourceBuffer: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnRemoveSourceBuffer: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMFBufferListNotify_Impl: windows_core::IUnknownImpl {
    fn OnAddSourceBuffer(&self);
    fn OnRemoveSourceBuffer(&self);
}
impl IMFBufferListNotify_Vtbl {
    pub const fn new<Identity: IMFBufferListNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAddSourceBuffer<Identity: IMFBufferListNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFBufferListNotify_Impl::OnAddSourceBuffer(this);
            }
        }
        unsafe extern "system" fn OnRemoveSourceBuffer<Identity: IMFBufferListNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFBufferListNotify_Impl::OnRemoveSourceBuffer(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAddSourceBuffer: OnAddSourceBuffer::<Identity, OFFSET>,
            OnRemoveSourceBuffer: OnRemoveSourceBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFBufferListNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFBufferListNotify {}
windows_core::imp::define_interface!(IMFCdmSuspendNotify, IMFCdmSuspendNotify_Vtbl, 0x7a5645d2_43bd_47fd_87b7_dcd24cc7d692);
windows_core::imp::interface_hierarchy!(IMFCdmSuspendNotify, windows_core::IUnknown);
impl IMFCdmSuspendNotify {
    pub unsafe fn Begin(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Begin)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn End(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).End)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCdmSuspendNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Begin: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub End: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFCdmSuspendNotify_Impl: windows_core::IUnknownImpl {
    fn Begin(&self) -> windows_core::Result<()>;
    fn End(&self) -> windows_core::Result<()>;
}
impl IMFCdmSuspendNotify_Vtbl {
    pub const fn new<Identity: IMFCdmSuspendNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Begin<Identity: IMFCdmSuspendNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCdmSuspendNotify_Impl::Begin(this).into()
            }
        }
        unsafe extern "system" fn End<Identity: IMFCdmSuspendNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCdmSuspendNotify_Impl::End(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Begin: Begin::<Identity, OFFSET>, End: End::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCdmSuspendNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFCdmSuspendNotify {}
windows_core::imp::define_interface!(IMFExtendedDRMTypeSupport, IMFExtendedDRMTypeSupport_Vtbl, 0x332ec562_3758_468d_a784_e38f23552128);
windows_core::imp::interface_hierarchy!(IMFExtendedDRMTypeSupport, windows_core::IUnknown);
impl IMFExtendedDRMTypeSupport {
    pub unsafe fn IsTypeSupportedEx(&self, r#type: &windows_core::BSTR, keysystem: &windows_core::BSTR) -> windows_core::Result<MF_MEDIA_ENGINE_CANPLAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTypeSupportedEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(r#type), core::mem::transmute_copy(keysystem), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFExtendedDRMTypeSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsTypeSupportedEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut MF_MEDIA_ENGINE_CANPLAY) -> windows_core::HRESULT,
}
pub trait IMFExtendedDRMTypeSupport_Impl: windows_core::IUnknownImpl {
    fn IsTypeSupportedEx(&self, r#type: &windows_core::BSTR, keysystem: &windows_core::BSTR) -> windows_core::Result<MF_MEDIA_ENGINE_CANPLAY>;
}
impl IMFExtendedDRMTypeSupport_Vtbl {
    pub const fn new<Identity: IMFExtendedDRMTypeSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsTypeSupportedEx<Identity: IMFExtendedDRMTypeSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut core::ffi::c_void, keysystem: *mut core::ffi::c_void, panswer: *mut MF_MEDIA_ENGINE_CANPLAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFExtendedDRMTypeSupport_Impl::IsTypeSupportedEx(this, core::mem::transmute(&r#type), core::mem::transmute(&keysystem)) {
                    Ok(ok__) => {
                        panswer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsTypeSupportedEx: IsTypeSupportedEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFExtendedDRMTypeSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFExtendedDRMTypeSupport {}
windows_core::imp::define_interface!(IMFHDCPStatus, IMFHDCPStatus_Vtbl, 0xde400f54_5bf1_40cf_8964_0bea136b1e3d);
windows_core::imp::interface_hierarchy!(IMFHDCPStatus, windows_core::IUnknown);
impl IMFHDCPStatus {
    pub unsafe fn Query(&self, pstatus: *mut MF_HDCP_STATUS, pfstatus: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Query)(windows_core::Interface::as_raw(self), pstatus as _, pfstatus as _) }
    }
    pub unsafe fn Set(&self, status: MF_HDCP_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Set)(windows_core::Interface::as_raw(self), status) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFHDCPStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Query: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_HDCP_STATUS, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Set: unsafe extern "system" fn(*mut core::ffi::c_void, MF_HDCP_STATUS) -> windows_core::HRESULT,
}
pub trait IMFHDCPStatus_Impl: windows_core::IUnknownImpl {
    fn Query(&self, pstatus: *mut MF_HDCP_STATUS, pfstatus: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn Set(&self, status: MF_HDCP_STATUS) -> windows_core::Result<()>;
}
impl IMFHDCPStatus_Vtbl {
    pub const fn new<Identity: IMFHDCPStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Query<Identity: IMFHDCPStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut MF_HDCP_STATUS, pfstatus: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHDCPStatus_Impl::Query(this, core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&pfstatus)).into()
            }
        }
        unsafe extern "system" fn Set<Identity: IMFHDCPStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, status: MF_HDCP_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHDCPStatus_Impl::Set(this, core::mem::transmute_copy(&status)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Query: Query::<Identity, OFFSET>, Set: Set::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFHDCPStatus as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFHDCPStatus {}
windows_core::imp::define_interface!(IMFMediaEngine, IMFMediaEngine_Vtbl, 0x98a1b0bb_03eb_4935_ae7c_93c1fa0e1c93);
windows_core::imp::interface_hierarchy!(IMFMediaEngine, windows_core::IUnknown);
impl IMFMediaEngine {
    pub unsafe fn GetError(&self) -> windows_core::Result<IMFMediaError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetErrorCode(&self, error: MF_MEDIA_ENGINE_ERR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetErrorCode)(windows_core::Interface::as_raw(self), error) }
    }
    pub unsafe fn SetSourceElements<P0>(&self, psrcelements: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFMediaEngineSrcElements>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSourceElements)(windows_core::Interface::as_raw(self), psrcelements.param().abi()) }
    }
    pub unsafe fn SetSource(&self, purl: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSource)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(purl)) }
    }
    pub unsafe fn GetCurrentSource(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetNetworkState(&self) -> u16 {
        unsafe { (windows_core::Interface::vtable(self).GetNetworkState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetPreload(&self) -> MF_MEDIA_ENGINE_PRELOAD {
        unsafe { (windows_core::Interface::vtable(self).GetPreload)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetPreload(&self, preload: MF_MEDIA_ENGINE_PRELOAD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPreload)(windows_core::Interface::as_raw(self), preload) }
    }
    pub unsafe fn GetBuffered(&self) -> windows_core::Result<IMFMediaTimeRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBuffered)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Load(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CanPlayType(&self, r#type: &windows_core::BSTR) -> windows_core::Result<MF_MEDIA_ENGINE_CANPLAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPlayType)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(r#type), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetReadyState(&self) -> u16 {
        unsafe { (windows_core::Interface::vtable(self).GetReadyState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsSeeking(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsSeeking)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCurrentTime(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentTime)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetCurrentTime(&self, seektime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentTime)(windows_core::Interface::as_raw(self), seektime) }
    }
    pub unsafe fn GetStartTime(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetStartTime)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDuration(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsPaused(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsPaused)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDefaultPlaybackRate(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetDefaultPlaybackRate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetDefaultPlaybackRate(&self, rate: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultPlaybackRate)(windows_core::Interface::as_raw(self), rate) }
    }
    pub unsafe fn GetPlaybackRate(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetPlaybackRate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetPlaybackRate(&self, rate: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPlaybackRate)(windows_core::Interface::as_raw(self), rate) }
    }
    pub unsafe fn GetPlayed(&self) -> windows_core::Result<IMFMediaTimeRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlayed)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSeekable(&self) -> windows_core::Result<IMFMediaTimeRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSeekable)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsEnded(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsEnded)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetAutoPlay(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetAutoPlay)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAutoPlay(&self, autoplay: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoPlay)(windows_core::Interface::as_raw(self), autoplay.into()) }
    }
    pub unsafe fn GetLoop(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetLoop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLoop(&self, r#loop: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLoop)(windows_core::Interface::as_raw(self), r#loop.into()) }
    }
    pub unsafe fn Play(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Play)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetMuted(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetMuted)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetMuted(&self, muted: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMuted)(windows_core::Interface::as_raw(self), muted.into()) }
    }
    pub unsafe fn GetVolume(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetVolume)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetVolume(&self, volume: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVolume)(windows_core::Interface::as_raw(self), volume) }
    }
    pub unsafe fn HasVideo(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasVideo)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn HasAudio(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).HasAudio)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetNativeVideoSize(&self, cx: Option<*mut u32>, cy: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNativeVideoSize)(windows_core::Interface::as_raw(self), cx.unwrap_or(core::mem::zeroed()) as _, cy.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetVideoAspectRatio(&self, cx: Option<*mut u32>, cy: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoAspectRatio)(windows_core::Interface::as_raw(self), cx.unwrap_or(core::mem::zeroed()) as _, cy.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
    pub unsafe fn TransferVideoFrame<P0>(&self, pdstsurf: P0, psrc: Option<*const super::mfidl::MFVideoNormalizedRect>, pdst: *const super::windef::RECT, pborderclr: Option<*const super::mfobjects::MFARGB>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).TransferVideoFrame)(windows_core::Interface::as_raw(self), pdstsurf.param().abi(), psrc.unwrap_or(core::mem::zeroed()) as _, pdst, pborderclr.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn OnVideoStreamTick(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnVideoStreamTick)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngine_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, MF_MEDIA_ENGINE_ERR) -> windows_core::HRESULT,
    pub SetSourceElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNetworkState: unsafe extern "system" fn(*mut core::ffi::c_void) -> u16,
    pub GetPreload: unsafe extern "system" fn(*mut core::ffi::c_void) -> MF_MEDIA_ENGINE_PRELOAD,
    pub SetPreload: unsafe extern "system" fn(*mut core::ffi::c_void, MF_MEDIA_ENGINE_PRELOAD) -> windows_core::HRESULT,
    pub GetBuffered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanPlayType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MF_MEDIA_ENGINE_CANPLAY) -> windows_core::HRESULT,
    pub GetReadyState: unsafe extern "system" fn(*mut core::ffi::c_void) -> u16,
    pub IsSeeking: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetCurrentTime: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetCurrentTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub IsPaused: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetDefaultPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetDefaultPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetPlaybackRate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetPlayed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSeekable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsEnded: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetAutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetAutoPlay: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetLoop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetLoop: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub Play: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMuted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub SetMuted: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetVolume: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub HasVideo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub HasAudio: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetNativeVideoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub GetVideoAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
    pub TransferVideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::mfidl::MFVideoNormalizedRect, *const super::windef::RECT, *const super::mfobjects::MFARGB) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "mfobjects", feature = "windef")))]
    TransferVideoFrame: usize,
    pub OnVideoStreamTick: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
pub trait IMFMediaEngine_Impl: windows_core::IUnknownImpl {
    fn GetError(&self) -> windows_core::Result<IMFMediaError>;
    fn SetErrorCode(&self, error: MF_MEDIA_ENGINE_ERR) -> windows_core::Result<()>;
    fn SetSourceElements(&self, psrcelements: windows_core::Ref<IMFMediaEngineSrcElements>) -> windows_core::Result<()>;
    fn SetSource(&self, purl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCurrentSource(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetNetworkState(&self) -> u16;
    fn GetPreload(&self) -> MF_MEDIA_ENGINE_PRELOAD;
    fn SetPreload(&self, preload: MF_MEDIA_ENGINE_PRELOAD) -> windows_core::Result<()>;
    fn GetBuffered(&self) -> windows_core::Result<IMFMediaTimeRange>;
    fn Load(&self) -> windows_core::Result<()>;
    fn CanPlayType(&self, r#type: &windows_core::BSTR) -> windows_core::Result<MF_MEDIA_ENGINE_CANPLAY>;
    fn GetReadyState(&self) -> u16;
    fn IsSeeking(&self) -> windows_core::BOOL;
    fn GetCurrentTime(&self) -> f64;
    fn SetCurrentTime(&self, seektime: f64) -> windows_core::Result<()>;
    fn GetStartTime(&self) -> f64;
    fn GetDuration(&self) -> f64;
    fn IsPaused(&self) -> windows_core::BOOL;
    fn GetDefaultPlaybackRate(&self) -> f64;
    fn SetDefaultPlaybackRate(&self, rate: f64) -> windows_core::Result<()>;
    fn GetPlaybackRate(&self) -> f64;
    fn SetPlaybackRate(&self, rate: f64) -> windows_core::Result<()>;
    fn GetPlayed(&self) -> windows_core::Result<IMFMediaTimeRange>;
    fn GetSeekable(&self) -> windows_core::Result<IMFMediaTimeRange>;
    fn IsEnded(&self) -> windows_core::BOOL;
    fn GetAutoPlay(&self) -> windows_core::BOOL;
    fn SetAutoPlay(&self, autoplay: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetLoop(&self) -> windows_core::BOOL;
    fn SetLoop(&self, r#loop: windows_core::BOOL) -> windows_core::Result<()>;
    fn Play(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn GetMuted(&self) -> windows_core::BOOL;
    fn SetMuted(&self, muted: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetVolume(&self) -> f64;
    fn SetVolume(&self, volume: f64) -> windows_core::Result<()>;
    fn HasVideo(&self) -> windows_core::BOOL;
    fn HasAudio(&self) -> windows_core::BOOL;
    fn GetNativeVideoSize(&self, cx: *mut u32, cy: *mut u32) -> windows_core::Result<()>;
    fn GetVideoAspectRatio(&self, cx: *mut u32, cy: *mut u32) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn TransferVideoFrame(&self, pdstsurf: windows_core::Ref<windows_core::IUnknown>, psrc: *const super::mfidl::MFVideoNormalizedRect, pdst: *const super::windef::RECT, pborderclr: *const super::mfobjects::MFARGB) -> windows_core::Result<()>;
    fn OnVideoStreamTick(&self) -> windows_core::Result<i64>;
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
impl IMFMediaEngine_Vtbl {
    pub const fn new<Identity: IMFMediaEngine_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetError<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetError(this) {
                    Ok(ok__) => {
                        pperror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetErrorCode<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: MF_MEDIA_ENGINE_ERR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetErrorCode(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn SetSourceElements<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcelements: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetSourceElements(this, core::mem::transmute_copy(&psrcelements)).into()
            }
        }
        unsafe extern "system" fn SetSource<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, purl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetSource(this, core::mem::transmute(&purl)).into()
            }
        }
        unsafe extern "system" fn GetCurrentSource<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppurl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetCurrentSource(this) {
                    Ok(ok__) => {
                        ppurl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNetworkState<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u16 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetNetworkState(this)
            }
        }
        unsafe extern "system" fn GetPreload<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MF_MEDIA_ENGINE_PRELOAD {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetPreload(this)
            }
        }
        unsafe extern "system" fn SetPreload<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preload: MF_MEDIA_ENGINE_PRELOAD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetPreload(this, core::mem::transmute_copy(&preload)).into()
            }
        }
        unsafe extern "system" fn GetBuffered<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffered: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetBuffered(this) {
                    Ok(ok__) => {
                        ppbuffered.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Load<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::Load(this).into()
            }
        }
        unsafe extern "system" fn CanPlayType<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut core::ffi::c_void, panswer: *mut MF_MEDIA_ENGINE_CANPLAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::CanPlayType(this, core::mem::transmute(&r#type)) {
                    Ok(ok__) => {
                        panswer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReadyState<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u16 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetReadyState(this)
            }
        }
        unsafe extern "system" fn IsSeeking<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::IsSeeking(this)
            }
        }
        unsafe extern "system" fn GetCurrentTime<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetCurrentTime(this)
            }
        }
        unsafe extern "system" fn SetCurrentTime<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seektime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetCurrentTime(this, core::mem::transmute_copy(&seektime)).into()
            }
        }
        unsafe extern "system" fn GetStartTime<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetStartTime(this)
            }
        }
        unsafe extern "system" fn GetDuration<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetDuration(this)
            }
        }
        unsafe extern "system" fn IsPaused<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::IsPaused(this)
            }
        }
        unsafe extern "system" fn GetDefaultPlaybackRate<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetDefaultPlaybackRate(this)
            }
        }
        unsafe extern "system" fn SetDefaultPlaybackRate<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rate: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetDefaultPlaybackRate(this, core::mem::transmute_copy(&rate)).into()
            }
        }
        unsafe extern "system" fn GetPlaybackRate<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetPlaybackRate(this)
            }
        }
        unsafe extern "system" fn SetPlaybackRate<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rate: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetPlaybackRate(this, core::mem::transmute_copy(&rate)).into()
            }
        }
        unsafe extern "system" fn GetPlayed<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppplayed: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetPlayed(this) {
                    Ok(ok__) => {
                        ppplayed.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSeekable<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppseekable: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::GetSeekable(this) {
                    Ok(ok__) => {
                        ppseekable.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsEnded<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::IsEnded(this)
            }
        }
        unsafe extern "system" fn GetAutoPlay<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetAutoPlay(this)
            }
        }
        unsafe extern "system" fn SetAutoPlay<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autoplay: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetAutoPlay(this, core::mem::transmute_copy(&autoplay)).into()
            }
        }
        unsafe extern "system" fn GetLoop<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetLoop(this)
            }
        }
        unsafe extern "system" fn SetLoop<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#loop: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetLoop(this, core::mem::transmute_copy(&r#loop)).into()
            }
        }
        unsafe extern "system" fn Play<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::Play(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn GetMuted<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetMuted(this)
            }
        }
        unsafe extern "system" fn SetMuted<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, muted: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetMuted(this, core::mem::transmute_copy(&muted)).into()
            }
        }
        unsafe extern "system" fn GetVolume<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetVolume(this)
            }
        }
        unsafe extern "system" fn SetVolume<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, volume: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::SetVolume(this, core::mem::transmute_copy(&volume)).into()
            }
        }
        unsafe extern "system" fn HasVideo<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::HasVideo(this)
            }
        }
        unsafe extern "system" fn HasAudio<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::HasAudio(this)
            }
        }
        unsafe extern "system" fn GetNativeVideoSize<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: *mut u32, cy: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetNativeVideoSize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
            }
        }
        unsafe extern "system" fn GetVideoAspectRatio<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: *mut u32, cy: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::GetVideoAspectRatio(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::Shutdown(this).into()
            }
        }
        unsafe extern "system" fn TransferVideoFrame<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstsurf: *mut core::ffi::c_void, psrc: *const super::mfidl::MFVideoNormalizedRect, pdst: *const super::windef::RECT, pborderclr: *const super::mfobjects::MFARGB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngine_Impl::TransferVideoFrame(this, core::mem::transmute_copy(&pdstsurf), core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&pborderclr)).into()
            }
        }
        unsafe extern "system" fn OnVideoStreamTick<Identity: IMFMediaEngine_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppts: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngine_Impl::OnVideoStreamTick(this) {
                    Ok(ok__) => {
                        ppts.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetError: GetError::<Identity, OFFSET>,
            SetErrorCode: SetErrorCode::<Identity, OFFSET>,
            SetSourceElements: SetSourceElements::<Identity, OFFSET>,
            SetSource: SetSource::<Identity, OFFSET>,
            GetCurrentSource: GetCurrentSource::<Identity, OFFSET>,
            GetNetworkState: GetNetworkState::<Identity, OFFSET>,
            GetPreload: GetPreload::<Identity, OFFSET>,
            SetPreload: SetPreload::<Identity, OFFSET>,
            GetBuffered: GetBuffered::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            CanPlayType: CanPlayType::<Identity, OFFSET>,
            GetReadyState: GetReadyState::<Identity, OFFSET>,
            IsSeeking: IsSeeking::<Identity, OFFSET>,
            GetCurrentTime: GetCurrentTime::<Identity, OFFSET>,
            SetCurrentTime: SetCurrentTime::<Identity, OFFSET>,
            GetStartTime: GetStartTime::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            IsPaused: IsPaused::<Identity, OFFSET>,
            GetDefaultPlaybackRate: GetDefaultPlaybackRate::<Identity, OFFSET>,
            SetDefaultPlaybackRate: SetDefaultPlaybackRate::<Identity, OFFSET>,
            GetPlaybackRate: GetPlaybackRate::<Identity, OFFSET>,
            SetPlaybackRate: SetPlaybackRate::<Identity, OFFSET>,
            GetPlayed: GetPlayed::<Identity, OFFSET>,
            GetSeekable: GetSeekable::<Identity, OFFSET>,
            IsEnded: IsEnded::<Identity, OFFSET>,
            GetAutoPlay: GetAutoPlay::<Identity, OFFSET>,
            SetAutoPlay: SetAutoPlay::<Identity, OFFSET>,
            GetLoop: GetLoop::<Identity, OFFSET>,
            SetLoop: SetLoop::<Identity, OFFSET>,
            Play: Play::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            GetMuted: GetMuted::<Identity, OFFSET>,
            SetMuted: SetMuted::<Identity, OFFSET>,
            GetVolume: GetVolume::<Identity, OFFSET>,
            SetVolume: SetVolume::<Identity, OFFSET>,
            HasVideo: HasVideo::<Identity, OFFSET>,
            HasAudio: HasAudio::<Identity, OFFSET>,
            GetNativeVideoSize: GetNativeVideoSize::<Identity, OFFSET>,
            GetVideoAspectRatio: GetVideoAspectRatio::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            TransferVideoFrame: TransferVideoFrame::<Identity, OFFSET>,
            OnVideoStreamTick: OnVideoStreamTick::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngine as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
impl windows_core::RuntimeName for IMFMediaEngine {}
windows_core::imp::define_interface!(IMFMediaEngineAudioEndpointId, IMFMediaEngineAudioEndpointId_Vtbl, 0x7a3bac98_0e76_49fb_8c20_8a86fd98eaf2);
windows_core::imp::interface_hierarchy!(IMFMediaEngineAudioEndpointId, windows_core::IUnknown);
impl IMFMediaEngineAudioEndpointId {
    pub unsafe fn SetAudioEndpointId<P0>(&self, pszendpointid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAudioEndpointId)(windows_core::Interface::as_raw(self), pszendpointid.param().abi()) }
    }
    pub unsafe fn GetAudioEndpointId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioEndpointId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineAudioEndpointId_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetAudioEndpointId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetAudioEndpointId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineAudioEndpointId_Impl: windows_core::IUnknownImpl {
    fn SetAudioEndpointId(&self, pszendpointid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAudioEndpointId(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IMFMediaEngineAudioEndpointId_Vtbl {
    pub const fn new<Identity: IMFMediaEngineAudioEndpointId_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAudioEndpointId<Identity: IMFMediaEngineAudioEndpointId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszendpointid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineAudioEndpointId_Impl::SetAudioEndpointId(this, core::mem::transmute(&pszendpointid)).into()
            }
        }
        unsafe extern "system" fn GetAudioEndpointId<Identity: IMFMediaEngineAudioEndpointId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszendpointid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineAudioEndpointId_Impl::GetAudioEndpointId(this) {
                    Ok(ok__) => {
                        ppszendpointid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAudioEndpointId: SetAudioEndpointId::<Identity, OFFSET>,
            GetAudioEndpointId: GetAudioEndpointId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineAudioEndpointId as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineAudioEndpointId {}
windows_core::imp::define_interface!(IMFMediaEngineClassFactory, IMFMediaEngineClassFactory_Vtbl, 0x4d645ace_26aa_4688_9be1_df3516990b93);
windows_core::imp::interface_hierarchy!(IMFMediaEngineClassFactory, windows_core::IUnknown);
impl IMFMediaEngineClassFactory {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn CreateInstance<P1>(&self, dwflags: u32, pattr: P1) -> windows_core::Result<IMFMediaEngine>
    where
        P1: windows_core::Param<super::mfobjects::IMFAttributes>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), dwflags, pattr.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTimeRange(&self) -> windows_core::Result<IMFMediaTimeRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTimeRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateError(&self) -> windows_core::Result<IMFMediaError> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateError)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineClassFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    CreateInstance: usize,
    pub CreateTimeRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFMediaEngineClassFactory_Impl: windows_core::IUnknownImpl {
    fn CreateInstance(&self, dwflags: u32, pattr: windows_core::Ref<super::mfobjects::IMFAttributes>) -> windows_core::Result<IMFMediaEngine>;
    fn CreateTimeRange(&self) -> windows_core::Result<IMFMediaTimeRange>;
    fn CreateError(&self) -> windows_core::Result<IMFMediaError>;
}
#[cfg(feature = "mfobjects")]
impl IMFMediaEngineClassFactory_Vtbl {
    pub const fn new<Identity: IMFMediaEngineClassFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateInstance<Identity: IMFMediaEngineClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pattr: *mut core::ffi::c_void, ppplayer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactory_Impl::CreateInstance(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pattr)) {
                    Ok(ok__) => {
                        ppplayer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTimeRange<Identity: IMFMediaEngineClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptimerange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactory_Impl::CreateTimeRange(this) {
                    Ok(ok__) => {
                        pptimerange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateError<Identity: IMFMediaEngineClassFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pperror: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactory_Impl::CreateError(this) {
                    Ok(ok__) => {
                        pperror.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            CreateTimeRange: CreateTimeRange::<Identity, OFFSET>,
            CreateError: CreateError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFMediaEngineClassFactory {}
windows_core::imp::define_interface!(IMFMediaEngineClassFactory2, IMFMediaEngineClassFactory2_Vtbl, 0x09083cef_867f_4bf6_8776_dee3a7b42fca);
windows_core::imp::interface_hierarchy!(IMFMediaEngineClassFactory2, windows_core::IUnknown);
impl IMFMediaEngineClassFactory2 {
    pub unsafe fn CreateMediaKeys2(&self, keysystem: &windows_core::BSTR, defaultcdmstorepath: &windows_core::BSTR, inprivatecdmstorepath: &windows_core::BSTR) -> windows_core::Result<IMFMediaKeys> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMediaKeys2)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(keysystem), core::mem::transmute_copy(defaultcdmstorepath), core::mem::transmute_copy(inprivatecdmstorepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineClassFactory2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateMediaKeys2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineClassFactory2_Impl: windows_core::IUnknownImpl {
    fn CreateMediaKeys2(&self, keysystem: &windows_core::BSTR, defaultcdmstorepath: &windows_core::BSTR, inprivatecdmstorepath: &windows_core::BSTR) -> windows_core::Result<IMFMediaKeys>;
}
impl IMFMediaEngineClassFactory2_Vtbl {
    pub const fn new<Identity: IMFMediaEngineClassFactory2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMediaKeys2<Identity: IMFMediaEngineClassFactory2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keysystem: *mut core::ffi::c_void, defaultcdmstorepath: *mut core::ffi::c_void, inprivatecdmstorepath: *mut core::ffi::c_void, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactory2_Impl::CreateMediaKeys2(this, core::mem::transmute(&keysystem), core::mem::transmute(&defaultcdmstorepath), core::mem::transmute(&inprivatecdmstorepath)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateMediaKeys2: CreateMediaKeys2::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactory2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineClassFactory2 {}
windows_core::imp::define_interface!(IMFMediaEngineClassFactory3, IMFMediaEngineClassFactory3_Vtbl, 0x3787614f_65f7_4003_b673_ead8293a0e60);
windows_core::imp::interface_hierarchy!(IMFMediaEngineClassFactory3, windows_core::IUnknown);
impl IMFMediaEngineClassFactory3 {
    #[cfg(feature = "propsys")]
    pub unsafe fn CreateMediaKeySystemAccess(&self, keysystem: &windows_core::BSTR, ppsupportedconfigurationsarray: *mut Option<super::propsys::IPropertyStore>, usize: u32, ppkeyaccess: *mut Option<IMFMediaKeySystemAccess>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CreateMediaKeySystemAccess)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(keysystem), core::mem::transmute(ppsupportedconfigurationsarray), usize, core::mem::transmute(ppkeyaccess)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineClassFactory3_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "propsys")]
    pub CreateMediaKeySystemAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    CreateMediaKeySystemAccess: usize,
}
#[cfg(feature = "propsys")]
pub trait IMFMediaEngineClassFactory3_Impl: windows_core::IUnknownImpl {
    fn CreateMediaKeySystemAccess(&self, keysystem: &windows_core::BSTR, ppsupportedconfigurationsarray: windows_core::OutRef<super::propsys::IPropertyStore>, usize: u32, ppkeyaccess: windows_core::OutRef<IMFMediaKeySystemAccess>) -> windows_core::Result<()>;
}
#[cfg(feature = "propsys")]
impl IMFMediaEngineClassFactory3_Vtbl {
    pub const fn new<Identity: IMFMediaEngineClassFactory3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMediaKeySystemAccess<Identity: IMFMediaEngineClassFactory3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keysystem: *mut core::ffi::c_void, ppsupportedconfigurationsarray: *mut *mut core::ffi::c_void, usize: u32, ppkeyaccess: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineClassFactory3_Impl::CreateMediaKeySystemAccess(this, core::mem::transmute(&keysystem), core::mem::transmute_copy(&ppsupportedconfigurationsarray), core::mem::transmute_copy(&usize), core::mem::transmute_copy(&ppkeyaccess)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateMediaKeySystemAccess: CreateMediaKeySystemAccess::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactory3 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "propsys")]
impl windows_core::RuntimeName for IMFMediaEngineClassFactory3 {}
windows_core::imp::define_interface!(IMFMediaEngineClassFactory4, IMFMediaEngineClassFactory4_Vtbl, 0xfbe256c1_43cf_4a9b_8cb8_ce8632a34186);
windows_core::imp::interface_hierarchy!(IMFMediaEngineClassFactory4, windows_core::IUnknown);
impl IMFMediaEngineClassFactory4 {
    pub unsafe fn CreateContentDecryptionModuleFactory<P0, T>(&self, keysystem: P0) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateContentDecryptionModuleFactory)(windows_core::Interface::as_raw(self), keysystem.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineClassFactory4_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateContentDecryptionModuleFactory: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineClassFactory4_Impl: windows_core::IUnknownImpl {
    fn CreateContentDecryptionModuleFactory(&self, keysystem: &windows_core::PCWSTR, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFMediaEngineClassFactory4_Vtbl {
    pub const fn new<Identity: IMFMediaEngineClassFactory4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateContentDecryptionModuleFactory<Identity: IMFMediaEngineClassFactory4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keysystem: windows_core::PCWSTR, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineClassFactory4_Impl::CreateContentDecryptionModuleFactory(this, core::mem::transmute(&keysystem), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateContentDecryptionModuleFactory: CreateContentDecryptionModuleFactory::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactory4 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineClassFactory4 {}
windows_core::imp::define_interface!(IMFMediaEngineClassFactoryEx, IMFMediaEngineClassFactoryEx_Vtbl, 0xc56156c6_ea5b_48a5_9df8_fbe035d0929e);
impl core::ops::Deref for IMFMediaEngineClassFactoryEx {
    type Target = IMFMediaEngineClassFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaEngineClassFactoryEx, windows_core::IUnknown, IMFMediaEngineClassFactory);
impl IMFMediaEngineClassFactoryEx {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn CreateMediaSourceExtension<P1>(&self, dwflags: u32, pattr: P1) -> windows_core::Result<IMFMediaSourceExtension>
    where
        P1: windows_core::Param<super::mfobjects::IMFAttributes>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMediaSourceExtension)(windows_core::Interface::as_raw(self), dwflags, pattr.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateMediaKeys(&self, keysystem: &windows_core::BSTR, cdmstorepath: &windows_core::BSTR) -> windows_core::Result<IMFMediaKeys> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMediaKeys)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(keysystem), core::mem::transmute_copy(cdmstorepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsTypeSupported(&self, r#type: &windows_core::BSTR, keysystem: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTypeSupported)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(r#type), core::mem::transmute_copy(keysystem), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineClassFactoryEx_Vtbl {
    pub base__: IMFMediaEngineClassFactory_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub CreateMediaSourceExtension: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    CreateMediaSourceExtension: usize,
    pub CreateMediaKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsTypeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFMediaEngineClassFactoryEx_Impl: IMFMediaEngineClassFactory_Impl {
    fn CreateMediaSourceExtension(&self, dwflags: u32, pattr: windows_core::Ref<super::mfobjects::IMFAttributes>) -> windows_core::Result<IMFMediaSourceExtension>;
    fn CreateMediaKeys(&self, keysystem: &windows_core::BSTR, cdmstorepath: &windows_core::BSTR) -> windows_core::Result<IMFMediaKeys>;
    fn IsTypeSupported(&self, r#type: &windows_core::BSTR, keysystem: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "mfobjects")]
impl IMFMediaEngineClassFactoryEx_Vtbl {
    pub const fn new<Identity: IMFMediaEngineClassFactoryEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMediaSourceExtension<Identity: IMFMediaEngineClassFactoryEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, pattr: *mut core::ffi::c_void, ppmse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactoryEx_Impl::CreateMediaSourceExtension(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pattr)) {
                    Ok(ok__) => {
                        ppmse.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMediaKeys<Identity: IMFMediaEngineClassFactoryEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keysystem: *mut core::ffi::c_void, cdmstorepath: *mut core::ffi::c_void, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactoryEx_Impl::CreateMediaKeys(this, core::mem::transmute(&keysystem), core::mem::transmute(&cdmstorepath)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsTypeSupported<Identity: IMFMediaEngineClassFactoryEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut core::ffi::c_void, keysystem: *mut core::ffi::c_void, issupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineClassFactoryEx_Impl::IsTypeSupported(this, core::mem::transmute(&r#type), core::mem::transmute(&keysystem)) {
                    Ok(ok__) => {
                        issupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMFMediaEngineClassFactory_Vtbl::new::<Identity, OFFSET>(),
            CreateMediaSourceExtension: CreateMediaSourceExtension::<Identity, OFFSET>,
            CreateMediaKeys: CreateMediaKeys::<Identity, OFFSET>,
            IsTypeSupported: IsTypeSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineClassFactoryEx as windows_core::Interface>::IID || iid == &<IMFMediaEngineClassFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFMediaEngineClassFactoryEx {}
windows_core::imp::define_interface!(IMFMediaEngineEME, IMFMediaEngineEME_Vtbl, 0x50dc93e4_ba4f_4275_ae66_83e836e57469);
windows_core::imp::interface_hierarchy!(IMFMediaEngineEME, windows_core::IUnknown);
impl IMFMediaEngineEME {
    pub unsafe fn get_Keys(&self) -> windows_core::Result<IMFMediaKeys> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Keys)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMediaKeys<P0>(&self, keys: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFMediaKeys>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMediaKeys)(windows_core::Interface::as_raw(self), keys.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineEME_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Keys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMediaKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineEME_Impl: windows_core::IUnknownImpl {
    fn get_Keys(&self) -> windows_core::Result<IMFMediaKeys>;
    fn SetMediaKeys(&self, keys: windows_core::Ref<IMFMediaKeys>) -> windows_core::Result<()>;
}
impl IMFMediaEngineEME_Vtbl {
    pub const fn new<Identity: IMFMediaEngineEME_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Keys<Identity: IMFMediaEngineEME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEME_Impl::get_Keys(this) {
                    Ok(ok__) => {
                        keys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMediaKeys<Identity: IMFMediaEngineEME_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keys: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEME_Impl::SetMediaKeys(this, core::mem::transmute_copy(&keys)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Keys: get_Keys::<Identity, OFFSET>,
            SetMediaKeys: SetMediaKeys::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineEME as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineEME {}
windows_core::imp::define_interface!(IMFMediaEngineEMENotify, IMFMediaEngineEMENotify_Vtbl, 0x9e184d15_cdb7_4f86_b49e_566689f4a601);
windows_core::imp::interface_hierarchy!(IMFMediaEngineEMENotify, windows_core::IUnknown);
impl IMFMediaEngineEMENotify {
    pub unsafe fn Encrypted(&self, pbinitdata: Option<&[u8]>, bstrinitdatatype: &windows_core::BSTR) {
        unsafe {
            (windows_core::Interface::vtable(self).Encrypted)(windows_core::Interface::as_raw(self), pbinitdata.map_or(core::ptr::null(), |slice| slice.as_ptr()), pbinitdata.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute_copy(bstrinitdatatype));
        }
    }
    pub unsafe fn WaitingForKey(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).WaitingForKey)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineEMENotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Encrypted: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut core::ffi::c_void),
    pub WaitingForKey: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMFMediaEngineEMENotify_Impl: windows_core::IUnknownImpl {
    fn Encrypted(&self, pbinitdata: *const u8, cb: u32, bstrinitdatatype: &windows_core::BSTR);
    fn WaitingForKey(&self);
}
impl IMFMediaEngineEMENotify_Vtbl {
    pub const fn new<Identity: IMFMediaEngineEMENotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Encrypted<Identity: IMFMediaEngineEMENotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbinitdata: *const u8, cb: u32, bstrinitdatatype: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEMENotify_Impl::Encrypted(this, core::mem::transmute_copy(&pbinitdata), core::mem::transmute_copy(&cb), core::mem::transmute(&bstrinitdatatype));
            }
        }
        unsafe extern "system" fn WaitingForKey<Identity: IMFMediaEngineEMENotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEMENotify_Impl::WaitingForKey(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Encrypted: Encrypted::<Identity, OFFSET>,
            WaitingForKey: WaitingForKey::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineEMENotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineEMENotify {}
windows_core::imp::define_interface!(IMFMediaEngineEx, IMFMediaEngineEx_Vtbl, 0x83015ead_b1e6_40d0_a98a_37145ffe1ad1);
impl core::ops::Deref for IMFMediaEngineEx {
    type Target = IMFMediaEngine;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaEngineEx, windows_core::IUnknown, IMFMediaEngine);
impl IMFMediaEngineEx {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetSourceFromByteStream<P0>(&self, pbytestream: P0, purl: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::mfobjects::IMFByteStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSourceFromByteStream)(windows_core::Interface::as_raw(self), pbytestream.param().abi(), core::mem::transmute_copy(purl)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetStatistics(&self, statisticid: MF_MEDIA_ENGINE_STATISTIC) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStatistics)(windows_core::Interface::as_raw(self), statisticid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
    pub unsafe fn UpdateVideoStream(&self, psrc: Option<*const super::mfidl::MFVideoNormalizedRect>, pdst: Option<*const super::windef::RECT>, pborderclr: Option<*const super::mfobjects::MFARGB>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateVideoStream)(windows_core::Interface::as_raw(self), psrc.unwrap_or(core::mem::zeroed()) as _, pdst.unwrap_or(core::mem::zeroed()) as _, pborderclr.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetBalance(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetBalance)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetBalance(&self, balance: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBalance)(windows_core::Interface::as_raw(self), balance) }
    }
    pub unsafe fn IsPlaybackRateSupported(&self, rate: f64) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsPlaybackRateSupported)(windows_core::Interface::as_raw(self), rate) }
    }
    pub unsafe fn FrameStep(&self, forward: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FrameStep)(windows_core::Interface::as_raw(self), forward.into()) }
    }
    pub unsafe fn GetResourceCharacteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetResourceCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPresentationAttribute(&self, guidmfattribute: *const windows_core::GUID) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPresentationAttribute)(windows_core::Interface::as_raw(self), guidmfattribute, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetNumberOfStreams(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfStreams)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetStreamAttribute(&self, dwstreamindex: u32, guidmfattribute: *const windows_core::GUID) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamAttribute)(windows_core::Interface::as_raw(self), dwstreamindex, guidmfattribute, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetStreamSelection(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStreamSelection(&self, dwstreamindex: u32, enabled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamSelection)(windows_core::Interface::as_raw(self), dwstreamindex, enabled.into()) }
    }
    pub unsafe fn ApplyStreamSelections(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ApplyStreamSelections)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsProtected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsProtected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn InsertVideoEffect<P0>(&self, peffect: P0, foptional: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertVideoEffect)(windows_core::Interface::as_raw(self), peffect.param().abi(), foptional.into()) }
    }
    pub unsafe fn InsertAudioEffect<P0>(&self, peffect: P0, foptional: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertAudioEffect)(windows_core::Interface::as_raw(self), peffect.param().abi(), foptional.into()) }
    }
    pub unsafe fn RemoveAllEffects(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllEffects)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetTimelineMarkerTimer(&self, timetofire: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTimelineMarkerTimer)(windows_core::Interface::as_raw(self), timetofire) }
    }
    pub unsafe fn GetTimelineMarkerTimer(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimelineMarkerTimer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CancelTimelineMarkerTimer(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelTimelineMarkerTimer)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsStereo3D(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsStereo3D)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStereo3DFramePackingMode(&self) -> windows_core::Result<MF_MEDIA_ENGINE_S3D_PACKING_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStereo3DFramePackingMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStereo3DFramePackingMode(&self, packmode: MF_MEDIA_ENGINE_S3D_PACKING_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStereo3DFramePackingMode)(windows_core::Interface::as_raw(self), packmode) }
    }
    #[cfg(feature = "mftransform")]
    pub unsafe fn GetStereo3DRenderMode(&self) -> windows_core::Result<super::mftransform::MF3DVideoOutputType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStereo3DRenderMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mftransform")]
    pub unsafe fn SetStereo3DRenderMode(&self, outputtype: super::mftransform::MF3DVideoOutputType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStereo3DRenderMode)(windows_core::Interface::as_raw(self), outputtype) }
    }
    pub unsafe fn EnableWindowlessSwapchainMode(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableWindowlessSwapchainMode)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn GetVideoSwapchainHandle(&self) -> windows_core::Result<super::winnt::HANDLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoSwapchainHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EnableHorizontalMirrorMode(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableHorizontalMirrorMode)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
    pub unsafe fn GetAudioStreamCategory(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioStreamCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAudioStreamCategory(&self, category: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAudioStreamCategory)(windows_core::Interface::as_raw(self), category) }
    }
    pub unsafe fn GetAudioEndpointRole(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioEndpointRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAudioEndpointRole(&self, role: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAudioEndpointRole)(windows_core::Interface::as_raw(self), role) }
    }
    pub unsafe fn GetRealTimeMode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRealTimeMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRealTimeMode(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRealTimeMode)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
    pub unsafe fn SetCurrentTimeEx(&self, seektime: f64, seekmode: MF_MEDIA_ENGINE_SEEK_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentTimeEx)(windows_core::Interface::as_raw(self), seektime, seekmode) }
    }
    pub unsafe fn EnableTimeUpdateTimer(&self, fenabletimer: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableTimeUpdateTimer)(windows_core::Interface::as_raw(self), fenabletimer.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineEx_Vtbl {
    pub base__: IMFMediaEngine_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetSourceFromByteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetSourceFromByteStream: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetStatistics: unsafe extern "system" fn(*mut core::ffi::c_void, MF_MEDIA_ENGINE_STATISTIC, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetStatistics: usize,
    #[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
    pub UpdateVideoStream: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::mfidl::MFVideoNormalizedRect, *const super::windef::RECT, *const super::mfobjects::MFARGB) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "mfobjects", feature = "windef")))]
    UpdateVideoStream: usize,
    pub GetBalance: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetBalance: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub IsPlaybackRateSupported: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::BOOL,
    pub FrameStep: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetResourceCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPresentationAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetPresentationAttribute: usize,
    pub GetNumberOfStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetStreamAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetStreamAttribute: usize,
    pub GetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetStreamSelection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub ApplyStreamSelections: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsProtected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub InsertVideoEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub InsertAudioEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub RemoveAllEffects: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTimelineMarkerTimer: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetTimelineMarkerTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CancelTimelineMarkerTimer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsStereo3D: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetStereo3DFramePackingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_MEDIA_ENGINE_S3D_PACKING_MODE) -> windows_core::HRESULT,
    pub SetStereo3DFramePackingMode: unsafe extern "system" fn(*mut core::ffi::c_void, MF_MEDIA_ENGINE_S3D_PACKING_MODE) -> windows_core::HRESULT,
    #[cfg(feature = "mftransform")]
    pub GetStereo3DRenderMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mftransform::MF3DVideoOutputType) -> windows_core::HRESULT,
    #[cfg(not(feature = "mftransform"))]
    GetStereo3DRenderMode: usize,
    #[cfg(feature = "mftransform")]
    pub SetStereo3DRenderMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::mftransform::MF3DVideoOutputType) -> windows_core::HRESULT,
    #[cfg(not(feature = "mftransform"))]
    SetStereo3DRenderMode: usize,
    pub EnableWindowlessSwapchainMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub GetVideoSwapchainHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    GetVideoSwapchainHandle: usize,
    pub EnableHorizontalMirrorMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetAudioStreamCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAudioStreamCategory: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAudioEndpointRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetAudioEndpointRole: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetRealTimeMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetRealTimeMode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCurrentTimeEx: unsafe extern "system" fn(*mut core::ffi::c_void, f64, MF_MEDIA_ENGINE_SEEK_MODE) -> windows_core::HRESULT,
    pub EnableTimeUpdateTimer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaEngineEx_Impl: IMFMediaEngine_Impl {
    fn SetSourceFromByteStream(&self, pbytestream: windows_core::Ref<super::mfobjects::IMFByteStream>, purl: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetStatistics(&self, statisticid: MF_MEDIA_ENGINE_STATISTIC) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn UpdateVideoStream(&self, psrc: *const super::mfidl::MFVideoNormalizedRect, pdst: *const super::windef::RECT, pborderclr: *const super::mfobjects::MFARGB) -> windows_core::Result<()>;
    fn GetBalance(&self) -> f64;
    fn SetBalance(&self, balance: f64) -> windows_core::Result<()>;
    fn IsPlaybackRateSupported(&self, rate: f64) -> windows_core::BOOL;
    fn FrameStep(&self, forward: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetResourceCharacteristics(&self) -> windows_core::Result<u32>;
    fn GetPresentationAttribute(&self, guidmfattribute: *const windows_core::GUID) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn GetNumberOfStreams(&self) -> windows_core::Result<u32>;
    fn GetStreamAttribute(&self, dwstreamindex: u32, guidmfattribute: *const windows_core::GUID) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn GetStreamSelection(&self, dwstreamindex: u32) -> windows_core::Result<windows_core::BOOL>;
    fn SetStreamSelection(&self, dwstreamindex: u32, enabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn ApplyStreamSelections(&self) -> windows_core::Result<()>;
    fn IsProtected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn InsertVideoEffect(&self, peffect: windows_core::Ref<windows_core::IUnknown>, foptional: windows_core::BOOL) -> windows_core::Result<()>;
    fn InsertAudioEffect(&self, peffect: windows_core::Ref<windows_core::IUnknown>, foptional: windows_core::BOOL) -> windows_core::Result<()>;
    fn RemoveAllEffects(&self) -> windows_core::Result<()>;
    fn SetTimelineMarkerTimer(&self, timetofire: f64) -> windows_core::Result<()>;
    fn GetTimelineMarkerTimer(&self) -> windows_core::Result<f64>;
    fn CancelTimelineMarkerTimer(&self) -> windows_core::Result<()>;
    fn IsStereo3D(&self) -> windows_core::BOOL;
    fn GetStereo3DFramePackingMode(&self) -> windows_core::Result<MF_MEDIA_ENGINE_S3D_PACKING_MODE>;
    fn SetStereo3DFramePackingMode(&self, packmode: MF_MEDIA_ENGINE_S3D_PACKING_MODE) -> windows_core::Result<()>;
    fn GetStereo3DRenderMode(&self) -> windows_core::Result<super::mftransform::MF3DVideoOutputType>;
    fn SetStereo3DRenderMode(&self, outputtype: super::mftransform::MF3DVideoOutputType) -> windows_core::Result<()>;
    fn EnableWindowlessSwapchainMode(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetVideoSwapchainHandle(&self) -> windows_core::Result<super::winnt::HANDLE>;
    fn EnableHorizontalMirrorMode(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetAudioStreamCategory(&self) -> windows_core::Result<u32>;
    fn SetAudioStreamCategory(&self, category: u32) -> windows_core::Result<()>;
    fn GetAudioEndpointRole(&self) -> windows_core::Result<u32>;
    fn SetAudioEndpointRole(&self, role: u32) -> windows_core::Result<()>;
    fn GetRealTimeMode(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetRealTimeMode(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetCurrentTimeEx(&self, seektime: f64, seekmode: MF_MEDIA_ENGINE_SEEK_MODE) -> windows_core::Result<()>;
    fn EnableTimeUpdateTimer(&self, fenabletimer: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaEngineEx_Vtbl {
    pub const fn new<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSourceFromByteStream<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytestream: *mut core::ffi::c_void, purl: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetSourceFromByteStream(this, core::mem::transmute_copy(&pbytestream), core::mem::transmute(&purl)).into()
            }
        }
        unsafe extern "system" fn GetStatistics<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, statisticid: MF_MEDIA_ENGINE_STATISTIC, pstatistic: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStatistics(this, core::mem::transmute_copy(&statisticid)) {
                    Ok(ok__) => {
                        pstatistic.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UpdateVideoStream<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrc: *const super::mfidl::MFVideoNormalizedRect, pdst: *const super::windef::RECT, pborderclr: *const super::mfobjects::MFARGB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::UpdateVideoStream(this, core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&pborderclr)).into()
            }
        }
        unsafe extern "system" fn GetBalance<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::GetBalance(this)
            }
        }
        unsafe extern "system" fn SetBalance<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, balance: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetBalance(this, core::mem::transmute_copy(&balance)).into()
            }
        }
        unsafe extern "system" fn IsPlaybackRateSupported<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rate: f64) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::IsPlaybackRateSupported(this, core::mem::transmute_copy(&rate))
            }
        }
        unsafe extern "system" fn FrameStep<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, forward: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::FrameStep(this, core::mem::transmute_copy(&forward)).into()
            }
        }
        unsafe extern "system" fn GetResourceCharacteristics<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcharacteristics: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetResourceCharacteristics(this) {
                    Ok(ok__) => {
                        pcharacteristics.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPresentationAttribute<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmfattribute: *const windows_core::GUID, pvvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetPresentationAttribute(this, core::mem::transmute_copy(&guidmfattribute)) {
                    Ok(ok__) => {
                        pvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNumberOfStreams<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstreamcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetNumberOfStreams(this) {
                    Ok(ok__) => {
                        pdwstreamcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamAttribute<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, guidmfattribute: *const windows_core::GUID, pvvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStreamAttribute(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&guidmfattribute)) {
                    Ok(ok__) => {
                        pvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamSelection<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, penabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStreamSelection(this, core::mem::transmute_copy(&dwstreamindex)) {
                    Ok(ok__) => {
                        penabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamSelection<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, enabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetStreamSelection(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn ApplyStreamSelections<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::ApplyStreamSelections(this).into()
            }
        }
        unsafe extern "system" fn IsProtected<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprotected: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::IsProtected(this) {
                    Ok(ok__) => {
                        pprotected.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InsertVideoEffect<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void, foptional: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::InsertVideoEffect(this, core::mem::transmute_copy(&peffect), core::mem::transmute_copy(&foptional)).into()
            }
        }
        unsafe extern "system" fn InsertAudioEffect<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void, foptional: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::InsertAudioEffect(this, core::mem::transmute_copy(&peffect), core::mem::transmute_copy(&foptional)).into()
            }
        }
        unsafe extern "system" fn RemoveAllEffects<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::RemoveAllEffects(this).into()
            }
        }
        unsafe extern "system" fn SetTimelineMarkerTimer<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timetofire: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetTimelineMarkerTimer(this, core::mem::transmute_copy(&timetofire)).into()
            }
        }
        unsafe extern "system" fn GetTimelineMarkerTimer<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimetofire: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetTimelineMarkerTimer(this) {
                    Ok(ok__) => {
                        ptimetofire.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CancelTimelineMarkerTimer<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::CancelTimelineMarkerTimer(this).into()
            }
        }
        unsafe extern "system" fn IsStereo3D<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::IsStereo3D(this)
            }
        }
        unsafe extern "system" fn GetStereo3DFramePackingMode<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packmode: *mut MF_MEDIA_ENGINE_S3D_PACKING_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStereo3DFramePackingMode(this) {
                    Ok(ok__) => {
                        packmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStereo3DFramePackingMode<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, packmode: MF_MEDIA_ENGINE_S3D_PACKING_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetStereo3DFramePackingMode(this, core::mem::transmute_copy(&packmode)).into()
            }
        }
        unsafe extern "system" fn GetStereo3DRenderMode<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputtype: *mut super::mftransform::MF3DVideoOutputType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetStereo3DRenderMode(this) {
                    Ok(ok__) => {
                        outputtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStereo3DRenderMode<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputtype: super::mftransform::MF3DVideoOutputType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetStereo3DRenderMode(this, core::mem::transmute_copy(&outputtype)).into()
            }
        }
        unsafe extern "system" fn EnableWindowlessSwapchainMode<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::EnableWindowlessSwapchainMode(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn GetVideoSwapchainHandle<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phswapchain: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetVideoSwapchainHandle(this) {
                    Ok(ok__) => {
                        phswapchain.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableHorizontalMirrorMode<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::EnableHorizontalMirrorMode(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn GetAudioStreamCategory<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcategory: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetAudioStreamCategory(this) {
                    Ok(ok__) => {
                        pcategory.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAudioStreamCategory<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetAudioStreamCategory(this, core::mem::transmute_copy(&category)).into()
            }
        }
        unsafe extern "system" fn GetAudioEndpointRole<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prole: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetAudioEndpointRole(this) {
                    Ok(ok__) => {
                        prole.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAudioEndpointRole<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, role: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetAudioEndpointRole(this, core::mem::transmute_copy(&role)).into()
            }
        }
        unsafe extern "system" fn GetRealTimeMode<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineEx_Impl::GetRealTimeMode(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRealTimeMode<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetRealTimeMode(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn SetCurrentTimeEx<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, seektime: f64, seekmode: MF_MEDIA_ENGINE_SEEK_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::SetCurrentTimeEx(this, core::mem::transmute_copy(&seektime), core::mem::transmute_copy(&seekmode)).into()
            }
        }
        unsafe extern "system" fn EnableTimeUpdateTimer<Identity: IMFMediaEngineEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabletimer: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineEx_Impl::EnableTimeUpdateTimer(this, core::mem::transmute_copy(&fenabletimer)).into()
            }
        }
        Self {
            base__: IMFMediaEngine_Vtbl::new::<Identity, OFFSET>(),
            SetSourceFromByteStream: SetSourceFromByteStream::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            UpdateVideoStream: UpdateVideoStream::<Identity, OFFSET>,
            GetBalance: GetBalance::<Identity, OFFSET>,
            SetBalance: SetBalance::<Identity, OFFSET>,
            IsPlaybackRateSupported: IsPlaybackRateSupported::<Identity, OFFSET>,
            FrameStep: FrameStep::<Identity, OFFSET>,
            GetResourceCharacteristics: GetResourceCharacteristics::<Identity, OFFSET>,
            GetPresentationAttribute: GetPresentationAttribute::<Identity, OFFSET>,
            GetNumberOfStreams: GetNumberOfStreams::<Identity, OFFSET>,
            GetStreamAttribute: GetStreamAttribute::<Identity, OFFSET>,
            GetStreamSelection: GetStreamSelection::<Identity, OFFSET>,
            SetStreamSelection: SetStreamSelection::<Identity, OFFSET>,
            ApplyStreamSelections: ApplyStreamSelections::<Identity, OFFSET>,
            IsProtected: IsProtected::<Identity, OFFSET>,
            InsertVideoEffect: InsertVideoEffect::<Identity, OFFSET>,
            InsertAudioEffect: InsertAudioEffect::<Identity, OFFSET>,
            RemoveAllEffects: RemoveAllEffects::<Identity, OFFSET>,
            SetTimelineMarkerTimer: SetTimelineMarkerTimer::<Identity, OFFSET>,
            GetTimelineMarkerTimer: GetTimelineMarkerTimer::<Identity, OFFSET>,
            CancelTimelineMarkerTimer: CancelTimelineMarkerTimer::<Identity, OFFSET>,
            IsStereo3D: IsStereo3D::<Identity, OFFSET>,
            GetStereo3DFramePackingMode: GetStereo3DFramePackingMode::<Identity, OFFSET>,
            SetStereo3DFramePackingMode: SetStereo3DFramePackingMode::<Identity, OFFSET>,
            GetStereo3DRenderMode: GetStereo3DRenderMode::<Identity, OFFSET>,
            SetStereo3DRenderMode: SetStereo3DRenderMode::<Identity, OFFSET>,
            EnableWindowlessSwapchainMode: EnableWindowlessSwapchainMode::<Identity, OFFSET>,
            GetVideoSwapchainHandle: GetVideoSwapchainHandle::<Identity, OFFSET>,
            EnableHorizontalMirrorMode: EnableHorizontalMirrorMode::<Identity, OFFSET>,
            GetAudioStreamCategory: GetAudioStreamCategory::<Identity, OFFSET>,
            SetAudioStreamCategory: SetAudioStreamCategory::<Identity, OFFSET>,
            GetAudioEndpointRole: GetAudioEndpointRole::<Identity, OFFSET>,
            SetAudioEndpointRole: SetAudioEndpointRole::<Identity, OFFSET>,
            GetRealTimeMode: GetRealTimeMode::<Identity, OFFSET>,
            SetRealTimeMode: SetRealTimeMode::<Identity, OFFSET>,
            SetCurrentTimeEx: SetCurrentTimeEx::<Identity, OFFSET>,
            EnableTimeUpdateTimer: EnableTimeUpdateTimer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineEx as windows_core::Interface>::IID || iid == &<IMFMediaEngine as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "mftransform", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaEngineEx {}
windows_core::imp::define_interface!(IMFMediaEngineExtension, IMFMediaEngineExtension_Vtbl, 0x2f69d622_20b5_41e9_afdf_89ced1dda04e);
windows_core::imp::interface_hierarchy!(IMFMediaEngineExtension, windows_core::IUnknown);
impl IMFMediaEngineExtension {
    pub unsafe fn CanPlayType(&self, audioonly: bool, mimetype: &windows_core::BSTR) -> windows_core::Result<MF_MEDIA_ENGINE_CANPLAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanPlayType)(windows_core::Interface::as_raw(self), audioonly.into(), core::mem::transmute_copy(mimetype), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub unsafe fn BeginCreateObject<P1, P4, P5>(&self, bstrurl: &windows_core::BSTR, pbytestream: P1, r#type: super::mfidl::MF_OBJECT_TYPE, ppiunknowncancelcookie: *mut Option<windows_core::IUnknown>, pcallback: P4, punkstate: P5) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::mfobjects::IMFByteStream>,
        P4: windows_core::Param<super::mfobjects::IMFAsyncCallback>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginCreateObject)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrurl), pbytestream.param().abi(), r#type, core::mem::transmute(ppiunknowncancelcookie), pcallback.param().abi(), punkstate.param().abi()) }
    }
    pub unsafe fn CancelObjectCreation<P0>(&self, piunknowncancelcookie: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CancelObjectCreation)(windows_core::Interface::as_raw(self), piunknowncancelcookie.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndCreateObject<P0>(&self, presult: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<super::mfobjects::IMFAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndCreateObject)(windows_core::Interface::as_raw(self), presult.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CanPlayType: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *mut core::ffi::c_void, *mut MF_MEDIA_ENGINE_CANPLAY) -> windows_core::HRESULT,
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub BeginCreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::mfidl::MF_OBJECT_TYPE, *mut *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "mfobjects")))]
    BeginCreateObject: usize,
    pub CancelObjectCreation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub EndCreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndCreateObject: usize,
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
pub trait IMFMediaEngineExtension_Impl: windows_core::IUnknownImpl {
    fn CanPlayType(&self, audioonly: windows_core::BOOL, mimetype: &windows_core::BSTR) -> windows_core::Result<MF_MEDIA_ENGINE_CANPLAY>;
    fn BeginCreateObject(&self, bstrurl: &windows_core::BSTR, pbytestream: windows_core::Ref<super::mfobjects::IMFByteStream>, r#type: super::mfidl::MF_OBJECT_TYPE, ppiunknowncancelcookie: windows_core::OutRef<windows_core::IUnknown>, pcallback: windows_core::Ref<super::mfobjects::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CancelObjectCreation(&self, piunknowncancelcookie: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndCreateObject(&self, presult: windows_core::Ref<super::mfobjects::IMFAsyncResult>) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
impl IMFMediaEngineExtension_Vtbl {
    pub const fn new<Identity: IMFMediaEngineExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanPlayType<Identity: IMFMediaEngineExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, audioonly: windows_core::BOOL, mimetype: *mut core::ffi::c_void, panswer: *mut MF_MEDIA_ENGINE_CANPLAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineExtension_Impl::CanPlayType(this, core::mem::transmute_copy(&audioonly), core::mem::transmute(&mimetype)) {
                    Ok(ok__) => {
                        panswer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginCreateObject<Identity: IMFMediaEngineExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrurl: *mut core::ffi::c_void, pbytestream: *mut core::ffi::c_void, r#type: super::mfidl::MF_OBJECT_TYPE, ppiunknowncancelcookie: *mut *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineExtension_Impl::BeginCreateObject(this, core::mem::transmute(&bstrurl), core::mem::transmute_copy(&pbytestream), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&ppiunknowncancelcookie), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn CancelObjectCreation<Identity: IMFMediaEngineExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piunknowncancelcookie: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineExtension_Impl::CancelObjectCreation(this, core::mem::transmute_copy(&piunknowncancelcookie)).into()
            }
        }
        unsafe extern "system" fn EndCreateObject<Identity: IMFMediaEngineExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineExtension_Impl::EndCreateObject(this, core::mem::transmute_copy(&presult)) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CanPlayType: CanPlayType::<Identity, OFFSET>,
            BeginCreateObject: BeginCreateObject::<Identity, OFFSET>,
            CancelObjectCreation: CancelObjectCreation::<Identity, OFFSET>,
            EndCreateObject: EndCreateObject::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineExtension as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
impl windows_core::RuntimeName for IMFMediaEngineExtension {}
windows_core::imp::define_interface!(IMFMediaEngineNeedKeyNotify, IMFMediaEngineNeedKeyNotify_Vtbl, 0x46a30204_a696_4b18_8804_246b8f031bb1);
windows_core::imp::interface_hierarchy!(IMFMediaEngineNeedKeyNotify, windows_core::IUnknown);
impl IMFMediaEngineNeedKeyNotify {
    pub unsafe fn NeedKey(&self, initdata: Option<&[u8]>) {
        unsafe {
            (windows_core::Interface::vtable(self).NeedKey)(windows_core::Interface::as_raw(self), initdata.map_or(core::ptr::null(), |slice| slice.as_ptr()), initdata.map_or(0, |slice| slice.len().try_into().unwrap()));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineNeedKeyNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NeedKey: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32),
}
pub trait IMFMediaEngineNeedKeyNotify_Impl: windows_core::IUnknownImpl {
    fn NeedKey(&self, initdata: *const u8, cb: u32);
}
impl IMFMediaEngineNeedKeyNotify_Vtbl {
    pub const fn new<Identity: IMFMediaEngineNeedKeyNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NeedKey<Identity: IMFMediaEngineNeedKeyNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initdata: *const u8, cb: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineNeedKeyNotify_Impl::NeedKey(this, core::mem::transmute_copy(&initdata), core::mem::transmute_copy(&cb));
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NeedKey: NeedKey::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineNeedKeyNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineNeedKeyNotify {}
windows_core::imp::define_interface!(IMFMediaEngineNotify, IMFMediaEngineNotify_Vtbl, 0xfee7c112_e776_42b5_9bbf_0048524e2bd5);
windows_core::imp::interface_hierarchy!(IMFMediaEngineNotify, windows_core::IUnknown);
impl IMFMediaEngineNotify {
    pub unsafe fn EventNotify(&self, event: u32, param1: usize, param2: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EventNotify)(windows_core::Interface::as_raw(self), event, param1, param2) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EventNotify: unsafe extern "system" fn(*mut core::ffi::c_void, u32, usize, u32) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineNotify_Impl: windows_core::IUnknownImpl {
    fn EventNotify(&self, event: u32, param1: usize, param2: u32) -> windows_core::Result<()>;
}
impl IMFMediaEngineNotify_Vtbl {
    pub const fn new<Identity: IMFMediaEngineNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EventNotify<Identity: IMFMediaEngineNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: u32, param1: usize, param2: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineNotify_Impl::EventNotify(this, core::mem::transmute_copy(&event), core::mem::transmute_copy(&param1), core::mem::transmute_copy(&param2)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), EventNotify: EventNotify::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineNotify {}
windows_core::imp::define_interface!(IMFMediaEngineOPMInfo, IMFMediaEngineOPMInfo_Vtbl, 0x765763e6_6c01_4b01_bb0f_b829f60ed28c);
windows_core::imp::interface_hierarchy!(IMFMediaEngineOPMInfo, windows_core::IUnknown);
impl IMFMediaEngineOPMInfo {
    pub unsafe fn GetOPMInfo(&self, pstatus: *mut MF_MEDIA_ENGINE_OPM_STATUS, pconstricted: *mut windows_core::BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOPMInfo)(windows_core::Interface::as_raw(self), pstatus as _, pconstricted as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineOPMInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOPMInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_MEDIA_ENGINE_OPM_STATUS, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineOPMInfo_Impl: windows_core::IUnknownImpl {
    fn GetOPMInfo(&self, pstatus: *mut MF_MEDIA_ENGINE_OPM_STATUS, pconstricted: *mut windows_core::BOOL) -> windows_core::Result<()>;
}
impl IMFMediaEngineOPMInfo_Vtbl {
    pub const fn new<Identity: IMFMediaEngineOPMInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOPMInfo<Identity: IMFMediaEngineOPMInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut MF_MEDIA_ENGINE_OPM_STATUS, pconstricted: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineOPMInfo_Impl::GetOPMInfo(this, core::mem::transmute_copy(&pstatus), core::mem::transmute_copy(&pconstricted)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOPMInfo: GetOPMInfo::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineOPMInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineOPMInfo {}
windows_core::imp::define_interface!(IMFMediaEngineProtectedContent, IMFMediaEngineProtectedContent_Vtbl, 0x9f8021e8_9c8c_487e_bb5c_79aa4779938c);
windows_core::imp::interface_hierarchy!(IMFMediaEngineProtectedContent, windows_core::IUnknown);
impl IMFMediaEngineProtectedContent {
    pub unsafe fn ShareResources<P0>(&self, punkdevicecontext: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).ShareResources)(windows_core::Interface::as_raw(self), punkdevicecontext.param().abi()) }
    }
    pub unsafe fn GetRequiredProtections(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRequiredProtections)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetOPMWindow(&self, hwnd: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetOPMWindow)(windows_core::Interface::as_raw(self), hwnd) }
    }
    #[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
    pub unsafe fn TransferVideoFrame<P0>(&self, pdstsurf: P0, psrc: Option<*const super::mfidl::MFVideoNormalizedRect>, pdst: *const super::windef::RECT, pborderclr: Option<*const super::mfobjects::MFARGB>) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransferVideoFrame)(windows_core::Interface::as_raw(self), pdstsurf.param().abi(), psrc.unwrap_or(core::mem::zeroed()) as _, pdst, pborderclr.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn SetContentProtectionManager<P0>(&self, pcpm: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::mfidl::IMFContentProtectionManager>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContentProtectionManager)(windows_core::Interface::as_raw(self), pcpm.param().abi()) }
    }
    pub unsafe fn SetApplicationCertificate(&self, pbblob: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetApplicationCertificate)(windows_core::Interface::as_raw(self), pbblob.as_ptr(), pbblob.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineProtectedContent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShareResources: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRequiredProtections: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetOPMWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetOPMWindow: usize,
    #[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
    pub TransferVideoFrame: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::mfidl::MFVideoNormalizedRect, *const super::windef::RECT, *const super::mfobjects::MFARGB, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "mfobjects", feature = "windef")))]
    TransferVideoFrame: usize,
    #[cfg(feature = "mfidl")]
    pub SetContentProtectionManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    SetContentProtectionManager: usize,
    pub SetApplicationCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
pub trait IMFMediaEngineProtectedContent_Impl: windows_core::IUnknownImpl {
    fn ShareResources(&self, punkdevicecontext: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetRequiredProtections(&self) -> windows_core::Result<u32>;
    fn SetOPMWindow(&self, hwnd: super::windef::HWND) -> windows_core::Result<()>;
    fn TransferVideoFrame(&self, pdstsurf: windows_core::Ref<windows_core::IUnknown>, psrc: *const super::mfidl::MFVideoNormalizedRect, pdst: *const super::windef::RECT, pborderclr: *const super::mfobjects::MFARGB) -> windows_core::Result<u32>;
    fn SetContentProtectionManager(&self, pcpm: windows_core::Ref<super::mfidl::IMFContentProtectionManager>) -> windows_core::Result<()>;
    fn SetApplicationCertificate(&self, pbblob: *const u8, cbblob: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
impl IMFMediaEngineProtectedContent_Vtbl {
    pub const fn new<Identity: IMFMediaEngineProtectedContent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShareResources<Identity: IMFMediaEngineProtectedContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkdevicecontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineProtectedContent_Impl::ShareResources(this, core::mem::transmute_copy(&punkdevicecontext)).into()
            }
        }
        unsafe extern "system" fn GetRequiredProtections<Identity: IMFMediaEngineProtectedContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pframeprotectionflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineProtectedContent_Impl::GetRequiredProtections(this) {
                    Ok(ok__) => {
                        pframeprotectionflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetOPMWindow<Identity: IMFMediaEngineProtectedContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineProtectedContent_Impl::SetOPMWindow(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn TransferVideoFrame<Identity: IMFMediaEngineProtectedContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstsurf: *mut core::ffi::c_void, psrc: *const super::mfidl::MFVideoNormalizedRect, pdst: *const super::windef::RECT, pborderclr: *const super::mfobjects::MFARGB, pframeprotectionflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineProtectedContent_Impl::TransferVideoFrame(this, core::mem::transmute_copy(&pdstsurf), core::mem::transmute_copy(&psrc), core::mem::transmute_copy(&pdst), core::mem::transmute_copy(&pborderclr)) {
                    Ok(ok__) => {
                        pframeprotectionflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContentProtectionManager<Identity: IMFMediaEngineProtectedContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpm: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineProtectedContent_Impl::SetContentProtectionManager(this, core::mem::transmute_copy(&pcpm)).into()
            }
        }
        unsafe extern "system" fn SetApplicationCertificate<Identity: IMFMediaEngineProtectedContent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbblob: *const u8, cbblob: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineProtectedContent_Impl::SetApplicationCertificate(this, core::mem::transmute_copy(&pbblob), core::mem::transmute_copy(&cbblob)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ShareResources: ShareResources::<Identity, OFFSET>,
            GetRequiredProtections: GetRequiredProtections::<Identity, OFFSET>,
            SetOPMWindow: SetOPMWindow::<Identity, OFFSET>,
            TransferVideoFrame: TransferVideoFrame::<Identity, OFFSET>,
            SetContentProtectionManager: SetContentProtectionManager::<Identity, OFFSET>,
            SetApplicationCertificate: SetApplicationCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineProtectedContent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "mfobjects", feature = "windef"))]
impl windows_core::RuntimeName for IMFMediaEngineProtectedContent {}
windows_core::imp::define_interface!(IMFMediaEngineSrcElements, IMFMediaEngineSrcElements_Vtbl, 0x7a5e5354_b114_4c72_b991_3131d75032ea);
windows_core::imp::interface_hierarchy!(IMFMediaEngineSrcElements, windows_core::IUnknown);
impl IMFMediaEngineSrcElements {
    pub unsafe fn GetLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetURL(&self, index: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetType(&self, index: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetMedia(&self, index: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMedia)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddElement(&self, purl: &windows_core::BSTR, ptype: &windows_core::BSTR, pmedia: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddElement)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(purl), core::mem::transmute_copy(ptype), core::mem::transmute_copy(pmedia)) }
    }
    pub unsafe fn RemoveAllElements(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllElements)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineSrcElements_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMedia: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllElements: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineSrcElements_Impl: windows_core::IUnknownImpl {
    fn GetLength(&self) -> u32;
    fn GetURL(&self, index: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetType(&self, index: u32) -> windows_core::Result<windows_core::BSTR>;
    fn GetMedia(&self, index: u32) -> windows_core::Result<windows_core::BSTR>;
    fn AddElement(&self, purl: &windows_core::BSTR, ptype: &windows_core::BSTR, pmedia: &windows_core::BSTR) -> windows_core::Result<()>;
    fn RemoveAllElements(&self) -> windows_core::Result<()>;
}
impl IMFMediaEngineSrcElements_Vtbl {
    pub const fn new<Identity: IMFMediaEngineSrcElements_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLength<Identity: IMFMediaEngineSrcElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineSrcElements_Impl::GetLength(this)
            }
        }
        unsafe extern "system" fn GetURL<Identity: IMFMediaEngineSrcElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, purl: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineSrcElements_Impl::GetURL(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        purl.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: IMFMediaEngineSrcElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineSrcElements_Impl::GetType(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMedia<Identity: IMFMediaEngineSrcElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pmedia: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineSrcElements_Impl::GetMedia(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pmedia.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddElement<Identity: IMFMediaEngineSrcElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, purl: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineSrcElements_Impl::AddElement(this, core::mem::transmute(&purl), core::mem::transmute(&ptype), core::mem::transmute(&pmedia)).into()
            }
        }
        unsafe extern "system" fn RemoveAllElements<Identity: IMFMediaEngineSrcElements_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineSrcElements_Impl::RemoveAllElements(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            GetURL: GetURL::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetMedia: GetMedia::<Identity, OFFSET>,
            AddElement: AddElement::<Identity, OFFSET>,
            RemoveAllElements: RemoveAllElements::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineSrcElements as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineSrcElements {}
windows_core::imp::define_interface!(IMFMediaEngineSrcElementsEx, IMFMediaEngineSrcElementsEx_Vtbl, 0x654a6bb3_e1a3_424a_9908_53a43a0dfda0);
impl core::ops::Deref for IMFMediaEngineSrcElementsEx {
    type Target = IMFMediaEngineSrcElements;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaEngineSrcElementsEx, windows_core::IUnknown, IMFMediaEngineSrcElements);
impl IMFMediaEngineSrcElementsEx {
    pub unsafe fn AddElementEx(&self, purl: &windows_core::BSTR, ptype: &windows_core::BSTR, pmedia: &windows_core::BSTR, keysystem: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddElementEx)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(purl), core::mem::transmute_copy(ptype), core::mem::transmute_copy(pmedia), core::mem::transmute_copy(keysystem)) }
    }
    pub unsafe fn GetKeySystem(&self, index: u32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeySystem)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineSrcElementsEx_Vtbl {
    pub base__: IMFMediaEngineSrcElements_Vtbl,
    pub AddElementEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetKeySystem: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineSrcElementsEx_Impl: IMFMediaEngineSrcElements_Impl {
    fn AddElementEx(&self, purl: &windows_core::BSTR, ptype: &windows_core::BSTR, pmedia: &windows_core::BSTR, keysystem: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetKeySystem(&self, index: u32) -> windows_core::Result<windows_core::BSTR>;
}
impl IMFMediaEngineSrcElementsEx_Vtbl {
    pub const fn new<Identity: IMFMediaEngineSrcElementsEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddElementEx<Identity: IMFMediaEngineSrcElementsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, purl: *mut core::ffi::c_void, ptype: *mut core::ffi::c_void, pmedia: *mut core::ffi::c_void, keysystem: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineSrcElementsEx_Impl::AddElementEx(this, core::mem::transmute(&purl), core::mem::transmute(&ptype), core::mem::transmute(&pmedia), core::mem::transmute(&keysystem)).into()
            }
        }
        unsafe extern "system" fn GetKeySystem<Identity: IMFMediaEngineSrcElementsEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineSrcElementsEx_Impl::GetKeySystem(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMFMediaEngineSrcElements_Vtbl::new::<Identity, OFFSET>(),
            AddElementEx: AddElementEx::<Identity, OFFSET>,
            GetKeySystem: GetKeySystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineSrcElementsEx as windows_core::Interface>::IID || iid == &<IMFMediaEngineSrcElements as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineSrcElementsEx {}
windows_core::imp::define_interface!(IMFMediaEngineSupportsSourceTransfer, IMFMediaEngineSupportsSourceTransfer_Vtbl, 0xa724b056_1b2e_4642_a6f3_db9420c52908);
windows_core::imp::interface_hierarchy!(IMFMediaEngineSupportsSourceTransfer, windows_core::IUnknown);
impl IMFMediaEngineSupportsSourceTransfer {
    pub unsafe fn ShouldTransferSource(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ShouldTransferSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub unsafe fn DetachMediaSource(&self, ppbytestream: *mut Option<super::mfobjects::IMFByteStream>, ppmediasource: *mut Option<super::mfidl::IMFMediaSource>, ppmse: *mut Option<IMFMediaSourceExtension>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DetachMediaSource)(windows_core::Interface::as_raw(self), core::mem::transmute(ppbytestream), core::mem::transmute(ppmediasource), core::mem::transmute(ppmse)) }
    }
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub unsafe fn AttachMediaSource<P0, P1, P2>(&self, pbytestream: P0, pmediasource: P1, pmse: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::mfobjects::IMFByteStream>,
        P1: windows_core::Param<super::mfidl::IMFMediaSource>,
        P2: windows_core::Param<IMFMediaSourceExtension>,
    {
        unsafe { (windows_core::Interface::vtable(self).AttachMediaSource)(windows_core::Interface::as_raw(self), pbytestream.param().abi(), pmediasource.param().abi(), pmse.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineSupportsSourceTransfer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShouldTransferSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub DetachMediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "mfobjects")))]
    DetachMediaSource: usize,
    #[cfg(all(feature = "mfidl", feature = "mfobjects"))]
    pub AttachMediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "mfobjects")))]
    AttachMediaSource: usize,
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
pub trait IMFMediaEngineSupportsSourceTransfer_Impl: windows_core::IUnknownImpl {
    fn ShouldTransferSource(&self) -> windows_core::Result<windows_core::BOOL>;
    fn DetachMediaSource(&self, ppbytestream: windows_core::OutRef<super::mfobjects::IMFByteStream>, ppmediasource: windows_core::OutRef<super::mfidl::IMFMediaSource>, ppmse: windows_core::OutRef<IMFMediaSourceExtension>) -> windows_core::Result<()>;
    fn AttachMediaSource(&self, pbytestream: windows_core::Ref<super::mfobjects::IMFByteStream>, pmediasource: windows_core::Ref<super::mfidl::IMFMediaSource>, pmse: windows_core::Ref<IMFMediaSourceExtension>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
impl IMFMediaEngineSupportsSourceTransfer_Vtbl {
    pub const fn new<Identity: IMFMediaEngineSupportsSourceTransfer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShouldTransferSource<Identity: IMFMediaEngineSupportsSourceTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfshouldtransfer: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineSupportsSourceTransfer_Impl::ShouldTransferSource(this) {
                    Ok(ok__) => {
                        pfshouldtransfer.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DetachMediaSource<Identity: IMFMediaEngineSupportsSourceTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbytestream: *mut *mut core::ffi::c_void, ppmediasource: *mut *mut core::ffi::c_void, ppmse: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineSupportsSourceTransfer_Impl::DetachMediaSource(this, core::mem::transmute_copy(&ppbytestream), core::mem::transmute_copy(&ppmediasource), core::mem::transmute_copy(&ppmse)).into()
            }
        }
        unsafe extern "system" fn AttachMediaSource<Identity: IMFMediaEngineSupportsSourceTransfer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytestream: *mut core::ffi::c_void, pmediasource: *mut core::ffi::c_void, pmse: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineSupportsSourceTransfer_Impl::AttachMediaSource(this, core::mem::transmute_copy(&pbytestream), core::mem::transmute_copy(&pmediasource), core::mem::transmute_copy(&pmse)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ShouldTransferSource: ShouldTransferSource::<Identity, OFFSET>,
            DetachMediaSource: DetachMediaSource::<Identity, OFFSET>,
            AttachMediaSource: AttachMediaSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineSupportsSourceTransfer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
impl windows_core::RuntimeName for IMFMediaEngineSupportsSourceTransfer {}
windows_core::imp::define_interface!(IMFMediaEngineTransferSource, IMFMediaEngineTransferSource_Vtbl, 0x24230452_fe54_40cc_94f3_fcc394c340d6);
windows_core::imp::interface_hierarchy!(IMFMediaEngineTransferSource, windows_core::IUnknown);
impl IMFMediaEngineTransferSource {
    pub unsafe fn TransferSourceToMediaEngine<P0>(&self, destination: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFMediaEngine>,
    {
        unsafe { (windows_core::Interface::vtable(self).TransferSourceToMediaEngine)(windows_core::Interface::as_raw(self), destination.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineTransferSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TransferSourceToMediaEngine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineTransferSource_Impl: windows_core::IUnknownImpl {
    fn TransferSourceToMediaEngine(&self, destination: windows_core::Ref<IMFMediaEngine>) -> windows_core::Result<()>;
}
impl IMFMediaEngineTransferSource_Vtbl {
    pub const fn new<Identity: IMFMediaEngineTransferSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TransferSourceToMediaEngine<Identity: IMFMediaEngineTransferSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineTransferSource_Impl::TransferSourceToMediaEngine(this, core::mem::transmute_copy(&destination)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TransferSourceToMediaEngine: TransferSourceToMediaEngine::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineTransferSource as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineTransferSource {}
windows_core::imp::define_interface!(IMFMediaEngineVideoRendererEffect, IMFMediaEngineVideoRendererEffect_Vtbl, 0x6bdf1188_6ec8_44ff_8ccc_bfca0d12afa3);
windows_core::imp::interface_hierarchy!(IMFMediaEngineVideoRendererEffect, windows_core::IUnknown);
impl IMFMediaEngineVideoRendererEffect {
    pub unsafe fn SetEffect<P0>(&self, peffect: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEffect)(windows_core::Interface::as_raw(self), peffect.param().abi()) }
    }
    pub unsafe fn ClearEffect(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearEffect)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineVideoRendererEffect_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearEffect: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineVideoRendererEffect_Impl: windows_core::IUnknownImpl {
    fn SetEffect(&self, peffect: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn ClearEffect(&self) -> windows_core::Result<()>;
}
impl IMFMediaEngineVideoRendererEffect_Vtbl {
    pub const fn new<Identity: IMFMediaEngineVideoRendererEffect_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEffect<Identity: IMFMediaEngineVideoRendererEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, peffect: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineVideoRendererEffect_Impl::SetEffect(this, core::mem::transmute_copy(&peffect)).into()
            }
        }
        unsafe extern "system" fn ClearEffect<Identity: IMFMediaEngineVideoRendererEffect_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineVideoRendererEffect_Impl::ClearEffect(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetEffect: SetEffect::<Identity, OFFSET>,
            ClearEffect: ClearEffect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineVideoRendererEffect as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineVideoRendererEffect {}
windows_core::imp::define_interface!(IMFMediaEngineWebSupport, IMFMediaEngineWebSupport_Vtbl, 0xba2743a1_07e0_48ef_84b6_9a2ed023ca6c);
windows_core::imp::interface_hierarchy!(IMFMediaEngineWebSupport, windows_core::IUnknown);
impl IMFMediaEngineWebSupport {
    pub unsafe fn ShouldDelayTheLoadEvent(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).ShouldDelayTheLoadEvent)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ConnectWebAudio(&self, dwsamplerate: u32) -> windows_core::Result<IAudioSourceProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectWebAudio)(windows_core::Interface::as_raw(self), dwsamplerate, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DisconnectWebAudio(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisconnectWebAudio)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaEngineWebSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ShouldDelayTheLoadEvent: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub ConnectWebAudio: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisconnectWebAudio: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaEngineWebSupport_Impl: windows_core::IUnknownImpl {
    fn ShouldDelayTheLoadEvent(&self) -> windows_core::BOOL;
    fn ConnectWebAudio(&self, dwsamplerate: u32) -> windows_core::Result<IAudioSourceProvider>;
    fn DisconnectWebAudio(&self) -> windows_core::Result<()>;
}
impl IMFMediaEngineWebSupport_Vtbl {
    pub const fn new<Identity: IMFMediaEngineWebSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShouldDelayTheLoadEvent<Identity: IMFMediaEngineWebSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineWebSupport_Impl::ShouldDelayTheLoadEvent(this)
            }
        }
        unsafe extern "system" fn ConnectWebAudio<Identity: IMFMediaEngineWebSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsamplerate: u32, ppsourceprovider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaEngineWebSupport_Impl::ConnectWebAudio(this, core::mem::transmute_copy(&dwsamplerate)) {
                    Ok(ok__) => {
                        ppsourceprovider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DisconnectWebAudio<Identity: IMFMediaEngineWebSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaEngineWebSupport_Impl::DisconnectWebAudio(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ShouldDelayTheLoadEvent: ShouldDelayTheLoadEvent::<Identity, OFFSET>,
            ConnectWebAudio: ConnectWebAudio::<Identity, OFFSET>,
            DisconnectWebAudio: DisconnectWebAudio::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaEngineWebSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaEngineWebSupport {}
windows_core::imp::define_interface!(IMFMediaError, IMFMediaError_Vtbl, 0xfc0e10d2_ab2a_4501_a951_06bb1075184c);
windows_core::imp::interface_hierarchy!(IMFMediaError, windows_core::IUnknown);
impl IMFMediaError {
    pub unsafe fn GetErrorCode(&self) -> u16 {
        unsafe { (windows_core::Interface::vtable(self).GetErrorCode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetExtendedErrorCode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtendedErrorCode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetErrorCode(&self, error: MF_MEDIA_ENGINE_ERR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetErrorCode)(windows_core::Interface::as_raw(self), error) }
    }
    pub unsafe fn SetExtendedErrorCode(&self, error: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetExtendedErrorCode)(windows_core::Interface::as_raw(self), error) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void) -> u16,
    pub GetExtendedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, MF_MEDIA_ENGINE_ERR) -> windows_core::HRESULT,
    pub SetExtendedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
}
pub trait IMFMediaError_Impl: windows_core::IUnknownImpl {
    fn GetErrorCode(&self) -> u16;
    fn GetExtendedErrorCode(&self) -> windows_core::Result<()>;
    fn SetErrorCode(&self, error: MF_MEDIA_ENGINE_ERR) -> windows_core::Result<()>;
    fn SetExtendedErrorCode(&self, error: windows_core::HRESULT) -> windows_core::Result<()>;
}
impl IMFMediaError_Vtbl {
    pub const fn new<Identity: IMFMediaError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetErrorCode<Identity: IMFMediaError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u16 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaError_Impl::GetErrorCode(this)
            }
        }
        unsafe extern "system" fn GetExtendedErrorCode<Identity: IMFMediaError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaError_Impl::GetExtendedErrorCode(this).into()
            }
        }
        unsafe extern "system" fn SetErrorCode<Identity: IMFMediaError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: MF_MEDIA_ENGINE_ERR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaError_Impl::SetErrorCode(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn SetExtendedErrorCode<Identity: IMFMediaError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaError_Impl::SetExtendedErrorCode(this, core::mem::transmute_copy(&error)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetErrorCode: GetErrorCode::<Identity, OFFSET>,
            GetExtendedErrorCode: GetExtendedErrorCode::<Identity, OFFSET>,
            SetErrorCode: SetErrorCode::<Identity, OFFSET>,
            SetExtendedErrorCode: SetExtendedErrorCode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaError as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaError {}
windows_core::imp::define_interface!(IMFMediaKeySession, IMFMediaKeySession_Vtbl, 0x24fa67d5_d1d0_4dc5_995c_c0efdc191fb5);
windows_core::imp::interface_hierarchy!(IMFMediaKeySession, windows_core::IUnknown);
impl IMFMediaKeySession {
    pub unsafe fn GetError(&self, code: *mut u16, systemcode: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetError)(windows_core::Interface::as_raw(self), code as _, systemcode as _) }
    }
    pub unsafe fn get_KeySystem(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeySystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_SessionId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Update(&self, key: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), key.as_ptr(), key.len().try_into().unwrap()) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaKeySession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetError: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16, *mut u32) -> windows_core::HRESULT,
    pub get_KeySystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaKeySession_Impl: windows_core::IUnknownImpl {
    fn GetError(&self, code: *mut u16, systemcode: *mut u32) -> windows_core::Result<()>;
    fn get_KeySystem(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_SessionId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Update(&self, key: *const u8, cb: u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IMFMediaKeySession_Vtbl {
    pub const fn new<Identity: IMFMediaKeySession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetError<Identity: IMFMediaKeySession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, code: *mut u16, systemcode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySession_Impl::GetError(this, core::mem::transmute_copy(&code), core::mem::transmute_copy(&systemcode)).into()
            }
        }
        unsafe extern "system" fn get_KeySystem<Identity: IMFMediaKeySession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keysystem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeySession_Impl::get_KeySystem(this) {
                    Ok(ok__) => {
                        keysystem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SessionId<Identity: IMFMediaKeySession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sessionid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeySession_Impl::get_SessionId(this) {
                    Ok(ok__) => {
                        sessionid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Update<Identity: IMFMediaKeySession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const u8, cb: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySession_Impl::Update(this, core::mem::transmute_copy(&key), core::mem::transmute_copy(&cb)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IMFMediaKeySession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySession_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetError: GetError::<Identity, OFFSET>,
            get_KeySystem: get_KeySystem::<Identity, OFFSET>,
            get_SessionId: get_SessionId::<Identity, OFFSET>,
            Update: Update::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaKeySession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaKeySession {}
windows_core::imp::define_interface!(IMFMediaKeySession2, IMFMediaKeySession2_Vtbl, 0xe9707e05_6d55_4636_b185_3de21210bd75);
impl core::ops::Deref for IMFMediaKeySession2 {
    type Target = IMFMediaKeySession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaKeySession2, windows_core::IUnknown, IMFMediaKeySession);
impl IMFMediaKeySession2 {
    #[cfg(feature = "mfidl")]
    pub unsafe fn get_KeyStatuses(&self, pkeystatusesarray: *mut *mut super::mfidl::MFMediaKeyStatus, pusize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_KeyStatuses)(windows_core::Interface::as_raw(self), pkeystatusesarray as _, pusize as _) }
    }
    pub unsafe fn Load(&self, bstrsessionid: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrsessionid), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GenerateRequest(&self, initdatatype: &windows_core::BSTR, pbinitdata: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GenerateRequest)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(initdatatype), pbinitdata.as_ptr(), pbinitdata.len().try_into().unwrap()) }
    }
    pub unsafe fn get_Expiration(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Expiration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Remove(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaKeySession2_Vtbl {
    pub base__: IMFMediaKeySession_Vtbl,
    #[cfg(feature = "mfidl")]
    pub get_KeyStatuses: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::mfidl::MFMediaKeyStatus, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    get_KeyStatuses: usize,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GenerateRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub get_Expiration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfidl")]
pub trait IMFMediaKeySession2_Impl: IMFMediaKeySession_Impl {
    fn get_KeyStatuses(&self, pkeystatusesarray: *mut *mut super::mfidl::MFMediaKeyStatus, pusize: *mut u32) -> windows_core::Result<()>;
    fn Load(&self, bstrsessionid: &windows_core::BSTR) -> windows_core::Result<windows_core::BOOL>;
    fn GenerateRequest(&self, initdatatype: &windows_core::BSTR, pbinitdata: *const u8, cb: u32) -> windows_core::Result<()>;
    fn get_Expiration(&self) -> windows_core::Result<f64>;
    fn Remove(&self) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "mfidl")]
impl IMFMediaKeySession2_Vtbl {
    pub const fn new<Identity: IMFMediaKeySession2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_KeyStatuses<Identity: IMFMediaKeySession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeystatusesarray: *mut *mut super::mfidl::MFMediaKeyStatus, pusize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySession2_Impl::get_KeyStatuses(this, core::mem::transmute_copy(&pkeystatusesarray), core::mem::transmute_copy(&pusize)).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IMFMediaKeySession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrsessionid: *mut core::ffi::c_void, pfloaded: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeySession2_Impl::Load(this, core::mem::transmute(&bstrsessionid)) {
                    Ok(ok__) => {
                        pfloaded.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GenerateRequest<Identity: IMFMediaKeySession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, initdatatype: *mut core::ffi::c_void, pbinitdata: *const u8, cb: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySession2_Impl::GenerateRequest(this, core::mem::transmute(&initdatatype), core::mem::transmute_copy(&pbinitdata), core::mem::transmute_copy(&cb)).into()
            }
        }
        unsafe extern "system" fn get_Expiration<Identity: IMFMediaKeySession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dblexpiration: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeySession2_Impl::get_Expiration(this) {
                    Ok(ok__) => {
                        dblexpiration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Remove<Identity: IMFMediaKeySession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySession2_Impl::Remove(this).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFMediaKeySession2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySession2_Impl::Shutdown(this).into()
            }
        }
        Self {
            base__: IMFMediaKeySession_Vtbl::new::<Identity, OFFSET>(),
            get_KeyStatuses: get_KeyStatuses::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            GenerateRequest: GenerateRequest::<Identity, OFFSET>,
            get_Expiration: get_Expiration::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaKeySession2 as windows_core::Interface>::IID || iid == &<IMFMediaKeySession as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfidl")]
impl windows_core::RuntimeName for IMFMediaKeySession2 {}
windows_core::imp::define_interface!(IMFMediaKeySessionNotify, IMFMediaKeySessionNotify_Vtbl, 0x6a0083f9_8947_4c1d_9ce0_cdee22b23135);
windows_core::imp::interface_hierarchy!(IMFMediaKeySessionNotify, windows_core::IUnknown);
impl IMFMediaKeySessionNotify {
    pub unsafe fn KeyMessage(&self, destinationurl: &windows_core::BSTR, message: &[u8]) {
        unsafe {
            (windows_core::Interface::vtable(self).KeyMessage)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(destinationurl), message.as_ptr(), message.len().try_into().unwrap());
        }
    }
    pub unsafe fn KeyAdded(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).KeyAdded)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn KeyError(&self, code: u16, systemcode: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).KeyError)(windows_core::Interface::as_raw(self), code, systemcode);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaKeySessionNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub KeyMessage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32),
    pub KeyAdded: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub KeyError: unsafe extern "system" fn(*mut core::ffi::c_void, u16, u32),
}
pub trait IMFMediaKeySessionNotify_Impl: windows_core::IUnknownImpl {
    fn KeyMessage(&self, destinationurl: &windows_core::BSTR, message: *const u8, cb: u32);
    fn KeyAdded(&self);
    fn KeyError(&self, code: u16, systemcode: u32);
}
impl IMFMediaKeySessionNotify_Vtbl {
    pub const fn new<Identity: IMFMediaKeySessionNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KeyMessage<Identity: IMFMediaKeySessionNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, destinationurl: *mut core::ffi::c_void, message: *const u8, cb: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySessionNotify_Impl::KeyMessage(this, core::mem::transmute(&destinationurl), core::mem::transmute_copy(&message), core::mem::transmute_copy(&cb));
            }
        }
        unsafe extern "system" fn KeyAdded<Identity: IMFMediaKeySessionNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySessionNotify_Impl::KeyAdded(this);
            }
        }
        unsafe extern "system" fn KeyError<Identity: IMFMediaKeySessionNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, code: u16, systemcode: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySessionNotify_Impl::KeyError(this, core::mem::transmute_copy(&code), core::mem::transmute_copy(&systemcode));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            KeyMessage: KeyMessage::<Identity, OFFSET>,
            KeyAdded: KeyAdded::<Identity, OFFSET>,
            KeyError: KeyError::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaKeySessionNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaKeySessionNotify {}
windows_core::imp::define_interface!(IMFMediaKeySessionNotify2, IMFMediaKeySessionNotify2_Vtbl, 0xc3a9e92a_da88_46b0_a110_6cf953026cb9);
impl core::ops::Deref for IMFMediaKeySessionNotify2 {
    type Target = IMFMediaKeySessionNotify;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaKeySessionNotify2, windows_core::IUnknown, IMFMediaKeySessionNotify);
impl IMFMediaKeySessionNotify2 {
    #[cfg(feature = "mfidl")]
    pub unsafe fn KeyMessage2(&self, emessagetype: super::mfidl::MF_MEDIAKEYSESSION_MESSAGETYPE, destinationurl: &windows_core::BSTR, pbmessage: &[u8]) {
        unsafe {
            (windows_core::Interface::vtable(self).KeyMessage2)(windows_core::Interface::as_raw(self), emessagetype, core::mem::transmute_copy(destinationurl), pbmessage.as_ptr(), pbmessage.len().try_into().unwrap());
        }
    }
    pub unsafe fn KeyStatusChange(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).KeyStatusChange)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaKeySessionNotify2_Vtbl {
    pub base__: IMFMediaKeySessionNotify_Vtbl,
    #[cfg(feature = "mfidl")]
    pub KeyMessage2: unsafe extern "system" fn(*mut core::ffi::c_void, super::mfidl::MF_MEDIAKEYSESSION_MESSAGETYPE, *mut core::ffi::c_void, *const u8, u32),
    #[cfg(not(feature = "mfidl"))]
    KeyMessage2: usize,
    pub KeyStatusChange: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "mfidl")]
pub trait IMFMediaKeySessionNotify2_Impl: IMFMediaKeySessionNotify_Impl {
    fn KeyMessage2(&self, emessagetype: super::mfidl::MF_MEDIAKEYSESSION_MESSAGETYPE, destinationurl: &windows_core::BSTR, pbmessage: *const u8, cbmessage: u32);
    fn KeyStatusChange(&self);
}
#[cfg(feature = "mfidl")]
impl IMFMediaKeySessionNotify2_Vtbl {
    pub const fn new<Identity: IMFMediaKeySessionNotify2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn KeyMessage2<Identity: IMFMediaKeySessionNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emessagetype: super::mfidl::MF_MEDIAKEYSESSION_MESSAGETYPE, destinationurl: *mut core::ffi::c_void, pbmessage: *const u8, cbmessage: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySessionNotify2_Impl::KeyMessage2(this, core::mem::transmute_copy(&emessagetype), core::mem::transmute(&destinationurl), core::mem::transmute_copy(&pbmessage), core::mem::transmute_copy(&cbmessage));
            }
        }
        unsafe extern "system" fn KeyStatusChange<Identity: IMFMediaKeySessionNotify2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeySessionNotify2_Impl::KeyStatusChange(this);
            }
        }
        Self {
            base__: IMFMediaKeySessionNotify_Vtbl::new::<Identity, OFFSET>(),
            KeyMessage2: KeyMessage2::<Identity, OFFSET>,
            KeyStatusChange: KeyStatusChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaKeySessionNotify2 as windows_core::Interface>::IID || iid == &<IMFMediaKeySessionNotify as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfidl")]
impl windows_core::RuntimeName for IMFMediaKeySessionNotify2 {}
windows_core::imp::define_interface!(IMFMediaKeySystemAccess, IMFMediaKeySystemAccess_Vtbl, 0xaec63fda_7a97_4944_b35c_6c6df8085cc3);
windows_core::imp::interface_hierarchy!(IMFMediaKeySystemAccess, windows_core::IUnknown);
impl IMFMediaKeySystemAccess {
    #[cfg(feature = "propsys")]
    pub unsafe fn CreateMediaKeys<P0>(&self, pcdmcustomconfig: P0) -> windows_core::Result<IMFMediaKeys2>
    where
        P0: windows_core::Param<super::propsys::IPropertyStore>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMediaKeys)(windows_core::Interface::as_raw(self), pcdmcustomconfig.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "propsys")]
    pub unsafe fn get_SupportedConfiguration(&self) -> windows_core::Result<super::propsys::IPropertyStore> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedConfiguration)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_KeySystem(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeySystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaKeySystemAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "propsys")]
    pub CreateMediaKeys: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    CreateMediaKeys: usize,
    #[cfg(feature = "propsys")]
    pub get_SupportedConfiguration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    get_SupportedConfiguration: usize,
    pub get_KeySystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "propsys")]
pub trait IMFMediaKeySystemAccess_Impl: windows_core::IUnknownImpl {
    fn CreateMediaKeys(&self, pcdmcustomconfig: windows_core::Ref<super::propsys::IPropertyStore>) -> windows_core::Result<IMFMediaKeys2>;
    fn get_SupportedConfiguration(&self) -> windows_core::Result<super::propsys::IPropertyStore>;
    fn get_KeySystem(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "propsys")]
impl IMFMediaKeySystemAccess_Vtbl {
    pub const fn new<Identity: IMFMediaKeySystemAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMediaKeys<Identity: IMFMediaKeySystemAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdmcustomconfig: *mut core::ffi::c_void, ppkeys: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeySystemAccess_Impl::CreateMediaKeys(this, core::mem::transmute_copy(&pcdmcustomconfig)) {
                    Ok(ok__) => {
                        ppkeys.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedConfiguration<Identity: IMFMediaKeySystemAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsupportedconfiguration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeySystemAccess_Impl::get_SupportedConfiguration(this) {
                    Ok(ok__) => {
                        ppsupportedconfiguration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_KeySystem<Identity: IMFMediaKeySystemAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeysystem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeySystemAccess_Impl::get_KeySystem(this) {
                    Ok(ok__) => {
                        pkeysystem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMediaKeys: CreateMediaKeys::<Identity, OFFSET>,
            get_SupportedConfiguration: get_SupportedConfiguration::<Identity, OFFSET>,
            get_KeySystem: get_KeySystem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaKeySystemAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "propsys")]
impl windows_core::RuntimeName for IMFMediaKeySystemAccess {}
windows_core::imp::define_interface!(IMFMediaKeys, IMFMediaKeys_Vtbl, 0x5cb31c05_61ff_418f_afda_caaf41421a38);
windows_core::imp::interface_hierarchy!(IMFMediaKeys, windows_core::IUnknown);
impl IMFMediaKeys {
    pub unsafe fn CreateSession<P5>(&self, mimetype: &windows_core::BSTR, initdata: Option<&[u8]>, customdata: Option<&[u8]>, notify: P5) -> windows_core::Result<IMFMediaKeySession>
    where
        P5: windows_core::Param<IMFMediaKeySessionNotify>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSession)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(mimetype), initdata.map_or(core::ptr::null(), |slice| slice.as_ptr()), initdata.map_or(0, |slice| slice.len().try_into().unwrap()), customdata.map_or(core::ptr::null(), |slice| slice.as_ptr()), customdata.map_or(0, |slice| slice.len().try_into().unwrap()), notify.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_KeySystem(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeySystem)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSuspendNotify(&self) -> windows_core::Result<IMFCdmSuspendNotify> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSuspendNotify)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaKeys_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const u8, u32, *const u8, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_KeySystem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSuspendNotify: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaKeys_Impl: windows_core::IUnknownImpl {
    fn CreateSession(&self, mimetype: &windows_core::BSTR, initdata: *const u8, cb: u32, customdata: *const u8, cbcustomdata: u32, notify: windows_core::Ref<IMFMediaKeySessionNotify>) -> windows_core::Result<IMFMediaKeySession>;
    fn get_KeySystem(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn GetSuspendNotify(&self) -> windows_core::Result<IMFCdmSuspendNotify>;
}
impl IMFMediaKeys_Vtbl {
    pub const fn new<Identity: IMFMediaKeys_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSession<Identity: IMFMediaKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mimetype: *mut core::ffi::c_void, initdata: *const u8, cb: u32, customdata: *const u8, cbcustomdata: u32, notify: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeys_Impl::CreateSession(this, core::mem::transmute(&mimetype), core::mem::transmute_copy(&initdata), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&customdata), core::mem::transmute_copy(&cbcustomdata), core::mem::transmute_copy(&notify)) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_KeySystem<Identity: IMFMediaKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keysystem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeys_Impl::get_KeySystem(this) {
                    Ok(ok__) => {
                        keysystem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFMediaKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeys_Impl::Shutdown(this).into()
            }
        }
        unsafe extern "system" fn GetSuspendNotify<Identity: IMFMediaKeys_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notify: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeys_Impl::GetSuspendNotify(this) {
                    Ok(ok__) => {
                        notify.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSession: CreateSession::<Identity, OFFSET>,
            get_KeySystem: get_KeySystem::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            GetSuspendNotify: GetSuspendNotify::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaKeys as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaKeys {}
windows_core::imp::define_interface!(IMFMediaKeys2, IMFMediaKeys2_Vtbl, 0x45892507_ad66_4de2_83a2_acbb13cd8d43);
impl core::ops::Deref for IMFMediaKeys2 {
    type Target = IMFMediaKeys;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFMediaKeys2, windows_core::IUnknown, IMFMediaKeys);
impl IMFMediaKeys2 {
    #[cfg(feature = "mfidl")]
    pub unsafe fn CreateSession2<P1>(&self, esessiontype: super::mfidl::MF_MEDIAKEYSESSION_TYPE, pmfmediakeysessionnotify2: P1) -> windows_core::Result<IMFMediaKeySession2>
    where
        P1: windows_core::Param<IMFMediaKeySessionNotify2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateSession2)(windows_core::Interface::as_raw(self), esessiontype, pmfmediakeysessionnotify2.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetServerCertificate(&self, pbservercertificate: Option<&[u8]>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetServerCertificate)(windows_core::Interface::as_raw(self), pbservercertificate.map_or(core::ptr::null(), |slice| slice.as_ptr()), pbservercertificate.map_or(0, |slice| slice.len().try_into().unwrap())) }
    }
    pub unsafe fn GetDOMException(&self, systemcode: windows_core::HRESULT) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDOMException)(windows_core::Interface::as_raw(self), systemcode, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaKeys2_Vtbl {
    pub base__: IMFMediaKeys_Vtbl,
    #[cfg(feature = "mfidl")]
    pub CreateSession2: unsafe extern "system" fn(*mut core::ffi::c_void, super::mfidl::MF_MEDIAKEYSESSION_TYPE, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    CreateSession2: usize,
    pub SetServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub GetDOMException: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, *mut windows_core::HRESULT) -> windows_core::HRESULT,
}
#[cfg(feature = "mfidl")]
pub trait IMFMediaKeys2_Impl: IMFMediaKeys_Impl {
    fn CreateSession2(&self, esessiontype: super::mfidl::MF_MEDIAKEYSESSION_TYPE, pmfmediakeysessionnotify2: windows_core::Ref<IMFMediaKeySessionNotify2>) -> windows_core::Result<IMFMediaKeySession2>;
    fn SetServerCertificate(&self, pbservercertificate: *const u8, cb: u32) -> windows_core::Result<()>;
    fn GetDOMException(&self, systemcode: windows_core::HRESULT) -> windows_core::Result<windows_core::HRESULT>;
}
#[cfg(feature = "mfidl")]
impl IMFMediaKeys2_Vtbl {
    pub const fn new<Identity: IMFMediaKeys2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateSession2<Identity: IMFMediaKeys2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, esessiontype: super::mfidl::MF_MEDIAKEYSESSION_TYPE, pmfmediakeysessionnotify2: *mut core::ffi::c_void, ppsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeys2_Impl::CreateSession2(this, core::mem::transmute_copy(&esessiontype), core::mem::transmute_copy(&pmfmediakeysessionnotify2)) {
                    Ok(ok__) => {
                        ppsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServerCertificate<Identity: IMFMediaKeys2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbservercertificate: *const u8, cb: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaKeys2_Impl::SetServerCertificate(this, core::mem::transmute_copy(&pbservercertificate), core::mem::transmute_copy(&cb)).into()
            }
        }
        unsafe extern "system" fn GetDOMException<Identity: IMFMediaKeys2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, systemcode: windows_core::HRESULT, code: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaKeys2_Impl::GetDOMException(this, core::mem::transmute_copy(&systemcode)) {
                    Ok(ok__) => {
                        code.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMFMediaKeys_Vtbl::new::<Identity, OFFSET>(),
            CreateSession2: CreateSession2::<Identity, OFFSET>,
            SetServerCertificate: SetServerCertificate::<Identity, OFFSET>,
            GetDOMException: GetDOMException::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaKeys2 as windows_core::Interface>::IID || iid == &<IMFMediaKeys as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfidl")]
impl windows_core::RuntimeName for IMFMediaKeys2 {}
windows_core::imp::define_interface!(IMFMediaSourceExtension, IMFMediaSourceExtension_Vtbl, 0xe467b94e_a713_4562_a802_816a42e9008a);
windows_core::imp::interface_hierarchy!(IMFMediaSourceExtension, windows_core::IUnknown);
impl IMFMediaSourceExtension {
    pub unsafe fn GetSourceBuffers(&self) -> Option<IMFSourceBufferList> {
        unsafe { (windows_core::Interface::vtable(self).GetSourceBuffers)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetActiveSourceBuffers(&self) -> Option<IMFSourceBufferList> {
        unsafe { (windows_core::Interface::vtable(self).GetActiveSourceBuffers)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetReadyState(&self) -> MF_MSE_READY {
        unsafe { (windows_core::Interface::vtable(self).GetReadyState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDuration(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetDuration(&self, duration: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDuration)(windows_core::Interface::as_raw(self), duration) }
    }
    pub unsafe fn AddSourceBuffer<P1>(&self, r#type: &windows_core::BSTR, pnotify: P1) -> windows_core::Result<IMFSourceBuffer>
    where
        P1: windows_core::Param<IMFSourceBufferNotify>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddSourceBuffer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(r#type), pnotify.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveSourceBuffer<P0>(&self, psourcebuffer: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFSourceBuffer>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveSourceBuffer)(windows_core::Interface::as_raw(self), psourcebuffer.param().abi()) }
    }
    pub unsafe fn SetEndOfStream(&self, error: MF_MSE_ERROR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEndOfStream)(windows_core::Interface::as_raw(self), error) }
    }
    pub unsafe fn IsTypeSupported(&self, r#type: &windows_core::BSTR) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsTypeSupported)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(r#type)) }
    }
    pub unsafe fn GetSourceBuffer(&self, dwstreamindex: u32) -> Option<IMFSourceBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetSourceBuffer)(windows_core::Interface::as_raw(self), dwstreamindex) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSourceExtension_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSourceBuffers: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<IMFSourceBufferList>,
    pub GetActiveSourceBuffers: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<IMFSourceBufferList>,
    pub GetReadyState: unsafe extern "system" fn(*mut core::ffi::c_void) -> MF_MSE_READY,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetDuration: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub AddSourceBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveSourceBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEndOfStream: unsafe extern "system" fn(*mut core::ffi::c_void, MF_MSE_ERROR) -> windows_core::HRESULT,
    pub IsTypeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetSourceBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<IMFSourceBuffer>,
}
pub trait IMFMediaSourceExtension_Impl: windows_core::IUnknownImpl {
    fn GetSourceBuffers(&self) -> Option<IMFSourceBufferList>;
    fn GetActiveSourceBuffers(&self) -> Option<IMFSourceBufferList>;
    fn GetReadyState(&self) -> MF_MSE_READY;
    fn GetDuration(&self) -> f64;
    fn SetDuration(&self, duration: f64) -> windows_core::Result<()>;
    fn AddSourceBuffer(&self, r#type: &windows_core::BSTR, pnotify: windows_core::Ref<IMFSourceBufferNotify>) -> windows_core::Result<IMFSourceBuffer>;
    fn RemoveSourceBuffer(&self, psourcebuffer: windows_core::Ref<IMFSourceBuffer>) -> windows_core::Result<()>;
    fn SetEndOfStream(&self, error: MF_MSE_ERROR) -> windows_core::Result<()>;
    fn IsTypeSupported(&self, r#type: &windows_core::BSTR) -> windows_core::BOOL;
    fn GetSourceBuffer(&self, dwstreamindex: u32) -> Option<IMFSourceBuffer>;
}
impl IMFMediaSourceExtension_Vtbl {
    pub const fn new<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSourceBuffers<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<IMFSourceBufferList> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtension_Impl::GetSourceBuffers(this)
            }
        }
        unsafe extern "system" fn GetActiveSourceBuffers<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<IMFSourceBufferList> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtension_Impl::GetActiveSourceBuffers(this)
            }
        }
        unsafe extern "system" fn GetReadyState<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MF_MSE_READY {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtension_Impl::GetReadyState(this)
            }
        }
        unsafe extern "system" fn GetDuration<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtension_Impl::GetDuration(this)
            }
        }
        unsafe extern "system" fn SetDuration<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, duration: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtension_Impl::SetDuration(this, core::mem::transmute_copy(&duration)).into()
            }
        }
        unsafe extern "system" fn AddSourceBuffer<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void, ppsourcebuffer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSourceExtension_Impl::AddSourceBuffer(this, core::mem::transmute(&r#type), core::mem::transmute_copy(&pnotify)) {
                    Ok(ok__) => {
                        ppsourcebuffer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveSourceBuffer<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcebuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtension_Impl::RemoveSourceBuffer(this, core::mem::transmute_copy(&psourcebuffer)).into()
            }
        }
        unsafe extern "system" fn SetEndOfStream<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, error: MF_MSE_ERROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtension_Impl::SetEndOfStream(this, core::mem::transmute_copy(&error)).into()
            }
        }
        unsafe extern "system" fn IsTypeSupported<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtension_Impl::IsTypeSupported(this, core::mem::transmute(&r#type))
            }
        }
        unsafe extern "system" fn GetSourceBuffer<Identity: IMFMediaSourceExtension_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32) -> Option<IMFSourceBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtension_Impl::GetSourceBuffer(this, core::mem::transmute_copy(&dwstreamindex))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSourceBuffers: GetSourceBuffers::<Identity, OFFSET>,
            GetActiveSourceBuffers: GetActiveSourceBuffers::<Identity, OFFSET>,
            GetReadyState: GetReadyState::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            SetDuration: SetDuration::<Identity, OFFSET>,
            AddSourceBuffer: AddSourceBuffer::<Identity, OFFSET>,
            RemoveSourceBuffer: RemoveSourceBuffer::<Identity, OFFSET>,
            SetEndOfStream: SetEndOfStream::<Identity, OFFSET>,
            IsTypeSupported: IsTypeSupported::<Identity, OFFSET>,
            GetSourceBuffer: GetSourceBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSourceExtension as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaSourceExtension {}
windows_core::imp::define_interface!(IMFMediaSourceExtensionLiveSeekableRange, IMFMediaSourceExtensionLiveSeekableRange_Vtbl, 0x5d1abfd6_450a_4d92_9efc_d6b6cbc1f4da);
windows_core::imp::interface_hierarchy!(IMFMediaSourceExtensionLiveSeekableRange, windows_core::IUnknown);
impl IMFMediaSourceExtensionLiveSeekableRange {
    pub unsafe fn SetLiveSeekableRange(&self, start: f64, end: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLiveSeekableRange)(windows_core::Interface::as_raw(self), start, end) }
    }
    pub unsafe fn ClearLiveSeekableRange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearLiveSeekableRange)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSourceExtensionLiveSeekableRange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLiveSeekableRange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub ClearLiveSeekableRange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaSourceExtensionLiveSeekableRange_Impl: windows_core::IUnknownImpl {
    fn SetLiveSeekableRange(&self, start: f64, end: f64) -> windows_core::Result<()>;
    fn ClearLiveSeekableRange(&self) -> windows_core::Result<()>;
}
impl IMFMediaSourceExtensionLiveSeekableRange_Vtbl {
    pub const fn new<Identity: IMFMediaSourceExtensionLiveSeekableRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLiveSeekableRange<Identity: IMFMediaSourceExtensionLiveSeekableRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: f64, end: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtensionLiveSeekableRange_Impl::SetLiveSeekableRange(this, core::mem::transmute_copy(&start), core::mem::transmute_copy(&end)).into()
            }
        }
        unsafe extern "system" fn ClearLiveSeekableRange<Identity: IMFMediaSourceExtensionLiveSeekableRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtensionLiveSeekableRange_Impl::ClearLiveSeekableRange(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLiveSeekableRange: SetLiveSeekableRange::<Identity, OFFSET>,
            ClearLiveSeekableRange: ClearLiveSeekableRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSourceExtensionLiveSeekableRange as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaSourceExtensionLiveSeekableRange {}
windows_core::imp::define_interface!(IMFMediaSourceExtensionNotify, IMFMediaSourceExtensionNotify_Vtbl, 0xa7901327_05dd_4469_a7b7_0e01979e361d);
windows_core::imp::interface_hierarchy!(IMFMediaSourceExtensionNotify, windows_core::IUnknown);
impl IMFMediaSourceExtensionNotify {
    pub unsafe fn OnSourceOpen(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnSourceOpen)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnSourceEnded(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnSourceEnded)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnSourceClose(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnSourceClose)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSourceExtensionNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSourceOpen: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnSourceEnded: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnSourceClose: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMFMediaSourceExtensionNotify_Impl: windows_core::IUnknownImpl {
    fn OnSourceOpen(&self);
    fn OnSourceEnded(&self);
    fn OnSourceClose(&self);
}
impl IMFMediaSourceExtensionNotify_Vtbl {
    pub const fn new<Identity: IMFMediaSourceExtensionNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSourceOpen<Identity: IMFMediaSourceExtensionNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtensionNotify_Impl::OnSourceOpen(this);
            }
        }
        unsafe extern "system" fn OnSourceEnded<Identity: IMFMediaSourceExtensionNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtensionNotify_Impl::OnSourceEnded(this);
            }
        }
        unsafe extern "system" fn OnSourceClose<Identity: IMFMediaSourceExtensionNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceExtensionNotify_Impl::OnSourceClose(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnSourceOpen: OnSourceOpen::<Identity, OFFSET>,
            OnSourceEnded: OnSourceEnded::<Identity, OFFSET>,
            OnSourceClose: OnSourceClose::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSourceExtensionNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaSourceExtensionNotify {}
windows_core::imp::define_interface!(IMFMediaTimeRange, IMFMediaTimeRange_Vtbl, 0xdb71a2fc_078a_414e_9df9_8c2531b0aa6c);
windows_core::imp::interface_hierarchy!(IMFMediaTimeRange, windows_core::IUnknown);
impl IMFMediaTimeRange {
    pub unsafe fn GetLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStart(&self, index: u32) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStart)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnd(&self, index: u32) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnd)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ContainsTime(&self, time: f64) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).ContainsTime)(windows_core::Interface::as_raw(self), time) }
    }
    pub unsafe fn AddRange(&self, starttime: f64, endtime: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddRange)(windows_core::Interface::as_raw(self), starttime, endtime) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaTimeRange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetStart: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f64) -> windows_core::HRESULT,
    pub GetEnd: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f64) -> windows_core::HRESULT,
    pub ContainsTime: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::BOOL,
    pub AddRange: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFMediaTimeRange_Impl: windows_core::IUnknownImpl {
    fn GetLength(&self) -> u32;
    fn GetStart(&self, index: u32) -> windows_core::Result<f64>;
    fn GetEnd(&self, index: u32) -> windows_core::Result<f64>;
    fn ContainsTime(&self, time: f64) -> windows_core::BOOL;
    fn AddRange(&self, starttime: f64, endtime: f64) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl IMFMediaTimeRange_Vtbl {
    pub const fn new<Identity: IMFMediaTimeRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLength<Identity: IMFMediaTimeRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaTimeRange_Impl::GetLength(this)
            }
        }
        unsafe extern "system" fn GetStart<Identity: IMFMediaTimeRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pstart: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaTimeRange_Impl::GetStart(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pstart.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnd<Identity: IMFMediaTimeRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pend: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaTimeRange_Impl::GetEnd(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        pend.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContainsTime<Identity: IMFMediaTimeRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: f64) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaTimeRange_Impl::ContainsTime(this, core::mem::transmute_copy(&time))
            }
        }
        unsafe extern "system" fn AddRange<Identity: IMFMediaTimeRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: f64, endtime: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaTimeRange_Impl::AddRange(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&endtime)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: IMFMediaTimeRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaTimeRange_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            GetStart: GetStart::<Identity, OFFSET>,
            GetEnd: GetEnd::<Identity, OFFSET>,
            ContainsTime: ContainsTime::<Identity, OFFSET>,
            AddRange: AddRange::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaTimeRange as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaTimeRange {}
windows_core::imp::define_interface!(IMFSourceBuffer, IMFSourceBuffer_Vtbl, 0xe2cd3a4b_af25_4d3d_9110_da0e6f8ee877);
windows_core::imp::interface_hierarchy!(IMFSourceBuffer, windows_core::IUnknown);
impl IMFSourceBuffer {
    pub unsafe fn GetUpdating(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).GetUpdating)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetBuffered(&self) -> windows_core::Result<IMFMediaTimeRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBuffered)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTimeStampOffset(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetTimeStampOffset)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetTimeStampOffset(&self, offset: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTimeStampOffset)(windows_core::Interface::as_raw(self), offset) }
    }
    pub unsafe fn GetAppendWindowStart(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetAppendWindowStart)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAppendWindowStart(&self, time: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAppendWindowStart)(windows_core::Interface::as_raw(self), time) }
    }
    pub unsafe fn GetAppendWindowEnd(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetAppendWindowEnd)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAppendWindowEnd(&self, time: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAppendWindowEnd)(windows_core::Interface::as_raw(self), time) }
    }
    pub unsafe fn Append(&self, pdata: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), pdata.as_ptr(), pdata.len().try_into().unwrap()) }
    }
    #[cfg(all(feature = "mfobjects", feature = "winnt"))]
    pub unsafe fn AppendByteStream<P0>(&self, pstream: P0, pmaxlen: Option<*const super::winnt::DWORDLONG>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::mfobjects::IMFByteStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AppendByteStream)(windows_core::Interface::as_raw(self), pstream.param().abi(), pmaxlen.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Remove(&self, start: f64, end: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), start, end) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceBuffer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUpdating: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetBuffered: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTimeStampOffset: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetTimeStampOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetAppendWindowStart: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetAppendWindowStart: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetAppendWindowEnd: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub SetAppendWindowEnd: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "mfobjects", feature = "winnt"))]
    pub AppendByteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::winnt::DWORDLONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "winnt")))]
    AppendByteStream: usize,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "winnt"))]
pub trait IMFSourceBuffer_Impl: windows_core::IUnknownImpl {
    fn GetUpdating(&self) -> windows_core::BOOL;
    fn GetBuffered(&self) -> windows_core::Result<IMFMediaTimeRange>;
    fn GetTimeStampOffset(&self) -> f64;
    fn SetTimeStampOffset(&self, offset: f64) -> windows_core::Result<()>;
    fn GetAppendWindowStart(&self) -> f64;
    fn SetAppendWindowStart(&self, time: f64) -> windows_core::Result<()>;
    fn GetAppendWindowEnd(&self) -> f64;
    fn SetAppendWindowEnd(&self, time: f64) -> windows_core::Result<()>;
    fn Append(&self, pdata: *const u8, len: u32) -> windows_core::Result<()>;
    fn AppendByteStream(&self, pstream: windows_core::Ref<super::mfobjects::IMFByteStream>, pmaxlen: *const super::winnt::DWORDLONG) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn Remove(&self, start: f64, end: f64) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "winnt"))]
impl IMFSourceBuffer_Vtbl {
    pub const fn new<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUpdating<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::GetUpdating(this)
            }
        }
        unsafe extern "system" fn GetBuffered<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbuffered: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSourceBuffer_Impl::GetBuffered(this) {
                    Ok(ok__) => {
                        ppbuffered.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTimeStampOffset<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::GetTimeStampOffset(this)
            }
        }
        unsafe extern "system" fn SetTimeStampOffset<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::SetTimeStampOffset(this, core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn GetAppendWindowStart<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::GetAppendWindowStart(this)
            }
        }
        unsafe extern "system" fn SetAppendWindowStart<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::SetAppendWindowStart(this, core::mem::transmute_copy(&time)).into()
            }
        }
        unsafe extern "system" fn GetAppendWindowEnd<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::GetAppendWindowEnd(this)
            }
        }
        unsafe extern "system" fn SetAppendWindowEnd<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, time: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::SetAppendWindowEnd(this, core::mem::transmute_copy(&time)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdata: *const u8, len: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::Append(this, core::mem::transmute_copy(&pdata), core::mem::transmute_copy(&len)).into()
            }
        }
        unsafe extern "system" fn AppendByteStream<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pmaxlen: *const super::winnt::DWORDLONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::AppendByteStream(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&pmaxlen)).into()
            }
        }
        unsafe extern "system" fn Abort<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IMFSourceBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: f64, end: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBuffer_Impl::Remove(this, core::mem::transmute_copy(&start), core::mem::transmute_copy(&end)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetUpdating: GetUpdating::<Identity, OFFSET>,
            GetBuffered: GetBuffered::<Identity, OFFSET>,
            GetTimeStampOffset: GetTimeStampOffset::<Identity, OFFSET>,
            SetTimeStampOffset: SetTimeStampOffset::<Identity, OFFSET>,
            GetAppendWindowStart: GetAppendWindowStart::<Identity, OFFSET>,
            SetAppendWindowStart: SetAppendWindowStart::<Identity, OFFSET>,
            GetAppendWindowEnd: GetAppendWindowEnd::<Identity, OFFSET>,
            SetAppendWindowEnd: SetAppendWindowEnd::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            AppendByteStream: AppendByteStream::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceBuffer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "winnt"))]
impl windows_core::RuntimeName for IMFSourceBuffer {}
windows_core::imp::define_interface!(IMFSourceBufferAppendMode, IMFSourceBufferAppendMode_Vtbl, 0x19666fb4_babe_4c55_bc03_0a074da37e2a);
windows_core::imp::interface_hierarchy!(IMFSourceBufferAppendMode, windows_core::IUnknown);
impl IMFSourceBufferAppendMode {
    pub unsafe fn GetAppendMode(&self) -> MF_MSE_APPEND_MODE {
        unsafe { (windows_core::Interface::vtable(self).GetAppendMode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetAppendMode(&self, mode: MF_MSE_APPEND_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAppendMode)(windows_core::Interface::as_raw(self), mode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceBufferAppendMode_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAppendMode: unsafe extern "system" fn(*mut core::ffi::c_void) -> MF_MSE_APPEND_MODE,
    pub SetAppendMode: unsafe extern "system" fn(*mut core::ffi::c_void, MF_MSE_APPEND_MODE) -> windows_core::HRESULT,
}
pub trait IMFSourceBufferAppendMode_Impl: windows_core::IUnknownImpl {
    fn GetAppendMode(&self) -> MF_MSE_APPEND_MODE;
    fn SetAppendMode(&self, mode: MF_MSE_APPEND_MODE) -> windows_core::Result<()>;
}
impl IMFSourceBufferAppendMode_Vtbl {
    pub const fn new<Identity: IMFSourceBufferAppendMode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAppendMode<Identity: IMFSourceBufferAppendMode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MF_MSE_APPEND_MODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBufferAppendMode_Impl::GetAppendMode(this)
            }
        }
        unsafe extern "system" fn SetAppendMode<Identity: IMFSourceBufferAppendMode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: MF_MSE_APPEND_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBufferAppendMode_Impl::SetAppendMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAppendMode: GetAppendMode::<Identity, OFFSET>,
            SetAppendMode: SetAppendMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceBufferAppendMode as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSourceBufferAppendMode {}
windows_core::imp::define_interface!(IMFSourceBufferList, IMFSourceBufferList_Vtbl, 0x249981f8_8325_41f3_b80c_3b9e3aad0cbe);
windows_core::imp::interface_hierarchy!(IMFSourceBufferList, windows_core::IUnknown);
impl IMFSourceBufferList {
    pub unsafe fn GetLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSourceBuffer(&self, index: u32) -> Option<IMFSourceBuffer> {
        unsafe { (windows_core::Interface::vtable(self).GetSourceBuffer)(windows_core::Interface::as_raw(self), index) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceBufferList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSourceBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> Option<IMFSourceBuffer>,
}
pub trait IMFSourceBufferList_Impl: windows_core::IUnknownImpl {
    fn GetLength(&self) -> u32;
    fn GetSourceBuffer(&self, index: u32) -> Option<IMFSourceBuffer>;
}
impl IMFSourceBufferList_Vtbl {
    pub const fn new<Identity: IMFSourceBufferList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLength<Identity: IMFSourceBufferList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBufferList_Impl::GetLength(this)
            }
        }
        unsafe extern "system" fn GetSourceBuffer<Identity: IMFSourceBufferList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> Option<IMFSourceBuffer> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBufferList_Impl::GetSourceBuffer(this, core::mem::transmute_copy(&index))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            GetSourceBuffer: GetSourceBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceBufferList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSourceBufferList {}
windows_core::imp::define_interface!(IMFSourceBufferNotify, IMFSourceBufferNotify_Vtbl, 0x87e47623_2ceb_45d6_9b88_d8520c4dcbbc);
windows_core::imp::interface_hierarchy!(IMFSourceBufferNotify, windows_core::IUnknown);
impl IMFSourceBufferNotify {
    pub unsafe fn OnUpdateStart(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnUpdateStart)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnAbort(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnAbort)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnError(&self, hr: windows_core::HRESULT) {
        unsafe {
            (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hr);
        }
    }
    pub unsafe fn OnUpdate(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnUpdate)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn OnUpdateEnd(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).OnUpdateEnd)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceBufferNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnUpdateStart: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnAbort: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT),
    pub OnUpdate: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub OnUpdateEnd: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMFSourceBufferNotify_Impl: windows_core::IUnknownImpl {
    fn OnUpdateStart(&self);
    fn OnAbort(&self);
    fn OnError(&self, hr: windows_core::HRESULT);
    fn OnUpdate(&self);
    fn OnUpdateEnd(&self);
}
impl IMFSourceBufferNotify_Vtbl {
    pub const fn new<Identity: IMFSourceBufferNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnUpdateStart<Identity: IMFSourceBufferNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBufferNotify_Impl::OnUpdateStart(this);
            }
        }
        unsafe extern "system" fn OnAbort<Identity: IMFSourceBufferNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBufferNotify_Impl::OnAbort(this);
            }
        }
        unsafe extern "system" fn OnError<Identity: IMFSourceBufferNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBufferNotify_Impl::OnError(this, core::mem::transmute_copy(&hr));
            }
        }
        unsafe extern "system" fn OnUpdate<Identity: IMFSourceBufferNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBufferNotify_Impl::OnUpdate(this);
            }
        }
        unsafe extern "system" fn OnUpdateEnd<Identity: IMFSourceBufferNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceBufferNotify_Impl::OnUpdateEnd(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnUpdateStart: OnUpdateStart::<Identity, OFFSET>,
            OnAbort: OnAbort::<Identity, OFFSET>,
            OnError: OnError::<Identity, OFFSET>,
            OnUpdate: OnUpdate::<Identity, OFFSET>,
            OnUpdateEnd: OnUpdateEnd::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceBufferNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSourceBufferNotify {}
windows_core::imp::define_interface!(IMFTimedText, IMFTimedText_Vtbl, 0x1f2a94c9_a3df_430d_9d0f_acd85ddc29af);
windows_core::imp::interface_hierarchy!(IMFTimedText, windows_core::IUnknown);
impl IMFTimedText {
    pub unsafe fn RegisterNotifications<P0>(&self, notify: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTimedTextNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterNotifications)(windows_core::Interface::as_raw(self), notify.param().abi()) }
    }
    pub unsafe fn SelectTrack(&self, trackid: u32, selected: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SelectTrack)(windows_core::Interface::as_raw(self), trackid, selected.into()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn AddDataSource<P0, P1, P2>(&self, bytestream: P0, label: P1, language: P2, kind: MF_TIMED_TEXT_TRACK_KIND, isdefault: bool) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::mfobjects::IMFByteStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddDataSource)(windows_core::Interface::as_raw(self), bytestream.param().abi(), label.param().abi(), language.param().abi(), kind, isdefault.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddDataSourceFromUrl<P0, P1, P2>(&self, url: P0, label: P1, language: P2, kind: MF_TIMED_TEXT_TRACK_KIND, isdefault: bool) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddDataSourceFromUrl)(windows_core::Interface::as_raw(self), url.param().abi(), label.param().abi(), language.param().abi(), kind, isdefault.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddTrack<P0, P1>(&self, label: P0, language: P1, kind: MF_TIMED_TEXT_TRACK_KIND) -> windows_core::Result<IMFTimedTextTrack>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddTrack)(windows_core::Interface::as_raw(self), label.param().abi(), language.param().abi(), kind, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveTrack<P0>(&self, track: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTimedTextTrack>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveTrack)(windows_core::Interface::as_raw(self), track.param().abi()) }
    }
    pub unsafe fn GetCueTimeOffset(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCueTimeOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCueTimeOffset(&self, offset: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCueTimeOffset)(windows_core::Interface::as_raw(self), offset) }
    }
    pub unsafe fn GetTracks(&self) -> windows_core::Result<IMFTimedTextTrackList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTracks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetActiveTracks(&self) -> windows_core::Result<IMFTimedTextTrackList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveTracks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTextTracks(&self) -> windows_core::Result<IMFTimedTextTrackList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextTracks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMetadataTracks(&self) -> windows_core::Result<IMFTimedTextTrackList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataTracks)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetInBandEnabled(&self, enabled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInBandEnabled)(windows_core::Interface::as_raw(self), enabled.into()) }
    }
    pub unsafe fn IsInBandEnabled(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsInBandEnabled)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedText_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterNotifications: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectTrack: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub AddDataSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, MF_TIMED_TEXT_TRACK_KIND, windows_core::BOOL, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    AddDataSource: usize,
    pub AddDataSourceFromUrl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, MF_TIMED_TEXT_TRACK_KIND, windows_core::BOOL, *mut u32) -> windows_core::HRESULT,
    pub AddTrack: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, MF_TIMED_TEXT_TRACK_KIND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveTrack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCueTimeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SetCueTimeOffset: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub GetTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActiveTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTextTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMetadataTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInBandEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub IsInBandEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTimedText_Impl: windows_core::IUnknownImpl {
    fn RegisterNotifications(&self, notify: windows_core::Ref<IMFTimedTextNotify>) -> windows_core::Result<()>;
    fn SelectTrack(&self, trackid: u32, selected: windows_core::BOOL) -> windows_core::Result<()>;
    fn AddDataSource(&self, bytestream: windows_core::Ref<super::mfobjects::IMFByteStream>, label: &windows_core::PCWSTR, language: &windows_core::PCWSTR, kind: MF_TIMED_TEXT_TRACK_KIND, isdefault: windows_core::BOOL) -> windows_core::Result<u32>;
    fn AddDataSourceFromUrl(&self, url: &windows_core::PCWSTR, label: &windows_core::PCWSTR, language: &windows_core::PCWSTR, kind: MF_TIMED_TEXT_TRACK_KIND, isdefault: windows_core::BOOL) -> windows_core::Result<u32>;
    fn AddTrack(&self, label: &windows_core::PCWSTR, language: &windows_core::PCWSTR, kind: MF_TIMED_TEXT_TRACK_KIND) -> windows_core::Result<IMFTimedTextTrack>;
    fn RemoveTrack(&self, track: windows_core::Ref<IMFTimedTextTrack>) -> windows_core::Result<()>;
    fn GetCueTimeOffset(&self) -> windows_core::Result<f64>;
    fn SetCueTimeOffset(&self, offset: f64) -> windows_core::Result<()>;
    fn GetTracks(&self) -> windows_core::Result<IMFTimedTextTrackList>;
    fn GetActiveTracks(&self) -> windows_core::Result<IMFTimedTextTrackList>;
    fn GetTextTracks(&self) -> windows_core::Result<IMFTimedTextTrackList>;
    fn GetMetadataTracks(&self) -> windows_core::Result<IMFTimedTextTrackList>;
    fn SetInBandEnabled(&self, enabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn IsInBandEnabled(&self) -> windows_core::BOOL;
}
#[cfg(feature = "mfobjects")]
impl IMFTimedText_Vtbl {
    pub const fn new<Identity: IMFTimedText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterNotifications<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedText_Impl::RegisterNotifications(this, core::mem::transmute_copy(&notify)).into()
            }
        }
        unsafe extern "system" fn SelectTrack<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackid: u32, selected: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedText_Impl::SelectTrack(this, core::mem::transmute_copy(&trackid), core::mem::transmute_copy(&selected)).into()
            }
        }
        unsafe extern "system" fn AddDataSource<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bytestream: *mut core::ffi::c_void, label: windows_core::PCWSTR, language: windows_core::PCWSTR, kind: MF_TIMED_TEXT_TRACK_KIND, isdefault: windows_core::BOOL, trackid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedText_Impl::AddDataSource(this, core::mem::transmute_copy(&bytestream), core::mem::transmute(&label), core::mem::transmute(&language), core::mem::transmute_copy(&kind), core::mem::transmute_copy(&isdefault)) {
                    Ok(ok__) => {
                        trackid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDataSourceFromUrl<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, url: windows_core::PCWSTR, label: windows_core::PCWSTR, language: windows_core::PCWSTR, kind: MF_TIMED_TEXT_TRACK_KIND, isdefault: windows_core::BOOL, trackid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedText_Impl::AddDataSourceFromUrl(this, core::mem::transmute(&url), core::mem::transmute(&label), core::mem::transmute(&language), core::mem::transmute_copy(&kind), core::mem::transmute_copy(&isdefault)) {
                    Ok(ok__) => {
                        trackid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTrack<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, label: windows_core::PCWSTR, language: windows_core::PCWSTR, kind: MF_TIMED_TEXT_TRACK_KIND, track: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedText_Impl::AddTrack(this, core::mem::transmute(&label), core::mem::transmute(&language), core::mem::transmute_copy(&kind)) {
                    Ok(ok__) => {
                        track.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveTrack<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, track: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedText_Impl::RemoveTrack(this, core::mem::transmute_copy(&track)).into()
            }
        }
        unsafe extern "system" fn GetCueTimeOffset<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedText_Impl::GetCueTimeOffset(this) {
                    Ok(ok__) => {
                        offset.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCueTimeOffset<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedText_Impl::SetCueTimeOffset(this, core::mem::transmute_copy(&offset)).into()
            }
        }
        unsafe extern "system" fn GetTracks<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, tracks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedText_Impl::GetTracks(this) {
                    Ok(ok__) => {
                        tracks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActiveTracks<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, activetracks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedText_Impl::GetActiveTracks(this) {
                    Ok(ok__) => {
                        activetracks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextTracks<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, texttracks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedText_Impl::GetTextTracks(this) {
                    Ok(ok__) => {
                        texttracks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMetadataTracks<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadatatracks: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedText_Impl::GetMetadataTracks(this) {
                    Ok(ok__) => {
                        metadatatracks.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInBandEnabled<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedText_Impl::SetInBandEnabled(this, core::mem::transmute_copy(&enabled)).into()
            }
        }
        unsafe extern "system" fn IsInBandEnabled<Identity: IMFTimedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedText_Impl::IsInBandEnabled(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterNotifications: RegisterNotifications::<Identity, OFFSET>,
            SelectTrack: SelectTrack::<Identity, OFFSET>,
            AddDataSource: AddDataSource::<Identity, OFFSET>,
            AddDataSourceFromUrl: AddDataSourceFromUrl::<Identity, OFFSET>,
            AddTrack: AddTrack::<Identity, OFFSET>,
            RemoveTrack: RemoveTrack::<Identity, OFFSET>,
            GetCueTimeOffset: GetCueTimeOffset::<Identity, OFFSET>,
            SetCueTimeOffset: SetCueTimeOffset::<Identity, OFFSET>,
            GetTracks: GetTracks::<Identity, OFFSET>,
            GetActiveTracks: GetActiveTracks::<Identity, OFFSET>,
            GetTextTracks: GetTextTracks::<Identity, OFFSET>,
            GetMetadataTracks: GetMetadataTracks::<Identity, OFFSET>,
            SetInBandEnabled: SetInBandEnabled::<Identity, OFFSET>,
            IsInBandEnabled: IsInBandEnabled::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedText as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTimedText {}
windows_core::imp::define_interface!(IMFTimedTextBinary, IMFTimedTextBinary_Vtbl, 0x4ae3a412_0545_43c4_bf6f_6b97a5c6c432);
windows_core::imp::interface_hierarchy!(IMFTimedTextBinary, windows_core::IUnknown);
impl IMFTimedTextBinary {
    pub unsafe fn GetData(&self, data: *mut *mut u8, length: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), data as _, length as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextBinary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
}
pub trait IMFTimedTextBinary_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, data: *mut *mut u8, length: *mut u32) -> windows_core::Result<()>;
}
impl IMFTimedTextBinary_Vtbl {
    pub const fn new<Identity: IMFTimedTextBinary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: IMFTimedTextBinary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, length: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextBinary_Impl::GetData(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&length)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextBinary as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTimedTextBinary {}
windows_core::imp::define_interface!(IMFTimedTextBouten, IMFTimedTextBouten_Vtbl, 0x3c5f3e8a_90c0_464e_8136_898d2975f847);
windows_core::imp::interface_hierarchy!(IMFTimedTextBouten, windows_core::IUnknown);
impl IMFTimedTextBouten {
    pub unsafe fn GetBoutenType(&self) -> windows_core::Result<MF_TIMED_TEXT_BOUTEN_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoutenType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetBoutenColor(&self) -> windows_core::Result<super::mfobjects::MFARGB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoutenColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBoutenPosition(&self) -> windows_core::Result<MF_TIMED_TEXT_BOUTEN_POSITION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoutenPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextBouten_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBoutenType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_BOUTEN_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetBoutenColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mfobjects::MFARGB) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetBoutenColor: usize,
    pub GetBoutenPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_BOUTEN_POSITION) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTimedTextBouten_Impl: windows_core::IUnknownImpl {
    fn GetBoutenType(&self) -> windows_core::Result<MF_TIMED_TEXT_BOUTEN_TYPE>;
    fn GetBoutenColor(&self) -> windows_core::Result<super::mfobjects::MFARGB>;
    fn GetBoutenPosition(&self) -> windows_core::Result<MF_TIMED_TEXT_BOUTEN_POSITION>;
}
#[cfg(feature = "mfobjects")]
impl IMFTimedTextBouten_Vtbl {
    pub const fn new<Identity: IMFTimedTextBouten_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBoutenType<Identity: IMFTimedTextBouten_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut MF_TIMED_TEXT_BOUTEN_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextBouten_Impl::GetBoutenType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBoutenColor<Identity: IMFTimedTextBouten_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::mfobjects::MFARGB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextBouten_Impl::GetBoutenColor(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBoutenPosition<Identity: IMFTimedTextBouten_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut MF_TIMED_TEXT_BOUTEN_POSITION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextBouten_Impl::GetBoutenPosition(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBoutenType: GetBoutenType::<Identity, OFFSET>,
            GetBoutenColor: GetBoutenColor::<Identity, OFFSET>,
            GetBoutenPosition: GetBoutenPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextBouten as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTimedTextBouten {}
windows_core::imp::define_interface!(IMFTimedTextCue, IMFTimedTextCue_Vtbl, 0x1e560447_9a2b_43e1_a94c_b0aaabfbfbc9);
windows_core::imp::interface_hierarchy!(IMFTimedTextCue, windows_core::IUnknown);
impl IMFTimedTextCue {
    pub unsafe fn GetId(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetOriginalId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOriginalId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCueKind(&self) -> MF_TIMED_TEXT_TRACK_KIND {
        unsafe { (windows_core::Interface::vtable(self).GetCueKind)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetStartTime(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetStartTime)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDuration(&self) -> f64 {
        unsafe { (windows_core::Interface::vtable(self).GetDuration)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTrackId(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetTrackId)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetData(&self) -> windows_core::Result<IMFTimedTextBinary> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRegion(&self) -> windows_core::Result<IMFTimedTextRegion> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRegion)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStyle(&self) -> windows_core::Result<IMFTimedTextStyle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStyle)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLineCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLineCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLine(&self, index: u32) -> windows_core::Result<IMFTimedTextFormattedText> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLine)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextCue_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetOriginalId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetCueKind: unsafe extern "system" fn(*mut core::ffi::c_void) -> MF_TIMED_TEXT_TRACK_KIND,
    pub GetStartTime: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub GetDuration: unsafe extern "system" fn(*mut core::ffi::c_void) -> f64,
    pub GetTrackId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRegion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLineCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLine: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFTimedTextCue_Impl: windows_core::IUnknownImpl {
    fn GetId(&self) -> u32;
    fn GetOriginalId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetCueKind(&self) -> MF_TIMED_TEXT_TRACK_KIND;
    fn GetStartTime(&self) -> f64;
    fn GetDuration(&self) -> f64;
    fn GetTrackId(&self) -> u32;
    fn GetData(&self) -> windows_core::Result<IMFTimedTextBinary>;
    fn GetRegion(&self) -> windows_core::Result<IMFTimedTextRegion>;
    fn GetStyle(&self) -> windows_core::Result<IMFTimedTextStyle>;
    fn GetLineCount(&self) -> u32;
    fn GetLine(&self, index: u32) -> windows_core::Result<IMFTimedTextFormattedText>;
}
impl IMFTimedTextCue_Vtbl {
    pub const fn new<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetId<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCue_Impl::GetId(this)
            }
        }
        unsafe extern "system" fn GetOriginalId<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, originalid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextCue_Impl::GetOriginalId(this) {
                    Ok(ok__) => {
                        originalid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCueKind<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MF_TIMED_TEXT_TRACK_KIND {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCue_Impl::GetCueKind(this)
            }
        }
        unsafe extern "system" fn GetStartTime<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCue_Impl::GetStartTime(this)
            }
        }
        unsafe extern "system" fn GetDuration<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> f64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCue_Impl::GetDuration(this)
            }
        }
        unsafe extern "system" fn GetTrackId<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCue_Impl::GetTrackId(this)
            }
        }
        unsafe extern "system" fn GetData<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextCue_Impl::GetData(this) {
                    Ok(ok__) => {
                        data.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRegion<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, region: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextCue_Impl::GetRegion(this) {
                    Ok(ok__) => {
                        region.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStyle<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, style: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextCue_Impl::GetStyle(this) {
                    Ok(ok__) => {
                        style.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLineCount<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCue_Impl::GetLineCount(this)
            }
        }
        unsafe extern "system" fn GetLine<Identity: IMFTimedTextCue_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, line: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextCue_Impl::GetLine(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        line.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            GetOriginalId: GetOriginalId::<Identity, OFFSET>,
            GetCueKind: GetCueKind::<Identity, OFFSET>,
            GetStartTime: GetStartTime::<Identity, OFFSET>,
            GetDuration: GetDuration::<Identity, OFFSET>,
            GetTrackId: GetTrackId::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            GetRegion: GetRegion::<Identity, OFFSET>,
            GetStyle: GetStyle::<Identity, OFFSET>,
            GetLineCount: GetLineCount::<Identity, OFFSET>,
            GetLine: GetLine::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextCue as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTimedTextCue {}
windows_core::imp::define_interface!(IMFTimedTextCueList, IMFTimedTextCueList_Vtbl, 0xad128745_211b_40a0_9981_fe65f166d0fd);
windows_core::imp::interface_hierarchy!(IMFTimedTextCueList, windows_core::IUnknown);
impl IMFTimedTextCueList {
    pub unsafe fn GetLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCueByIndex(&self, index: u32) -> windows_core::Result<IMFTimedTextCue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCueByIndex)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCueById(&self, id: u32) -> windows_core::Result<IMFTimedTextCue> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCueById)(windows_core::Interface::as_raw(self), id, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCueByOriginalId<P0>(&self, originalid: P0) -> windows_core::Result<IMFTimedTextCue>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCueByOriginalId)(windows_core::Interface::as_raw(self), originalid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddTextCue<P2>(&self, start: f64, duration: f64, text: P2, cue: Option<*mut Option<IMFTimedTextCue>>) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddTextCue)(windows_core::Interface::as_raw(self), start, duration, text.param().abi(), cue.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn AddDataCue(&self, start: f64, duration: f64, data: &[u8], cue: Option<*mut Option<IMFTimedTextCue>>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDataCue)(windows_core::Interface::as_raw(self), start, duration, data.as_ptr(), data.len().try_into().unwrap(), cue.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn RemoveCue<P0>(&self, cue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTimedTextCue>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveCue)(windows_core::Interface::as_raw(self), cue.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextCueList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetCueByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCueById: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCueByOriginalId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddTextCue: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDataCue: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *const u8, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveCue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFTimedTextCueList_Impl: windows_core::IUnknownImpl {
    fn GetLength(&self) -> u32;
    fn GetCueByIndex(&self, index: u32) -> windows_core::Result<IMFTimedTextCue>;
    fn GetCueById(&self, id: u32) -> windows_core::Result<IMFTimedTextCue>;
    fn GetCueByOriginalId(&self, originalid: &windows_core::PCWSTR) -> windows_core::Result<IMFTimedTextCue>;
    fn AddTextCue(&self, start: f64, duration: f64, text: &windows_core::PCWSTR, cue: windows_core::OutRef<IMFTimedTextCue>) -> windows_core::Result<()>;
    fn AddDataCue(&self, start: f64, duration: f64, data: *const u8, datasize: u32, cue: windows_core::OutRef<IMFTimedTextCue>) -> windows_core::Result<()>;
    fn RemoveCue(&self, cue: windows_core::Ref<IMFTimedTextCue>) -> windows_core::Result<()>;
}
impl IMFTimedTextCueList_Vtbl {
    pub const fn new<Identity: IMFTimedTextCueList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLength<Identity: IMFTimedTextCueList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCueList_Impl::GetLength(this)
            }
        }
        unsafe extern "system" fn GetCueByIndex<Identity: IMFTimedTextCueList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, cue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextCueList_Impl::GetCueByIndex(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        cue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCueById<Identity: IMFTimedTextCueList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: u32, cue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextCueList_Impl::GetCueById(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        cue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCueByOriginalId<Identity: IMFTimedTextCueList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, originalid: windows_core::PCWSTR, cue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextCueList_Impl::GetCueByOriginalId(this, core::mem::transmute(&originalid)) {
                    Ok(ok__) => {
                        cue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTextCue<Identity: IMFTimedTextCueList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: f64, duration: f64, text: windows_core::PCWSTR, cue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCueList_Impl::AddTextCue(this, core::mem::transmute_copy(&start), core::mem::transmute_copy(&duration), core::mem::transmute(&text), core::mem::transmute_copy(&cue)).into()
            }
        }
        unsafe extern "system" fn AddDataCue<Identity: IMFTimedTextCueList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, start: f64, duration: f64, data: *const u8, datasize: u32, cue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCueList_Impl::AddDataCue(this, core::mem::transmute_copy(&start), core::mem::transmute_copy(&duration), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize), core::mem::transmute_copy(&cue)).into()
            }
        }
        unsafe extern "system" fn RemoveCue<Identity: IMFTimedTextCueList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cue: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextCueList_Impl::RemoveCue(this, core::mem::transmute_copy(&cue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            GetCueByIndex: GetCueByIndex::<Identity, OFFSET>,
            GetCueById: GetCueById::<Identity, OFFSET>,
            GetCueByOriginalId: GetCueByOriginalId::<Identity, OFFSET>,
            AddTextCue: AddTextCue::<Identity, OFFSET>,
            AddDataCue: AddDataCue::<Identity, OFFSET>,
            RemoveCue: RemoveCue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextCueList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTimedTextCueList {}
windows_core::imp::define_interface!(IMFTimedTextFormattedText, IMFTimedTextFormattedText_Vtbl, 0xe13af3c1_4d47_4354_b1f5_e83ae0ecae60);
windows_core::imp::interface_hierarchy!(IMFTimedTextFormattedText, windows_core::IUnknown);
impl IMFTimedTextFormattedText {
    pub unsafe fn GetText(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSubformattingCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetSubformattingCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSubformatting(&self, index: u32, firstchar: *mut u32, charlength: *mut u32, style: *mut Option<IMFTimedTextStyle>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSubformatting)(windows_core::Interface::as_raw(self), index, firstchar as _, charlength as _, core::mem::transmute(style)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextFormattedText_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetSubformattingCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSubformatting: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFTimedTextFormattedText_Impl: windows_core::IUnknownImpl {
    fn GetText(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetSubformattingCount(&self) -> u32;
    fn GetSubformatting(&self, index: u32, firstchar: *mut u32, charlength: *mut u32, style: windows_core::OutRef<IMFTimedTextStyle>) -> windows_core::Result<()>;
}
impl IMFTimedTextFormattedText_Vtbl {
    pub const fn new<Identity: IMFTimedTextFormattedText_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetText<Identity: IMFTimedTextFormattedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextFormattedText_Impl::GetText(this) {
                    Ok(ok__) => {
                        text.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSubformattingCount<Identity: IMFTimedTextFormattedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextFormattedText_Impl::GetSubformattingCount(this)
            }
        }
        unsafe extern "system" fn GetSubformatting<Identity: IMFTimedTextFormattedText_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, firstchar: *mut u32, charlength: *mut u32, style: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextFormattedText_Impl::GetSubformatting(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&firstchar), core::mem::transmute_copy(&charlength), core::mem::transmute_copy(&style)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetText: GetText::<Identity, OFFSET>,
            GetSubformattingCount: GetSubformattingCount::<Identity, OFFSET>,
            GetSubformatting: GetSubformatting::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextFormattedText as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTimedTextFormattedText {}
windows_core::imp::define_interface!(IMFTimedTextNotify, IMFTimedTextNotify_Vtbl, 0xdf6b87b6_ce12_45db_aba7_432fe054e57d);
windows_core::imp::interface_hierarchy!(IMFTimedTextNotify, windows_core::IUnknown);
impl IMFTimedTextNotify {
    pub unsafe fn TrackAdded(&self, trackid: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).TrackAdded)(windows_core::Interface::as_raw(self), trackid);
        }
    }
    pub unsafe fn TrackRemoved(&self, trackid: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).TrackRemoved)(windows_core::Interface::as_raw(self), trackid);
        }
    }
    pub unsafe fn TrackSelected(&self, trackid: u32, selected: bool) {
        unsafe {
            (windows_core::Interface::vtable(self).TrackSelected)(windows_core::Interface::as_raw(self), trackid, selected.into());
        }
    }
    pub unsafe fn TrackReadyStateChanged(&self, trackid: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).TrackReadyStateChanged)(windows_core::Interface::as_raw(self), trackid);
        }
    }
    pub unsafe fn Error(&self, errorcode: MF_TIMED_TEXT_ERROR_CODE, extendederrorcode: windows_core::HRESULT, sourcetrackid: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).Error)(windows_core::Interface::as_raw(self), errorcode, extendederrorcode, sourcetrackid);
        }
    }
    pub unsafe fn Cue<P2>(&self, cueevent: MF_TIMED_TEXT_CUE_EVENT, currenttime: f64, cue: P2)
    where
        P2: windows_core::Param<IMFTimedTextCue>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).Cue)(windows_core::Interface::as_raw(self), cueevent, currenttime, cue.param().abi());
        }
    }
    pub unsafe fn Reset(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TrackAdded: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub TrackRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub TrackSelected: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL),
    pub TrackReadyStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub Error: unsafe extern "system" fn(*mut core::ffi::c_void, MF_TIMED_TEXT_ERROR_CODE, windows_core::HRESULT, u32),
    pub Cue: unsafe extern "system" fn(*mut core::ffi::c_void, MF_TIMED_TEXT_CUE_EVENT, f64, *mut core::ffi::c_void),
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMFTimedTextNotify_Impl: windows_core::IUnknownImpl {
    fn TrackAdded(&self, trackid: u32);
    fn TrackRemoved(&self, trackid: u32);
    fn TrackSelected(&self, trackid: u32, selected: windows_core::BOOL);
    fn TrackReadyStateChanged(&self, trackid: u32);
    fn Error(&self, errorcode: MF_TIMED_TEXT_ERROR_CODE, extendederrorcode: windows_core::HRESULT, sourcetrackid: u32);
    fn Cue(&self, cueevent: MF_TIMED_TEXT_CUE_EVENT, currenttime: f64, cue: windows_core::Ref<IMFTimedTextCue>);
    fn Reset(&self);
}
impl IMFTimedTextNotify_Vtbl {
    pub const fn new<Identity: IMFTimedTextNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TrackAdded<Identity: IMFTimedTextNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackid: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextNotify_Impl::TrackAdded(this, core::mem::transmute_copy(&trackid));
            }
        }
        unsafe extern "system" fn TrackRemoved<Identity: IMFTimedTextNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackid: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextNotify_Impl::TrackRemoved(this, core::mem::transmute_copy(&trackid));
            }
        }
        unsafe extern "system" fn TrackSelected<Identity: IMFTimedTextNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackid: u32, selected: windows_core::BOOL) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextNotify_Impl::TrackSelected(this, core::mem::transmute_copy(&trackid), core::mem::transmute_copy(&selected));
            }
        }
        unsafe extern "system" fn TrackReadyStateChanged<Identity: IMFTimedTextNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackid: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextNotify_Impl::TrackReadyStateChanged(this, core::mem::transmute_copy(&trackid));
            }
        }
        unsafe extern "system" fn Error<Identity: IMFTimedTextNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, errorcode: MF_TIMED_TEXT_ERROR_CODE, extendederrorcode: windows_core::HRESULT, sourcetrackid: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextNotify_Impl::Error(this, core::mem::transmute_copy(&errorcode), core::mem::transmute_copy(&extendederrorcode), core::mem::transmute_copy(&sourcetrackid));
            }
        }
        unsafe extern "system" fn Cue<Identity: IMFTimedTextNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cueevent: MF_TIMED_TEXT_CUE_EVENT, currenttime: f64, cue: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextNotify_Impl::Cue(this, core::mem::transmute_copy(&cueevent), core::mem::transmute_copy(&currenttime), core::mem::transmute_copy(&cue));
            }
        }
        unsafe extern "system" fn Reset<Identity: IMFTimedTextNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextNotify_Impl::Reset(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TrackAdded: TrackAdded::<Identity, OFFSET>,
            TrackRemoved: TrackRemoved::<Identity, OFFSET>,
            TrackSelected: TrackSelected::<Identity, OFFSET>,
            TrackReadyStateChanged: TrackReadyStateChanged::<Identity, OFFSET>,
            Error: Error::<Identity, OFFSET>,
            Cue: Cue::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTimedTextNotify {}
windows_core::imp::define_interface!(IMFTimedTextRegion, IMFTimedTextRegion_Vtbl, 0xc8d22afc_bc47_4bdf_9b04_787e49ce3f58);
windows_core::imp::interface_hierarchy!(IMFTimedTextRegion, windows_core::IUnknown);
impl IMFTimedTextRegion {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPosition(&self, px: *mut f64, py: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), px as _, py as _, unittype as _) }
    }
    pub unsafe fn GetExtent(&self, pwidth: *mut f64, pheight: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtent)(windows_core::Interface::as_raw(self), pwidth as _, pheight as _, unittype as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetBackgroundColor(&self) -> windows_core::Result<super::mfobjects::MFARGB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackgroundColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetWritingMode(&self) -> windows_core::Result<MF_TIMED_TEXT_WRITING_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWritingMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDisplayAlignment(&self) -> windows_core::Result<MF_TIMED_TEXT_DISPLAY_ALIGNMENT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLineHeight(&self, plineheight: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetLineHeight)(windows_core::Interface::as_raw(self), plineheight as _, unittype as _) }
    }
    pub unsafe fn GetClipOverflow(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipOverflow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPadding(&self, before: *mut f64, start: *mut f64, after: *mut f64, end: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPadding)(windows_core::Interface::as_raw(self), before as _, start as _, after as _, end as _, unittype as _) }
    }
    pub unsafe fn GetWrap(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWrap)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetZIndex(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetZIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetScrollMode(&self) -> windows_core::Result<MF_TIMED_TEXT_SCROLL_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScrollMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextRegion_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64, *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT,
    pub GetExtent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64, *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mfobjects::MFARGB) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetBackgroundColor: usize,
    pub GetWritingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_WRITING_MODE) -> windows_core::HRESULT,
    pub GetDisplayAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_DISPLAY_ALIGNMENT) -> windows_core::HRESULT,
    pub GetLineHeight: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT,
    pub GetClipOverflow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut f64, *mut f64, *mut f64, *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT,
    pub GetWrap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetZIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetScrollMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_SCROLL_MODE) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTimedTextRegion_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetPosition(&self, px: *mut f64, py: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::Result<()>;
    fn GetExtent(&self, pwidth: *mut f64, pheight: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::Result<()>;
    fn GetBackgroundColor(&self) -> windows_core::Result<super::mfobjects::MFARGB>;
    fn GetWritingMode(&self) -> windows_core::Result<MF_TIMED_TEXT_WRITING_MODE>;
    fn GetDisplayAlignment(&self) -> windows_core::Result<MF_TIMED_TEXT_DISPLAY_ALIGNMENT>;
    fn GetLineHeight(&self, plineheight: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::Result<()>;
    fn GetClipOverflow(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetPadding(&self, before: *mut f64, start: *mut f64, after: *mut f64, end: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::Result<()>;
    fn GetWrap(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetZIndex(&self) -> windows_core::Result<i32>;
    fn GetScrollMode(&self) -> windows_core::Result<MF_TIMED_TEXT_SCROLL_MODE>;
}
#[cfg(feature = "mfobjects")]
impl IMFTimedTextRegion_Vtbl {
    pub const fn new<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRegion_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPosition<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, px: *mut f64, py: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextRegion_Impl::GetPosition(this, core::mem::transmute_copy(&px), core::mem::transmute_copy(&py), core::mem::transmute_copy(&unittype)).into()
            }
        }
        unsafe extern "system" fn GetExtent<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwidth: *mut f64, pheight: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextRegion_Impl::GetExtent(this, core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight), core::mem::transmute_copy(&unittype)).into()
            }
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bgcolor: *mut super::mfobjects::MFARGB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRegion_Impl::GetBackgroundColor(this) {
                    Ok(ok__) => {
                        bgcolor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWritingMode<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writingmode: *mut MF_TIMED_TEXT_WRITING_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRegion_Impl::GetWritingMode(this) {
                    Ok(ok__) => {
                        writingmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDisplayAlignment<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, displayalign: *mut MF_TIMED_TEXT_DISPLAY_ALIGNMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRegion_Impl::GetDisplayAlignment(this) {
                    Ok(ok__) => {
                        displayalign.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLineHeight<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plineheight: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextRegion_Impl::GetLineHeight(this, core::mem::transmute_copy(&plineheight), core::mem::transmute_copy(&unittype)).into()
            }
        }
        unsafe extern "system" fn GetClipOverflow<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipoverflow: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRegion_Impl::GetClipOverflow(this) {
                    Ok(ok__) => {
                        clipoverflow.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPadding<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, before: *mut f64, start: *mut f64, after: *mut f64, end: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextRegion_Impl::GetPadding(this, core::mem::transmute_copy(&before), core::mem::transmute_copy(&start), core::mem::transmute_copy(&after), core::mem::transmute_copy(&end), core::mem::transmute_copy(&unittype)).into()
            }
        }
        unsafe extern "system" fn GetWrap<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wrap: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRegion_Impl::GetWrap(this) {
                    Ok(ok__) => {
                        wrap.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetZIndex<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zindex: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRegion_Impl::GetZIndex(this) {
                    Ok(ok__) => {
                        zindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetScrollMode<Identity: IMFTimedTextRegion_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scrollmode: *mut MF_TIMED_TEXT_SCROLL_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRegion_Impl::GetScrollMode(this) {
                    Ok(ok__) => {
                        scrollmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            GetExtent: GetExtent::<Identity, OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Identity, OFFSET>,
            GetWritingMode: GetWritingMode::<Identity, OFFSET>,
            GetDisplayAlignment: GetDisplayAlignment::<Identity, OFFSET>,
            GetLineHeight: GetLineHeight::<Identity, OFFSET>,
            GetClipOverflow: GetClipOverflow::<Identity, OFFSET>,
            GetPadding: GetPadding::<Identity, OFFSET>,
            GetWrap: GetWrap::<Identity, OFFSET>,
            GetZIndex: GetZIndex::<Identity, OFFSET>,
            GetScrollMode: GetScrollMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextRegion as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTimedTextRegion {}
windows_core::imp::define_interface!(IMFTimedTextRuby, IMFTimedTextRuby_Vtbl, 0x76c6a6f5_4955_4de5_b27b_14b734cc14b4);
windows_core::imp::interface_hierarchy!(IMFTimedTextRuby, windows_core::IUnknown);
impl IMFTimedTextRuby {
    pub unsafe fn GetRubyText(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRubyText)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRubyPosition(&self) -> windows_core::Result<MF_TIMED_TEXT_RUBY_POSITION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRubyPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRubyAlign(&self) -> windows_core::Result<MF_TIMED_TEXT_RUBY_ALIGN> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRubyAlign)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRubyReserve(&self) -> windows_core::Result<MF_TIMED_TEXT_RUBY_RESERVE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRubyReserve)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextRuby_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRubyText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetRubyPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_RUBY_POSITION) -> windows_core::HRESULT,
    pub GetRubyAlign: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_RUBY_ALIGN) -> windows_core::HRESULT,
    pub GetRubyReserve: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_RUBY_RESERVE) -> windows_core::HRESULT,
}
pub trait IMFTimedTextRuby_Impl: windows_core::IUnknownImpl {
    fn GetRubyText(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetRubyPosition(&self) -> windows_core::Result<MF_TIMED_TEXT_RUBY_POSITION>;
    fn GetRubyAlign(&self) -> windows_core::Result<MF_TIMED_TEXT_RUBY_ALIGN>;
    fn GetRubyReserve(&self) -> windows_core::Result<MF_TIMED_TEXT_RUBY_RESERVE>;
}
impl IMFTimedTextRuby_Vtbl {
    pub const fn new<Identity: IMFTimedTextRuby_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRubyText<Identity: IMFTimedTextRuby_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rubytext: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRuby_Impl::GetRubyText(this) {
                    Ok(ok__) => {
                        rubytext.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRubyPosition<Identity: IMFTimedTextRuby_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut MF_TIMED_TEXT_RUBY_POSITION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRuby_Impl::GetRubyPosition(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRubyAlign<Identity: IMFTimedTextRuby_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut MF_TIMED_TEXT_RUBY_ALIGN) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRuby_Impl::GetRubyAlign(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRubyReserve<Identity: IMFTimedTextRuby_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut MF_TIMED_TEXT_RUBY_RESERVE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextRuby_Impl::GetRubyReserve(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRubyText: GetRubyText::<Identity, OFFSET>,
            GetRubyPosition: GetRubyPosition::<Identity, OFFSET>,
            GetRubyAlign: GetRubyAlign::<Identity, OFFSET>,
            GetRubyReserve: GetRubyReserve::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextRuby as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTimedTextRuby {}
windows_core::imp::define_interface!(IMFTimedTextStyle, IMFTimedTextStyle_Vtbl, 0x09b2455d_b834_4f01_a347_9052e21c450e);
windows_core::imp::interface_hierarchy!(IMFTimedTextStyle, windows_core::IUnknown);
impl IMFTimedTextStyle {
    pub unsafe fn GetName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsExternal(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsExternal)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetFontFamily(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontFamily)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFontSize(&self, fontsize: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFontSize)(windows_core::Interface::as_raw(self), fontsize as _, unittype as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetColor(&self) -> windows_core::Result<super::mfobjects::MFARGB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetBackgroundColor(&self) -> windows_core::Result<super::mfobjects::MFARGB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackgroundColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetShowBackgroundAlways(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetShowBackgroundAlways)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFontStyle(&self) -> windows_core::Result<MF_TIMED_TEXT_FONT_STYLE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetBold(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBold)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRightToLeft(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRightToLeft)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTextAlignment(&self) -> windows_core::Result<MF_TIMED_TEXT_ALIGNMENT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextAlignment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTextDecoration(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTextDecoration)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetTextOutline(&self, color: *mut super::mfobjects::MFARGB, thickness: *mut f64, blurradius: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTextOutline)(windows_core::Interface::as_raw(self), color as _, thickness as _, blurradius as _, unittype as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextStyle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsExternal: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetFontFamily: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetFontSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64, *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mfobjects::MFARGB) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetColor: usize,
    #[cfg(feature = "mfobjects")]
    pub GetBackgroundColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mfobjects::MFARGB) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetBackgroundColor: usize,
    pub GetShowBackgroundAlways: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFontStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_FONT_STYLE) -> windows_core::HRESULT,
    pub GetBold: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetRightToLeft: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetTextAlignment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TIMED_TEXT_ALIGNMENT) -> windows_core::HRESULT,
    pub GetTextDecoration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetTextOutline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mfobjects::MFARGB, *mut f64, *mut f64, *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetTextOutline: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTimedTextStyle_Impl: windows_core::IUnknownImpl {
    fn GetName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsExternal(&self) -> windows_core::BOOL;
    fn GetFontFamily(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetFontSize(&self, fontsize: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::Result<()>;
    fn GetColor(&self) -> windows_core::Result<super::mfobjects::MFARGB>;
    fn GetBackgroundColor(&self) -> windows_core::Result<super::mfobjects::MFARGB>;
    fn GetShowBackgroundAlways(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetFontStyle(&self) -> windows_core::Result<MF_TIMED_TEXT_FONT_STYLE>;
    fn GetBold(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetRightToLeft(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetTextAlignment(&self) -> windows_core::Result<MF_TIMED_TEXT_ALIGNMENT>;
    fn GetTextDecoration(&self) -> windows_core::Result<u32>;
    fn GetTextOutline(&self, color: *mut super::mfobjects::MFARGB, thickness: *mut f64, blurradius: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFTimedTextStyle_Vtbl {
    pub const fn new<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetName<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetName(this) {
                    Ok(ok__) => {
                        name.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsExternal<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextStyle_Impl::IsExternal(this)
            }
        }
        unsafe extern "system" fn GetFontFamily<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontfamily: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetFontFamily(this) {
                    Ok(ok__) => {
                        fontfamily.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontSize<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontsize: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextStyle_Impl::GetFontSize(this, core::mem::transmute_copy(&fontsize), core::mem::transmute_copy(&unittype)).into()
            }
        }
        unsafe extern "system" fn GetColor<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut super::mfobjects::MFARGB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetColor(this) {
                    Ok(ok__) => {
                        color.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bgcolor: *mut super::mfobjects::MFARGB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetBackgroundColor(this) {
                    Ok(ok__) => {
                        bgcolor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetShowBackgroundAlways<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, showbackgroundalways: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetShowBackgroundAlways(this) {
                    Ok(ok__) => {
                        showbackgroundalways.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontStyle<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fontstyle: *mut MF_TIMED_TEXT_FONT_STYLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetFontStyle(this) {
                    Ok(ok__) => {
                        fontstyle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBold<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bold: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetBold(this) {
                    Ok(ok__) => {
                        bold.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRightToLeft<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, righttoleft: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetRightToLeft(this) {
                    Ok(ok__) => {
                        righttoleft.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextAlignment<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textalign: *mut MF_TIMED_TEXT_ALIGNMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetTextAlignment(this) {
                    Ok(ok__) => {
                        textalign.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextDecoration<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textdecoration: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle_Impl::GetTextDecoration(this) {
                    Ok(ok__) => {
                        textdecoration.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTextOutline<Identity: IMFTimedTextStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, color: *mut super::mfobjects::MFARGB, thickness: *mut f64, blurradius: *mut f64, unittype: *mut MF_TIMED_TEXT_UNIT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextStyle_Impl::GetTextOutline(this, core::mem::transmute_copy(&color), core::mem::transmute_copy(&thickness), core::mem::transmute_copy(&blurradius), core::mem::transmute_copy(&unittype)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, OFFSET>,
            IsExternal: IsExternal::<Identity, OFFSET>,
            GetFontFamily: GetFontFamily::<Identity, OFFSET>,
            GetFontSize: GetFontSize::<Identity, OFFSET>,
            GetColor: GetColor::<Identity, OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Identity, OFFSET>,
            GetShowBackgroundAlways: GetShowBackgroundAlways::<Identity, OFFSET>,
            GetFontStyle: GetFontStyle::<Identity, OFFSET>,
            GetBold: GetBold::<Identity, OFFSET>,
            GetRightToLeft: GetRightToLeft::<Identity, OFFSET>,
            GetTextAlignment: GetTextAlignment::<Identity, OFFSET>,
            GetTextDecoration: GetTextDecoration::<Identity, OFFSET>,
            GetTextOutline: GetTextOutline::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextStyle as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTimedTextStyle {}
windows_core::imp::define_interface!(IMFTimedTextStyle2, IMFTimedTextStyle2_Vtbl, 0xdb639199_c809_4c89_bfca_d0bbb9729d6e);
windows_core::imp::interface_hierarchy!(IMFTimedTextStyle2, windows_core::IUnknown);
impl IMFTimedTextStyle2 {
    pub unsafe fn GetRuby(&self) -> windows_core::Result<IMFTimedTextRuby> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRuby)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetBouten(&self) -> windows_core::Result<IMFTimedTextBouten> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBouten)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsTextCombined(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTextCombined)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFontAngleInDegrees(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFontAngleInDegrees)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextStyle2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRuby: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBouten: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsTextCombined: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFontAngleInDegrees: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
pub trait IMFTimedTextStyle2_Impl: windows_core::IUnknownImpl {
    fn GetRuby(&self) -> windows_core::Result<IMFTimedTextRuby>;
    fn GetBouten(&self) -> windows_core::Result<IMFTimedTextBouten>;
    fn IsTextCombined(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetFontAngleInDegrees(&self) -> windows_core::Result<f64>;
}
impl IMFTimedTextStyle2_Vtbl {
    pub const fn new<Identity: IMFTimedTextStyle2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRuby<Identity: IMFTimedTextStyle2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ruby: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle2_Impl::GetRuby(this) {
                    Ok(ok__) => {
                        ruby.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBouten<Identity: IMFTimedTextStyle2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bouten: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle2_Impl::GetBouten(this) {
                    Ok(ok__) => {
                        bouten.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsTextCombined<Identity: IMFTimedTextStyle2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle2_Impl::IsTextCombined(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFontAngleInDegrees<Identity: IMFTimedTextStyle2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextStyle2_Impl::GetFontAngleInDegrees(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRuby: GetRuby::<Identity, OFFSET>,
            GetBouten: GetBouten::<Identity, OFFSET>,
            IsTextCombined: IsTextCombined::<Identity, OFFSET>,
            GetFontAngleInDegrees: GetFontAngleInDegrees::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextStyle2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTimedTextStyle2 {}
windows_core::imp::define_interface!(IMFTimedTextTrack, IMFTimedTextTrack_Vtbl, 0x8822c32d_654e_4233_bf21_d7f2e67d30d4);
windows_core::imp::interface_hierarchy!(IMFTimedTextTrack, windows_core::IUnknown);
impl IMFTimedTextTrack {
    pub unsafe fn GetId(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetId)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetLabel(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLabel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLabel<P0>(&self, label: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLabel)(windows_core::Interface::as_raw(self), label.param().abi()) }
    }
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTrackKind(&self) -> MF_TIMED_TEXT_TRACK_KIND {
        unsafe { (windows_core::Interface::vtable(self).GetTrackKind)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsInBand(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsInBand)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetInBandMetadataTrackDispatchType(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInBandMetadataTrackDispatchType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsActive(&self) -> windows_core::BOOL {
        unsafe { (windows_core::Interface::vtable(self).IsActive)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetErrorCode(&self) -> MF_TIMED_TEXT_ERROR_CODE {
        unsafe { (windows_core::Interface::vtable(self).GetErrorCode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetExtendedErrorCode(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetExtendedErrorCode)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetDataFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDataFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetReadyState(&self) -> MF_TIMED_TEXT_TRACK_READY_STATE {
        unsafe { (windows_core::Interface::vtable(self).GetReadyState)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCueList(&self) -> windows_core::Result<IMFTimedTextCueList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCueList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextTrack_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetId: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetTrackKind: unsafe extern "system" fn(*mut core::ffi::c_void) -> MF_TIMED_TEXT_TRACK_KIND,
    pub IsInBand: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetInBandMetadataTrackDispatchType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsActive: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::BOOL,
    pub GetErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void) -> MF_TIMED_TEXT_ERROR_CODE,
    pub GetExtendedErrorCode: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDataFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetReadyState: unsafe extern "system" fn(*mut core::ffi::c_void) -> MF_TIMED_TEXT_TRACK_READY_STATE,
    pub GetCueList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFTimedTextTrack_Impl: windows_core::IUnknownImpl {
    fn GetId(&self) -> u32;
    fn GetLabel(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLabel(&self, label: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetTrackKind(&self) -> MF_TIMED_TEXT_TRACK_KIND;
    fn IsInBand(&self) -> windows_core::BOOL;
    fn GetInBandMetadataTrackDispatchType(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsActive(&self) -> windows_core::BOOL;
    fn GetErrorCode(&self) -> MF_TIMED_TEXT_ERROR_CODE;
    fn GetExtendedErrorCode(&self) -> windows_core::Result<()>;
    fn GetDataFormat(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetReadyState(&self) -> MF_TIMED_TEXT_TRACK_READY_STATE;
    fn GetCueList(&self) -> windows_core::Result<IMFTimedTextCueList>;
}
impl IMFTimedTextTrack_Vtbl {
    pub const fn new<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetId<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextTrack_Impl::GetId(this)
            }
        }
        unsafe extern "system" fn GetLabel<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, label: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextTrack_Impl::GetLabel(this) {
                    Ok(ok__) => {
                        label.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLabel<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, label: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextTrack_Impl::SetLabel(this, core::mem::transmute(&label)).into()
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, language: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextTrack_Impl::GetLanguage(this) {
                    Ok(ok__) => {
                        language.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTrackKind<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MF_TIMED_TEXT_TRACK_KIND {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextTrack_Impl::GetTrackKind(this)
            }
        }
        unsafe extern "system" fn IsInBand<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextTrack_Impl::IsInBand(this)
            }
        }
        unsafe extern "system" fn GetInBandMetadataTrackDispatchType<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dispatchtype: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextTrack_Impl::GetInBandMetadataTrackDispatchType(this) {
                    Ok(ok__) => {
                        dispatchtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsActive<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextTrack_Impl::IsActive(this)
            }
        }
        unsafe extern "system" fn GetErrorCode<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MF_TIMED_TEXT_ERROR_CODE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextTrack_Impl::GetErrorCode(this)
            }
        }
        unsafe extern "system" fn GetExtendedErrorCode<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextTrack_Impl::GetExtendedErrorCode(this).into()
            }
        }
        unsafe extern "system" fn GetDataFormat<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextTrack_Impl::GetDataFormat(this) {
                    Ok(ok__) => {
                        format.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReadyState<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MF_TIMED_TEXT_TRACK_READY_STATE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextTrack_Impl::GetReadyState(this)
            }
        }
        unsafe extern "system" fn GetCueList<Identity: IMFTimedTextTrack_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextTrack_Impl::GetCueList(this) {
                    Ok(ok__) => {
                        cues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetId: GetId::<Identity, OFFSET>,
            GetLabel: GetLabel::<Identity, OFFSET>,
            SetLabel: SetLabel::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            GetTrackKind: GetTrackKind::<Identity, OFFSET>,
            IsInBand: IsInBand::<Identity, OFFSET>,
            GetInBandMetadataTrackDispatchType: GetInBandMetadataTrackDispatchType::<Identity, OFFSET>,
            IsActive: IsActive::<Identity, OFFSET>,
            GetErrorCode: GetErrorCode::<Identity, OFFSET>,
            GetExtendedErrorCode: GetExtendedErrorCode::<Identity, OFFSET>,
            GetDataFormat: GetDataFormat::<Identity, OFFSET>,
            GetReadyState: GetReadyState::<Identity, OFFSET>,
            GetCueList: GetCueList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextTrack as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTimedTextTrack {}
windows_core::imp::define_interface!(IMFTimedTextTrackList, IMFTimedTextTrackList_Vtbl, 0x23ff334c_442c_445f_bccc_edc438aa11e2);
windows_core::imp::interface_hierarchy!(IMFTimedTextTrackList, windows_core::IUnknown);
impl IMFTimedTextTrackList {
    pub unsafe fn GetLength(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetLength)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetTrack(&self, index: u32) -> windows_core::Result<IMFTimedTextTrack> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTrack)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTrackById(&self, trackid: u32) -> windows_core::Result<IMFTimedTextTrack> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTrackById)(windows_core::Interface::as_raw(self), trackid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimedTextTrackList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLength: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetTrack: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTrackById: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFTimedTextTrackList_Impl: windows_core::IUnknownImpl {
    fn GetLength(&self) -> u32;
    fn GetTrack(&self, index: u32) -> windows_core::Result<IMFTimedTextTrack>;
    fn GetTrackById(&self, trackid: u32) -> windows_core::Result<IMFTimedTextTrack>;
}
impl IMFTimedTextTrackList_Vtbl {
    pub const fn new<Identity: IMFTimedTextTrackList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLength<Identity: IMFTimedTextTrackList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimedTextTrackList_Impl::GetLength(this)
            }
        }
        unsafe extern "system" fn GetTrack<Identity: IMFTimedTextTrackList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, track: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextTrackList_Impl::GetTrack(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        track.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTrackById<Identity: IMFTimedTextTrackList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackid: u32, track: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimedTextTrackList_Impl::GetTrackById(this, core::mem::transmute_copy(&trackid)) {
                    Ok(ok__) => {
                        track.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLength: GetLength::<Identity, OFFSET>,
            GetTrack: GetTrack::<Identity, OFFSET>,
            GetTrackById: GetTrackById::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimedTextTrackList as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTimedTextTrackList {}
pub type MF_HDCP_STATUS = i32;
pub const MF_HDCP_STATUS_OFF: MF_HDCP_STATUS = 1;
pub const MF_HDCP_STATUS_ON: MF_HDCP_STATUS = 0;
pub const MF_HDCP_STATUS_ON_WITH_TYPE_ENFORCEMENT: MF_HDCP_STATUS = 2;
pub const MF_INVALID_PRESENTATION_TIME: u64 = 9223372036854775808;
pub const MF_MEDIAENGINE_KEYERR_CLIENT: MF_MEDIA_ENGINE_KEYERR = 2;
pub const MF_MEDIAENGINE_KEYERR_DOMAIN: MF_MEDIA_ENGINE_KEYERR = 6;
pub const MF_MEDIAENGINE_KEYERR_HARDWARECHANGE: MF_MEDIA_ENGINE_KEYERR = 5;
pub const MF_MEDIAENGINE_KEYERR_OUTPUT: MF_MEDIA_ENGINE_KEYERR = 4;
pub const MF_MEDIAENGINE_KEYERR_SERVICE: MF_MEDIA_ENGINE_KEYERR = 3;
pub const MF_MEDIAENGINE_KEYERR_UNKNOWN: MF_MEDIA_ENGINE_KEYERR = 1;
pub type MF_MEDIAKEYS_REQUIREMENT = i32;
pub const MF_MEDIAKEYS_REQUIREMENT_NOT_ALLOWED: MF_MEDIAKEYS_REQUIREMENT = 3;
pub const MF_MEDIAKEYS_REQUIREMENT_OPTIONAL: MF_MEDIAKEYS_REQUIREMENT = 2;
pub const MF_MEDIAKEYS_REQUIREMENT_REQUIRED: MF_MEDIAKEYS_REQUIREMENT = 1;
pub const MF_MEDIA_ENGINE_AUDIOONLY: MF_MEDIA_ENGINE_CREATEFLAGS = 1;
pub type MF_MEDIA_ENGINE_CANPLAY = i32;
pub const MF_MEDIA_ENGINE_CANPLAY_MAYBE: MF_MEDIA_ENGINE_CANPLAY = 1;
pub const MF_MEDIA_ENGINE_CANPLAY_NOT_SUPPORTED: MF_MEDIA_ENGINE_CANPLAY = 0;
pub const MF_MEDIA_ENGINE_CANPLAY_PROBABLY: MF_MEDIA_ENGINE_CANPLAY = 2;
pub type MF_MEDIA_ENGINE_CREATEFLAGS = i32;
pub const MF_MEDIA_ENGINE_CREATEFLAGS_MASK: MF_MEDIA_ENGINE_CREATEFLAGS = 31;
pub const MF_MEDIA_ENGINE_DISABLE_LOCAL_PLUGINS: MF_MEDIA_ENGINE_CREATEFLAGS = 16;
pub const MF_MEDIA_ENGINE_ENABLE_PROTECTED_CONTENT: MF_MEDIA_ENGINE_PROTECTION_FLAGS = 1;
pub type MF_MEDIA_ENGINE_ERR = i32;
pub const MF_MEDIA_ENGINE_ERR_ABORTED: MF_MEDIA_ENGINE_ERR = 1;
pub const MF_MEDIA_ENGINE_ERR_DECODE: MF_MEDIA_ENGINE_ERR = 3;
pub const MF_MEDIA_ENGINE_ERR_ENCRYPTED: MF_MEDIA_ENGINE_ERR = 5;
pub const MF_MEDIA_ENGINE_ERR_NETWORK: MF_MEDIA_ENGINE_ERR = 2;
pub const MF_MEDIA_ENGINE_ERR_NOERROR: MF_MEDIA_ENGINE_ERR = 0;
pub const MF_MEDIA_ENGINE_ERR_SRC_NOT_SUPPORTED: MF_MEDIA_ENGINE_ERR = 4;
pub type MF_MEDIA_ENGINE_EVENT = i32;
pub const MF_MEDIA_ENGINE_EVENT_ABORT: MF_MEDIA_ENGINE_EVENT = 4;
pub const MF_MEDIA_ENGINE_EVENT_AUDIOENDPOINTCHANGE: MF_MEDIA_ENGINE_EVENT = 1016;
pub const MF_MEDIA_ENGINE_EVENT_BALANCECHANGE: MF_MEDIA_ENGINE_EVENT = 1003;
pub const MF_MEDIA_ENGINE_EVENT_BUFFERINGENDED: MF_MEDIA_ENGINE_EVENT = 1006;
pub const MF_MEDIA_ENGINE_EVENT_BUFFERINGSTARTED: MF_MEDIA_ENGINE_EVENT = 1005;
pub const MF_MEDIA_ENGINE_EVENT_CANPLAY: MF_MEDIA_ENGINE_EVENT = 14;
pub const MF_MEDIA_ENGINE_EVENT_CANPLAYTHROUGH: MF_MEDIA_ENGINE_EVENT = 15;
pub const MF_MEDIA_ENGINE_EVENT_DELAYLOADEVENT_CHANGED: MF_MEDIA_ENGINE_EVENT = 1013;
pub const MF_MEDIA_ENGINE_EVENT_DOWNLOADCOMPLETE: MF_MEDIA_ENGINE_EVENT = 1004;
pub const MF_MEDIA_ENGINE_EVENT_DURATIONCHANGE: MF_MEDIA_ENGINE_EVENT = 21;
pub const MF_MEDIA_ENGINE_EVENT_EMPTIED: MF_MEDIA_ENGINE_EVENT = 6;
pub const MF_MEDIA_ENGINE_EVENT_ENDED: MF_MEDIA_ENGINE_EVENT = 19;
pub const MF_MEDIA_ENGINE_EVENT_ERROR: MF_MEDIA_ENGINE_EVENT = 5;
pub const MF_MEDIA_ENGINE_EVENT_FIRSTFRAMEREADY: MF_MEDIA_ENGINE_EVENT = 1009;
pub const MF_MEDIA_ENGINE_EVENT_FORMATCHANGE: MF_MEDIA_ENGINE_EVENT = 1000;
pub const MF_MEDIA_ENGINE_EVENT_FRAMESTEPCOMPLETED: MF_MEDIA_ENGINE_EVENT = 1007;
pub const MF_MEDIA_ENGINE_EVENT_LOADEDDATA: MF_MEDIA_ENGINE_EVENT = 11;
pub const MF_MEDIA_ENGINE_EVENT_LOADEDMETADATA: MF_MEDIA_ENGINE_EVENT = 10;
pub const MF_MEDIA_ENGINE_EVENT_LOADSTART: MF_MEDIA_ENGINE_EVENT = 1;
pub const MF_MEDIA_ENGINE_EVENT_NOTIFYSTABLESTATE: MF_MEDIA_ENGINE_EVENT = 1008;
pub const MF_MEDIA_ENGINE_EVENT_OPMINFO: MF_MEDIA_ENGINE_EVENT = 1011;
pub const MF_MEDIA_ENGINE_EVENT_PAUSE: MF_MEDIA_ENGINE_EVENT = 9;
pub const MF_MEDIA_ENGINE_EVENT_PLAY: MF_MEDIA_ENGINE_EVENT = 8;
pub const MF_MEDIA_ENGINE_EVENT_PLAYING: MF_MEDIA_ENGINE_EVENT = 13;
pub const MF_MEDIA_ENGINE_EVENT_PROGRESS: MF_MEDIA_ENGINE_EVENT = 2;
pub const MF_MEDIA_ENGINE_EVENT_PURGEQUEUEDEVENTS: MF_MEDIA_ENGINE_EVENT = 1001;
pub const MF_MEDIA_ENGINE_EVENT_RATECHANGE: MF_MEDIA_ENGINE_EVENT = 20;
pub const MF_MEDIA_ENGINE_EVENT_RESOURCELOST: MF_MEDIA_ENGINE_EVENT = 1012;
pub const MF_MEDIA_ENGINE_EVENT_SEEKED: MF_MEDIA_ENGINE_EVENT = 17;
pub const MF_MEDIA_ENGINE_EVENT_SEEKING: MF_MEDIA_ENGINE_EVENT = 16;
pub const MF_MEDIA_ENGINE_EVENT_STALLED: MF_MEDIA_ENGINE_EVENT = 7;
pub const MF_MEDIA_ENGINE_EVENT_STREAMRENDERINGERROR: MF_MEDIA_ENGINE_EVENT = 1014;
pub const MF_MEDIA_ENGINE_EVENT_SUPPORTEDRATES_CHANGED: MF_MEDIA_ENGINE_EVENT = 1015;
pub const MF_MEDIA_ENGINE_EVENT_SUSPEND: MF_MEDIA_ENGINE_EVENT = 3;
pub const MF_MEDIA_ENGINE_EVENT_TIMELINE_MARKER: MF_MEDIA_ENGINE_EVENT = 1002;
pub const MF_MEDIA_ENGINE_EVENT_TIMEUPDATE: MF_MEDIA_ENGINE_EVENT = 18;
pub const MF_MEDIA_ENGINE_EVENT_TRACKSCHANGE: MF_MEDIA_ENGINE_EVENT = 1010;
pub const MF_MEDIA_ENGINE_EVENT_VOLUMECHANGE: MF_MEDIA_ENGINE_EVENT = 22;
pub const MF_MEDIA_ENGINE_EVENT_WAITING: MF_MEDIA_ENGINE_EVENT = 12;
pub type MF_MEDIA_ENGINE_EXTENSION_TYPE = i32;
pub const MF_MEDIA_ENGINE_EXTENSION_TYPE_BYTESTREAM: MF_MEDIA_ENGINE_EXTENSION_TYPE = 1;
pub const MF_MEDIA_ENGINE_EXTENSION_TYPE_MEDIASOURCE: MF_MEDIA_ENGINE_EXTENSION_TYPE = 0;
pub const MF_MEDIA_ENGINE_FORCEMUTE: MF_MEDIA_ENGINE_CREATEFLAGS = 4;
pub type MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS = i32;
pub const MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAG_PROTECTED: MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS = 1;
pub const MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAG_REQUIRES_ANTI_SCREEN_SCRAPE_PROTECTION: MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS = 4;
pub const MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAG_REQUIRES_SURFACE_PROTECTION: MF_MEDIA_ENGINE_FRAME_PROTECTION_FLAGS = 2;
pub type MF_MEDIA_ENGINE_KEYERR = i32;
pub type MF_MEDIA_ENGINE_NETWORK = i32;
pub const MF_MEDIA_ENGINE_NETWORK_EMPTY: MF_MEDIA_ENGINE_NETWORK = 0;
pub const MF_MEDIA_ENGINE_NETWORK_IDLE: MF_MEDIA_ENGINE_NETWORK = 1;
pub const MF_MEDIA_ENGINE_NETWORK_LOADING: MF_MEDIA_ENGINE_NETWORK = 2;
pub const MF_MEDIA_ENGINE_NETWORK_NO_SOURCE: MF_MEDIA_ENGINE_NETWORK = 3;
pub const MF_MEDIA_ENGINE_OPM_ESTABLISHED: MF_MEDIA_ENGINE_OPM_STATUS = 1;
pub const MF_MEDIA_ENGINE_OPM_FAILED: MF_MEDIA_ENGINE_OPM_STATUS = 5;
pub const MF_MEDIA_ENGINE_OPM_FAILED_BDA: MF_MEDIA_ENGINE_OPM_STATUS = 3;
pub const MF_MEDIA_ENGINE_OPM_FAILED_UNSIGNED_DRIVER: MF_MEDIA_ENGINE_OPM_STATUS = 4;
pub const MF_MEDIA_ENGINE_OPM_FAILED_VM: MF_MEDIA_ENGINE_OPM_STATUS = 2;
pub const MF_MEDIA_ENGINE_OPM_NOT_REQUESTED: MF_MEDIA_ENGINE_OPM_STATUS = 0;
pub type MF_MEDIA_ENGINE_OPM_STATUS = i32;
pub type MF_MEDIA_ENGINE_PRELOAD = i32;
pub const MF_MEDIA_ENGINE_PRELOAD_AUTOMATIC: MF_MEDIA_ENGINE_PRELOAD = 4;
pub const MF_MEDIA_ENGINE_PRELOAD_EMPTY: MF_MEDIA_ENGINE_PRELOAD = 1;
pub const MF_MEDIA_ENGINE_PRELOAD_METADATA: MF_MEDIA_ENGINE_PRELOAD = 3;
pub const MF_MEDIA_ENGINE_PRELOAD_MISSING: MF_MEDIA_ENGINE_PRELOAD = 0;
pub const MF_MEDIA_ENGINE_PRELOAD_NONE: MF_MEDIA_ENGINE_PRELOAD = 2;
pub type MF_MEDIA_ENGINE_PROTECTION_FLAGS = i32;
pub type MF_MEDIA_ENGINE_READY = i32;
pub const MF_MEDIA_ENGINE_READY_HAVE_CURRENT_DATA: MF_MEDIA_ENGINE_READY = 2;
pub const MF_MEDIA_ENGINE_READY_HAVE_ENOUGH_DATA: MF_MEDIA_ENGINE_READY = 4;
pub const MF_MEDIA_ENGINE_READY_HAVE_FUTURE_DATA: MF_MEDIA_ENGINE_READY = 3;
pub const MF_MEDIA_ENGINE_READY_HAVE_METADATA: MF_MEDIA_ENGINE_READY = 1;
pub const MF_MEDIA_ENGINE_READY_HAVE_NOTHING: MF_MEDIA_ENGINE_READY = 0;
pub const MF_MEDIA_ENGINE_REAL_TIME_MODE: MF_MEDIA_ENGINE_CREATEFLAGS = 8;
pub type MF_MEDIA_ENGINE_S3D_PACKING_MODE = i32;
pub const MF_MEDIA_ENGINE_S3D_PACKING_MODE_NONE: MF_MEDIA_ENGINE_S3D_PACKING_MODE = 0;
pub const MF_MEDIA_ENGINE_S3D_PACKING_MODE_SIDE_BY_SIDE: MF_MEDIA_ENGINE_S3D_PACKING_MODE = 1;
pub const MF_MEDIA_ENGINE_S3D_PACKING_MODE_TOP_BOTTOM: MF_MEDIA_ENGINE_S3D_PACKING_MODE = 2;
pub type MF_MEDIA_ENGINE_SEEK_MODE = i32;
pub const MF_MEDIA_ENGINE_SEEK_MODE_APPROXIMATE: MF_MEDIA_ENGINE_SEEK_MODE = 1;
pub const MF_MEDIA_ENGINE_SEEK_MODE_NORMAL: MF_MEDIA_ENGINE_SEEK_MODE = 0;
pub type MF_MEDIA_ENGINE_STATISTIC = i32;
pub const MF_MEDIA_ENGINE_STATISTIC_BUFFER_PROGRESS: MF_MEDIA_ENGINE_STATISTIC = 3;
pub const MF_MEDIA_ENGINE_STATISTIC_BYTES_DOWNLOADED: MF_MEDIA_ENGINE_STATISTIC = 2;
pub const MF_MEDIA_ENGINE_STATISTIC_FRAMES_CORRUPTED: MF_MEDIA_ENGINE_STATISTIC = 6;
pub const MF_MEDIA_ENGINE_STATISTIC_FRAMES_DROPPED: MF_MEDIA_ENGINE_STATISTIC = 1;
pub const MF_MEDIA_ENGINE_STATISTIC_FRAMES_PER_SECOND: MF_MEDIA_ENGINE_STATISTIC = 4;
pub const MF_MEDIA_ENGINE_STATISTIC_FRAMES_RENDERED: MF_MEDIA_ENGINE_STATISTIC = 0;
pub const MF_MEDIA_ENGINE_STATISTIC_PLAYBACK_JITTER: MF_MEDIA_ENGINE_STATISTIC = 5;
pub const MF_MEDIA_ENGINE_STATISTIC_TOTAL_FRAME_DELAY: MF_MEDIA_ENGINE_STATISTIC = 7;
pub type MF_MEDIA_ENGINE_STREAMTYPE_FAILED = i32;
pub const MF_MEDIA_ENGINE_STREAMTYPE_FAILED_AUDIO: MF_MEDIA_ENGINE_STREAMTYPE_FAILED = 1;
pub const MF_MEDIA_ENGINE_STREAMTYPE_FAILED_UNKNOWN: MF_MEDIA_ENGINE_STREAMTYPE_FAILED = 0;
pub const MF_MEDIA_ENGINE_STREAMTYPE_FAILED_VIDEO: MF_MEDIA_ENGINE_STREAMTYPE_FAILED = 2;
pub const MF_MEDIA_ENGINE_USE_PMP_FOR_ALL_CONTENT: MF_MEDIA_ENGINE_PROTECTION_FLAGS = 2;
pub const MF_MEDIA_ENGINE_USE_UNPROTECTED_PMP: MF_MEDIA_ENGINE_PROTECTION_FLAGS = 4;
pub const MF_MEDIA_ENGINE_WAITFORSTABLE_STATE: MF_MEDIA_ENGINE_CREATEFLAGS = 2;
pub type MF_MSE_APPEND_MODE = i32;
pub const MF_MSE_APPEND_MODE_SEGMENTS: MF_MSE_APPEND_MODE = 0;
pub const MF_MSE_APPEND_MODE_SEQUENCE: MF_MSE_APPEND_MODE = 1;
pub type MF_MSE_ERROR = i32;
pub const MF_MSE_ERROR_DECODE: MF_MSE_ERROR = 2;
pub const MF_MSE_ERROR_NETWORK: MF_MSE_ERROR = 1;
pub const MF_MSE_ERROR_NOERROR: MF_MSE_ERROR = 0;
pub const MF_MSE_ERROR_UNKNOWN_ERROR: MF_MSE_ERROR = 3;
pub const MF_MSE_OPUS_SUPPORT_OFF: MF_MSE_OPUS_SUPPORT_TYPE = 1;
pub const MF_MSE_OPUS_SUPPORT_ON: MF_MSE_OPUS_SUPPORT_TYPE = 0;
pub type MF_MSE_OPUS_SUPPORT_TYPE = i32;
pub type MF_MSE_READY = i32;
pub const MF_MSE_READY_CLOSED: MF_MSE_READY = 1;
pub const MF_MSE_READY_ENDED: MF_MSE_READY = 3;
pub const MF_MSE_READY_OPEN: MF_MSE_READY = 2;
pub const MF_MSE_VP9_SUPPORT_DEFAULT: MF_MSE_VP9_SUPPORT_TYPE = 0;
pub const MF_MSE_VP9_SUPPORT_OFF: MF_MSE_VP9_SUPPORT_TYPE = 2;
pub const MF_MSE_VP9_SUPPORT_ON: MF_MSE_VP9_SUPPORT_TYPE = 1;
pub type MF_MSE_VP9_SUPPORT_TYPE = i32;
pub type MF_TIMED_TEXT_ALIGNMENT = i32;
pub const MF_TIMED_TEXT_ALIGNMENT_CENTER: MF_TIMED_TEXT_ALIGNMENT = 2;
pub const MF_TIMED_TEXT_ALIGNMENT_END: MF_TIMED_TEXT_ALIGNMENT = 1;
pub const MF_TIMED_TEXT_ALIGNMENT_START: MF_TIMED_TEXT_ALIGNMENT = 0;
pub type MF_TIMED_TEXT_BOUTEN_POSITION = i32;
pub const MF_TIMED_TEXT_BOUTEN_POSITION_AFTER: MF_TIMED_TEXT_BOUTEN_POSITION = 1;
pub const MF_TIMED_TEXT_BOUTEN_POSITION_BEFORE: MF_TIMED_TEXT_BOUTEN_POSITION = 0;
pub const MF_TIMED_TEXT_BOUTEN_POSITION_OUTSIDE: MF_TIMED_TEXT_BOUTEN_POSITION = 2;
pub type MF_TIMED_TEXT_BOUTEN_TYPE = i32;
pub const MF_TIMED_TEXT_BOUTEN_TYPE_AUTO: MF_TIMED_TEXT_BOUTEN_TYPE = 1;
pub const MF_TIMED_TEXT_BOUTEN_TYPE_FILLEDCIRCLE: MF_TIMED_TEXT_BOUTEN_TYPE = 2;
pub const MF_TIMED_TEXT_BOUTEN_TYPE_FILLEDDOT: MF_TIMED_TEXT_BOUTEN_TYPE = 4;
pub const MF_TIMED_TEXT_BOUTEN_TYPE_FILLEDSESAME: MF_TIMED_TEXT_BOUTEN_TYPE = 6;
pub const MF_TIMED_TEXT_BOUTEN_TYPE_NONE: MF_TIMED_TEXT_BOUTEN_TYPE = 0;
pub const MF_TIMED_TEXT_BOUTEN_TYPE_OPENCIRCLE: MF_TIMED_TEXT_BOUTEN_TYPE = 3;
pub const MF_TIMED_TEXT_BOUTEN_TYPE_OPENDOT: MF_TIMED_TEXT_BOUTEN_TYPE = 5;
pub const MF_TIMED_TEXT_BOUTEN_TYPE_OPENSESAME: MF_TIMED_TEXT_BOUTEN_TYPE = 7;
pub type MF_TIMED_TEXT_CUE_EVENT = i32;
pub const MF_TIMED_TEXT_CUE_EVENT_ACTIVE: MF_TIMED_TEXT_CUE_EVENT = 0;
pub const MF_TIMED_TEXT_CUE_EVENT_CLEAR: MF_TIMED_TEXT_CUE_EVENT = 2;
pub const MF_TIMED_TEXT_CUE_EVENT_INACTIVE: MF_TIMED_TEXT_CUE_EVENT = 1;
pub type MF_TIMED_TEXT_DECORATION = i32;
pub const MF_TIMED_TEXT_DECORATION_LINE_THROUGH: MF_TIMED_TEXT_DECORATION = 2;
pub const MF_TIMED_TEXT_DECORATION_NONE: MF_TIMED_TEXT_DECORATION = 0;
pub const MF_TIMED_TEXT_DECORATION_OVERLINE: MF_TIMED_TEXT_DECORATION = 4;
pub const MF_TIMED_TEXT_DECORATION_UNDERLINE: MF_TIMED_TEXT_DECORATION = 1;
pub type MF_TIMED_TEXT_DISPLAY_ALIGNMENT = i32;
pub const MF_TIMED_TEXT_DISPLAY_ALIGNMENT_AFTER: MF_TIMED_TEXT_DISPLAY_ALIGNMENT = 1;
pub const MF_TIMED_TEXT_DISPLAY_ALIGNMENT_BEFORE: MF_TIMED_TEXT_DISPLAY_ALIGNMENT = 0;
pub const MF_TIMED_TEXT_DISPLAY_ALIGNMENT_CENTER: MF_TIMED_TEXT_DISPLAY_ALIGNMENT = 2;
pub type MF_TIMED_TEXT_ERROR_CODE = i32;
pub const MF_TIMED_TEXT_ERROR_CODE_DATA_FORMAT: MF_TIMED_TEXT_ERROR_CODE = 2;
pub const MF_TIMED_TEXT_ERROR_CODE_FATAL: MF_TIMED_TEXT_ERROR_CODE = 1;
pub const MF_TIMED_TEXT_ERROR_CODE_INTERNAL: MF_TIMED_TEXT_ERROR_CODE = 4;
pub const MF_TIMED_TEXT_ERROR_CODE_NETWORK: MF_TIMED_TEXT_ERROR_CODE = 3;
pub const MF_TIMED_TEXT_ERROR_CODE_NOERROR: MF_TIMED_TEXT_ERROR_CODE = 0;
pub type MF_TIMED_TEXT_FONT_STYLE = i32;
pub const MF_TIMED_TEXT_FONT_STYLE_ITALIC: MF_TIMED_TEXT_FONT_STYLE = 2;
pub const MF_TIMED_TEXT_FONT_STYLE_NORMAL: MF_TIMED_TEXT_FONT_STYLE = 0;
pub const MF_TIMED_TEXT_FONT_STYLE_OBLIQUE: MF_TIMED_TEXT_FONT_STYLE = 1;
pub type MF_TIMED_TEXT_RUBY_ALIGN = i32;
pub const MF_TIMED_TEXT_RUBY_ALIGN_CENTER: MF_TIMED_TEXT_RUBY_ALIGN = 0;
pub const MF_TIMED_TEXT_RUBY_ALIGN_END: MF_TIMED_TEXT_RUBY_ALIGN = 2;
pub const MF_TIMED_TEXT_RUBY_ALIGN_SPACEAROUND: MF_TIMED_TEXT_RUBY_ALIGN = 3;
pub const MF_TIMED_TEXT_RUBY_ALIGN_SPACEBETWEEN: MF_TIMED_TEXT_RUBY_ALIGN = 4;
pub const MF_TIMED_TEXT_RUBY_ALIGN_START: MF_TIMED_TEXT_RUBY_ALIGN = 1;
pub const MF_TIMED_TEXT_RUBY_ALIGN_WITHBASE: MF_TIMED_TEXT_RUBY_ALIGN = 5;
pub type MF_TIMED_TEXT_RUBY_POSITION = i32;
pub const MF_TIMED_TEXT_RUBY_POSITION_AFTER: MF_TIMED_TEXT_RUBY_POSITION = 1;
pub const MF_TIMED_TEXT_RUBY_POSITION_BEFORE: MF_TIMED_TEXT_RUBY_POSITION = 0;
pub const MF_TIMED_TEXT_RUBY_POSITION_OUTSIDE: MF_TIMED_TEXT_RUBY_POSITION = 2;
pub type MF_TIMED_TEXT_RUBY_RESERVE = i32;
pub const MF_TIMED_TEXT_RUBY_RESERVE_AFTER: MF_TIMED_TEXT_RUBY_RESERVE = 2;
pub const MF_TIMED_TEXT_RUBY_RESERVE_BEFORE: MF_TIMED_TEXT_RUBY_RESERVE = 1;
pub const MF_TIMED_TEXT_RUBY_RESERVE_BOTH: MF_TIMED_TEXT_RUBY_RESERVE = 3;
pub const MF_TIMED_TEXT_RUBY_RESERVE_NONE: MF_TIMED_TEXT_RUBY_RESERVE = 0;
pub const MF_TIMED_TEXT_RUBY_RESERVE_OUTSIDE: MF_TIMED_TEXT_RUBY_RESERVE = 4;
pub type MF_TIMED_TEXT_SCROLL_MODE = i32;
pub const MF_TIMED_TEXT_SCROLL_MODE_POP_ON: MF_TIMED_TEXT_SCROLL_MODE = 0;
pub const MF_TIMED_TEXT_SCROLL_MODE_ROLL_UP: MF_TIMED_TEXT_SCROLL_MODE = 1;
pub type MF_TIMED_TEXT_TRACK_KIND = i32;
pub const MF_TIMED_TEXT_TRACK_KIND_CAPTIONS: MF_TIMED_TEXT_TRACK_KIND = 2;
pub const MF_TIMED_TEXT_TRACK_KIND_METADATA: MF_TIMED_TEXT_TRACK_KIND = 3;
pub const MF_TIMED_TEXT_TRACK_KIND_SUBTITLES: MF_TIMED_TEXT_TRACK_KIND = 1;
pub const MF_TIMED_TEXT_TRACK_KIND_UNKNOWN: MF_TIMED_TEXT_TRACK_KIND = 0;
pub type MF_TIMED_TEXT_TRACK_READY_STATE = i32;
pub const MF_TIMED_TEXT_TRACK_READY_STATE_ERROR: MF_TIMED_TEXT_TRACK_READY_STATE = 3;
pub const MF_TIMED_TEXT_TRACK_READY_STATE_LOADED: MF_TIMED_TEXT_TRACK_READY_STATE = 2;
pub const MF_TIMED_TEXT_TRACK_READY_STATE_LOADING: MF_TIMED_TEXT_TRACK_READY_STATE = 1;
pub const MF_TIMED_TEXT_TRACK_READY_STATE_NONE: MF_TIMED_TEXT_TRACK_READY_STATE = 0;
pub type MF_TIMED_TEXT_UNIT_TYPE = i32;
pub const MF_TIMED_TEXT_UNIT_TYPE_PERCENTAGE: MF_TIMED_TEXT_UNIT_TYPE = 1;
pub const MF_TIMED_TEXT_UNIT_TYPE_PIXELS: MF_TIMED_TEXT_UNIT_TYPE = 0;
pub type MF_TIMED_TEXT_WRITING_MODE = i32;
pub const MF_TIMED_TEXT_WRITING_MODE_LR: MF_TIMED_TEXT_WRITING_MODE = 4;
pub const MF_TIMED_TEXT_WRITING_MODE_LRTB: MF_TIMED_TEXT_WRITING_MODE = 0;
pub const MF_TIMED_TEXT_WRITING_MODE_RL: MF_TIMED_TEXT_WRITING_MODE = 5;
pub const MF_TIMED_TEXT_WRITING_MODE_RLTB: MF_TIMED_TEXT_WRITING_MODE = 1;
pub const MF_TIMED_TEXT_WRITING_MODE_TB: MF_TIMED_TEXT_WRITING_MODE = 6;
pub const MF_TIMED_TEXT_WRITING_MODE_TBLR: MF_TIMED_TEXT_WRITING_MODE = 3;
pub const MF_TIMED_TEXT_WRITING_MODE_TBRL: MF_TIMED_TEXT_WRITING_MODE = 2;
