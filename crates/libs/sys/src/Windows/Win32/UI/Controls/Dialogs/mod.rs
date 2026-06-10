windows_link::link!("comdlg32.dll" "system" fn ChooseColorA(param0 : *mut CHOOSECOLORA) -> windows_sys::core::BOOL);
windows_link::link!("comdlg32.dll" "system" fn ChooseColorW(param0 : *mut CHOOSECOLORW) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("comdlg32.dll" "system" fn ChooseFontA(param0 : *mut CHOOSEFONTA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("comdlg32.dll" "system" fn ChooseFontW(param0 : *mut CHOOSEFONTW) -> windows_sys::core::BOOL);
windows_link::link!("comdlg32.dll" "system" fn CommDlgExtendedError() -> COMMON_DLG_ERRORS);
windows_link::link!("comdlg32.dll" "system" fn FindTextA(param0 : *mut FINDREPLACEA) -> super::super::super::Foundation::HWND);
windows_link::link!("comdlg32.dll" "system" fn FindTextW(param0 : *mut FINDREPLACEW) -> super::super::super::Foundation::HWND);
windows_link::link!("comdlg32.dll" "system" fn GetFileTitleA(param0 : windows_sys::core::PCSTR, buf : windows_sys::core::PSTR, cchsize : u16) -> i16);
windows_link::link!("comdlg32.dll" "system" fn GetFileTitleW(param0 : windows_sys::core::PCWSTR, buf : windows_sys::core::PWSTR, cchsize : u16) -> i16);
windows_link::link!("comdlg32.dll" "system" fn GetOpenFileNameA(param0 : *mut OPENFILENAMEA) -> windows_sys::core::BOOL);
windows_link::link!("comdlg32.dll" "system" fn GetOpenFileNameW(param0 : *mut OPENFILENAMEW) -> windows_sys::core::BOOL);
windows_link::link!("comdlg32.dll" "system" fn GetSaveFileNameA(param0 : *mut OPENFILENAMEA) -> windows_sys::core::BOOL);
windows_link::link!("comdlg32.dll" "system" fn GetSaveFileNameW(param0 : *mut OPENFILENAMEW) -> windows_sys::core::BOOL);
windows_link::link!("comdlg32.dll" "system" fn PageSetupDlgA(param0 : *mut PAGESETUPDLGA) -> windows_sys::core::BOOL);
windows_link::link!("comdlg32.dll" "system" fn PageSetupDlgW(param0 : *mut PAGESETUPDLGW) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("comdlg32.dll" "system" fn PrintDlgA(ppd : *mut PRINTDLGA) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("comdlg32.dll" "system" fn PrintDlgExA(ppd : *mut PRINTDLGEXA) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("comdlg32.dll" "system" fn PrintDlgExW(ppd : *mut PRINTDLGEXW) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("comdlg32.dll" "system" fn PrintDlgW(ppd : *mut PRINTDLGW) -> windows_sys::core::BOOL);
windows_link::link!("comdlg32.dll" "system" fn ReplaceTextA(param0 : *mut FINDREPLACEA) -> super::super::super::Foundation::HWND);
windows_link::link!("comdlg32.dll" "system" fn ReplaceTextW(param0 : *mut FINDREPLACEW) -> super::super::super::Foundation::HWND);
pub const BOLD_FONTTYPE: CHOOSEFONT_FONT_TYPE = 256;
pub const CCERR_CHOOSECOLORCODES: COMMON_DLG_ERRORS = 20480;
pub const CC_ANYCOLOR: CHOOSECOLOR_FLAGS = 256;
pub const CC_ENABLEHOOK: CHOOSECOLOR_FLAGS = 16;
pub const CC_ENABLETEMPLATE: CHOOSECOLOR_FLAGS = 32;
pub const CC_ENABLETEMPLATEHANDLE: CHOOSECOLOR_FLAGS = 64;
pub const CC_FULLOPEN: CHOOSECOLOR_FLAGS = 2;
pub const CC_PREVENTFULLOPEN: CHOOSECOLOR_FLAGS = 4;
pub const CC_RGBINIT: CHOOSECOLOR_FLAGS = 1;
pub const CC_SHOWHELP: CHOOSECOLOR_FLAGS = 8;
pub const CC_SOLIDCOLOR: CHOOSECOLOR_FLAGS = 128;
pub const CDERR_DIALOGFAILURE: COMMON_DLG_ERRORS = 65535;
pub const CDERR_FINDRESFAILURE: COMMON_DLG_ERRORS = 6;
pub const CDERR_GENERALCODES: COMMON_DLG_ERRORS = 0;
pub const CDERR_INITIALIZATION: COMMON_DLG_ERRORS = 2;
pub const CDERR_LOADRESFAILURE: COMMON_DLG_ERRORS = 7;
pub const CDERR_LOADSTRFAILURE: COMMON_DLG_ERRORS = 5;
pub const CDERR_LOCKRESFAILURE: COMMON_DLG_ERRORS = 8;
pub const CDERR_MEMALLOCFAILURE: COMMON_DLG_ERRORS = 9;
pub const CDERR_MEMLOCKFAILURE: COMMON_DLG_ERRORS = 10;
pub const CDERR_NOHINSTANCE: COMMON_DLG_ERRORS = 4;
pub const CDERR_NOHOOK: COMMON_DLG_ERRORS = 11;
pub const CDERR_NOTEMPLATE: COMMON_DLG_ERRORS = 3;
pub const CDERR_REGISTERMSGFAIL: COMMON_DLG_ERRORS = 12;
pub const CDERR_STRUCTSIZE: COMMON_DLG_ERRORS = 1;
pub const CDM_FIRST: u32 = 1124;
pub const CDM_GETFILEPATH: u32 = 1125;
pub const CDM_GETFOLDERIDLIST: u32 = 1127;
pub const CDM_GETFOLDERPATH: u32 = 1126;
pub const CDM_GETSPEC: u32 = 1124;
pub const CDM_HIDECONTROL: u32 = 1129;
pub const CDM_LAST: u32 = 1224;
pub const CDM_SETCONTROLTEXT: u32 = 1128;
pub const CDM_SETDEFEXT: u32 = 1130;
pub const CDN_FILEOK: u32 = 4294966690;
pub const CDN_FOLDERCHANGE: u32 = 4294966693;
pub const CDN_HELP: u32 = 4294966691;
pub const CDN_INCLUDEITEM: u32 = 4294966688;
pub const CDN_INITDONE: u32 = 4294966695;
pub const CDN_SELCHANGE: u32 = 4294966694;
pub const CDN_SHAREVIOLATION: u32 = 4294966692;
pub const CDN_TYPECHANGE: u32 = 4294966689;
pub const CD_LBSELADD: u32 = 2;
pub const CD_LBSELCHANGE: u32 = 0;
pub const CD_LBSELNOITEMS: i32 = -1;
pub const CD_LBSELSUB: u32 = 1;
pub const CFERR_CHOOSEFONTCODES: COMMON_DLG_ERRORS = 8192;
pub const CFERR_MAXLESSTHANMIN: COMMON_DLG_ERRORS = 8194;
pub const CFERR_NOFONTS: COMMON_DLG_ERRORS = 8193;
pub const CF_ANSIONLY: CHOOSEFONT_FLAGS = 1024;
pub const CF_APPLY: CHOOSEFONT_FLAGS = 512;
pub const CF_BOTH: CHOOSEFONT_FLAGS = 3;
pub const CF_EFFECTS: CHOOSEFONT_FLAGS = 256;
pub const CF_ENABLEHOOK: CHOOSEFONT_FLAGS = 8;
pub const CF_ENABLETEMPLATE: CHOOSEFONT_FLAGS = 16;
pub const CF_ENABLETEMPLATEHANDLE: CHOOSEFONT_FLAGS = 32;
pub const CF_FIXEDPITCHONLY: CHOOSEFONT_FLAGS = 16384;
pub const CF_FORCEFONTEXIST: CHOOSEFONT_FLAGS = 65536;
pub const CF_INACTIVEFONTS: CHOOSEFONT_FLAGS = 33554432;
pub const CF_INITTOLOGFONTSTRUCT: CHOOSEFONT_FLAGS = 64;
pub const CF_LIMITSIZE: CHOOSEFONT_FLAGS = 8192;
pub const CF_NOFACESEL: CHOOSEFONT_FLAGS = 524288;
pub const CF_NOOEMFONTS: CHOOSEFONT_FLAGS = 2048;
pub const CF_NOSCRIPTSEL: CHOOSEFONT_FLAGS = 8388608;
pub const CF_NOSIMULATIONS: CHOOSEFONT_FLAGS = 4096;
pub const CF_NOSIZESEL: CHOOSEFONT_FLAGS = 2097152;
pub const CF_NOSTYLESEL: CHOOSEFONT_FLAGS = 1048576;
pub const CF_NOVECTORFONTS: CHOOSEFONT_FLAGS = 2048;
pub const CF_NOVERTFONTS: CHOOSEFONT_FLAGS = 16777216;
pub const CF_PRINTERFONTS: CHOOSEFONT_FLAGS = 2;
pub const CF_SCALABLEONLY: CHOOSEFONT_FLAGS = 131072;
pub const CF_SCREENFONTS: CHOOSEFONT_FLAGS = 1;
pub const CF_SCRIPTSONLY: CHOOSEFONT_FLAGS = 1024;
pub const CF_SELECTSCRIPT: CHOOSEFONT_FLAGS = 4194304;
pub const CF_SHOWHELP: CHOOSEFONT_FLAGS = 4;
pub const CF_TTONLY: CHOOSEFONT_FLAGS = 262144;
pub const CF_USESTYLE: CHOOSEFONT_FLAGS = 128;
pub const CF_WYSIWYG: CHOOSEFONT_FLAGS = 32768;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CHOOSECOLOR_FLAGS = u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: windows_sys::core::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: windows_sys::core::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: windows_sys::core::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: windows_sys::core::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CHOOSEFONT_FLAGS = u32;
pub type CHOOSEFONT_FONT_TYPE = u16;
pub const COLOROKSTRING: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_ColorOK");
pub const COLOROKSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_ColorOK");
pub const COLOROKSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_ColorOK");
pub const COLOR_ADD: u32 = 712;
pub const COLOR_BLUE: u32 = 708;
pub const COLOR_BLUEACCEL: u32 = 728;
pub const COLOR_BOX1: u32 = 720;
pub const COLOR_CURRENT: u32 = 709;
pub const COLOR_CUSTOM1: u32 = 721;
pub const COLOR_ELEMENT: u32 = 716;
pub const COLOR_GREEN: u32 = 707;
pub const COLOR_GREENACCEL: u32 = 727;
pub const COLOR_HUE: u32 = 703;
pub const COLOR_HUEACCEL: u32 = 723;
pub const COLOR_HUESCROLL: u32 = 700;
pub const COLOR_LUM: u32 = 705;
pub const COLOR_LUMACCEL: u32 = 725;
pub const COLOR_LUMSCROLL: u32 = 702;
pub const COLOR_MIX: u32 = 719;
pub const COLOR_PALETTE: u32 = 718;
pub const COLOR_RAINBOW: u32 = 710;
pub const COLOR_RED: u32 = 706;
pub const COLOR_REDACCEL: u32 = 726;
pub const COLOR_SAMPLES: u32 = 717;
pub const COLOR_SAT: u32 = 704;
pub const COLOR_SATACCEL: u32 = 724;
pub const COLOR_SATSCROLL: u32 = 701;
pub const COLOR_SAVE: u32 = 711;
pub const COLOR_SCHEMES: u32 = 715;
pub const COLOR_SOLID: u32 = 713;
pub const COLOR_SOLID_LEFT: u32 = 730;
pub const COLOR_SOLID_RIGHT: u32 = 731;
pub const COLOR_TUNE: u32 = 714;
pub type COMMON_DLG_ERRORS = u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
pub const DLG_COLOR: u32 = 10;
pub const DN_DEFAULTPRN: u32 = 1;
pub const FILEOKSTRING: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_FileNameOK");
pub const FILEOKSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_FileNameOK");
pub const FILEOKSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_FileNameOK");
pub const FINDMSGSTRING: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_FindReplace");
pub const FINDMSGSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_FindReplace");
pub const FINDMSGSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_FindReplace");
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: windows_sys::core::PSTR,
    pub lpstrReplaceWith: windows_sys::core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for FINDREPLACEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: windows_sys::core::PSTR,
    pub lpstrReplaceWith: windows_sys::core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FINDREPLACEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: windows_sys::core::PWSTR,
    pub lpstrReplaceWith: windows_sys::core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for FINDREPLACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: windows_sys::core::PWSTR,
    pub lpstrReplaceWith: windows_sys::core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for FINDREPLACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FINDREPLACE_FLAGS = u32;
pub const FNERR_BUFFERTOOSMALL: COMMON_DLG_ERRORS = 12291;
pub const FNERR_FILENAMECODES: COMMON_DLG_ERRORS = 12288;
pub const FNERR_INVALIDFILENAME: COMMON_DLG_ERRORS = 12290;
pub const FNERR_SUBCLASSFAILURE: COMMON_DLG_ERRORS = 12289;
pub const FRERR_BUFFERLENGTHZERO: COMMON_DLG_ERRORS = 16385;
pub const FRERR_FINDREPLACECODES: COMMON_DLG_ERRORS = 16384;
pub const FRM_FIRST: u32 = 1124;
pub const FRM_LAST: u32 = 1224;
pub const FRM_SETOPERATIONRESULT: u32 = 1124;
pub const FRM_SETOPERATIONRESULTTEXT: u32 = 1125;
pub const FR_DIALOGTERM: FINDREPLACE_FLAGS = 64;
pub const FR_DOWN: FINDREPLACE_FLAGS = 1;
pub const FR_ENABLEHOOK: FINDREPLACE_FLAGS = 256;
pub const FR_ENABLETEMPLATE: FINDREPLACE_FLAGS = 512;
pub const FR_ENABLETEMPLATEHANDLE: FINDREPLACE_FLAGS = 8192;
pub const FR_FINDNEXT: FINDREPLACE_FLAGS = 8;
pub const FR_HIDEMATCHCASE: FINDREPLACE_FLAGS = 32768;
pub const FR_HIDEUPDOWN: FINDREPLACE_FLAGS = 16384;
pub const FR_HIDEWHOLEWORD: FINDREPLACE_FLAGS = 65536;
pub const FR_MATCHALEFHAMZA: FINDREPLACE_FLAGS = 2147483648;
pub const FR_MATCHCASE: FINDREPLACE_FLAGS = 4;
pub const FR_MATCHDIAC: FINDREPLACE_FLAGS = 536870912;
pub const FR_MATCHKASHIDA: FINDREPLACE_FLAGS = 1073741824;
pub const FR_NOMATCHCASE: FINDREPLACE_FLAGS = 2048;
pub const FR_NOUPDOWN: FINDREPLACE_FLAGS = 1024;
pub const FR_NOWHOLEWORD: FINDREPLACE_FLAGS = 4096;
pub const FR_NOWRAPAROUND: FINDREPLACE_FLAGS = 524288;
pub const FR_RAW: FINDREPLACE_FLAGS = 131072;
pub const FR_REPLACE: FINDREPLACE_FLAGS = 16;
pub const FR_REPLACEALL: FINDREPLACE_FLAGS = 32;
pub const FR_SHOWHELP: FINDREPLACE_FLAGS = 128;
pub const FR_SHOWWRAPAROUND: FINDREPLACE_FLAGS = 262144;
pub const FR_WHOLEWORD: FINDREPLACE_FLAGS = 2;
pub const FR_WRAPAROUND: FINDREPLACE_FLAGS = 1048576;
pub const HELPMSGSTRING: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_help");
pub const HELPMSGSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_help");
pub const HELPMSGSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_help");
pub const ITALIC_FONTTYPE: CHOOSEFONT_FONT_TYPE = 512;
pub const LBSELCHSTRING: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_LBSelChangedNotify");
pub const LBSELCHSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_LBSelChangedNotify");
pub const LBSELCHSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_LBSelChangedNotify");
pub type LPCCHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPCFHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPFRHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPOFNHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPPAGEPAINTHOOK = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPPAGESETUPHOOK = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPPRINTHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub type LPSETUPHOOKPROC = Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
pub const NUM_BASIC_COLORS: u32 = 48;
pub const NUM_CUSTOM_COLORS: u32 = 16;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: windows_sys::core::PSTR,
}
#[cfg(target_arch = "x86")]
impl Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: windows_sys::core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
impl Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: windows_sys::core::PWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: windows_sys::core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OFN_ALLOWMULTISELECT: OPEN_FILENAME_FLAGS = 512;
pub const OFN_CREATEPROMPT: OPEN_FILENAME_FLAGS = 8192;
pub const OFN_DONTADDTORECENT: OPEN_FILENAME_FLAGS = 33554432;
pub const OFN_ENABLEHOOK: OPEN_FILENAME_FLAGS = 32;
pub const OFN_ENABLEINCLUDENOTIFY: OPEN_FILENAME_FLAGS = 4194304;
pub const OFN_ENABLESIZING: OPEN_FILENAME_FLAGS = 8388608;
pub const OFN_ENABLETEMPLATE: OPEN_FILENAME_FLAGS = 64;
pub const OFN_ENABLETEMPLATEHANDLE: OPEN_FILENAME_FLAGS = 128;
pub const OFN_EXPLORER: OPEN_FILENAME_FLAGS = 524288;
pub const OFN_EXTENSIONDIFFERENT: OPEN_FILENAME_FLAGS = 1024;
pub const OFN_EX_NONE: OPEN_FILENAME_FLAGS_EX = 0;
pub const OFN_EX_NOPLACESBAR: OPEN_FILENAME_FLAGS_EX = 1;
pub const OFN_FILEMUSTEXIST: OPEN_FILENAME_FLAGS = 4096;
pub const OFN_FORCESHOWHIDDEN: OPEN_FILENAME_FLAGS = 268435456;
pub const OFN_HIDEREADONLY: OPEN_FILENAME_FLAGS = 4;
pub const OFN_LONGNAMES: OPEN_FILENAME_FLAGS = 2097152;
pub const OFN_NOCHANGEDIR: OPEN_FILENAME_FLAGS = 8;
pub const OFN_NODEREFERENCELINKS: OPEN_FILENAME_FLAGS = 1048576;
pub const OFN_NOLONGNAMES: OPEN_FILENAME_FLAGS = 262144;
pub const OFN_NONETWORKBUTTON: OPEN_FILENAME_FLAGS = 131072;
pub const OFN_NOREADONLYRETURN: OPEN_FILENAME_FLAGS = 32768;
pub const OFN_NOTESTFILECREATE: OPEN_FILENAME_FLAGS = 65536;
pub const OFN_NOVALIDATE: OPEN_FILENAME_FLAGS = 256;
pub const OFN_OVERWRITEPROMPT: OPEN_FILENAME_FLAGS = 2;
pub const OFN_PATHMUSTEXIST: OPEN_FILENAME_FLAGS = 2048;
pub const OFN_READONLY: OPEN_FILENAME_FLAGS = 1;
pub const OFN_SHAREAWARE: OPEN_FILENAME_FLAGS = 16384;
pub const OFN_SHAREFALLTHROUGH: u32 = 2;
pub const OFN_SHARENOWARN: u32 = 1;
pub const OFN_SHAREWARN: u32 = 0;
pub const OFN_SHOWHELP: OPEN_FILENAME_FLAGS = 16;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_sys::core::PCSTR,
    pub lpstrCustomFilter: windows_sys::core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_sys::core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_sys::core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_sys::core::PCSTR,
    pub lpstrTitle: windows_sys::core::PCSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_sys::core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(target_arch = "x86")]
impl Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_sys::core::PCSTR,
    pub lpstrCustomFilter: windows_sys::core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_sys::core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_sys::core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_sys::core::PCSTR,
    pub lpstrTitle: windows_sys::core::PCSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_sys::core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_sys::core::PCWSTR,
    pub lpstrCustomFilter: windows_sys::core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_sys::core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_sys::core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_sys::core::PCWSTR,
    pub lpstrTitle: windows_sys::core::PCWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_sys::core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(target_arch = "x86")]
impl Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_sys::core::PCWSTR,
    pub lpstrCustomFilter: windows_sys::core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_sys::core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_sys::core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_sys::core::PCWSTR,
    pub lpstrTitle: windows_sys::core::PCWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_sys::core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_sys::core::PCSTR,
    pub lpstrCustomFilter: windows_sys::core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_sys::core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_sys::core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_sys::core::PCSTR,
    pub lpstrTitle: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_sys::core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
impl Default for OPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_sys::core::PCSTR,
    pub lpstrCustomFilter: windows_sys::core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_sys::core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_sys::core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_sys::core::PCSTR,
    pub lpstrTitle: windows_sys::core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_sys::core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_sys::core::PCWSTR,
    pub lpstrCustomFilter: windows_sys::core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_sys::core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_sys::core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_sys::core::PCWSTR,
    pub lpstrTitle: windows_sys::core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_sys::core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
impl Default for OPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: windows_sys::core::PCWSTR,
    pub lpstrCustomFilter: windows_sys::core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_sys::core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_sys::core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_sys::core::PCWSTR,
    pub lpstrTitle: windows_sys::core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_sys::core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for OPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OPEN_FILENAME_FLAGS = u32;
pub type OPEN_FILENAME_FLAGS_EX = u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_sys::core::PCSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(target_arch = "x86")]
impl Default for PAGESETUPDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_sys::core::PCSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PAGESETUPDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_sys::core::PCWSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(target_arch = "x86")]
impl Default for PAGESETUPDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_sys::core::PCWSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PAGESETUPDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PAGESETUPDLG_FLAGS = u32;
pub const PDERR_CREATEICFAILURE: COMMON_DLG_ERRORS = 4106;
pub const PDERR_DEFAULTDIFFERENT: COMMON_DLG_ERRORS = 4108;
pub const PDERR_DNDMMISMATCH: COMMON_DLG_ERRORS = 4105;
pub const PDERR_GETDEVMODEFAIL: COMMON_DLG_ERRORS = 4101;
pub const PDERR_INITFAILURE: COMMON_DLG_ERRORS = 4102;
pub const PDERR_LOADDRVFAILURE: COMMON_DLG_ERRORS = 4100;
pub const PDERR_NODEFAULTPRN: COMMON_DLG_ERRORS = 4104;
pub const PDERR_NODEVICES: COMMON_DLG_ERRORS = 4103;
pub const PDERR_PARSEFAILURE: COMMON_DLG_ERRORS = 4098;
pub const PDERR_PRINTERCODES: COMMON_DLG_ERRORS = 4096;
pub const PDERR_PRINTERNOTFOUND: COMMON_DLG_ERRORS = 4107;
pub const PDERR_RETDEFFAILURE: COMMON_DLG_ERRORS = 4099;
pub const PDERR_SETUPFAILURE: COMMON_DLG_ERRORS = 4097;
pub const PD_ALLPAGES: PRINTDLGEX_FLAGS = 0;
pub const PD_COLLATE: PRINTDLGEX_FLAGS = 16;
pub const PD_CURRENTPAGE: PRINTDLGEX_FLAGS = 4194304;
pub const PD_DISABLEPRINTTOFILE: PRINTDLGEX_FLAGS = 524288;
pub const PD_ENABLEPRINTHOOK: PRINTDLGEX_FLAGS = 4096;
pub const PD_ENABLEPRINTTEMPLATE: PRINTDLGEX_FLAGS = 16384;
pub const PD_ENABLEPRINTTEMPLATEHANDLE: PRINTDLGEX_FLAGS = 65536;
pub const PD_ENABLESETUPHOOK: PRINTDLGEX_FLAGS = 8192;
pub const PD_ENABLESETUPTEMPLATE: PRINTDLGEX_FLAGS = 32768;
pub const PD_ENABLESETUPTEMPLATEHANDLE: PRINTDLGEX_FLAGS = 131072;
pub const PD_EXCLUSIONFLAGS: PRINTDLGEX_FLAGS = 16777216;
pub const PD_HIDEPRINTTOFILE: PRINTDLGEX_FLAGS = 1048576;
pub const PD_NOCURRENTPAGE: PRINTDLGEX_FLAGS = 8388608;
pub const PD_NONETWORKBUTTON: PRINTDLGEX_FLAGS = 2097152;
pub const PD_NOPAGENUMS: PRINTDLGEX_FLAGS = 8;
pub const PD_NOSELECTION: PRINTDLGEX_FLAGS = 4;
pub const PD_NOWARNING: PRINTDLGEX_FLAGS = 128;
pub const PD_PAGENUMS: PRINTDLGEX_FLAGS = 2;
pub const PD_PRINTSETUP: PRINTDLGEX_FLAGS = 64;
pub const PD_PRINTTOFILE: PRINTDLGEX_FLAGS = 32;
pub const PD_RESULT_APPLY: u32 = 2;
pub const PD_RESULT_CANCEL: u32 = 0;
pub const PD_RESULT_PRINT: u32 = 1;
pub const PD_RETURNDC: PRINTDLGEX_FLAGS = 256;
pub const PD_RETURNDEFAULT: PRINTDLGEX_FLAGS = 1024;
pub const PD_RETURNIC: PRINTDLGEX_FLAGS = 512;
pub const PD_SELECTION: PRINTDLGEX_FLAGS = 1;
pub const PD_SHOWHELP: PRINTDLGEX_FLAGS = 2048;
pub const PD_USEDEVMODECOPIES: PRINTDLGEX_FLAGS = 262144;
pub const PD_USEDEVMODECOPIESANDCOLLATE: PRINTDLGEX_FLAGS = 262144;
pub const PD_USELARGETEMPLATE: PRINTDLGEX_FLAGS = 268435456;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_sys::core::PCSTR,
    pub lpSetupTemplateName: windows_sys::core::PCSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_sys::core::PCSTR,
    pub lpSetupTemplateName: windows_sys::core::PCSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: windows_sys::core::PCSTR,
    pub lpCallback: *mut core::ffi::c_void,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: windows_sys::core::PCSTR,
    pub lpCallback: *mut core::ffi::c_void,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: windows_sys::core::PCWSTR,
    pub lpCallback: *mut core::ffi::c_void,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpPrintTemplateName: windows_sys::core::PCWSTR,
    pub lpCallback: *mut core::ffi::c_void,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRINTDLGEX_FLAGS = u32;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_sys::core::PCWSTR,
    pub lpSetupTemplateName: windows_sys::core::PCWSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy)]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_sys::core::PCWSTR,
    pub lpSetupTemplateName: windows_sys::core::PCWSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for PRINTDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_FONTTYPE: CHOOSEFONT_FONT_TYPE = 16384;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
