#[cfg(feature = "wtypes")]
windows_link::link!("oleaut32.dll" "system" fn OleCreateFontIndirect(lpfontdesc : *mut FONTDESC, riid : *const windows_sys::core::GUID, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("oleaut32.dll" "system" fn OleCreatePictureIndirect(lppictdesc : *mut PICTDESC, riid : *const windows_sys::core::GUID, fown : windows_sys::core::BOOL, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "windef", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn OleCreatePropertyFrame(hwndowner : super::windef::HWND, x : u32, y : u32, lpszcaption : windows_sys::core::PCWSTR, cobjects : u32, ppunk : *mut *mut core::ffi::c_void, cpages : u32, ppageclsid : *mut windows_sys::core::GUID, lcid : super::winnt::LCID, dwreserved : u32, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt"))]
windows_link::link!("oleaut32.dll" "system" fn OleCreatePropertyFrameIndirect(lpparams : *mut OCPFIPARAMS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "windef"))]
windows_link::link!("oleaut32.dll" "system" fn OleIconToCursor(hinstexe : super::minwindef::HINSTANCE, hicon : super::windef::HICON) -> super::windef::HCURSOR);
#[cfg(feature = "objidlbase")]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPicture(lpstream : *mut core::ffi::c_void, lsize : i32, frunmode : windows_sys::core::BOOL, riid : *const windows_sys::core::GUID, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPictureEx(lpstream : *mut core::ffi::c_void, lsize : i32, frunmode : windows_sys::core::BOOL, riid : *const windows_sys::core::GUID, xsizedesired : u32, ysizedesired : u32, dwflags : u32, lplpvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPictureFile(varfilename : super::oaidl::VARIANT, lplpdisppicture : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPictureFileEx(varfilename : super::oaidl::VARIANT, xsizedesired : u32, ysizedesired : u32, dwflags : u32, lplpdisppicture : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "ocidl")]
windows_link::link!("oleaut32.dll" "system" fn OleLoadPicturePath(szurlorpath : windows_sys::core::PCWSTR, punkcaller : *mut core::ffi::c_void, dwreserved : u32, clrreserved : super::ocidl::OLE_COLOR, riid : *const windows_sys::core::GUID, ppvret : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "oaidl")]
windows_link::link!("oleaut32.dll" "system" fn OleSavePictureFile(lpdisppicture : *mut core::ffi::c_void, bstrfilename : windows_sys::core::BSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "ocidl", feature = "windef"))]
windows_link::link!("oleaut32.dll" "system" fn OleTranslateColor(clr : super::ocidl::OLE_COLOR, hpal : super::windef::HPALETTE, lpcolorref : *mut super::windef::COLORREF) -> windows_sys::core::HRESULT);
pub const CLSID_CColorPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35201_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CFontPropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35200_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_CPicturePropPage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35202_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_ConvertVBX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb8f0822_0164_101b_84ed_08002b2ec713);
pub const CLSID_PersistPropset: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfb8f0821_0164_101b_84ed_08002b2ec713);
pub const CLSID_StdFont: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35203_8f91_11ce_9de3_00aa004bb851);
pub const CLSID_StdPicture: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0be35204_8f91_11ce_9de3_00aa004bb851);
pub const CONNECT_E_ADVISELIMIT: i32 = -2147220991;
pub const CONNECT_E_CANNOTCONNECT: i32 = -2147220990;
pub const CONNECT_E_FIRST: i32 = -2147220992;
pub const CONNECT_E_LAST: i32 = -2147220977;
pub const CONNECT_E_NOCONNECTION: i32 = -2147220992;
pub const CONNECT_E_OVERRIDDEN: i32 = -2147220989;
pub const CONNECT_S_FIRST: u32 = 262656;
pub const CONNECT_S_LAST: u32 = 262671;
pub const CTL_E_BADFILEMODE: i32 = -2146828234;
pub const CTL_E_BADFILENAME: i32 = -2146828224;
pub const CTL_E_BADFILENAMEORNUMBER: i32 = -2146828236;
pub const CTL_E_BADRECORDLENGTH: i32 = -2146828229;
pub const CTL_E_BADRECORDNUMBER: i32 = -2146828225;
pub const CTL_E_CANTSAVEFILETOTEMP: i32 = -2146827553;
pub const CTL_E_CUSTOM_FIRST: i32 = -2146827688;
pub const CTL_E_DEVICEIOERROR: i32 = -2146828231;
pub const CTL_E_DEVICEUNAVAILABLE: i32 = -2146828220;
pub const CTL_E_DISKFULL: i32 = -2146828227;
pub const CTL_E_DISKNOTREADY: i32 = -2146828217;
pub const CTL_E_DIVISIONBYZERO: i32 = -2146828277;
pub const CTL_E_FILEALREADYEXISTS: i32 = -2146828230;
pub const CTL_E_FILEALREADYOPEN: i32 = -2146828233;
pub const CTL_E_FILENOTFOUND: i32 = -2146828235;
pub const CTL_E_GETNOTSUPPORTED: i32 = -2146827894;
pub const CTL_E_GETNOTSUPPORTEDATRUNTIME: i32 = -2146827895;
pub const CTL_E_ILLEGALFUNCTIONCALL: i32 = -2146828283;
pub const CTL_E_INVALIDCLIPBOARDFORMAT: i32 = -2146827828;
pub const CTL_E_INVALIDFILEFORMAT: i32 = -2146827967;
pub const CTL_E_INVALIDPATTERNSTRING: i32 = -2146828195;
pub const CTL_E_INVALIDPICTURE: i32 = -2146827807;
pub const CTL_E_INVALIDPROPERTYARRAYINDEX: i32 = -2146827907;
pub const CTL_E_INVALIDPROPERTYVALUE: i32 = -2146827908;
pub const CTL_E_INVALIDUSEOFNULL: i32 = -2146828194;
pub const CTL_E_NEEDPROPERTYARRAYINDEX: i32 = -2146827903;
pub const CTL_E_OUTOFMEMORY: i32 = -2146828281;
pub const CTL_E_OUTOFSTACKSPACE: i32 = -2146828260;
pub const CTL_E_OUTOFSTRINGSPACE: i32 = -2146828274;
pub const CTL_E_OVERFLOW: i32 = -2146828282;
pub const CTL_E_PATHFILEACCESSERROR: i32 = -2146828213;
pub const CTL_E_PATHNOTFOUND: i32 = -2146828212;
pub const CTL_E_PERMISSIONDENIED: i32 = -2146828218;
pub const CTL_E_PRINTERERROR: i32 = -2146827806;
pub const CTL_E_PROPERTYNOTFOUND: i32 = -2146827866;
pub const CTL_E_REPLACEMENTSTOOLONG: i32 = -2146827542;
pub const CTL_E_SEARCHTEXTNOTFOUND: i32 = -2146827544;
pub const CTL_E_SETNOTPERMITTED: i32 = -2146827901;
pub const CTL_E_SETNOTSUPPORTED: i32 = -2146827905;
pub const CTL_E_SETNOTSUPPORTEDATRUNTIME: i32 = -2146827906;
pub const CTL_E_TOOMANYFILES: i32 = -2146828221;
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
pub const DISPID_FONT_BOLD: u32 = 3;
pub const DISPID_FONT_CHANGED: u32 = 9;
pub const DISPID_FONT_CHARSET: u32 = 8;
pub const DISPID_FONT_ITALIC: u32 = 4;
pub const DISPID_FONT_NAME: u32 = 0;
pub const DISPID_FONT_SIZE: u32 = 2;
pub const DISPID_FONT_STRIKE: u32 = 6;
pub const DISPID_FONT_UNDER: u32 = 5;
pub const DISPID_FONT_WEIGHT: u32 = 7;
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
pub const DISPID_PICT_HANDLE: u32 = 0;
pub const DISPID_PICT_HEIGHT: u32 = 5;
pub const DISPID_PICT_HPAL: u32 = 2;
pub const DISPID_PICT_RENDER: u32 = 6;
pub const DISPID_PICT_TYPE: u32 = 3;
pub const DISPID_PICT_WIDTH: u32 = 4;
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
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy)]
pub struct FONTDESC {
    pub cbSizeofstruct: u32,
    pub lpstrName: windows_sys::core::PWSTR,
    pub cySize: super::wtypes::CY,
    pub sWeight: i16,
    pub sCharset: i16,
    pub fItalic: windows_sys::core::BOOL,
    pub fUnderline: windows_sys::core::BOOL,
    pub fStrikethrough: windows_sys::core::BOOL,
}
#[cfg(feature = "wtypes")]
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
#[cfg(feature = "wtypes")]
pub type LPFONTDESC = *mut FONTDESC;
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt"))]
pub type LPOCPFIPARAMS = *mut OCPFIPARAMS;
#[cfg(all(feature = "minwindef", feature = "windef"))]
pub type LPPICTDESC = *mut PICTDESC;
pub const LP_COLOR: u32 = 4;
pub const LP_DEFAULT: u32 = 0;
pub const LP_MONOCHROME: u32 = 1;
pub const LP_VGACOLOR: u32 = 2;
pub const OCM_CHARTOITEM: u32 = 8239;
pub const OCM_COMMAND: u32 = 8465;
pub const OCM_COMPAREITEM: u32 = 8249;
pub const OCM_CTLCOLORBTN: u32 = 8501;
pub const OCM_CTLCOLORDLG: u32 = 8502;
pub const OCM_CTLCOLOREDIT: u32 = 8499;
pub const OCM_CTLCOLORLISTBOX: u32 = 8500;
pub const OCM_CTLCOLORMSGBOX: u32 = 8498;
pub const OCM_CTLCOLORSCROLLBAR: u32 = 8503;
pub const OCM_CTLCOLORSTATIC: u32 = 8504;
pub const OCM_DELETEITEM: u32 = 8237;
pub const OCM_DRAWITEM: u32 = 8235;
pub const OCM_HSCROLL: u32 = 8468;
pub const OCM_MEASUREITEM: u32 = 8236;
pub const OCM_NOTIFY: u32 = 8270;
pub const OCM_PARENTNOTIFY: u32 = 8720;
pub const OCM_VKEYTOITEM: u32 = 8238;
pub const OCM_VSCROLL: u32 = 8469;
pub const OCM__BASE: u32 = 8192;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct OCPFIPARAMS {
    pub cbStructSize: u32,
    pub hWndOwner: super::windef::HWND,
    pub x: i32,
    pub y: i32,
    pub lpszCaption: windows_sys::core::PCWSTR,
    pub cObjects: u32,
    pub lplpUnk: *mut *mut core::ffi::c_void,
    pub cPages: u32,
    pub lpPages: *mut windows_sys::core::GUID,
    pub lcid: super::winnt::LCID,
    pub dispidInitialProperty: super::oaidl::DISPID,
}
#[cfg(all(feature = "oaidl", feature = "windef", feature = "winnt"))]
impl Default for OCPFIPARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OLEIVERB_PROPERTIES: i32 = -7;
#[cfg(feature = "wtypes")]
pub type OLE_CANCELBOOL = super::wtypes::VARIANT_BOOL;
#[cfg(feature = "wtypes")]
pub type OLE_ENABLEDEFAULTBOOL = super::wtypes::VARIANT_BOOL;
#[cfg(feature = "wtypes")]
pub type OLE_OPTEXCLUSIVE = super::wtypes::VARIANT_BOOL;
pub type OLE_TRISTATE = i32;
pub type OLE_XPOS_PIXELS = i32;
pub type OLE_XSIZE_PIXELS = i32;
pub type OLE_YPOS_PIXELS = i32;
pub type OLE_YSIZE_PIXELS = i32;
pub const PERPROP_E_FIRST: i32 = -2147220992;
pub const PERPROP_E_LAST: i32 = -2147220977;
pub const PERPROP_E_NOPAGEAVAILABLE: i32 = -2147220992;
pub const PERPROP_S_FIRST: u32 = 262656;
pub const PERPROP_S_LAST: u32 = 262671;
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
#[derive(Clone, Copy)]
pub struct PICTDESC_0_0 {
    pub hbitmap: super::windef::HBITMAP,
    pub hpal: super::windef::HPALETTE,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for PICTDESC_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct PICTDESC_0_1 {
    pub hmeta: super::minwindef::HMETAFILE,
    pub xExt: i32,
    pub yExt: i32,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for PICTDESC_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct PICTDESC_0_2 {
    pub hicon: super::windef::HICON,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for PICTDESC_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct PICTDESC_0_3 {
    pub hemf: super::windef::HENHMETAFILE,
}
#[cfg(all(feature = "minwindef", feature = "windef"))]
impl Default for PICTDESC_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PICTYPE_BITMAP: u32 = 1;
pub const PICTYPE_ENHMETAFILE: u32 = 4;
pub const PICTYPE_ICON: u32 = 3;
pub const PICTYPE_METAFILE: u32 = 2;
pub const PICTYPE_NONE: u32 = 0;
pub const PICTYPE_UNINITIALIZED: i32 = -1;
pub const SELFREG_E_CLASS: i32 = -2147220991;
pub const SELFREG_E_FIRST: i32 = -2147220992;
pub const SELFREG_E_LAST: i32 = -2147220977;
pub const SELFREG_E_TYPELIB: i32 = -2147220992;
pub const SELFREG_S_FIRST: u32 = 262656;
pub const SELFREG_S_LAST: u32 = 262671;
pub const VT_BLOB_PROPSET: u32 = 75;
pub const VT_COLOR: u32 = 3;
pub const VT_FONT: u32 = 9;
pub const VT_HANDLE: u32 = 3;
pub const VT_OPTEXCLUSIVE: u32 = 11;
pub const VT_PICTURE: u32 = 9;
pub const VT_STORED_PROPSET: u32 = 74;
pub const VT_STREAMED_PROPSET: u32 = 73;
pub const VT_TRISTATE: u32 = 2;
pub const VT_VERBOSE_ENUM: u32 = 76;
pub const VT_XPOS_HIMETRIC: u32 = 3;
pub const VT_XPOS_PIXELS: u32 = 3;
pub const VT_XSIZE_HIMETRIC: u32 = 3;
pub const VT_XSIZE_PIXELS: u32 = 3;
pub const VT_YPOS_HIMETRIC: u32 = 3;
pub const VT_YPOS_PIXELS: u32 = 3;
pub const VT_YSIZE_HIMETRIC: u32 = 3;
pub const VT_YSIZE_PIXELS: u32 = 3;
pub const triChecked: OLE_TRISTATE = 1;
pub const triGray: OLE_TRISTATE = 2;
pub const triUnchecked: OLE_TRISTATE = 0;
