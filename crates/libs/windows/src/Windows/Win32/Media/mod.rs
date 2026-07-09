#[cfg(feature = "Win32_Media_Audio")]
pub mod Audio;
#[cfg(feature = "Win32_Media_DeviceManager")]
pub mod DeviceManager;
#[cfg(feature = "Win32_Media_DirectShow")]
pub mod DirectShow;
#[cfg(feature = "Win32_Media_DxMediaObjects")]
pub mod DxMediaObjects;
#[cfg(feature = "Win32_Media_KernelStreaming")]
pub mod KernelStreaming;
#[cfg(feature = "Win32_Media_LibrarySharingServices")]
pub mod LibrarySharingServices;
#[cfg(feature = "Win32_Media_MediaFoundation")]
pub mod MediaFoundation;
#[cfg(feature = "Win32_Media_MediaPlayer")]
pub mod MediaPlayer;
#[cfg(feature = "Win32_Media_Multimedia")]
pub mod Multimedia;
#[cfg(feature = "Win32_Media_PictureAcquisition")]
pub mod PictureAcquisition;
#[cfg(feature = "Win32_Media_Speech")]
pub mod Speech;
#[cfg(feature = "Win32_Media_Streaming")]
pub mod Streaming;
#[cfg(feature = "Win32_Media_WindowsMediaFormat")]
pub mod WindowsMediaFormat;
#[inline]
pub unsafe fn timeBeginPeriod(uperiod: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn timeBeginPeriod(uperiod : u32) -> u32);
    unsafe { timeBeginPeriod(uperiod) }
}
#[inline]
pub unsafe fn timeEndPeriod(uperiod: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn timeEndPeriod(uperiod : u32) -> u32);
    unsafe { timeEndPeriod(uperiod) }
}
#[inline]
pub unsafe fn timeGetDevCaps(ptc: *mut TIMECAPS, cbtc: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn timeGetDevCaps(ptc : *mut TIMECAPS, cbtc : u32) -> u32);
    unsafe { timeGetDevCaps(ptc as _, cbtc) }
}
#[inline]
pub unsafe fn timeGetSystemTime(pmmt: *mut MMTIME, cbmmt: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn timeGetSystemTime(pmmt : *mut MMTIME, cbmmt : u32) -> u32);
    unsafe { timeGetSystemTime(pmmt as _, cbmmt) }
}
#[inline]
pub unsafe fn timeGetTime() -> u32 {
    windows_core::link!("winmm.dll" "system" fn timeGetTime() -> u32);
    unsafe { timeGetTime() }
}
#[inline]
pub unsafe fn timeKillEvent(utimerid: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn timeKillEvent(utimerid : u32) -> u32);
    unsafe { timeKillEvent(utimerid) }
}
#[inline]
pub unsafe fn timeSetEvent(udelay: u32, uresolution: u32, fptc: LPTIMECALLBACK, dwuser: usize, fuevent: u32) -> u32 {
    windows_core::link!("winmm.dll" "system" fn timeSetEvent(udelay : u32, uresolution : u32, fptc : LPTIMECALLBACK, dwuser : usize, fuevent : u32) -> u32);
    unsafe { timeSetEvent(udelay, uresolution, fptc, dwuser, fuevent) }
}
pub const ED_DEVCAP_ATN_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(5047);
pub const ED_DEVCAP_RTC_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(5050);
pub const ED_DEVCAP_TIMECODE_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(4121);
windows_core::imp::define_interface!(IReferenceClock, IReferenceClock_Vtbl, 0x56a86897_0ad4_11ce_b03a_0020af0ba770);
windows_core::imp::interface_hierarchy!(IReferenceClock, windows_core::IUnknown);
impl IReferenceClock {
    pub unsafe fn GetTime(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AdviseTime(&self, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdviseTime)(windows_core::Interface::as_raw(self), basetime, streamtime, hevent, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AdvisePeriodic(&self, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE) -> windows_core::Result<usize> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AdvisePeriodic)(windows_core::Interface::as_raw(self), starttime, periodtime, hsemaphore, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Unadvise(&self, dwadvisecookie: usize) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unadvise)(windows_core::Interface::as_raw(self), dwadvisecookie) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub AdviseTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64, i64, super::Foundation::HANDLE, *mut usize) -> windows_core::HRESULT,
    pub AdvisePeriodic: unsafe extern "system" fn(*mut core::ffi::c_void, i64, i64, super::Foundation::HANDLE, *mut usize) -> windows_core::HRESULT,
    pub Unadvise: unsafe extern "system" fn(*mut core::ffi::c_void, usize) -> windows_core::HRESULT,
}
pub trait IReferenceClock_Impl: windows_core::IUnknownImpl {
    fn GetTime(&self) -> windows_core::Result<i64>;
    fn AdviseTime(&self, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE) -> windows_core::Result<usize>;
    fn AdvisePeriodic(&self, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE) -> windows_core::Result<usize>;
    fn Unadvise(&self, dwadvisecookie: usize) -> windows_core::Result<()>;
}
impl IReferenceClock_Vtbl {
    pub const fn new<Identity: IReferenceClock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTime<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptime: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceClock_Impl::GetTime(this) {
                    Ok(ok__) => {
                        ptime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AdviseTime<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceClock_Impl::AdviseTime(this, core::mem::transmute_copy(&basetime), core::mem::transmute_copy(&streamtime), core::mem::transmute_copy(&hevent)) {
                    Ok(ok__) => {
                        pdwadvisecookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AdvisePeriodic<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceClock_Impl::AdvisePeriodic(this, core::mem::transmute_copy(&starttime), core::mem::transmute_copy(&periodtime), core::mem::transmute_copy(&hsemaphore)) {
                    Ok(ok__) => {
                        pdwadvisecookie.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Unadvise<Identity: IReferenceClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwadvisecookie: usize) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceClock_Impl::Unadvise(this, core::mem::transmute_copy(&dwadvisecookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTime: GetTime::<Identity, OFFSET>,
            AdviseTime: AdviseTime::<Identity, OFFSET>,
            AdvisePeriodic: AdvisePeriodic::<Identity, OFFSET>,
            Unadvise: Unadvise::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceClock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IReferenceClock {}
windows_core::imp::define_interface!(IReferenceClock2, IReferenceClock2_Vtbl, 0x36b73885_c2c8_11cf_8b46_00805f6cef60);
impl core::ops::Deref for IReferenceClock2 {
    type Target = IReferenceClock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IReferenceClock2, windows_core::IUnknown, IReferenceClock);
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClock2_Vtbl {
    pub base__: IReferenceClock_Vtbl,
}
pub trait IReferenceClock2_Impl: IReferenceClock_Impl {}
impl IReferenceClock2_Vtbl {
    pub const fn new<Identity: IReferenceClock2_Impl, const OFFSET: isize>() -> Self {
        Self { base__: IReferenceClock_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceClock2 as windows_core::Interface>::IID || iid == &<IReferenceClock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IReferenceClock2 {}
windows_core::imp::define_interface!(IReferenceClockTimerControl, IReferenceClockTimerControl_Vtbl, 0xebec459c_2eca_4d42_a8af_30df557614b8);
windows_core::imp::interface_hierarchy!(IReferenceClockTimerControl, windows_core::IUnknown);
impl IReferenceClockTimerControl {
    pub unsafe fn SetDefaultTimerResolution(&self, timerresolution: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultTimerResolution)(windows_core::Interface::as_raw(self), timerresolution) }
    }
    pub unsafe fn GetDefaultTimerResolution(&self) -> windows_core::Result<i64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultTimerResolution)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClockTimerControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDefaultTimerResolution: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetDefaultTimerResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
}
pub trait IReferenceClockTimerControl_Impl: windows_core::IUnknownImpl {
    fn SetDefaultTimerResolution(&self, timerresolution: i64) -> windows_core::Result<()>;
    fn GetDefaultTimerResolution(&self) -> windows_core::Result<i64>;
}
impl IReferenceClockTimerControl_Vtbl {
    pub const fn new<Identity: IReferenceClockTimerControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDefaultTimerResolution<Identity: IReferenceClockTimerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timerresolution: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IReferenceClockTimerControl_Impl::SetDefaultTimerResolution(this, core::mem::transmute_copy(&timerresolution)).into()
            }
        }
        unsafe extern "system" fn GetDefaultTimerResolution<Identity: IReferenceClockTimerControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimerresolution: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IReferenceClockTimerControl_Impl::GetDefaultTimerResolution(this) {
                    Ok(ok__) => {
                        ptimerresolution.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultTimerResolution: SetDefaultTimerResolution::<Identity, OFFSET>,
            GetDefaultTimerResolution: GetDefaultTimerResolution::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IReferenceClockTimerControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IReferenceClockTimerControl {}
pub const JOYERR_BASE: u32 = 160;
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPDRVCALLBACK = Option<unsafe extern "system" fn(hdrvr: Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub type LPTIMECALLBACK = Option<unsafe extern "system" fn(utimerid: u32, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub const MAXERRORLENGTH: u32 = 256;
pub const MAXPNAMELEN: u32 = 32;
pub const MCIERR_BASE: u32 = 256;
pub const MCI_CD_OFFSET: u32 = 1088;
pub const MCI_SEQ_OFFSET: u32 = 1216;
pub const MCI_STRING_OFFSET: u32 = 512;
pub const MCI_VD_OFFSET: u32 = 1024;
pub const MCI_WAVE_OFFSET: u32 = 1152;
pub const MIDIERR_BASE: u32 = 64;
pub const MIXERR_BASE: u32 = 1024;
pub const MMSYSERR_ALLOCATED: u32 = 4;
pub const MMSYSERR_BADDB: u32 = 14;
pub const MMSYSERR_BADDEVICEID: u32 = 2;
pub const MMSYSERR_BADERRNUM: u32 = 9;
pub const MMSYSERR_BASE: u32 = 0;
pub const MMSYSERR_DELETEERROR: u32 = 18;
pub const MMSYSERR_ERROR: u32 = 1;
pub const MMSYSERR_HANDLEBUSY: u32 = 12;
pub const MMSYSERR_INVALFLAG: u32 = 10;
pub const MMSYSERR_INVALHANDLE: u32 = 5;
pub const MMSYSERR_INVALIDALIAS: u32 = 13;
pub const MMSYSERR_INVALPARAM: u32 = 11;
pub const MMSYSERR_KEYNOTFOUND: u32 = 15;
pub const MMSYSERR_LASTERROR: u32 = 21;
pub const MMSYSERR_MOREDATA: u32 = 21;
pub const MMSYSERR_NODRIVER: u32 = 6;
pub const MMSYSERR_NODRIVERCB: u32 = 20;
pub const MMSYSERR_NOERROR: u32 = 0;
pub const MMSYSERR_NOMEM: u32 = 7;
pub const MMSYSERR_NOTENABLED: u32 = 3;
pub const MMSYSERR_NOTSUPPORTED: u32 = 8;
pub const MMSYSERR_READERROR: u32 = 16;
pub const MMSYSERR_VALNOTFOUND: u32 = 19;
pub const MMSYSERR_WRITEERROR: u32 = 17;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MMTIME {
    pub wType: u32,
    pub u: MMTIME_0,
}
impl Default for MMTIME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MMTIME_0 {
    pub ms: u32,
    pub sample: u32,
    pub cb: u32,
    pub ticks: u32,
    pub smpte: MMTIME_0_0,
    pub midi: MMTIME_0_1,
}
impl Default for MMTIME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MMTIME_0_0 {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub frame: u8,
    pub fps: u8,
    pub dummy: u8,
    pub pad: [u8; 2],
}
impl Default for MMTIME_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MMTIME_0_1 {
    pub songptrpos: u32,
}
pub const MM_ADLIB: u32 = 9;
pub const MM_DRVM_CLOSE: u32 = 977;
pub const MM_DRVM_DATA: u32 = 978;
pub const MM_DRVM_ERROR: u32 = 979;
pub const MM_DRVM_OPEN: u32 = 976;
pub const MM_JOY1BUTTONDOWN: u32 = 949;
pub const MM_JOY1BUTTONUP: u32 = 951;
pub const MM_JOY1MOVE: u32 = 928;
pub const MM_JOY1ZMOVE: u32 = 930;
pub const MM_JOY2BUTTONDOWN: u32 = 950;
pub const MM_JOY2BUTTONUP: u32 = 952;
pub const MM_JOY2MOVE: u32 = 929;
pub const MM_JOY2ZMOVE: u32 = 931;
pub const MM_MCINOTIFY: u32 = 953;
pub const MM_MCISIGNAL: u32 = 971;
pub const MM_MICROSOFT: u32 = 1;
pub const MM_MIDI_MAPPER: u32 = 1;
pub const MM_MIM_CLOSE: u32 = 962;
pub const MM_MIM_DATA: u32 = 963;
pub const MM_MIM_ERROR: u32 = 965;
pub const MM_MIM_LONGDATA: u32 = 964;
pub const MM_MIM_LONGERROR: u32 = 966;
pub const MM_MIM_MOREDATA: u32 = 972;
pub const MM_MIM_OPEN: u32 = 961;
pub const MM_MIXM_CONTROL_CHANGE: u32 = 977;
pub const MM_MIXM_LINE_CHANGE: u32 = 976;
pub const MM_MOM_CLOSE: u32 = 968;
pub const MM_MOM_DONE: u32 = 969;
pub const MM_MOM_OPEN: u32 = 967;
pub const MM_MOM_POSITIONCB: u32 = 970;
pub const MM_MPU401_MIDIIN: u32 = 11;
pub const MM_MPU401_MIDIOUT: u32 = 10;
pub const MM_PC_JOYSTICK: u32 = 12;
pub const MM_SNDBLST_MIDIIN: u32 = 4;
pub const MM_SNDBLST_MIDIOUT: u32 = 3;
pub const MM_SNDBLST_SYNTH: u32 = 5;
pub const MM_SNDBLST_WAVEIN: u32 = 7;
pub const MM_SNDBLST_WAVEOUT: u32 = 6;
pub const MM_STREAM_CLOSE: u32 = 981;
pub const MM_STREAM_DONE: u32 = 982;
pub const MM_STREAM_ERROR: u32 = 983;
pub const MM_STREAM_OPEN: u32 = 980;
pub const MM_WAVE_MAPPER: u32 = 2;
pub const MM_WIM_CLOSE: u32 = 959;
pub const MM_WIM_DATA: u32 = 960;
pub const MM_WIM_OPEN: u32 = 958;
pub const MM_WOM_CLOSE: u32 = 956;
pub const MM_WOM_DONE: u32 = 957;
pub const MM_WOM_OPEN: u32 = 955;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TIMECAPS {
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TIMECODE {
    pub Anonymous: TIMECODE_0,
    pub qw: u64,
}
impl Default for TIMECODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TIMECODE_0 {
    pub wFrameRate: u16,
    pub wFrameFract: u16,
    pub dwFrames: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TIMECODE_SAMPLE {
    pub qwTick: i64,
    pub timecode: TIMECODE,
    pub dwUser: u32,
    pub dwFlags: TIMECODE_SAMPLE_FLAGS,
}
impl Default for TIMECODE_SAMPLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TIMECODE_SAMPLE_FLAGS(pub u32);
impl TIMECODE_SAMPLE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const TIMERR_BASE: u32 = 96;
pub const TIMERR_NOCANDO: u32 = 97;
pub const TIMERR_NOERROR: u32 = 0;
pub const TIMERR_STRUCT: u32 = 129;
pub const TIME_BYTES: u32 = 4;
pub const TIME_CALLBACK_EVENT_PULSE: u32 = 32;
pub const TIME_CALLBACK_EVENT_SET: u32 = 16;
pub const TIME_CALLBACK_FUNCTION: u32 = 0;
pub const TIME_KILL_SYNCHRONOUS: u32 = 256;
pub const TIME_MIDI: u32 = 16;
pub const TIME_MS: u32 = 1;
pub const TIME_ONESHOT: u32 = 0;
pub const TIME_PERIODIC: u32 = 1;
pub const TIME_SAMPLES: u32 = 2;
pub const TIME_SMPTE: u32 = 8;
pub const TIME_TICKS: u32 = 32;
pub const WAVERR_BASE: u32 = 32;
