pub const CALLBACK_EVENT: i32 = 327680;
pub const CALLBACK_NULL: i32 = 0;
pub const CALLBACK_TASK: i32 = 131072;
pub const CALLBACK_THREAD: i32 = 131072;
pub const CALLBACK_TYPEMASK: i32 = 458752;
pub const CALLBACK_WINDOW: i32 = 65536;
pub type DRVCALLBACK = Option<unsafe extern "system" fn(hdrvr: HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDRVR(pub *mut core::ffi::c_void);
impl Default for HDRVR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JOYERR_BASE: i32 = 160;
pub type LPDRVCALLBACK = Option<unsafe extern "system" fn(hdrvr: HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub type LPMMTIME = *mut MMTIME;
pub type LPUINT = *mut u32;
pub const MAXERRORLENGTH: i32 = 256;
pub const MAXPNAMELEN: i32 = 32;
pub const MAX_JOYSTICKOEMVXDNAME: i32 = 260;
pub const MCIERR_BASE: i32 = 256;
pub const MCI_CD_OFFSET: i32 = 1088;
pub const MCI_SEQ_OFFSET: i32 = 1216;
pub const MCI_STRING_OFFSET: i32 = 512;
pub const MCI_VD_OFFSET: i32 = 1024;
pub const MCI_WAVE_OFFSET: i32 = 1152;
pub const MIDIERR_BASE: i32 = 64;
pub const MIXERR_BASE: i32 = 1024;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MMRESULT(pub u32);
pub const MMSYSERR_ALLOCATED: i32 = 4;
pub const MMSYSERR_BADDB: i32 = 14;
pub const MMSYSERR_BADDEVICEID: i32 = 2;
pub const MMSYSERR_BADERRNUM: i32 = 9;
pub const MMSYSERR_BASE: i32 = 0;
pub const MMSYSERR_DELETEERROR: i32 = 18;
pub const MMSYSERR_ERROR: i32 = 1;
pub const MMSYSERR_HANDLEBUSY: i32 = 12;
pub const MMSYSERR_INVALFLAG: i32 = 10;
pub const MMSYSERR_INVALHANDLE: i32 = 5;
pub const MMSYSERR_INVALIDALIAS: i32 = 13;
pub const MMSYSERR_INVALPARAM: i32 = 11;
pub const MMSYSERR_KEYNOTFOUND: i32 = 15;
pub const MMSYSERR_LASTERROR: i32 = 21;
pub const MMSYSERR_MOREDATA: i32 = 21;
pub const MMSYSERR_NODRIVER: i32 = 6;
pub const MMSYSERR_NODRIVERCB: i32 = 20;
pub const MMSYSERR_NOERROR: i32 = 0;
pub const MMSYSERR_NOMEM: i32 = 7;
pub const MMSYSERR_NOTENABLED: i32 = 3;
pub const MMSYSERR_NOTSUPPORTED: i32 = 8;
pub const MMSYSERR_READERROR: i32 = 16;
pub const MMSYSERR_VALNOTFOUND: i32 = 19;
pub const MMSYSERR_WRITEERROR: i32 = 17;
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
pub const MM_DRVM_CLOSE: i32 = 977;
pub const MM_DRVM_DATA: i32 = 978;
pub const MM_DRVM_ERROR: i32 = 979;
pub const MM_DRVM_OPEN: i32 = 976;
pub const MM_JOY1BUTTONDOWN: i32 = 949;
pub const MM_JOY1BUTTONUP: i32 = 951;
pub const MM_JOY1MOVE: i32 = 928;
pub const MM_JOY1ZMOVE: i32 = 930;
pub const MM_JOY2BUTTONDOWN: i32 = 950;
pub const MM_JOY2BUTTONUP: i32 = 952;
pub const MM_JOY2MOVE: i32 = 929;
pub const MM_JOY2ZMOVE: i32 = 931;
pub const MM_MCINOTIFY: i32 = 953;
pub const MM_MCISIGNAL: i32 = 971;
pub const MM_MIM_CLOSE: i32 = 962;
pub const MM_MIM_DATA: i32 = 963;
pub const MM_MIM_ERROR: i32 = 965;
pub const MM_MIM_LONGDATA: i32 = 964;
pub const MM_MIM_LONGERROR: i32 = 966;
pub const MM_MIM_MOREDATA: i32 = 972;
pub const MM_MIM_OPEN: i32 = 961;
pub const MM_MIXM_CONTROL_CHANGE: i32 = 977;
pub const MM_MIXM_LINE_CHANGE: i32 = 976;
pub const MM_MOM_CLOSE: i32 = 968;
pub const MM_MOM_DONE: i32 = 969;
pub const MM_MOM_OPEN: i32 = 967;
pub const MM_MOM_POSITIONCB: i32 = 970;
pub const MM_STREAM_CLOSE: i32 = 981;
pub const MM_STREAM_DONE: i32 = 982;
pub const MM_STREAM_ERROR: i32 = 983;
pub const MM_STREAM_OPEN: i32 = 980;
pub const MM_WIM_CLOSE: i32 = 959;
pub const MM_WIM_DATA: i32 = 960;
pub const MM_WIM_OPEN: i32 = 958;
pub const MM_WOM_CLOSE: i32 = 956;
pub const MM_WOM_DONE: i32 = 957;
pub const MM_WOM_OPEN: i32 = 955;
pub type NPMMTIME = *mut MMTIME;
pub type PDRVCALLBACK = Option<unsafe extern "system" fn(hdrvr: HDRVR, umsg: u32, dwuser: usize, dw1: usize, dw2: usize)>;
pub type PMMTIME = *mut MMTIME;
pub const TIMERR_BASE: i32 = 96;
pub const TIME_BYTES: i32 = 4;
pub const TIME_MIDI: i32 = 16;
pub const TIME_MS: i32 = 1;
pub const TIME_SAMPLES: i32 = 2;
pub const TIME_SMPTE: i32 = 8;
pub const TIME_TICKS: i32 = 32;
pub const WAVERR_BASE: i32 = 32;
