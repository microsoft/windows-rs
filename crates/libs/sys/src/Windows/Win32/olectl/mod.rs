#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn OleCreateFontIndirect(lpfontdesc : LPFONTDESC, riid : *const windows_sys::core::GUID, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("oleaut32.dll" "system" fn OleCreatePictureIndirect(lppictdesc : LPPICTDESC, riid : *const windows_sys::core::GUID, fown : windows_sys::core::BOOL, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn OleCreatePropertyFrame(hwndowner : super::HWND, x : u32, y : u32, lpszcaption : super::LPCOLESTR, cobjects : u32, ppunk : *mut *mut core::ffi::c_void, cpages : u32, ppageclsid : super::LPCLSID, lcid : super::LCID, dwreserved : u32, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn OleCreatePropertyFrameIndirect(lpparams : LPOCPFIPARAMS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("oleaut32.dll" "system" fn OleIconToCursor(hinstexe : super::HINSTANCE, hicon : super::HICON) -> super::HCURSOR);
#[cfg(feature = "objidlbase")]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPicture(lpstream : super::LPSTREAM, lsize : i32, frunmode : windows_sys::core::BOOL, riid : *const windows_sys::core::GUID, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPictureEx(lpstream : super::LPSTREAM, lsize : i32, frunmode : windows_sys::core::BOOL, riid : *const windows_sys::core::GUID, xsizedesired : u32, ysizedesired : u32, dwflags : u32, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPictureFile(varfilename : super::VARIANT, lplpdisppicture : *mut super::LPDISPATCH) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPictureFileEx(varfilename : super::VARIANT, xsizedesired : u32, ysizedesired : u32, dwflags : u32, lplpdisppicture : *mut super::LPDISPATCH) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "ocidl", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPicturePath(szurlorpath : super::LPOLESTR, punkcaller : *mut core::ffi::c_void, dwreserved : u32, clrreserved : super::OLE_COLOR, riid : *const windows_sys::core::GUID, ppvret : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "oaidl")]
windows_link::link!("oleaut32.dll" "system" fn OleSavePictureFile(lpdisppicture : super::LPDISPATCH, bstrfilename : windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "ocidl", feature = "windef"))]
windows_link::link!("oleaut32.dll" "system" fn OleTranslateColor(clr : super::OLE_COLOR, hpal : super::HPALETTE, lpcolorref : *mut super::COLORREF) -> windows_sys::core::HRESULT);
pub const CLSID_CColorPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35201_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CFontPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35200_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CPicturePropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35202_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_ConvertVBX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb8f0822_0164_101b_84ed_08002b2ec713);
pub const CLSID_PersistPropset: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb8f0821_0164_101b_84ed_08002b2ec713);
pub const CLSID_StdFont: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35203_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_StdPicture: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35204_8f91_11ce_9de3_00aa004bb851);
#[cfg(feature = "wtypesbase")]
pub const CONNECT_E_ADVISELIMIT: super::SCODE = 0x80040201_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CONNECT_E_CANNOTCONNECT: super::SCODE = 0x80040202_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CONNECT_E_FIRST: super::SCODE = 0x80040200_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CONNECT_E_LAST: super::SCODE = 0x8004020F_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CONNECT_E_NOCONNECTION: super::SCODE = 0x80040200_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CONNECT_E_OVERRIDDEN: super::SCODE = 0x80040203_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CONNECT_S_FIRST: super::SCODE = 0x40200_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CONNECT_S_LAST: super::SCODE = 0x4020F_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_BADFILEMODE: super::SCODE = 0x800A0036_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_BADFILENAME: super::SCODE = 0x800A0040_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_BADFILENAMEORNUMBER: super::SCODE = 0x800A0034_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_BADRECORDLENGTH: super::SCODE = 0x800A003B_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_BADRECORDNUMBER: super::SCODE = 0x800A003F_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_CANTSAVEFILETOTEMP: super::SCODE = 0x800A02DF_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_CUSTOM_FIRST: super::SCODE = 0x800A0258_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_DEVICEIOERROR: super::SCODE = 0x800A0039_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_DEVICEUNAVAILABLE: super::SCODE = 0x800A0044_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_DISKFULL: super::SCODE = 0x800A003D_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_DISKNOTREADY: super::SCODE = 0x800A0047_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_DIVISIONBYZERO: super::SCODE = 0x800A000B_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_FILEALREADYEXISTS: super::SCODE = 0x800A003A_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_FILEALREADYOPEN: super::SCODE = 0x800A0037_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_FILENOTFOUND: super::SCODE = 0x800A0035_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_GETNOTSUPPORTED: super::SCODE = 0x800A018A_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_GETNOTSUPPORTEDATRUNTIME: super::SCODE = 0x800A0189_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_ILLEGALFUNCTIONCALL: super::SCODE = 0x800A0005_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_INVALIDCLIPBOARDFORMAT: super::SCODE = 0x800A01CC_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_INVALIDFILEFORMAT: super::SCODE = 0x800A0141_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_INVALIDPATTERNSTRING: super::SCODE = 0x800A005D_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_INVALIDPICTURE: super::SCODE = 0x800A01E1_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_INVALIDPROPERTYARRAYINDEX: super::SCODE = 0x800A017D_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_INVALIDPROPERTYVALUE: super::SCODE = 0x800A017C_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_INVALIDUSEOFNULL: super::SCODE = 0x800A005E_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_NEEDPROPERTYARRAYINDEX: super::SCODE = 0x800A0181_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_OUTOFMEMORY: super::SCODE = 0x800A0007_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_OUTOFSTACKSPACE: super::SCODE = 0x800A001C_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_OUTOFSTRINGSPACE: super::SCODE = 0x800A000E_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_OVERFLOW: super::SCODE = 0x800A0006_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_PATHFILEACCESSERROR: super::SCODE = 0x800A004B_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_PATHNOTFOUND: super::SCODE = 0x800A004C_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_PERMISSIONDENIED: super::SCODE = 0x800A0046_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_PRINTERERROR: super::SCODE = 0x800A01E2_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_PROPERTYNOTFOUND: super::SCODE = 0x800A01A6_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_REPLACEMENTSTOOLONG: super::SCODE = 0x800A02EA_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_SEARCHTEXTNOTFOUND: super::SCODE = 0x800A02E8_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_SETNOTPERMITTED: super::SCODE = 0x800A0183_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_SETNOTSUPPORTED: super::SCODE = 0x800A017F_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_SETNOTSUPPORTEDATRUNTIME: super::SCODE = 0x800A017E_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const CTL_E_TOOMANYFILES: super::SCODE = 0x800A0043_u32 as _;
pub const DISPID_ABOUTBOX: i32 = -552;
pub const DISPID_ACCELERATOR: i32 = -543;
pub const DISPID_ADDITEM: i32 = -553;
pub const DISPID_AMBIENT_APPEARANCE: i32 = -716;
pub const DISPID_AMBIENT_AUTOCLIP: i32 = -715;
pub const DISPID_AMBIENT_BACKCOLOR: i32 = -701;
pub const DISPID_AMBIENT_CHARSET: i32 = -727;
pub const DISPID_AMBIENT_CODEPAGE: i32 = -725;
pub const DISPID_AMBIENT_DISPLAYASDEFAULT: i32 = -713;
pub const DISPID_AMBIENT_DISPLAYNAME: i32 = -702;
pub const DISPID_AMBIENT_FONT: i32 = -703;
pub const DISPID_AMBIENT_FORECOLOR: i32 = -704;
pub const DISPID_AMBIENT_LOCALEID: i32 = -705;
pub const DISPID_AMBIENT_MESSAGEREFLECT: i32 = -706;
pub const DISPID_AMBIENT_PALETTE: i32 = -726;
pub const DISPID_AMBIENT_RIGHTTOLEFT: i32 = -732;
pub const DISPID_AMBIENT_SCALEUNITS: i32 = -707;
pub const DISPID_AMBIENT_SHOWGRABHANDLES: i32 = -711;
pub const DISPID_AMBIENT_SHOWHATCHING: i32 = -712;
pub const DISPID_AMBIENT_SUPPORTSMNEMONICS: i32 = -714;
pub const DISPID_AMBIENT_TEXTALIGN: i32 = -708;
pub const DISPID_AMBIENT_TOPTOBOTTOM: i32 = -733;
pub const DISPID_AMBIENT_TRANSFERPRIORITY: i32 = -728;
pub const DISPID_AMBIENT_UIDEAD: i32 = -710;
pub const DISPID_AMBIENT_USERMODE: i32 = -709;
pub const DISPID_APPEARANCE: i32 = -520;
pub const DISPID_AUTOSIZE: i32 = -500;
pub const DISPID_BACKCOLOR: i32 = -501;
pub const DISPID_BACKSTYLE: i32 = -502;
pub const DISPID_BORDERCOLOR: i32 = -503;
pub const DISPID_BORDERSTYLE: i32 = -504;
pub const DISPID_BORDERVISIBLE: i32 = -519;
pub const DISPID_BORDERWIDTH: i32 = -505;
pub const DISPID_CAPTION: i32 = -518;
pub const DISPID_CLEAR: i32 = -554;
pub const DISPID_CLICK: i32 = -600;
pub const DISPID_CLICK_VALUE: i32 = -610;
pub const DISPID_COLUMN: i32 = -529;
pub const DISPID_DBLCLICK: i32 = -601;
pub const DISPID_DISPLAYSTYLE: i32 = -540;
pub const DISPID_DOCLICK: i32 = -551;
pub const DISPID_DRAWMODE: i32 = -507;
pub const DISPID_DRAWSTYLE: i32 = -508;
pub const DISPID_DRAWWIDTH: i32 = -509;
pub const DISPID_Delete: i32 = -801;
pub const DISPID_ENABLED: i32 = -514;
pub const DISPID_ENTERKEYBEHAVIOR: i32 = -544;
pub const DISPID_ERROREVENT: i32 = -608;
pub const DISPID_FILLCOLOR: i32 = -510;
pub const DISPID_FILLSTYLE: i32 = -511;
pub const DISPID_FONT: i32 = -512;
pub const DISPID_FONT_BOLD: i32 = 3;
pub const DISPID_FONT_CHANGED: i32 = 9;
pub const DISPID_FONT_CHARSET: i32 = 8;
pub const DISPID_FONT_ITALIC: i32 = 4;
pub const DISPID_FONT_NAME: i32 = 0;
pub const DISPID_FONT_SIZE: i32 = 2;
pub const DISPID_FONT_STRIKE: i32 = 6;
pub const DISPID_FONT_UNDER: i32 = 5;
pub const DISPID_FONT_WEIGHT: i32 = 7;
pub const DISPID_FORECOLOR: i32 = -513;
pub const DISPID_GROUPNAME: i32 = -541;
pub const DISPID_HWND: i32 = -515;
pub const DISPID_IMEMODE: i32 = -542;
pub const DISPID_KEYDOWN: i32 = -602;
pub const DISPID_KEYPRESS: i32 = -603;
pub const DISPID_KEYUP: i32 = -604;
pub const DISPID_LIST: i32 = -528;
pub const DISPID_LISTCOUNT: i32 = -531;
pub const DISPID_LISTINDEX: i32 = -526;
pub const DISPID_MAXLENGTH: i32 = -533;
pub const DISPID_MOUSEDOWN: i32 = -605;
pub const DISPID_MOUSEICON: i32 = -522;
pub const DISPID_MOUSEMOVE: i32 = -606;
pub const DISPID_MOUSEPOINTER: i32 = -521;
pub const DISPID_MOUSEUP: i32 = -607;
pub const DISPID_MULTILINE: i32 = -537;
pub const DISPID_MULTISELECT: i32 = -532;
pub const DISPID_NUMBEROFCOLUMNS: i32 = -539;
pub const DISPID_NUMBEROFROWS: i32 = -538;
pub const DISPID_Name: i32 = -800;
pub const DISPID_Object: i32 = -802;
pub const DISPID_PASSWORDCHAR: i32 = -534;
pub const DISPID_PICTURE: i32 = -523;
pub const DISPID_PICT_HANDLE: i32 = 0;
pub const DISPID_PICT_HEIGHT: i32 = 5;
pub const DISPID_PICT_HPAL: i32 = 2;
pub const DISPID_PICT_RENDER: i32 = 6;
pub const DISPID_PICT_TYPE: i32 = 3;
pub const DISPID_PICT_WIDTH: i32 = 4;
pub const DISPID_Parent: i32 = -803;
pub const DISPID_READYSTATE: i32 = -525;
pub const DISPID_READYSTATECHANGE: i32 = -609;
pub const DISPID_REFRESH: i32 = -550;
pub const DISPID_REMOVEITEM: i32 = -555;
pub const DISPID_RIGHTTOLEFT: i32 = -611;
pub const DISPID_SCROLLBARS: i32 = -535;
pub const DISPID_SELECTED: i32 = -527;
pub const DISPID_SELLENGTH: i32 = -548;
pub const DISPID_SELSTART: i32 = -547;
pub const DISPID_SELTEXT: i32 = -546;
pub const DISPID_TABKEYBEHAVIOR: i32 = -545;
pub const DISPID_TABSTOP: i32 = -516;
pub const DISPID_TEXT: i32 = -517;
pub const DISPID_THIS: i32 = -613;
pub const DISPID_TOPTOBOTTOM: i32 = -612;
pub const DISPID_VALID: i32 = -524;
pub const DISPID_WORDWRAP: i32 = -536;
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct FONTDESC {
    pub cbSizeofstruct: u32,
    pub lpstrName: super::LPOLESTR,
    pub cySize: super::CY,
    pub sWeight: i16,
    pub sCharset: i16,
    pub fItalic: windows_sys::core::BOOL,
    pub fUnderline: windows_sys::core::BOOL,
    pub fStrikethrough: windows_sys::core::BOOL,
}
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
impl Default for FONTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GUID_CHECKVALUEEXCLUSIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430c_be0f_101a_8bbb_00aa00300cab);
pub const GUID_COLOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504301_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTBOLD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430f_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTITALIC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504310_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430d_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTSIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430e_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTSTRIKETHROUGH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504312_be0f_101a_8bbb_00aa00300cab);
pub const GUID_FONTUNDERSCORE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504311_be0f_101a_8bbb_00aa00300cab);
pub const GUID_HANDLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504313_be0f_101a_8bbb_00aa00300cab);
pub const GUID_HIMETRIC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504300_be0f_101a_8bbb_00aa00300cab);
pub const GUID_OPTIONVALUEEXCLUSIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6650430b_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XPOS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504306_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XPOSPIXEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504302_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XSIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504308_be0f_101a_8bbb_00aa00300cab);
pub const GUID_XSIZEPIXEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504304_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YPOS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504307_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YPOSPIXEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504303_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YSIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504309_be0f_101a_8bbb_00aa00300cab);
pub const GUID_YSIZEPIXEL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66504305_be0f_101a_8bbb_00aa00300cab);
pub const IID_IPropertyFrame: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb196b28a_bab4_101a_b69c_00aa00341d07);
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
pub type LPFONTDESC = *mut FONTDESC;
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
pub type LPOCPFIPARAMS = *mut OCPFIPARAMS;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPPICTDESC = *mut PICTDESC;
pub const LP_COLOR: i32 = 4;
pub const LP_DEFAULT: i32 = 0;
pub const LP_MONOCHROME: i32 = 1;
pub const LP_VGACOLOR: i32 = 2;
pub const OCM_CHARTOITEM: i32 = 8239;
pub const OCM_COMMAND: i32 = 8465;
pub const OCM_COMPAREITEM: i32 = 8249;
pub const OCM_CTLCOLORBTN: i32 = 8501;
pub const OCM_CTLCOLORDLG: i32 = 8502;
pub const OCM_CTLCOLOREDIT: i32 = 8499;
pub const OCM_CTLCOLORLISTBOX: i32 = 8500;
pub const OCM_CTLCOLORMSGBOX: i32 = 8498;
pub const OCM_CTLCOLORSCROLLBAR: i32 = 8503;
pub const OCM_CTLCOLORSTATIC: i32 = 8504;
pub const OCM_DELETEITEM: i32 = 8237;
pub const OCM_DRAWITEM: i32 = 8235;
pub const OCM_HSCROLL: i32 = 8468;
pub const OCM_MEASUREITEM: i32 = 8236;
pub const OCM_NOTIFY: i32 = 8270;
pub const OCM_PARENTNOTIFY: i32 = 8720;
pub const OCM_VKEYTOITEM: i32 = 8238;
pub const OCM_VSCROLL: i32 = 8469;
pub const OCM__BASE: i32 = 8192;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct OCPFIPARAMS {
    pub cbStructSize: u32,
    pub hWndOwner: super::HWND,
    pub x: i32,
    pub y: i32,
    pub lpszCaption: super::LPCOLESTR,
    pub cObjects: u32,
    pub lplpUnk: *mut *mut core::ffi::c_void,
    pub cPages: u32,
    pub lpPages: *mut windows_sys::core::GUID,
    pub lcid: super::LCID,
    pub dispidInitialProperty: super::DISPID,
}
pub const OLEIVERB_PROPERTIES: i32 = -7;
pub const OLEMISC_ACTSLIKEBUTTON: i32 = 4096;
pub const OLEMISC_ACTSLIKELABEL: i32 = 8192;
pub const OLEMISC_ALIGNABLE: i32 = 32768;
pub const OLEMISC_ALWAYSRUN: i32 = 2048;
pub const OLEMISC_IMEMODE: i32 = 262144;
pub const OLEMISC_INVISIBLEATRUNTIME: i32 = 1024;
pub const OLEMISC_NOUIACTIVATE: i32 = 16384;
pub const OLEMISC_SETCLIENTSITEFIRST: i32 = 131072;
pub const OLEMISC_SIMPLEFRAME: i32 = 65536;
#[cfg(feature = "wtypes")]
pub type OLE_CANCELBOOL = super::VARIANT_BOOL;
#[cfg(feature = "wtypes")]
pub type OLE_ENABLEDEFAULTBOOL = super::VARIANT_BOOL;
#[cfg(feature = "wtypes")]
pub type OLE_OPTEXCLUSIVE = super::VARIANT_BOOL;
pub type OLE_TRISTATE = i32;
pub type OLE_XPOS_CONTAINER = f32;
pub type OLE_XPOS_PIXELS = i32;
pub type OLE_XSIZE_CONTAINER = f32;
pub type OLE_XSIZE_PIXELS = i32;
pub type OLE_YPOS_CONTAINER = f32;
pub type OLE_YPOS_PIXELS = i32;
pub type OLE_YSIZE_CONTAINER = f32;
pub type OLE_YSIZE_PIXELS = i32;
#[cfg(feature = "wtypesbase")]
pub const PERPROP_E_FIRST: super::SCODE = 0x80040200_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const PERPROP_E_LAST: super::SCODE = 0x8004020F_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const PERPROP_E_NOPAGEAVAILABLE: super::SCODE = 0x80040200_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const PERPROP_S_FIRST: super::SCODE = 0x40200_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const PERPROP_S_LAST: super::SCODE = 0x4020F_u32 as _;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct PICTDESC {
    pub cbSizeofstruct: u32,
    pub picType: u32,
    pub Anonymous: PICTDESC_0,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for PICTDESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub union PICTDESC_0 {
    pub bmp: PICTDESC_0_0,
    pub wmf: PICTDESC_0_1,
    pub icon: PICTDESC_0_2,
    pub emf: PICTDESC_0_3,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for PICTDESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PICTDESC_0_0 {
    pub hbitmap: super::HBITMAP,
    pub hpal: super::HPALETTE,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PICTDESC_0_1 {
    pub hmeta: super::HMETAFILE,
    pub xExt: i32,
    pub yExt: i32,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PICTDESC_0_2 {
    pub hicon: super::HICON,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct PICTDESC_0_3 {
    pub hemf: super::HENHMETAFILE,
}
pub const PICTYPE_BITMAP: i32 = 1;
pub const PICTYPE_ENHMETAFILE: i32 = 4;
pub const PICTYPE_ICON: i32 = 3;
pub const PICTYPE_METAFILE: i32 = 2;
pub const PICTYPE_NONE: i32 = 0;
pub const PICTYPE_UNINITIALIZED: i32 = -1;
#[cfg(feature = "wtypesbase")]
pub const SELFREG_E_CLASS: super::SCODE = 0x80040201_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const SELFREG_E_FIRST: super::SCODE = 0x80040200_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const SELFREG_E_LAST: super::SCODE = 0x8004020F_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const SELFREG_E_TYPELIB: super::SCODE = 0x80040200_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const SELFREG_S_FIRST: super::SCODE = 0x40200_u32 as _;
#[cfg(feature = "wtypesbase")]
pub const SELFREG_S_LAST: super::SCODE = 0x4020F_u32 as _;
pub const VT_BLOB_PROPSET: i32 = 75;
#[cfg(feature = "wtypes")]
pub const VT_COLOR: super::VARENUM = 3;
#[cfg(feature = "wtypes")]
pub const VT_FONT: super::VARENUM = 9;
#[cfg(feature = "wtypes")]
pub const VT_HANDLE: super::VARENUM = 3;
#[cfg(feature = "wtypes")]
pub const VT_OPTEXCLUSIVE: super::VARENUM = 11;
#[cfg(feature = "wtypes")]
pub const VT_PICTURE: super::VARENUM = 9;
pub const VT_STORED_PROPSET: i32 = 74;
pub const VT_STREAMED_PROPSET: i32 = 73;
#[cfg(feature = "wtypes")]
pub const VT_TRISTATE: super::VARENUM = 2;
pub const VT_VERBOSE_ENUM: i32 = 76;
#[cfg(feature = "wtypes")]
pub const VT_XPOS_HIMETRIC: super::VARENUM = 3;
#[cfg(feature = "wtypes")]
pub const VT_XPOS_PIXELS: super::VARENUM = 3;
#[cfg(feature = "wtypes")]
pub const VT_XSIZE_HIMETRIC: super::VARENUM = 3;
#[cfg(feature = "wtypes")]
pub const VT_XSIZE_PIXELS: super::VARENUM = 3;
#[cfg(feature = "wtypes")]
pub const VT_YPOS_HIMETRIC: super::VARENUM = 3;
#[cfg(feature = "wtypes")]
pub const VT_YPOS_PIXELS: super::VARENUM = 3;
#[cfg(feature = "wtypes")]
pub const VT_YSIZE_HIMETRIC: super::VARENUM = 3;
#[cfg(feature = "wtypes")]
pub const VT_YSIZE_PIXELS: super::VARENUM = 3;
pub const triChecked: OLE_TRISTATE = 1;
pub const triGray: OLE_TRISTATE = 2;
pub const triUnchecked: OLE_TRISTATE = 0;
