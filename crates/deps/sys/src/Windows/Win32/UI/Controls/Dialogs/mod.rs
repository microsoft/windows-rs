#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: u32,
    pub lpCustColors: *mut u32,
    pub Flags: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: u32,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpszStyle: super::super::super::Foundation::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CF_APPLY: u32 = 512u32;
pub const CF_ANSIONLY: u32 = 1024u32;
pub const CF_BOTH: u32 = 3u32;
pub const CF_EFFECTS: u32 = 256u32;
pub const CF_ENABLEHOOK: u32 = 8u32;
pub const CF_ENABLETEMPLATE: u32 = 16u32;
pub const CF_ENABLETEMPLATEHANDLE: u32 = 32u32;
pub const CF_FIXEDPITCHONLY: u32 = 16384u32;
pub const CF_FORCEFONTEXIST: u32 = 65536u32;
pub const CF_INACTIVEFONTS: u32 = 33554432u32;
pub const CF_INITTOLOGFONTSTRUCT: u32 = 64u32;
pub const CF_LIMITSIZE: u32 = 8192u32;
pub const CF_NOOEMFONTS: u32 = 2048u32;
pub const CF_NOFACESEL: u32 = 524288u32;
pub const CF_NOSCRIPTSEL: u32 = 8388608u32;
pub const CF_NOSIMULATIONS: u32 = 4096u32;
pub const CF_NOSIZESEL: u32 = 2097152u32;
pub const CF_NOSTYLESEL: u32 = 1048576u32;
pub const CF_NOVECTORFONTS: u32 = 2048u32;
pub const CF_NOVERTFONTS: u32 = 16777216u32;
pub const CF_PRINTERFONTS: u32 = 2u32;
pub const CF_SCALABLEONLY: u32 = 131072u32;
pub const CF_SCREENFONTS: u32 = 1u32;
pub const CF_SCRIPTSONLY: u32 = 1024u32;
pub const CF_SELECTSCRIPT: u32 = 4194304u32;
pub const CF_SHOWHELP: u32 = 4u32;
pub const CF_TTONLY: u32 = 262144u32;
pub const CF_USESTYLE: u32 = 128u32;
pub const CF_WYSIWYG: u32 = 32768u32;
pub const BOLD_FONTTYPE: u16 = 256u16;
pub const ITALIC_FONTTYPE: u16 = 512u16;
pub const PRINTER_FONTTYPE: u16 = 16384u16;
pub const REGULAR_FONTTYPE: u16 = 1024u16;
pub const SCREEN_FONTTYPE: u16 = 8192u16;
pub const SIMULATED_FONTTYPE: u16 = 32768u16;
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
pub const CDERR_DIALOGFAILURE: u32 = 65535u32;
pub const CDERR_GENERALCODES: u32 = 0u32;
pub const CDERR_STRUCTSIZE: u32 = 1u32;
pub const CDERR_INITIALIZATION: u32 = 2u32;
pub const CDERR_NOTEMPLATE: u32 = 3u32;
pub const CDERR_NOHINSTANCE: u32 = 4u32;
pub const CDERR_LOADSTRFAILURE: u32 = 5u32;
pub const CDERR_FINDRESFAILURE: u32 = 6u32;
pub const CDERR_LOADRESFAILURE: u32 = 7u32;
pub const CDERR_LOCKRESFAILURE: u32 = 8u32;
pub const CDERR_MEMALLOCFAILURE: u32 = 9u32;
pub const CDERR_MEMLOCKFAILURE: u32 = 10u32;
pub const CDERR_NOHOOK: u32 = 11u32;
pub const CDERR_REGISTERMSGFAIL: u32 = 12u32;
pub const PDERR_PRINTERCODES: u32 = 4096u32;
pub const PDERR_SETUPFAILURE: u32 = 4097u32;
pub const PDERR_PARSEFAILURE: u32 = 4098u32;
pub const PDERR_RETDEFFAILURE: u32 = 4099u32;
pub const PDERR_LOADDRVFAILURE: u32 = 4100u32;
pub const PDERR_GETDEVMODEFAIL: u32 = 4101u32;
pub const PDERR_INITFAILURE: u32 = 4102u32;
pub const PDERR_NODEVICES: u32 = 4103u32;
pub const PDERR_NODEFAULTPRN: u32 = 4104u32;
pub const PDERR_DNDMMISMATCH: u32 = 4105u32;
pub const PDERR_CREATEICFAILURE: u32 = 4106u32;
pub const PDERR_PRINTERNOTFOUND: u32 = 4107u32;
pub const PDERR_DEFAULTDIFFERENT: u32 = 4108u32;
pub const CFERR_CHOOSEFONTCODES: u32 = 8192u32;
pub const CFERR_NOFONTS: u32 = 8193u32;
pub const CFERR_MAXLESSTHANMIN: u32 = 8194u32;
pub const FNERR_FILENAMECODES: u32 = 12288u32;
pub const FNERR_SUBCLASSFAILURE: u32 = 12289u32;
pub const FNERR_INVALIDFILENAME: u32 = 12290u32;
pub const FNERR_BUFFERTOOSMALL: u32 = 12291u32;
pub const FRERR_FINDREPLACECODES: u32 = 16384u32;
pub const FRERR_BUFFERLENGTHZERO: u32 = 16385u32;
pub const CCERR_CHOOSECOLORCODES: u32 = 20480u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for DEVNAMES {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for DEVNAMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for DEVNAMES {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for DEVNAMES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DLG_COLOR: u32 = 10u32;
pub const DN_DEFAULTPRN: u32 = 1u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PWSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: super::super::super::Foundation::PWSTR,
    pub lpstrReplaceWith: super::super::super::Foundation::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FR_DIALOGTERM: u32 = 64u32;
pub const FR_DOWN: u32 = 1u32;
pub const FR_ENABLEHOOK: u32 = 256u32;
pub const FR_ENABLETEMPLATE: u32 = 512u32;
pub const FR_ENABLETEMPLATEHANDLE: u32 = 8192u32;
pub const FR_FINDNEXT: u32 = 8u32;
pub const FR_HIDEUPDOWN: u32 = 16384u32;
pub const FR_HIDEMATCHCASE: u32 = 32768u32;
pub const FR_HIDEWHOLEWORD: u32 = 65536u32;
pub const FR_MATCHCASE: u32 = 4u32;
pub const FR_NOMATCHCASE: u32 = 2048u32;
pub const FR_NOUPDOWN: u32 = 1024u32;
pub const FR_NOWHOLEWORD: u32 = 4096u32;
pub const FR_REPLACE: u32 = 16u32;
pub const FR_REPLACEALL: u32 = 32u32;
pub const FR_SHOWHELP: u32 = 128u32;
pub const FR_WHOLEWORD: u32 = 2u32;
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
impl ::core::marker::Copy for IPrintDialogCallback {}
impl ::core::clone::Clone for IPrintDialogCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintDialogServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintDialogServices {}
impl ::core::clone::Clone for IPrintDialogServices {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OFN_SHAREFALLTHROUGH: u32 = 2u32;
pub const OFN_SHARENOWARN: u32 = 1u32;
pub const OFN_SHAREWARN: u32 = 0u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4A {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PSTR,
    pub lpstrTitle: super::super::super::Foundation::PSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4A {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4W {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4W {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lpstrFilter: super::super::super::Foundation::PWSTR,
    pub lpstrCustomFilter: super::super::super::Foundation::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: super::super::super::Foundation::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: super::super::super::Foundation::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: super::super::super::Foundation::PWSTR,
    pub lpstrTitle: super::super::super::Foundation::PWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: super::super::super::Foundation::PWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: super::super::super::Foundation::PWSTR,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4W {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4W {
    fn clone(&self) -> Self {
        *self
    }
}
pub const OFN_READONLY: u32 = 1u32;
pub const OFN_OVERWRITEPROMPT: u32 = 2u32;
pub const OFN_HIDEREADONLY: u32 = 4u32;
pub const OFN_NOCHANGEDIR: u32 = 8u32;
pub const OFN_SHOWHELP: u32 = 16u32;
pub const OFN_ENABLEHOOK: u32 = 32u32;
pub const OFN_ENABLETEMPLATE: u32 = 64u32;
pub const OFN_ENABLETEMPLATEHANDLE: u32 = 128u32;
pub const OFN_NOVALIDATE: u32 = 256u32;
pub const OFN_ALLOWMULTISELECT: u32 = 512u32;
pub const OFN_EXTENSIONDIFFERENT: u32 = 1024u32;
pub const OFN_PATHMUSTEXIST: u32 = 2048u32;
pub const OFN_FILEMUSTEXIST: u32 = 4096u32;
pub const OFN_CREATEPROMPT: u32 = 8192u32;
pub const OFN_SHAREAWARE: u32 = 16384u32;
pub const OFN_NOREADONLYRETURN: u32 = 32768u32;
pub const OFN_NOTESTFILECREATE: u32 = 65536u32;
pub const OFN_NONETWORKBUTTON: u32 = 131072u32;
pub const OFN_NOLONGNAMES: u32 = 262144u32;
pub const OFN_EXPLORER: u32 = 524288u32;
pub const OFN_NODEREFERENCELINKS: u32 = 1048576u32;
pub const OFN_LONGNAMES: u32 = 2097152u32;
pub const OFN_ENABLEINCLUDENOTIFY: u32 = 4194304u32;
pub const OFN_ENABLESIZING: u32 = 8388608u32;
pub const OFN_DONTADDTORECENT: u32 = 33554432u32;
pub const OFN_FORCESHOWHIDDEN: u32 = 268435456u32;
pub const OFN_EX_NONE: u32 = 0u32;
pub const OFN_EX_NOPLACESBAR: u32 = 1u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HINSTANCE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPageSetupTemplate: isize,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PSD_DEFAULTMINMARGINS: u32 = 0u32;
pub const PSD_DISABLEMARGINS: u32 = 16u32;
pub const PSD_DISABLEORIENTATION: u32 = 256u32;
pub const PSD_DISABLEPAGEPAINTING: u32 = 524288u32;
pub const PSD_DISABLEPAPER: u32 = 512u32;
pub const PSD_DISABLEPRINTER: u32 = 32u32;
pub const PSD_ENABLEPAGEPAINTHOOK: u32 = 262144u32;
pub const PSD_ENABLEPAGESETUPHOOK: u32 = 8192u32;
pub const PSD_ENABLEPAGESETUPTEMPLATE: u32 = 32768u32;
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: u32 = 131072u32;
pub const PSD_INHUNDREDTHSOFMILLIMETERS: u32 = 8u32;
pub const PSD_INTHOUSANDTHSOFINCHES: u32 = 4u32;
pub const PSD_INWININIINTLMEASURE: u32 = 0u32;
pub const PSD_MARGINS: u32 = 2u32;
pub const PSD_MINMARGINS: u32 = 1u32;
pub const PSD_NONETWORKBUTTON: u32 = 2097152u32;
pub const PSD_NOWARNING: u32 = 128u32;
pub const PSD_RETURNDEFAULT: u32 = 1024u32;
pub const PSD_SHOWHELP: u32 = 2048u32;
pub const PD_RESULT_APPLY: u32 = 2u32;
pub const PD_RESULT_CANCEL: u32 = 0u32;
pub const PD_RESULT_PRINT: u32 = 1u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
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
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
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
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
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
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpCallback: ::windows_sys::core::IUnknown,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGEXA {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
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
    pub lpPrintTemplateName: super::super::super::Foundation::PSTR,
    pub lpCallback: ::windows_sys::core::IUnknown,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGEXA {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
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
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpCallback: ::windows_sys::core::IUnknown,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGEXW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
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
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpCallback: ::windows_sys::core::IUnknown,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGEXW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGEXW {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PD_ALLPAGES: u32 = 0u32;
pub const PD_COLLATE: u32 = 16u32;
pub const PD_CURRENTPAGE: u32 = 4194304u32;
pub const PD_DISABLEPRINTTOFILE: u32 = 524288u32;
pub const PD_ENABLEPRINTTEMPLATE: u32 = 16384u32;
pub const PD_ENABLEPRINTTEMPLATEHANDLE: u32 = 65536u32;
pub const PD_EXCLUSIONFLAGS: u32 = 16777216u32;
pub const PD_HIDEPRINTTOFILE: u32 = 1048576u32;
pub const PD_NOCURRENTPAGE: u32 = 8388608u32;
pub const PD_NOPAGENUMS: u32 = 8u32;
pub const PD_NOSELECTION: u32 = 4u32;
pub const PD_NOWARNING: u32 = 128u32;
pub const PD_PAGENUMS: u32 = 2u32;
pub const PD_PRINTTOFILE: u32 = 32u32;
pub const PD_RETURNDC: u32 = 256u32;
pub const PD_RETURNDEFAULT: u32 = 1024u32;
pub const PD_RETURNIC: u32 = 512u32;
pub const PD_SELECTION: u32 = 1u32;
pub const PD_USEDEVMODECOPIES: u32 = 262144u32;
pub const PD_USEDEVMODECOPIESANDCOLLATE: u32 = 262144u32;
pub const PD_USELARGETEMPLATE: u32 = 268435456u32;
pub const PD_ENABLEPRINTHOOK: u32 = 4096u32;
pub const PD_ENABLESETUPHOOK: u32 = 8192u32;
pub const PD_ENABLESETUPTEMPLATE: u32 = 32768u32;
pub const PD_ENABLESETUPTEMPLATEHANDLE: u32 = 131072u32;
pub const PD_NONETWORKBUTTON: u32 = 2097152u32;
pub const PD_PRINTSETUP: u32 = 64u32;
pub const PD_SHOWHELP: u32 = 2048u32;
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
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
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGW {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: isize,
    pub hDevNames: isize,
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
    pub lpPrintTemplateName: super::super::super::Foundation::PWSTR,
    pub lpSetupTemplateName: super::super::super::Foundation::PWSTR,
    pub hPrintTemplate: isize,
    pub hSetupTemplate: isize,
}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGW {}
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::marker::Copy for PRINTPAGERANGE {}
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
impl ::core::clone::Clone for PRINTPAGERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[cfg(any(target_arch = "x86",))]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
#[cfg(any(target_arch = "x86",))]
impl ::core::marker::Copy for PRINTPAGERANGE {}
#[cfg(any(target_arch = "x86",))]
impl ::core::clone::Clone for PRINTPAGERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
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
