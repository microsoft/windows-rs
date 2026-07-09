windows_link::link!("hlink.dll" "system" fn HlinkClone(pihl : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, pihlsiteforclone : *mut core::ffi::c_void, dwsitedata : u32, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkCreateBrowseContext(piunkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_windef")]
windows_link::link!("hlink.dll" "system" fn HlinkCreateExtensionServices(pwzadditionalheaders : windows_sys::core::PCWSTR, phwnd : super::windef::HWND, pszusername : windows_sys::core::PCWSTR, pszpassword : windows_sys::core::PCWSTR, piunkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkCreateFromData(pidataobj : *mut core::ffi::c_void, pihlsite : *mut core::ffi::c_void, dwsitedata : u32, piunkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkCreateFromMoniker(pimktrgt : *mut core::ffi::c_void, pwzlocation : windows_sys::core::PCWSTR, pwzfriendlyname : windows_sys::core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : u32, piunkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkCreateFromString(pwztarget : windows_sys::core::PCWSTR, pwzlocation : windows_sys::core::PCWSTR, pwzfriendlyname : windows_sys::core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : u32, piunkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkCreateShortcut(grfhlshortcutf : u32, pihl : *mut core::ffi::c_void, pwzdir : windows_sys::core::PCWSTR, pwzfilename : windows_sys::core::PCWSTR, ppwzshortcutfile : *mut windows_sys::core::PWSTR, dwreserved : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkCreateShortcutFromMoniker(grfhlshortcutf : u32, pimktarget : *mut core::ffi::c_void, pwzlocation : windows_sys::core::PCWSTR, pwzdir : windows_sys::core::PCWSTR, pwzfilename : windows_sys::core::PCWSTR, ppwzshortcutfile : *mut windows_sys::core::PWSTR, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkCreateShortcutFromString(grfhlshortcutf : u32, pwztarget : windows_sys::core::PCWSTR, pwzlocation : windows_sys::core::PCWSTR, pwzdir : windows_sys::core::PCWSTR, pwzfilename : windows_sys::core::PCWSTR, ppwzshortcutfile : *mut windows_sys::core::PWSTR, dwreserved : u32) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkGetSpecialReference(ureference : u32, ppwzreference : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkGetValueFromParams(pwzparams : windows_sys::core::PCWSTR, pwzname : windows_sys::core::PCWSTR, ppwzvalue : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkIsShortcut(pwzfilename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_objidl", feature = "Win32_urlmon"))]
windows_link::link!("hlink.dll" "system" fn HlinkNavigate(pihl : *mut core::ffi::c_void, pihlframe : *mut core::ffi::c_void, grfhlnf : u32, pbc : *mut core::ffi::c_void, pibsc : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_objidl", feature = "Win32_urlmon"))]
windows_link::link!("hlink.dll" "system" fn HlinkNavigateToStringReference(pwztarget : windows_sys::core::PCWSTR, pwzlocation : windows_sys::core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : u32, pihlframe : *mut core::ffi::c_void, grfhlnf : u32, pibc : *mut core::ffi::c_void, pibsc : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkOnNavigate(pihlframe : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void, grfhlnf : u32, pimktarget : *mut core::ffi::c_void, pwzlocation : windows_sys::core::PCWSTR, pwzfriendlyname : windows_sys::core::PCWSTR, puhlid : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkOnRenameDocument(dwreserved : u32, pihlbc : *mut core::ffi::c_void, pimkold : *mut core::ffi::c_void, pimknew : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkParseDisplayName(pibc : *mut core::ffi::c_void, pwzdisplayname : windows_sys::core::PCWSTR, fnoforceabs : windows_sys::core::BOOL, pccheaten : *mut u32, ppimk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkPreprocessMoniker(pibc : *mut core::ffi::c_void, pimkin : *mut core::ffi::c_void, ppimkout : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkQueryCreateFromData(pidataobj : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_objidl", feature = "Win32_urlmon", feature = "Win32_wtypes"))]
windows_link::link!("hlink.dll" "system" fn HlinkResolveMonikerForData(pimkreference : *mut core::ffi::c_void, reserved : u32, pibc : *mut core::ffi::c_void, cfmtetc : u32, rgfmtetc : *mut super::objidl::FORMATETC, pibsc : *mut core::ffi::c_void, pimkbase : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkResolveShortcut(pwzshortcutfilename : windows_sys::core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : u32, piunkouter : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkResolveShortcutToMoniker(pwzshortcutfilename : windows_sys::core::PCWSTR, ppimktarget : *mut *mut core::ffi::c_void, ppwzlocation : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkResolveShortcutToString(pwzshortcutfilename : windows_sys::core::PCWSTR, ppwztarget : *mut windows_sys::core::PWSTR, ppwzlocation : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_objidl", feature = "Win32_urlmon", feature = "Win32_wtypes"))]
windows_link::link!("hlink.dll" "system" fn HlinkResolveStringForData(pwzreference : windows_sys::core::PCWSTR, reserved : u32, pibc : *mut core::ffi::c_void, cfmtetc : u32, rgfmtetc : *mut super::objidl::FORMATETC, pibsc : *mut core::ffi::c_void, pimkbase : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkSetSpecialReference(ureference : u32, pwzreference : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkTranslateURL(pwzurl : windows_sys::core::PCWSTR, grfflags : u32, ppwztranslatedurl : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkUpdateStackItem(pihlframe : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void, uhlid : u32, pimktrgt : *mut core::ffi::c_void, pwzlocation : windows_sys::core::PCWSTR, pwzfriendlyname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_objidlbase")]
windows_link::link!("hlink.dll" "system" fn OleSaveToStreamEx(piunk : *mut core::ffi::c_void, pistm : *mut core::ffi::c_void, fcleardirty : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
pub const HLBWIF_DOCWNDMAXIMIZED: i32 = 8;
pub const HLBWIF_FRAMEWNDMAXIMIZED: i32 = 4;
pub const HLBWIF_HASDOCWNDINFO: i32 = 2;
pub const HLBWIF_HASFRAMEWNDINFO: i32 = 1;
pub const HLBWIF_HASWEBTOOLBARINFO: i32 = 16;
pub const HLBWIF_WEBTOOLBARHIDDEN: i32 = 32;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct HLBWINFO {
    pub cbSize: u32,
    pub grfHLBWIF: u32,
    pub rcFramePos: super::windef::RECT,
    pub rcDocPos: super::windef::RECT,
    pub hltbinfo: HLTBINFO,
}
pub type HLFNAMEF = i32;
pub const HLFNAMEF_DEFAULT: HLFNAMEF = 0;
pub const HLFNAMEF_TRYCACHE: HLFNAMEF = 1;
pub const HLFNAMEF_TRYFULLTARGET: HLFNAMEF = 4;
pub const HLFNAMEF_TRYPRETTYTARGET: HLFNAMEF = 2;
pub const HLFNAMEF_TRYWIN95SHORTCUT: HLFNAMEF = 8;
pub const HLID_CURRENT: i32 = -3;
pub const HLID_INVALID: i32 = 0;
pub const HLID_NEXT: i32 = -2;
pub const HLID_PREVIOUS: i32 = -1;
pub const HLID_STACKBOTTOM: i32 = -4;
pub const HLID_STACKTOP: i32 = -5;
pub type HLINKGETREF = i32;
pub const HLINKGETREF_ABSOLUTE: HLINKGETREF = 1;
pub const HLINKGETREF_DEFAULT: HLINKGETREF = 0;
pub const HLINKGETREF_RELATIVE: HLINKGETREF = 2;
pub type HLINKMISC = i32;
pub const HLINKMISC_RELATIVE: HLINKMISC = 1;
pub type HLINKSETF = i32;
pub const HLINKSETF_LOCATION: HLINKSETF = 2;
pub const HLINKSETF_TARGET: HLINKSETF = 1;
pub type HLINKWHICHMK = i32;
pub const HLINKWHICHMK_BASE: HLINKWHICHMK = 2;
pub const HLINKWHICHMK_CONTAINER: HLINKWHICHMK = 1;
pub const HLINK_E_FIRST: i32 = -2147221248;
pub const HLINK_S_DONTHIDE: u32 = 262400;
pub const HLINK_S_FIRST: u32 = 262400;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HLITEM {
    pub uHLID: u32,
    pub pwzFriendlyName: windows_sys::core::PWSTR,
}
impl Default for HLITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HLNF = i32;
pub const HLNF_CREATENOHISTORY: HLNF = 32;
pub const HLNF_INTERNALJUMP: HLNF = 1;
pub const HLNF_NAVIGATINGBACK: HLNF = 4;
pub const HLNF_NAVIGATINGFORWARD: HLNF = 8;
pub const HLNF_NAVIGATINGTOSTACKITEM: HLNF = 16;
pub const HLNF_OPENINNEWWINDOW: HLNF = 2;
pub const HLQF_ISCURRENT: i32 = 2;
pub const HLQF_ISVALID: i32 = 1;
pub type HLSHORTCUTF = i32;
pub const HLSHORTCUTF_DEFAULT: HLSHORTCUTF = 0;
pub const HLSHORTCUTF_DONTACTUALLYCREATE: HLSHORTCUTF = 1;
pub const HLSHORTCUTF_MAYUSEEXISTINGSHORTCUT: HLSHORTCUTF = 8;
pub const HLSHORTCUTF_USEFILENAMEFROMFRIENDLYNAME: HLSHORTCUTF = 2;
pub const HLSHORTCUTF_USEUNIQUEFILENAME: HLSHORTCUTF = 4;
pub type HLSR = i32;
pub const HLSR_HISTORYFOLDER: HLSR = 2;
pub const HLSR_HOME: HLSR = 0;
pub const HLSR_SEARCHPAGE: HLSR = 1;
#[repr(C)]
#[cfg(feature = "Win32_windef")]
#[derive(Clone, Copy, Default)]
pub struct HLTBINFO {
    pub uDockType: u32,
    pub rcTbPos: super::windef::RECT,
}
pub const HLTB_DOCKEDBOTTOM: i32 = 3;
pub const HLTB_DOCKEDLEFT: i32 = 0;
pub const HLTB_DOCKEDRIGHT: i32 = 2;
pub const HLTB_DOCKEDTOP: i32 = 1;
pub const HLTB_FLOATING: i32 = 4;
pub type HLTRANSLATEF = i32;
pub const HLTRANSLATEF_DEFAULT: HLTRANSLATEF = 0;
pub const HLTRANSLATEF_DONTAPPLYDEFAULTPREFIX: HLTRANSLATEF = 1;
#[cfg(feature = "Win32_windef")]
pub type LPHLBWINFO = *mut HLBWINFO;
pub type LPHLITEM = *mut HLITEM;
