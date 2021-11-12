#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChooseColorA(param0: *mut CHOOSECOLORA) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChooseColorW(param0: *mut CHOOSECOLORW) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ChooseFontA(param0: *mut CHOOSEFONTA) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ChooseFontW(param0: *mut CHOOSEFONTW) -> super::super::super::Foundation::BOOL;
    pub fn CommDlgExtendedError() -> COMMON_DLG_ERRORS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTitleA(param0: super::super::super::Foundation::PSTR, buf: super::super::super::Foundation::PSTR, cchsize: u16) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTitleW(param0: super::super::super::Foundation::PWSTR, buf: super::super::super::Foundation::PWSTR, cchsize: u16) -> i16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSaveFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSaveFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PageSetupDlgA(param0: *mut PAGESETUPDLGA) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PageSetupDlgW(param0: *mut PAGESETUPDLGW) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgA(ppd: *mut PRINTDLGA) -> super::super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgExA(ppd: *mut PRINTDLGEXA) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgExW(ppd: *mut PRINTDLGEXW) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgW(ppd: *mut PRINTDLGW) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND;
}
pub const CDM_FIRST: u32 = 1124u32;
pub const CDM_GETFILEPATH: u32 = 1125u32;
pub const CDM_GETFOLDERIDLIST: u32 = 1127u32;
pub const CDM_GETFOLDERPATH: u32 = 1126u32;
pub const CDM_GETSPEC: u32 = 1124u32;
pub const CDM_HIDECONTROL: u32 = 1129u32;
pub const CDM_LAST: u32 = 1224u32;
pub const CDM_SETCONTROLTEXT: u32 = 1128u32;
pub const CDM_SETDEFEXT: u32 = 1130u32;
pub const CD_LBSELADD: u32 = 2u32;
pub const CD_LBSELCHANGE: u32 = 0u32;
pub const CD_LBSELNOITEMS: i32 = -1i32;
pub const CD_LBSELSUB: u32 = 1u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CHOOSECOLORA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CHOOSECOLORA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CHOOSECOLORW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CHOOSECOLORW(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct CHOOSEFONTA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct CHOOSEFONTA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct CHOOSEFONTW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct CHOOSEFONTW(i32);
#[repr(transparent)]
pub struct CHOOSEFONT_FLAGS(pub u32);
pub const CF_APPLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(512u32);
pub const CF_ANSIONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1024u32);
pub const CF_BOTH: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(3u32);
pub const CF_EFFECTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(256u32);
pub const CF_ENABLEHOOK: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8u32);
pub const CF_ENABLETEMPLATE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16u32);
pub const CF_ENABLETEMPLATEHANDLE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(32u32);
pub const CF_FIXEDPITCHONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16384u32);
pub const CF_FORCEFONTEXIST: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(65536u32);
pub const CF_INACTIVEFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(33554432u32);
pub const CF_INITTOLOGFONTSTRUCT: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(64u32);
pub const CF_LIMITSIZE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8192u32);
pub const CF_NOOEMFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2048u32);
pub const CF_NOFACESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(524288u32);
pub const CF_NOSCRIPTSEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8388608u32);
pub const CF_NOSIMULATIONS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4096u32);
pub const CF_NOSIZESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2097152u32);
pub const CF_NOSTYLESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1048576u32);
pub const CF_NOVECTORFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2048u32);
pub const CF_NOVERTFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16777216u32);
pub const CF_PRINTERFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2u32);
pub const CF_SCALABLEONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(131072u32);
pub const CF_SCREENFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1u32);
pub const CF_SCRIPTSONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1024u32);
pub const CF_SELECTSCRIPT: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4194304u32);
pub const CF_SHOWHELP: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4u32);
pub const CF_TTONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(262144u32);
pub const CF_USESTYLE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(128u32);
pub const CF_WYSIWYG: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(32768u32);
#[repr(transparent)]
pub struct CHOOSEFONT_FONT_TYPE(pub u16);
pub const BOLD_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(256u16);
pub const ITALIC_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(512u16);
pub const PRINTER_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(16384u16);
pub const REGULAR_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(1024u16);
pub const SCREEN_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(8192u16);
pub const SIMULATED_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(32768u16);
pub const COLOR_ADD: u32 = 712u32;
pub const COLOR_BLUE: u32 = 708u32;
pub const COLOR_BLUEACCEL: u32 = 728u32;
pub const COLOR_BOX1: u32 = 720u32;
pub const COLOR_CURRENT: u32 = 709u32;
pub const COLOR_CUSTOM1: u32 = 721u32;
pub const COLOR_ELEMENT: u32 = 716u32;
pub const COLOR_GREEN: u32 = 707u32;
pub const COLOR_GREENACCEL: u32 = 727u32;
pub const COLOR_HUE: u32 = 703u32;
pub const COLOR_HUEACCEL: u32 = 723u32;
pub const COLOR_HUESCROLL: u32 = 700u32;
pub const COLOR_LUM: u32 = 705u32;
pub const COLOR_LUMACCEL: u32 = 725u32;
pub const COLOR_LUMSCROLL: u32 = 702u32;
pub const COLOR_MIX: u32 = 719u32;
pub const COLOR_PALETTE: u32 = 718u32;
pub const COLOR_RAINBOW: u32 = 710u32;
pub const COLOR_RED: u32 = 706u32;
pub const COLOR_REDACCEL: u32 = 726u32;
pub const COLOR_SAMPLES: u32 = 717u32;
pub const COLOR_SAT: u32 = 704u32;
pub const COLOR_SATACCEL: u32 = 724u32;
pub const COLOR_SATSCROLL: u32 = 701u32;
pub const COLOR_SAVE: u32 = 711u32;
pub const COLOR_SCHEMES: u32 = 715u32;
pub const COLOR_SOLID: u32 = 713u32;
pub const COLOR_SOLID_LEFT: u32 = 730u32;
pub const COLOR_SOLID_RIGHT: u32 = 731u32;
pub const COLOR_TUNE: u32 = 714u32;
#[repr(transparent)]
pub struct COMMON_DLG_ERRORS(pub u32);
pub const CDERR_DIALOGFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(65535u32);
pub const CDERR_GENERALCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(0u32);
pub const CDERR_STRUCTSIZE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(1u32);
pub const CDERR_INITIALIZATION: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(2u32);
pub const CDERR_NOTEMPLATE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(3u32);
pub const CDERR_NOHINSTANCE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4u32);
pub const CDERR_LOADSTRFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(5u32);
pub const CDERR_FINDRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(6u32);
pub const CDERR_LOADRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(7u32);
pub const CDERR_LOCKRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8u32);
pub const CDERR_MEMALLOCFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(9u32);
pub const CDERR_MEMLOCKFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(10u32);
pub const CDERR_NOHOOK: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(11u32);
pub const CDERR_REGISTERMSGFAIL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12u32);
pub const PDERR_PRINTERCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4096u32);
pub const PDERR_SETUPFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4097u32);
pub const PDERR_PARSEFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4098u32);
pub const PDERR_RETDEFFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4099u32);
pub const PDERR_LOADDRVFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4100u32);
pub const PDERR_GETDEVMODEFAIL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4101u32);
pub const PDERR_INITFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4102u32);
pub const PDERR_NODEVICES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4103u32);
pub const PDERR_NODEFAULTPRN: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4104u32);
pub const PDERR_DNDMMISMATCH: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4105u32);
pub const PDERR_CREATEICFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4106u32);
pub const PDERR_PRINTERNOTFOUND: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4107u32);
pub const PDERR_DEFAULTDIFFERENT: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4108u32);
pub const CFERR_CHOOSEFONTCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8192u32);
pub const CFERR_NOFONTS: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8193u32);
pub const CFERR_MAXLESSTHANMIN: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8194u32);
pub const FNERR_FILENAMECODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12288u32);
pub const FNERR_SUBCLASSFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12289u32);
pub const FNERR_INVALIDFILENAME: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12290u32);
pub const FNERR_BUFFERTOOSMALL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12291u32);
pub const FRERR_FINDREPLACECODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(16384u32);
pub const FRERR_BUFFERLENGTHZERO: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(16385u32);
pub const CCERR_CHOOSECOLORCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(20480u32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct DEVNAMES(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct DEVNAMES(i32);
pub const DLG_COLOR: u32 = 10u32;
pub const DN_DEFAULTPRN: u32 = 1u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FINDREPLACEA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FINDREPLACEA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FINDREPLACEW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FINDREPLACEW(i32);
#[repr(transparent)]
pub struct FINDREPLACE_FLAGS(pub u32);
pub const FR_DIALOGTERM: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(64u32);
pub const FR_DOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1u32);
pub const FR_ENABLEHOOK: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(256u32);
pub const FR_ENABLETEMPLATE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(512u32);
pub const FR_ENABLETEMPLATEHANDLE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(8192u32);
pub const FR_FINDNEXT: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(8u32);
pub const FR_HIDEUPDOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(16384u32);
pub const FR_HIDEMATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(32768u32);
pub const FR_HIDEWHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(65536u32);
pub const FR_MATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(4u32);
pub const FR_NOMATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(2048u32);
pub const FR_NOUPDOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1024u32);
pub const FR_NOWHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(4096u32);
pub const FR_REPLACE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(16u32);
pub const FR_REPLACEALL: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(32u32);
pub const FR_SHOWHELP: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(128u32);
pub const FR_WHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(2u32);
pub const FRM_FIRST: u32 = 1124u32;
pub const FRM_LAST: u32 = 1224u32;
pub const FRM_SETOPERATIONRESULT: u32 = 1124u32;
pub const FRM_SETOPERATIONRESULTTEXT: u32 = 1125u32;
pub const FR_NOWRAPAROUND: u32 = 524288u32;
pub const FR_RAW: u32 = 131072u32;
pub const FR_SHOWWRAPAROUND: u32 = 262144u32;
pub const FR_WRAPAROUND: u32 = 1048576u32;
#[repr(transparent)]
pub struct IPrintDialogCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintDialogServices(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type LPCCHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPCFHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPFRHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPOFNHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPPAGEPAINTHOOK = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPPAGESETUPHOOK = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPPRINTHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
#[cfg(feature = "Win32_Foundation")]
pub type LPSETUPHOOKPROC = unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize;
pub const NUM_BASIC_COLORS: u32 = 48u32;
pub const NUM_CUSTOM_COLORS: u32 = 16u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OFNOTIFYA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OFNOTIFYA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OFNOTIFYEXA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OFNOTIFYEXA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OFNOTIFYEXW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OFNOTIFYEXW(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OFNOTIFYW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OFNOTIFYW(i32);
pub const OFN_SHAREFALLTHROUGH: u32 = 2u32;
pub const OFN_SHARENOWARN: u32 = 1u32;
pub const OFN_SHAREWARN: u32 = 0u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPENFILENAMEA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPENFILENAMEA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPENFILENAMEW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPENFILENAMEW(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPENFILENAME_NT4A(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPENFILENAME_NT4A(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPENFILENAME_NT4W(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPENFILENAME_NT4W(i32);
#[repr(transparent)]
pub struct OPEN_FILENAME_FLAGS(pub u32);
pub const OFN_READONLY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1u32);
pub const OFN_OVERWRITEPROMPT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2u32);
pub const OFN_HIDEREADONLY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4u32);
pub const OFN_NOCHANGEDIR: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8u32);
pub const OFN_SHOWHELP: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(16u32);
pub const OFN_ENABLEHOOK: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(32u32);
pub const OFN_ENABLETEMPLATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(64u32);
pub const OFN_ENABLETEMPLATEHANDLE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(128u32);
pub const OFN_NOVALIDATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(256u32);
pub const OFN_ALLOWMULTISELECT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(512u32);
pub const OFN_EXTENSIONDIFFERENT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1024u32);
pub const OFN_PATHMUSTEXIST: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2048u32);
pub const OFN_FILEMUSTEXIST: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4096u32);
pub const OFN_CREATEPROMPT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8192u32);
pub const OFN_SHAREAWARE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(16384u32);
pub const OFN_NOREADONLYRETURN: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(32768u32);
pub const OFN_NOTESTFILECREATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(65536u32);
pub const OFN_NONETWORKBUTTON: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(131072u32);
pub const OFN_NOLONGNAMES: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(262144u32);
pub const OFN_EXPLORER: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(524288u32);
pub const OFN_NODEREFERENCELINKS: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1048576u32);
pub const OFN_LONGNAMES: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2097152u32);
pub const OFN_ENABLEINCLUDENOTIFY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4194304u32);
pub const OFN_ENABLESIZING: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8388608u32);
pub const OFN_DONTADDTORECENT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(33554432u32);
pub const OFN_FORCESHOWHIDDEN: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(268435456u32);
#[repr(transparent)]
pub struct OPEN_FILENAME_FLAGS_EX(pub u32);
pub const OFN_EX_NONE: OPEN_FILENAME_FLAGS_EX = OPEN_FILENAME_FLAGS_EX(0u32);
pub const OFN_EX_NOPLACESBAR: OPEN_FILENAME_FLAGS_EX = OPEN_FILENAME_FLAGS_EX(1u32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PAGESETUPDLGA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PAGESETUPDLGA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PAGESETUPDLGW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PAGESETUPDLGW(i32);
#[repr(transparent)]
pub struct PAGESETUPDLG_FLAGS(pub u32);
pub const PSD_DEFAULTMINMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(0u32);
pub const PSD_DISABLEMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(16u32);
pub const PSD_DISABLEORIENTATION: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(256u32);
pub const PSD_DISABLEPAGEPAINTING: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(524288u32);
pub const PSD_DISABLEPAPER: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(512u32);
pub const PSD_DISABLEPRINTER: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(32u32);
pub const PSD_ENABLEPAGEPAINTHOOK: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(262144u32);
pub const PSD_ENABLEPAGESETUPHOOK: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(8192u32);
pub const PSD_ENABLEPAGESETUPTEMPLATE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(32768u32);
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(131072u32);
pub const PSD_INHUNDREDTHSOFMILLIMETERS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(8u32);
pub const PSD_INTHOUSANDTHSOFINCHES: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(4u32);
pub const PSD_INWININIINTLMEASURE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(0u32);
pub const PSD_MARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2u32);
pub const PSD_MINMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(1u32);
pub const PSD_NONETWORKBUTTON: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2097152u32);
pub const PSD_NOWARNING: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(128u32);
pub const PSD_RETURNDEFAULT: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(1024u32);
pub const PSD_SHOWHELP: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2048u32);
pub const PD_RESULT_APPLY: u32 = 2u32;
pub const PD_RESULT_CANCEL: u32 = 0u32;
pub const PD_RESULT_PRINT: u32 = 1u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct PRINTDLGA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct PRINTDLGA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct PRINTDLGEXA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct PRINTDLGEXA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct PRINTDLGEXW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct PRINTDLGEXW(i32);
#[repr(transparent)]
pub struct PRINTDLGEX_FLAGS(pub u32);
pub const PD_ALLPAGES: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(0u32);
pub const PD_COLLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16u32);
pub const PD_CURRENTPAGE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4194304u32);
pub const PD_DISABLEPRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(524288u32);
pub const PD_ENABLEPRINTTEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16384u32);
pub const PD_ENABLEPRINTTEMPLATEHANDLE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(65536u32);
pub const PD_EXCLUSIONFLAGS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16777216u32);
pub const PD_HIDEPRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1048576u32);
pub const PD_NOCURRENTPAGE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8388608u32);
pub const PD_NOPAGENUMS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8u32);
pub const PD_NOSELECTION: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4u32);
pub const PD_NOWARNING: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(128u32);
pub const PD_PAGENUMS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2u32);
pub const PD_PRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(32u32);
pub const PD_RETURNDC: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(256u32);
pub const PD_RETURNDEFAULT: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1024u32);
pub const PD_RETURNIC: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(512u32);
pub const PD_SELECTION: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1u32);
pub const PD_USEDEVMODECOPIES: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(262144u32);
pub const PD_USEDEVMODECOPIESANDCOLLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(262144u32);
pub const PD_USELARGETEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(268435456u32);
pub const PD_ENABLEPRINTHOOK: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4096u32);
pub const PD_ENABLESETUPHOOK: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8192u32);
pub const PD_ENABLESETUPTEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(32768u32);
pub const PD_ENABLESETUPTEMPLATEHANDLE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(131072u32);
pub const PD_NONETWORKBUTTON: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2097152u32);
pub const PD_PRINTSETUP: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(64u32);
pub const PD_SHOWHELP: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2048u32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct PRINTDLGW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct PRINTDLGW(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct PRINTPAGERANGE(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct PRINTPAGERANGE(i32);
pub const PS_OPENTYPE_FONTTYPE: u32 = 65536u32;
pub const START_PAGE_GENERAL: u32 = 4294967295u32;
pub const SYMBOL_FONTTYPE: u32 = 524288u32;
pub const TT_OPENTYPE_FONTTYPE: u32 = 131072u32;
pub const TYPE1_FONTTYPE: u32 = 262144u32;
pub const WM_CHOOSEFONT_GETLOGFONT: u32 = 1025u32;
pub const WM_CHOOSEFONT_SETFLAGS: u32 = 1126u32;
pub const WM_CHOOSEFONT_SETLOGFONT: u32 = 1125u32;
pub const WM_PSD_ENVSTAMPRECT: u32 = 1029u32;
pub const WM_PSD_FULLPAGERECT: u32 = 1025u32;
pub const WM_PSD_GREEKTEXTRECT: u32 = 1028u32;
pub const WM_PSD_MARGINRECT: u32 = 1027u32;
pub const WM_PSD_MINMARGINRECT: u32 = 1026u32;
pub const WM_PSD_YAFULLPAGERECT: u32 = 1030u32;
