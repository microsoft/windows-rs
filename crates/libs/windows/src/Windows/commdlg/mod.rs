#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ChooseColorA(param0: *mut CHOOSECOLORA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseColorA(param0 : *mut CHOOSECOLORA) -> windows_core::BOOL);
    unsafe { ChooseColorA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ChooseColorW(param0: *mut CHOOSECOLORW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseColorW(param0 : *mut CHOOSECOLORW) -> windows_core::BOOL);
    unsafe { ChooseColorW(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ChooseFontA(param0: *mut CHOOSEFONTA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseFontA(param0 : *mut CHOOSEFONTA) -> windows_core::BOOL);
    unsafe { ChooseFontA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ChooseFontW(param0: *mut CHOOSEFONTW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseFontW(param0 : *mut CHOOSEFONTW) -> windows_core::BOOL);
    unsafe { ChooseFontW(param0 as _) }
}
#[inline]
pub unsafe fn CommDlgExtendedError() -> u32 {
    windows_core::link!("comdlg32.dll" "system" fn CommDlgExtendedError() -> u32);
    unsafe { CommDlgExtendedError() }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn FindTextA(param0: *mut FINDREPLACEA) -> super::windef::HWND {
    windows_core::link!("comdlg32.dll" "system" fn FindTextA(param0 : *mut FINDREPLACEA) -> super::windef::HWND);
    unsafe { FindTextA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn FindTextW(param0: *mut FINDREPLACEW) -> super::windef::HWND {
    windows_core::link!("comdlg32.dll" "system" fn FindTextW(param0 : *mut FINDREPLACEW) -> super::windef::HWND);
    unsafe { FindTextW(param0 as _) }
}
#[inline]
pub unsafe fn GetFileTitleA<P0>(param0: P0, buf: &mut [u8]) -> i16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comdlg32.dll" "system" fn GetFileTitleA(param0 : windows_core::PCSTR, buf : windows_core::PSTR, cchsize : u16) -> i16);
    unsafe { GetFileTitleA(param0.param().abi(), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetFileTitleW<P0>(param0: P0, buf: &mut [u16]) -> i16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comdlg32.dll" "system" fn GetFileTitleW(param0 : windows_core::PCWSTR, buf : windows_core::PWSTR, cchsize : u16) -> i16);
    unsafe { GetFileTitleW(param0.param().abi(), core::mem::transmute(buf.as_ptr()), buf.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetOpenFileNameA(param0: *mut OPENFILENAMEA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetOpenFileNameA(param0 : *mut OPENFILENAMEA) -> windows_core::BOOL);
    unsafe { GetOpenFileNameA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetOpenFileNameW(param0: *mut OPENFILENAMEW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetOpenFileNameW(param0 : *mut OPENFILENAMEW) -> windows_core::BOOL);
    unsafe { GetOpenFileNameW(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetSaveFileNameA(param0: *mut OPENFILENAMEA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetSaveFileNameA(param0 : *mut OPENFILENAMEA) -> windows_core::BOOL);
    unsafe { GetSaveFileNameA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn GetSaveFileNameW(param0: *mut OPENFILENAMEW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetSaveFileNameW(param0 : *mut OPENFILENAMEW) -> windows_core::BOOL);
    unsafe { GetSaveFileNameW(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn PageSetupDlgA(param0: *mut PAGESETUPDLGA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PageSetupDlgA(param0 : *mut PAGESETUPDLGA) -> windows_core::BOOL);
    unsafe { PageSetupDlgA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn PageSetupDlgW(param0: *mut PAGESETUPDLGW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PageSetupDlgW(param0 : *mut PAGESETUPDLGW) -> windows_core::BOOL);
    unsafe { PageSetupDlgW(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn PrintDlgA(ppd: *mut PRINTDLGA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgA(ppd : *mut PRINTDLGA) -> windows_core::BOOL);
    unsafe { PrintDlgA(ppd as _) }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn PrintDlgExA(ppd: *mut PRINTDLGEXA) -> windows_core::HRESULT {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgExA(ppd : *mut PRINTDLGEXA) -> windows_core::HRESULT);
    unsafe { PrintDlgExA(core::mem::transmute(ppd)) }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn PrintDlgExW(ppd: *mut PRINTDLGEXW) -> windows_core::HRESULT {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgExW(ppd : *mut PRINTDLGEXW) -> windows_core::HRESULT);
    unsafe { PrintDlgExW(core::mem::transmute(ppd)) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn PrintDlgW(ppd: *mut PRINTDLGW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgW(ppd : *mut PRINTDLGW) -> windows_core::BOOL);
    unsafe { PrintDlgW(ppd as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ReplaceTextA(param0: *mut FINDREPLACEA) -> super::windef::HWND {
    windows_core::link!("comdlg32.dll" "system" fn ReplaceTextA(param0 : *mut FINDREPLACEA) -> super::windef::HWND);
    unsafe { ReplaceTextA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[inline]
pub unsafe fn ReplaceTextW(param0: *mut FINDREPLACEW) -> super::windef::HWND {
    windows_core::link!("comdlg32.dll" "system" fn ReplaceTextW(param0 : *mut FINDREPLACEW) -> super::windef::HWND);
    unsafe { ReplaceTextW(param0 as _) }
}
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
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type CHOOSECOLOR = CHOOSECOLORA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
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
    pub lpTemplateName: windows_core::PCSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug)]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::windef::HWND,
    pub rgbResult: super::windef::COLORREF,
    pub lpCustColors: *mut super::windef::COLORREF,
    pub Flags: u32,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
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
    pub lpTemplateName: windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug)]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::windef::HWND,
    pub rgbResult: super::windef::COLORREF,
    pub lpCustColors: *mut super::windef::COLORREF,
    pub Flags: u32,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
pub type CHOOSEFONT = CHOOSEFONTA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
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
    pub lpTemplateName: windows_core::PCSTR,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpszStyle: windows_core::PSTR,
    pub nFontType: u16,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, Default)]
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
    pub lpTemplateName: windows_core::PCSTR,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpszStyle: windows_core::PSTR,
    pub nFontType: u16,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
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
    pub lpTemplateName: windows_core::PCWSTR,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpszStyle: windows_core::PWSTR,
    pub nFontType: u16,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, Default)]
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
    pub lpTemplateName: windows_core::PCWSTR,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpszStyle: windows_core::PWSTR,
    pub nFontType: u16,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
pub const COLOROKSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_ColorOK");
pub const COLOROKSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_ColorOK");
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
pub const DN_DEFAULTPRN: u32 = 1;
pub const FILEOKSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_FileNameOK");
pub const FILEOKSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_FileNameOK");
pub const FINDMSGSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_FindReplace");
pub const FINDMSGSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_FindReplace");
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type FINDREPLACE = FINDREPLACEA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub Flags: u32,
    pub lpstrFindWhat: windows_core::PSTR,
    pub lpstrReplaceWith: windows_core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub Flags: u32,
    pub lpstrFindWhat: windows_core::PSTR,
    pub lpstrReplaceWith: windows_core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub Flags: u32,
    pub lpstrFindWhat: windows_core::PWSTR,
    pub lpstrReplaceWith: windows_core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub Flags: u32,
    pub lpstrFindWhat: windows_core::PWSTR,
    pub lpstrReplaceWith: windows_core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
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
pub const HELPMSGSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_help");
pub const HELPMSGSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_help");
windows_core::imp::define_interface!(IPrintDialogCallback, IPrintDialogCallback_Vtbl, 0x5852a2c3_6530_11d1_b6a3_0000f8757bf9);
windows_core::imp::interface_hierarchy!(IPrintDialogCallback, windows_core::IUnknown);
impl IPrintDialogCallback {
    pub unsafe fn InitDone(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitDone)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SelectionChange(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SelectionChange)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub unsafe fn HandleMessage(&self, hdlg: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<super::minwindef::LRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HandleMessage)(windows_core::Interface::as_raw(self), hdlg, umsg, wparam, lparam, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitDone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "windef"))]
    pub HandleMessage: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, u32, super::minwindef::WPARAM, super::minwindef::LPARAM, *mut super::minwindef::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "windef")))]
    HandleMessage: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub trait IPrintDialogCallback_Impl: windows_core::IUnknownImpl {
    fn InitDone(&self) -> windows_core::Result<()>;
    fn SelectionChange(&self) -> windows_core::Result<()>;
    fn HandleMessage(&self, hdlg: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM) -> windows_core::Result<super::minwindef::LRESULT>;
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl IPrintDialogCallback_Vtbl {
    pub const fn new<Identity: IPrintDialogCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitDone<Identity: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogCallback_Impl::InitDone(this).into()
            }
        }
        unsafe extern "system" fn SelectionChange<Identity: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogCallback_Impl::SelectionChange(this).into()
            }
        }
        unsafe extern "system" fn HandleMessage<Identity: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdlg: super::windef::HWND, umsg: u32, wparam: super::minwindef::WPARAM, lparam: super::minwindef::LPARAM, presult: *mut super::minwindef::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPrintDialogCallback_Impl::HandleMessage(this, core::mem::transmute_copy(&hdlg), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam)) {
                    Ok(ok__) => {
                        presult.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitDone: InitDone::<Identity, OFFSET>,
            SelectionChange: SelectionChange::<Identity, OFFSET>,
            HandleMessage: HandleMessage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDialogCallback as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl windows_core::RuntimeName for IPrintDialogCallback {}
windows_core::imp::define_interface!(IPrintDialogServices, IPrintDialogServices_Vtbl, 0x509aaeda_5639_11d1_b6a1_0000f8757bf9);
windows_core::imp::interface_hierarchy!(IPrintDialogServices, windows_core::IUnknown);
impl IPrintDialogServices {
    #[cfg(all(feature = "windef", feature = "wingdi"))]
    pub unsafe fn GetCurrentDevMode(&self, pdevmode: super::wingdi::LPDEVMODE, pcbsize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentDevMode)(windows_core::Interface::as_raw(self), pdevmode as _, pcbsize as _) }
    }
    pub unsafe fn GetCurrentPrinterName(&self, pprintername: Option<windows_core::PWSTR>, pcchsize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPrinterName)(windows_core::Interface::as_raw(self), pprintername.unwrap_or(core::mem::zeroed()) as _, pcchsize as _) }
    }
    pub unsafe fn GetCurrentPortName(&self, pportname: Option<windows_core::PWSTR>, pcchsize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPortName)(windows_core::Interface::as_raw(self), pportname.unwrap_or(core::mem::zeroed()) as _, pcchsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "windef", feature = "wingdi"))]
    pub GetCurrentDevMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::wingdi::LPDEVMODE, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "windef", feature = "wingdi")))]
    GetCurrentDevMode: usize,
    pub GetCurrentPrinterName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetCurrentPortName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
pub trait IPrintDialogServices_Impl: windows_core::IUnknownImpl {
    fn GetCurrentDevMode(&self, pdevmode: super::wingdi::LPDEVMODE, pcbsize: *mut u32) -> windows_core::Result<()>;
    fn GetCurrentPrinterName(&self, pprintername: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::Result<()>;
    fn GetCurrentPortName(&self, pportname: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl IPrintDialogServices_Vtbl {
    pub const fn new<Identity: IPrintDialogServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentDevMode<Identity: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: super::wingdi::LPDEVMODE, pcbsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogServices_Impl::GetCurrentDevMode(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&pcbsize)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPrinterName<Identity: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintername: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogServices_Impl::GetCurrentPrinterName(this, core::mem::transmute_copy(&pprintername), core::mem::transmute_copy(&pcchsize)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPortName<Identity: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportname: windows_core::PWSTR, pcchsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogServices_Impl::GetCurrentPortName(this, core::mem::transmute_copy(&pportname), core::mem::transmute_copy(&pcchsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentDevMode: GetCurrentDevMode::<Identity, OFFSET>,
            GetCurrentPrinterName: GetCurrentPrinterName::<Identity, OFFSET>,
            GetCurrentPortName: GetCurrentPortName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPrintDialogServices as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IPrintDialogServices {}
pub const ITALIC_FONTTYPE: u32 = 512;
pub const LBSELCHSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_LBSelChangedNotify");
pub const LBSELCHSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_LBSelChangedNotify");
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCCHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPCFHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPCHOOSECOLOR(pub LPCHOOSECOLORA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCHOOSECOLORA(pub *mut CHOOSECOLORA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPCHOOSECOLORA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPCHOOSECOLORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCHOOSECOLORW(pub *mut CHOOSECOLORW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPCHOOSECOLORW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPCHOOSECOLORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPCHOOSEFONT(pub LPCHOOSEFONTA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCHOOSEFONTA(pub *mut CHOOSEFONTA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
impl LPCHOOSEFONTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
impl Default for LPCHOOSEFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCHOOSEFONTW(pub *mut CHOOSEFONTW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
impl LPCHOOSEFONTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
impl Default for LPCHOOSEFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPDEVNAMES(pub *mut DEVNAMES);
impl LPDEVNAMES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPDEVNAMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPFINDREPLACE(pub LPFINDREPLACEA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPFINDREPLACEA(pub *mut FINDREPLACEA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPFINDREPLACEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPFINDREPLACEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPFINDREPLACEW(pub *mut FINDREPLACEW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPFINDREPLACEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPFINDREPLACEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPFRHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPOFNHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPOFNOTIFY(pub LPOFNOTIFYA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOFNOTIFYA(pub *mut OFNOTIFYA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPOFNOTIFYA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPOFNOTIFYA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPOFNOTIFYEX(pub LPOFNOTIFYEXA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOFNOTIFYEXA(pub *mut OFNOTIFYEXA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPOFNOTIFYEXA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPOFNOTIFYEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOFNOTIFYEXW(pub *mut OFNOTIFYEXW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPOFNOTIFYEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPOFNOTIFYEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOFNOTIFYW(pub *mut OFNOTIFYW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl LPOFNOTIFYW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for LPOFNOTIFYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPOPENFILENAME(pub LPOPENFILENAMEA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOPENFILENAMEA(pub *mut OPENFILENAMEA);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPOPENFILENAMEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPOPENFILENAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOPENFILENAMEW(pub *mut OPENFILENAMEW);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPOPENFILENAMEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPOPENFILENAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPOPENFILENAME_NT4(pub LPOPENFILENAME_NT4A);
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOPENFILENAME_NT4A(pub *mut OPENFILENAME_NT4A);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPOPENFILENAME_NT4A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPOPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOPENFILENAME_NT4W(pub *mut OPENFILENAME_NT4W);
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl LPOPENFILENAME_NT4W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for LPOPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPPAGEPAINTHOOK = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPPAGESETUPDLG(pub LPPAGESETUPDLGA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPAGESETUPDLGA(pub *mut PAGESETUPDLGA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl LPPAGESETUPDLGA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for LPPAGESETUPDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPAGESETUPDLGW(pub *mut PAGESETUPDLGW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl LPPAGESETUPDLGW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for LPPAGESETUPDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPPAGESETUPHOOK = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPPRINTDLG(pub LPPRINTDLGA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPRINTDLGA(pub *mut PRINTDLGA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl LPPRINTDLGA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for LPPRINTDLGA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPPRINTDLGEX(pub LPPRINTDLGEXA);
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPRINTDLGEXA(pub *mut PRINTDLGEXA);
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
impl LPPRINTDLGEXA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
impl Default for LPPRINTDLGEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPRINTDLGEXW(pub *mut PRINTDLGEXW);
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
impl LPPRINTDLGEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
impl Default for LPPRINTDLGEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPRINTDLGW(pub *mut PRINTDLGW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl LPPRINTDLGW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for LPPRINTDLGW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPPRINTHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPPRINTPAGERANGE(pub *mut PRINTPAGERANGE);
impl LPPRINTPAGERANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPPRINTPAGERANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPSETUPHOOKPROC = Option<unsafe extern "system" fn(param0: super::windef::HWND, param1: u32, param2: super::minwindef::WPARAM, param3: super::minwindef::LPARAM) -> usize>;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type OFNOTIFY = OFNOTIFYA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct OFNOTIFYA {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub pszFile: windows_core::PSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OFNOTIFYA {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub pszFile: windows_core::PSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
pub type OFNOTIFYEX = OFNOTIFYEXA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXA {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OFNOTIFYEXA {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct OFNOTIFYEXW {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OFNOTIFYEXW {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub psf: *mut core::ffi::c_void,
    pub pidl: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
impl Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct OFNOTIFYW {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub pszFile: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OFNOTIFYW {
    pub hdr: super::winuser::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub pszFile: windows_core::PWSTR,
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
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type OPENFILENAME = OPENFILENAMEA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug)]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug)]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type OPENFILENAME_NT4 = OPENFILENAME_NT4A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::windef::HWND,
    pub hInstance: super::minwindef::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::minwindef::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
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
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type PAGESETUPDLG = PAGESETUPDLGA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
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
    pub lpPageSetupTemplateName: windows_core::PCSTR,
    pub hPageSetupTemplate: super::minwindef::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
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
    pub lpPageSetupTemplateName: windows_core::PCSTR,
    pub hPageSetupTemplate: super::minwindef::HGLOBAL,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
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
    pub lpPageSetupTemplateName: windows_core::PCWSTR,
    pub hPageSetupTemplate: super::minwindef::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
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
    pub lpPageSetupTemplateName: windows_core::PCWSTR,
    pub hPageSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PCCHOOSEFONT(pub PCCHOOSEFONTA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCCHOOSEFONTA(pub *const CHOOSEFONTA);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
impl PCCHOOSEFONTA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
impl Default for PCCHOOSEFONTA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCCHOOSEFONTW(pub *const CHOOSEFONTW);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
impl PCCHOOSEFONTW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "wingdi"))]
impl Default for PCCHOOSEFONTW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCDEVNAMES(pub *const DEVNAMES);
impl PCDEVNAMES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCDEVNAMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCPRINTPAGERANGE(pub *const PRINTPAGERANGE);
impl PCPRINTPAGERANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCPRINTPAGERANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type PRINTDLG = PRINTDLGA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
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
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpSetupTemplateName: windows_core::PCSTR,
    pub hPrintTemplate: super::minwindef::HGLOBAL,
    pub hSetupTemplate: super::minwindef::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
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
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpSetupTemplateName: windows_core::PCSTR,
    pub hPrintTemplate: super::minwindef::HGLOBAL,
    pub hSetupTemplate: super::minwindef::HGLOBAL,
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
pub type PRINTDLGEX = PRINTDLGEXA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
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
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpCallback: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::prsht::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
impl Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
#[derive(Clone, Debug, PartialEq)]
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
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpCallback: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::prsht::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
impl Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
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
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpCallback: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::prsht::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
impl Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
#[derive(Clone, Debug, PartialEq)]
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
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpCallback: core::mem::ManuallyDrop<Option<windows_core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::prsht::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "windef", feature = "winnt"))]
impl Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
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
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpSetupTemplateName: windows_core::PCWSTR,
    pub hPrintTemplate: super::minwindef::HGLOBAL,
    pub hSetupTemplate: super::minwindef::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
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
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpSetupTemplateName: windows_core::PCWSTR,
    pub hPrintTemplate: super::minwindef::HGLOBAL,
    pub hSetupTemplate: super::minwindef::HGLOBAL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
pub const SETRGBSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_SetRGBColor");
pub const SETRGBSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_SetRGBColor");
pub const SHAREVISTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_ShareViolation");
pub const SHAREVISTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_ShareViolation");
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
