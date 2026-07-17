pub const AMDDS_ALL: u32 = 255;
pub const AMDDS_DCIPS: u32 = 1;
pub const AMDDS_DEFAULT: u32 = 255;
pub const AMDDS_NONE: u32 = 0;
pub const AMDDS_PRIMARY: u32 = 3;
pub const AMDDS_PS: u32 = 2;
pub const AMDDS_RGB: u32 = 84;
pub const AMDDS_RGBFLP: u32 = 64;
pub const AMDDS_RGBOFF: u32 = 16;
pub const AMDDS_RGBOVR: u32 = 4;
pub const AMDDS_YUV: u32 = 168;
pub const AMDDS_YUVFLP: u32 = 128;
pub const AMDDS_YUVOFF: u32 = 32;
pub const AMDDS_YUVOVR: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AM_FRAMESTEP_STEP {
    pub dwFramesToStep: u32,
}
pub type AM_PROPERTY_FRAMESTEP = i32;
pub const AM_PROPERTY_FRAMESTEP_CANCEL: AM_PROPERTY_FRAMESTEP = 2;
pub const AM_PROPERTY_FRAMESTEP_CANSTEP: AM_PROPERTY_FRAMESTEP = 3;
pub const AM_PROPERTY_FRAMESTEP_CANSTEPMULTIPLE: AM_PROPERTY_FRAMESTEP = 4;
pub const AM_PROPERTY_FRAMESTEP_STEP: AM_PROPERTY_FRAMESTEP = 1;
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ANALOGVIDEOINFO {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwActiveWidth: u32,
    pub dwActiveHeight: u32,
    pub AvgTimePerFrame: super::ksmedia::REFERENCE_TIME,
}
windows_core::imp::define_interface!(IBaseVideoMixer, IBaseVideoMixer_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IBaseVideoMixer, windows_core::IUnknown);
impl IBaseVideoMixer {
    pub unsafe fn SetLeadPin(&self, ipin: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLeadPin)(windows_core::Interface::as_raw(self), ipin) }
    }
    pub unsafe fn GetLeadPin(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLeadPin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInputPinCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputPinCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsUsingClock(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsUsingClock)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetUsingClock(&self, bvalue: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUsingClock)(windows_core::Interface::as_raw(self), bvalue) }
    }
    pub unsafe fn GetClockPeriod(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClockPeriod)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClockPeriod(&self, bvalue: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClockPeriod)(windows_core::Interface::as_raw(self), bvalue) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBaseVideoMixer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLeadPin: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetLeadPin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetInputPinCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsUsingClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetUsingClock: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetClockPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetClockPeriod: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
