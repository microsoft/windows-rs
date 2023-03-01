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
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[inline]
pub unsafe fn timeBeginPeriod(uperiod: u32) -> u32 {
    ::windows::imp::link ! ( "winmm.dll""system" fn timeBeginPeriod ( uperiod : u32 ) -> u32 );
    timeBeginPeriod(uperiod)
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[inline]
pub unsafe fn timeEndPeriod(uperiod: u32) -> u32 {
    ::windows::imp::link ! ( "winmm.dll""system" fn timeEndPeriod ( uperiod : u32 ) -> u32 );
    timeEndPeriod(uperiod)
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[inline]
pub unsafe fn timeGetDevCaps(ptc: *mut TIMECAPS, cbtc: u32) -> u32 {
    ::windows::imp::link ! ( "winmm.dll""system" fn timeGetDevCaps ( ptc : *mut TIMECAPS , cbtc : u32 ) -> u32 );
    timeGetDevCaps(ptc, cbtc)
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[inline]
pub unsafe fn timeGetSystemTime(pmmt: *mut MMTIME, cbmmt: u32) -> u32 {
    ::windows::imp::link ! ( "winmm.dll""system" fn timeGetSystemTime ( pmmt : *mut MMTIME , cbmmt : u32 ) -> u32 );
    timeGetSystemTime(pmmt, cbmmt)
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[inline]
pub unsafe fn timeGetTime() -> u32 {
    ::windows::imp::link ! ( "winmm.dll""system" fn timeGetTime ( ) -> u32 );
    timeGetTime()
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[inline]
pub unsafe fn timeKillEvent(utimerid: u32) -> u32 {
    ::windows::imp::link ! ( "winmm.dll""system" fn timeKillEvent ( utimerid : u32 ) -> u32 );
    timeKillEvent(utimerid)
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[inline]
pub unsafe fn timeSetEvent(udelay: u32, uresolution: u32, fptc: LPTIMECALLBACK, dwuser: usize, fuevent: u32) -> u32 {
    ::windows::imp::link ! ( "winmm.dll""system" fn timeSetEvent ( udelay : u32 , uresolution : u32 , fptc : LPTIMECALLBACK , dwuser : usize , fuevent : u32 ) -> u32 );
    timeSetEvent(udelay, uresolution, fptc, dwuser, fuevent)
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[repr(transparent)]
pub struct IReferenceClock(::windows::core::IUnknown);
impl IReferenceClock {
    pub unsafe fn GetTime(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).GetTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdviseTime<P0>(&self, basetime: i64, streamtime: i64, hevent: P0) -> ::windows::core::Result<usize>
    where
        P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<usize>();
        (::windows::core::Interface::vtable(self).AdviseTime)(::windows::core::Interface::as_raw(self), basetime, streamtime, hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdvisePeriodic<P0>(&self, starttime: i64, periodtime: i64, hsemaphore: P0) -> ::windows::core::Result<usize>
    where
        P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<usize>();
        (::windows::core::Interface::vtable(self).AdvisePeriodic)(::windows::core::Interface::as_raw(self), starttime, periodtime, hsemaphore.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwadvisecookie: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Unadvise)(::windows::core::Interface::as_raw(self), dwadvisecookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IReferenceClock, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IReferenceClock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceClock {}
impl ::core::fmt::Debug for IReferenceClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceClock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceClock {
    type Vtable = IReferenceClock_Vtbl;
}
impl ::core::clone::Clone for IReferenceClock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IReferenceClock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56a86897_0ad4_11ce_b03a_0020af0ba770);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptime: *mut i64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AdviseTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, basetime: i64, streamtime: i64, hevent: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdviseTime: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AdvisePeriodic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, starttime: i64, periodtime: i64, hsemaphore: super::Foundation::HANDLE, pdwadvisecookie: *mut usize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AdvisePeriodic: usize,
    pub Unadvise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwadvisecookie: usize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[repr(transparent)]
pub struct IReferenceClock2(::windows::core::IUnknown);
impl IReferenceClock2 {
    pub unsafe fn GetTime(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).base__.GetTime)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdviseTime<P0>(&self, basetime: i64, streamtime: i64, hevent: P0) -> ::windows::core::Result<usize>
    where
        P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<usize>();
        (::windows::core::Interface::vtable(self).base__.AdviseTime)(::windows::core::Interface::as_raw(self), basetime, streamtime, hevent.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AdvisePeriodic<P0>(&self, starttime: i64, periodtime: i64, hsemaphore: P0) -> ::windows::core::Result<usize>
    where
        P0: ::windows::core::IntoParam<super::Foundation::HANDLE>,
    {
        let mut result__ = ::windows::core::zeroed::<usize>();
        (::windows::core::Interface::vtable(self).base__.AdvisePeriodic)(::windows::core::Interface::as_raw(self), starttime, periodtime, hsemaphore.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Unadvise(&self, dwadvisecookie: usize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Unadvise)(::windows::core::Interface::as_raw(self), dwadvisecookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IReferenceClock2, ::windows::core::IUnknown, IReferenceClock);
impl ::core::cmp::PartialEq for IReferenceClock2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceClock2 {}
impl ::core::fmt::Debug for IReferenceClock2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceClock2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceClock2 {
    type Vtable = IReferenceClock2_Vtbl;
}
impl ::core::clone::Clone for IReferenceClock2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IReferenceClock2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36b73885_c2c8_11cf_8b46_00805f6cef60);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClock2_Vtbl {
    pub base__: IReferenceClock_Vtbl,
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[repr(transparent)]
pub struct IReferenceClockTimerControl(::windows::core::IUnknown);
impl IReferenceClockTimerControl {
    pub unsafe fn SetDefaultTimerResolution(&self, timerresolution: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultTimerResolution)(::windows::core::Interface::as_raw(self), timerresolution).ok()
    }
    pub unsafe fn GetDefaultTimerResolution(&self) -> ::windows::core::Result<i64> {
        let mut result__ = ::windows::core::zeroed::<i64>();
        (::windows::core::Interface::vtable(self).GetDefaultTimerResolution)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IReferenceClockTimerControl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IReferenceClockTimerControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReferenceClockTimerControl {}
impl ::core::fmt::Debug for IReferenceClockTimerControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReferenceClockTimerControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReferenceClockTimerControl {
    type Vtable = IReferenceClockTimerControl_Vtbl;
}
impl ::core::clone::Clone for IReferenceClockTimerControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IReferenceClockTimerControl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebec459c_2eca_4d42_a8af_30df557614b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceClockTimerControl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetDefaultTimerResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, timerresolution: i64) -> ::windows::core::HRESULT,
    pub GetDefaultTimerResolution: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptimerresolution: *mut i64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const JOYERR_BASE: u32 = 160u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MAXERRORLENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MAXPNAMELEN: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MCIERR_BASE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MCI_CD_OFFSET: u32 = 1088u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MCI_SEQ_OFFSET: u32 = 1216u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MCI_STRING_OFFSET: u32 = 512u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MCI_VD_OFFSET: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MCI_WAVE_OFFSET: u32 = 1152u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MIDIERR_BASE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MIXERR_BASE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_ALLOCATED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_BADDB: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_BADDEVICEID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_BADERRNUM: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_DELETEERROR: u32 = 18u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_ERROR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_HANDLEBUSY: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_INVALFLAG: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_INVALHANDLE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_INVALIDALIAS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_INVALPARAM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_KEYNOTFOUND: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_LASTERROR: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_MOREDATA: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_NODRIVER: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_NODRIVERCB: u32 = 20u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_NOERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_NOMEM: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_NOTENABLED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_NOTSUPPORTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_READERROR: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_VALNOTFOUND: u32 = 19u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MMSYSERR_WRITEERROR: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_ADLIB: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_DRVM_CLOSE: u32 = 977u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_DRVM_DATA: u32 = 978u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_DRVM_ERROR: u32 = 979u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_DRVM_OPEN: u32 = 976u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_JOY1BUTTONDOWN: u32 = 949u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_JOY1BUTTONUP: u32 = 951u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_JOY1MOVE: u32 = 928u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_JOY1ZMOVE: u32 = 930u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_JOY2BUTTONDOWN: u32 = 950u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_JOY2BUTTONUP: u32 = 952u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_JOY2MOVE: u32 = 929u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_JOY2ZMOVE: u32 = 931u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MCINOTIFY: u32 = 953u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MCISIGNAL: u32 = 971u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MICROSOFT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIDI_MAPPER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIM_CLOSE: u32 = 962u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIM_DATA: u32 = 963u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIM_ERROR: u32 = 965u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIM_LONGDATA: u32 = 964u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIM_LONGERROR: u32 = 966u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIM_MOREDATA: u32 = 972u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIM_OPEN: u32 = 961u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIXM_CONTROL_CHANGE: u32 = 977u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MIXM_LINE_CHANGE: u32 = 976u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MOM_CLOSE: u32 = 968u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MOM_DONE: u32 = 969u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MOM_OPEN: u32 = 967u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MOM_POSITIONCB: u32 = 970u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MPU401_MIDIIN: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_MPU401_MIDIOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_PC_JOYSTICK: u32 = 12u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_SNDBLST_MIDIIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_SNDBLST_MIDIOUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_SNDBLST_SYNTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_SNDBLST_WAVEIN: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_SNDBLST_WAVEOUT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_STREAM_CLOSE: u32 = 981u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_STREAM_DONE: u32 = 982u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_STREAM_ERROR: u32 = 983u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_STREAM_OPEN: u32 = 980u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_WAVE_MAPPER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_WIM_CLOSE: u32 = 959u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_WIM_DATA: u32 = 960u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_WIM_OPEN: u32 = 958u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_WOM_CLOSE: u32 = 956u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_WOM_DONE: u32 = 957u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const MM_WOM_OPEN: u32 = 955u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIMERR_BASE: u32 = 96u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIMERR_NOCANDO: u32 = 97u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIMERR_NOERROR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIMERR_STRUCT: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_BYTES: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_CALLBACK_EVENT_PULSE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_CALLBACK_EVENT_SET: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_CALLBACK_FUNCTION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_KILL_SYNCHRONOUS: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_MIDI: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_MS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_ONESHOT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_PERIODIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_SAMPLES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_SMPTE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const TIME_TICKS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const WAVERR_BASE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Media\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TIMECODE_SAMPLE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const ED_DEVCAP_TIMECODE_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(4121u32);
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const ED_DEVCAP_ATN_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(5047u32);
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub const ED_DEVCAP_RTC_READ: TIMECODE_SAMPLE_FLAGS = TIMECODE_SAMPLE_FLAGS(5050u32);
impl ::core::marker::Copy for TIMECODE_SAMPLE_FLAGS {}
impl ::core::clone::Clone for TIMECODE_SAMPLE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TIMECODE_SAMPLE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for TIMECODE_SAMPLE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for TIMECODE_SAMPLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TIMECODE_SAMPLE_FLAGS").field(&self.0).finish()
    }
}
impl TIMECODE_SAMPLE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for TIMECODE_SAMPLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for TIMECODE_SAMPLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HTASK(pub isize);
impl HTASK {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HTASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HTASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HTASK {}
impl ::core::fmt::Debug for HTASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HTASK").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HTASK {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub struct MMTIME {
    pub wType: u32,
    pub u: MMTIME_0,
}
impl ::core::marker::Copy for MMTIME {}
impl ::core::clone::Clone for MMTIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MMTIME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MMTIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub union MMTIME_0 {
    pub ms: u32,
    pub sample: u32,
    pub cb: u32,
    pub ticks: u32,
    pub smpte: MMTIME_0_1,
    pub midi: MMTIME_0_0,
}
impl ::core::marker::Copy for MMTIME_0 {}
impl ::core::clone::Clone for MMTIME_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MMTIME_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MMTIME_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub struct MMTIME_0_0 {
    pub songptrpos: u32,
}
impl ::core::marker::Copy for MMTIME_0_0 {}
impl ::core::clone::Clone for MMTIME_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MMTIME_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MMTIME_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub struct MMTIME_0_1 {
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub frame: u8,
    pub fps: u8,
    pub dummy: u8,
    pub pad: [u8; 2],
}
impl ::core::marker::Copy for MMTIME_0_1 {}
impl ::core::clone::Clone for MMTIME_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMTIME_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMTIME_0_1").field("hour", &self.hour).field("min", &self.min).field("sec", &self.sec).field("frame", &self.frame).field("fps", &self.fps).field("dummy", &self.dummy).field("pad", &self.pad).finish()
    }
}
impl ::windows::core::TypeKind for MMTIME_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMTIME_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hour == other.hour && self.min == other.min && self.sec == other.sec && self.frame == other.frame && self.fps == other.fps && self.dummy == other.dummy && self.pad == other.pad
    }
}
impl ::core::cmp::Eq for MMTIME_0_1 {}
impl ::core::default::Default for MMTIME_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub struct TIMECAPS {
    pub wPeriodMin: u32,
    pub wPeriodMax: u32,
}
impl ::core::marker::Copy for TIMECAPS {}
impl ::core::clone::Clone for TIMECAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIMECAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMECAPS").field("wPeriodMin", &self.wPeriodMin).field("wPeriodMax", &self.wPeriodMax).finish()
    }
}
impl ::windows::core::TypeKind for TIMECAPS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TIMECAPS {
    fn eq(&self, other: &Self) -> bool {
        self.wPeriodMin == other.wPeriodMin && self.wPeriodMax == other.wPeriodMax
    }
}
impl ::core::cmp::Eq for TIMECAPS {}
impl ::core::default::Default for TIMECAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub union TIMECODE {
    pub Anonymous: TIMECODE_0,
    pub qw: u64,
}
impl ::core::marker::Copy for TIMECODE {}
impl ::core::clone::Clone for TIMECODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TIMECODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TIMECODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub struct TIMECODE_0 {
    pub wFrameRate: u16,
    pub wFrameFract: u16,
    pub dwFrames: u32,
}
impl ::core::marker::Copy for TIMECODE_0 {}
impl ::core::clone::Clone for TIMECODE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TIMECODE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TIMECODE_0").field("wFrameRate", &self.wFrameRate).field("wFrameFract", &self.wFrameFract).field("dwFrames", &self.dwFrames).finish()
    }
}
impl ::windows::core::TypeKind for TIMECODE_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for TIMECODE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wFrameRate == other.wFrameRate && self.wFrameFract == other.wFrameFract && self.dwFrames == other.dwFrames
    }
}
impl ::core::cmp::Eq for TIMECODE_0 {}
impl ::core::default::Default for TIMECODE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub struct TIMECODE_SAMPLE {
    pub qwTick: i64,
    pub timecode: TIMECODE,
    pub dwUser: u32,
    pub dwFlags: TIMECODE_SAMPLE_FLAGS,
}
impl ::core::marker::Copy for TIMECODE_SAMPLE {}
impl ::core::clone::Clone for TIMECODE_SAMPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for TIMECODE_SAMPLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for TIMECODE_SAMPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Media\"`, `\"Win32_Media_Multimedia\"`*"]
#[cfg(feature = "Win32_Media_Multimedia")]
pub type LPDRVCALLBACK = ::core::option::Option<unsafe extern "system" fn(hdrvr: Multimedia::HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize) -> ()>;
#[doc = "*Required features: `\"Win32_Media\"`*"]
pub type LPTIMECALLBACK = ::core::option::Option<unsafe extern "system" fn(utimerid: u32, umsg: u32, dwuser: usize, dw1: usize, dw2: usize) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
