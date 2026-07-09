#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn ChooseColorA(param0 : *mut CHOOSECOLORA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn ChooseColorW(param0 : *mut CHOOSECOLORW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
windows_link::link!("comdlg32.dll" "system" fn ChooseFontA(param0 : *mut CHOOSEFONTA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
windows_link::link!("comdlg32.dll" "system" fn ChooseFontW(param0 : *mut CHOOSEFONTW) -> windows_sys::core::BOOL);
windows_link::link!("comdlg32.dll" "system" fn CommDlgExtendedError() -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn FindTextA(param0 : *mut FINDREPLACEA) -> super::windef::HWND);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn FindTextW(param0 : *mut FINDREPLACEW) -> super::windef::HWND);
windows_link::link!("comdlg32.dll" "system" fn GetFileTitleA(param0 : windows_sys::core::PCSTR, buf : windows_sys::core::PSTR, cchsize : u16) -> i16);
windows_link::link!("comdlg32.dll" "system" fn GetFileTitleW(param0 : windows_sys::core::PCWSTR, buf : windows_sys::core::PWSTR, cchsize : u16) -> i16);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn GetOpenFileNameA(param0 : *mut OPENFILENAMEA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn GetOpenFileNameW(param0 : *mut OPENFILENAMEW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn GetSaveFileNameA(param0 : *mut OPENFILENAMEA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn GetSaveFileNameW(param0 : *mut OPENFILENAMEW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("comdlg32.dll" "system" fn PageSetupDlgA(param0 : *mut PAGESETUPDLGA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("comdlg32.dll" "system" fn PageSetupDlgW(param0 : *mut PAGESETUPDLGW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("comdlg32.dll" "system" fn PrintDlgA(ppd : *mut PRINTDLGA) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("comdlg32.dll" "system" fn PrintDlgExA(ppd : *mut PRINTDLGEXA) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("comdlg32.dll" "system" fn PrintDlgExW(ppd : *mut PRINTDLGEXW) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
windows_link::link!("comdlg32.dll" "system" fn PrintDlgW(ppd : *mut PRINTDLGW) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn ReplaceTextA(param0 : *mut FINDREPLACEA) -> super::windef::HWND);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
windows_link::link!("comdlg32.dll" "system" fn ReplaceTextW(param0 : *mut FINDREPLACEW) -> super::windef::HWND);
pub const BOLD_FONTTYPE: u32 = 256;
pub const CC_ANYCOLOR: u32 = 256;
pub const CC_ENABLEHOOK: u32 = 16;
pub const CC_ENABLETEMPLATE: u32 = 32;
pub const CC_ENABLETEMPLATEHANDLE: u32 = 64;
pub const CC_FULLOPEN: u32 = 2;
pub const CC_PREVENTFULLOPEN: u32 = 4;
pub const CC_RGBINIT: u32 = 1;
pub const CC_SHOWHELP: u32 = 8;
pub const CC_SOLIDCOLOR: u32 = 128;
pub const CDM_FIRST: u32 = 1124;
pub const CDM_GETFILEPATH: u32 = 1125;
pub const CDM_GETFOLDERIDLIST: u32 = 1127;
pub const CDM_GETFOLDERPATH: u32 = 1126;
pub const CDM_GETSPEC: u32 = 1124;
pub const CDM_HIDECONTROL: u32 = 1129;
pub const CDM_LAST: u32 = 1224;
pub const CDM_SETCONTROLTEXT: u32 = 1128;
pub const CDM_SETDEFEXT: u32 = 1130;
pub const CDN_FILEOK: i32 = -606;
pub const CDN_FIRST: i32 = -601;
pub const CDN_FOLDERCHANGE: i32 = -603;
pub const CDN_HELP: i32 = -605;
pub const CDN_INCLUDEITEM: i32 = -608;
pub const CDN_INITDONE: i32 = -601;
pub const CDN_LAST: i32 = -699;
pub const CDN_SELCHANGE: i32 = -602;
pub const CDN_SHAREVIOLATION: i32 = -604;
pub const CDN_TYPECHANGE: i32 = -607;
pub const CD_LBSELADD: u32 = 2;
pub const CD_LBSELCHANGE: u32 = 0;
pub const CD_LBSELNOITEMS: i32 = -1;
pub const CD_LBSELSUB: u32 = 1;
pub const CF_ANSIONLY: u32 = 1024;
pub const CF_APPLY: u32 = 512;
pub const CF_BOTH: u32 = 3;
pub const CF_EFFECTS: u32 = 256;
pub const CF_ENABLEHOOK: u32 = 8;
pub const CF_ENABLETEMPLATE: u32 = 16;
pub const CF_ENABLETEMPLATEHANDLE: u32 = 32;
pub const CF_FIXEDPITCHONLY: u32 = 16384;
pub const CF_FORCEFONTEXIST: u32 = 65536;
pub const CF_INACTIVEFONTS: u32 = 33554432;
pub const CF_INITTOLOGFONTSTRUCT: u32 = 64;
pub const CF_LIMITSIZE: u32 = 8192;
pub const CF_NOFACESEL: u32 = 524288;
pub const CF_NOOEMFONTS: u32 = 2048;
pub const CF_NOSCRIPTSEL: u32 = 8388608;
pub const CF_NOSIMULATIONS: u32 = 4096;
pub const CF_NOSIZESEL: u32 = 2097152;
pub const CF_NOSTYLESEL: u32 = 1048576;
pub const CF_NOVECTORFONTS: u32 = 2048;
pub const CF_NOVERTFONTS: u32 = 16777216;
pub const CF_PRINTERFONTS: u32 = 2;
pub const CF_SCALABLEONLY: u32 = 131072;
pub const CF_SCREENFONTS: u32 = 1;
pub const CF_SCRIPTSONLY: u32 = 1024;
pub const CF_SELECTSCRIPT: u32 = 4194304;
pub const CF_SHOWHELP: u32 = 4;
pub const CF_TTONLY: u32 = 262144;
pub const CF_USESTYLE: u32 = 128;
pub const CF_WYSIWYG: u32 = 32768;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type CHOOSECOLOR = CHOOSECOLORA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::windef::HWND,
    pub rgbResult: super::windef::COLORREF,
    pub lpCustColors: *mut super::windef::COLORREF,
    pub Flags: u32,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::windef::HWND,
    pub rgbResult: super::windef::COLORREF,
    pub lpCustColors: *mut super::windef::COLORREF,
    pub Flags: u32,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::windef::HWND,
    pub rgbResult: super::windef::COLORREF,
    pub lpCustColors: *mut super::windef::COLORREF,
    pub Flags: u32,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::windef::HWND,
    pub rgbResult: super::windef::COLORREF,
    pub lpCustColors: *mut super::windef::COLORREF,
    pub Flags: u32,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type CHOOSEFONT = CHOOSEFONTA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDC: super::windef::HDC,
    pub lpLogFont: super::wingdi::LPLOGFONTA,
    pub iPointSize: i32,
    pub Flags: u32,
    pub rgbColors: super::windef::COLORREF,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpszStyle: windows_sys::core::PSTR,
    pub nFontType: u16,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDC: super::windef::HDC,
    pub lpLogFont: super::wingdi::LPLOGFONTA,
    pub iPointSize: i32,
    pub Flags: u32,
    pub rgbColors: super::windef::COLORREF,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpszStyle: windows_sys::core::PSTR,
    pub nFontType: u16,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDC: super::windef::HDC,
    pub lpLogFont: super::wingdi::LPLOGFONTW,
    pub iPointSize: i32,
    pub Flags: u32,
    pub rgbColors: super::windef::COLORREF,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpszStyle: windows_sys::core::PWSTR,
    pub nFontType: u16,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
#[derive(Clone, Copy)]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDC: super::windef::HDC,
    pub lpLogFont: super::wingdi::LPLOGFONTW,
    pub iPointSize: i32,
    pub Flags: u32,
    pub rgbColors: super::windef::COLORREF,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpszStyle: windows_sys::core::PWSTR,
    pub nFontType: u16,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
impl Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COLOROKSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_ColorOK");
pub const COLOROKSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_ColorOK");
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
pub const DN_DEFAULTPRN: u32 = 1;
pub const FILEOKSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_FileNameOK");
pub const FILEOKSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_FileNameOK");
pub const FINDMSGSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_FindReplace");
pub const FINDMSGSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_FindReplace");
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type FINDREPLACE = FINDREPLACEA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub Flags: u32,
    pub lpstrFindWhat: windows_sys::core::PSTR,
    pub lpstrReplaceWith: windows_sys::core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for FINDREPLACEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub Flags: u32,
    pub lpstrFindWhat: windows_sys::core::PSTR,
    pub lpstrReplaceWith: windows_sys::core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for FINDREPLACEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub Flags: u32,
    pub lpstrFindWhat: windows_sys::core::PWSTR,
    pub lpstrReplaceWith: windows_sys::core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for FINDREPLACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub Flags: u32,
    pub lpstrFindWhat: windows_sys::core::PWSTR,
    pub lpstrReplaceWith: windows_sys::core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for FINDREPLACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FRM_FIRST: u32 = 1124;
pub const FRM_LAST: u32 = 1224;
pub const FRM_SETOPERATIONRESULT: u32 = 1124;
pub const FRM_SETOPERATIONRESULTTEXT: u32 = 1125;
pub const FR_DIALOGTERM: u32 = 64;
pub const FR_DOWN: u32 = 1;
pub const FR_ENABLEHOOK: u32 = 256;
pub const FR_ENABLETEMPLATE: u32 = 512;
pub const FR_ENABLETEMPLATEHANDLE: u32 = 8192;
pub const FR_FINDNEXT: u32 = 8;
pub const FR_HIDEMATCHCASE: u32 = 32768;
pub const FR_HIDEUPDOWN: u32 = 16384;
pub const FR_HIDEWHOLEWORD: u32 = 65536;
pub const FR_MATCHALEFHAMZA: u32 = 2147483648;
pub const FR_MATCHCASE: u32 = 4;
pub const FR_MATCHDIAC: u32 = 536870912;
pub const FR_MATCHKASHIDA: u32 = 1073741824;
pub const FR_NOMATCHCASE: u32 = 2048;
pub const FR_NOUPDOWN: u32 = 1024;
pub const FR_NOWHOLEWORD: u32 = 4096;
pub const FR_NOWRAPAROUND: u32 = 524288;
pub const FR_RAW: u32 = 131072;
pub const FR_REPLACE: u32 = 16;
pub const FR_REPLACEALL: u32 = 32;
pub const FR_SHOWHELP: u32 = 128;
pub const FR_SHOWWRAPAROUND: u32 = 262144;
pub const FR_WHOLEWORD: u32 = 2;
pub const FR_WRAPAROUND: u32 = 1048576;
pub const HELPMSGSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_help");
pub const HELPMSGSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_help");
pub const ITALIC_FONTTYPE: u32 = 512;
pub const LBSELCHSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_LBSelChangedNotify");
pub const LBSELCHSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_LBSelChangedNotify");
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPCCHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPCFHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPCHOOSECOLOR = LPCHOOSECOLORA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPCHOOSECOLORA = *mut CHOOSECOLORA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPCHOOSECOLORW = *mut CHOOSECOLORW;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type LPCHOOSEFONT = LPCHOOSEFONTA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type LPCHOOSEFONTA = *mut CHOOSEFONTA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type LPCHOOSEFONTW = *mut CHOOSEFONTW;
pub type LPDEVNAMES = *mut DEVNAMES;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPFINDREPLACE = LPFINDREPLACEA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPFINDREPLACEA = *mut FINDREPLACEA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPFINDREPLACEW = *mut FINDREPLACEW;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPFRHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPOFNHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type LPOFNOTIFY = LPOFNOTIFYA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type LPOFNOTIFYA = *mut OFNOTIFYA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type LPOFNOTIFYEX = LPOFNOTIFYEXA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type LPOFNOTIFYEXA = *mut OFNOTIFYEXA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type LPOFNOTIFYEXW = *mut OFNOTIFYEXW;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type LPOFNOTIFYW = *mut OFNOTIFYW;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPOPENFILENAME = LPOPENFILENAMEA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPOPENFILENAMEA = *mut OPENFILENAMEA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPOPENFILENAMEW = *mut OPENFILENAMEW;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPOPENFILENAME_NT4 = LPOPENFILENAME_NT4A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPOPENFILENAME_NT4A = *mut OPENFILENAME_NT4A;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPOPENFILENAME_NT4W = *mut OPENFILENAME_NT4W;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPPAGEPAINTHOOK = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPPAGESETUPDLG = LPPAGESETUPDLGA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPPAGESETUPDLGA = *mut PAGESETUPDLGA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPPAGESETUPDLGW = *mut PAGESETUPDLGW;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPPAGESETUPHOOK = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPPRINTDLG = LPPRINTDLGA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPPRINTDLGA = *mut PRINTDLGA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPPRINTDLGEX = LPPRINTDLGEXA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPPRINTDLGEXA = *mut PRINTDLGEXA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPPRINTDLGEXW = *mut PRINTDLGEXW;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type LPPRINTDLGW = *mut PRINTDLGW;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPPRINTHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
pub type LPPRINTPAGERANGE = *mut PRINTPAGERANGE;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type LPSETUPHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type OFNOTIFY = OFNOTIFYA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYA {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub pszFile: windows_sys::core::PSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYA {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub pszFile: windows_sys::core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
pub type OFNOTIFYEX = OFNOTIFYEXA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXA {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXA {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXW {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXW {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYW {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub pszFile: windows_sys::core::PWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYW {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub pszFile: windows_sys::core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winuser"))]
impl Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OFN_ALLOWMULTISELECT: u32 = 512;
pub const OFN_CREATEPROMPT: u32 = 8192;
pub const OFN_DONTADDTORECENT: u32 = 33554432;
pub const OFN_ENABLEHOOK: u32 = 32;
pub const OFN_ENABLEINCLUDENOTIFY: u32 = 4194304;
pub const OFN_ENABLESIZING: u32 = 8388608;
pub const OFN_ENABLETEMPLATE: u32 = 64;
pub const OFN_ENABLETEMPLATEHANDLE: u32 = 128;
pub const OFN_EXPLORER: u32 = 524288;
pub const OFN_EXTENSIONDIFFERENT: u32 = 1024;
pub const OFN_EX_NOPLACESBAR: u32 = 1;
pub const OFN_FILEMUSTEXIST: u32 = 4096;
pub const OFN_FORCESHOWHIDDEN: u32 = 268435456;
pub const OFN_HIDEREADONLY: u32 = 4;
pub const OFN_LONGNAMES: u32 = 2097152;
pub const OFN_NOCHANGEDIR: u32 = 8;
pub const OFN_NODEREFERENCELINKS: u32 = 1048576;
pub const OFN_NOLONGNAMES: u32 = 262144;
pub const OFN_NONETWORKBUTTON: u32 = 131072;
pub const OFN_NOREADONLYRETURN: u32 = 32768;
pub const OFN_NOTESTFILECREATE: u32 = 65536;
pub const OFN_NOVALIDATE: u32 = 256;
pub const OFN_OVERWRITEPROMPT: u32 = 2;
pub const OFN_PATHMUSTEXIST: u32 = 2048;
pub const OFN_READONLY: u32 = 1;
pub const OFN_SHAREAWARE: u32 = 16384;
pub const OFN_SHAREFALLTHROUGH: u32 = 2;
pub const OFN_SHARENOWARN: u32 = 1;
pub const OFN_SHAREWARN: u32 = 0;
pub const OFN_SHOWHELP: u32 = 16;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type OPENFILENAME = OPENFILENAMEA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
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
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
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
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
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
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
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
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
pub type OPENFILENAME_NT4 = OPENFILENAME_NT4A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
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
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for OPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
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
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for OPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
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
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for OPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
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
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_sys::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef"))]
impl Default for OPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const OPENFILENAME_SIZE_VERSION_400: u32 = 76;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const OPENFILENAME_SIZE_VERSION_400: u32 = 136;
#[cfg(target_arch = "x86")]
pub const OPENFILENAME_SIZE_VERSION_400A: u32 = 76;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const OPENFILENAME_SIZE_VERSION_400A: u32 = 136;
#[cfg(target_arch = "x86")]
pub const OPENFILENAME_SIZE_VERSION_400W: u32 = 76;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const OPENFILENAME_SIZE_VERSION_400W: u32 = 136;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PAGESETUPDLG = PAGESETUPDLGA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub Flags: u32,
    pub ptPaperSize: super::windef::POINT,
    pub rtMinMargin: super::windef::RECT,
    pub rtMargin: super::windef::RECT,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_sys::core::PCSTR,
    pub hPageSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PAGESETUPDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub Flags: u32,
    pub ptPaperSize: super::windef::POINT,
    pub rtMinMargin: super::windef::RECT,
    pub rtMargin: super::windef::RECT,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_sys::core::PCSTR,
    pub hPageSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PAGESETUPDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub Flags: u32,
    pub ptPaperSize: super::windef::POINT,
    pub rtMinMargin: super::windef::RECT,
    pub rtMargin: super::windef::RECT,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_sys::core::PCWSTR,
    pub hPageSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PAGESETUPDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub Flags: u32,
    pub ptPaperSize: super::windef::POINT,
    pub rtMinMargin: super::windef::RECT,
    pub rtMargin: super::windef::RECT,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_sys::core::PCWSTR,
    pub hPageSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PAGESETUPDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PCCHOOSEFONT = PCCHOOSEFONTA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PCCHOOSEFONTA = *const CHOOSEFONTA;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_wingdi"))]
pub type PCCHOOSEFONTW = *const CHOOSEFONTW;
pub type PCDEVNAMES = *const DEVNAMES;
pub type PCPRINTPAGERANGE = *const PRINTPAGERANGE;
pub const PD_ALLPAGES: u32 = 0;
pub const PD_COLLATE: u32 = 16;
pub const PD_CURRENTPAGE: u32 = 4194304;
pub const PD_DISABLEPRINTTOFILE: u32 = 524288;
pub const PD_ENABLEPRINTHOOK: u32 = 4096;
pub const PD_ENABLEPRINTTEMPLATE: u32 = 16384;
pub const PD_ENABLEPRINTTEMPLATEHANDLE: u32 = 65536;
pub const PD_ENABLESETUPHOOK: u32 = 8192;
pub const PD_ENABLESETUPTEMPLATE: u32 = 32768;
pub const PD_ENABLESETUPTEMPLATEHANDLE: u32 = 131072;
pub const PD_EXCLUSIONFLAGS: u32 = 16777216;
pub const PD_EXCL_COPIESANDCOLLATE: u32 = 33024;
pub const PD_HIDEPRINTTOFILE: u32 = 1048576;
pub const PD_NOCURRENTPAGE: u32 = 8388608;
pub const PD_NONETWORKBUTTON: u32 = 2097152;
pub const PD_NOPAGENUMS: u32 = 8;
pub const PD_NOSELECTION: u32 = 4;
pub const PD_NOWARNING: u32 = 128;
pub const PD_PAGENUMS: u32 = 2;
pub const PD_PRINTSETUP: u32 = 64;
pub const PD_PRINTTOFILE: u32 = 32;
pub const PD_RESULT_APPLY: u32 = 2;
pub const PD_RESULT_CANCEL: u32 = 0;
pub const PD_RESULT_PRINT: u32 = 1;
pub const PD_RETURNDC: u32 = 256;
pub const PD_RETURNDEFAULT: u32 = 1024;
pub const PD_RETURNIC: u32 = 512;
pub const PD_SELECTION: u32 = 1;
pub const PD_SHOWHELP: u32 = 2048;
pub const PD_USEDEVMODECOPIES: u32 = 262144;
pub const PD_USEDEVMODECOPIESANDCOLLATE: u32 = 262144;
pub const PD_USELARGETEMPLATE: u32 = 268435456;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PRINTDLG = PRINTDLGA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub hDC: super::windef::HDC,
    pub Flags: u32,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_sys::core::PCSTR,
    pub lpSetupTemplateName: windows_sys::core::PCSTR,
    pub hPrintTemplate: super::minwindef::HGLOBAL,
    pub hSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PRINTDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub hDC: super::windef::HDC,
    pub Flags: u32,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_sys::core::PCSTR,
    pub lpSetupTemplateName: windows_sys::core::PCSTR,
    pub hPrintTemplate: super::minwindef::HGLOBAL,
    pub hSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PRINTDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
pub type PRINTDLGEX = PRINTDLGEXA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub hDC: super::windef::HDC,
    pub Flags: u32,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: LPPRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpPrintTemplateName: windows_sys::core::PCSTR,
    pub lpCallback: *mut core::ffi::c_void,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::prsht::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub hDC: super::windef::HDC,
    pub Flags: u32,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: LPPRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpPrintTemplateName: windows_sys::core::PCSTR,
    pub lpCallback: *mut core::ffi::c_void,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::prsht::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub hDC: super::windef::HDC,
    pub Flags: u32,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: LPPRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpPrintTemplateName: windows_sys::core::PCWSTR,
    pub lpCallback: *mut core::ffi::c_void,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::prsht::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub hDC: super::windef::HDC,
    pub Flags: u32,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: LPPRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpPrintTemplateName: windows_sys::core::PCWSTR,
    pub lpCallback: *mut core::ffi::c_void,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::prsht::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_prsht", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub hDC: super::windef::HDC,
    pub Flags: u32,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_sys::core::PCWSTR,
    pub lpSetupTemplateName: windows_sys::core::PCWSTR,
    pub hPrintTemplate: super::minwindef::HGLOBAL,
    pub hSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PRINTDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hDevMode: super::minwindef::HGLOBAL,
    pub hDevNames: super::minwindef::HGLOBAL,
    pub hDC: super::windef::HDC,
    pub Flags: u32,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_sys::core::PCWSTR,
    pub lpSetupTemplateName: windows_sys::core::PCWSTR,
    pub hPrintTemplate: super::minwindef::HGLOBAL,
    pub hSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_windef", feature = "Win32_winnt"))]
impl Default for PRINTDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRINTER_FONTTYPE: u32 = 16384;
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
pub const PSD_DEFAULTMINMARGINS: u32 = 0;
pub const PSD_DISABLEMARGINS: u32 = 16;
pub const PSD_DISABLEORIENTATION: u32 = 256;
pub const PSD_DISABLEPAGEPAINTING: u32 = 524288;
pub const PSD_DISABLEPAPER: u32 = 512;
pub const PSD_DISABLEPRINTER: u32 = 32;
pub const PSD_ENABLEPAGEPAINTHOOK: u32 = 262144;
pub const PSD_ENABLEPAGESETUPHOOK: u32 = 8192;
pub const PSD_ENABLEPAGESETUPTEMPLATE: u32 = 32768;
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: u32 = 131072;
pub const PSD_INHUNDREDTHSOFMILLIMETERS: u32 = 8;
pub const PSD_INTHOUSANDTHSOFINCHES: u32 = 4;
pub const PSD_INWININIINTLMEASURE: u32 = 0;
pub const PSD_MARGINS: u32 = 2;
pub const PSD_MINMARGINS: u32 = 1;
pub const PSD_NONETWORKBUTTON: u32 = 2097152;
pub const PSD_NOWARNING: u32 = 128;
pub const PSD_RETURNDEFAULT: u32 = 1024;
pub const PSD_SHOWHELP: u32 = 2048;
pub const PS_OPENTYPE_FONTTYPE: u32 = 65536;
pub const REGULAR_FONTTYPE: u32 = 1024;
pub const SCREEN_FONTTYPE: u32 = 8192;
pub const SETRGBSTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_SetRGBColor");
pub const SETRGBSTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_SetRGBColor");
pub const SHAREVISTRINGA: windows_sys::core::PCSTR = windows_sys::core::s!("commdlg_ShareViolation");
pub const SHAREVISTRINGW: windows_sys::core::PCWSTR = windows_sys::core::w!("commdlg_ShareViolation");
pub const SIMULATED_FONTTYPE: u32 = 32768;
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
pub const WM_PSD_PAGESETUPDLG: u32 = 1024;
pub const WM_PSD_YAFULLPAGERECT: u32 = 1030;
