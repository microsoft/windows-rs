#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn CommonPropertySheetUIA(hwndowner: super::windef::HWND, pfnpropsheetui: PFNPROPSHEETUI, lparam: super::minwindef::LPARAM, presult: Option<*mut u32>) -> i32 {
    windows_core::link!("compstui.dll" "system" fn CommonPropertySheetUIA(hwndowner : super::windef::HWND, pfnpropsheetui : PFNPROPSHEETUI, lparam : super::minwindef::LPARAM, presult : *mut u32) -> i32);
    unsafe { CommonPropertySheetUIA(hwndowner, pfnpropsheetui, lparam, presult.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn CommonPropertySheetUIW(hwndowner: super::windef::HWND, pfnpropsheetui: PFNPROPSHEETUI, lparam: super::minwindef::LPARAM, presult: Option<*mut u32>) -> i32 {
    windows_core::link!("compstui.dll" "system" fn CommonPropertySheetUIW(hwndowner : super::windef::HWND, pfnpropsheetui : PFNPROPSHEETUI, lparam : super::minwindef::LPARAM, presult : *mut u32) -> i32);
    unsafe { CommonPropertySheetUIW(hwndowner, pfnpropsheetui, lparam, presult.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn GetCPSUIUserData(hdlg: super::windef::HWND) -> usize {
    windows_core::link!("compstui.dll" "system" fn GetCPSUIUserData(hdlg : super::windef::HWND) -> usize);
    unsafe { GetCPSUIUserData(hdlg) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn SetCPSUIUserData(hdlg: super::windef::HWND, cpsuiuserdata: usize) -> windows_core::BOOL {
    windows_core::link!("compstui.dll" "system" fn SetCPSUIUserData(hdlg : super::windef::HWND, cpsuiuserdata : usize) -> windows_core::BOOL);
    unsafe { SetCPSUIUserData(hdlg, cpsuiuserdata) }
}
pub const APPLYCPSUI_NO_NEWDEF: u32 = 1;
pub const APPLYCPSUI_OK_CANCEL_BUTTON: u32 = 2;
pub const CHKBOXS_FALSE_PDATA: u32 = 3;
pub const CHKBOXS_FALSE_TRUE: u32 = 0;
pub const CHKBOXS_NONE_PDATA: u32 = 6;
pub const CHKBOXS_NO_PDATA: u32 = 4;
pub const CHKBOXS_NO_YES: u32 = 1;
pub const CHKBOXS_OFF_ON: u32 = 2;
pub const CHKBOXS_OFF_PDATA: u32 = 5;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy, Debug)]
pub struct COMPROPSHEETUI {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::minwindef::HINSTANCE,
    pub pCallerName: super::winnt::LPTSTR,
    pub UserData: usize,
    pub pHelpFile: super::winnt::LPTSTR,
    pub pfnCallBack: _CPSUICALLBACK,
    pub pOptItem: POPTITEM,
    pub pDlgPage: PDLGPAGE,
    pub cOptItem: u16,
    pub cDlgPage: u16,
    pub IconID: usize,
    pub pOptItemName: super::winnt::LPTSTR,
    pub CallerVersion: u16,
    pub OptItemVersion: u16,
    pub dwReserved: [usize; 4],
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for COMPROPSHEETUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPSFUNC_ADD_HPROPSHEETPAGE: u32 = 0;
pub const CPSFUNC_ADD_PCOMPROPSHEETUI: u32 = 2;
pub const CPSFUNC_ADD_PCOMPROPSHEETUIA: u32 = 2;
pub const CPSFUNC_ADD_PCOMPROPSHEETUIW: u32 = 3;
pub const CPSFUNC_ADD_PFNPROPSHEETUI: u32 = 4;
pub const CPSFUNC_ADD_PFNPROPSHEETUIA: u32 = 4;
pub const CPSFUNC_ADD_PFNPROPSHEETUIW: u32 = 5;
pub const CPSFUNC_ADD_PROPSHEETPAGE: u32 = 15;
pub const CPSFUNC_ADD_PROPSHEETPAGEA: u32 = 15;
pub const CPSFUNC_ADD_PROPSHEETPAGEW: u32 = 1;
pub const CPSFUNC_DELETE_HCOMPROPSHEET: u32 = 6;
pub const CPSFUNC_DO_APPLY_CPSUI: u32 = 25;
pub const CPSFUNC_GET_HPSUIPAGES: u32 = 10;
pub const CPSFUNC_GET_PAGECOUNT: u32 = 8;
pub const CPSFUNC_GET_PFNPROPSHEETUI_ICON: u32 = 14;
pub const CPSFUNC_IGNORE_CPSUI_PSN_APPLY: u32 = 24;
pub const CPSFUNC_INSERT_PSUIPAGE: u32 = 16;
pub const CPSFUNC_INSERT_PSUIPAGEA: u32 = 16;
pub const CPSFUNC_INSERT_PSUIPAGEW: u32 = 17;
pub const CPSFUNC_LOAD_CPSUI_ICON: u32 = 13;
pub const CPSFUNC_LOAD_CPSUI_STRING: u32 = 11;
pub const CPSFUNC_LOAD_CPSUI_STRINGA: u32 = 11;
pub const CPSFUNC_LOAD_CPSUI_STRINGW: u32 = 12;
pub const CPSFUNC_QUERY_DATABLOCK: u32 = 22;
pub const CPSFUNC_SET_DATABLOCK: u32 = 21;
pub const CPSFUNC_SET_DMPUB_HIDEBITS: u32 = 23;
pub const CPSFUNC_SET_FUSION_CONTEXT: u32 = 26;
pub const CPSFUNC_SET_HSTARTPAGE: u32 = 7;
pub const CPSFUNC_SET_PSUIPAGE_ICON: u32 = 20;
pub const CPSFUNC_SET_PSUIPAGE_TITLE: u32 = 18;
pub const CPSFUNC_SET_PSUIPAGE_TITLEA: u32 = 18;
pub const CPSFUNC_SET_PSUIPAGE_TITLEW: u32 = 19;
pub const CPSFUNC_SET_RESULT: u32 = 9;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct CPSUICBPARAM {
    pub cbSize: u16,
    pub Reason: u16,
    pub hDlg: super::windef::HWND,
    pub pOptItem: POPTITEM,
    pub cOptItem: u16,
    pub Flags: u16,
    pub pCurItem: POPTITEM,
    pub Anonymous: CPSUICBPARAM_0,
    pub UserData: usize,
    pub Result: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for CPSUICBPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union CPSUICBPARAM_0 {
    pub OldSel: i32,
    pub pOldSel: super::winnt::LPTSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for CPSUICBPARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPSUICB_ACTION_ITEMS_APPLIED: u32 = 4;
pub const CPSUICB_ACTION_NONE: u32 = 0;
pub const CPSUICB_ACTION_NO_APPLY_EXIT: u32 = 3;
pub const CPSUICB_ACTION_OPTIF_CHANGED: u32 = 1;
pub const CPSUICB_ACTION_REINIT_ITEMS: u32 = 2;
pub const CPSUICB_REASON_ABOUT: u32 = 9;
pub const CPSUICB_REASON_APPLYNOW: u32 = 6;
pub const CPSUICB_REASON_DLGPROC: u32 = 3;
pub const CPSUICB_REASON_ECB_CHANGED: u32 = 2;
pub const CPSUICB_REASON_EXTPUSH: u32 = 5;
pub const CPSUICB_REASON_ITEMS_REVERTED: u32 = 8;
pub const CPSUICB_REASON_KILLACTIVE: u32 = 11;
pub const CPSUICB_REASON_OPTITEM_SETFOCUS: u32 = 7;
pub const CPSUICB_REASON_PUSHBUTTON: u32 = 1;
pub const CPSUICB_REASON_SEL_CHANGED: u32 = 0;
pub const CPSUICB_REASON_SETACTIVE: u32 = 10;
pub const CPSUICB_REASON_UNDO_CHANGES: u32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CPSUIDATABLOCK {
    pub cbData: u32,
    pub pbData: super::minwindef::LPBYTE,
}
pub const CPSUIF_ABOUT_CALLBACK: u32 = 4;
pub const CPSUIF_ICONID_AS_HICON: u32 = 2;
pub const CPSUIF_UPDATE_PERMISSION: u32 = 1;
pub const CPSUI_CANCEL: u32 = 0;
pub const CPSUI_OK: u32 = 1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub const CPSUI_PDLGPAGE_ADVDOCPROP: PDLGPAGE = PDLGPAGE(2 as _);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub const CPSUI_PDLGPAGE_DOCPROP: PDLGPAGE = PDLGPAGE(1 as _);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub const CPSUI_PDLGPAGE_PRINTERPROP: PDLGPAGE = PDLGPAGE(3 as _);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub const CPSUI_PDLGPAGE_TREEVIEWONLY: PDLGPAGE = PDLGPAGE(4 as _);
pub const CPSUI_REBOOTSYSTEM: u32 = 3;
pub const CPSUI_RESTARTWINDOWS: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct DLGPAGE {
    pub cbSize: u16,
    pub Flags: u16,
    pub DlgProc: super::winuser::DLGPROC,
    pub pTabName: super::winnt::LPTSTR,
    pub IconID: usize,
    pub Anonymous: DLGPAGE_0,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for DLGPAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union DLGPAGE_0 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::winnt::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for DLGPAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMPUB_BOOKLET_EDGE: u32 = 21;
pub const DMPUB_COLOR: u32 = 6;
pub const DMPUB_COPIES_COLLATE: u32 = 3;
pub const DMPUB_DEFSOURCE: u32 = 4;
pub const DMPUB_DITHERTYPE: u32 = 13;
pub const DMPUB_DUPLEX: u32 = 7;
pub const DMPUB_FIRST: u32 = 1;
pub const DMPUB_FORMNAME: u32 = 9;
pub const DMPUB_ICMINTENT: u32 = 11;
pub const DMPUB_ICMMETHOD: u32 = 10;
pub const DMPUB_LAST: u32 = 21;
pub const DMPUB_MANUAL_DUPLEX: u32 = 19;
pub const DMPUB_MEDIATYPE: u32 = 12;
pub const DMPUB_NONE: u32 = 0;
pub const DMPUB_NUP: u32 = 16;
pub const DMPUB_NUP_DIRECTION: u32 = 18;
pub const DMPUB_OEM_GRAPHIC_ITEM: u32 = 98;
pub const DMPUB_OEM_PAPER_ITEM: u32 = 97;
pub const DMPUB_OEM_ROOT_ITEM: u32 = 99;
pub const DMPUB_ORIENTATION: u32 = 1;
pub const DMPUB_OUTPUTBIN: u32 = 14;
pub const DMPUB_PAGEORDER: u32 = 17;
pub const DMPUB_PRINTQUALITY: u32 = 5;
pub const DMPUB_QUALITY: u32 = 15;
pub const DMPUB_SCALE: u32 = 2;
pub const DMPUB_STAPLE: u32 = 20;
pub const DMPUB_TTOPTION: u32 = 8;
pub const DMPUB_USER: u32 = 100;
pub const DPF_ICONID_AS_HICON: u32 = 1;
pub const DPF_USE_HDLGTEMPLATE: u32 = 2;
pub const DP_STD_DOCPROPPAGE1: u32 = 65533;
pub const DP_STD_DOCPROPPAGE2: u32 = 65534;
pub const DP_STD_RESERVED_START: u32 = 65520;
pub const DP_STD_TREEVIEWPAGE: u32 = 65535;
pub const ECBF_CHECKNAME_AT_FRONT: u32 = 1;
pub const ECBF_CHECKNAME_ONLY: u32 = 128;
pub const ECBF_CHECKNAME_ONLY_ENABLED: u32 = 2;
pub const ECBF_ICONID_AS_HICON: u32 = 4;
pub const ECBF_MASK: u32 = 255;
pub const ECBF_OVERLAY_ECBICON_IF_CHECKED: u32 = 16;
pub const ECBF_OVERLAY_NO_ICON: u32 = 64;
pub const ECBF_OVERLAY_STOP_ICON: u32 = 32;
pub const ECBF_OVERLAY_WARNING_ICON: u32 = 8;
pub const EPF_ICONID_AS_HICON: u32 = 8;
pub const EPF_INCL_SETUP_TITLE: u32 = 2;
pub const EPF_MASK: u32 = 255;
pub const EPF_NO_DOT_DOT_DOT: u32 = 4;
pub const EPF_OVERLAY_NO_ICON: u32 = 64;
pub const EPF_OVERLAY_STOP_ICON: u32 = 32;
pub const EPF_OVERLAY_WARNING_ICON: u32 = 16;
pub const EPF_PUSH_TYPE_DLGPROC: u32 = 1;
pub const EPF_USE_HDLGTEMPLATE: u32 = 128;
pub const ERR_CPSUI_ALLOCMEM_FAILED: i32 = -2;
pub const ERR_CPSUI_CREATEPROPPAGE_FAILED: i32 = -10;
pub const ERR_CPSUI_CREATE_IMAGELIST_FAILED: i32 = -33;
pub const ERR_CPSUI_CREATE_TRACKBAR_FAILED: i32 = -31;
pub const ERR_CPSUI_CREATE_UDARROW_FAILED: i32 = -32;
pub const ERR_CPSUI_DMCOPIES_USE_EXTPUSH: i32 = -43;
pub const ERR_CPSUI_FUNCTION_NOT_IMPLEMENTED: i32 = -9999;
pub const ERR_CPSUI_GETLASTERROR: i32 = -1;
pub const ERR_CPSUI_INTERNAL_ERROR: i32 = -10000;
pub const ERR_CPSUI_INVALID_DLGPAGEIDX: i32 = -16;
pub const ERR_CPSUI_INVALID_DLGPAGE_CBSIZE: i32 = -14;
pub const ERR_CPSUI_INVALID_DMPUBID: i32 = -29;
pub const ERR_CPSUI_INVALID_DMPUB_TVOT: i32 = -30;
pub const ERR_CPSUI_INVALID_ECB_CBSIZE: i32 = -26;
pub const ERR_CPSUI_INVALID_EDITBOX_BUF_SIZE: i32 = -25;
pub const ERR_CPSUI_INVALID_EDITBOX_PSEL: i32 = -24;
pub const ERR_CPSUI_INVALID_EXTPUSH_CBSIZE: i32 = -39;
pub const ERR_CPSUI_INVALID_LBCB_TYPE: i32 = -35;
pub const ERR_CPSUI_INVALID_LPARAM: i32 = -4;
pub const ERR_CPSUI_INVALID_OPTITEM_CBSIZE: i32 = -19;
pub const ERR_CPSUI_INVALID_OPTPARAM_CBSIZE: i32 = -23;
pub const ERR_CPSUI_INVALID_OPTTYPE_CBSIZE: i32 = -20;
pub const ERR_CPSUI_INVALID_OPTTYPE_COUNT: i32 = -21;
pub const ERR_CPSUI_INVALID_PDATA: i32 = -3;
pub const ERR_CPSUI_INVALID_PDLGPAGE: i32 = -13;
pub const ERR_CPSUI_INVALID_PUSHBUTTON_TYPE: i32 = -38;
pub const ERR_CPSUI_INVALID_TVOT_TYPE: i32 = -34;
pub const ERR_CPSUI_MORE_THAN_ONE_STDPAGE: i32 = -12;
pub const ERR_CPSUI_MORE_THAN_ONE_TVPAGE: i32 = -11;
pub const ERR_CPSUI_NO_EXTPUSH_DLGTEMPLATEID: i32 = -41;
pub const ERR_CPSUI_NO_PROPSHEETPAGE: i32 = -8;
pub const ERR_CPSUI_NULL_CALLERNAME: i32 = -6;
pub const ERR_CPSUI_NULL_ECB_PCHECKEDNAME: i32 = -28;
pub const ERR_CPSUI_NULL_ECB_PTITLE: i32 = -27;
pub const ERR_CPSUI_NULL_EXTPUSH_CALLBACK: i32 = -42;
pub const ERR_CPSUI_NULL_EXTPUSH_DLGPROC: i32 = -40;
pub const ERR_CPSUI_NULL_HINST: i32 = -5;
pub const ERR_CPSUI_NULL_OPTITEMNAME: i32 = -7;
pub const ERR_CPSUI_NULL_POPTITEM: i32 = -18;
pub const ERR_CPSUI_NULL_POPTPARAM: i32 = -22;
pub const ERR_CPSUI_SUBITEM_DIFF_DLGPAGEIDX: i32 = -17;
pub const ERR_CPSUI_SUBITEM_DIFF_OPTIF_HIDE: i32 = -36;
pub const ERR_CPSUI_TOO_MANY_DLGPAGES: i32 = -15;
pub const ERR_CPSUI_TOO_MANY_PROPSHEETPAGES: i32 = -9;
pub const ERR_CPSUI_ZERO_OPTITEM: i32 = -44;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTCHKBOX {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: super::winnt::LPTSTR,
    pub pSeparator: super::winnt::LPTSTR,
    pub pCheckedName: super::winnt::LPTSTR,
    pub IconID: usize,
    pub wReserved: [u16; 4],
    pub dwReserved: [usize; 2],
}
#[cfg(feature = "winnt")]
impl Default for EXTCHKBOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct EXTPUSH {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: super::winnt::LPTSTR,
    pub Anonymous: EXTPUSH_0,
    pub IconID: usize,
    pub Anonymous2: EXTPUSH_1,
    pub dwReserved: [usize; 3],
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for EXTPUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union EXTPUSH_0 {
    pub DlgProc: super::winuser::DLGPROC,
    pub pfnCallBack: super::minwindef::FARPROC,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for EXTPUSH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union EXTPUSH_1 {
    pub DlgTemplateID: u16,
    pub hDlgTemplate: super::winnt::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for EXTPUSH_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub const HINSPSUIPAGE_FIRST: super::winnt::HANDLE = super::winnt::HANDLE(4294967294i64 as _);
#[cfg(feature = "winnt")]
pub const HINSPSUIPAGE_LAST: super::winnt::HANDLE = super::winnt::HANDLE(4294967295i64 as _);
pub const IDI_CPSUI_ADVANCE: u32 = 64058;
pub const IDI_CPSUI_AUTOSEL: u32 = 64025;
pub const IDI_CPSUI_COLLATE: u32 = 64030;
pub const IDI_CPSUI_COLOR: u32 = 64040;
pub const IDI_CPSUI_COPY: u32 = 64046;
pub const IDI_CPSUI_DEVICE: u32 = 64060;
pub const IDI_CPSUI_DEVICE2: u32 = 64061;
pub const IDI_CPSUI_DEVICE_FEATURE: u32 = 64080;
pub const IDI_CPSUI_DITHER_COARSE: u32 = 64042;
pub const IDI_CPSUI_DITHER_FINE: u32 = 64043;
pub const IDI_CPSUI_DITHER_LINEART: u32 = 64044;
pub const IDI_CPSUI_DITHER_NONE: u32 = 64041;
pub const IDI_CPSUI_DOCUMENT: u32 = 64059;
pub const IDI_CPSUI_DUPLEX_HORZ: u32 = 64032;
pub const IDI_CPSUI_DUPLEX_HORZ_L: u32 = 64085;
pub const IDI_CPSUI_DUPLEX_NONE: u32 = 64031;
pub const IDI_CPSUI_DUPLEX_NONE_L: u32 = 64084;
pub const IDI_CPSUI_DUPLEX_VERT: u32 = 64033;
pub const IDI_CPSUI_DUPLEX_VERT_L: u32 = 64086;
pub const IDI_CPSUI_EMPTY: u32 = 64000;
pub const IDI_CPSUI_ENVELOPE: u32 = 64010;
pub const IDI_CPSUI_ENVELOPE_FEED: u32 = 64097;
pub const IDI_CPSUI_ERROR: u32 = 64050;
pub const IDI_CPSUI_FALSE: u32 = 64005;
pub const IDI_CPSUI_FAX: u32 = 64095;
pub const IDI_CPSUI_FONTCART: u32 = 64013;
pub const IDI_CPSUI_FONTCARTHDR: u32 = 64012;
pub const IDI_CPSUI_FONTCART_SLOT: u32 = 64098;
pub const IDI_CPSUI_FONTSUB: u32 = 64081;
pub const IDI_CPSUI_FORMTRAYASSIGN: u32 = 64076;
pub const IDI_CPSUI_GENERIC_ITEM: u32 = 64073;
pub const IDI_CPSUI_GENERIC_OPTION: u32 = 64072;
pub const IDI_CPSUI_GRAPHIC: u32 = 64057;
pub const IDI_CPSUI_HALFTONE_SETUP: u32 = 64048;
pub const IDI_CPSUI_HTCLRADJ: u32 = 64047;
pub const IDI_CPSUI_HT_DEVICE: u32 = 64017;
pub const IDI_CPSUI_HT_HOST: u32 = 64016;
pub const IDI_CPSUI_ICM_INTENT: u32 = 64053;
pub const IDI_CPSUI_ICM_METHOD: u32 = 64052;
pub const IDI_CPSUI_ICM_OPTION: u32 = 64051;
pub const IDI_CPSUI_ICONID_FIRST: u32 = 64000;
pub const IDI_CPSUI_ICONID_LAST: u32 = 64111;
pub const IDI_CPSUI_INSTALLABLE_OPTION: u32 = 64078;
pub const IDI_CPSUI_LANDSCAPE: u32 = 64023;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWL: u32 = 64100;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWLR: u32 = 64104;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWS: u32 = 64101;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL: u32 = 64102;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL_NB: u32 = 64106;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP: u32 = 64103;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP_NB: u32 = 64107;
pub const IDI_CPSUI_LAYOUT_BMP_PORTRAIT: u32 = 64099;
pub const IDI_CPSUI_LAYOUT_BMP_ROT_PORT: u32 = 64105;
pub const IDI_CPSUI_LF_PEN_PLOTTER: u32 = 64087;
pub const IDI_CPSUI_LF_RASTER_PLOTTER: u32 = 64089;
pub const IDI_CPSUI_MANUAL_FEED: u32 = 64094;
pub const IDI_CPSUI_MEM: u32 = 64011;
pub const IDI_CPSUI_MONO: u32 = 64039;
pub const IDI_CPSUI_NO: u32 = 64003;
pub const IDI_CPSUI_NOTINSTALLED: u32 = 64069;
pub const IDI_CPSUI_NUP_BORDER: u32 = 64111;
pub const IDI_CPSUI_OFF: u32 = 64007;
pub const IDI_CPSUI_ON: u32 = 64008;
pub const IDI_CPSUI_OPTION: u32 = 64066;
pub const IDI_CPSUI_OPTION2: u32 = 64067;
pub const IDI_CPSUI_OUTBIN: u32 = 64055;
pub const IDI_CPSUI_OUTPUT: u32 = 64056;
pub const IDI_CPSUI_PAGE_PROTECT: u32 = 64096;
pub const IDI_CPSUI_PAPER_OUTPUT: u32 = 64009;
pub const IDI_CPSUI_PAPER_TRAY: u32 = 64026;
pub const IDI_CPSUI_PAPER_TRAY2: u32 = 64027;
pub const IDI_CPSUI_PAPER_TRAY3: u32 = 64028;
pub const IDI_CPSUI_PEN_CARROUSEL: u32 = 64092;
pub const IDI_CPSUI_PLOTTER_PEN: u32 = 64093;
pub const IDI_CPSUI_PORTRAIT: u32 = 64022;
pub const IDI_CPSUI_POSTSCRIPT: u32 = 64082;
pub const IDI_CPSUI_PRINTER: u32 = 64062;
pub const IDI_CPSUI_PRINTER2: u32 = 64063;
pub const IDI_CPSUI_PRINTER3: u32 = 64064;
pub const IDI_CPSUI_PRINTER4: u32 = 64065;
pub const IDI_CPSUI_PRINTER_FEATURE: u32 = 64079;
pub const IDI_CPSUI_PRINTER_FOLDER: u32 = 64077;
pub const IDI_CPSUI_QUESTION: u32 = 64075;
pub const IDI_CPSUI_RES_DRAFT: u32 = 64034;
pub const IDI_CPSUI_RES_HIGH: u32 = 64037;
pub const IDI_CPSUI_RES_LOW: u32 = 64035;
pub const IDI_CPSUI_RES_MEDIUM: u32 = 64036;
pub const IDI_CPSUI_RES_PRESENTATION: u32 = 64038;
pub const IDI_CPSUI_ROLL_PAPER: u32 = 64091;
pub const IDI_CPSUI_ROT_LAND: u32 = 64024;
pub const IDI_CPSUI_ROT_PORT: u32 = 64110;
pub const IDI_CPSUI_RUN_DIALOG: u32 = 64074;
pub const IDI_CPSUI_SCALING: u32 = 64045;
pub const IDI_CPSUI_SEL_NONE: u32 = 64001;
pub const IDI_CPSUI_SF_PEN_PLOTTER: u32 = 64088;
pub const IDI_CPSUI_SF_RASTER_PLOTTER: u32 = 64090;
pub const IDI_CPSUI_STAPLER_OFF: u32 = 64015;
pub const IDI_CPSUI_STAPLER_ON: u32 = 64014;
pub const IDI_CPSUI_STD_FORM: u32 = 64054;
pub const IDI_CPSUI_STOP: u32 = 64068;
pub const IDI_CPSUI_STOP_WARNING_OVERLAY: u32 = 64071;
pub const IDI_CPSUI_TELEPHONE: u32 = 64083;
pub const IDI_CPSUI_TRANSPARENT: u32 = 64029;
pub const IDI_CPSUI_TRUE: u32 = 64006;
pub const IDI_CPSUI_TT_DOWNLOADSOFT: u32 = 64019;
pub const IDI_CPSUI_TT_DOWNLOADVECT: u32 = 64020;
pub const IDI_CPSUI_TT_PRINTASGRAPHIC: u32 = 64018;
pub const IDI_CPSUI_TT_SUBDEV: u32 = 64021;
pub const IDI_CPSUI_WARNING: u32 = 64002;
pub const IDI_CPSUI_WARNING_OVERLAY: u32 = 64070;
pub const IDI_CPSUI_WATERMARK: u32 = 64049;
pub const IDI_CPSUI_YES: u32 = 64004;
pub const IDS_CPSUI_ABOUT: u32 = 64848;
pub const IDS_CPSUI_ADVANCED: u32 = 64722;
pub const IDS_CPSUI_ADVANCEDOCUMENT: u32 = 64716;
pub const IDS_CPSUI_ALL: u32 = 64841;
pub const IDS_CPSUI_AUTOSELECT: u32 = 64718;
pub const IDS_CPSUI_BACKTOFRONT: u32 = 64857;
pub const IDS_CPSUI_BOND: u32 = 64786;
pub const IDS_CPSUI_BOOKLET: u32 = 64873;
pub const IDS_CPSUI_BOOKLET_EDGE: u32 = 64888;
pub const IDS_CPSUI_BOOKLET_EDGE_LEFT: u32 = 64889;
pub const IDS_CPSUI_BOOKLET_EDGE_RIGHT: u32 = 64890;
pub const IDS_CPSUI_CASSETTE_TRAY: u32 = 64810;
pub const IDS_CPSUI_CHANGE: u32 = 64702;
pub const IDS_CPSUI_CHANGED: u32 = 64846;
pub const IDS_CPSUI_CHANGES: u32 = 64845;
pub const IDS_CPSUI_COARSE: u32 = 64787;
pub const IDS_CPSUI_COLLATE: u32 = 64756;
pub const IDS_CPSUI_COLLATED: u32 = 64757;
pub const IDS_CPSUI_COLON_SEP: u32 = 64707;
pub const IDS_CPSUI_COLOR: u32 = 64764;
pub const IDS_CPSUI_COLOR_APPERANCE: u32 = 64744;
pub const IDS_CPSUI_COPIES: u32 = 64831;
pub const IDS_CPSUI_COPY: u32 = 64830;
pub const IDS_CPSUI_DEFAULT: u32 = 64732;
pub const IDS_CPSUI_DEFAULTDOCUMENT: u32 = 64714;
pub const IDS_CPSUI_DEFAULT_TRAY: u32 = 64811;
pub const IDS_CPSUI_DEVICE: u32 = 64842;
pub const IDS_CPSUI_DEVICEOPTIONS: u32 = 64725;
pub const IDS_CPSUI_DEVICE_SETTINGS: u32 = 64852;
pub const IDS_CPSUI_DITHERING: u32 = 64752;
pub const IDS_CPSUI_DOCUMENT: u32 = 64715;
pub const IDS_CPSUI_DOWN_THEN_LEFT: u32 = 64882;
pub const IDS_CPSUI_DOWN_THEN_RIGHT: u32 = 64880;
pub const IDS_CPSUI_DRAFT: u32 = 64759;
pub const IDS_CPSUI_DUPLEX: u32 = 64745;
pub const IDS_CPSUI_ENVELOPE_TRAY: u32 = 64804;
pub const IDS_CPSUI_ENVMANUAL_TRAY: u32 = 64805;
pub const IDS_CPSUI_ERRDIFFUSE: u32 = 64790;
pub const IDS_CPSUI_ERROR: u32 = 64733;
pub const IDS_CPSUI_EXIST: u32 = 64736;
pub const IDS_CPSUI_FALSE: u32 = 64726;
pub const IDS_CPSUI_FAST: u32 = 64838;
pub const IDS_CPSUI_FAX: u32 = 64835;
pub const IDS_CPSUI_FINE: u32 = 64788;
pub const IDS_CPSUI_FORMNAME: u32 = 64747;
pub const IDS_CPSUI_FORMSOURCE: u32 = 64812;
pub const IDS_CPSUI_FORMTRAYASSIGN: u32 = 64798;
pub const IDS_CPSUI_FRONTTOBACK: u32 = 64856;
pub const IDS_CPSUI_GLOSSY: u32 = 64783;
pub const IDS_CPSUI_GRAPHIC: u32 = 64720;
pub const IDS_CPSUI_GRAYSCALE: u32 = 64765;
pub const IDS_CPSUI_HALFTONE: u32 = 64791;
pub const IDS_CPSUI_HALFTONE_SETUP: u32 = 64817;
pub const IDS_CPSUI_HIGH: u32 = 64762;
pub const IDS_CPSUI_HORIZONTAL: u32 = 64768;
pub const IDS_CPSUI_HTCLRADJ: u32 = 64792;
pub const IDS_CPSUI_ICM: u32 = 64748;
pub const IDS_CPSUI_ICMINTENT: u32 = 64750;
pub const IDS_CPSUI_ICMMETHOD: u32 = 64749;
pub const IDS_CPSUI_ICM_BLACKWHITE: u32 = 64776;
pub const IDS_CPSUI_ICM_COLORMETRIC: u32 = 64781;
pub const IDS_CPSUI_ICM_CONTRAST: u32 = 64780;
pub const IDS_CPSUI_ICM_NO: u32 = 64777;
pub const IDS_CPSUI_ICM_SATURATION: u32 = 64779;
pub const IDS_CPSUI_ICM_YES: u32 = 64778;
pub const IDS_CPSUI_INSTFONTCART: u32 = 64818;
pub const IDS_CPSUI_LANDSCAPE: u32 = 64754;
pub const IDS_CPSUI_LARGECAP_TRAY: u32 = 64809;
pub const IDS_CPSUI_LARGEFMT_TRAY: u32 = 64808;
pub const IDS_CPSUI_LBCB_NOSEL: u32 = 64712;
pub const IDS_CPSUI_LEFT_ANGLE: u32 = 64708;
pub const IDS_CPSUI_LEFT_SLOT: u32 = 64823;
pub const IDS_CPSUI_LEFT_THEN_DOWN: u32 = 64881;
pub const IDS_CPSUI_LINEART: u32 = 64789;
pub const IDS_CPSUI_LONG_SIDE: u32 = 64770;
pub const IDS_CPSUI_LOW: u32 = 64760;
pub const IDS_CPSUI_LOWER_TRAY: u32 = 64801;
pub const IDS_CPSUI_MAILBOX: u32 = 64829;
pub const IDS_CPSUI_MAKE: u32 = 64833;
pub const IDS_CPSUI_MANUALFEED: u32 = 64813;
pub const IDS_CPSUI_MANUAL_DUPLEX: u32 = 64883;
pub const IDS_CPSUI_MANUAL_DUPLEX_OFF: u32 = 64885;
pub const IDS_CPSUI_MANUAL_DUPLEX_ON: u32 = 64884;
pub const IDS_CPSUI_MANUAL_TRAY: u32 = 64803;
pub const IDS_CPSUI_MEDIA: u32 = 64751;
pub const IDS_CPSUI_MEDIUM: u32 = 64761;
pub const IDS_CPSUI_MIDDLE_TRAY: u32 = 64802;
pub const IDS_CPSUI_MONOCHROME: u32 = 64766;
pub const IDS_CPSUI_MORE: u32 = 64701;
pub const IDS_CPSUI_NO: u32 = 64728;
pub const IDS_CPSUI_NONE: u32 = 64734;
pub const IDS_CPSUI_NOT: u32 = 64735;
pub const IDS_CPSUI_NOTINSTALLED: u32 = 64737;
pub const IDS_CPSUI_NO_NAME: u32 = 64850;
pub const IDS_CPSUI_NUM_OF_COPIES: u32 = 64740;
pub const IDS_CPSUI_NUP: u32 = 64864;
pub const IDS_CPSUI_NUP_BORDER: u32 = 64891;
pub const IDS_CPSUI_NUP_BORDERED: u32 = 64892;
pub const IDS_CPSUI_NUP_DIRECTION: u32 = 64878;
pub const IDS_CPSUI_NUP_FOURUP: u32 = 64867;
pub const IDS_CPSUI_NUP_NINEUP: u32 = 64869;
pub const IDS_CPSUI_NUP_NORMAL: u32 = 64865;
pub const IDS_CPSUI_NUP_SIXTEENUP: u32 = 64870;
pub const IDS_CPSUI_NUP_SIXUP: u32 = 64868;
pub const IDS_CPSUI_NUP_TWOUP: u32 = 64866;
pub const IDS_CPSUI_OF: u32 = 64704;
pub const IDS_CPSUI_OFF: u32 = 64730;
pub const IDS_CPSUI_ON: u32 = 64731;
pub const IDS_CPSUI_ONLYONE: u32 = 64800;
pub const IDS_CPSUI_OPTION: u32 = 64703;
pub const IDS_CPSUI_OPTIONS: u32 = 64721;
pub const IDS_CPSUI_ORIENTATION: u32 = 64738;
pub const IDS_CPSUI_OUTBINASSIGN: u32 = 64796;
pub const IDS_CPSUI_OUTPUTBIN: u32 = 64863;
pub const IDS_CPSUI_PAGEORDER: u32 = 64855;
pub const IDS_CPSUI_PAGEPROTECT: u32 = 64816;
pub const IDS_CPSUI_PAPER_OUTPUT: u32 = 64719;
pub const IDS_CPSUI_PERCENT: u32 = 64711;
pub const IDS_CPSUI_PLOT: u32 = 64836;
pub const IDS_CPSUI_PORTRAIT: u32 = 64753;
pub const IDS_CPSUI_POSTER: u32 = 64874;
pub const IDS_CPSUI_POSTER_2x2: u32 = 64875;
pub const IDS_CPSUI_POSTER_3x3: u32 = 64876;
pub const IDS_CPSUI_POSTER_4x4: u32 = 64877;
pub const IDS_CPSUI_PRESENTATION: u32 = 64763;
pub const IDS_CPSUI_PRINT: u32 = 64834;
pub const IDS_CPSUI_PRINTER: u32 = 64717;
pub const IDS_CPSUI_PRINTERMEM_KB: u32 = 64814;
pub const IDS_CPSUI_PRINTERMEM_MB: u32 = 64815;
pub const IDS_CPSUI_PRINTFLDSETTING: u32 = 64758;
pub const IDS_CPSUI_PRINTQUALITY: u32 = 64742;
pub const IDS_CPSUI_PROPERTIES: u32 = 64713;
pub const IDS_CPSUI_QUALITY_BEST: u32 = 64861;
pub const IDS_CPSUI_QUALITY_BETTER: u32 = 64860;
pub const IDS_CPSUI_QUALITY_CUSTOM: u32 = 64862;
pub const IDS_CPSUI_QUALITY_DRAFT: u32 = 64859;
pub const IDS_CPSUI_QUALITY_SETTINGS: u32 = 64858;
pub const IDS_CPSUI_RANGE_FROM: u32 = 64705;
pub const IDS_CPSUI_REGULAR: u32 = 64785;
pub const IDS_CPSUI_RESET: u32 = 64840;
pub const IDS_CPSUI_RESOLUTION: u32 = 64743;
pub const IDS_CPSUI_REVERT: u32 = 64844;
pub const IDS_CPSUI_RIGHT_ANGLE: u32 = 64709;
pub const IDS_CPSUI_RIGHT_SLOT: u32 = 64824;
pub const IDS_CPSUI_RIGHT_THEN_DOWN: u32 = 64879;
pub const IDS_CPSUI_ROTATED: u32 = 64839;
pub const IDS_CPSUI_ROT_LAND: u32 = 64755;
pub const IDS_CPSUI_ROT_PORT: u32 = 64886;
pub const IDS_CPSUI_SCALING: u32 = 64739;
pub const IDS_CPSUI_SETTING: u32 = 64851;
pub const IDS_CPSUI_SETTINGS: u32 = 64843;
pub const IDS_CPSUI_SETUP: u32 = 64700;
pub const IDS_CPSUI_SHORT_SIDE: u32 = 64771;
pub const IDS_CPSUI_SIDE1: u32 = 64871;
pub const IDS_CPSUI_SIDE2: u32 = 64872;
pub const IDS_CPSUI_SIMPLEX: u32 = 64767;
pub const IDS_CPSUI_SLASH_SEP: u32 = 64710;
pub const IDS_CPSUI_SLOT1: u32 = 64819;
pub const IDS_CPSUI_SLOT2: u32 = 64820;
pub const IDS_CPSUI_SLOT3: u32 = 64821;
pub const IDS_CPSUI_SLOT4: u32 = 64822;
pub const IDS_CPSUI_SLOW: u32 = 64837;
pub const IDS_CPSUI_SMALLFMT_TRAY: u32 = 64807;
pub const IDS_CPSUI_SOURCE: u32 = 64741;
pub const IDS_CPSUI_STACKER: u32 = 64828;
pub const IDS_CPSUI_STANDARD: u32 = 64782;
pub const IDS_CPSUI_STAPLE: u32 = 64887;
pub const IDS_CPSUI_STAPLER: u32 = 64825;
pub const IDS_CPSUI_STAPLER_OFF: u32 = 64827;
pub const IDS_CPSUI_STAPLER_ON: u32 = 64826;
pub const IDS_CPSUI_STDDOCPROPTAB: u32 = 64723;
pub const IDS_CPSUI_STDDOCPROPTAB1: u32 = 64853;
pub const IDS_CPSUI_STDDOCPROPTAB2: u32 = 64854;
pub const IDS_CPSUI_STDDOCPROPTVTAB: u32 = 64724;
pub const IDS_CPSUI_STRID_FIRST: u32 = 64700;
pub const IDS_CPSUI_STRID_LAST: u32 = 64892;
pub const IDS_CPSUI_TO: u32 = 64706;
pub const IDS_CPSUI_TOTAL: u32 = 64832;
pub const IDS_CPSUI_TRACTOR_TRAY: u32 = 64806;
pub const IDS_CPSUI_TRANSPARENCY: u32 = 64784;
pub const IDS_CPSUI_TRUE: u32 = 64727;
pub const IDS_CPSUI_TTOPTION: u32 = 64746;
pub const IDS_CPSUI_TT_DOWNLOADSOFT: u32 = 64773;
pub const IDS_CPSUI_TT_DOWNLOADVECT: u32 = 64774;
pub const IDS_CPSUI_TT_PRINTASGRAPHIC: u32 = 64772;
pub const IDS_CPSUI_TT_SUBDEV: u32 = 64775;
pub const IDS_CPSUI_UPPER_TRAY: u32 = 64799;
pub const IDS_CPSUI_USE_DEVICE_HT: u32 = 64794;
pub const IDS_CPSUI_USE_HOST_HT: u32 = 64793;
pub const IDS_CPSUI_USE_PRINTER_HT: u32 = 64795;
pub const IDS_CPSUI_VERSION: u32 = 64849;
pub const IDS_CPSUI_VERTICAL: u32 = 64769;
pub const IDS_CPSUI_WARNING: u32 = 64847;
pub const IDS_CPSUI_WATERMARK: u32 = 64797;
pub const IDS_CPSUI_YES: u32 = 64729;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INSERTPSUIPAGE_INFO {
    pub cbSize: u16,
    pub Type: u8,
    pub Mode: u8,
    pub dwData1: usize,
    pub dwData2: usize,
    pub dwData3: usize,
}
pub const INSPSUIPAGE_MODE_AFTER: u32 = 1;
pub const INSPSUIPAGE_MODE_BEFORE: u32 = 0;
pub const INSPSUIPAGE_MODE_FIRST_CHILD: u32 = 2;
pub const INSPSUIPAGE_MODE_INDEX: u32 = 4;
pub const INSPSUIPAGE_MODE_LAST_CHILD: u32 = 3;
pub const MAX_CPSFUNC_INDEX: u32 = 26;
pub const MAX_DLGPAGE_COUNT: u32 = 64;
pub const MAX_PROPSHEETUI_REASON_INDEX: u32 = 5;
pub const MAX_PSUIPAGEINSERT_INDEX: u32 = 5;
pub const MAX_RES_STR_CHARS: u32 = 160;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OIEXT {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::minwindef::HINSTANCE,
    pub pHelpFile: super::winnt::LPTSTR,
    pub dwReserved: [usize; 4],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for OIEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OIEXTF_ANSI_STRING: u32 = 1;
pub const OPTCF_HIDE: u32 = 1;
pub const OPTCF_MASK: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTCOMBO {
    pub cbSize: u16,
    pub Flags: u8,
    pub cListItem: u16,
    pub pListItem: POPTPARAM,
    pub Sel: i32,
    pub dwReserved: [u32; 3],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for OPTCOMBO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPTIF_CALLBACK: u32 = 4;
pub const OPTIF_CHANGED: u32 = 8;
pub const OPTIF_CHANGEONCE: u32 = 16;
pub const OPTIF_COLLAPSE: u32 = 1;
pub const OPTIF_DISABLED: u32 = 32;
pub const OPTIF_ECB_CHECKED: u32 = 64;
pub const OPTIF_EXT_DISABLED: u32 = 256;
pub const OPTIF_EXT_HIDE: u32 = 128;
pub const OPTIF_EXT_IS_EXTPUSH: u32 = 1024;
pub const OPTIF_HAS_POIEXT: u32 = 65536;
pub const OPTIF_HIDE: u32 = 2;
pub const OPTIF_INITIAL_TVITEM: u32 = 32768;
pub const OPTIF_MASK: u32 = 131071;
pub const OPTIF_NO_GROUPBOX_NAME: u32 = 2048;
pub const OPTIF_OVERLAY_NO_ICON: u32 = 16384;
pub const OPTIF_OVERLAY_STOP_ICON: u32 = 8192;
pub const OPTIF_OVERLAY_WARNING_ICON: u32 = 4096;
pub const OPTIF_SEL_AS_HICON: u32 = 512;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct OPTITEM {
    pub cbSize: u16,
    pub Level: u8,
    pub DlgPageIdx: u8,
    pub Flags: u32,
    pub UserData: usize,
    pub pName: super::winnt::LPTSTR,
    pub Anonymous: OPTITEM_0,
    pub Anonymous2: OPTITEM_1,
    pub pOptType: POPTTYPE,
    pub HelpIndex: u32,
    pub DMPubID: u8,
    pub UserItemID: u8,
    pub wReserved: u16,
    pub pOIExt: POIEXT,
    pub dwReserved: [usize; 3],
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for OPTITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union OPTITEM_0 {
    pub Sel: i32,
    pub pSel: super::winnt::LPTSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for OPTITEM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub union OPTITEM_1 {
    pub pExtChkBox: PEXTCHKBOX,
    pub pExtPush: PEXTPUSH,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for OPTITEM_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTPARAM {
    pub cbSize: u16,
    pub Flags: u8,
    pub Style: u8,
    pub pData: super::winnt::LPTSTR,
    pub IconID: usize,
    pub lParam: super::minwindef::LPARAM,
    pub dwReserved: [usize; 2],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for OPTPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPTPF_DISABLED: u32 = 2;
pub const OPTPF_HIDE: u32 = 1;
pub const OPTPF_ICONID_AS_HICON: u32 = 4;
pub const OPTPF_MASK: u32 = 127;
pub const OPTPF_OVERLAY_NO_ICON: u32 = 32;
pub const OPTPF_OVERLAY_STOP_ICON: u32 = 16;
pub const OPTPF_OVERLAY_WARNING_ICON: u32 = 8;
pub const OPTPF_USE_HDLGTEMPLATE: u32 = 64;
pub const OPTTF_MASK: u32 = 3;
pub const OPTTF_NOSPACE_BEFORE_POSTFIX: u32 = 2;
pub const OPTTF_TYPE_DISABLED: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPTTYPE {
    pub cbSize: u16,
    pub Type: u8,
    pub Flags: u8,
    pub Count: u16,
    pub BegCtrlID: u16,
    pub pOptParam: POPTPARAM,
    pub Style: u16,
    pub wReserved: [u16; 3],
    pub dwReserved: [usize; 3],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for OPTTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OTS_LBCB_INCL_ITEM_NONE: u32 = 8;
pub const OTS_LBCB_NO_ICON16_IN_ITEM: u32 = 16;
pub const OTS_LBCB_PROPPAGE_CBUSELB: u32 = 4;
pub const OTS_LBCB_PROPPAGE_LBUSECB: u32 = 2;
pub const OTS_LBCB_SORT: u32 = 1;
pub const OTS_MASK: u32 = 255;
pub const OTS_PUSH_ENABLE_ALWAYS: u32 = 128;
pub const OTS_PUSH_INCL_SETUP_TITLE: u32 = 32;
pub const OTS_PUSH_NO_DOT_DOT_DOT: u32 = 64;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOMPROPSHEETUI(pub *mut COMPROPSHEETUI);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl PCOMPROPSHEETUI {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PCOMPROPSHEETUI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCPSUICBPARAM(pub *mut CPSUICBPARAM);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl PCPSUICBPARAM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PCPSUICBPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCPSUIDATABLOCK(pub *mut CPSUIDATABLOCK);
#[cfg(feature = "minwindef")]
impl PCPSUIDATABLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PCPSUIDATABLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDLGPAGE(pub *mut DLGPAGE);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl PDLGPAGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PDLGPAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEXTCHKBOX(pub *mut EXTCHKBOX);
#[cfg(feature = "winnt")]
impl PEXTCHKBOX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "winnt")]
impl Default for PEXTCHKBOX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEXTPUSH(pub *mut EXTPUSH);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl PEXTPUSH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for PEXTPUSH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PFNCOMPROPSHEET = Option<unsafe extern "system" fn(hcompropsheet: super::winnt::HANDLE, function: u32, lparam1: super::minwindef::LPARAM, lparam2: super::minwindef::LPARAM) -> isize>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PFNPROPSHEETUI = Option<unsafe extern "system" fn(ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::minwindef::LPARAM) -> i32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PINSERTPSUIPAGE_INFO(pub *mut INSERTPSUIPAGE_INFO);
impl PINSERTPSUIPAGE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PINSERTPSUIPAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POIEXT(pub *mut OIEXT);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl POIEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for POIEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POPTCOMBO(pub *mut OPTCOMBO);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl POPTCOMBO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for POPTCOMBO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POPTITEM(pub *mut OPTITEM);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl POPTITEM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for POPTITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POPTPARAM(pub *mut OPTPARAM);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl POPTPARAM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for POPTPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POPTTYPE(pub *mut OPTTYPE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl POPTTYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for POPTTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "windef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROPSHEETUI_GETICON_INFO(pub *mut PROPSHEETUI_GETICON_INFO);
#[cfg(feature = "windef")]
impl PPROPSHEETUI_GETICON_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "windef")]
impl Default for PPROPSHEETUI_GETICON_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROPSHEETUI_INFO(pub *mut PROPSHEETUI_INFO);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl PPROPSHEETUI_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for PPROPSHEETUI_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROPSHEETUI_INFO_HEADER(pub *mut PROPSHEETUI_INFO_HEADER);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl PPROPSHEETUI_INFO_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for PPROPSHEETUI_INFO_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPSPINFO(pub *mut PSPINFO);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl PPSPINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for PPSPINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROPSHEETUI_GETICON_INFO {
    pub cbSize: u16,
    pub Flags: u16,
    pub cxIcon: u16,
    pub cyIcon: u16,
    pub hIcon: super::windef::HICON,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PROPSHEETUI_INFO {
    pub cbSize: u16,
    pub Version: u16,
    pub Flags: u16,
    pub Reason: u16,
    pub hComPropSheet: super::winnt::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
    pub lParamInit: super::minwindef::LPARAM,
    pub UserData: usize,
    pub Result: usize,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETUI_INFO_HEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: super::winnt::LPTSTR,
    pub hWndParent: super::windef::HWND,
    pub hInst: super::minwindef::HINSTANCE,
    pub Anonymous: PROPSHEETUI_INFO_HEADER_0,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for PROPSHEETUI_INFO_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union PROPSHEETUI_INFO_HEADER_0 {
    pub hIcon: super::windef::HICON,
    pub IconID: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for PROPSHEETUI_INFO_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROPSHEETUI_INFO_VERSION: u32 = 256;
pub const PROPSHEETUI_REASON_BEFORE_INIT: u32 = 5;
pub const PROPSHEETUI_REASON_DESTROY: u32 = 2;
pub const PROPSHEETUI_REASON_GET_ICON: u32 = 4;
pub const PROPSHEETUI_REASON_GET_INFO_HEADER: u32 = 1;
pub const PROPSHEETUI_REASON_INIT: u32 = 0;
pub const PROPSHEETUI_REASON_SET_RESULT: u32 = 3;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSETRESULT_INFO(pub *mut SETRESULT_INFO);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl PSETRESULT_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for PSETRESULT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PSPINFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hComPropSheet: super::winnt::HANDLE,
    pub hCPSUIPage: super::winnt::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
}
pub const PSUIHDRF_DEFTITLE: u32 = 16;
pub const PSUIHDRF_EXACT_PTITLE: u32 = 32;
pub const PSUIHDRF_NOAPPLYNOW: u32 = 2;
pub const PSUIHDRF_OBSOLETE: u32 = 1;
pub const PSUIHDRF_PROPTITLE: u32 = 4;
pub const PSUIHDRF_USEHICON: u32 = 8;
pub const PSUIINFO_UNICODE: u32 = 1;
pub const PSUIPAGEINSERT_DLL: u32 = 5;
pub const PSUIPAGEINSERT_GROUP_PARENT: u32 = 0;
pub const PSUIPAGEINSERT_HPROPSHEETPAGE: u32 = 4;
pub const PSUIPAGEINSERT_PCOMPROPSHEETUI: u32 = 1;
pub const PSUIPAGEINSERT_PFNPROPSHEETUI: u32 = 2;
pub const PSUIPAGEINSERT_PROPSHEETPAGE: u32 = 3;
pub const PUSHBUTTON_TYPE_CALLBACK: u32 = 1;
pub const PUSHBUTTON_TYPE_DLGPROC: u32 = 0;
pub const PUSHBUTTON_TYPE_HTCLRADJ: u32 = 2;
pub const PUSHBUTTON_TYPE_HTSETUP: u32 = 3;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SETRESULT_INFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hSetResult: super::winnt::HANDLE,
    pub Result: super::minwindef::LRESULT,
}
pub const SR_OWNER: u32 = 0;
pub const SR_OWNER_PARENT: u32 = 1;
pub const SSP_STDPAGE1: u32 = 10001;
pub const SSP_STDPAGE2: u32 = 10002;
pub const SSP_TVPAGE: u32 = 10000;
pub const TVOT_2STATES: u32 = 0;
pub const TVOT_3STATES: u32 = 1;
pub const TVOT_CHKBOX: u32 = 9;
pub const TVOT_COMBOBOX: u32 = 6;
pub const TVOT_EDITBOX: u32 = 7;
pub const TVOT_LAST: u32 = 10;
pub const TVOT_LISTBOX: u32 = 5;
pub const TVOT_NONE: u32 = 11;
pub const TVOT_NSTATES_EX: u32 = 10;
pub const TVOT_PUSHBUTTON: u32 = 8;
pub const TVOT_SCROLLBAR: u32 = 4;
pub const TVOT_TRACKBAR: u32 = 3;
pub const TVOT_UDARROW: u32 = 2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type _CPSUICALLBACK = Option<unsafe extern "system" fn(pcpsuicbparam: *mut CPSUICBPARAM) -> i32>;
