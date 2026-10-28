#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkClone(pihl : *mut core::ffi::c_void, riid : *const super::IID, pihlsiteforclone : *mut core::ffi::c_void, dwsitedata : super::DWORD, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "guiddef")]
windows_link::link!("hlink.dll" "system" fn HlinkCreateBrowseContext(piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "windef"))]
windows_link::link!("hlink.dll" "system" fn HlinkCreateExtensionServices(pwzadditionalheaders : windows_sys::core::PCWSTR, phwnd : super::HWND, pszusername : windows_sys::core::PCWSTR, pszpassword : windows_sys::core::PCWSTR, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkCreateFromData(pidataobj : *mut core::ffi::c_void, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "objidl", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkCreateFromMoniker(pimktrgt : *mut core::ffi::c_void, pwzlocation : windows_sys::core::PCWSTR, pwzfriendlyname : windows_sys::core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkCreateFromString(pwztarget : windows_sys::core::PCWSTR, pwzlocation : windows_sys::core::PCWSTR, pwzfriendlyname : windows_sys::core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("hlink.dll" "system" fn HlinkCreateShortcut(grfhlshortcutf : super::DWORD, pihl : *mut core::ffi::c_void, pwzdir : windows_sys::core::PCWSTR, pwzfilename : windows_sys::core::PCWSTR, ppwzshortcutfile : *mut windows_sys::core::PWSTR, dwreserved : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkCreateShortcutFromMoniker(grfhlshortcutf : super::DWORD, pimktarget : *mut core::ffi::c_void, pwzlocation : windows_sys::core::PCWSTR, pwzdir : windows_sys::core::PCWSTR, pwzfilename : windows_sys::core::PCWSTR, ppwzshortcutfile : *mut windows_sys::core::PWSTR, dwreserved : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("hlink.dll" "system" fn HlinkCreateShortcutFromString(grfhlshortcutf : super::DWORD, pwztarget : windows_sys::core::PCWSTR, pwzlocation : windows_sys::core::PCWSTR, pwzdir : windows_sys::core::PCWSTR, pwzfilename : windows_sys::core::PCWSTR, ppwzshortcutfile : *mut windows_sys::core::PWSTR, dwreserved : super::DWORD) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("hlink.dll" "system" fn HlinkGetSpecialReference(ureference : super::ULONG, ppwzreference : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkGetValueFromParams(pwzparams : windows_sys::core::PCWSTR, pwzname : windows_sys::core::PCWSTR, ppwzvalue : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkIsShortcut(pwzfilename : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
windows_link::link!("hlink.dll" "system" fn HlinkNavigate(pihl : *mut core::ffi::c_void, pihlframe : *mut core::ffi::c_void, grfhlnf : super::DWORD, pbc : super::LPBC, pibsc : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon"))]
windows_link::link!("hlink.dll" "system" fn HlinkNavigateToStringReference(pwztarget : windows_sys::core::PCWSTR, pwzlocation : windows_sys::core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, pihlframe : *mut core::ffi::c_void, grfhlnf : super::DWORD, pibc : super::LPBC, pibsc : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkOnNavigate(pihlframe : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void, grfhlnf : super::DWORD, pimktarget : *mut core::ffi::c_void, pwzlocation : windows_sys::core::PCWSTR, pwzfriendlyname : windows_sys::core::PCWSTR, puhlid : *mut super::ULONG) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkOnRenameDocument(dwreserved : super::DWORD, pihlbc : *mut core::ffi::c_void, pimkold : *mut core::ffi::c_void, pimknew : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkParseDisplayName(pibc : super::LPBC, pwzdisplayname : windows_sys::core::PCWSTR, fnoforceabs : windows_sys::core::BOOL, pccheaten : *mut super::ULONG, ppimk : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkPreprocessMoniker(pibc : super::LPBC, pimkin : *mut core::ffi::c_void, ppimkout : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkQueryCreateFromData(pidataobj : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon", feature = "wtypes"))]
windows_link::link!("hlink.dll" "system" fn HlinkResolveMonikerForData(pimkreference : super::LPMONIKER, reserved : super::DWORD, pibc : super::LPBC, cfmtetc : super::ULONG, rgfmtetc : *mut super::FORMATETC, pibsc : *mut core::ffi::c_void, pimkbase : super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkResolveShortcut(pwzshortcutfilename : windows_sys::core::PCWSTR, pihlsite : *mut core::ffi::c_void, dwsitedata : super::DWORD, piunkouter : *mut core::ffi::c_void, riid : *const super::IID, ppvobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidl")]
windows_link::link!("hlink.dll" "system" fn HlinkResolveShortcutToMoniker(pwzshortcutfilename : windows_sys::core::PCWSTR, ppimktarget : *mut *mut core::ffi::c_void, ppwzlocation : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("hlink.dll" "system" fn HlinkResolveShortcutToString(pwzshortcutfilename : windows_sys::core::PCWSTR, ppwztarget : *mut windows_sys::core::PWSTR, ppwzlocation : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared", feature = "urlmon", feature = "wtypes"))]
windows_link::link!("hlink.dll" "system" fn HlinkResolveStringForData(pwzreference : windows_sys::core::PCWSTR, reserved : super::DWORD, pibc : super::LPBC, cfmtetc : super::ULONG, rgfmtetc : *mut super::FORMATETC, pibsc : *mut core::ffi::c_void, pimkbase : super::LPMONIKER) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("hlink.dll" "system" fn HlinkSetSpecialReference(ureference : super::ULONG, pwzreference : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("hlink.dll" "system" fn HlinkTranslateURL(pwzurl : windows_sys::core::PCWSTR, grfflags : super::DWORD, ppwztranslatedurl : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "objidl", feature = "shared"))]
windows_link::link!("hlink.dll" "system" fn HlinkUpdateStackItem(pihlframe : *mut core::ffi::c_void, pihlbc : *mut core::ffi::c_void, uhlid : super::ULONG, pimktrgt : *mut core::ffi::c_void, pwzlocation : windows_sys::core::PCWSTR, pwzfriendlyname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "objidlbase")]
windows_link::link!("hlink.dll" "system" fn OleSaveToStreamEx(piunk : *mut core::ffi::c_void, pistm : *mut core::ffi::c_void, fcleardirty : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
pub const CFSTR_HYPERLINK: windows_sys::core::PCSTR = windows_sys::core::s!("Hyperlink");
pub const HLBWIF_DOCWNDMAXIMIZED: i32 = 8;
pub const HLBWIF_FRAMEWNDMAXIMIZED: i32 = 4;
pub const HLBWIF_HASDOCWNDINFO: i32 = 2;
pub const HLBWIF_HASFRAMEWNDINFO: i32 = 1;
pub const HLBWIF_HASWEBTOOLBARINFO: i32 = 16;
pub const HLBWIF_WEBTOOLBARHIDDEN: i32 = 32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct HLBWINFO {
    pub cbSize: super::ULONG,
    pub grfHLBWIF: super::DWORD,
    pub rcFramePos: super::RECT,
    pub rcDocPos: super::RECT,
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
pub const HLINK_E_FIRST: windows_sys::core::HRESULT = 0x80040100_u32 as _;
pub const HLINK_S_DONTHIDE: windows_sys::core::HRESULT = 0x40100_u32 as _;
pub const HLINK_S_FIRST: windows_sys::core::HRESULT = 0x40100_u32 as _;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HLITEM {
    pub uHLID: super::ULONG,
    pub pwzFriendlyName: windows_sys::core::PWSTR,
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
#[cfg(all(feature = "shared", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct HLTBINFO {
    pub uDockType: super::ULONG,
    pub rcTbPos: super::RECT,
}
pub const HLTB_DOCKEDBOTTOM: i32 = 3;
pub const HLTB_DOCKEDLEFT: i32 = 0;
pub const HLTB_DOCKEDRIGHT: i32 = 2;
pub const HLTB_DOCKEDTOP: i32 = 1;
pub const HLTB_FLOATING: i32 = 4;
pub type HLTRANSLATEF = i32;
pub const HLTRANSLATEF_DEFAULT: HLTRANSLATEF = 0;
pub const HLTRANSLATEF_DONTAPPLYDEFAULTPREFIX: HLTRANSLATEF = 1;
pub type LPENUMHLITEM = *mut core::ffi::c_void;
pub type LPEXTENSIONSERVICES = *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "windef"))]
pub type LPHLBWINFO = *mut HLBWINFO;
pub type LPHLINK = *mut core::ffi::c_void;
pub type LPHLINKBROWSECONTEXT = *mut core::ffi::c_void;
pub type LPHLINKFRAME = *mut core::ffi::c_void;
pub type LPHLINKSITE = *mut core::ffi::c_void;
pub type LPHLINKTARGET = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type LPHLITEM = *mut HLITEM;
