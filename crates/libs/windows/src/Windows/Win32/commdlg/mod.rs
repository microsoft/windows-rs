#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn ChooseColorA(param0: LPCHOOSECOLORA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseColorA(param0 : LPCHOOSECOLORA) -> windows_core::BOOL);
    unsafe { ChooseColorA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn ChooseColorW(param0: LPCHOOSECOLORW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseColorW(param0 : LPCHOOSECOLORW) -> windows_core::BOOL);
    unsafe { ChooseColorW(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
#[inline]
pub unsafe fn ChooseFontA(param0: LPCHOOSEFONTA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseFontA(param0 : LPCHOOSEFONTA) -> windows_core::BOOL);
    unsafe { ChooseFontA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[inline]
pub unsafe fn ChooseFontW(param0: LPCHOOSEFONTW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn ChooseFontW(param0 : LPCHOOSEFONTW) -> windows_core::BOOL);
    unsafe { ChooseFontW(param0 as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CommDlgExtendedError() -> super::DWORD {
    windows_core::link!("comdlg32.dll" "system" fn CommDlgExtendedError() -> super::DWORD);
    unsafe { CommDlgExtendedError() }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn FindTextA(param0: LPFINDREPLACEA) -> super::HWND {
    windows_core::link!("comdlg32.dll" "system" fn FindTextA(param0 : LPFINDREPLACEA) -> super::HWND);
    unsafe { FindTextA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn FindTextW(param0: LPFINDREPLACEW) -> super::HWND {
    windows_core::link!("comdlg32.dll" "system" fn FindTextW(param0 : LPFINDREPLACEW) -> super::HWND);
    unsafe { FindTextW(param0 as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetFileTitleA<P0>(param0: P0, buf: windows_core::PSTR, cchsize: super::WORD) -> i16
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("comdlg32.dll" "system" fn GetFileTitleA(param0 : windows_core::PCSTR, buf : windows_core::PSTR, cchsize : super::WORD) -> i16);
    unsafe { GetFileTitleA(param0.param().abi(), buf, cchsize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetFileTitleW<P0>(param0: P0, buf: windows_core::PWSTR, cchsize: super::WORD) -> i16
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("comdlg32.dll" "system" fn GetFileTitleW(param0 : windows_core::PCWSTR, buf : windows_core::PWSTR, cchsize : super::WORD) -> i16);
    unsafe { GetFileTitleW(param0.param().abi(), buf, cchsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn GetOpenFileNameA(param0: LPOPENFILENAMEA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetOpenFileNameA(param0 : LPOPENFILENAMEA) -> windows_core::BOOL);
    unsafe { GetOpenFileNameA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn GetOpenFileNameW(param0: LPOPENFILENAMEW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetOpenFileNameW(param0 : LPOPENFILENAMEW) -> windows_core::BOOL);
    unsafe { GetOpenFileNameW(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn GetSaveFileNameA(param0: LPOPENFILENAMEA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetSaveFileNameA(param0 : LPOPENFILENAMEA) -> windows_core::BOOL);
    unsafe { GetSaveFileNameA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn GetSaveFileNameW(param0: LPOPENFILENAMEW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn GetSaveFileNameW(param0 : LPOPENFILENAMEW) -> windows_core::BOOL);
    unsafe { GetSaveFileNameW(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PageSetupDlgA(param0: LPPAGESETUPDLGA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PageSetupDlgA(param0 : LPPAGESETUPDLGA) -> windows_core::BOOL);
    unsafe { PageSetupDlgA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PageSetupDlgW(param0: LPPAGESETUPDLGW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PageSetupDlgW(param0 : LPPAGESETUPDLGW) -> windows_core::BOOL);
    unsafe { PageSetupDlgW(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PrintDlgA(ppd: LPPRINTDLGA) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgA(ppd : LPPRINTDLGA) -> windows_core::BOOL);
    unsafe { PrintDlgA(ppd as _) }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
#[inline]
pub unsafe fn PrintDlgExA(ppd: LPPRINTDLGEXA) -> windows_core::HRESULT {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgExA(ppd : LPPRINTDLGEXA) -> windows_core::HRESULT);
    unsafe { PrintDlgExA(ppd) }
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
#[inline]
pub unsafe fn PrintDlgExW(ppd: LPPRINTDLGEXW) -> windows_core::HRESULT {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgExW(ppd : LPPRINTDLGEXW) -> windows_core::HRESULT);
    unsafe { PrintDlgExW(ppd) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn PrintDlgW(ppd: LPPRINTDLGW) -> windows_core::BOOL {
    windows_core::link!("comdlg32.dll" "system" fn PrintDlgW(ppd : LPPRINTDLGW) -> windows_core::BOOL);
    unsafe { PrintDlgW(ppd as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn ReplaceTextA(param0: LPFINDREPLACEA) -> super::HWND {
    windows_core::link!("comdlg32.dll" "system" fn ReplaceTextA(param0 : LPFINDREPLACEA) -> super::HWND);
    unsafe { ReplaceTextA(param0 as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[inline]
pub unsafe fn ReplaceTextW(param0: LPFINDREPLACEW) -> super::HWND {
    windows_core::link!("comdlg32.dll" "system" fn ReplaceTextW(param0 : LPFINDREPLACEW) -> super::HWND);
    unsafe { ReplaceTextW(param0 as _) }
}
pub const BOLD_FONTTYPE: i32 = 256;
pub const CC_ANYCOLOR: i32 = 256;
pub const CC_ENABLEHOOK: i32 = 16;
pub const CC_ENABLETEMPLATE: i32 = 32;
pub const CC_ENABLETEMPLATEHANDLE: i32 = 64;
pub const CC_FULLOPEN: i32 = 2;
pub const CC_PREVENTFULLOPEN: i32 = 4;
pub const CC_RGBINIT: i32 = 1;
pub const CC_SHOWHELP: i32 = 8;
pub const CC_SOLIDCOLOR: i32 = 128;
pub const CDM_FIRST: i32 = 1124;
pub const CDM_GETFILEPATH: i32 = 1125;
pub const CDM_GETFOLDERIDLIST: i32 = 1127;
pub const CDM_GETFOLDERPATH: i32 = 1126;
pub const CDM_GETSPEC: i32 = 1124;
pub const CDM_HIDECONTROL: i32 = 1129;
pub const CDM_LAST: i32 = 1224;
pub const CDM_SETCONTROLTEXT: i32 = 1128;
pub const CDM_SETDEFEXT: i32 = 1130;
pub const CDN_FILEOK: u32 = 4294966690;
pub const CDN_FOLDERCHANGE: u32 = 4294966693;
pub const CDN_HELP: u32 = 4294966691;
pub const CDN_INCLUDEITEM: u32 = 4294966688;
pub const CDN_INITDONE: u32 = 4294966695;
pub const CDN_SELCHANGE: u32 = 4294966694;
pub const CDN_SHAREVIOLATION: u32 = 4294966692;
pub const CDN_TYPECHANGE: u32 = 4294966689;
pub const CD_LBSELADD: i32 = 2;
pub const CD_LBSELCHANGE: i32 = 0;
pub const CD_LBSELNOITEMS: i32 = -1;
pub const CD_LBSELSUB: i32 = 1;
pub const CF_ANSIONLY: i32 = 1024;
pub const CF_APPLY: i32 = 512;
pub const CF_BOTH: i32 = 3;
pub const CF_EFFECTS: i32 = 256;
pub const CF_ENABLEHOOK: i32 = 8;
pub const CF_ENABLETEMPLATE: i32 = 16;
pub const CF_ENABLETEMPLATEHANDLE: i32 = 32;
pub const CF_FIXEDPITCHONLY: i32 = 16384;
pub const CF_FORCEFONTEXIST: i32 = 65536;
pub const CF_INACTIVEFONTS: i32 = 33554432;
pub const CF_INITTOLOGFONTSTRUCT: i32 = 64;
pub const CF_LIMITSIZE: i32 = 8192;
pub const CF_NOFACESEL: i32 = 524288;
pub const CF_NOOEMFONTS: i32 = 2048;
pub const CF_NOSCRIPTSEL: i32 = 8388608;
pub const CF_NOSIMULATIONS: i32 = 4096;
pub const CF_NOSIZESEL: i32 = 2097152;
pub const CF_NOSTYLESEL: i32 = 1048576;
pub const CF_NOVECTORFONTS: i32 = 2048;
pub const CF_NOVERTFONTS: i32 = 16777216;
pub const CF_PRINTERFONTS: i32 = 2;
pub const CF_SCALABLEONLY: i32 = 131072;
pub const CF_SCREENFONTS: i32 = 1;
pub const CF_SCRIPTSONLY: i32 = 1024;
pub const CF_SELECTSCRIPT: i32 = 4194304;
pub const CF_SHOWHELP: i32 = 4;
pub const CF_TTONLY: i32 = 262144;
pub const CF_USESTYLE: i32 = 128;
pub const CF_WYSIWYG: i32 = 32768;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type CHOOSECOLOR = CHOOSECOLORA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct CHOOSECOLORA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HWND,
    pub rgbResult: super::COLORREF,
    pub lpCustColors: *mut super::COLORREF,
    pub Flags: super::DWORD,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CHOOSECOLORA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HWND,
    pub rgbResult: super::COLORREF,
    pub lpCustColors: *mut super::COLORREF,
    pub Flags: super::DWORD,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct CHOOSECOLORW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HWND,
    pub rgbResult: super::COLORREF,
    pub lpCustColors: *mut super::COLORREF,
    pub Flags: super::DWORD,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CHOOSECOLORW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HWND,
    pub rgbResult: super::COLORREF,
    pub lpCustColors: *mut super::COLORREF,
    pub Flags: super::DWORD,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
pub type CHOOSEFONT = CHOOSEFONTA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Default)]
pub struct CHOOSEFONTA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDC: super::HDC,
    pub lpLogFont: super::LPLOGFONTA,
    pub iPointSize: super::INT,
    pub Flags: super::DWORD,
    pub rgbColors: super::COLORREF,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub hInstance: super::HINSTANCE,
    pub lpszStyle: windows_core::PSTR,
    pub nFontType: super::WORD,
    pub ___MISSING_ALIGNMENT__: super::WORD,
    pub nSizeMin: super::INT,
    pub nSizeMax: super::INT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CHOOSEFONTA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDC: super::HDC,
    pub lpLogFont: super::LPLOGFONTA,
    pub iPointSize: super::INT,
    pub Flags: super::DWORD,
    pub rgbColors: super::COLORREF,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub hInstance: super::HINSTANCE,
    pub lpszStyle: windows_core::PSTR,
    pub nFontType: super::WORD,
    pub ___MISSING_ALIGNMENT__: super::WORD,
    pub nSizeMin: super::INT,
    pub nSizeMax: super::INT,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CHOOSEFONTW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDC: super::HDC,
    pub lpLogFont: super::LPLOGFONTW,
    pub iPointSize: super::INT,
    pub Flags: super::DWORD,
    pub rgbColors: super::COLORREF,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub hInstance: super::HINSTANCE,
    pub lpszStyle: windows_core::PWSTR,
    pub nFontType: super::WORD,
    pub ___MISSING_ALIGNMENT__: super::WORD,
    pub nSizeMin: super::INT,
    pub nSizeMax: super::INT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct CHOOSEFONTW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDC: super::HDC,
    pub lpLogFont: super::LPLOGFONTW,
    pub iPointSize: super::INT,
    pub Flags: super::DWORD,
    pub rgbColors: super::COLORREF,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub hInstance: super::HINSTANCE,
    pub lpszStyle: windows_core::PWSTR,
    pub nFontType: super::WORD,
    pub ___MISSING_ALIGNMENT__: super::WORD,
    pub nSizeMin: super::INT,
    pub nSizeMax: super::INT,
}
pub const COLOROKSTRING: windows_core::PCSTR = windows_core::s!("commdlg_ColorOK");
pub const COLOROKSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_ColorOK");
pub const COLOROKSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_ColorOK");
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEVNAMES {
    pub wDriverOffset: super::WORD,
    pub wDeviceOffset: super::WORD,
    pub wOutputOffset: super::WORD,
    pub wDefault: super::WORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVNAMES {
    pub wDriverOffset: super::WORD,
    pub wDeviceOffset: super::WORD,
    pub wOutputOffset: super::WORD,
    pub wDefault: super::WORD,
}
pub const DN_DEFAULTPRN: i32 = 1;
pub const FILEOKSTRING: windows_core::PCSTR = windows_core::s!("commdlg_FileNameOK");
pub const FILEOKSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_FileNameOK");
pub const FILEOKSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_FileNameOK");
pub const FINDMSGSTRING: windows_core::PCSTR = windows_core::s!("commdlg_FindReplace");
pub const FINDMSGSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_FindReplace");
pub const FINDMSGSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_FindReplace");
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type FINDREPLACE = FINDREPLACEA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct FINDREPLACEA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub Flags: super::DWORD,
    pub lpstrFindWhat: windows_core::PSTR,
    pub lpstrReplaceWith: windows_core::PSTR,
    pub wFindWhatLen: super::WORD,
    pub wReplaceWithLen: super::WORD,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct FINDREPLACEA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub Flags: super::DWORD,
    pub lpstrFindWhat: windows_core::PSTR,
    pub lpstrReplaceWith: windows_core::PSTR,
    pub wFindWhatLen: super::WORD,
    pub wReplaceWithLen: super::WORD,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct FINDREPLACEW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub Flags: super::DWORD,
    pub lpstrFindWhat: windows_core::PWSTR,
    pub lpstrReplaceWith: windows_core::PWSTR,
    pub wFindWhatLen: super::WORD,
    pub wReplaceWithLen: super::WORD,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct FINDREPLACEW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub Flags: super::DWORD,
    pub lpstrFindWhat: windows_core::PWSTR,
    pub lpstrReplaceWith: windows_core::PWSTR,
    pub wFindWhatLen: super::WORD,
    pub wReplaceWithLen: super::WORD,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
pub const FRM_FIRST: i32 = 1124;
pub const FRM_LAST: i32 = 1224;
pub const FRM_SETOPERATIONRESULT: i32 = 1124;
pub const FRM_SETOPERATIONRESULTTEXT: i32 = 1125;
pub const FR_DIALOGTERM: i32 = 64;
pub const FR_DOWN: i32 = 1;
pub const FR_ENABLEHOOK: i32 = 256;
pub const FR_ENABLETEMPLATE: i32 = 512;
pub const FR_ENABLETEMPLATEHANDLE: i32 = 8192;
pub const FR_FINDNEXT: i32 = 8;
pub const FR_HIDEMATCHCASE: i32 = 32768;
pub const FR_HIDEUPDOWN: i32 = 16384;
pub const FR_HIDEWHOLEWORD: i32 = 65536;
pub const FR_MATCHCASE: i32 = 4;
pub const FR_NOMATCHCASE: i32 = 2048;
pub const FR_NOUPDOWN: i32 = 1024;
pub const FR_NOWHOLEWORD: i32 = 4096;
pub const FR_NOWRAPAROUND: i32 = 524288;
pub const FR_RAW: i32 = 131072;
pub const FR_REPLACE: i32 = 16;
pub const FR_REPLACEALL: i32 = 32;
pub const FR_SHOWHELP: i32 = 128;
pub const FR_SHOWWRAPAROUND: i32 = 262144;
pub const FR_WHOLEWORD: i32 = 2;
pub const FR_WRAPAROUND: i32 = 1048576;
pub const HELPMSGSTRING: windows_core::PCSTR = windows_core::s!("commdlg_help");
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
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub unsafe fn HandleMessage(&self, hdlg: super::HWND, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM, presult: *mut super::LRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HandleMessage)(windows_core::Interface::as_raw(self), hdlg as _, umsg, wparam, lparam, presult as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitDone: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectionChange: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
    pub HandleMessage: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, super::UINT, super::WPARAM, super::LPARAM, *mut super::LRESULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "shared", feature = "windef")))]
    HandleMessage: usize,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub trait IPrintDialogCallback_Impl: windows_core::IUnknownImpl {
    fn InitDone(&self) -> windows_core::Result<()>;
    fn SelectionChange(&self) -> windows_core::Result<()>;
    fn HandleMessage(&self, hdlg: super::HWND, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM, presult: *mut super::LRESULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
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
        unsafe extern "system" fn HandleMessage<Identity: IPrintDialogCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hdlg: super::HWND, umsg: super::UINT, wparam: super::WPARAM, lparam: super::LPARAM, presult: *mut super::LRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogCallback_Impl::HandleMessage(this, core::mem::transmute_copy(&hdlg), core::mem::transmute_copy(&umsg), core::mem::transmute_copy(&wparam), core::mem::transmute_copy(&lparam), core::mem::transmute_copy(&presult)).into()
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IPrintDialogCallback {}
windows_core::imp::define_interface!(IPrintDialogServices, IPrintDialogServices_Vtbl, 0x509aaeda_5639_11d1_b6a1_0000f8757bf9);
windows_core::imp::interface_hierarchy!(IPrintDialogServices, windows_core::IUnknown);
impl IPrintDialogServices {
    #[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
    pub unsafe fn GetCurrentDevMode(&self, pdevmode: super::LPDEVMODE, pcbsize: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentDevMode)(windows_core::Interface::as_raw(self), pdevmode as _, pcbsize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCurrentPrinterName(&self, pprintername: Option<windows_core::PWSTR>, pcchsize: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPrinterName)(windows_core::Interface::as_raw(self), pprintername.unwrap_or(core::mem::zeroed()) as _, pcchsize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetCurrentPortName(&self, pportname: Option<windows_core::PWSTR>, pcchsize: *mut super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPortName)(windows_core::Interface::as_raw(self), pportname.unwrap_or(core::mem::zeroed()) as _, pcchsize as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
    pub GetCurrentDevMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::LPDEVMODE, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef", feature = "wingdi")))]
    GetCurrentDevMode: usize,
    #[cfg(feature = "shared")]
    pub GetCurrentPrinterName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCurrentPrinterName: usize,
    #[cfg(feature = "shared")]
    pub GetCurrentPortName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetCurrentPortName: usize,
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
pub trait IPrintDialogServices_Impl: windows_core::IUnknownImpl {
    fn GetCurrentDevMode(&self, pdevmode: super::LPDEVMODE, pcbsize: *mut super::UINT) -> windows_core::Result<()>;
    fn GetCurrentPrinterName(&self, pprintername: windows_core::PWSTR, pcchsize: *mut super::UINT) -> windows_core::Result<()>;
    fn GetCurrentPortName(&self, pportname: windows_core::PWSTR, pcchsize: *mut super::UINT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
impl IPrintDialogServices_Vtbl {
    pub const fn new<Identity: IPrintDialogServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentDevMode<Identity: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdevmode: super::LPDEVMODE, pcbsize: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogServices_Impl::GetCurrentDevMode(this, core::mem::transmute_copy(&pdevmode), core::mem::transmute_copy(&pcbsize)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPrinterName<Identity: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprintername: windows_core::PWSTR, pcchsize: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPrintDialogServices_Impl::GetCurrentPrinterName(this, core::mem::transmute_copy(&pprintername), core::mem::transmute_copy(&pcchsize)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPortName<Identity: IPrintDialogServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pportname: windows_core::PWSTR, pcchsize: *mut super::UINT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "windef", feature = "wingdi"))]
impl windows_core::RuntimeName for IPrintDialogServices {}
pub const ITALIC_FONTTYPE: i32 = 512;
pub const LBSELCHSTRING: windows_core::PCSTR = windows_core::s!("commdlg_LBSelChangedNotify");
pub const LBSELCHSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_LBSelChangedNotify");
pub const LBSELCHSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_LBSelChangedNotify");
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPCCHOOKPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::UINT, param2: super::WPARAM, param3: super::LPARAM) -> super::UINT_PTR>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPCFHOOKPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::UINT, param2: super::WPARAM, param3: super::LPARAM) -> super::UINT_PTR>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPCHOOSECOLOR = *mut CHOOSECOLORA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPCHOOSECOLORA = *mut CHOOSECOLORA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPCHOOSECOLORW = *mut CHOOSECOLORW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
pub type LPCHOOSEFONT = *mut CHOOSEFONTA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
pub type LPCHOOSEFONTA = *mut CHOOSEFONTA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type LPCHOOSEFONTW = *mut CHOOSEFONTW;
#[cfg(feature = "shared")]
pub type LPDEVNAMES = *mut DEVNAMES;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPFINDREPLACE = *mut FINDREPLACEA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPFINDREPLACEA = *mut FINDREPLACEA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPFINDREPLACEW = *mut FINDREPLACEW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPFRHOOKPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::UINT, param2: super::WPARAM, param3: super::LPARAM) -> super::UINT_PTR>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPOFNHOOKPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::UINT, param2: super::WPARAM, param3: super::LPARAM) -> super::UINT_PTR>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type LPOFNOTIFY = *mut OFNOTIFYA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type LPOFNOTIFYA = *mut OFNOTIFYA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type LPOFNOTIFYEX = *mut OFNOTIFYEXA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type LPOFNOTIFYEXA = *mut OFNOTIFYEXA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type LPOFNOTIFYEXW = *mut OFNOTIFYEXW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type LPOFNOTIFYW = *mut OFNOTIFYW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPOPENFILENAME = *mut OPENFILENAMEA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPOPENFILENAMEA = *mut OPENFILENAMEA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPOPENFILENAMEW = *mut OPENFILENAMEW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPOPENFILENAME_NT4 = *mut OPENFILENAME_NT4A;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPOPENFILENAME_NT4A = *mut OPENFILENAME_NT4A;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPOPENFILENAME_NT4W = *mut OPENFILENAME_NT4W;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPPAGEPAINTHOOK = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::UINT, param2: super::WPARAM, param3: super::LPARAM) -> super::UINT_PTR>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPPAGESETUPDLG = *mut PAGESETUPDLGA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPPAGESETUPDLGA = *mut PAGESETUPDLGA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPPAGESETUPDLGW = *mut PAGESETUPDLGW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPPAGESETUPHOOK = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::UINT, param2: super::WPARAM, param3: super::LPARAM) -> super::UINT_PTR>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPPRINTDLG = *mut PRINTDLGA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPPRINTDLGA = *mut PRINTDLGA;
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
pub type LPPRINTDLGEX = *mut PRINTDLGEXA;
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
pub type LPPRINTDLGEXA = *mut PRINTDLGEXA;
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
pub type LPPRINTDLGEXW = *mut PRINTDLGEXW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPPRINTDLGW = *mut PRINTDLGW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPPRINTHOOKPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::UINT, param2: super::WPARAM, param3: super::LPARAM) -> super::UINT_PTR>;
#[cfg(feature = "shared")]
pub type LPPRINTPAGERANGE = *mut PRINTPAGERANGE;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type LPSETUPHOOKPROC = Option<unsafe extern "system" fn(param0: super::HWND, param1: super::UINT, param2: super::WPARAM, param3: super::LPARAM) -> super::UINT_PTR>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type OFNOTIFY = OFNOTIFYA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub pszFile: windows_core::PSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub pszFile: windows_core::PSTR,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
pub type OFNOTIFYEX = OFNOTIFYEXA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub psf: super::LPVOID,
    pub pidl: super::LPVOID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: LPOPENFILENAMEA,
    pub psf: super::LPVOID,
    pub pidl: super::LPVOID,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub psf: super::LPVOID,
    pub pidl: super::LPVOID,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub psf: super::LPVOID,
    pub pidl: super::LPVOID,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Default)]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub pszFile: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "winuser"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: LPOPENFILENAMEW,
    pub pszFile: windows_core::PWSTR,
}
pub const OFN_ALLOWMULTISELECT: i32 = 512;
pub const OFN_CREATEPROMPT: i32 = 8192;
pub const OFN_DONTADDTORECENT: i32 = 33554432;
pub const OFN_ENABLEHOOK: i32 = 32;
pub const OFN_ENABLEINCLUDENOTIFY: i32 = 4194304;
pub const OFN_ENABLESIZING: i32 = 8388608;
pub const OFN_ENABLETEMPLATE: i32 = 64;
pub const OFN_ENABLETEMPLATEHANDLE: i32 = 128;
pub const OFN_EXPLORER: i32 = 524288;
pub const OFN_EXTENSIONDIFFERENT: i32 = 1024;
pub const OFN_EX_NOPLACESBAR: i32 = 1;
pub const OFN_FILEMUSTEXIST: i32 = 4096;
pub const OFN_FORCESHOWHIDDEN: i32 = 268435456;
pub const OFN_HIDEREADONLY: i32 = 4;
pub const OFN_LONGNAMES: i32 = 2097152;
pub const OFN_NOCHANGEDIR: i32 = 8;
pub const OFN_NODEREFERENCELINKS: i32 = 1048576;
pub const OFN_NOLONGNAMES: i32 = 262144;
pub const OFN_NONETWORKBUTTON: i32 = 131072;
pub const OFN_NOREADONLYRETURN: i32 = 32768;
pub const OFN_NOTESTFILECREATE: i32 = 65536;
pub const OFN_NOVALIDATE: i32 = 256;
pub const OFN_OVERWRITEPROMPT: i32 = 2;
pub const OFN_PATHMUSTEXIST: i32 = 2048;
pub const OFN_READONLY: i32 = 1;
pub const OFN_SHAREAWARE: i32 = 16384;
pub const OFN_SHAREFALLTHROUGH: i32 = 2;
pub const OFN_SHARENOWARN: i32 = 1;
pub const OFN_SHAREWARN: i32 = 0;
pub const OFN_SHOWHELP: i32 = 16;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type OPENFILENAME = OPENFILENAMEA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct OPENFILENAMEA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: super::DWORD,
    pub nFilterIndex: super::DWORD,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: super::DWORD,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: super::DWORD,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: super::DWORD,
    pub nFileOffset: super::WORD,
    pub nFileExtension: super::WORD,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: super::DWORD,
    pub FlagsEx: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENFILENAMEA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: super::DWORD,
    pub nFilterIndex: super::DWORD,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: super::DWORD,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: super::DWORD,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: super::DWORD,
    pub nFileOffset: super::WORD,
    pub nFileExtension: super::WORD,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: super::DWORD,
    pub FlagsEx: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct OPENFILENAMEW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: super::DWORD,
    pub nFilterIndex: super::DWORD,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: super::DWORD,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: super::DWORD,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: super::DWORD,
    pub nFileOffset: super::WORD,
    pub nFileExtension: super::WORD,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: super::DWORD,
    pub FlagsEx: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENFILENAMEW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: super::DWORD,
    pub nFilterIndex: super::DWORD,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: super::DWORD,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: super::DWORD,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: super::DWORD,
    pub nFileOffset: super::WORD,
    pub nFileExtension: super::WORD,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
    pub pvReserved: *mut core::ffi::c_void,
    pub dwReserved: super::DWORD,
    pub FlagsEx: super::DWORD,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type OPENFILENAME_NT4 = OPENFILENAME_NT4A;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: super::DWORD,
    pub nFilterIndex: super::DWORD,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: super::DWORD,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: super::DWORD,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: super::DWORD,
    pub nFileOffset: super::WORD,
    pub nFileExtension: super::WORD,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpstrFilter: windows_core::PCSTR,
    pub lpstrCustomFilter: windows_core::PSTR,
    pub nMaxCustFilter: super::DWORD,
    pub nFilterIndex: super::DWORD,
    pub lpstrFile: windows_core::PSTR,
    pub nMaxFile: super::DWORD,
    pub lpstrFileTitle: windows_core::PSTR,
    pub nMaxFileTitle: super::DWORD,
    pub lpstrInitialDir: windows_core::PCSTR,
    pub lpstrTitle: windows_core::PCSTR,
    pub Flags: super::DWORD,
    pub nFileOffset: super::WORD,
    pub nFileExtension: super::WORD,
    pub lpstrDefExt: windows_core::PCSTR,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCSTR,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: super::DWORD,
    pub nFilterIndex: super::DWORD,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: super::DWORD,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: super::DWORD,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: super::DWORD,
    pub nFileOffset: super::WORD,
    pub nFileExtension: super::WORD,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hInstance: super::HINSTANCE,
    pub lpstrFilter: windows_core::PCWSTR,
    pub lpstrCustomFilter: windows_core::PWSTR,
    pub nMaxCustFilter: super::DWORD,
    pub nFilterIndex: super::DWORD,
    pub lpstrFile: windows_core::PWSTR,
    pub nMaxFile: super::DWORD,
    pub lpstrFileTitle: windows_core::PWSTR,
    pub nMaxFileTitle: super::DWORD,
    pub lpstrInitialDir: windows_core::PCWSTR,
    pub lpstrTitle: windows_core::PCWSTR,
    pub Flags: super::DWORD,
    pub nFileOffset: super::WORD,
    pub nFileExtension: super::WORD,
    pub lpstrDefExt: windows_core::PCWSTR,
    pub lCustData: super::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: windows_core::PCWSTR,
}
#[cfg(target_arch = "x86")]
pub const OPENFILENAME_SIZE_VERSION_400: u32 = 76;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const OPENFILENAME_SIZE_VERSION_400: u64 = 136;
#[cfg(target_arch = "x86")]
pub const OPENFILENAME_SIZE_VERSION_400A: u32 = 76;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const OPENFILENAME_SIZE_VERSION_400A: u64 = 136;
#[cfg(target_arch = "x86")]
pub const OPENFILENAME_SIZE_VERSION_400W: u32 = 76;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const OPENFILENAME_SIZE_VERSION_400W: u64 = 136;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PAGESETUPDLG = PAGESETUPDLGA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PAGESETUPDLGA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub Flags: super::DWORD,
    pub ptPaperSize: super::POINT,
    pub rtMinMargin: super::RECT,
    pub rtMargin: super::RECT,
    pub hInstance: super::HINSTANCE,
    pub lCustData: super::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_core::PCSTR,
    pub hPageSetupTemplate: super::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PAGESETUPDLGA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub Flags: super::DWORD,
    pub ptPaperSize: super::POINT,
    pub rtMinMargin: super::RECT,
    pub rtMargin: super::RECT,
    pub hInstance: super::HINSTANCE,
    pub lCustData: super::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_core::PCSTR,
    pub hPageSetupTemplate: super::HGLOBAL,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PAGESETUPDLGW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub Flags: super::DWORD,
    pub ptPaperSize: super::POINT,
    pub rtMinMargin: super::RECT,
    pub rtMargin: super::RECT,
    pub hInstance: super::HINSTANCE,
    pub lCustData: super::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_core::PCWSTR,
    pub hPageSetupTemplate: super::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PAGESETUPDLGW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub Flags: super::DWORD,
    pub ptPaperSize: super::POINT,
    pub rtMinMargin: super::RECT,
    pub rtMargin: super::RECT,
    pub hInstance: super::HINSTANCE,
    pub lCustData: super::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: windows_core::PCWSTR,
    pub hPageSetupTemplate: super::HGLOBAL,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PCCHOOSEFONT = *const CHOOSEFONTA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi"))]
pub type PCCHOOSEFONTA = *const CHOOSEFONTA;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PCCHOOSEFONTW = *const CHOOSEFONTW;
#[cfg(feature = "shared")]
pub type PCDEVNAMES = *const DEVNAMES;
#[cfg(feature = "shared")]
pub type PCPRINTPAGERANGE = *const PRINTPAGERANGE;
pub const PD_ALLPAGES: i32 = 0;
pub const PD_COLLATE: i32 = 16;
pub const PD_CURRENTPAGE: i32 = 4194304;
pub const PD_DISABLEPRINTTOFILE: i32 = 524288;
pub const PD_ENABLEPRINTHOOK: i32 = 4096;
pub const PD_ENABLEPRINTTEMPLATE: i32 = 16384;
pub const PD_ENABLEPRINTTEMPLATEHANDLE: i32 = 65536;
pub const PD_ENABLESETUPHOOK: i32 = 8192;
pub const PD_ENABLESETUPTEMPLATE: i32 = 32768;
pub const PD_ENABLESETUPTEMPLATEHANDLE: i32 = 131072;
pub const PD_EXCLUSIONFLAGS: i32 = 16777216;
pub const PD_EXCL_COPIESANDCOLLATE: i32 = 33024;
pub const PD_HIDEPRINTTOFILE: i32 = 1048576;
pub const PD_NOCURRENTPAGE: i32 = 8388608;
pub const PD_NONETWORKBUTTON: i32 = 2097152;
pub const PD_NOPAGENUMS: i32 = 8;
pub const PD_NOSELECTION: i32 = 4;
pub const PD_NOWARNING: i32 = 128;
pub const PD_PAGENUMS: i32 = 2;
pub const PD_PRINTSETUP: i32 = 64;
pub const PD_PRINTTOFILE: i32 = 32;
pub const PD_RESULT_APPLY: i32 = 2;
pub const PD_RESULT_CANCEL: i32 = 0;
pub const PD_RESULT_PRINT: i32 = 1;
pub const PD_RETURNDC: i32 = 256;
pub const PD_RETURNDEFAULT: i32 = 1024;
pub const PD_RETURNIC: i32 = 512;
pub const PD_SELECTION: i32 = 1;
pub const PD_SHOWHELP: i32 = 2048;
pub const PD_USEDEVMODECOPIES: i32 = 262144;
pub const PD_USEDEVMODECOPIESANDCOLLATE: i32 = 262144;
pub const PD_USELARGETEMPLATE: i32 = 268435456;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
pub type PRINTDLG = PRINTDLGA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTDLGA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub hDC: super::HDC,
    pub Flags: super::DWORD,
    pub nFromPage: super::WORD,
    pub nToPage: super::WORD,
    pub nMinPage: super::WORD,
    pub nMaxPage: super::WORD,
    pub nCopies: super::WORD,
    pub hInstance: super::HINSTANCE,
    pub lCustData: super::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpSetupTemplateName: windows_core::PCSTR,
    pub hPrintTemplate: super::HGLOBAL,
    pub hSetupTemplate: super::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PRINTDLGA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub hDC: super::HDC,
    pub Flags: super::DWORD,
    pub nFromPage: super::WORD,
    pub nToPage: super::WORD,
    pub nMinPage: super::WORD,
    pub nMaxPage: super::WORD,
    pub nCopies: super::WORD,
    pub hInstance: super::HINSTANCE,
    pub lCustData: super::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpSetupTemplateName: windows_core::PCSTR,
    pub hPrintTemplate: super::HGLOBAL,
    pub hSetupTemplate: super::HGLOBAL,
}
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
pub type PRINTDLGEX = PRINTDLGEXA;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
#[derive(Default)]
pub struct PRINTDLGEXA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub hDC: super::HDC,
    pub Flags: super::DWORD,
    pub Flags2: super::DWORD,
    pub ExclusionFlags: super::DWORD,
    pub nPageRanges: super::DWORD,
    pub nMaxPageRanges: super::DWORD,
    pub lpPageRanges: LPPRINTPAGERANGE,
    pub nMinPage: super::DWORD,
    pub nMaxPage: super::DWORD,
    pub nCopies: super::DWORD,
    pub hInstance: super::HINSTANCE,
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpCallback: Option<super::LPUNKNOWN>,
    pub nPropertyPages: super::DWORD,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: super::DWORD,
    pub dwResultAction: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PRINTDLGEXA {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub hDC: super::HDC,
    pub Flags: super::DWORD,
    pub Flags2: super::DWORD,
    pub ExclusionFlags: super::DWORD,
    pub nPageRanges: super::DWORD,
    pub nMaxPageRanges: super::DWORD,
    pub lpPageRanges: LPPRINTPAGERANGE,
    pub nMinPage: super::DWORD,
    pub nMaxPage: super::DWORD,
    pub nCopies: super::DWORD,
    pub hInstance: super::HINSTANCE,
    pub lpPrintTemplateName: windows_core::PCSTR,
    pub lpCallback: Option<super::LPUNKNOWN>,
    pub nPropertyPages: super::DWORD,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: super::DWORD,
    pub dwResultAction: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
#[derive(Default)]
pub struct PRINTDLGEXW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub hDC: super::HDC,
    pub Flags: super::DWORD,
    pub Flags2: super::DWORD,
    pub ExclusionFlags: super::DWORD,
    pub nPageRanges: super::DWORD,
    pub nMaxPageRanges: super::DWORD,
    pub lpPageRanges: LPPRINTPAGERANGE,
    pub nMinPage: super::DWORD,
    pub nMaxPage: super::DWORD,
    pub nCopies: super::DWORD,
    pub hInstance: super::HINSTANCE,
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpCallback: Option<super::LPUNKNOWN>,
    pub nPropertyPages: super::DWORD,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: super::DWORD,
    pub dwResultAction: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "prsht", feature = "shared", feature = "unknwnbase", feature = "windef"))]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PRINTDLGEXW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub hDC: super::HDC,
    pub Flags: super::DWORD,
    pub Flags2: super::DWORD,
    pub ExclusionFlags: super::DWORD,
    pub nPageRanges: super::DWORD,
    pub nMaxPageRanges: super::DWORD,
    pub lpPageRanges: LPPRINTPAGERANGE,
    pub nMinPage: super::DWORD,
    pub nMaxPage: super::DWORD,
    pub nCopies: super::DWORD,
    pub hInstance: super::HINSTANCE,
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpCallback: Option<super::LPUNKNOWN>,
    pub nPropertyPages: super::DWORD,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: super::DWORD,
    pub dwResultAction: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PRINTDLGW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub hDC: super::HDC,
    pub Flags: super::DWORD,
    pub nFromPage: super::WORD,
    pub nToPage: super::WORD,
    pub nMinPage: super::WORD,
    pub nMaxPage: super::WORD,
    pub nCopies: super::WORD,
    pub hInstance: super::HINSTANCE,
    pub lCustData: super::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpSetupTemplateName: windows_core::PCWSTR,
    pub hPrintTemplate: super::HGLOBAL,
    pub hSetupTemplate: super::HGLOBAL,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PRINTDLGW {
    pub lStructSize: super::DWORD,
    pub hwndOwner: super::HWND,
    pub hDevMode: super::HGLOBAL,
    pub hDevNames: super::HGLOBAL,
    pub hDC: super::HDC,
    pub Flags: super::DWORD,
    pub nFromPage: super::WORD,
    pub nToPage: super::WORD,
    pub nMinPage: super::WORD,
    pub nMaxPage: super::WORD,
    pub nCopies: super::WORD,
    pub hInstance: super::HINSTANCE,
    pub lCustData: super::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: windows_core::PCWSTR,
    pub lpSetupTemplateName: windows_core::PCWSTR,
    pub hPrintTemplate: super::HGLOBAL,
    pub hSetupTemplate: super::HGLOBAL,
}
pub const PRINTER_FONTTYPE: i32 = 16384;
#[repr(C, packed(1))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PRINTPAGERANGE {
    pub nFromPage: super::DWORD,
    pub nToPage: super::DWORD,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PRINTPAGERANGE {
    pub nFromPage: super::DWORD,
    pub nToPage: super::DWORD,
}
pub const PSD_DEFAULTMINMARGINS: i32 = 0;
pub const PSD_DISABLEMARGINS: i32 = 16;
pub const PSD_DISABLEORIENTATION: i32 = 256;
pub const PSD_DISABLEPAGEPAINTING: i32 = 524288;
pub const PSD_DISABLEPAPER: i32 = 512;
pub const PSD_DISABLEPRINTER: i32 = 32;
pub const PSD_ENABLEPAGEPAINTHOOK: i32 = 262144;
pub const PSD_ENABLEPAGESETUPHOOK: i32 = 8192;
pub const PSD_ENABLEPAGESETUPTEMPLATE: i32 = 32768;
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: i32 = 131072;
pub const PSD_INHUNDREDTHSOFMILLIMETERS: i32 = 8;
pub const PSD_INTHOUSANDTHSOFINCHES: i32 = 4;
pub const PSD_INWININIINTLMEASURE: i32 = 0;
pub const PSD_MARGINS: i32 = 2;
pub const PSD_MINMARGINS: i32 = 1;
pub const PSD_NONETWORKBUTTON: i32 = 2097152;
pub const PSD_NOWARNING: i32 = 128;
pub const PSD_RETURNDEFAULT: i32 = 1024;
pub const PSD_SHOWHELP: i32 = 2048;
pub const PS_OPENTYPE_FONTTYPE: i32 = 65536;
pub const REGULAR_FONTTYPE: i32 = 1024;
pub const SCREEN_FONTTYPE: i32 = 8192;
pub const SETRGBSTRING: windows_core::PCSTR = windows_core::s!("commdlg_SetRGBColor");
pub const SETRGBSTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_SetRGBColor");
pub const SETRGBSTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_SetRGBColor");
pub const SHAREVISTRING: windows_core::PCSTR = windows_core::s!("commdlg_ShareViolation");
pub const SHAREVISTRINGA: windows_core::PCSTR = windows_core::s!("commdlg_ShareViolation");
pub const SHAREVISTRINGW: windows_core::PCWSTR = windows_core::w!("commdlg_ShareViolation");
pub const SIMULATED_FONTTYPE: i32 = 32768;
pub const START_PAGE_GENERAL: u32 = 4294967295;
pub const SYMBOL_FONTTYPE: i32 = 524288;
pub const TT_OPENTYPE_FONTTYPE: i32 = 131072;
pub const TYPE1_FONTTYPE: i32 = 262144;
pub const WM_CHOOSEFONT_GETLOGFONT: i32 = 1025;
pub const WM_CHOOSEFONT_SETFLAGS: i32 = 1126;
pub const WM_CHOOSEFONT_SETLOGFONT: i32 = 1125;
pub const WM_PSD_ENVSTAMPRECT: i32 = 1029;
pub const WM_PSD_FULLPAGERECT: i32 = 1025;
pub const WM_PSD_GREEKTEXTRECT: i32 = 1028;
pub const WM_PSD_MARGINRECT: i32 = 1027;
pub const WM_PSD_MINMARGINRECT: i32 = 1026;
pub const WM_PSD_PAGESETUPDLG: i32 = 1024;
pub const WM_PSD_YAFULLPAGERECT: i32 = 1030;
