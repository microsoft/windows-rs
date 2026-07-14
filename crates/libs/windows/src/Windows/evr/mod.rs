#[inline]
pub unsafe fn MFCreateVideoMixer<P0, T>(powner: P0, riiddevice: *const windows_core::GUID) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("evr.dll" "C" fn MFCreateVideoMixer(powner : *mut core::ffi::c_void, riiddevice : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFCreateVideoMixer(powner.param().abi(), riiddevice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn MFCreateVideoMixerAndPresenter<P0, P1>(pmixerowner: P0, ppresenterowner: P1, riidmixer: *const windows_core::GUID, ppvvideomixer: *mut *mut core::ffi::c_void, riidpresenter: *const windows_core::GUID, ppvvideopresenter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("evr.dll" "C" fn MFCreateVideoMixerAndPresenter(pmixerowner : *mut core::ffi::c_void, ppresenterowner : *mut core::ffi::c_void, riidmixer : *const windows_core::GUID, ppvvideomixer : *mut *mut core::ffi::c_void, riidpresenter : *const windows_core::GUID, ppvvideopresenter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFCreateVideoMixerAndPresenter(pmixerowner.param().abi(), ppresenterowner.param().abi(), riidmixer, ppvvideomixer as _, riidpresenter, ppvvideopresenter as _) }
}
#[inline]
pub unsafe fn MFCreateVideoPresenter<P0, T>(powner: P0, riiddevice: *const windows_core::GUID) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("evr.dll" "C" fn MFCreateVideoPresenter(powner : *mut core::ffi::c_void, riiddevice : *const windows_core::GUID, riid : *const windows_core::GUID, ppvideopresenter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFCreateVideoPresenter(powner.param().abi(), riiddevice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn MFCreateVideoRenderer(riidrenderer: *const windows_core::GUID, ppvideorenderer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("mf.dll" "C" fn MFCreateVideoRenderer(riidrenderer : *const windows_core::GUID, ppvideorenderer : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFCreateVideoRenderer(riidrenderer, ppvideorenderer as _) }
}
#[inline]
pub unsafe fn MFCreateVideoSampleAllocator<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("evr.dll" "C" fn MFCreateVideoSampleAllocator(riid : *const windows_core::GUID, ppsampleallocator : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFCreateVideoSampleAllocator(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateVideoSampleFromSurface<P0>(punksurface: P0) -> windows_core::Result<super::mfobjects::IMFSample>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("evr.dll" "C" fn MFCreateVideoSampleFromSurface(punksurface : *mut core::ffi::c_void, ppsample : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateVideoSampleFromSurface(punksurface.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
pub type EVRFilterConfigPrefs = i32;
pub const EVRFilterConfigPrefs_EnableQoS: EVRFilterConfigPrefs = 1;
pub const EVRFilterConfigPrefs_Mask: EVRFilterConfigPrefs = 1;
windows_core::imp::define_interface!(IEVRFilterConfig, IEVRFilterConfig_Vtbl, 0x83e91e85_82c1_4ea7_801d_85dc50b75086);
windows_core::imp::interface_hierarchy!(IEVRFilterConfig, windows_core::IUnknown);
impl IEVRFilterConfig {
    pub unsafe fn SetNumberOfStreams(&self, dwmaxstreams: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNumberOfStreams)(windows_core::Interface::as_raw(self), dwmaxstreams) }
    }
    pub unsafe fn GetNumberOfStreams(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfStreams)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEVRFilterConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetNumberOfStreams: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNumberOfStreams: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEVRFilterConfig_Impl: windows_core::IUnknownImpl {
    fn SetNumberOfStreams(&self, dwmaxstreams: u32) -> windows_core::Result<()>;
    fn GetNumberOfStreams(&self) -> windows_core::Result<u32>;
}
impl IEVRFilterConfig_Vtbl {
    pub const fn new<Identity: IEVRFilterConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetNumberOfStreams<Identity: IEVRFilterConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxstreams: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEVRFilterConfig_Impl::SetNumberOfStreams(this, core::mem::transmute_copy(&dwmaxstreams)).into()
            }
        }
        unsafe extern "system" fn GetNumberOfStreams<Identity: IEVRFilterConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmaxstreams: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEVRFilterConfig_Impl::GetNumberOfStreams(this) {
                    Ok(ok__) => {
                        pdwmaxstreams.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetNumberOfStreams: SetNumberOfStreams::<Identity, OFFSET>,
            GetNumberOfStreams: GetNumberOfStreams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEVRFilterConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEVRFilterConfig {}
windows_core::imp::define_interface!(IEVRFilterConfigEx, IEVRFilterConfigEx_Vtbl, 0xaea36028_796d_454f_beee_b48071e24304);
impl core::ops::Deref for IEVRFilterConfigEx {
    type Target = IEVRFilterConfig;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IEVRFilterConfigEx, windows_core::IUnknown, IEVRFilterConfig);
impl IEVRFilterConfigEx {
    pub unsafe fn SetConfigPrefs(&self, dwconfigflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConfigPrefs)(windows_core::Interface::as_raw(self), dwconfigflags) }
    }
    pub unsafe fn GetConfigPrefs(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfigPrefs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEVRFilterConfigEx_Vtbl {
    pub base__: IEVRFilterConfig_Vtbl,
    pub SetConfigPrefs: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetConfigPrefs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IEVRFilterConfigEx_Impl: IEVRFilterConfig_Impl {
    fn SetConfigPrefs(&self, dwconfigflags: u32) -> windows_core::Result<()>;
    fn GetConfigPrefs(&self) -> windows_core::Result<u32>;
}
impl IEVRFilterConfigEx_Vtbl {
    pub const fn new<Identity: IEVRFilterConfigEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetConfigPrefs<Identity: IEVRFilterConfigEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwconfigflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEVRFilterConfigEx_Impl::SetConfigPrefs(this, core::mem::transmute_copy(&dwconfigflags)).into()
            }
        }
        unsafe extern "system" fn GetConfigPrefs<Identity: IEVRFilterConfigEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwconfigflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEVRFilterConfigEx_Impl::GetConfigPrefs(this) {
                    Ok(ok__) => {
                        pdwconfigflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IEVRFilterConfig_Vtbl::new::<Identity, OFFSET>(),
            SetConfigPrefs: SetConfigPrefs::<Identity, OFFSET>,
            GetConfigPrefs: GetConfigPrefs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEVRFilterConfigEx as windows_core::Interface>::IID || iid == &<IEVRFilterConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEVRFilterConfigEx {}
windows_core::imp::define_interface!(IEVRTrustedVideoPlugin, IEVRTrustedVideoPlugin_Vtbl, 0x83a4ce40_7710_494b_a893_a472049af630);
windows_core::imp::interface_hierarchy!(IEVRTrustedVideoPlugin, windows_core::IUnknown);
impl IEVRTrustedVideoPlugin {
    pub unsafe fn IsInTrustedVideoMode(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsInTrustedVideoMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanConstrict(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanConstrict)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetConstriction(&self, dwkpix: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConstriction)(windows_core::Interface::as_raw(self), dwkpix) }
    }
    pub unsafe fn DisableImageExport(&self, bdisable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisableImageExport)(windows_core::Interface::as_raw(self), bdisable.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEVRTrustedVideoPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsInTrustedVideoMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CanConstrict: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetConstriction: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DisableImageExport: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IEVRTrustedVideoPlugin_Impl: windows_core::IUnknownImpl {
    fn IsInTrustedVideoMode(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CanConstrict(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetConstriction(&self, dwkpix: u32) -> windows_core::Result<()>;
    fn DisableImageExport(&self, bdisable: windows_core::BOOL) -> windows_core::Result<()>;
}
impl IEVRTrustedVideoPlugin_Vtbl {
    pub const fn new<Identity: IEVRTrustedVideoPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsInTrustedVideoMode<Identity: IEVRTrustedVideoPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pyes: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEVRTrustedVideoPlugin_Impl::IsInTrustedVideoMode(this) {
                    Ok(ok__) => {
                        pyes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanConstrict<Identity: IEVRTrustedVideoPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pyes: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IEVRTrustedVideoPlugin_Impl::CanConstrict(this) {
                    Ok(ok__) => {
                        pyes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConstriction<Identity: IEVRTrustedVideoPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwkpix: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEVRTrustedVideoPlugin_Impl::SetConstriction(this, core::mem::transmute_copy(&dwkpix)).into()
            }
        }
        unsafe extern "system" fn DisableImageExport<Identity: IEVRTrustedVideoPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bdisable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IEVRTrustedVideoPlugin_Impl::DisableImageExport(this, core::mem::transmute_copy(&bdisable)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsInTrustedVideoMode: IsInTrustedVideoMode::<Identity, OFFSET>,
            CanConstrict: CanConstrict::<Identity, OFFSET>,
            SetConstriction: SetConstriction::<Identity, OFFSET>,
            DisableImageExport: DisableImageExport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IEVRTrustedVideoPlugin as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IEVRTrustedVideoPlugin {}
windows_core::imp::define_interface!(IMFDesiredSample, IMFDesiredSample_Vtbl, 0x56c294d0_753e_4260_8d61_a3d8820b1d54);
windows_core::imp::interface_hierarchy!(IMFDesiredSample, windows_core::IUnknown);
impl IMFDesiredSample {
    pub unsafe fn GetDesiredSampleTimeAndDuration(&self, phnssampletime: *mut i64, phnssampleduration: *mut i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDesiredSampleTimeAndDuration)(windows_core::Interface::as_raw(self), phnssampletime as _, phnssampleduration as _) }
    }
    pub unsafe fn SetDesiredSampleTimeAndDuration(&self, hnssampletime: i64, hnssampleduration: i64) {
        unsafe {
            (windows_core::Interface::vtable(self).SetDesiredSampleTimeAndDuration)(windows_core::Interface::as_raw(self), hnssampletime, hnssampleduration);
        }
    }
    pub unsafe fn Clear(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDesiredSample_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDesiredSampleTimeAndDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64, *mut i64) -> windows_core::HRESULT,
    pub SetDesiredSampleTimeAndDuration: unsafe extern "system" fn(*mut core::ffi::c_void, i64, i64),
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMFDesiredSample_Impl: windows_core::IUnknownImpl {
    fn GetDesiredSampleTimeAndDuration(&self, phnssampletime: *mut i64, phnssampleduration: *mut i64) -> windows_core::Result<()>;
    fn SetDesiredSampleTimeAndDuration(&self, hnssampletime: i64, hnssampleduration: i64);
    fn Clear(&self);
}
impl IMFDesiredSample_Vtbl {
    pub const fn new<Identity: IMFDesiredSample_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDesiredSampleTimeAndDuration<Identity: IMFDesiredSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnssampletime: *mut i64, phnssampleduration: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDesiredSample_Impl::GetDesiredSampleTimeAndDuration(this, core::mem::transmute_copy(&phnssampletime), core::mem::transmute_copy(&phnssampleduration)).into()
            }
        }
        unsafe extern "system" fn SetDesiredSampleTimeAndDuration<Identity: IMFDesiredSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnssampletime: i64, hnssampleduration: i64) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDesiredSample_Impl::SetDesiredSampleTimeAndDuration(this, core::mem::transmute_copy(&hnssampletime), core::mem::transmute_copy(&hnssampleduration));
            }
        }
        unsafe extern "system" fn Clear<Identity: IMFDesiredSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFDesiredSample_Impl::Clear(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDesiredSampleTimeAndDuration: GetDesiredSampleTimeAndDuration::<Identity, OFFSET>,
            SetDesiredSampleTimeAndDuration: SetDesiredSampleTimeAndDuration::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDesiredSample as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFDesiredSample {}
windows_core::imp::define_interface!(IMFTopologyServiceLookup, IMFTopologyServiceLookup_Vtbl, 0xfa993889_4383_415a_a930_dd472a8cf6f7);
windows_core::imp::interface_hierarchy!(IMFTopologyServiceLookup, windows_core::IUnknown);
impl IMFTopologyServiceLookup {
    pub unsafe fn LookupService(&self, r#type: MF_SERVICE_LOOKUP_TYPE, dwindex: u32, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobjects: &mut [*mut core::ffi::c_void; 1], pnobjects: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LookupService)(windows_core::Interface::as_raw(self), r#type, dwindex, guidservice, riid, ppvobjects.as_mut_ptr(), pnobjects as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTopologyServiceLookup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LookupService: unsafe extern "system" fn(*mut core::ffi::c_void, MF_SERVICE_LOOKUP_TYPE, u32, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IMFTopologyServiceLookup_Impl: windows_core::IUnknownImpl {
    fn LookupService(&self, r#type: MF_SERVICE_LOOKUP_TYPE, dwindex: u32, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobjects: *mut *mut core::ffi::c_void, pnobjects: *mut u32) -> windows_core::Result<()>;
}
impl IMFTopologyServiceLookup_Vtbl {
    pub const fn new<Identity: IMFTopologyServiceLookup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LookupService<Identity: IMFTopologyServiceLookup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, r#type: MF_SERVICE_LOOKUP_TYPE, dwindex: u32, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobjects: *mut *mut core::ffi::c_void, pnobjects: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyServiceLookup_Impl::LookupService(this, core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobjects), core::mem::transmute_copy(&pnobjects)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LookupService: LookupService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTopologyServiceLookup as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTopologyServiceLookup {}
windows_core::imp::define_interface!(IMFTopologyServiceLookupClient, IMFTopologyServiceLookupClient_Vtbl, 0xfa99388a_4383_415a_a930_dd472a8cf6f7);
windows_core::imp::interface_hierarchy!(IMFTopologyServiceLookupClient, windows_core::IUnknown);
impl IMFTopologyServiceLookupClient {
    pub unsafe fn InitServicePointers<P0>(&self, plookup: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTopologyServiceLookup>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitServicePointers)(windows_core::Interface::as_raw(self), plookup.param().abi()) }
    }
    pub unsafe fn ReleaseServicePointers(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseServicePointers)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTopologyServiceLookupClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitServicePointers: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseServicePointers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFTopologyServiceLookupClient_Impl: windows_core::IUnknownImpl {
    fn InitServicePointers(&self, plookup: windows_core::Ref<IMFTopologyServiceLookup>) -> windows_core::Result<()>;
    fn ReleaseServicePointers(&self) -> windows_core::Result<()>;
}
impl IMFTopologyServiceLookupClient_Vtbl {
    pub const fn new<Identity: IMFTopologyServiceLookupClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitServicePointers<Identity: IMFTopologyServiceLookupClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plookup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyServiceLookupClient_Impl::InitServicePointers(this, core::mem::transmute_copy(&plookup)).into()
            }
        }
        unsafe extern "system" fn ReleaseServicePointers<Identity: IMFTopologyServiceLookupClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyServiceLookupClient_Impl::ReleaseServicePointers(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitServicePointers: InitServicePointers::<Identity, OFFSET>,
            ReleaseServicePointers: ReleaseServicePointers::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTopologyServiceLookupClient as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTopologyServiceLookupClient {}
windows_core::imp::define_interface!(IMFVideoDeviceID, IMFVideoDeviceID_Vtbl, 0xa38d9567_5a9c_4f3c_b293_8eb415b279ba);
windows_core::imp::interface_hierarchy!(IMFVideoDeviceID, windows_core::IUnknown);
impl IMFVideoDeviceID {
    pub unsafe fn GetDeviceID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoDeviceID_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
pub trait IMFVideoDeviceID_Impl: windows_core::IUnknownImpl {
    fn GetDeviceID(&self) -> windows_core::Result<windows_core::GUID>;
}
impl IMFVideoDeviceID_Vtbl {
    pub const fn new<Identity: IMFVideoDeviceID_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceID<Identity: IMFVideoDeviceID_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoDeviceID_Impl::GetDeviceID(this) {
                    Ok(ok__) => {
                        pdeviceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDeviceID: GetDeviceID::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoDeviceID as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFVideoDeviceID {}
windows_core::imp::define_interface!(IMFVideoDisplayControl, IMFVideoDisplayControl_Vtbl, 0xa490b1e4_ab84_4d31_a1b2_181e03b1077a);
windows_core::imp::interface_hierarchy!(IMFVideoDisplayControl, windows_core::IUnknown);
impl IMFVideoDisplayControl {
    #[cfg(feature = "windef")]
    pub unsafe fn GetNativeVideoSize(&self, pszvideo: *mut super::windef::SIZE, pszarvideo: *mut super::windef::SIZE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNativeVideoSize)(windows_core::Interface::as_raw(self), pszvideo as _, pszarvideo as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetIdealVideoSize(&self, pszmin: *mut super::windef::SIZE, pszmax: *mut super::windef::SIZE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIdealVideoSize)(windows_core::Interface::as_raw(self), pszmin as _, pszmax as _) }
    }
    #[cfg(all(feature = "mfidl", feature = "windef"))]
    pub unsafe fn SetVideoPosition(&self, pnrcsource: *const super::mfidl::MFVideoNormalizedRect, prcdest: *const super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVideoPosition)(windows_core::Interface::as_raw(self), pnrcsource, prcdest) }
    }
    #[cfg(all(feature = "mfidl", feature = "windef"))]
    pub unsafe fn GetVideoPosition(&self, pnrcsource: *mut super::mfidl::MFVideoNormalizedRect, prcdest: *mut super::windef::RECT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetVideoPosition)(windows_core::Interface::as_raw(self), pnrcsource as _, prcdest as _) }
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
    pub unsafe fn SetVideoWindow(&self, hwndvideo: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVideoWindow)(windows_core::Interface::as_raw(self), hwndvideo) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetVideoWindow(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoWindow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RepaintVideo(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RepaintVideo)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wingdi")]
    pub unsafe fn GetCurrentImage(&self, pbih: *mut super::wingdi::BITMAPINFOHEADER, pdib: *mut *mut u8, pcbdib: *mut u32, ptimestamp: *mut i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentImage)(windows_core::Interface::as_raw(self), pbih as _, pdib as _, pcbdib as _, ptimestamp as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetBorderColor(&self, clr: super::windef::COLORREF) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBorderColor)(windows_core::Interface::as_raw(self), clr) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetBorderColor(&self) -> windows_core::Result<super::windef::COLORREF> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBorderColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRenderingPrefs(&self, dwrenderflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRenderingPrefs)(windows_core::Interface::as_raw(self), dwrenderflags) }
    }
    pub unsafe fn GetRenderingPrefs(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRenderingPrefs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFullscreen(&self, ffullscreen: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFullscreen)(windows_core::Interface::as_raw(self), ffullscreen.into()) }
    }
    pub unsafe fn GetFullscreen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFullscreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoDisplayControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetNativeVideoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::SIZE, *mut super::windef::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetNativeVideoSize: usize,
    #[cfg(feature = "windef")]
    pub GetIdealVideoSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::SIZE, *mut super::windef::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetIdealVideoSize: usize,
    #[cfg(all(feature = "mfidl", feature = "windef"))]
    pub SetVideoPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::mfidl::MFVideoNormalizedRect, *const super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "windef")))]
    SetVideoPosition: usize,
    #[cfg(all(feature = "mfidl", feature = "windef"))]
    pub GetVideoPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mfidl::MFVideoNormalizedRect, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "windef")))]
    GetVideoPosition: usize,
    pub SetAspectRatioMode: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetAspectRatioMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetVideoWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetVideoWindow: usize,
    #[cfg(feature = "windef")]
    pub GetVideoWindow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetVideoWindow: usize,
    pub RepaintVideo: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wingdi")]
    pub GetCurrentImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wingdi::BITMAPINFOHEADER, *mut *mut u8, *mut u32, *mut i64) -> windows_core::HRESULT,
    #[cfg(not(feature = "wingdi"))]
    GetCurrentImage: usize,
    #[cfg(feature = "windef")]
    pub SetBorderColor: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetBorderColor: usize,
    #[cfg(feature = "windef")]
    pub GetBorderColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::COLORREF) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetBorderColor: usize,
    pub SetRenderingPrefs: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetRenderingPrefs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFullscreen: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetFullscreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfidl", feature = "windef", feature = "wingdi"))]
pub trait IMFVideoDisplayControl_Impl: windows_core::IUnknownImpl {
    fn GetNativeVideoSize(&self, pszvideo: *mut super::windef::SIZE, pszarvideo: *mut super::windef::SIZE) -> windows_core::Result<()>;
    fn GetIdealVideoSize(&self, pszmin: *mut super::windef::SIZE, pszmax: *mut super::windef::SIZE) -> windows_core::Result<()>;
    fn SetVideoPosition(&self, pnrcsource: *const super::mfidl::MFVideoNormalizedRect, prcdest: *const super::windef::RECT) -> windows_core::Result<()>;
    fn GetVideoPosition(&self, pnrcsource: *mut super::mfidl::MFVideoNormalizedRect, prcdest: *mut super::windef::RECT) -> windows_core::Result<()>;
    fn SetAspectRatioMode(&self, dwaspectratiomode: u32) -> windows_core::Result<()>;
    fn GetAspectRatioMode(&self) -> windows_core::Result<u32>;
    fn SetVideoWindow(&self, hwndvideo: super::windef::HWND) -> windows_core::Result<()>;
    fn GetVideoWindow(&self) -> windows_core::Result<super::windef::HWND>;
    fn RepaintVideo(&self) -> windows_core::Result<()>;
    fn GetCurrentImage(&self, pbih: *mut super::wingdi::BITMAPINFOHEADER, pdib: *mut *mut u8, pcbdib: *mut u32, ptimestamp: *mut i64) -> windows_core::Result<()>;
    fn SetBorderColor(&self, clr: super::windef::COLORREF) -> windows_core::Result<()>;
    fn GetBorderColor(&self) -> windows_core::Result<super::windef::COLORREF>;
    fn SetRenderingPrefs(&self, dwrenderflags: u32) -> windows_core::Result<()>;
    fn GetRenderingPrefs(&self) -> windows_core::Result<u32>;
    fn SetFullscreen(&self, ffullscreen: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetFullscreen(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "mfidl", feature = "windef", feature = "wingdi"))]
impl IMFVideoDisplayControl_Vtbl {
    pub const fn new<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNativeVideoSize<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvideo: *mut super::windef::SIZE, pszarvideo: *mut super::windef::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::GetNativeVideoSize(this, core::mem::transmute_copy(&pszvideo), core::mem::transmute_copy(&pszarvideo)).into()
            }
        }
        unsafe extern "system" fn GetIdealVideoSize<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszmin: *mut super::windef::SIZE, pszmax: *mut super::windef::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::GetIdealVideoSize(this, core::mem::transmute_copy(&pszmin), core::mem::transmute_copy(&pszmax)).into()
            }
        }
        unsafe extern "system" fn SetVideoPosition<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnrcsource: *const super::mfidl::MFVideoNormalizedRect, prcdest: *const super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::SetVideoPosition(this, core::mem::transmute_copy(&pnrcsource), core::mem::transmute_copy(&prcdest)).into()
            }
        }
        unsafe extern "system" fn GetVideoPosition<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnrcsource: *mut super::mfidl::MFVideoNormalizedRect, prcdest: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::GetVideoPosition(this, core::mem::transmute_copy(&pnrcsource), core::mem::transmute_copy(&prcdest)).into()
            }
        }
        unsafe extern "system" fn SetAspectRatioMode<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaspectratiomode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::SetAspectRatioMode(this, core::mem::transmute_copy(&dwaspectratiomode)).into()
            }
        }
        unsafe extern "system" fn GetAspectRatioMode<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwaspectratiomode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoDisplayControl_Impl::GetAspectRatioMode(this) {
                    Ok(ok__) => {
                        pdwaspectratiomode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVideoWindow<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndvideo: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::SetVideoWindow(this, core::mem::transmute_copy(&hwndvideo)).into()
            }
        }
        unsafe extern "system" fn GetVideoWindow<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwndvideo: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoDisplayControl_Impl::GetVideoWindow(this) {
                    Ok(ok__) => {
                        phwndvideo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RepaintVideo<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::RepaintVideo(this).into()
            }
        }
        unsafe extern "system" fn GetCurrentImage<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbih: *mut super::wingdi::BITMAPINFOHEADER, pdib: *mut *mut u8, pcbdib: *mut u32, ptimestamp: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::GetCurrentImage(this, core::mem::transmute_copy(&pbih), core::mem::transmute_copy(&pdib), core::mem::transmute_copy(&pcbdib), core::mem::transmute_copy(&ptimestamp)).into()
            }
        }
        unsafe extern "system" fn SetBorderColor<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clr: super::windef::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::SetBorderColor(this, core::mem::transmute_copy(&clr)).into()
            }
        }
        unsafe extern "system" fn GetBorderColor<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclr: *mut super::windef::COLORREF) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoDisplayControl_Impl::GetBorderColor(this) {
                    Ok(ok__) => {
                        pclr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRenderingPrefs<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwrenderflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::SetRenderingPrefs(this, core::mem::transmute_copy(&dwrenderflags)).into()
            }
        }
        unsafe extern "system" fn GetRenderingPrefs<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrenderflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoDisplayControl_Impl::GetRenderingPrefs(this) {
                    Ok(ok__) => {
                        pdwrenderflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFullscreen<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ffullscreen: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoDisplayControl_Impl::SetFullscreen(this, core::mem::transmute_copy(&ffullscreen)).into()
            }
        }
        unsafe extern "system" fn GetFullscreen<Identity: IMFVideoDisplayControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pffullscreen: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoDisplayControl_Impl::GetFullscreen(this) {
                    Ok(ok__) => {
                        pffullscreen.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNativeVideoSize: GetNativeVideoSize::<Identity, OFFSET>,
            GetIdealVideoSize: GetIdealVideoSize::<Identity, OFFSET>,
            SetVideoPosition: SetVideoPosition::<Identity, OFFSET>,
            GetVideoPosition: GetVideoPosition::<Identity, OFFSET>,
            SetAspectRatioMode: SetAspectRatioMode::<Identity, OFFSET>,
            GetAspectRatioMode: GetAspectRatioMode::<Identity, OFFSET>,
            SetVideoWindow: SetVideoWindow::<Identity, OFFSET>,
            GetVideoWindow: GetVideoWindow::<Identity, OFFSET>,
            RepaintVideo: RepaintVideo::<Identity, OFFSET>,
            GetCurrentImage: GetCurrentImage::<Identity, OFFSET>,
            SetBorderColor: SetBorderColor::<Identity, OFFSET>,
            GetBorderColor: GetBorderColor::<Identity, OFFSET>,
            SetRenderingPrefs: SetRenderingPrefs::<Identity, OFFSET>,
            GetRenderingPrefs: GetRenderingPrefs::<Identity, OFFSET>,
            SetFullscreen: SetFullscreen::<Identity, OFFSET>,
            GetFullscreen: GetFullscreen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoDisplayControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IMFVideoDisplayControl {}
windows_core::imp::define_interface!(IMFVideoMixerControl, IMFVideoMixerControl_Vtbl, 0xa5c6c53f_c202_4aa5_9695_175ba8c508a5);
windows_core::imp::interface_hierarchy!(IMFVideoMixerControl, windows_core::IUnknown);
impl IMFVideoMixerControl {
    pub unsafe fn SetStreamZOrder(&self, dwstreamid: u32, dwz: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamZOrder)(windows_core::Interface::as_raw(self), dwstreamid, dwz) }
    }
    pub unsafe fn GetStreamZOrder(&self, dwstreamid: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamZOrder)(windows_core::Interface::as_raw(self), dwstreamid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn SetStreamOutputRect(&self, dwstreamid: u32, pnrcoutput: *const super::mfidl::MFVideoNormalizedRect) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamOutputRect)(windows_core::Interface::as_raw(self), dwstreamid, pnrcoutput) }
    }
    #[cfg(feature = "mfidl")]
    pub unsafe fn GetStreamOutputRect(&self, dwstreamid: u32) -> windows_core::Result<super::mfidl::MFVideoNormalizedRect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamOutputRect)(windows_core::Interface::as_raw(self), dwstreamid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoMixerControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetStreamZOrder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub GetStreamZOrder: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfidl")]
    pub SetStreamOutputRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::mfidl::MFVideoNormalizedRect) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    SetStreamOutputRect: usize,
    #[cfg(feature = "mfidl")]
    pub GetStreamOutputRect: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut super::mfidl::MFVideoNormalizedRect) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfidl"))]
    GetStreamOutputRect: usize,
}
#[cfg(feature = "mfidl")]
pub trait IMFVideoMixerControl_Impl: windows_core::IUnknownImpl {
    fn SetStreamZOrder(&self, dwstreamid: u32, dwz: u32) -> windows_core::Result<()>;
    fn GetStreamZOrder(&self, dwstreamid: u32) -> windows_core::Result<u32>;
    fn SetStreamOutputRect(&self, dwstreamid: u32, pnrcoutput: *const super::mfidl::MFVideoNormalizedRect) -> windows_core::Result<()>;
    fn GetStreamOutputRect(&self, dwstreamid: u32) -> windows_core::Result<super::mfidl::MFVideoNormalizedRect>;
}
#[cfg(feature = "mfidl")]
impl IMFVideoMixerControl_Vtbl {
    pub const fn new<Identity: IMFVideoMixerControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStreamZOrder<Identity: IMFVideoMixerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, dwz: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoMixerControl_Impl::SetStreamZOrder(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&dwz)).into()
            }
        }
        unsafe extern "system" fn GetStreamZOrder<Identity: IMFVideoMixerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, pdwz: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoMixerControl_Impl::GetStreamZOrder(this, core::mem::transmute_copy(&dwstreamid)) {
                    Ok(ok__) => {
                        pdwz.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStreamOutputRect<Identity: IMFVideoMixerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, pnrcoutput: *const super::mfidl::MFVideoNormalizedRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoMixerControl_Impl::SetStreamOutputRect(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&pnrcoutput)).into()
            }
        }
        unsafe extern "system" fn GetStreamOutputRect<Identity: IMFVideoMixerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, pnrcoutput: *mut super::mfidl::MFVideoNormalizedRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoMixerControl_Impl::GetStreamOutputRect(this, core::mem::transmute_copy(&dwstreamid)) {
                    Ok(ok__) => {
                        pnrcoutput.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetStreamZOrder: SetStreamZOrder::<Identity, OFFSET>,
            GetStreamZOrder: GetStreamZOrder::<Identity, OFFSET>,
            SetStreamOutputRect: SetStreamOutputRect::<Identity, OFFSET>,
            GetStreamOutputRect: GetStreamOutputRect::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoMixerControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfidl")]
impl windows_core::RuntimeName for IMFVideoMixerControl {}
windows_core::imp::define_interface!(IMFVideoMixerControl2, IMFVideoMixerControl2_Vtbl, 0x8459616d_966e_4930_b658_54fa7e5a16d3);
impl core::ops::Deref for IMFVideoMixerControl2 {
    type Target = IMFVideoMixerControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFVideoMixerControl2, windows_core::IUnknown, IMFVideoMixerControl);
impl IMFVideoMixerControl2 {
    pub unsafe fn SetMixingPrefs(&self, dwmixflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMixingPrefs)(windows_core::Interface::as_raw(self), dwmixflags) }
    }
    pub unsafe fn GetMixingPrefs(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMixingPrefs)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoMixerControl2_Vtbl {
    pub base__: IMFVideoMixerControl_Vtbl,
    pub SetMixingPrefs: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetMixingPrefs: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "mfidl")]