pub trait IBaseVideoMixer_Impl: windows_core::IUnknownImpl {
    fn SetLeadPin(&self, ipin: i32) -> windows_core::Result<()>;
    fn GetLeadPin(&self) -> windows_core::Result<i32>;
    fn GetInputPinCount(&self) -> windows_core::Result<i32>;
    fn IsUsingClock(&self) -> windows_core::Result<i32>;
    fn SetUsingClock(&self, bvalue: i32) -> windows_core::Result<()>;
    fn GetClockPeriod(&self) -> windows_core::Result<i32>;
    fn SetClockPeriod(&self, bvalue: i32) -> windows_core::Result<()>;
}
impl IBaseVideoMixer_Vtbl {
    pub const fn new<Identity: IBaseVideoMixer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLeadPin<Identity: IBaseVideoMixer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipin: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBaseVideoMixer_Impl::SetLeadPin(this, core::mem::transmute_copy(&ipin)).into()
            }
        }
        unsafe extern "system" fn GetLeadPin<Identity: IBaseVideoMixer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipin: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBaseVideoMixer_Impl::GetLeadPin(this) {
                    Ok(ok__) => {
                        pipin.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInputPinCount<Identity: IBaseVideoMixer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pipincount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBaseVideoMixer_Impl::GetInputPinCount(this) {
                    Ok(ok__) => {
                        pipincount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsUsingClock<Identity: IBaseVideoMixer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBaseVideoMixer_Impl::IsUsingClock(this) {
                    Ok(ok__) => {
                        pbvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUsingClock<Identity: IBaseVideoMixer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bvalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBaseVideoMixer_Impl::SetUsingClock(this, core::mem::transmute_copy(&bvalue)).into()
            }
        }
        unsafe extern "system" fn GetClockPeriod<Identity: IBaseVideoMixer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBaseVideoMixer_Impl::GetClockPeriod(this) {
                    Ok(ok__) => {
                        pbvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClockPeriod<Identity: IBaseVideoMixer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bvalue: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBaseVideoMixer_Impl::SetClockPeriod(this, core::mem::transmute_copy(&bvalue)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLeadPin: SetLeadPin::<Identity, OFFSET>,
            GetLeadPin: GetLeadPin::<Identity, OFFSET>,
            GetInputPinCount: GetInputPinCount::<Identity, OFFSET>,
            IsUsingClock: IsUsingClock::<Identity, OFFSET>,
            SetUsingClock: SetUsingClock::<Identity, OFFSET>,
            GetClockPeriod: GetClockPeriod::<Identity, OFFSET>,
            SetClockPeriod: SetClockPeriod::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBaseVideoMixer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBaseVideoMixer {}
windows_core::imp::define_interface!(IDirectDrawVideo, IDirectDrawVideo_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IDirectDrawVideo, windows_core::IUnknown);
impl IDirectDrawVideo {
    pub unsafe fn GetSwitches(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSwitches)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSwitches(&self, switches: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSwitches)(windows_core::Interface::as_raw(self), switches) }
    }
    #[cfg(feature = "ddraw")]
    pub unsafe fn GetCaps(&self, pcaps: *mut super::ddraw::DDCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCaps)(windows_core::Interface::as_raw(self), pcaps as _) }
    }
    #[cfg(feature = "ddraw")]
    pub unsafe fn GetEmulatedCaps(&self, pcaps: *mut super::ddraw::DDCAPS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEmulatedCaps)(windows_core::Interface::as_raw(self), pcaps as _) }
    }
    #[cfg(all(feature = "ddraw", feature = "ksmedia"))]
    pub unsafe fn GetSurfaceDesc(&self, psurfacedesc: *mut super::ddraw::DDSURFACEDESC) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSurfaceDesc)(windows_core::Interface::as_raw(self), psurfacedesc as _) }
    }
    pub unsafe fn GetFourCCCodes(&self, pcount: *mut u32, pcodes: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFourCCCodes)(windows_core::Interface::as_raw(self), pcount as _, pcodes as _) }
    }
    #[cfg(feature = "ddraw")]
    pub unsafe fn SetDirectDraw<P0>(&self, pdirectdraw: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::ddraw::IDirectDraw>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDirectDraw)(windows_core::Interface::as_raw(self), pdirectdraw.param().abi()) }
    }
    #[cfg(feature = "ddraw")]
    pub unsafe fn GetDirectDraw(&self) -> windows_core::Result<super::ddraw::IDirectDraw> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDirectDraw)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSurfaceType(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSurfaceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDefault(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefault)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn UseScanLine(&self, usescanline: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UseScanLine)(windows_core::Interface::as_raw(self), usescanline) }
    }
    pub unsafe fn CanUseScanLine(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanUseScanLine)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UseOverlayStretch(&self, useoverlaystretch: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UseOverlayStretch)(windows_core::Interface::as_raw(self), useoverlaystretch) }
    }
    pub unsafe fn CanUseOverlayStretch(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanUseOverlayStretch)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UseWhenFullScreen(&self, usewhenfullscreen: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UseWhenFullScreen)(windows_core::Interface::as_raw(self), usewhenfullscreen) }
    }
    pub unsafe fn WillUseFullScreen(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WillUseFullScreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirectDrawVideo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSwitches: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetSwitches: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "ddraw")]
    pub GetCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ddraw::DDCAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "ddraw"))]
    GetCaps: usize,
    #[cfg(feature = "ddraw")]
    pub GetEmulatedCaps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ddraw::DDCAPS) -> windows_core::HRESULT,
    #[cfg(not(feature = "ddraw"))]
    GetEmulatedCaps: usize,
    #[cfg(all(feature = "ddraw", feature = "ksmedia"))]
    pub GetSurfaceDesc: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ddraw::DDSURFACEDESC) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "ddraw", feature = "ksmedia")))]
    GetSurfaceDesc: usize,
    pub GetFourCCCodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "ddraw")]
    pub SetDirectDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ddraw"))]
    SetDirectDraw: usize,
    #[cfg(feature = "ddraw")]
    pub GetDirectDraw: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "ddraw"))]
    GetDirectDraw: usize,
    pub GetSurfaceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetDefault: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UseScanLine: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CanUseScanLine: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UseOverlayStretch: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CanUseOverlayStretch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UseWhenFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub WillUseFullScreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "ddraw", feature = "ksmedia"))]
