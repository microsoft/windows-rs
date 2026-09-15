#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HACCEL_UserFree(param0 : *mut u32, param1 : *mut super::HACCEL));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HACCEL_UserFree64(param0 : *mut u32, param1 : *mut super::HACCEL));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HACCEL_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HACCEL) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HACCEL_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HACCEL) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HACCEL_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut super::HACCEL) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HACCEL_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut super::HACCEL) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HACCEL_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HACCEL) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HACCEL_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HACCEL) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HPALETTE_UserFree(param0 : *mut u32, param1 : *mut super::HPALETTE));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HPALETTE_UserFree64(param0 : *mut u32, param1 : *mut super::HPALETTE));
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HPALETTE_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HPALETTE) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HPALETTE_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HPALETTE) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HPALETTE_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut super::HPALETTE) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HPALETTE_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut super::HPALETTE) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HPALETTE_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HPALETTE) -> *mut u8);
#[cfg(feature = "windef")]
windows_link::link!("ole32.dll" "system" fn HPALETTE_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HPALETTE) -> *mut u8);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn HRGN_UserFree(param0 : *mut u32, param1 : *mut super::HRGN));
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserFree64(param0 : *mut u32, param1 : *mut super::HRGN));
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn HRGN_UserMarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HRGN) -> *mut u8);
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserMarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HRGN) -> *mut u8);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn HRGN_UserSize(param0 : *mut u32, param1 : u32, param2 : *mut super::HRGN) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserSize64(param0 : *mut u32, param1 : u32, param2 : *mut super::HRGN) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("ole32.dll" "system" fn HRGN_UserUnmarshal(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HRGN) -> *mut u8);
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-marshal-l1-1-0.dll" "system" fn HRGN_UserUnmarshal64(param0 : *mut u32, param1 : *mut u8, param2 : *mut super::HRGN) -> *mut u8);
pub type ACTIVATEFLAGS = i32;
pub const ACTIVATE_WINDOWLESS: ACTIVATEFLAGS = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CADWORD {
    pub cElems: u32,
    pub pElems: *mut u32,
}
#[repr(C)]
#[cfg(feature = "wtypesbase")]
#[derive(Clone, Copy, Default)]
pub struct CALPOLESTR {
    pub cElems: u32,
    pub pElems: *mut super::LPOLESTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CAUUID {
    pub cElems: u32,
    pub pElems: *mut windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONNECTDATA {
    pub pUnk: *mut core::ffi::c_void,
    pub dwCookie: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct CONTROLINFO {
    pub cb: u32,
    pub hAccel: super::HACCEL,
    pub cAccel: u16,
    pub dwFlags: u32,
}
pub type CTRLINFO = i32;
pub const CTRLINFO_EATS_ESCAPE: CTRLINFO = 2;
pub const CTRLINFO_EATS_RETURN: CTRLINFO = 1;
pub type DVASPECT2 = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DVASPECTINFO {
    pub cb: u32,
    pub dwFlags: u32,
}
pub type DVASPECTINFOFLAG = i32;
pub const DVASPECTINFOFLAG_CANOPTIMIZE: DVASPECTINFOFLAG = 1;
pub const DVASPECT_OPAQUE: DVASPECT2 = 16;
pub const DVASPECT_TRANSPARENT: DVASPECT2 = 32;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DVEXTENTINFO {
    pub cb: u32,
    pub dwExtentMode: u32,
    pub sizelProposed: super::SIZEL,
}
pub type DVEXTENTMODE = i32;
pub const DVEXTENT_CONTENT: DVEXTENTMODE = 0;
pub const DVEXTENT_INTEGRAL: DVEXTENTMODE = 1;
pub type GUIDKIND = i32;
pub const GUIDKIND_DEFAULT_SOURCE_DISP_IID: GUIDKIND = 1;
pub type HHANDLE = usize;
pub type HITRESULT = i32;
pub const HITRESULT_CLOSE: HITRESULT = 2;
pub const HITRESULT_HIT: HITRESULT = 3;
pub const HITRESULT_OUTSIDE: HITRESULT = 0;
pub const HITRESULT_TRANSPARENT: HITRESULT = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LICINFO {
    pub cbLicInfo: i32,
    pub fRuntimeKeyAvail: windows_sys::core::BOOL,
    pub fLicVerified: windows_sys::core::BOOL,
}
#[cfg(feature = "objidl")]
pub type LPADVISESINKEX = *mut core::ffi::c_void;
pub type LPCADWORD = *mut CADWORD;
#[cfg(feature = "wtypesbase")]
pub type LPCALPOLESTR = *mut CALPOLESTR;
pub type LPCAUUID = *mut CAUUID;
#[cfg(feature = "unknwnbase")]
pub type LPCLASSFACTORY2 = *mut core::ffi::c_void;
pub type LPCONNECTDATA = *mut CONNECTDATA;
pub type LPCONNECTIONPOINT = *mut core::ffi::c_void;
pub type LPCONNECTIONPOINTCONTAINER = *mut core::ffi::c_void;
#[cfg(feature = "windef")]
pub type LPCONTROLINFO = *mut CONTROLINFO;
pub type LPENUMCONNECTIONPOINTS = *mut core::ffi::c_void;
pub type LPENUMCONNECTIONS = *mut core::ffi::c_void;
pub type LPENUMOLEUNDOUNITS = *mut core::ffi::c_void;
pub type LPFONT = *mut core::ffi::c_void;
#[cfg(feature = "oaidl")]
pub type LPFONTDISP = *mut core::ffi::c_void;
#[cfg(feature = "oaidl")]
pub type LPFONTEVENTS = *mut core::ffi::c_void;
pub type LPLICINFO = *mut LICINFO;
pub type LPOBJECTWITHSITE = *mut core::ffi::c_void;
pub type LPOLECONTROL = *mut core::ffi::c_void;
pub type LPOLECONTROLSITE = *mut core::ffi::c_void;
#[cfg(feature = "oleidl")]
pub type LPOLEINPLACEOBJECTWINDOWLESS = *mut core::ffi::c_void;
#[cfg(feature = "oleidl")]
pub type LPOLEINPLACESITEEX = *mut core::ffi::c_void;
#[cfg(feature = "oleidl")]
pub type LPOLEINPLACESITEWINDOWLESS = *mut core::ffi::c_void;
pub type LPOLEPARENTUNDOUNIT = *mut core::ffi::c_void;
pub type LPOLEUNDOMANAGER = *mut core::ffi::c_void;
pub type LPOLEUNDOUNIT = *mut core::ffi::c_void;
pub type LPPERPROPERTYBROWSING = *mut core::ffi::c_void;
#[cfg(feature = "objidl")]
pub type LPPERSISTMEMORY = *mut core::ffi::c_void;
#[cfg(feature = "objidl")]
pub type LPPERSISTPROPERTYBAG = *mut core::ffi::c_void;
#[cfg(feature = "objidl")]
pub type LPPERSISTPROPERTYBAG2 = *mut core::ffi::c_void;
#[cfg(feature = "objidl")]
pub type LPPERSISTSTREAMINIT = *mut core::ffi::c_void;
pub type LPPICTURE = *mut core::ffi::c_void;
pub type LPPICTURE2 = *mut core::ffi::c_void;
#[cfg(feature = "oaidl")]
pub type LPPICTUREDISP = *mut core::ffi::c_void;
pub type LPPOINTERINACTIVE = *mut core::ffi::c_void;
#[cfg(feature = "minwindef")]
pub type LPPOINTF = *mut POINTF;
pub type LPPROPERTYBAG2 = *mut core::ffi::c_void;
pub type LPPROPERTYNOTIFYSINK = *mut core::ffi::c_void;
pub type LPPROPERTYPAGE = *mut core::ffi::c_void;
pub type LPPROPERTYPAGE2 = *mut core::ffi::c_void;
pub type LPPROPERTYPAGESITE = *mut core::ffi::c_void;
#[cfg(all(feature = "windef", feature = "wtypesbase"))]
pub type LPPROPPAGEINFO = *mut PROPPAGEINFO;
pub type LPPROVIDECLASSINFO = *mut core::ffi::c_void;
pub type LPPROVIDECLASSINFO2 = *mut core::ffi::c_void;
pub type LPPROVIDEMULTIPLECLASSINFO = *mut core::ffi::c_void;
pub type LPQUICKACTIVATE = *mut core::ffi::c_void;
pub type LPSIMPLEFRAMESITE = *mut core::ffi::c_void;
pub type LPSPECIFYPROPERTYPAGES = *mut core::ffi::c_void;
#[cfg(feature = "wingdi")]
pub type LPTEXTMETRICOLE = *mut TEXTMETRICOLE;
#[cfg(feature = "oleidl")]
pub type LPVIEWOBJECTEX = *mut core::ffi::c_void;
pub const MULTICLASSINFO_GETIIDPRIMARY: i32 = 4;
pub const MULTICLASSINFO_GETIIDSOURCE: i32 = 8;
pub const MULTICLASSINFO_GETNUMRESERVEDDISPIDS: i32 = 2;
pub const MULTICLASSINFO_GETTYPEINFO: i32 = 1;
pub type OLEDCFLAGS = i32;
pub const OLEDC_NODRAW: OLEDCFLAGS = 1;
pub const OLEDC_OFFSCREEN: OLEDCFLAGS = 4;
pub const OLEDC_PAINTBKGND: OLEDCFLAGS = 2;
pub type OLE_COLOR = u32;
pub type OLE_HANDLE = u32;
pub type OLE_XPOS_HIMETRIC = i32;
pub type OLE_XSIZE_HIMETRIC = i32;
pub type OLE_YPOS_HIMETRIC = i32;
pub type OLE_YSIZE_HIMETRIC = i32;
pub type PCONNECTDATA = *mut CONNECTDATA;
pub type PCONNECTIONPOINT = *mut core::ffi::c_void;
pub type PCONNECTIONPOINTCONTAINER = *mut core::ffi::c_void;
pub type PENUMCONNECTIONPOINTS = *mut core::ffi::c_void;
pub type PENUMCONNECTIONS = *mut core::ffi::c_void;
pub type PICTUREATTRIBUTES = i32;
pub const PICTURE_SCALABLE: PICTUREATTRIBUTES = 1;
pub const PICTURE_TRANSPARENT: PICTUREATTRIBUTES = 2;
pub type POINTERINACTIVE = i32;
pub const POINTERINACTIVE_ACTIVATEONDRAG: POINTERINACTIVE = 4;
pub const POINTERINACTIVE_ACTIVATEONENTRY: POINTERINACTIVE = 1;
pub const POINTERINACTIVE_DEACTIVATEONLEAVE: POINTERINACTIVE = 2;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct POINTF {
    pub x: super::FLOAT,
    pub y: super::FLOAT,
}
#[repr(C)]
#[cfg(all(feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct PROPBAG2 {
    pub dwType: u32,
    pub vt: super::VARTYPE,
    pub cfType: super::CLIPFORMAT,
    pub dwHint: u32,
    pub pstrName: super::LPOLESTR,
    pub clsid: windows_sys::core::GUID,
}
pub type PROPBAG2_TYPE = i32;
pub const PROPBAG2_TYPE_DATA: PROPBAG2_TYPE = 1;
pub const PROPBAG2_TYPE_MONIKER: PROPBAG2_TYPE = 6;
pub const PROPBAG2_TYPE_OBJECT: PROPBAG2_TYPE = 3;
pub const PROPBAG2_TYPE_STORAGE: PROPBAG2_TYPE = 5;
pub const PROPBAG2_TYPE_STREAM: PROPBAG2_TYPE = 4;
pub const PROPBAG2_TYPE_UNDEFINED: PROPBAG2_TYPE = 0;
pub const PROPBAG2_TYPE_URL: PROPBAG2_TYPE = 2;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "wtypesbase"))]
#[derive(Clone, Copy, Default)]
pub struct PROPPAGEINFO {
    pub cb: u32,
    pub pszTitle: super::LPOLESTR,
    pub size: super::SIZE,
    pub pszDocString: super::LPOLESTR,
    pub pszHelpFile: super::LPOLESTR,
    pub dwHelpContext: u32,
}
pub type PROPPAGESTATUS = i32;
pub const PROPPAGESTATUS_CLEAN: PROPPAGESTATUS = 4;
pub const PROPPAGESTATUS_DIRTY: PROPPAGESTATUS = 1;
pub const PROPPAGESTATUS_VALIDATE: PROPPAGESTATUS = 2;
#[repr(C)]
#[cfg(all(feature = "objidl", feature = "oleidl", feature = "servprov", feature = "urlmon", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct QACONTAINER {
    pub cbSize: u32,
    pub pClientSite: *mut core::ffi::c_void,
    pub pAdviseSink: *mut core::ffi::c_void,
    pub pPropertyNotifySink: *mut core::ffi::c_void,
    pub pUnkEventSink: *mut core::ffi::c_void,
    pub dwAmbientFlags: u32,
    pub colorFore: OLE_COLOR,
    pub colorBack: OLE_COLOR,
    pub pFont: *mut core::ffi::c_void,
    pub pUndoMgr: *mut core::ffi::c_void,
    pub dwAppearance: u32,
    pub lcid: i32,
    pub hpal: super::HPALETTE,
    pub pBindHost: *mut core::ffi::c_void,
    pub pOleControlSite: *mut core::ffi::c_void,
    pub pServiceProvider: *mut core::ffi::c_void,
}
pub type QACONTAINERFLAGS = i32;
pub const QACONTAINER_AUTOCLIP: QACONTAINERFLAGS = 32;
pub const QACONTAINER_DISPLAYASDEFAULT: QACONTAINERFLAGS = 8;
pub const QACONTAINER_MESSAGEREFLECT: QACONTAINERFLAGS = 64;
pub const QACONTAINER_SHOWGRABHANDLES: QACONTAINERFLAGS = 2;
pub const QACONTAINER_SHOWHATCHING: QACONTAINERFLAGS = 1;
pub const QACONTAINER_SUPPORTSMNEMONICS: QACONTAINERFLAGS = 128;
pub const QACONTAINER_UIDEAD: QACONTAINERFLAGS = 16;
pub const QACONTAINER_USERMODE: QACONTAINERFLAGS = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct QACONTROL {
    pub cbSize: u32,
    pub dwMiscStatus: u32,
    pub dwViewStatus: u32,
    pub dwEventCookie: u32,
    pub dwPropNotifyCookie: u32,
    pub dwPointerActivationPolicy: u32,
}
pub type READYSTATE = i32;
pub const READYSTATE_COMPLETE: READYSTATE = 4;
pub const READYSTATE_INTERACTIVE: READYSTATE = 3;
pub const READYSTATE_LOADED: READYSTATE = 2;
pub const READYSTATE_LOADING: READYSTATE = 1;
pub const READYSTATE_UNINITIALIZED: READYSTATE = 0;
#[cfg(feature = "wingdi")]
pub type TEXTMETRICOLE = super::TEXTMETRICW;
pub const TIFLAGS_EXTENDDISPATCHONLY: i32 = 1;
pub type UASFLAGS = i32;
pub const UAS_BLOCKED: UASFLAGS = 1;
pub const UAS_MASK: UASFLAGS = 3;
pub const UAS_NOPARENTENABLE: UASFLAGS = 2;
pub const UAS_NORMAL: UASFLAGS = 0;
pub type VIEWSTATUS = i32;
pub const VIEWSTATUS_3DSURFACE: VIEWSTATUS = 32;
pub const VIEWSTATUS_DVASPECTOPAQUE: VIEWSTATUS = 4;
pub const VIEWSTATUS_DVASPECTTRANSPARENT: VIEWSTATUS = 8;
pub const VIEWSTATUS_OPAQUE: VIEWSTATUS = 1;
pub const VIEWSTATUS_SOLIDBKGND: VIEWSTATUS = 2;
pub const VIEWSTATUS_SURFACE: VIEWSTATUS = 16;
pub type XFORMCOORDS = i32;
pub const XFORMCOORDS_CONTAINERTOHIMETRIC: XFORMCOORDS = 8;
pub const XFORMCOORDS_EVENTCOMPAT: XFORMCOORDS = 16;
pub const XFORMCOORDS_HIMETRICTOCONTAINER: XFORMCOORDS = 4;
pub const XFORMCOORDS_POSITION: XFORMCOORDS = 1;
pub const XFORMCOORDS_SIZE: XFORMCOORDS = 2;
