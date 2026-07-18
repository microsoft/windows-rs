pub const CALLBACK_EVENT: u32 = 327680;
pub const CALLBACK_NULL: u32 = 0;
pub const CALLBACK_TASK: u32 = 131072;
pub const CALLBACK_THREAD: u32 = 131072;
pub const CALLBACK_TYPEMASK: u32 = 458752;
pub const CALLBACK_WINDOW: u32 = 65536;
pub type DRVCALLBACK = Option<unsafe extern "system" fn(hdrvr: HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDRVR(pub *mut core::ffi::c_void);
impl Default for HDRVR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JOYERR_BASE: u32 = 160;
pub type LPDRVCALLBACK = Option<unsafe extern "system" fn(hdrvr: HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub type LPMMTIME = *mut MMTIME;
pub type LPUINT = *mut u32;
pub const MAXERRORLENGTH: u32 = 256;
pub const MAXPNAMELEN: u32 = 32;
pub const MAX_JOYSTICKOEMVXDNAME: u32 = 260;
pub const MCIERR_BASE: u32 = 256;
pub const MCI_CD_OFFSET: u32 = 1088;
pub const MCI_SEQ_OFFSET: u32 = 1216;
pub const MCI_STRING_OFFSET: u32 = 512;
pub const MCI_VD_OFFSET: u32 = 1024;
pub const MCI_WAVE_OFFSET: u32 = 1152;
pub const MIDIERR_BASE: u32 = 64;
pub const MIXERR_BASE: u32 = 1024;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MMRESULT(pub u32);
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MMVERSION(pub u32);
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
pub const MM_STREAM_CLOSE: u32 = 981;
pub const MM_STREAM_DONE: u32 = 982;
pub const MM_STREAM_ERROR: u32 = 983;
pub const MM_STREAM_OPEN: u32 = 980;
pub const MM_WIM_CLOSE: u32 = 959;
pub const MM_WIM_DATA: u32 = 960;
pub const MM_WIM_OPEN: u32 = 958;
pub const MM_WOM_CLOSE: u32 = 956;
pub const MM_WOM_DONE: u32 = 957;
pub const MM_WOM_OPEN: u32 = 955;
pub type NPMMTIME = *mut MMTIME;
pub type PDRVCALLBACK = Option<unsafe extern "system" fn(hdrvr: HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub type PMMTIME = *mut MMTIME;
pub const TIMERR_BASE: u32 = 96;
pub const TIME_BYTES: u32 = 4;
pub const TIME_MIDI: u32 = 16;
pub const TIME_MS: u32 = 1;
pub const TIME_SAMPLES: u32 = 2;
pub const TIME_SMPTE: u32 = 8;
pub const TIME_TICKS: u32 = 32;
pub const WAVERR_BASE: u32 = 32;