pub trait IDirectDrawVideo_Impl: windows_core::IUnknownImpl {
    fn GetSwitches(&self) -> windows_core::Result<u32>;
    fn SetSwitches(&self, switches: u32) -> windows_core::Result<()>;
    fn GetCaps(&self, pcaps: *mut super::ddraw::DDCAPS) -> windows_core::Result<()>;
    fn GetEmulatedCaps(&self, pcaps: *mut super::ddraw::DDCAPS) -> windows_core::Result<()>;
    fn GetSurfaceDesc(&self, psurfacedesc: *mut super::ddraw::DDSURFACEDESC) -> windows_core::Result<()>;
    fn GetFourCCCodes(&self, pcount: *mut u32, pcodes: *mut u32) -> windows_core::Result<()>;
    fn SetDirectDraw(&self, pdirectdraw: windows_core::Ref<super::ddraw::IDirectDraw>) -> windows_core::Result<()>;
    fn GetDirectDraw(&self) -> windows_core::Result<super::ddraw::IDirectDraw>;
    fn GetSurfaceType(&self) -> windows_core::Result<u32>;
    fn SetDefault(&self) -> windows_core::Result<()>;
    fn UseScanLine(&self, usescanline: i32) -> windows_core::Result<()>;
    fn CanUseScanLine(&self) -> windows_core::Result<i32>;
    fn UseOverlayStretch(&self, useoverlaystretch: i32) -> windows_core::Result<()>;
    fn CanUseOverlayStretch(&self) -> windows_core::Result<i32>;
    fn UseWhenFullScreen(&self, usewhenfullscreen: i32) -> windows_core::Result<()>;
    fn WillUseFullScreen(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "ddraw", feature = "ksmedia"))]
