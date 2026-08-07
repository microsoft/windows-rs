#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("compstui.dll" "system" fn CommonPropertySheetUIA(hwndowner : super::HWND, pfnpropsheetui : PFNPROPSHEETUI, lparam : super::LPARAM, presult : *mut u32) -> i32);
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
windows_link::link!("compstui.dll" "system" fn CommonPropertySheetUIW(hwndowner : super::HWND, pfnpropsheetui : PFNPROPSHEETUI, lparam : super::LPARAM, presult : *mut u32) -> i32);
#[cfg(feature = "windef")]
windows_link::link!("compstui.dll" "system" fn GetCPSUIUserData(hdlg : super::HWND) -> usize);
#[cfg(feature = "windef")]
windows_link::link!("compstui.dll" "system" fn SetCPSUIUserData(hdlg : super::HWND, cpsuiuserdata : usize) -> windows_sys::core::BOOL);
pub const APPLYCPSUI_NO_NEWDEF: i32 = 1;
pub const APPLYCPSUI_OK_CANCEL_BUTTON: i32 = 2;
pub const CHKBOXS_FALSE_PDATA: i32 = 3;
pub const CHKBOXS_FALSE_TRUE: i32 = 0;
pub const CHKBOXS_NONE_PDATA: i32 = 6;
pub const CHKBOXS_NO_PDATA: i32 = 4;
pub const CHKBOXS_NO_YES: i32 = 1;
pub const CHKBOXS_OFF_ON: i32 = 2;
pub const CHKBOXS_OFF_PDATA: i32 = 5;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct COMPROPSHEETUI {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::HINSTANCE,
    pub pCallerName: super::LPTSTR,
    pub UserData: usize,
    pub pHelpFile: super::LPTSTR,
    pub pfnCallBack: _CPSUICALLBACK,
    pub pOptItem: POPTITEM,
    pub pDlgPage: PDLGPAGE,
    pub cOptItem: u16,
    pub cDlgPage: u16,
    pub IconID: usize,
    pub pOptItemName: super::LPTSTR,
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
pub const CPSFUNC_ADD_HPROPSHEETPAGE: i32 = 0;
pub const CPSFUNC_ADD_PCOMPROPSHEETUI: i32 = 2;
pub const CPSFUNC_ADD_PCOMPROPSHEETUIA: i32 = 2;
pub const CPSFUNC_ADD_PCOMPROPSHEETUIW: i32 = 3;
pub const CPSFUNC_ADD_PFNPROPSHEETUI: i32 = 4;
pub const CPSFUNC_ADD_PFNPROPSHEETUIA: i32 = 4;
pub const CPSFUNC_ADD_PFNPROPSHEETUIW: i32 = 5;
pub const CPSFUNC_ADD_PROPSHEETPAGE: i32 = 15;
pub const CPSFUNC_ADD_PROPSHEETPAGEA: i32 = 15;
pub const CPSFUNC_ADD_PROPSHEETPAGEW: i32 = 1;
pub const CPSFUNC_DELETE_HCOMPROPSHEET: i32 = 6;
pub const CPSFUNC_DO_APPLY_CPSUI: i32 = 25;
pub const CPSFUNC_GET_HPSUIPAGES: i32 = 10;
pub const CPSFUNC_GET_PAGECOUNT: i32 = 8;
pub const CPSFUNC_GET_PFNPROPSHEETUI_ICON: i32 = 14;
pub const CPSFUNC_IGNORE_CPSUI_PSN_APPLY: i32 = 24;
pub const CPSFUNC_INSERT_PSUIPAGE: i32 = 16;
pub const CPSFUNC_INSERT_PSUIPAGEA: i32 = 16;
pub const CPSFUNC_INSERT_PSUIPAGEW: i32 = 17;
pub const CPSFUNC_LOAD_CPSUI_ICON: i32 = 13;
pub const CPSFUNC_LOAD_CPSUI_STRING: i32 = 11;
pub const CPSFUNC_LOAD_CPSUI_STRINGA: i32 = 11;
pub const CPSFUNC_LOAD_CPSUI_STRINGW: i32 = 12;
pub const CPSFUNC_QUERY_DATABLOCK: i32 = 22;
pub const CPSFUNC_SET_DATABLOCK: i32 = 21;
pub const CPSFUNC_SET_DMPUB_HIDEBITS: i32 = 23;
pub const CPSFUNC_SET_FUSION_CONTEXT: i32 = 26;
pub const CPSFUNC_SET_HSTARTPAGE: i32 = 7;
pub const CPSFUNC_SET_PSUIPAGE_ICON: i32 = 20;
pub const CPSFUNC_SET_PSUIPAGE_TITLE: i32 = 18;
pub const CPSFUNC_SET_PSUIPAGE_TITLEA: i32 = 18;
pub const CPSFUNC_SET_PSUIPAGE_TITLEW: i32 = 19;
pub const CPSFUNC_SET_RESULT: i32 = 9;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct CPSUICBPARAM {
    pub cbSize: u16,
    pub Reason: u16,
    pub hDlg: super::HWND,
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
    pub pOldSel: super::LPTSTR,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for CPSUICBPARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPSUICB_ACTION_ITEMS_APPLIED: i32 = 4;
pub const CPSUICB_ACTION_NONE: i32 = 0;
pub const CPSUICB_ACTION_NO_APPLY_EXIT: i32 = 3;
pub const CPSUICB_ACTION_OPTIF_CHANGED: i32 = 1;
pub const CPSUICB_ACTION_REINIT_ITEMS: i32 = 2;
pub const CPSUICB_REASON_ABOUT: i32 = 9;
pub const CPSUICB_REASON_APPLYNOW: i32 = 6;
pub const CPSUICB_REASON_DLGPROC: i32 = 3;
pub const CPSUICB_REASON_ECB_CHANGED: i32 = 2;
pub const CPSUICB_REASON_EXTPUSH: i32 = 5;
pub const CPSUICB_REASON_ITEMS_REVERTED: i32 = 8;
pub const CPSUICB_REASON_KILLACTIVE: i32 = 11;
pub const CPSUICB_REASON_OPTITEM_SETFOCUS: i32 = 7;
pub const CPSUICB_REASON_PUSHBUTTON: i32 = 1;
pub const CPSUICB_REASON_SEL_CHANGED: i32 = 0;
pub const CPSUICB_REASON_SETACTIVE: i32 = 10;
pub const CPSUICB_REASON_UNDO_CHANGES: i32 = 4;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct CPSUIDATABLOCK {
    pub cbData: u32,
    pub pbData: super::LPBYTE,
}
pub const CPSUIF_ABOUT_CALLBACK: i32 = 4;
pub const CPSUIF_ICONID_AS_HICON: i32 = 2;
pub const CPSUIF_UPDATE_PERMISSION: i32 = 1;
pub const CPSUI_CANCEL: i32 = 0;
pub const CPSUI_OK: i32 = 1;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub const CPSUI_PDLGPAGE_ADVDOCPROP: PDLGPAGE = 2 as _;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub const CPSUI_PDLGPAGE_DOCPROP: PDLGPAGE = 1 as _;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub const CPSUI_PDLGPAGE_PRINTERPROP: PDLGPAGE = 3 as _;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub const CPSUI_PDLGPAGE_TREEVIEWONLY: PDLGPAGE = 4 as _;
pub const CPSUI_REBOOTSYSTEM: i32 = 3;
pub const CPSUI_RESTARTWINDOWS: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct DLGPAGE {
    pub cbSize: u16,
    pub Flags: u16,
    pub DlgProc: super::DLGPROC,
    pub pTabName: super::LPTSTR,
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
    pub hDlgTemplate: super::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for DLGPAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMPUB_BOOKLET_EDGE: i32 = 21;
pub const DMPUB_COLOR: i32 = 6;
pub const DMPUB_COPIES_COLLATE: i32 = 3;
pub const DMPUB_DEFSOURCE: i32 = 4;
pub const DMPUB_DITHERTYPE: i32 = 13;
pub const DMPUB_DUPLEX: i32 = 7;
pub const DMPUB_FIRST: i32 = 1;
pub const DMPUB_FORMNAME: i32 = 9;
pub const DMPUB_ICMINTENT: i32 = 11;
pub const DMPUB_ICMMETHOD: i32 = 10;
pub const DMPUB_LAST: i32 = 21;
pub const DMPUB_MANUAL_DUPLEX: i32 = 19;
pub const DMPUB_MEDIATYPE: i32 = 12;
pub const DMPUB_NONE: i32 = 0;
pub const DMPUB_NUP: i32 = 16;
pub const DMPUB_NUP_DIRECTION: i32 = 18;
pub const DMPUB_OEM_GRAPHIC_ITEM: i32 = 98;
pub const DMPUB_OEM_PAPER_ITEM: i32 = 97;
pub const DMPUB_OEM_ROOT_ITEM: i32 = 99;
pub const DMPUB_ORIENTATION: i32 = 1;
pub const DMPUB_OUTPUTBIN: i32 = 14;
pub const DMPUB_PAGEORDER: i32 = 17;
pub const DMPUB_PRINTQUALITY: i32 = 5;
pub const DMPUB_QUALITY: i32 = 15;
pub const DMPUB_SCALE: i32 = 2;
pub const DMPUB_STAPLE: i32 = 20;
pub const DMPUB_TTOPTION: i32 = 8;
pub const DMPUB_USER: i32 = 100;
pub const DPF_ICONID_AS_HICON: i32 = 1;
pub const DPF_USE_HDLGTEMPLATE: i32 = 2;
pub const DP_STD_DOCPROPPAGE1: i32 = 65533;
pub const DP_STD_DOCPROPPAGE2: i32 = 65534;
pub const DP_STD_RESERVED_START: i32 = 65520;
pub const DP_STD_TREEVIEWPAGE: i32 = 65535;
pub const ECBF_CHECKNAME_AT_FRONT: i32 = 1;
pub const ECBF_CHECKNAME_ONLY: i32 = 128;
pub const ECBF_CHECKNAME_ONLY_ENABLED: i32 = 2;
pub const ECBF_ICONID_AS_HICON: i32 = 4;
pub const ECBF_MASK: i32 = 255;
pub const ECBF_OVERLAY_ECBICON_IF_CHECKED: i32 = 16;
pub const ECBF_OVERLAY_NO_ICON: i32 = 64;
pub const ECBF_OVERLAY_STOP_ICON: i32 = 32;
pub const ECBF_OVERLAY_WARNING_ICON: i32 = 8;
pub const EPF_ICONID_AS_HICON: i32 = 8;
pub const EPF_INCL_SETUP_TITLE: i32 = 2;
pub const EPF_MASK: i32 = 255;
pub const EPF_NO_DOT_DOT_DOT: i32 = 4;
pub const EPF_OVERLAY_NO_ICON: i32 = 64;
pub const EPF_OVERLAY_STOP_ICON: i32 = 32;
pub const EPF_OVERLAY_WARNING_ICON: i32 = 16;
pub const EPF_PUSH_TYPE_DLGPROC: i32 = 1;
pub const EPF_USE_HDLGTEMPLATE: i32 = 128;
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
#[derive(Clone, Copy)]
pub struct EXTCHKBOX {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: super::LPTSTR,
    pub pSeparator: super::LPTSTR,
    pub pCheckedName: super::LPTSTR,
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
    pub pTitle: super::LPTSTR,
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
    pub DlgProc: super::DLGPROC,
    pub pfnCallBack: super::FARPROC,
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
    pub hDlgTemplate: super::HANDLE,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
impl Default for EXTPUSH_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub const HINSPSUIPAGE_FIRST: super::HANDLE = 4294967294i64 as _;
#[cfg(feature = "winnt")]
pub const HINSPSUIPAGE_LAST: super::HANDLE = 4294967295i64 as _;
pub const IDI_CPSUI_ADVANCE: i32 = 64058;
pub const IDI_CPSUI_AUTOSEL: i32 = 64025;
pub const IDI_CPSUI_COLLATE: i32 = 64030;
pub const IDI_CPSUI_COLOR: i32 = 64040;
pub const IDI_CPSUI_COPY: i32 = 64046;
pub const IDI_CPSUI_DEVICE: i32 = 64060;
pub const IDI_CPSUI_DEVICE2: i32 = 64061;
pub const IDI_CPSUI_DEVICE_FEATURE: i32 = 64080;
pub const IDI_CPSUI_DITHER_COARSE: i32 = 64042;
pub const IDI_CPSUI_DITHER_FINE: i32 = 64043;
pub const IDI_CPSUI_DITHER_LINEART: i32 = 64044;
pub const IDI_CPSUI_DITHER_NONE: i32 = 64041;
pub const IDI_CPSUI_DOCUMENT: i32 = 64059;
pub const IDI_CPSUI_DUPLEX_HORZ: i32 = 64032;
pub const IDI_CPSUI_DUPLEX_HORZ_L: i32 = 64085;
pub const IDI_CPSUI_DUPLEX_NONE: i32 = 64031;
pub const IDI_CPSUI_DUPLEX_NONE_L: i32 = 64084;
pub const IDI_CPSUI_DUPLEX_VERT: i32 = 64033;
pub const IDI_CPSUI_DUPLEX_VERT_L: i32 = 64086;
pub const IDI_CPSUI_EMPTY: i32 = 64000;
pub const IDI_CPSUI_ENVELOPE: i32 = 64010;
pub const IDI_CPSUI_ENVELOPE_FEED: i32 = 64097;
pub const IDI_CPSUI_ERROR: i32 = 64050;
pub const IDI_CPSUI_FALSE: i32 = 64005;
pub const IDI_CPSUI_FAX: i32 = 64095;
pub const IDI_CPSUI_FONTCART: i32 = 64013;
pub const IDI_CPSUI_FONTCARTHDR: i32 = 64012;
pub const IDI_CPSUI_FONTCART_SLOT: i32 = 64098;
pub const IDI_CPSUI_FONTSUB: i32 = 64081;
pub const IDI_CPSUI_FORMTRAYASSIGN: i32 = 64076;
pub const IDI_CPSUI_GENERIC_ITEM: i32 = 64073;
pub const IDI_CPSUI_GENERIC_OPTION: i32 = 64072;
pub const IDI_CPSUI_GRAPHIC: i32 = 64057;
pub const IDI_CPSUI_HALFTONE_SETUP: i32 = 64048;
pub const IDI_CPSUI_HTCLRADJ: i32 = 64047;
pub const IDI_CPSUI_HT_DEVICE: i32 = 64017;
pub const IDI_CPSUI_HT_HOST: i32 = 64016;
pub const IDI_CPSUI_ICM_INTENT: i32 = 64053;
pub const IDI_CPSUI_ICM_METHOD: i32 = 64052;
pub const IDI_CPSUI_ICM_OPTION: i32 = 64051;
pub const IDI_CPSUI_ICONID_FIRST: i32 = 64000;
pub const IDI_CPSUI_ICONID_LAST: i32 = 64111;
pub const IDI_CPSUI_INSTALLABLE_OPTION: i32 = 64078;
pub const IDI_CPSUI_LANDSCAPE: i32 = 64023;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWL: i32 = 64100;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWLR: i32 = 64104;
pub const IDI_CPSUI_LAYOUT_BMP_ARROWS: i32 = 64101;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL: i32 = 64102;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETL_NB: i32 = 64106;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP: i32 = 64103;
pub const IDI_CPSUI_LAYOUT_BMP_BOOKLETP_NB: i32 = 64107;
pub const IDI_CPSUI_LAYOUT_BMP_PORTRAIT: i32 = 64099;
pub const IDI_CPSUI_LAYOUT_BMP_ROT_PORT: i32 = 64105;
pub const IDI_CPSUI_LF_PEN_PLOTTER: i32 = 64087;
pub const IDI_CPSUI_LF_RASTER_PLOTTER: i32 = 64089;
pub const IDI_CPSUI_MANUAL_FEED: i32 = 64094;
pub const IDI_CPSUI_MEM: i32 = 64011;
pub const IDI_CPSUI_MONO: i32 = 64039;
pub const IDI_CPSUI_NO: i32 = 64003;
pub const IDI_CPSUI_NOTINSTALLED: i32 = 64069;
pub const IDI_CPSUI_NUP_BORDER: i32 = 64111;
pub const IDI_CPSUI_OFF: i32 = 64007;
pub const IDI_CPSUI_ON: i32 = 64008;
pub const IDI_CPSUI_OPTION: i32 = 64066;
pub const IDI_CPSUI_OPTION2: i32 = 64067;
pub const IDI_CPSUI_OUTBIN: i32 = 64055;
pub const IDI_CPSUI_OUTPUT: i32 = 64056;
pub const IDI_CPSUI_PAGE_PROTECT: i32 = 64096;
pub const IDI_CPSUI_PAPER_OUTPUT: i32 = 64009;
pub const IDI_CPSUI_PAPER_TRAY: i32 = 64026;
pub const IDI_CPSUI_PAPER_TRAY2: i32 = 64027;
pub const IDI_CPSUI_PAPER_TRAY3: i32 = 64028;
pub const IDI_CPSUI_PEN_CARROUSEL: i32 = 64092;
pub const IDI_CPSUI_PLOTTER_PEN: i32 = 64093;
pub const IDI_CPSUI_PORTRAIT: i32 = 64022;
pub const IDI_CPSUI_POSTSCRIPT: i32 = 64082;
pub const IDI_CPSUI_PRINTER: i32 = 64062;
pub const IDI_CPSUI_PRINTER2: i32 = 64063;
pub const IDI_CPSUI_PRINTER3: i32 = 64064;
pub const IDI_CPSUI_PRINTER4: i32 = 64065;
pub const IDI_CPSUI_PRINTER_FEATURE: i32 = 64079;
pub const IDI_CPSUI_PRINTER_FOLDER: i32 = 64077;
pub const IDI_CPSUI_QUESTION: i32 = 64075;
pub const IDI_CPSUI_RES_DRAFT: i32 = 64034;
pub const IDI_CPSUI_RES_HIGH: i32 = 64037;
pub const IDI_CPSUI_RES_LOW: i32 = 64035;
pub const IDI_CPSUI_RES_MEDIUM: i32 = 64036;
pub const IDI_CPSUI_RES_PRESENTATION: i32 = 64038;
pub const IDI_CPSUI_ROLL_PAPER: i32 = 64091;
pub const IDI_CPSUI_ROT_LAND: i32 = 64024;
pub const IDI_CPSUI_ROT_PORT: i32 = 64110;
pub const IDI_CPSUI_RUN_DIALOG: i32 = 64074;
pub const IDI_CPSUI_SCALING: i32 = 64045;
pub const IDI_CPSUI_SEL_NONE: i32 = 64001;
pub const IDI_CPSUI_SF_PEN_PLOTTER: i32 = 64088;
pub const IDI_CPSUI_SF_RASTER_PLOTTER: i32 = 64090;
pub const IDI_CPSUI_STAPLER_OFF: i32 = 64015;
pub const IDI_CPSUI_STAPLER_ON: i32 = 64014;
pub const IDI_CPSUI_STD_FORM: i32 = 64054;
pub const IDI_CPSUI_STOP: i32 = 64068;
pub const IDI_CPSUI_STOP_WARNING_OVERLAY: i32 = 64071;
pub const IDI_CPSUI_TELEPHONE: i32 = 64083;
pub const IDI_CPSUI_TRANSPARENT: i32 = 64029;
pub const IDI_CPSUI_TRUE: i32 = 64006;
pub const IDI_CPSUI_TT_DOWNLOADSOFT: i32 = 64019;
pub const IDI_CPSUI_TT_DOWNLOADVECT: i32 = 64020;
pub const IDI_CPSUI_TT_PRINTASGRAPHIC: i32 = 64018;
pub const IDI_CPSUI_TT_SUBDEV: i32 = 64021;
pub const IDI_CPSUI_WARNING: i32 = 64002;
pub const IDI_CPSUI_WARNING_OVERLAY: i32 = 64070;
pub const IDI_CPSUI_WATERMARK: i32 = 64049;
pub const IDI_CPSUI_YES: i32 = 64004;
pub const IDS_CPSUI_ABOUT: i32 = 64848;
pub const IDS_CPSUI_ADVANCED: i32 = 64722;
pub const IDS_CPSUI_ADVANCEDOCUMENT: i32 = 64716;
pub const IDS_CPSUI_ALL: i32 = 64841;
pub const IDS_CPSUI_AUTOSELECT: i32 = 64718;
pub const IDS_CPSUI_BACKTOFRONT: i32 = 64857;
pub const IDS_CPSUI_BOND: i32 = 64786;
pub const IDS_CPSUI_BOOKLET: i32 = 64873;
pub const IDS_CPSUI_BOOKLET_EDGE: i32 = 64888;
pub const IDS_CPSUI_BOOKLET_EDGE_LEFT: i32 = 64889;
pub const IDS_CPSUI_BOOKLET_EDGE_RIGHT: i32 = 64890;
pub const IDS_CPSUI_CASSETTE_TRAY: i32 = 64810;
pub const IDS_CPSUI_CHANGE: i32 = 64702;
pub const IDS_CPSUI_CHANGED: i32 = 64846;
pub const IDS_CPSUI_CHANGES: i32 = 64845;
pub const IDS_CPSUI_COARSE: i32 = 64787;
pub const IDS_CPSUI_COLLATE: i32 = 64756;
pub const IDS_CPSUI_COLLATED: i32 = 64757;
pub const IDS_CPSUI_COLON_SEP: i32 = 64707;
pub const IDS_CPSUI_COLOR: i32 = 64764;
pub const IDS_CPSUI_COLOR_APPERANCE: i32 = 64744;
pub const IDS_CPSUI_COPIES: i32 = 64831;
pub const IDS_CPSUI_COPY: i32 = 64830;
pub const IDS_CPSUI_DEFAULT: i32 = 64732;
pub const IDS_CPSUI_DEFAULTDOCUMENT: i32 = 64714;
pub const IDS_CPSUI_DEFAULT_TRAY: i32 = 64811;
pub const IDS_CPSUI_DEVICE: i32 = 64842;
pub const IDS_CPSUI_DEVICEOPTIONS: i32 = 64725;
pub const IDS_CPSUI_DEVICE_SETTINGS: i32 = 64852;
pub const IDS_CPSUI_DITHERING: i32 = 64752;
pub const IDS_CPSUI_DOCUMENT: i32 = 64715;
pub const IDS_CPSUI_DOWN_THEN_LEFT: i32 = 64882;
pub const IDS_CPSUI_DOWN_THEN_RIGHT: i32 = 64880;
pub const IDS_CPSUI_DRAFT: i32 = 64759;
pub const IDS_CPSUI_DUPLEX: i32 = 64745;
pub const IDS_CPSUI_ENVELOPE_TRAY: i32 = 64804;
pub const IDS_CPSUI_ENVMANUAL_TRAY: i32 = 64805;
pub const IDS_CPSUI_ERRDIFFUSE: i32 = 64790;
pub const IDS_CPSUI_ERROR: i32 = 64733;
pub const IDS_CPSUI_EXIST: i32 = 64736;
pub const IDS_CPSUI_FALSE: i32 = 64726;
pub const IDS_CPSUI_FAST: i32 = 64838;
pub const IDS_CPSUI_FAX: i32 = 64835;
pub const IDS_CPSUI_FINE: i32 = 64788;
pub const IDS_CPSUI_FORMNAME: i32 = 64747;
pub const IDS_CPSUI_FORMSOURCE: i32 = 64812;
pub const IDS_CPSUI_FORMTRAYASSIGN: i32 = 64798;
pub const IDS_CPSUI_FRONTTOBACK: i32 = 64856;
pub const IDS_CPSUI_GLOSSY: i32 = 64783;
pub const IDS_CPSUI_GRAPHIC: i32 = 64720;
pub const IDS_CPSUI_GRAYSCALE: i32 = 64765;
pub const IDS_CPSUI_HALFTONE: i32 = 64791;
pub const IDS_CPSUI_HALFTONE_SETUP: i32 = 64817;
pub const IDS_CPSUI_HIGH: i32 = 64762;
pub const IDS_CPSUI_HORIZONTAL: i32 = 64768;
pub const IDS_CPSUI_HTCLRADJ: i32 = 64792;
pub const IDS_CPSUI_ICM: i32 = 64748;
pub const IDS_CPSUI_ICMINTENT: i32 = 64750;
pub const IDS_CPSUI_ICMMETHOD: i32 = 64749;
pub const IDS_CPSUI_ICM_BLACKWHITE: i32 = 64776;
pub const IDS_CPSUI_ICM_COLORMETRIC: i32 = 64781;
pub const IDS_CPSUI_ICM_CONTRAST: i32 = 64780;
pub const IDS_CPSUI_ICM_NO: i32 = 64777;
pub const IDS_CPSUI_ICM_SATURATION: i32 = 64779;
pub const IDS_CPSUI_ICM_YES: i32 = 64778;
pub const IDS_CPSUI_INSTFONTCART: i32 = 64818;
pub const IDS_CPSUI_LANDSCAPE: i32 = 64754;
pub const IDS_CPSUI_LARGECAP_TRAY: i32 = 64809;
pub const IDS_CPSUI_LARGEFMT_TRAY: i32 = 64808;
pub const IDS_CPSUI_LBCB_NOSEL: i32 = 64712;
pub const IDS_CPSUI_LEFT_ANGLE: i32 = 64708;
pub const IDS_CPSUI_LEFT_SLOT: i32 = 64823;
pub const IDS_CPSUI_LEFT_THEN_DOWN: i32 = 64881;
pub const IDS_CPSUI_LINEART: i32 = 64789;
pub const IDS_CPSUI_LONG_SIDE: i32 = 64770;
pub const IDS_CPSUI_LOW: i32 = 64760;
pub const IDS_CPSUI_LOWER_TRAY: i32 = 64801;
pub const IDS_CPSUI_MAILBOX: i32 = 64829;
pub const IDS_CPSUI_MAKE: i32 = 64833;
pub const IDS_CPSUI_MANUALFEED: i32 = 64813;
pub const IDS_CPSUI_MANUAL_DUPLEX: i32 = 64883;
pub const IDS_CPSUI_MANUAL_DUPLEX_OFF: i32 = 64885;
pub const IDS_CPSUI_MANUAL_DUPLEX_ON: i32 = 64884;
pub const IDS_CPSUI_MANUAL_TRAY: i32 = 64803;
pub const IDS_CPSUI_MEDIA: i32 = 64751;
pub const IDS_CPSUI_MEDIUM: i32 = 64761;
pub const IDS_CPSUI_MIDDLE_TRAY: i32 = 64802;
pub const IDS_CPSUI_MONOCHROME: i32 = 64766;
pub const IDS_CPSUI_MORE: i32 = 64701;
pub const IDS_CPSUI_NO: i32 = 64728;
pub const IDS_CPSUI_NONE: i32 = 64734;
pub const IDS_CPSUI_NOT: i32 = 64735;
pub const IDS_CPSUI_NOTINSTALLED: i32 = 64737;
pub const IDS_CPSUI_NO_NAME: i32 = 64850;
pub const IDS_CPSUI_NUM_OF_COPIES: i32 = 64740;
pub const IDS_CPSUI_NUP: i32 = 64864;
pub const IDS_CPSUI_NUP_BORDER: i32 = 64891;
pub const IDS_CPSUI_NUP_BORDERED: i32 = 64892;
pub const IDS_CPSUI_NUP_DIRECTION: i32 = 64878;
pub const IDS_CPSUI_NUP_FOURUP: i32 = 64867;
pub const IDS_CPSUI_NUP_NINEUP: i32 = 64869;
pub const IDS_CPSUI_NUP_NORMAL: i32 = 64865;
pub const IDS_CPSUI_NUP_SIXTEENUP: i32 = 64870;
pub const IDS_CPSUI_NUP_SIXUP: i32 = 64868;
pub const IDS_CPSUI_NUP_TWOUP: i32 = 64866;
pub const IDS_CPSUI_OF: i32 = 64704;
pub const IDS_CPSUI_OFF: i32 = 64730;
pub const IDS_CPSUI_ON: i32 = 64731;
pub const IDS_CPSUI_ONLYONE: i32 = 64800;
pub const IDS_CPSUI_OPTION: i32 = 64703;
pub const IDS_CPSUI_OPTIONS: i32 = 64721;
pub const IDS_CPSUI_ORIENTATION: i32 = 64738;
pub const IDS_CPSUI_OUTBINASSIGN: i32 = 64796;
pub const IDS_CPSUI_OUTPUTBIN: i32 = 64863;
pub const IDS_CPSUI_PAGEORDER: i32 = 64855;
pub const IDS_CPSUI_PAGEPROTECT: i32 = 64816;
pub const IDS_CPSUI_PAPER_OUTPUT: i32 = 64719;
pub const IDS_CPSUI_PERCENT: i32 = 64711;
pub const IDS_CPSUI_PLOT: i32 = 64836;
pub const IDS_CPSUI_PORTRAIT: i32 = 64753;
pub const IDS_CPSUI_POSTER: i32 = 64874;
pub const IDS_CPSUI_POSTER_2x2: i32 = 64875;
pub const IDS_CPSUI_POSTER_3x3: i32 = 64876;
pub const IDS_CPSUI_POSTER_4x4: i32 = 64877;
pub const IDS_CPSUI_PRESENTATION: i32 = 64763;
pub const IDS_CPSUI_PRINT: i32 = 64834;
pub const IDS_CPSUI_PRINTER: i32 = 64717;
pub const IDS_CPSUI_PRINTERMEM_KB: i32 = 64814;
pub const IDS_CPSUI_PRINTERMEM_MB: i32 = 64815;
pub const IDS_CPSUI_PRINTFLDSETTING: i32 = 64758;
pub const IDS_CPSUI_PRINTQUALITY: i32 = 64742;
pub const IDS_CPSUI_PROPERTIES: i32 = 64713;
pub const IDS_CPSUI_QUALITY_BEST: i32 = 64861;
pub const IDS_CPSUI_QUALITY_BETTER: i32 = 64860;
pub const IDS_CPSUI_QUALITY_CUSTOM: i32 = 64862;
pub const IDS_CPSUI_QUALITY_DRAFT: i32 = 64859;
pub const IDS_CPSUI_QUALITY_SETTINGS: i32 = 64858;
pub const IDS_CPSUI_RANGE_FROM: i32 = 64705;
pub const IDS_CPSUI_REGULAR: i32 = 64785;
pub const IDS_CPSUI_RESET: i32 = 64840;
pub const IDS_CPSUI_RESOLUTION: i32 = 64743;
pub const IDS_CPSUI_REVERT: i32 = 64844;
pub const IDS_CPSUI_RIGHT_ANGLE: i32 = 64709;
pub const IDS_CPSUI_RIGHT_SLOT: i32 = 64824;
pub const IDS_CPSUI_RIGHT_THEN_DOWN: i32 = 64879;
pub const IDS_CPSUI_ROTATED: i32 = 64839;
pub const IDS_CPSUI_ROT_LAND: i32 = 64755;
pub const IDS_CPSUI_ROT_PORT: i32 = 64886;
pub const IDS_CPSUI_SCALING: i32 = 64739;
pub const IDS_CPSUI_SETTING: i32 = 64851;
pub const IDS_CPSUI_SETTINGS: i32 = 64843;
pub const IDS_CPSUI_SETUP: i32 = 64700;
pub const IDS_CPSUI_SHORT_SIDE: i32 = 64771;
pub const IDS_CPSUI_SIDE1: i32 = 64871;
pub const IDS_CPSUI_SIDE2: i32 = 64872;
pub const IDS_CPSUI_SIMPLEX: i32 = 64767;
pub const IDS_CPSUI_SLASH_SEP: i32 = 64710;
pub const IDS_CPSUI_SLOT1: i32 = 64819;
pub const IDS_CPSUI_SLOT2: i32 = 64820;
pub const IDS_CPSUI_SLOT3: i32 = 64821;
pub const IDS_CPSUI_SLOT4: i32 = 64822;
pub const IDS_CPSUI_SLOW: i32 = 64837;
pub const IDS_CPSUI_SMALLFMT_TRAY: i32 = 64807;
pub const IDS_CPSUI_SOURCE: i32 = 64741;
pub const IDS_CPSUI_STACKER: i32 = 64828;
pub const IDS_CPSUI_STANDARD: i32 = 64782;
pub const IDS_CPSUI_STAPLE: i32 = 64887;
pub const IDS_CPSUI_STAPLER: i32 = 64825;
pub const IDS_CPSUI_STAPLER_OFF: i32 = 64827;
pub const IDS_CPSUI_STAPLER_ON: i32 = 64826;
pub const IDS_CPSUI_STDDOCPROPTAB: i32 = 64723;
pub const IDS_CPSUI_STDDOCPROPTAB1: i32 = 64853;
pub const IDS_CPSUI_STDDOCPROPTAB2: i32 = 64854;
pub const IDS_CPSUI_STDDOCPROPTVTAB: i32 = 64724;
pub const IDS_CPSUI_STRID_FIRST: i32 = 64700;
pub const IDS_CPSUI_STRID_LAST: i32 = 64892;
pub const IDS_CPSUI_TO: i32 = 64706;
pub const IDS_CPSUI_TOTAL: i32 = 64832;
pub const IDS_CPSUI_TRACTOR_TRAY: i32 = 64806;
pub const IDS_CPSUI_TRANSPARENCY: i32 = 64784;
pub const IDS_CPSUI_TRUE: i32 = 64727;
pub const IDS_CPSUI_TTOPTION: i32 = 64746;
pub const IDS_CPSUI_TT_DOWNLOADSOFT: i32 = 64773;
pub const IDS_CPSUI_TT_DOWNLOADVECT: i32 = 64774;
pub const IDS_CPSUI_TT_PRINTASGRAPHIC: i32 = 64772;
pub const IDS_CPSUI_TT_SUBDEV: i32 = 64775;
pub const IDS_CPSUI_UPPER_TRAY: i32 = 64799;
pub const IDS_CPSUI_USE_DEVICE_HT: i32 = 64794;
pub const IDS_CPSUI_USE_HOST_HT: i32 = 64793;
pub const IDS_CPSUI_USE_PRINTER_HT: i32 = 64795;
pub const IDS_CPSUI_VERSION: i32 = 64849;
pub const IDS_CPSUI_VERTICAL: i32 = 64769;
pub const IDS_CPSUI_WARNING: i32 = 64847;
pub const IDS_CPSUI_WATERMARK: i32 = 64797;
pub const IDS_CPSUI_YES: i32 = 64729;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INSERTPSUIPAGE_INFO {
    pub cbSize: u16,
    pub Type: u8,
    pub Mode: u8,
    pub dwData1: usize,
    pub dwData2: usize,
    pub dwData3: usize,
}
pub const INSPSUIPAGE_MODE_AFTER: i32 = 1;
pub const INSPSUIPAGE_MODE_BEFORE: i32 = 0;
pub const INSPSUIPAGE_MODE_FIRST_CHILD: i32 = 2;
pub const INSPSUIPAGE_MODE_INDEX: i32 = 4;
pub const INSPSUIPAGE_MODE_LAST_CHILD: i32 = 3;
pub const MAX_CPSFUNC_INDEX: i32 = 26;
pub const MAX_DLGPAGE_COUNT: i32 = 64;
pub const MAX_PROPSHEETUI_REASON_INDEX: i32 = 5;
pub const MAX_PSUIPAGEINSERT_INDEX: i32 = 5;
pub const MAX_RES_STR_CHARS: i32 = 160;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct OIEXT {
    pub cbSize: u16,
    pub Flags: u16,
    pub hInstCaller: super::HINSTANCE,
    pub pHelpFile: super::LPTSTR,
    pub dwReserved: [usize; 4],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for OIEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OIEXTF_ANSI_STRING: i32 = 1;
pub const OPTCF_HIDE: i32 = 1;
pub const OPTCF_MASK: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
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
pub const OPTIF_CALLBACK: i32 = 4;
pub const OPTIF_CHANGED: i32 = 8;
pub const OPTIF_CHANGEONCE: i32 = 16;
pub const OPTIF_COLLAPSE: i32 = 1;
pub const OPTIF_DISABLED: i32 = 32;
pub const OPTIF_ECB_CHECKED: i32 = 64;
pub const OPTIF_EXT_DISABLED: i32 = 256;
pub const OPTIF_EXT_HIDE: i32 = 128;
pub const OPTIF_EXT_IS_EXTPUSH: i32 = 1024;
pub const OPTIF_HAS_POIEXT: i32 = 65536;
pub const OPTIF_HIDE: i32 = 2;
pub const OPTIF_INITIAL_TVITEM: i32 = 32768;
pub const OPTIF_MASK: i32 = 131071;
pub const OPTIF_NO_GROUPBOX_NAME: i32 = 2048;
pub const OPTIF_OVERLAY_NO_ICON: i32 = 16384;
pub const OPTIF_OVERLAY_STOP_ICON: i32 = 8192;
pub const OPTIF_OVERLAY_WARNING_ICON: i32 = 4096;
pub const OPTIF_SEL_AS_HICON: i32 = 512;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
#[derive(Clone, Copy)]
pub struct OPTITEM {
    pub cbSize: u16,
    pub Level: u8,
    pub DlgPageIdx: u8,
    pub Flags: u32,
    pub UserData: usize,
    pub pName: super::LPTSTR,
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
    pub pSel: super::LPTSTR,
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
#[derive(Clone, Copy)]
pub struct OPTPARAM {
    pub cbSize: u16,
    pub Flags: u8,
    pub Style: u8,
    pub pData: super::LPTSTR,
    pub IconID: usize,
    pub lParam: super::LPARAM,
    pub dwReserved: [usize; 2],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for OPTPARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OPTPF_DISABLED: i32 = 2;
pub const OPTPF_HIDE: i32 = 1;
pub const OPTPF_ICONID_AS_HICON: i32 = 4;
pub const OPTPF_MASK: i32 = 127;
pub const OPTPF_OVERLAY_NO_ICON: i32 = 32;
pub const OPTPF_OVERLAY_STOP_ICON: i32 = 16;
pub const OPTPF_OVERLAY_WARNING_ICON: i32 = 8;
pub const OPTPF_USE_HDLGTEMPLATE: i32 = 64;
pub const OPTTF_MASK: i32 = 3;
pub const OPTTF_NOSPACE_BEFORE_POSTFIX: i32 = 2;
pub const OPTTF_TYPE_DISABLED: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
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
pub const OTS_LBCB_INCL_ITEM_NONE: i32 = 8;
pub const OTS_LBCB_NO_ICON16_IN_ITEM: i32 = 16;
pub const OTS_LBCB_PROPPAGE_CBUSELB: i32 = 4;
pub const OTS_LBCB_PROPPAGE_LBUSECB: i32 = 2;
pub const OTS_LBCB_SORT: i32 = 1;
pub const OTS_MASK: i32 = 255;
pub const OTS_PUSH_ENABLE_ALWAYS: i32 = 128;
pub const OTS_PUSH_INCL_SETUP_TITLE: i32 = 32;
pub const OTS_PUSH_NO_DOT_DOT_DOT: i32 = 64;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PCOMPROPSHEETUI = *mut COMPROPSHEETUI;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PCPSUICBPARAM = *mut CPSUICBPARAM;
#[cfg(feature = "minwindef")]
pub type PCPSUIDATABLOCK = *mut CPSUIDATABLOCK;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PDLGPAGE = *mut DLGPAGE;
#[cfg(feature = "winnt")]
pub type PEXTCHKBOX = *mut EXTCHKBOX;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type PEXTPUSH = *mut EXTPUSH;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PFNCOMPROPSHEET = Option<unsafe extern "system" fn(hcompropsheet: super::HANDLE, function: u32, lparam1: super::LPARAM, lparam2: super::LPARAM) -> isize>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PFNPROPSHEETUI = Option<unsafe extern "system" fn(ppsuiinfo: *mut PROPSHEETUI_INFO, lparam: super::LPARAM) -> i32>;
pub type PINSERTPSUIPAGE_INFO = *mut INSERTPSUIPAGE_INFO;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type POIEXT = *mut OIEXT;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type POPTCOMBO = *mut OPTCOMBO;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type POPTITEM = *mut OPTITEM;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type POPTPARAM = *mut OPTPARAM;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type POPTTYPE = *mut OPTTYPE;
#[cfg(feature = "windef")]
pub type PPROPSHEETUI_GETICON_INFO = *mut PROPSHEETUI_GETICON_INFO;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PPROPSHEETUI_INFO = *mut PROPSHEETUI_INFO;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
pub type PPROPSHEETUI_INFO_HEADER = *mut PROPSHEETUI_INFO_HEADER;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PPSPINFO = *mut PSPINFO;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct PROPSHEETUI_GETICON_INFO {
    pub cbSize: u16,
    pub Flags: u16,
    pub cxIcon: u16,
    pub cyIcon: u16,
    pub hIcon: super::HICON,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PROPSHEETUI_INFO {
    pub cbSize: u16,
    pub Version: u16,
    pub Flags: u16,
    pub Reason: u16,
    pub hComPropSheet: super::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
    pub lParamInit: super::LPARAM,
    pub UserData: usize,
    pub Result: usize,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROPSHEETUI_INFO_HEADER {
    pub cbSize: u16,
    pub Flags: u16,
    pub pTitle: super::LPTSTR,
    pub hWndParent: super::HWND,
    pub hInst: super::HINSTANCE,
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
    pub hIcon: super::HICON,
    pub IconID: usize,
}
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt"))]
impl Default for PROPSHEETUI_INFO_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROPSHEETUI_INFO_VERSION: i32 = 256;
pub const PROPSHEETUI_REASON_BEFORE_INIT: i32 = 5;
pub const PROPSHEETUI_REASON_DESTROY: i32 = 2;
pub const PROPSHEETUI_REASON_GET_ICON: i32 = 4;
pub const PROPSHEETUI_REASON_GET_INFO_HEADER: i32 = 1;
pub const PROPSHEETUI_REASON_INIT: i32 = 0;
pub const PROPSHEETUI_REASON_SET_RESULT: i32 = 3;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PSETRESULT_INFO = *mut SETRESULT_INFO;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PSPINFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hComPropSheet: super::HANDLE,
    pub hCPSUIPage: super::HANDLE,
    pub pfnComPropSheet: PFNCOMPROPSHEET,
}
pub const PSUIHDRF_DEFTITLE: i32 = 16;
pub const PSUIHDRF_EXACT_PTITLE: i32 = 32;
pub const PSUIHDRF_NOAPPLYNOW: i32 = 2;
pub const PSUIHDRF_OBSOLETE: i32 = 1;
pub const PSUIHDRF_PROPTITLE: i32 = 4;
pub const PSUIHDRF_USEHICON: i32 = 8;
pub const PSUIINFO_UNICODE: i32 = 1;
pub const PSUIPAGEINSERT_DLL: i32 = 5;
pub const PSUIPAGEINSERT_GROUP_PARENT: i32 = 0;
pub const PSUIPAGEINSERT_HPROPSHEETPAGE: i32 = 4;
pub const PSUIPAGEINSERT_PCOMPROPSHEETUI: i32 = 1;
pub const PSUIPAGEINSERT_PFNPROPSHEETUI: i32 = 2;
pub const PSUIPAGEINSERT_PROPSHEETPAGE: i32 = 3;
pub const PUSHBUTTON_TYPE_CALLBACK: i32 = 1;
pub const PUSHBUTTON_TYPE_DLGPROC: i32 = 0;
pub const PUSHBUTTON_TYPE_HTCLRADJ: i32 = 2;
pub const PUSHBUTTON_TYPE_HTSETUP: i32 = 3;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SETRESULT_INFO {
    pub cbSize: u16,
    pub wReserved: u16,
    pub hSetResult: super::HANDLE,
    pub Result: super::LRESULT,
}
pub const SR_OWNER: i32 = 0;
pub const SR_OWNER_PARENT: i32 = 1;
pub const SSP_STDPAGE1: i32 = 10001;
pub const SSP_STDPAGE2: i32 = 10002;
pub const SSP_TVPAGE: i32 = 10000;
pub const TVOT_2STATES: i32 = 0;
pub const TVOT_3STATES: i32 = 1;
pub const TVOT_CHKBOX: i32 = 9;
pub const TVOT_COMBOBOX: i32 = 6;
pub const TVOT_EDITBOX: i32 = 7;
pub const TVOT_LAST: i32 = 10;
pub const TVOT_LISTBOX: i32 = 5;
pub const TVOT_NONE: i32 = 11;
pub const TVOT_NSTATES_EX: i32 = 10;
pub const TVOT_PUSHBUTTON: i32 = 8;
pub const TVOT_SCROLLBAR: i32 = 4;
pub const TVOT_TRACKBAR: i32 = 3;
pub const TVOT_UDARROW: i32 = 2;
#[cfg(all(feature = "minwindef", feature = "windef", feature = "winnt", feature = "winuser"))]
pub type _CPSUICALLBACK = Option<unsafe extern "system" fn(pcpsuicbparam: *mut CPSUICBPARAM) -> i32>;
