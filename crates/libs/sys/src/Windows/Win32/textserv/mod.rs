pub const CARET_CUSTOM: CARET_FLAGS = 1;
pub type CARET_FLAGS = i32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub union CARET_INFO {
    pub hbitmap: super::HBITMAP,
    pub caretFlags: CARET_FLAGS,
}
#[cfg(feature = "windef")]
impl Default for CARET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CARET_ITALIC: CARET_FLAGS = 32;
pub const CARET_NONE: CARET_FLAGS = 0;
pub const CARET_NULL: CARET_FLAGS = 64;
pub const CARET_ROTATE90: CARET_FLAGS = 128;
pub const CARET_RTL: CARET_FLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CHANGENOTIFY {
    pub dwChangeType: u32,
    pub pvCookieData: *mut core::ffi::c_void,
}
impl Default for CHANGENOTIFY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CHANGETYPE = i32;
pub const CN_GENERIC: CHANGETYPE = 0;
pub const CN_NEWREDO: CHANGETYPE = 4;
pub const CN_NEWUNDO: CHANGETYPE = 2;
pub const CN_TEXTCHANGED: CHANGETYPE = 1;
pub type PCreateTextServices = Option<unsafe extern "system" fn(punkouter: *mut core::ffi::c_void, pitexthost: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub type PShutdownTextServices = Option<unsafe extern "system" fn(ptextservices: *mut core::ffi::c_void) -> windows_sys::core::HRESULT>;
pub const S_MSG_KEY_IGNORED: u32 = 262657;
pub const TXES_ISDIALOG: u32 = 1;
pub type TXTBACKSTYLE = i32;
pub const TXTBACK_OPAQUE: TXTBACKSTYLE = 1;
pub const TXTBACK_TRANSPARENT: TXTBACKSTYLE = 0;
pub const TXTBIT_ADVANCEDINPUT: u32 = 536870912;
pub const TXTBIT_ALLOWBEEP: u32 = 2048;
pub const TXTBIT_AUTOWORDSEL: u32 = 128;
pub const TXTBIT_BACKSTYLECHANGE: u32 = 16384;
pub const TXTBIT_CHARFORMATCHANGE: u32 = 131072;
pub const TXTBIT_CLIENTRECTCHANGE: u32 = 1048576;
pub const TXTBIT_D2DDWRITE: u32 = 16777216;
pub const TXTBIT_D2DPIXELSNAPPED: u32 = 67108864;
pub const TXTBIT_D2DSIMPLETYPOGRAPHY: u32 = 33554432;
pub const TXTBIT_D2DSUBPIXELLINES: u32 = 134217728;
pub const TXTBIT_DISABLEDRAG: u32 = 4096;
pub const TXTBIT_EXTENTCHANGE: u32 = 524288;
pub const TXTBIT_FLASHLASTPASSWORDCHAR: u32 = 268435456;
pub const TXTBIT_HIDESELECTION: u32 = 32;
pub const TXTBIT_MAXLENGTHCHANGE: u32 = 32768;
pub const TXTBIT_MULTILINE: u32 = 2;
pub const TXTBIT_NOTHREADREFCOUNT: u32 = 4194304;
pub const TXTBIT_PARAFORMATCHANGE: u32 = 262144;
pub const TXTBIT_READONLY: u32 = 4;
pub const TXTBIT_RICHTEXT: u32 = 1;
pub const TXTBIT_SAVESELECTION: u32 = 64;
pub const TXTBIT_SCROLLBARCHANGE: u32 = 65536;
pub const TXTBIT_SELBARCHANGE: u32 = 512;
pub const TXTBIT_SHOWACCELERATOR: u32 = 8;
pub const TXTBIT_SHOWPASSWORD: u32 = 8388608;
pub const TXTBIT_USECURRENTBKG: u32 = 2097152;
pub const TXTBIT_USEPASSWORD: u32 = 16;
pub const TXTBIT_VERTICAL: u32 = 256;
pub const TXTBIT_VIEWINSETCHANGE: u32 = 8192;
pub const TXTBIT_WORDWRAP: u32 = 1024;
pub type TXTHITRESULT = i32;
pub const TXTHITRESULT_CLOSE: TXTHITRESULT = 2;
pub const TXTHITRESULT_HIT: TXTHITRESULT = 3;
pub const TXTHITRESULT_NOHIT: TXTHITRESULT = 0;
pub const TXTHITRESULT_TRANSPARENT: TXTHITRESULT = 1;
pub type TXTNATURALSIZE = i32;
pub const TXTNS_EMU: TXTNATURALSIZE = -2147483648;
pub const TXTNS_FITTOCONTENT: TXTNATURALSIZE = 1;
pub const TXTNS_FITTOCONTENT2: TXTNATURALSIZE = 0;
pub const TXTNS_FITTOCONTENT3: TXTNATURALSIZE = 3;
pub const TXTNS_FITTOCONTENTWSP: TXTNATURALSIZE = 4;
pub const TXTNS_INCLUDELASTLINE: TXTNATURALSIZE = 1073741824;
pub const TXTNS_ROUNDTOLINE: TXTNATURALSIZE = 2;
pub type TXTVIEW = i32;
pub const TXTVIEW_ACTIVE: TXTVIEW = 0;
pub const TXTVIEW_INACTIVE: TXTVIEW = -1;