impl IDirectDrawVideo_Vtbl {
    pub const fn new<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSwitches<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pswitches: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawVideo_Impl::GetSwitches(this) {
                    Ok(ok__) => {
                        pswitches.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSwitches<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, switches: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::SetSwitches(this, core::mem::transmute_copy(&switches)).into()
            }
        }
        unsafe extern "system" fn GetCaps<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut super::ddraw::DDCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::GetCaps(this, core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetEmulatedCaps<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcaps: *mut super::ddraw::DDCAPS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::GetEmulatedCaps(this, core::mem::transmute_copy(&pcaps)).into()
            }
        }
        unsafe extern "system" fn GetSurfaceDesc<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psurfacedesc: *mut super::ddraw::DDSURFACEDESC) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::GetSurfaceDesc(this, core::mem::transmute_copy(&psurfacedesc)).into()
            }
        }
        unsafe extern "system" fn GetFourCCCodes<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32, pcodes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::GetFourCCCodes(this, core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&pcodes)).into()
            }
        }
        unsafe extern "system" fn SetDirectDraw<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdirectdraw: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::SetDirectDraw(this, core::mem::transmute_copy(&pdirectdraw)).into()
            }
        }
        unsafe extern "system" fn GetDirectDraw<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdirectdraw: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawVideo_Impl::GetDirectDraw(this) {
                    Ok(ok__) => {
                        ppdirectdraw.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSurfaceType<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psurfacetype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawVideo_Impl::GetSurfaceType(this) {
                    Ok(ok__) => {
                        psurfacetype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefault<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::SetDefault(this).into()
            }
        }
        unsafe extern "system" fn UseScanLine<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usescanline: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::UseScanLine(this, core::mem::transmute_copy(&usescanline)).into()
            }
        }
        unsafe extern "system" fn CanUseScanLine<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usescanline: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawVideo_Impl::CanUseScanLine(this) {
                    Ok(ok__) => {
                        usescanline.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UseOverlayStretch<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useoverlaystretch: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::UseOverlayStretch(this, core::mem::transmute_copy(&useoverlaystretch)).into()
            }
        }
        unsafe extern "system" fn CanUseOverlayStretch<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, useoverlaystretch: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawVideo_Impl::CanUseOverlayStretch(this) {
                    Ok(ok__) => {
                        useoverlaystretch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UseWhenFullScreen<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usewhenfullscreen: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDirectDrawVideo_Impl::UseWhenFullScreen(this, core::mem::transmute_copy(&usewhenfullscreen)).into()
            }
        }
        unsafe extern "system" fn WillUseFullScreen<Identity: IDirectDrawVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, usewhenfullscreen: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDirectDrawVideo_Impl::WillUseFullScreen(this) {
                    Ok(ok__) => {
                        usewhenfullscreen.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSwitches: GetSwitches::<Identity, OFFSET>,
            SetSwitches: SetSwitches::<Identity, OFFSET>,
            GetCaps: GetCaps::<Identity, OFFSET>,
            GetEmulatedCaps: GetEmulatedCaps::<Identity, OFFSET>,
            GetSurfaceDesc: GetSurfaceDesc::<Identity, OFFSET>,
            GetFourCCCodes: GetFourCCCodes::<Identity, OFFSET>,
            SetDirectDraw: SetDirectDraw::<Identity, OFFSET>,
            GetDirectDraw: GetDirectDraw::<Identity, OFFSET>,
            GetSurfaceType: GetSurfaceType::<Identity, OFFSET>,
            SetDefault: SetDefault::<Identity, OFFSET>,
            UseScanLine: UseScanLine::<Identity, OFFSET>,
            CanUseScanLine: CanUseScanLine::<Identity, OFFSET>,
            UseOverlayStretch: UseOverlayStretch::<Identity, OFFSET>,
            CanUseOverlayStretch: CanUseOverlayStretch::<Identity, OFFSET>,
            UseWhenFullScreen: UseWhenFullScreen::<Identity, OFFSET>,
            WillUseFullScreen: WillUseFullScreen::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDirectDrawVideo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "ddraw", feature = "ksmedia"))]
