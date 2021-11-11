#![allow(non_snake_case, non_camel_case_types)]
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
#[link(name = "windows")]
extern "system" {
    fn HTASK();
    fn IReferenceClock();
    fn IReferenceClock2();
    fn IReferenceClockTimerControl();
    fn JOYERR_BASE();
    fn LPDRVCALLBACK();
    fn LPTIMECALLBACK();
    fn MAXERRORLENGTH();
    fn MAXPNAMELEN();
    fn MCIERR_BASE();
    fn MCI_CD_OFFSET();
    fn MCI_SEQ_OFFSET();
    fn MCI_STRING_OFFSET();
    fn MCI_VD_OFFSET();
    fn MCI_WAVE_OFFSET();
    fn MIDIERR_BASE();
    fn MIXERR_BASE();
    fn MMSYSERR_ALLOCATED();
    fn MMSYSERR_BADDB();
    fn MMSYSERR_BADDEVICEID();
    fn MMSYSERR_BADERRNUM();
    fn MMSYSERR_BASE();
    fn MMSYSERR_DELETEERROR();
    fn MMSYSERR_ERROR();
    fn MMSYSERR_HANDLEBUSY();
    fn MMSYSERR_INVALFLAG();
    fn MMSYSERR_INVALHANDLE();
    fn MMSYSERR_INVALIDALIAS();
    fn MMSYSERR_INVALPARAM();
    fn MMSYSERR_KEYNOTFOUND();
    fn MMSYSERR_LASTERROR();
    fn MMSYSERR_MOREDATA();
    fn MMSYSERR_NODRIVER();
    fn MMSYSERR_NODRIVERCB();
    fn MMSYSERR_NOERROR();
    fn MMSYSERR_NOMEM();
    fn MMSYSERR_NOTENABLED();
    fn MMSYSERR_NOTSUPPORTED();
    fn MMSYSERR_READERROR();
    fn MMSYSERR_VALNOTFOUND();
    fn MMSYSERR_WRITEERROR();
    fn MMTIME();
    fn MM_ADLIB();
    fn MM_DRVM_CLOSE();
    fn MM_DRVM_DATA();
    fn MM_DRVM_ERROR();
    fn MM_DRVM_OPEN();
    fn MM_JOY1BUTTONDOWN();
    fn MM_JOY1BUTTONUP();
    fn MM_JOY1MOVE();
    fn MM_JOY1ZMOVE();
    fn MM_JOY2BUTTONDOWN();
    fn MM_JOY2BUTTONUP();
    fn MM_JOY2MOVE();
    fn MM_JOY2ZMOVE();
    fn MM_MCINOTIFY();
    fn MM_MCISIGNAL();
    fn MM_MICROSOFT();
    fn MM_MIDI_MAPPER();
    fn MM_MIM_CLOSE();
    fn MM_MIM_DATA();
    fn MM_MIM_ERROR();
    fn MM_MIM_LONGDATA();
    fn MM_MIM_LONGERROR();
    fn MM_MIM_MOREDATA();
    fn MM_MIM_OPEN();
    fn MM_MIXM_CONTROL_CHANGE();
    fn MM_MIXM_LINE_CHANGE();
    fn MM_MOM_CLOSE();
    fn MM_MOM_DONE();
    fn MM_MOM_OPEN();
    fn MM_MOM_POSITIONCB();
    fn MM_MPU401_MIDIIN();
    fn MM_MPU401_MIDIOUT();
    fn MM_PC_JOYSTICK();
    fn MM_SNDBLST_MIDIIN();
    fn MM_SNDBLST_MIDIOUT();
    fn MM_SNDBLST_SYNTH();
    fn MM_SNDBLST_WAVEIN();
    fn MM_SNDBLST_WAVEOUT();
    fn MM_STREAM_CLOSE();
    fn MM_STREAM_DONE();
    fn MM_STREAM_ERROR();
    fn MM_STREAM_OPEN();
    fn MM_WAVE_MAPPER();
    fn MM_WIM_CLOSE();
    fn MM_WIM_DATA();
    fn MM_WIM_OPEN();
    fn MM_WOM_CLOSE();
    fn MM_WOM_DONE();
    fn MM_WOM_OPEN();
    fn TIMECAPS();
    fn TIMECODE();
    fn TIMECODE_SAMPLE();
    fn TIMECODE_SAMPLE_FLAGS();
    fn TIMERR_BASE();
    fn TIMERR_NOCANDO();
    fn TIMERR_NOERROR();
    fn TIMERR_STRUCT();
    fn TIME_BYTES();
    fn TIME_CALLBACK_EVENT_PULSE();
    fn TIME_CALLBACK_EVENT_SET();
    fn TIME_CALLBACK_FUNCTION();
    fn TIME_KILL_SYNCHRONOUS();
    fn TIME_MIDI();
    fn TIME_MS();
    fn TIME_ONESHOT();
    fn TIME_PERIODIC();
    fn TIME_SAMPLES();
    fn TIME_SMPTE();
    fn TIME_TICKS();
    fn WAVERR_BASE();
    fn timeBeginPeriod();
    fn timeEndPeriod();
    fn timeGetDevCaps();
    fn timeGetSystemTime();
    fn timeGetTime();
    fn timeKillEvent();
    fn timeSetEvent();
}
