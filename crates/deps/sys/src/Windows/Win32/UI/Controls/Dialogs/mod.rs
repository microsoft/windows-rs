#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChooseColorA(param0: *mut CHOOSECOLORA) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChooseColorW(param0: *mut CHOOSECOLORW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ChooseFontA(param0: *mut CHOOSEFONTA) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ChooseFontW(param0: *mut CHOOSEFONTW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
    pub fn CommDlgExtendedError() -> COMMON_DLG_ERRORS;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTitleA(param0: super::super::super::Foundation::PSTR, buf: super::super::super::Foundation::PSTR, cchsize: u16) -> i16;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileTitleW(param0: super::super::super::Foundation::PWSTR, buf: super::super::super::Foundation::PWSTR, cchsize: u16) -> i16;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSaveFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSaveFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PageSetupDlgA(param0: *mut PAGESETUPDLGA) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PageSetupDlgW(param0: *mut PAGESETUPDLGW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgA(ppd: *mut PRINTDLGA) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgExA(ppd: *mut PRINTDLGEXA) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgExW(ppd: *mut PRINTDLGEXW) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PrintDlgW(ppd: *mut PRINTDLGW) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND;
    #[doc = "*Required features: `Win32_UI_Controls_Dialogs`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplaceTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND;
}
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_FIRST: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_GETFILEPATH: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_GETFOLDERIDLIST: u32 = 1127u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_GETFOLDERPATH: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_GETSPEC: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_HIDECONTROL: u32 = 1129u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_LAST: u32 = 1224u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_SETCONTROLTEXT: u32 = 1128u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CDM_SETDEFEXT: u32 = 1130u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CD_LBSELADD: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CD_LBSELCHANGE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CD_LBSELNOITEMS: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const CD_LBSELSUB: u32 = 1u32;
pub struct CHOOSECOLORA(i32);
pub struct CHOOSECOLORA(i32);
pub struct CHOOSECOLORW(i32);
pub struct CHOOSECOLORW(i32);
pub struct CHOOSEFONTA(i32);
pub struct CHOOSEFONTA(i32);
pub struct CHOOSEFONTW(i32);
pub struct CHOOSEFONTW(i32);
pub struct CHOOSEFONT_FLAGS(i32);
pub struct CHOOSEFONT_FONT_TYPE(i32);
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_ADD: u32 = 712u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_BLUE: u32 = 708u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_BLUEACCEL: u32 = 728u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_BOX1: u32 = 720u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_CURRENT: u32 = 709u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_CUSTOM1: u32 = 721u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_ELEMENT: u32 = 716u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_GREEN: u32 = 707u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_GREENACCEL: u32 = 727u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_HUE: u32 = 703u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_HUEACCEL: u32 = 723u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_HUESCROLL: u32 = 700u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_LUM: u32 = 705u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_LUMACCEL: u32 = 725u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_LUMSCROLL: u32 = 702u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_MIX: u32 = 719u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_PALETTE: u32 = 718u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_RAINBOW: u32 = 710u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_RED: u32 = 706u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_REDACCEL: u32 = 726u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SAMPLES: u32 = 717u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SAT: u32 = 704u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SATACCEL: u32 = 724u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SATSCROLL: u32 = 701u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SAVE: u32 = 711u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SCHEMES: u32 = 715u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SOLID: u32 = 713u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SOLID_LEFT: u32 = 730u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_SOLID_RIGHT: u32 = 731u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const COLOR_TUNE: u32 = 714u32;
pub struct COMMON_DLG_ERRORS(i32);
pub struct DEVNAMES(i32);
pub struct DEVNAMES(i32);
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const DLG_COLOR: u32 = 10u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const DN_DEFAULTPRN: u32 = 1u32;
pub struct FINDREPLACEA(i32);
pub struct FINDREPLACEA(i32);
pub struct FINDREPLACEW(i32);
pub struct FINDREPLACEW(i32);
pub struct FINDREPLACE_FLAGS(i32);
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FRM_FIRST: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FRM_LAST: u32 = 1224u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FRM_SETOPERATIONRESULT: u32 = 1124u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FRM_SETOPERATIONRESULTTEXT: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FR_NOWRAPAROUND: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FR_RAW: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FR_SHOWWRAPAROUND: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const FR_WRAPAROUND: u32 = 1048576u32;
pub struct IPrintDialogCallback(i32);
pub struct IPrintDialogServices(i32);
pub struct LPCCHOOKPROC(i32);
pub struct LPCFHOOKPROC(i32);
pub struct LPFRHOOKPROC(i32);
pub struct LPOFNHOOKPROC(i32);
pub struct LPPAGEPAINTHOOK(i32);
pub struct LPPAGESETUPHOOK(i32);
pub struct LPPRINTHOOKPROC(i32);
pub struct LPSETUPHOOKPROC(i32);
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const NUM_BASIC_COLORS: u32 = 48u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const NUM_CUSTOM_COLORS: u32 = 16u32;
pub struct OFNOTIFYA(i32);
pub struct OFNOTIFYA(i32);
pub struct OFNOTIFYEXA(i32);
pub struct OFNOTIFYEXA(i32);
pub struct OFNOTIFYEXW(i32);
pub struct OFNOTIFYEXW(i32);
pub struct OFNOTIFYW(i32);
pub struct OFNOTIFYW(i32);
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const OFN_SHAREFALLTHROUGH: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const OFN_SHARENOWARN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const OFN_SHAREWARN: u32 = 0u32;
pub struct OPENFILENAMEA(i32);
pub struct OPENFILENAMEA(i32);
pub struct OPENFILENAMEW(i32);
pub struct OPENFILENAMEW(i32);
pub struct OPENFILENAME_NT4A(i32);
pub struct OPENFILENAME_NT4A(i32);
pub struct OPENFILENAME_NT4W(i32);
pub struct OPENFILENAME_NT4W(i32);
pub struct OPEN_FILENAME_FLAGS(i32);
pub struct OPEN_FILENAME_FLAGS_EX(i32);
pub struct PAGESETUPDLGA(i32);
pub struct PAGESETUPDLGA(i32);
pub struct PAGESETUPDLGW(i32);
pub struct PAGESETUPDLGW(i32);
pub struct PAGESETUPDLG_FLAGS(i32);
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const PD_RESULT_APPLY: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const PD_RESULT_CANCEL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const PD_RESULT_PRINT: u32 = 1u32;
pub struct PRINTDLGA(i32);
pub struct PRINTDLGA(i32);
pub struct PRINTDLGEXA(i32);
pub struct PRINTDLGEXA(i32);
pub struct PRINTDLGEXW(i32);
pub struct PRINTDLGEXW(i32);
pub struct PRINTDLGEX_FLAGS(i32);
pub struct PRINTDLGW(i32);
pub struct PRINTDLGW(i32);
pub struct PRINTPAGERANGE(i32);
pub struct PRINTPAGERANGE(i32);
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const PS_OPENTYPE_FONTTYPE: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const START_PAGE_GENERAL: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const SYMBOL_FONTTYPE: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const TT_OPENTYPE_FONTTYPE: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const TYPE1_FONTTYPE: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_CHOOSEFONT_GETLOGFONT: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_CHOOSEFONT_SETFLAGS: u32 = 1126u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_CHOOSEFONT_SETLOGFONT: u32 = 1125u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_ENVSTAMPRECT: u32 = 1029u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_FULLPAGERECT: u32 = 1025u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_GREEKTEXTRECT: u32 = 1028u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_MARGINRECT: u32 = 1027u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_MINMARGINRECT: u32 = 1026u32;
#[doc = "*Required features: `Win32_UI_Controls_Dialogs`*"]
pub const WM_PSD_YAFULLPAGERECT: u32 = 1030u32;