impl windows_core::RuntimeName for IDirectDrawVideo {}
windows_core::imp::define_interface!(IFullScreenVideo, IFullScreenVideo_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IFullScreenVideo, windows_core::IUnknown);
impl IFullScreenVideo {
    pub unsafe fn CountModes(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CountModes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetModeInfo(&self, mode: i32, pwidth: *mut i32, pheight: *mut i32, pdepth: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetModeInfo)(windows_core::Interface::as_raw(self), mode, pwidth as _, pheight as _, pdepth as _) }
    }
    pub unsafe fn GetCurrentMode(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsModeAvailable(&self, mode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsModeAvailable)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn IsModeEnabled(&self, mode: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsModeEnabled)(windows_core::Interface::as_raw(self), mode) }
    }
    pub unsafe fn SetEnabled(&self, mode: i32, benabled: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEnabled)(windows_core::Interface::as_raw(self), mode, benabled) }
    }
    pub unsafe fn GetClipFactor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClipFactor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClipFactor(&self, clipfactor: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClipFactor)(windows_core::Interface::as_raw(self), clipfactor) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetMessageDrain(&self, hwnd: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMessageDrain)(windows_core::Interface::as_raw(self), hwnd) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetMessageDrain(&self) -> windows_core::Result<super::windef::HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMessageDrain)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMonitor(&self, monitor: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMonitor)(windows_core::Interface::as_raw(self), monitor) }
    }
    pub unsafe fn GetMonitor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMonitor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HideOnDeactivate(&self, hide: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HideOnDeactivate)(windows_core::Interface::as_raw(self), hide) }
    }
    pub unsafe fn IsHideOnDeactivate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsHideOnDeactivate)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetCaption(&self, strcaption: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCaption)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(strcaption)) }
    }
    pub unsafe fn GetCaption(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaption)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetDefault(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefault)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenVideo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CountModes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetModeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut i32, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub GetCurrentMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub IsModeAvailable: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsModeEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    pub GetClipFactor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetClipFactor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetMessageDrain: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetMessageDrain: usize,
    #[cfg(feature = "windef")]
    pub GetMessageDrain: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetMessageDrain: usize,
    pub SetMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub GetMonitor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub HideOnDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsHideOnDeactivate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCaption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaption: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefault: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IFullScreenVideo_Impl: windows_core::IUnknownImpl {
    fn CountModes(&self) -> windows_core::Result<i32>;
    fn GetModeInfo(&self, mode: i32, pwidth: *mut i32, pheight: *mut i32, pdepth: *mut i32) -> windows_core::Result<()>;
    fn GetCurrentMode(&self) -> windows_core::Result<i32>;
    fn IsModeAvailable(&self, mode: i32) -> windows_core::Result<()>;
    fn IsModeEnabled(&self, mode: i32) -> windows_core::Result<()>;
    fn SetEnabled(&self, mode: i32, benabled: i32) -> windows_core::Result<()>;
    fn GetClipFactor(&self) -> windows_core::Result<i32>;
    fn SetClipFactor(&self, clipfactor: i32) -> windows_core::Result<()>;
    fn SetMessageDrain(&self, hwnd: super::windef::HWND) -> windows_core::Result<()>;
    fn GetMessageDrain(&self) -> windows_core::Result<super::windef::HWND>;
    fn SetMonitor(&self, monitor: i32) -> windows_core::Result<()>;
    fn GetMonitor(&self) -> windows_core::Result<i32>;
    fn HideOnDeactivate(&self, hide: i32) -> windows_core::Result<()>;
    fn IsHideOnDeactivate(&self) -> windows_core::Result<()>;
    fn SetCaption(&self, strcaption: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetCaption(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetDefault(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IFullScreenVideo_Vtbl {
    pub const fn new<Identity: IFullScreenVideo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CountModes<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmodes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFullScreenVideo_Impl::CountModes(this) {
                    Ok(ok__) => {
                        pmodes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetModeInfo<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32, pwidth: *mut i32, pheight: *mut i32, pdepth: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::GetModeInfo(this, core::mem::transmute_copy(&mode), core::mem::transmute_copy(&pwidth), core::mem::transmute_copy(&pheight), core::mem::transmute_copy(&pdepth)).into()
            }
        }
        unsafe extern "system" fn GetCurrentMode<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFullScreenVideo_Impl::GetCurrentMode(this) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsModeAvailable<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::IsModeAvailable(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn IsModeEnabled<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::IsModeEnabled(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: i32, benabled: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::SetEnabled(this, core::mem::transmute_copy(&mode), core::mem::transmute_copy(&benabled)).into()
            }
        }
        unsafe extern "system" fn GetClipFactor<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclipfactor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFullScreenVideo_Impl::GetClipFactor(this) {
                    Ok(ok__) => {
                        pclipfactor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClipFactor<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clipfactor: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::SetClipFactor(this, core::mem::transmute_copy(&clipfactor)).into()
            }
        }
        unsafe extern "system" fn SetMessageDrain<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::SetMessageDrain(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        unsafe extern "system" fn GetMessageDrain<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: *mut super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFullScreenVideo_Impl::GetMessageDrain(this) {
                    Ok(ok__) => {
                        hwnd.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMonitor<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitor: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::SetMonitor(this, core::mem::transmute_copy(&monitor)).into()
            }
        }
        unsafe extern "system" fn GetMonitor<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, monitor: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFullScreenVideo_Impl::GetMonitor(this) {
                    Ok(ok__) => {
                        monitor.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HideOnDeactivate<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hide: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::HideOnDeactivate(this, core::mem::transmute_copy(&hide)).into()
            }
        }
        unsafe extern "system" fn IsHideOnDeactivate<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::IsHideOnDeactivate(this).into()
            }
        }
        unsafe extern "system" fn SetCaption<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, strcaption: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::SetCaption(this, core::mem::transmute(&strcaption)).into()
            }
        }
        unsafe extern "system" fn GetCaption<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstrcaption: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFullScreenVideo_Impl::GetCaption(this) {
                    Ok(ok__) => {
                        pstrcaption.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefault<Identity: IFullScreenVideo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideo_Impl::SetDefault(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CountModes: CountModes::<Identity, OFFSET>,
            GetModeInfo: GetModeInfo::<Identity, OFFSET>,
            GetCurrentMode: GetCurrentMode::<Identity, OFFSET>,
            IsModeAvailable: IsModeAvailable::<Identity, OFFSET>,
            IsModeEnabled: IsModeEnabled::<Identity, OFFSET>,
            SetEnabled: SetEnabled::<Identity, OFFSET>,
            GetClipFactor: GetClipFactor::<Identity, OFFSET>,
            SetClipFactor: SetClipFactor::<Identity, OFFSET>,
            SetMessageDrain: SetMessageDrain::<Identity, OFFSET>,
            GetMessageDrain: GetMessageDrain::<Identity, OFFSET>,
            SetMonitor: SetMonitor::<Identity, OFFSET>,
            GetMonitor: GetMonitor::<Identity, OFFSET>,
            HideOnDeactivate: HideOnDeactivate::<Identity, OFFSET>,
            IsHideOnDeactivate: IsHideOnDeactivate::<Identity, OFFSET>,
            SetCaption: SetCaption::<Identity, OFFSET>,
            GetCaption: GetCaption::<Identity, OFFSET>,
            SetDefault: SetDefault::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFullScreenVideo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IFullScreenVideo {}
windows_core::imp::define_interface!(IFullScreenVideoEx, IFullScreenVideoEx_Vtbl, 0);
impl core::ops::Deref for IFullScreenVideoEx {
    type Target = IFullScreenVideo;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IFullScreenVideoEx, windows_core::IUnknown, IFullScreenVideo);
impl IFullScreenVideoEx {
    #[cfg(feature = "windef")]
    pub unsafe fn SetAcceleratorTable(&self, hwnd: super::windef::HWND, haccel: super::windef::HACCEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAcceleratorTable)(windows_core::Interface::as_raw(self), hwnd, haccel) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetAcceleratorTable(&self, phwnd: *mut super::windef::HWND, phaccel: *mut super::windef::HACCEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAcceleratorTable)(windows_core::Interface::as_raw(self), phwnd as _, phaccel as _) }
    }
    pub unsafe fn KeepPixelAspectRatio(&self, keepaspect: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).KeepPixelAspectRatio)(windows_core::Interface::as_raw(self), keepaspect) }
    }
    pub unsafe fn IsKeepPixelAspectRatio(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsKeepPixelAspectRatio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFullScreenVideoEx_Vtbl {
    pub base__: IFullScreenVideo_Vtbl,
    #[cfg(feature = "windef")]
    pub SetAcceleratorTable: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, super::windef::HACCEL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetAcceleratorTable: usize,
    #[cfg(feature = "windef")]
    pub GetAcceleratorTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::HWND, *mut super::windef::HACCEL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetAcceleratorTable: usize,
    pub KeepPixelAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsKeepPixelAspectRatio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IFullScreenVideoEx_Impl: IFullScreenVideo_Impl {
    fn SetAcceleratorTable(&self, hwnd: super::windef::HWND, haccel: super::windef::HACCEL) -> windows_core::Result<()>;
    fn GetAcceleratorTable(&self, phwnd: *mut super::windef::HWND, phaccel: *mut super::windef::HACCEL) -> windows_core::Result<()>;
    fn KeepPixelAspectRatio(&self, keepaspect: i32) -> windows_core::Result<()>;
    fn IsKeepPixelAspectRatio(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "windef")]
impl IFullScreenVideoEx_Vtbl {
    pub const fn new<Identity: IFullScreenVideoEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAcceleratorTable<Identity: IFullScreenVideoEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, haccel: super::windef::HACCEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideoEx_Impl::SetAcceleratorTable(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&haccel)).into()
            }
        }
        unsafe extern "system" fn GetAcceleratorTable<Identity: IFullScreenVideoEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phwnd: *mut super::windef::HWND, phaccel: *mut super::windef::HACCEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideoEx_Impl::GetAcceleratorTable(this, core::mem::transmute_copy(&phwnd), core::mem::transmute_copy(&phaccel)).into()
            }
        }
        unsafe extern "system" fn KeepPixelAspectRatio<Identity: IFullScreenVideoEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, keepaspect: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IFullScreenVideoEx_Impl::KeepPixelAspectRatio(this, core::mem::transmute_copy(&keepaspect)).into()
            }
        }
        unsafe extern "system" fn IsKeepPixelAspectRatio<Identity: IFullScreenVideoEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeepaspect: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IFullScreenVideoEx_Impl::IsKeepPixelAspectRatio(this) {
                    Ok(ok__) => {
                        pkeepaspect.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IFullScreenVideo_Vtbl::new::<Identity, OFFSET>(),
            SetAcceleratorTable: SetAcceleratorTable::<Identity, OFFSET>,
            GetAcceleratorTable: GetAcceleratorTable::<Identity, OFFSET>,
            KeepPixelAspectRatio: KeepPixelAspectRatio::<Identity, OFFSET>,
            IsKeepPixelAspectRatio: IsKeepPixelAspectRatio::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IFullScreenVideoEx as windows_core::Interface>::IID || iid == &<IFullScreenVideo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IFullScreenVideoEx {}
windows_core::imp::define_interface!(IQualProp, IQualProp_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IQualProp, windows_core::IUnknown);
impl IQualProp {
    pub unsafe fn get_FramesDroppedInRenderer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FramesDroppedInRenderer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_FramesDrawn(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FramesDrawn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_AvgFrameRate(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AvgFrameRate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Jitter(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Jitter)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_AvgSyncOffset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AvgSyncOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_DevSyncOffset(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DevSyncOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IQualProp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_FramesDroppedInRenderer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_FramesDrawn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_AvgFrameRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Jitter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_AvgSyncOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_DevSyncOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IQualProp_Impl: windows_core::IUnknownImpl {
    fn get_FramesDroppedInRenderer(&self) -> windows_core::Result<i32>;
    fn get_FramesDrawn(&self) -> windows_core::Result<i32>;
    fn get_AvgFrameRate(&self) -> windows_core::Result<i32>;
    fn get_Jitter(&self) -> windows_core::Result<i32>;
    fn get_AvgSyncOffset(&self) -> windows_core::Result<i32>;
    fn get_DevSyncOffset(&self) -> windows_core::Result<i32>;
}
impl IQualProp_Vtbl {
    pub const fn new<Identity: IQualProp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_FramesDroppedInRenderer<Identity: IQualProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcframes: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQualProp_Impl::get_FramesDroppedInRenderer(this) {
                    Ok(ok__) => {
                        pcframes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FramesDrawn<Identity: IQualProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcframesdrawn: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQualProp_Impl::get_FramesDrawn(this) {
                    Ok(ok__) => {
                        pcframesdrawn.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AvgFrameRate<Identity: IQualProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piavgframerate: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQualProp_Impl::get_AvgFrameRate(this) {
                    Ok(ok__) => {
                        piavgframerate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Jitter<Identity: IQualProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ijitter: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQualProp_Impl::get_Jitter(this) {
                    Ok(ok__) => {
                        ijitter.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AvgSyncOffset<Identity: IQualProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piavg: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQualProp_Impl::get_AvgSyncOffset(this) {
                    Ok(ok__) => {
                        piavg.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DevSyncOffset<Identity: IQualProp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pidev: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IQualProp_Impl::get_DevSyncOffset(this) {
                    Ok(ok__) => {
                        pidev.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_FramesDroppedInRenderer: get_FramesDroppedInRenderer::<Identity, OFFSET>,
            get_FramesDrawn: get_FramesDrawn::<Identity, OFFSET>,
            get_AvgFrameRate: get_AvgFrameRate::<Identity, OFFSET>,
            get_Jitter: get_Jitter::<Identity, OFFSET>,
            get_AvgSyncOffset: get_AvgSyncOffset::<Identity, OFFSET>,
            get_DevSyncOffset: get_DevSyncOffset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IQualProp as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IQualProp {}
pub const MAX_SIZE_MPEG1_SEQUENCE_INFO: u32 = 140;
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MPEG1VIDEOINFO {
    pub hdr: VIDEOINFOHEADER,
    pub dwStartTimeCode: u32,
    pub cbSequenceHeader: u32,
    pub bSequenceHeader: [u8; 1],
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi"))]
impl Default for MPEG1VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIZE_PREHEADER: u32 = 48;
#[repr(C)]
#[cfg(feature = "wingdi")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TRUECOLORINFO {
    pub dwBitMasks: [u32; 3],
    pub bmiColors: [super::wingdi::RGBQUAD; 256],
}
#[cfg(feature = "wingdi")]
impl Default for TRUECOLORINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub struct VIDEOINFO {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::ksmedia::REFERENCE_TIME,
    pub bmiHeader: super::wingdi::BITMAPINFOHEADER,
    pub Anonymous: VIDEOINFO_0,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi"))]
impl Default for VIDEOINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy)]
pub union VIDEOINFO_0 {
    pub bmiColors: [super::wingdi::RGBQUAD; 256],
    pub dwBitMasks: [u32; 3],
    pub TrueColorInfo: TRUECOLORINFO,
}
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi"))]
impl Default for VIDEOINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "ksmedia", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIDEOINFOHEADER {
    pub rcSource: super::windef::RECT,
    pub rcTarget: super::windef::RECT,
    pub dwBitRate: u32,
    pub dwBitErrorRate: u32,
    pub AvgTimePerFrame: super::ksmedia::REFERENCE_TIME,
    pub bmiHeader: super::wingdi::BITMAPINFOHEADER,
}
pub const iBLUE: u32 = 2;
pub const iEGA_COLORS: u32 = 16;
pub const iGREEN: u32 = 1;
pub const iMASK_COLORS: u32 = 3;
pub const iMAXBITS: u32 = 8;
pub const iPALETTE: u32 = 8;
pub const iPALETTE_COLORS: u32 = 256;
pub const iRED: u32 = 0;
pub const iTRUECOLOR: u32 = 16;