pub const PSD_DEFAULTMINMARGINS: PAGESETUPDLG_FLAGS = 0;
pub const PSD_DISABLEMARGINS: PAGESETUPDLG_FLAGS = 16;
pub const PSD_DISABLEORIENTATION: PAGESETUPDLG_FLAGS = 256;
pub const PSD_DISABLEPAGEPAINTING: PAGESETUPDLG_FLAGS = 524288;
pub const PSD_DISABLEPAPER: PAGESETUPDLG_FLAGS = 512;
pub const PSD_DISABLEPRINTER: PAGESETUPDLG_FLAGS = 32;
pub const PSD_ENABLEPAGEPAINTHOOK: PAGESETUPDLG_FLAGS = 262144;
pub const PSD_ENABLEPAGESETUPHOOK: PAGESETUPDLG_FLAGS = 8192;
pub const PSD_ENABLEPAGESETUPTEMPLATE: PAGESETUPDLG_FLAGS = 32768;
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: PAGESETUPDLG_FLAGS = 131072;
pub const PSD_INHUNDREDTHSOFMILLIMETERS: PAGESETUPDLG_FLAGS = 8;
pub const PSD_INTHOUSANDTHSOFINCHES: PAGESETUPDLG_FLAGS = 4;
pub const PSD_INWININIINTLMEASURE: PAGESETUPDLG_FLAGS = 0;
pub const PSD_MARGINS: PAGESETUPDLG_FLAGS = 2;
pub const PSD_MINMARGINS: PAGESETUPDLG_FLAGS = 1;
pub const PSD_NONETWORKBUTTON: PAGESETUPDLG_FLAGS = 2097152;
pub const PSD_NOWARNING: PAGESETUPDLG_FLAGS = 128;
pub const PSD_RETURNDEFAULT: PAGESETUPDLG_FLAGS = 1024;
pub const PSD_SHOWHELP: PAGESETUPDLG_FLAGS = 2048;
pub const PS_OPENTYPE_FONTTYPE: u32 = 65536;
pub const REGULAR_FONTTYPE: CHOOSEFONT_FONT_TYPE = 1024;
pub const SCREEN_FONTTYPE: CHOOSEFONT_FONT_TYPE = 8192;
pub const SETRGBSTRING: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_SetRGBColor");
pub const SETRGBSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_SetRGBColor");
pub const SETRGBSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_SetRGBColor");
pub const SHAREVISTRING: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_ShareViolation");
pub const SHAREVISTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_ShareViolation");
pub const SHAREVISTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_ShareViolation");
pub const SIMULATED_FONTTYPE: CHOOSEFONT_FONT_TYPE = 32768;
pub const START_PAGE_GENERAL: u32 = 4294967295;
pub const SYMBOL_FONTTYPE: u32 = 524288;
pub const TT_OPENTYPE_FONTTYPE: u32 = 131072;
pub const TYPE1_FONTTYPE: u32 = 262144;
pub const WM_CHOOSEFONT_GETLOGFONT: u32 = 1025;
pub const WM_CHOOSEFONT_SETFLAGS: u32 = 1126;
pub const WM_CHOOSEFONT_SETLOGFONT: u32 = 1125;
pub const WM_PSD_ENVSTAMPRECT: u32 = 1029;
pub const WM_PSD_FULLPAGERECT: u32 = 1025;
pub const WM_PSD_GREEKTEXTRECT: u32 = 1028;
pub const WM_PSD_MARGINRECT: u32 = 1027;
pub const WM_PSD_MINMARGINRECT: u32 = 1026;
pub const WM_PSD_YAFULLPAGERECT: u32 = 1030;