pub trait IMFVideoMixerControl2_Impl: IMFVideoMixerControl_Impl {
    fn SetMixingPrefs(&self, dwmixflags: u32) -> windows_core::Result<()>;
    fn GetMixingPrefs(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "mfidl")]
impl IMFVideoMixerControl2_Vtbl {
    pub const fn new<Identity: IMFVideoMixerControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMixingPrefs<Identity: IMFVideoMixerControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmixflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoMixerControl2_Impl::SetMixingPrefs(this, core::mem::transmute_copy(&dwmixflags)).into()
            }
        }
        unsafe extern "system" fn GetMixingPrefs<Identity: IMFVideoMixerControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwmixflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoMixerControl2_Impl::GetMixingPrefs(this) {
                    Ok(ok__) => {
                        pdwmixflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMFVideoMixerControl_Vtbl::new::<Identity, OFFSET>(),
            SetMixingPrefs: SetMixingPrefs::<Identity, OFFSET>,
            GetMixingPrefs: GetMixingPrefs::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoMixerControl2 as windows_core::Interface>::IID || iid == &<IMFVideoMixerControl as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfidl")]
impl windows_core::RuntimeName for IMFVideoMixerControl2 {}
windows_core::imp::define_interface!(IMFVideoPositionMapper, IMFVideoPositionMapper_Vtbl, 0x1f6a9f17_e70b_4e24_8ae4_0b2c3ba7a4ae);
windows_core::imp::interface_hierarchy!(IMFVideoPositionMapper, windows_core::IUnknown);
impl IMFVideoPositionMapper {
    pub unsafe fn MapOutputCoordinateToInputStream(&self, xout: f32, yout: f32, dwoutputstreamindex: u32, dwinputstreamindex: u32, pxin: *mut f32, pyin: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MapOutputCoordinateToInputStream)(windows_core::Interface::as_raw(self), xout, yout, dwoutputstreamindex, dwinputstreamindex, pxin as _, pyin as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoPositionMapper_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub MapOutputCoordinateToInputStream: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, u32, u32, *mut f32, *mut f32) -> windows_core::HRESULT,
}
pub trait IMFVideoPositionMapper_Impl: windows_core::IUnknownImpl {
    fn MapOutputCoordinateToInputStream(&self, xout: f32, yout: f32, dwoutputstreamindex: u32, dwinputstreamindex: u32, pxin: *mut f32, pyin: *mut f32) -> windows_core::Result<()>;
}
impl IMFVideoPositionMapper_Vtbl {
    pub const fn new<Identity: IMFVideoPositionMapper_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MapOutputCoordinateToInputStream<Identity: IMFVideoPositionMapper_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, xout: f32, yout: f32, dwoutputstreamindex: u32, dwinputstreamindex: u32, pxin: *mut f32, pyin: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoPositionMapper_Impl::MapOutputCoordinateToInputStream(this, core::mem::transmute_copy(&xout), core::mem::transmute_copy(&yout), core::mem::transmute_copy(&dwoutputstreamindex), core::mem::transmute_copy(&dwinputstreamindex), core::mem::transmute_copy(&pxin), core::mem::transmute_copy(&pyin)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MapOutputCoordinateToInputStream: MapOutputCoordinateToInputStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoPositionMapper as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFVideoPositionMapper {}
#[cfg(feature = "mfidl")]
windows_core::imp::define_interface!(IMFVideoPresenter, IMFVideoPresenter_Vtbl, 0x29aff080_182a_4a5d_af3b_448f3a6346cb);
#[cfg(feature = "mfidl")]
impl core::ops::Deref for IMFVideoPresenter {
    type Target = super::mfidl::IMFClockStateSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfidl")]
windows_core::imp::interface_hierarchy!(IMFVideoPresenter, windows_core::IUnknown, super::mfidl::IMFClockStateSink);
#[cfg(feature = "mfidl")]
impl IMFVideoPresenter {
    pub unsafe fn ProcessMessage(&self, emessage: MFVP_MESSAGE_TYPE, ulparam: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessMessage)(windows_core::Interface::as_raw(self), emessage, ulparam) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetCurrentMediaType(&self) -> windows_core::Result<super::mfobjects::IMFVideoMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentMediaType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoPresenter_Vtbl {
    pub base__: super::mfidl::IMFClockStateSink_Vtbl,
    pub ProcessMessage: unsafe extern "system" fn(*mut core::ffi::c_void, MFVP_MESSAGE_TYPE, usize) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetCurrentMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetCurrentMediaType: usize,
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
pub trait IMFVideoPresenter_Impl: super::mfidl::IMFClockStateSink_Impl {
    fn ProcessMessage(&self, emessage: MFVP_MESSAGE_TYPE, ulparam: usize) -> windows_core::Result<()>;
    fn GetCurrentMediaType(&self) -> windows_core::Result<super::mfobjects::IMFVideoMediaType>;
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
impl IMFVideoPresenter_Vtbl {
    pub const fn new<Identity: IMFVideoPresenter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessMessage<Identity: IMFVideoPresenter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emessage: MFVP_MESSAGE_TYPE, ulparam: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoPresenter_Impl::ProcessMessage(this, core::mem::transmute_copy(&emessage), core::mem::transmute_copy(&ulparam)).into()
            }
        }
        unsafe extern "system" fn GetCurrentMediaType<Identity: IMFVideoPresenter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoPresenter_Impl::GetCurrentMediaType(this) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::mfidl::IMFClockStateSink_Vtbl::new::<Identity, OFFSET>(),
            ProcessMessage: ProcessMessage::<Identity, OFFSET>,
            GetCurrentMediaType: GetCurrentMediaType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoPresenter as windows_core::Interface>::IID || iid == &<super::mfidl::IMFClockStateSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "mfobjects"))]
impl windows_core::RuntimeName for IMFVideoPresenter {}
windows_core::imp::define_interface!(IMFVideoRenderer, IMFVideoRenderer_Vtbl, 0xdfdfd197_a9ca_43d8_b341_6af3503792cd);
windows_core::imp::interface_hierarchy!(IMFVideoRenderer, windows_core::IUnknown);
impl IMFVideoRenderer {
    #[cfg(all(feature = "mfidl", feature = "mftransform"))]
    pub unsafe fn InitializeRenderer<P0, P1>(&self, pvideomixer: P0, pvideopresenter: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::mftransform::IMFTransform>,
        P1: windows_core::Param<IMFVideoPresenter>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeRenderer)(windows_core::Interface::as_raw(self), pvideomixer.param().abi(), pvideopresenter.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoRenderer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfidl", feature = "mftransform"))]
    pub InitializeRenderer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfidl", feature = "mftransform")))]
    InitializeRenderer: usize,
}
#[cfg(all(feature = "mfidl", feature = "mftransform"))]
pub trait IMFVideoRenderer_Impl: windows_core::IUnknownImpl {
    fn InitializeRenderer(&self, pvideomixer: windows_core::Ref<super::mftransform::IMFTransform>, pvideopresenter: windows_core::Ref<IMFVideoPresenter>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfidl", feature = "mftransform"))]
impl IMFVideoRenderer_Vtbl {
    pub const fn new<Identity: IMFVideoRenderer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeRenderer<Identity: IMFVideoRenderer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvideomixer: *mut core::ffi::c_void, pvideopresenter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoRenderer_Impl::InitializeRenderer(this, core::mem::transmute_copy(&pvideomixer), core::mem::transmute_copy(&pvideopresenter)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InitializeRenderer: InitializeRenderer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoRenderer as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfidl", feature = "mftransform"))]
impl windows_core::RuntimeName for IMFVideoRenderer {}
pub const MFEVRDLL: u32 = 0;
pub const MFVP_MESSAGE_BEGINSTREAMING: MFVP_MESSAGE_TYPE = 3;
pub const MFVP_MESSAGE_CANCELSTEP: MFVP_MESSAGE_TYPE = 7;
pub const MFVP_MESSAGE_ENDOFSTREAM: MFVP_MESSAGE_TYPE = 5;
pub const MFVP_MESSAGE_ENDSTREAMING: MFVP_MESSAGE_TYPE = 4;
pub const MFVP_MESSAGE_FLUSH: MFVP_MESSAGE_TYPE = 0;
pub const MFVP_MESSAGE_INVALIDATEMEDIATYPE: MFVP_MESSAGE_TYPE = 1;
pub const MFVP_MESSAGE_PROCESSINPUTNOTIFY: MFVP_MESSAGE_TYPE = 2;
pub const MFVP_MESSAGE_STEP: MFVP_MESSAGE_TYPE = 6;
pub type MFVP_MESSAGE_TYPE = i32;
pub const MFVideoARMode_Mask: MFVideoAspectRatioMode = 7;
pub const MFVideoARMode_NonLinearStretch: MFVideoAspectRatioMode = 4;
pub const MFVideoARMode_None: MFVideoAspectRatioMode = 0;
pub const MFVideoARMode_PreservePicture: MFVideoAspectRatioMode = 1;
pub const MFVideoARMode_PreservePixel: MFVideoAspectRatioMode = 2;
pub type MFVideoAspectRatioMode = i32;
pub type MFVideoMixPrefs = i32;
pub const MFVideoMixPrefs_AllowDropToBob: MFVideoMixPrefs = 4;
pub const MFVideoMixPrefs_AllowDropToHalfInterlace: MFVideoMixPrefs = 2;
pub const MFVideoMixPrefs_EnableRotation: MFVideoMixPrefs = 16;
pub const MFVideoMixPrefs_ForceBob: MFVideoMixPrefs = 8;
pub const MFVideoMixPrefs_ForceHalfInterlace: MFVideoMixPrefs = 1;
pub const MFVideoMixPrefs_Mask: MFVideoMixPrefs = 31;
pub type MFVideoRenderPrefs = i32;
pub const MFVideoRenderPrefs_AllowBatching: MFVideoRenderPrefs = 32;
pub const MFVideoRenderPrefs_AllowOutputThrottling: MFVideoRenderPrefs = 4;
pub const MFVideoRenderPrefs_AllowScaling: MFVideoRenderPrefs = 128;
pub const MFVideoRenderPrefs_DoNotClipToDevice: MFVideoRenderPrefs = 2;
pub const MFVideoRenderPrefs_DoNotRenderBorder: MFVideoRenderPrefs = 1;
pub const MFVideoRenderPrefs_DoNotRepaintOnStop: MFVideoRenderPrefs = 256;
pub const MFVideoRenderPrefs_ForceBatching: MFVideoRenderPrefs = 16;
pub const MFVideoRenderPrefs_ForceOutputThrottling: MFVideoRenderPrefs = 8;
pub const MFVideoRenderPrefs_ForceScaling: MFVideoRenderPrefs = 64;
pub const MFVideoRenderPrefs_Mask: MFVideoRenderPrefs = 511;
pub const MF_SERVICE_LOOKUP_ALL: MF_SERVICE_LOOKUP_TYPE = 4;
pub const MF_SERVICE_LOOKUP_DOWNSTREAM: MF_SERVICE_LOOKUP_TYPE = 2;
pub const MF_SERVICE_LOOKUP_DOWNSTREAM_DIRECT: MF_SERVICE_LOOKUP_TYPE = 3;
pub const MF_SERVICE_LOOKUP_GLOBAL: MF_SERVICE_LOOKUP_TYPE = 5;
pub type MF_SERVICE_LOOKUP_TYPE = i32;
pub const MF_SERVICE_LOOKUP_UPSTREAM: MF_SERVICE_LOOKUP_TYPE = 0;
pub const MF_SERVICE_LOOKUP_UPSTREAM_DIRECT: MF_SERVICE_LOOKUP_TYPE = 1;
pub const MR_BUFFER_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0xa562248c_9ac6_4ffc_9fba_3af8f8ad1a4d);
pub const MR_VIDEO_ACCELERATION_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0xefef5175_5c7d_4ce2_bbbd_34ff8bca6554);
pub const MR_VIDEO_MIXER_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0x073cd2fc_6cf4_40b7_8859_e89552c841f8);
pub const MR_VIDEO_RENDER_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0x1092a86c_ab1a_459a_a336_831fbc4d11ff);
pub const VIDEO_ZOOM_RECT: windows_core::GUID = windows_core::GUID::from_u128(0x7aaa1638_1b7f_4c93_bd89_5b9c9fb6fcf0);
