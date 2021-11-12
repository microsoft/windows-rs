#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_UI_Shell_Common")]
pub mod Common;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub mod PropertiesSystem;
#[link(name = "windows")]
extern "system" {
    pub fn AssocCreate(clsid: ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocCreateForClasses(rgclasses: *const ASSOCIATIONELEMENT, cclasses: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn AssocGetDetailsOfPropKey(psf: IShellFolder, pidl: *const Common::ITEMIDLIST, pkey: *const PropertiesSystem::PROPERTYKEY, pv: *mut super::super::System::Com::VARIANT, pffoundpropkey: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn AssocGetPerceivedType(pszext: super::super::Foundation::PWSTR, ptype: *mut Common::PERCEIVED, pflag: *mut u32, ppsztype: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssocIsDangerous(pszassoc: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryKeyA(flags: u32, key: ASSOCKEY, pszassoc: super::super::Foundation::PSTR, pszextra: super::super::Foundation::PSTR, phkeyout: *mut super::super::System::Registry::HKEY) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryKeyW(flags: u32, key: ASSOCKEY, pszassoc: super::super::Foundation::PWSTR, pszextra: super::super::Foundation::PWSTR, phkeyout: *mut super::super::System::Registry::HKEY) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssocQueryStringA(flags: u32, str: ASSOCSTR, pszassoc: super::super::Foundation::PSTR, pszextra: super::super::Foundation::PSTR, pszout: super::super::Foundation::PSTR, pcchout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryStringByKeyA(flags: u32, str: ASSOCSTR, hkassoc: super::super::System::Registry::HKEY, pszextra: super::super::Foundation::PSTR, pszout: super::super::Foundation::PSTR, pcchout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryStringByKeyW(flags: u32, str: ASSOCSTR, hkassoc: super::super::System::Registry::HKEY, pszextra: super::super::Foundation::PWSTR, pszout: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssocQueryStringW(flags: u32, str: ASSOCSTR, pszassoc: super::super::Foundation::PWSTR, pszextra: super::super::Foundation::PWSTR, pszout: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
    pub fn CDefFolderMenu_Create2(pidlfolder: *const Common::ITEMIDLIST, hwnd: super::super::Foundation::HWND, cidl: u32, apidl: *const *const Common::ITEMIDLIST, psf: IShellFolder, pfn: LPFNDFMCALLBACK, nkeys: u32, ahkeys: *const super::super::System::Registry::HKEY, ppcm: *mut IContextMenu) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn CIDLData_CreateFromIDArray(pidlfolder: *const Common::ITEMIDLIST, cidl: u32, apidl: *const *const Common::ITEMIDLIST, ppdtobj: *mut super::super::System::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChrCmpIA(w1: u16, w2: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChrCmpIW(w1: u16, w2: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorAdjustLuma(clrrgb: u32, n: i32, fscale: super::super::Foundation::BOOL) -> u32;
    pub fn ColorHLSToRGB(whue: u16, wluminance: u16, wsaturation: u16) -> u32;
    pub fn ColorRGBToHLS(clrrgb: u32, pwhue: *mut u16, pwluminance: *mut u16, pwsaturation: *mut u16);
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommandLineToArgvW(lpcmdline: super::super::Foundation::PWSTR, pnumargs: *mut i32) -> *mut super::super::Foundation::PWSTR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ConnectToConnectionPoint(punk: ::windows_sys::core::IUnknown, riidevent: *const ::windows_sys::core::GUID, fconnect: super::super::Foundation::BOOL, punktarget: ::windows_sys::core::IUnknown, pdwcookie: *mut u32, ppcpout: *mut super::super::System::Com::IConnectionPoint) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateProfile(pszusersid: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszprofilepath: super::super::Foundation::PWSTR, cchprofilepath: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_AutoScroll(hwnd: super::super::Foundation::HWND, pad: *mut AUTO_SCROLL_DATA, pptnow: *const super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_DragEnterEx(hwndtarget: super::super::Foundation::HWND, ptstart: super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DAD_DragEnterEx2(hwndtarget: super::super::Foundation::HWND, ptstart: super::super::Foundation::POINT, pdtobject: super::super::System::Com::IDataObject) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_DragLeave() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_DragMove(pt: super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DAD_SetDragImage(him: super::Controls::HIMAGELIST, pptoffset: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_ShowDragImage(fshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefSubclassProc(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteProfileA(lpsidstring: super::super::Foundation::PSTR, lpprofilepath: super::super::Foundation::PSTR, lpcomputername: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteProfileW(lpsidstring: super::super::Foundation::PWSTR, lpprofilepath: super::super::Foundation::PWSTR, lpcomputername: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoEnvironmentSubstA(pszsrc: super::super::Foundation::PSTR, cchsrc: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoEnvironmentSubstW(pszsrc: super::super::Foundation::PWSTR, cchsrc: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragAcceptFiles(hwnd: super::super::Foundation::HWND, faccept: super::super::Foundation::BOOL);
    pub fn DragFinish(hdrop: HDROP);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragQueryFileA(hdrop: HDROP, ifile: u32, lpszfile: super::super::Foundation::PSTR, cch: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragQueryFileW(hdrop: HDROP, ifile: u32, lpszfile: super::super::Foundation::PWSTR, cch: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragQueryPoint(hdrop: HDROP, ppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    pub fn DriveType(idrive: i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn DuplicateIcon(hinst: super::super::Foundation::HINSTANCE, hicon: super::WindowsAndMessaging::HICON) -> super::WindowsAndMessaging::HICON;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconA(hinst: super::super::Foundation::HINSTANCE, psziconpath: super::super::Foundation::PSTR, piicon: *mut u16) -> super::WindowsAndMessaging::HICON;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconExA(hinst: super::super::Foundation::HINSTANCE, psziconpath: super::super::Foundation::PSTR, piiconindex: *mut u16, piiconid: *mut u16) -> super::WindowsAndMessaging::HICON;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconExW(hinst: super::super::Foundation::HINSTANCE, psziconpath: super::super::Foundation::PWSTR, piiconindex: *mut u16, piiconid: *mut u16) -> super::WindowsAndMessaging::HICON;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconW(hinst: super::super::Foundation::HINSTANCE, psziconpath: super::super::Foundation::PWSTR, piicon: *mut u16) -> super::WindowsAndMessaging::HICON;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconA(hinst: super::super::Foundation::HINSTANCE, pszexefilename: super::super::Foundation::PSTR, niconindex: u32) -> super::WindowsAndMessaging::HICON;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconExA(lpszfile: super::super::Foundation::PSTR, niconindex: i32, phiconlarge: *mut super::WindowsAndMessaging::HICON, phiconsmall: *mut super::WindowsAndMessaging::HICON, nicons: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconExW(lpszfile: super::super::Foundation::PWSTR, niconindex: i32, phiconlarge: *mut super::WindowsAndMessaging::HICON, phiconsmall: *mut super::WindowsAndMessaging::HICON, nicons: u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconW(hinst: super::super::Foundation::HINSTANCE, pszexefilename: super::super::Foundation::PWSTR, niconindex: u32) -> super::WindowsAndMessaging::HICON;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableA(lpfile: super::super::Foundation::PSTR, lpdirectory: super::super::Foundation::PSTR, lpresult: super::super::Foundation::PSTR) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableW(lpfile: super::super::Foundation::PWSTR, lpdirectory: super::super::Foundation::PWSTR, lpresult: super::super::Foundation::PWSTR) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAcceptLanguagesA(pszlanguages: super::super::Foundation::PSTR, pcchlanguages: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAcceptLanguagesW(pszlanguages: super::super::Foundation::PWSTR, pcchlanguages: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAllUsersProfileDirectoryA(lpprofiledir: super::super::Foundation::PSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAllUsersProfileDirectoryW(lpprofiledir: super::super::Foundation::PWSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentProcessExplicitAppUserModelID(appid: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultUserProfileDirectoryA(lpprofiledir: super::super::Foundation::PSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultUserProfileDirectoryW(lpprofiledir: super::super::Foundation::PWSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    pub fn GetDpiForShellUIComponent(param0: SHELL_UI_COMPONENT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileNameFromBrowse(hwnd: super::super::Foundation::HWND, pszfilepath: super::super::Foundation::PWSTR, cchfilepath: u32, pszworkingdir: super::super::Foundation::PWSTR, pszdefext: super::super::Foundation::PWSTR, pszfilters: super::super::Foundation::PWSTR, psztitle: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn GetMenuContextHelpId(param0: super::WindowsAndMessaging::HMENU) -> u32;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn GetMenuPosFromID(hmenu: super::WindowsAndMessaging::HMENU, id: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileType(dwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfilesDirectoryA(lpprofiledir: super::super::Foundation::PSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfilesDirectoryW(lpprofiledir: super::super::Foundation::PWSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn GetScaleFactorForDevice(devicetype: DISPLAY_DEVICE_TYPE) -> Common::DEVICE_SCALE_FACTOR;
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Shell_Common"))]
    pub fn GetScaleFactorForMonitor(hmon: super::super::Graphics::Gdi::HMONITOR, pscale: *mut Common::DEVICE_SCALE_FACTOR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserProfileDirectoryA(htoken: super::super::Foundation::HANDLE, lpprofiledir: super::super::Foundation::PSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserProfileDirectoryW(htoken: super::super::Foundation::HANDLE, lpprofiledir: super::super::Foundation::PWSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowContextHelpId(param0: super::super::Foundation::HWND) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowSubclass(hwnd: super::super::Foundation::HWND, pfnsubclass: SUBCLASSPROC, uidsubclass: usize, pdwrefdata: *mut usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserFree(param0: *const u32, param1: *const super::super::Graphics::Gdi::HMONITOR);
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserFree64(param0: *const u32, param1: *const super::super::Graphics::Gdi::HMONITOR);
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HMONITOR) -> *mut u8;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HMONITOR) -> *mut u8;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserSize(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HMONITOR) -> u32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HMONITOR) -> u32;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HMONITOR) -> *mut u8;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HMONITOR) -> *mut u8;
    pub fn HashData(pbdata: *const u8, cbdata: u32, pbhash: *mut u8, cbhash: u32) -> ::windows_sys::core::HRESULT;
    pub fn HlinkClone(pihl: IHlink, riid: *const ::windows_sys::core::GUID, pihlsiteforclone: IHlinkSite, dwsitedata: u32, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn HlinkCreateBrowseContext(piunkouter: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateExtensionServices(pwzadditionalheaders: super::super::Foundation::PWSTR, phwnd: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, piunkouter: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkCreateFromData(pidataobj: super::super::System::Com::IDataObject, pihlsite: IHlinkSite, dwsitedata: u32, piunkouter: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkCreateFromMoniker(pimktrgt: super::super::System::Com::IMoniker, pwzlocation: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR, pihlsite: IHlinkSite, dwsitedata: u32, piunkouter: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateFromString(pwztarget: super::super::Foundation::PWSTR, pwzlocation: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR, pihlsite: IHlinkSite, dwsitedata: u32, piunkouter: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateShortcut(grfhlshortcutf: u32, pihl: IHlink, pwzdir: super::super::Foundation::PWSTR, pwzfilename: super::super::Foundation::PWSTR, ppwzshortcutfile: *mut super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkCreateShortcutFromMoniker(grfhlshortcutf: u32, pimktarget: super::super::System::Com::IMoniker, pwzlocation: super::super::Foundation::PWSTR, pwzdir: super::super::Foundation::PWSTR, pwzfilename: super::super::Foundation::PWSTR, ppwzshortcutfile: *mut super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateShortcutFromString(grfhlshortcutf: u32, pwztarget: super::super::Foundation::PWSTR, pwzlocation: super::super::Foundation::PWSTR, pwzdir: super::super::Foundation::PWSTR, pwzfilename: super::super::Foundation::PWSTR, ppwzshortcutfile: *mut super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkGetSpecialReference(ureference: u32, ppwzreference: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkGetValueFromParams(pwzparams: super::super::Foundation::PWSTR, pwzname: super::super::Foundation::PWSTR, ppwzvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkIsShortcut(pwzfilename: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkNavigate(pihl: IHlink, pihlframe: IHlinkFrame, grfhlnf: u32, pbc: super::super::System::Com::IBindCtx, pibsc: super::super::System::Com::IBindStatusCallback, pihlbc: IHlinkBrowseContext) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkNavigateToStringReference(pwztarget: super::super::Foundation::PWSTR, pwzlocation: super::super::Foundation::PWSTR, pihlsite: IHlinkSite, dwsitedata: u32, pihlframe: IHlinkFrame, grfhlnf: u32, pibc: super::super::System::Com::IBindCtx, pibsc: super::super::System::Com::IBindStatusCallback, pihlbc: IHlinkBrowseContext) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkOnNavigate(pihlframe: IHlinkFrame, pihlbc: IHlinkBrowseContext, grfhlnf: u32, pimktarget: super::super::System::Com::IMoniker, pwzlocation: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR, puhlid: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkOnRenameDocument(dwreserved: u32, pihlbc: IHlinkBrowseContext, pimkold: super::super::System::Com::IMoniker, pimknew: super::super::System::Com::IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkParseDisplayName(pibc: super::super::System::Com::IBindCtx, pwzdisplayname: super::super::Foundation::PWSTR, fnoforceabs: super::super::Foundation::BOOL, pccheaten: *mut u32, ppimk: *mut super::super::System::Com::IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkPreprocessMoniker(pibc: super::super::System::Com::IBindCtx, pimkin: super::super::System::Com::IMoniker, ppimkout: *mut super::super::System::Com::IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkQueryCreateFromData(pidataobj: super::super::System::Com::IDataObject) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkResolveMonikerForData(pimkreference: super::super::System::Com::IMoniker, reserved: u32, pibc: super::super::System::Com::IBindCtx, cfmtetc: u32, rgfmtetc: *mut super::super::System::Com::FORMATETC, pibsc: super::super::System::Com::IBindStatusCallback, pimkbase: super::super::System::Com::IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkResolveShortcut(pwzshortcutfilename: super::super::Foundation::PWSTR, pihlsite: IHlinkSite, dwsitedata: u32, piunkouter: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkResolveShortcutToMoniker(pwzshortcutfilename: super::super::Foundation::PWSTR, ppimktarget: *mut super::super::System::Com::IMoniker, ppwzlocation: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkResolveShortcutToString(pwzshortcutfilename: super::super::Foundation::PWSTR, ppwztarget: *mut super::super::Foundation::PWSTR, ppwzlocation: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkResolveStringForData(pwzreference: super::super::Foundation::PWSTR, reserved: u32, pibc: super::super::System::Com::IBindCtx, cfmtetc: u32, rgfmtetc: *mut super::super::System::Com::FORMATETC, pibsc: super::super::System::Com::IBindStatusCallback, pimkbase: super::super::System::Com::IMoniker) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSetSpecialReference(ureference: u32, pwzreference: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkTranslateURL(pwzurl: super::super::Foundation::PWSTR, grfflags: u32, ppwztranslatedurl: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkUpdateStackItem(pihlframe: IHlinkFrame, pihlbc: IHlinkBrowseContext, uhlid: u32, pimktrgt: super::super::System::Com::IMoniker, pwzlocation: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILAppendID(pidl: *const Common::ITEMIDLIST, pmkid: *const Common::SHITEMID, fappend: super::super::Foundation::BOOL) -> *mut Common::ITEMIDLIST;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILClone(pidl: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILCloneFirst(pidl: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILCombine(pidl1: *const Common::ITEMIDLIST, pidl2: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILCreateFromPathA(pszpath: super::super::Foundation::PSTR) -> *mut Common::ITEMIDLIST;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILCreateFromPathW(pszpath: super::super::Foundation::PWSTR) -> *mut Common::ITEMIDLIST;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILFindChild(pidlparent: *const Common::ITEMIDLIST, pidlchild: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILFindLastID(pidl: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILFree(pidl: *const Common::ITEMIDLIST);
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILGetNext(pidl: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILGetSize(pidl: *const Common::ITEMIDLIST) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILIsEqual(pidl1: *const Common::ITEMIDLIST, pidl2: *const Common::ITEMIDLIST) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILIsParent(pidl1: *const Common::ITEMIDLIST, pidl2: *const Common::ITEMIDLIST, fimmediate: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn ILLoadFromStreamEx(pstm: super::super::System::Com::IStream, pidl: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILRemoveLastID(pidl: *mut Common::ITEMIDLIST) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn ILSaveToStream(pstm: super::super::System::Com::IStream, pidl: *const Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Copy(pstmfrom: super::super::System::Com::IStream, pstmto: super::super::System::Com::IStream, cb: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Read(pstm: super::super::System::Com::IStream, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn IStream_ReadPidl(pstm: super::super::System::Com::IStream, ppidlout: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn IStream_ReadStr(pstm: super::super::System::Com::IStream, ppsz: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Reset(pstm: super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Size(pstm: super::super::System::Com::IStream, pui: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Write(pstm: super::super::System::Com::IStream, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn IStream_WritePidl(pstm: super::super::System::Com::IStream, pidlwrite: *const Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn IStream_WriteStr(pstm: super::super::System::Com::IStream, psz: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn IUnknown_AtomicRelease(ppunk: *mut *mut ::core::ffi::c_void);
    pub fn IUnknown_GetSite(punk: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IUnknown_GetWindow(punk: ::windows_sys::core::IUnknown, phwnd: *mut super::super::Foundation::HWND) -> ::windows_sys::core::HRESULT;
    pub fn IUnknown_QueryService(punk: ::windows_sys::core::IUnknown, guidservice: *const ::windows_sys::core::GUID, riid: *const ::windows_sys::core::GUID, ppvout: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn IUnknown_Set(ppunk: *mut ::windows_sys::core::IUnknown, punk: ::windows_sys::core::IUnknown);
    pub fn IUnknown_SetSite(punk: ::windows_sys::core::IUnknown, punksite: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportPrivacySettings(pszfilename: super::super::Foundation::PWSTR, pfparseprivacypreferences: *mut super::super::Foundation::BOOL, pfparsepersiterules: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitNetworkAddressControl() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntlStrEqWorkerA(fcasesens: super::super::Foundation::BOOL, lpstring1: super::super::Foundation::PSTR, lpstring2: super::super::Foundation::PSTR, nchar: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntlStrEqWorkerW(fcasesens: super::super::Foundation::BOOL, lpstring1: super::super::Foundation::PWSTR, lpstring2: super::super::Foundation::PWSTR, nchar: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharSpaceA(wch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharSpaceW(wch: u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsInternetESCEnabled() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLFNDriveA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLFNDriveW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    pub fn IsNetDrive(idrive: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsOS(dwos: OS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUserAnAdmin() -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadUserProfileA(htoken: super::super::Foundation::HANDLE, lpprofileinfo: *mut PROFILEINFOA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadUserProfileW(htoken: super::super::Foundation::HANDLE, lpprofileinfo: *mut PROFILEINFOW) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleSaveToStreamEx(piunk: ::windows_sys::core::IUnknown, pistm: super::super::System::Com::IStream, fcleardirty: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn OpenRegStream(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseURLA(pcszurl: super::super::Foundation::PSTR, ppu: *mut PARSEDURLA) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseURLW(pcszurl: super::super::Foundation::PWSTR, ppu: *mut PARSEDURLW) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddBackslashA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddBackslashW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddExtensionA(pszpath: super::super::Foundation::PSTR, pszext: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddExtensionW(pszpath: super::super::Foundation::PWSTR, pszext: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAllocCanonicalize(pszpathin: super::super::Foundation::PWSTR, dwflags: u32, ppszpathout: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAllocCombine(pszpathin: super::super::Foundation::PWSTR, pszmore: super::super::Foundation::PWSTR, dwflags: u32, ppszpathout: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAppendA(pszpath: super::super::Foundation::PSTR, pszmore: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAppendW(pszpath: super::super::Foundation::PWSTR, pszmore: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathBuildRootA(pszroot: super::super::Foundation::PSTR, idrive: i32) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathBuildRootW(pszroot: super::super::Foundation::PWSTR, idrive: i32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCanonicalizeA(pszbuf: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCanonicalizeW(pszbuf: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAddBackslash(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAddBackslashEx(pszpath: super::super::Foundation::PWSTR, cchpath: usize, ppszend: *mut super::super::Foundation::PWSTR, pcchremaining: *mut usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAddExtension(pszpath: super::super::Foundation::PWSTR, cchpath: usize, pszext: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAppend(pszpath: super::super::Foundation::PWSTR, cchpath: usize, pszmore: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAppendEx(pszpath: super::super::Foundation::PWSTR, cchpath: usize, pszmore: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCanonicalize(pszpathout: super::super::Foundation::PWSTR, cchpathout: usize, pszpathin: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCanonicalizeEx(pszpathout: super::super::Foundation::PWSTR, cchpathout: usize, pszpathin: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCombine(pszpathout: super::super::Foundation::PWSTR, cchpathout: usize, pszpathin: super::super::Foundation::PWSTR, pszmore: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCombineEx(pszpathout: super::super::Foundation::PWSTR, cchpathout: usize, pszpathin: super::super::Foundation::PWSTR, pszmore: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchFindExtension(pszpath: super::super::Foundation::PWSTR, cchpath: usize, ppszext: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchIsRoot(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveBackslash(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveBackslashEx(pszpath: super::super::Foundation::PWSTR, cchpath: usize, ppszend: *mut super::super::Foundation::PWSTR, pcchremaining: *mut usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveExtension(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveFileSpec(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRenameExtension(pszpath: super::super::Foundation::PWSTR, cchpath: usize, pszext: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchSkipRoot(pszpath: super::super::Foundation::PWSTR, ppszrootend: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchStripPrefix(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchStripToRoot(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCleanupSpec(pszdir: super::super::Foundation::PWSTR, pszspec: super::super::Foundation::PWSTR) -> PCS_RET;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCombineA(pszdest: super::super::Foundation::PSTR, pszdir: super::super::Foundation::PSTR, pszfile: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCombineW(pszdest: super::super::Foundation::PWSTR, pszdir: super::super::Foundation::PWSTR, pszfile: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCommonPrefixA(pszfile1: super::super::Foundation::PSTR, pszfile2: super::super::Foundation::PSTR, achpath: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCommonPrefixW(pszfile1: super::super::Foundation::PWSTR, pszfile2: super::super::Foundation::PWSTR, achpath: super::super::Foundation::PWSTR) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PathCompactPathA(hdc: super::super::Graphics::Gdi::HDC, pszpath: super::super::Foundation::PSTR, dx: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCompactPathExA(pszout: super::super::Foundation::PSTR, pszsrc: super::super::Foundation::PSTR, cchmax: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCompactPathExW(pszout: super::super::Foundation::PWSTR, pszsrc: super::super::Foundation::PWSTR, cchmax: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PathCompactPathW(hdc: super::super::Graphics::Gdi::HDC, pszpath: super::super::Foundation::PWSTR, dx: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCreateFromUrlA(pszurl: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR, pcchpath: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCreateFromUrlAlloc(pszin: super::super::Foundation::PWSTR, ppszout: *mut super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCreateFromUrlW(pszurl: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR, pcchpath: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFileExistsA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFileExistsW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindExtensionA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindExtensionW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindFileNameA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindFileNameW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindNextComponentA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindNextComponentW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindOnPathA(pszpath: super::super::Foundation::PSTR, ppszotherdirs: *const *const i8) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindOnPathW(pszpath: super::super::Foundation::PWSTR, ppszotherdirs: *const *const u16) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindSuffixArrayA(pszpath: super::super::Foundation::PSTR, apszsuffix: *const super::super::Foundation::PSTR, iarraysize: i32) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindSuffixArrayW(pszpath: super::super::Foundation::PWSTR, apszsuffix: *const super::super::Foundation::PWSTR, iarraysize: i32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetArgsA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetArgsW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    pub fn PathGetCharTypeA(ch: u8) -> u32;
    pub fn PathGetCharTypeW(ch: u16) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetDriveNumberA(pszpath: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetDriveNumberW(pszpath: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetShortPath(pszlongpath: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsContentTypeA(pszpath: super::super::Foundation::PSTR, pszcontenttype: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsContentTypeW(pszpath: super::super::Foundation::PWSTR, pszcontenttype: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryEmptyA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryEmptyW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsExe(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsFileSpecA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsFileSpecW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsLFNFileSpecA(pszname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsLFNFileSpecW(pszname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsNetworkPathA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsNetworkPathW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsPrefixA(pszprefix: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsPrefixW(pszprefix: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRelativeA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRelativeW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRootA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRootW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSameRootA(pszpath1: super::super::Foundation::PSTR, pszpath2: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSameRootW(pszpath1: super::super::Foundation::PWSTR, pszpath2: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSlowA(pszfile: super::super::Foundation::PSTR, dwattr: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSlowW(pszfile: super::super::Foundation::PWSTR, dwattr: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSystemFolderA(pszpath: super::super::Foundation::PSTR, dwattrb: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSystemFolderW(pszpath: super::super::Foundation::PWSTR, dwattrb: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCEx(pszpath: super::super::Foundation::PWSTR, ppszserver: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerShareA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerShareW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsURLA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsURLW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakePrettyA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakePrettyW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakeSystemFolderA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakeSystemFolderW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakeUniqueName(pszuniquename: super::super::Foundation::PWSTR, cchmax: u32, psztemplate: super::super::Foundation::PWSTR, pszlongplate: super::super::Foundation::PWSTR, pszdir: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecA(pszfile: super::super::Foundation::PSTR, pszspec: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecExA(pszfile: super::super::Foundation::PSTR, pszspec: super::super::Foundation::PSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecExW(pszfile: super::super::Foundation::PWSTR, pszspec: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecW(pszfile: super::super::Foundation::PWSTR, pszspec: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathParseIconLocationA(psziconfile: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathParseIconLocationW(psziconfile: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathQualify(psz: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathQuoteSpacesA(lpsz: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathQuoteSpacesW(lpsz: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRelativePathToA(pszpath: super::super::Foundation::PSTR, pszfrom: super::super::Foundation::PSTR, dwattrfrom: u32, pszto: super::super::Foundation::PSTR, dwattrto: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRelativePathToW(pszpath: super::super::Foundation::PWSTR, pszfrom: super::super::Foundation::PWSTR, dwattrfrom: u32, pszto: super::super::Foundation::PWSTR, dwattrto: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveArgsA(pszpath: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveArgsW(pszpath: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBackslashA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBackslashW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBlanksA(pszpath: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBlanksW(pszpath: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveExtensionA(pszpath: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveExtensionW(pszpath: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveFileSpecA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveFileSpecW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRenameExtensionA(pszpath: super::super::Foundation::PSTR, pszext: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRenameExtensionW(pszpath: super::super::Foundation::PWSTR, pszext: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathResolve(pszpath: super::super::Foundation::PWSTR, dirs: *const *const u16, fflags: PRF_FLAGS) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSearchAndQualifyA(pszpath: super::super::Foundation::PSTR, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSearchAndQualifyW(pszpath: super::super::Foundation::PWSTR, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSetDlgItemPathA(hdlg: super::super::Foundation::HWND, id: i32, pszpath: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSetDlgItemPathW(hdlg: super::super::Foundation::HWND, id: i32, pszpath: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSkipRootA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSkipRootW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripPathA(pszpath: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripPathW(pszpath: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripToRootA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripToRootW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnExpandEnvStringsA(pszpath: super::super::Foundation::PSTR, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnExpandEnvStringsW(pszpath: super::super::Foundation::PWSTR, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUndecorateA(pszpath: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUndecorateW(pszpath: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnmakeSystemFolderA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnmakeSystemFolderW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnquoteSpacesA(lpsz: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnquoteSpacesW(lpsz: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathYetAnotherMakeUniqueName(pszuniquename: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR, pszshort: super::super::Foundation::PWSTR, pszfilespec: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PickIconDlg(hwnd: super::super::Foundation::HWND, psziconpath: super::super::Foundation::PWSTR, cchiconpath: u32, piiconindex: *mut i32) -> i32;
    pub fn QISearch(that: *mut ::core::ffi::c_void, pqit: *const QITAB, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadCabinetState(pcs: *mut CABINETSTATE, clength: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RealDriveType(idrive: i32, foktohitnet: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppConstrainedChangeNotification(routine: PAPPCONSTRAIN_CHANGE_ROUTINE, context: *const ::core::ffi::c_void, registration: *mut *mut _APPCONSTRAIN_REGISTRATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppStateChangeNotification(routine: PAPPSTATE_CHANGE_ROUTINE, context: *const ::core::ffi::c_void, registration: *mut *mut _APPSTATE_REGISTRATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterScaleChangeEvent(hevent: super::super::Foundation::HANDLE, pdwcookie: *mut usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterScaleChangeNotifications(displaydevice: DISPLAY_DEVICE_TYPE, hwndnotify: super::super::Foundation::HWND, umsgnotify: u32, pdwcookie: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveWindowSubclass(hwnd: super::super::Foundation::HWND, pfnsubclass: SUBCLASSPROC, uidsubclass: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestartDialog(hwnd: super::super::Foundation::HWND, pszprompt: super::super::Foundation::PWSTR, dwreturn: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestartDialogEx(hwnd: super::super::Foundation::HWND, pszprompt: super::super::Foundation::PWSTR, dwreturn: u32, dwreasoncode: u32) -> i32;
    pub fn RevokeScaleChangeNotifications(displaydevice: DISPLAY_DEVICE_TYPE, dwcookie: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SHAddFromPropSheetExtArray(hpsxa: HPSXA, lpfnaddpage: super::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> u32;
    pub fn SHAddToRecentDocs(uflags: u32, pv: *const ::core::ffi::c_void);
    pub fn SHAlloc(cb: usize) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAllocShared(pvdata: *const ::core::ffi::c_void, dwsize: u32, dwprocessid: u32) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAnsiToAnsi(pszsrc: super::super::Foundation::PSTR, pszdst: super::super::Foundation::PSTR, cchbuf: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAnsiToUnicode(pszsrc: super::super::Foundation::PSTR, pwszdst: super::super::Foundation::PWSTR, cwchbuf: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAppBarMessage(dwmessage: u32, pdata: *mut APPBARDATA) -> usize;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAssocEnumHandlers(pszextra: super::super::Foundation::PWSTR, affilter: ASSOC_FILTER, ppenumhandler: *mut IEnumAssocHandlers) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAssocEnumHandlersForProtocolByApplication(protocol: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, enumhandlers: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAutoComplete(hwndedit: super::super::Foundation::HWND, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHBindToFolderIDListParent(psfroot: IShellFolder, pidl: *const Common::ITEMIDLIST, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void, ppidllast: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBindToFolderIDListParentEx(psfroot: IShellFolder, pidl: *const Common::ITEMIDLIST, ppbc: super::super::System::Com::IBindCtx, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void, ppidllast: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBindToObject(psf: IShellFolder, pidl: *const Common::ITEMIDLIST, pbc: super::super::System::Com::IBindCtx, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHBindToParent(pidl: *const Common::ITEMIDLIST, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void, ppidllast: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBrowseForFolderA(lpbi: *const BROWSEINFOA) -> *mut Common::ITEMIDLIST;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBrowseForFolderW(lpbi: *const BROWSEINFOW) -> *mut Common::ITEMIDLIST;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCLSIDFromString(psz: super::super::Foundation::PWSTR, pclsid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHChangeNotification_Lock(hchange: super::super::Foundation::HANDLE, dwprocid: u32, pppidl: *mut *mut *mut Common::ITEMIDLIST, plevent: *mut i32) -> ShFindChangeNotificationHandle;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHChangeNotification_Unlock(hlock: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    pub fn SHChangeNotify(weventid: SHCNE_ID, uflags: SHCNF_FLAGS, dwitem1: *const ::core::ffi::c_void, dwitem2: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHChangeNotifyDeregister(ulid: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHChangeNotifyRegister(hwnd: super::super::Foundation::HWND, fsources: SHCNRF_SOURCE, fevents: i32, wmsg: u32, centries: i32, pshcne: *const SHChangeNotifyEntry) -> u32;
    pub fn SHChangeNotifyRegisterThread(status: SCNRT_STATUS);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCloneSpecialIDList(hwnd: super::super::Foundation::HWND, csidl: i32, fcreate: super::super::Foundation::BOOL) -> *mut Common::ITEMIDLIST;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCoCreateInstance(pszclsid: super::super::Foundation::PWSTR, pclsid: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHCopyKeyA(hkeysrc: super::super::System::Registry::HKEY, pszsrcsubkey: super::super::Foundation::PSTR, hkeydest: super::super::System::Registry::HKEY, freserved: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHCopyKeyW(hkeysrc: super::super::System::Registry::HKEY, pszsrcsubkey: super::super::Foundation::PWSTR, hkeydest: super::super::System::Registry::HKEY, freserved: u32) -> super::super::Foundation::LSTATUS;
    pub fn SHCreateAssociationRegistration(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCreateDataObject(pidlfolder: *const Common::ITEMIDLIST, cidl: u32, apidl: *const *const Common::ITEMIDLIST, pdtinner: super::super::System::Com::IDataObject, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCreateDefaultContextMenu(pdcm: *const DEFCONTEXTMENU, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn SHCreateDefaultExtractIcon(riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn SHCreateDefaultPropertiesOp(psi: IShellItem, ppfileop: *mut IFileOperation) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCreateDirectory(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PWSTR) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SHCreateDirectoryExA(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PSTR, psa: *const super::super::Security::SECURITY_ATTRIBUTES) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SHCreateDirectoryExW(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PWSTR, psa: *const super::super::Security::SECURITY_ATTRIBUTES) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCreateFileExtractIconW(pszfile: super::super::Foundation::PWSTR, dwfileattributes: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateItemFromIDList(pidl: *const Common::ITEMIDLIST, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateItemFromParsingName(pszpath: super::super::Foundation::PWSTR, pbc: super::super::System::Com::IBindCtx, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateItemFromRelativeName(psiparent: IShellItem, pszname: super::super::Foundation::PWSTR, pbc: super::super::System::Com::IBindCtx, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCreateItemInKnownFolder(kfid: *const ::windows_sys::core::GUID, dwkfflags: u32, pszitem: super::super::Foundation::PWSTR, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateItemWithParent(pidlparent: *const Common::ITEMIDLIST, psfparent: IShellFolder, pidl: *const Common::ITEMIDLIST, riid: *const ::windows_sys::core::GUID, ppvitem: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateMemStream(pinit: *const u8, cbinit: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Threading"))]
    pub fn SHCreateProcessAsUserW(pscpi: *mut SHCREATEPROCESSINFOW) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHCreatePropSheetExtArray(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, max_iface: u32) -> HPSXA;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateQueryCancelAutoPlayMoniker(ppmoniker: *mut super::super::System::Com::IMoniker) -> ::windows_sys::core::HRESULT;
    pub fn SHCreateShellFolderView(pcsfv: *const SFV_CREATE, ppsv: *mut IShellView) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCreateShellFolderViewEx(pcsfv: *const CSFV, ppsv: *mut IShellView) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateShellItem(pidlparent: *const Common::ITEMIDLIST, psfparent: IShellFolder, pidl: *const Common::ITEMIDLIST, ppsi: *mut IShellItem) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateShellItemArray(pidlparent: *const Common::ITEMIDLIST, psf: IShellFolder, cidl: u32, ppidl: *const *const Common::ITEMIDLIST, ppsiitemarray: *mut IShellItemArray) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateShellItemArrayFromDataObject(pdo: super::super::System::Com::IDataObject, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateShellItemArrayFromIDLists(cidl: u32, rgpidl: *const *const Common::ITEMIDLIST, ppsiitemarray: *mut IShellItemArray) -> ::windows_sys::core::HRESULT;
    pub fn SHCreateShellItemArrayFromShellItem(psi: IShellItem, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SHCreateShellPalette(hdc: super::super::Graphics::Gdi::HDC) -> super::super::Graphics::Gdi::HPALETTE;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateStdEnumFmtEtc(cfmt: u32, afmt: *const super::super::System::Com::FORMATETC, ppenumformatetc: *mut super::super::System::Com::IEnumFORMATETC) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateStreamOnFileA(pszfile: super::super::Foundation::PSTR, grfmode: u32, ppstm: *mut super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateStreamOnFileEx(pszfile: super::super::Foundation::PWSTR, grfmode: u32, dwattributes: u32, fcreate: super::super::Foundation::BOOL, pstmtemplate: super::super::System::Com::IStream, ppstm: *mut super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateStreamOnFileW(pszfile: super::super::Foundation::PWSTR, grfmode: u32, ppstm: *mut super::super::System::Com::IStream) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn SHCreateThread(pfnthreadproc: super::super::System::Threading::LPTHREAD_START_ROUTINE, pdata: *const ::core::ffi::c_void, flags: u32, pfncallback: super::super::System::Threading::LPTHREAD_START_ROUTINE) -> super::super::Foundation::BOOL;
    pub fn SHCreateThreadRef(pcref: *mut i32, ppunk: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn SHCreateThreadWithHandle(pfnthreadproc: super::super::System::Threading::LPTHREAD_START_ROUTINE, pdata: *const ::core::ffi::c_void, flags: u32, pfncallback: super::super::System::Threading::LPTHREAD_START_ROUTINE, phandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHDefExtractIconA(psziconfile: super::super::Foundation::PSTR, iindex: i32, uflags: u32, phiconlarge: *mut super::WindowsAndMessaging::HICON, phiconsmall: *mut super::WindowsAndMessaging::HICON, niconsize: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHDefExtractIconW(psziconfile: super::super::Foundation::PWSTR, iindex: i32, uflags: u32, phiconlarge: *mut super::WindowsAndMessaging::HICON, phiconsmall: *mut super::WindowsAndMessaging::HICON, niconsize: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteEmptyKeyA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteEmptyKeyW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteKeyA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteKeyW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteValueA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteValueW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    pub fn SHDestroyPropSheetExtArray(hpsxa: HPSXA);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn SHDoDragDrop(hwnd: super::super::Foundation::HWND, pdata: super::super::System::Com::IDataObject, pdsrc: super::super::System::Ole::IDropSource, dweffect: u32, pdweffect: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHEmptyRecycleBinA(hwnd: super::super::Foundation::HWND, pszrootpath: super::super::Foundation::PSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHEmptyRecycleBinW(hwnd: super::super::Foundation::HWND, pszrootpath: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumKeyExA(hkey: super::super::System::Registry::HKEY, dwindex: u32, pszname: super::super::Foundation::PSTR, pcchname: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumKeyExW(hkey: super::super::System::Registry::HKEY, dwindex: u32, pszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumValueA(hkey: super::super::System::Registry::HKEY, dwindex: u32, pszvaluename: super::super::Foundation::PSTR, pcchvaluename: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumValueW(hkey: super::super::System::Registry::HKEY, dwindex: u32, pszvaluename: super::super::Foundation::PWSTR, pcchvaluename: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumerateUnreadMailAccountsW(hkeyuser: super::super::System::Registry::HKEY, dwindex: u32, pszmailaddress: super::super::Foundation::PWSTR, cchmailaddress: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHEvaluateSystemCommandTemplate(pszcmdtemplate: super::super::Foundation::PWSTR, ppszapplication: *mut super::super::Foundation::PWSTR, ppszcommandline: *mut super::super::Foundation::PWSTR, ppszparameters: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFileOperationA(lpfileop: *mut SHFILEOPSTRUCTA) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFileOperationW(lpfileop: *mut SHFILEOPSTRUCTW) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHFindFiles(pidlfolder: *const Common::ITEMIDLIST, pidlsavefile: *const Common::ITEMIDLIST) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHFind_InitMenuPopup(hmenu: super::WindowsAndMessaging::HMENU, hwndowner: super::super::Foundation::HWND, idcmdfirst: u32, idcmdlast: u32) -> ::core::option::Option<IContextMenu>;
    pub fn SHFlushSFCache();
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFormatDateTimeA(pft: *const super::super::Foundation::FILETIME, pdwflags: *mut u32, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFormatDateTimeW(pft: *const super::super::Foundation::FILETIME, pdwflags: *mut u32, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFormatDrive(hwnd: super::super::Foundation::HWND, drive: u32, fmtid: SHFMT_ID, options: SHFMT_OPT) -> u32;
    pub fn SHFree(pv: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFreeNameMappings(hnamemappings: super::super::Foundation::HANDLE);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFreeShared(hdata: super::super::Foundation::HANDLE, dwprocessid: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHGetAttributesFromDataObject(pdo: super::super::System::Com::IDataObject, dwattributemask: u32, pdwattributes: *mut u32, pcitems: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetDataFromIDListA(psf: IShellFolder, pidl: *const Common::ITEMIDLIST, nformat: SHGDFIL_FORMAT, pv: *mut ::core::ffi::c_void, cb: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetDataFromIDListW(psf: IShellFolder, pidl: *const Common::ITEMIDLIST, nformat: SHGDFIL_FORMAT, pv: *mut ::core::ffi::c_void, cb: i32) -> ::windows_sys::core::HRESULT;
    pub fn SHGetDesktopFolder(ppshf: *mut IShellFolder) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetDiskFreeSpaceExA(pszdirectoryname: super::super::Foundation::PSTR, pulfreebytesavailabletocaller: *mut u64, pultotalnumberofbytes: *mut u64, pultotalnumberoffreebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetDiskFreeSpaceExW(pszdirectoryname: super::super::Foundation::PWSTR, pulfreebytesavailabletocaller: *mut u64, pultotalnumberofbytes: *mut u64, pultotalnumberoffreebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetDriveMedia(pszdrive: super::super::Foundation::PWSTR, pdwmediacontent: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHGetFileInfoA(pszpath: super::super::Foundation::PSTR, dwfileattributes: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, psfi: *mut SHFILEINFOA, cbfileinfo: u32, uflags: SHGFI_FLAGS) -> usize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHGetFileInfoW(pszpath: super::super::Foundation::PWSTR, dwfileattributes: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, psfi: *mut SHFILEINFOW, cbfileinfo: u32, uflags: SHGFI_FLAGS) -> usize;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetFolderLocation(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, ppidl: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathA(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszpath: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathAndSubDirA(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszsubdir: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathAndSubDirW(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszsubdir: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathW(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetIDListFromObject(punk: ::windows_sys::core::IUnknown, ppidl: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetIconOverlayIndexA(psziconpath: super::super::Foundation::PSTR, iiconindex: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetIconOverlayIndexW(psziconpath: super::super::Foundation::PWSTR, iiconindex: i32) -> i32;
    pub fn SHGetImageList(iimagelist: i32, riid: *const ::windows_sys::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn SHGetInstanceExplorer(ppunk: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    pub fn SHGetInverseCMAP(pbmap: *mut u8, cbmap: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHGetItemFromDataObject(pdtobj: super::super::System::Com::IDataObject, dwflags: DATAOBJ_GET_ITEM_FLAGS, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn SHGetItemFromObject(punk: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetKnownFolderIDList(rfid: *const ::windows_sys::core::GUID, dwflags: u32, htoken: super::super::Foundation::HANDLE, ppidl: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetKnownFolderItem(rfid: *const ::windows_sys::core::GUID, flags: KNOWN_FOLDER_FLAG, htoken: super::super::Foundation::HANDLE, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetKnownFolderPath(rfid: *const ::windows_sys::core::GUID, dwflags: u32, htoken: super::super::Foundation::HANDLE, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetLocalizedName(pszpath: super::super::Foundation::PWSTR, pszresmodule: super::super::Foundation::PWSTR, cch: u32, pidsres: *mut i32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHGetMalloc(ppmalloc: *mut super::super::System::Com::IMalloc) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetNameFromIDList(pidl: *const Common::ITEMIDLIST, sigdnname: SIGDN, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetNewLinkInfoA(pszlinkto: super::super::Foundation::PSTR, pszdir: super::super::Foundation::PSTR, pszname: super::super::Foundation::PSTR, pfmustcopy: *mut super::super::Foundation::BOOL, uflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetNewLinkInfoW(pszlinkto: super::super::Foundation::PWSTR, pszdir: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, pfmustcopy: *mut super::super::Foundation::BOOL, uflags: u32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetPathFromIDListA(pidl: *const Common::ITEMIDLIST, pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetPathFromIDListEx(pidl: *const Common::ITEMIDLIST, pszpath: super::super::Foundation::PWSTR, cchpath: u32, uopts: i32) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetPathFromIDListW(pidl: *const Common::ITEMIDLIST, pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetRealIDL(psf: IShellFolder, pidlsimple: *const Common::ITEMIDLIST, ppidlreal: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSetFolderCustomSettings(pfcs: *mut SHFOLDERCUSTOMSETTINGS, pszpath: super::super::Foundation::PWSTR, dwreadwrite: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSetSettings(lpss: *mut SHELLSTATEA, dwmask: SSF_MASK, bset: super::super::Foundation::BOOL);
    pub fn SHGetSettings(psfs: *mut SHELLFLAGSTATE, dwmask: u32);
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetSpecialFolderLocation(hwnd: super::super::Foundation::HWND, csidl: i32, ppidl: *mut *mut Common::ITEMIDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSpecialFolderPathA(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PSTR, csidl: i32, fcreate: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSpecialFolderPathW(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PWSTR, csidl: i32, fcreate: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn SHGetStockIconInfo(siid: SHSTOCKICONID, uflags: u32, psii: *mut SHSTOCKICONINFO) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn SHGetTemporaryPropertyForItem(psi: IShellItem, propkey: *const PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    pub fn SHGetThreadRef(ppunk: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHGetUnreadMailCountW(hkeyuser: super::super::System::Registry::HKEY, pszmailaddress: super::super::Foundation::PWSTR, pdwcount: *mut u32, pfiletime: *mut super::super::Foundation::FILETIME, pszshellexecutecommand: super::super::Foundation::PWSTR, cchshellexecutecommand: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHGetValueA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHGetValueW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetViewStatePropertyBag(pidl: *const Common::ITEMIDLIST, pszbagname: super::super::Foundation::PWSTR, dwflags: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn SHGlobalCounterDecrement(id: SHGLOBALCOUNTER) -> i32;
    pub fn SHGlobalCounterGetValue(id: SHGLOBALCOUNTER) -> i32;
    pub fn SHGlobalCounterIncrement(id: SHGLOBALCOUNTER) -> i32;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHHandleUpdateImage(pidlextra: *const Common::ITEMIDLIST) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHILCreateFromPath(pszpath: super::super::Foundation::PWSTR, ppidl: *mut *mut Common::ITEMIDLIST, rgfinout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHInvokePrinterCommandA(hwnd: super::super::Foundation::HWND, uaction: u32, lpbuf1: super::super::Foundation::PSTR, lpbuf2: super::super::Foundation::PSTR, fmodal: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHInvokePrinterCommandW(hwnd: super::super::Foundation::HWND, uaction: u32, lpbuf1: super::super::Foundation::PWSTR, lpbuf2: super::super::Foundation::PWSTR, fmodal: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHIsFileAvailableOffline(pwszpath: super::super::Foundation::PWSTR, pdwstatus: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHIsLowMemoryMachine(dwtype: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHLimitInputEdit(hwndedit: super::super::Foundation::HWND, psf: IShellFolder) -> ::windows_sys::core::HRESULT;
    pub fn SHLoadInProc(rclsid: *const ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHLoadIndirectString(pszsource: super::super::Foundation::PWSTR, pszoutbuf: super::super::Foundation::PWSTR, cchoutbuf: u32, ppvreserved: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn SHLoadNonloadedIconOverlayIdentifiers() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHLockShared(hdata: super::super::Foundation::HANDLE, dwprocessid: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHMapPIDLToSystemImageListIndex(pshf: IShellFolder, pidl: *const Common::ITEMIDLIST, piindexsel: *mut i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHMessageBoxCheckA(hwnd: super::super::Foundation::HWND, psztext: super::super::Foundation::PSTR, pszcaption: super::super::Foundation::PSTR, utype: u32, idefault: i32, pszregval: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHMessageBoxCheckW(hwnd: super::super::Foundation::HWND, psztext: super::super::Foundation::PWSTR, pszcaption: super::super::Foundation::PWSTR, utype: u32, idefault: i32, pszregval: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHMultiFileProperties(pdtobj: super::super::System::Com::IDataObject, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHObjectProperties(hwnd: super::super::Foundation::HWND, shopobjecttype: SHOP_TYPE, pszobjectname: super::super::Foundation::PWSTR, pszpropertypage: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHOpenFolderAndSelectItems(pidlfolder: *const Common::ITEMIDLIST, cidl: u32, apidl: *const *const Common::ITEMIDLIST, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenPropSheetW(pszcaption: super::super::Foundation::PWSTR, ahkeys: *const super::super::System::Registry::HKEY, ckeys: u32, pclsiddefault: *const ::windows_sys::core::GUID, pdtobj: super::super::System::Com::IDataObject, psb: IShellBrowser, pstartpage: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStream2A(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStream2W(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStreamA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStreamW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHOpenWithDialog(hwndparent: super::super::Foundation::HWND, poainfo: *const OPENASINFO) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHParseDisplayName(pszname: super::super::Foundation::PWSTR, pbc: super::super::System::Com::IBindCtx, ppidl: *mut *mut Common::ITEMIDLIST, sfgaoin: u32, psfgaoout: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHPathPrepareForWriteA(hwnd: super::super::Foundation::HWND, punkenablemodless: ::windows_sys::core::IUnknown, pszpath: super::super::Foundation::PSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHPathPrepareForWriteW(hwnd: super::super::Foundation::HWND, punkenablemodless: ::windows_sys::core::IUnknown, pszpath: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryInfoKeyA(hkey: super::super::System::Registry::HKEY, pcsubkeys: *mut u32, pcchmaxsubkeylen: *mut u32, pcvalues: *mut u32, pcchmaxvaluenamelen: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryInfoKeyW(hkey: super::super::System::Registry::HKEY, pcsubkeys: *mut u32, pcchmaxsubkeylen: *mut u32, pcvalues: *mut u32, pcchmaxvaluenamelen: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHQueryRecycleBinA(pszrootpath: super::super::Foundation::PSTR, pshqueryrbinfo: *mut SHQUERYRBINFO) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHQueryRecycleBinW(pszrootpath: super::super::Foundation::PWSTR, pshqueryrbinfo: *mut SHQUERYRBINFO) -> ::windows_sys::core::HRESULT;
    pub fn SHQueryUserNotificationState(pquns: *mut QUERY_USER_NOTIFICATION_STATE) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryValueExA(hkey: super::super::System::Registry::HKEY, pszvalue: super::super::Foundation::PSTR, pdwreserved: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryValueExW(hkey: super::super::System::Registry::HKEY, pszvalue: super::super::Foundation::PWSTR, pdwreserved: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegCloseUSKey(huskey: isize) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegCreateUSKeyA(pszpath: super::super::Foundation::PSTR, samdesired: u32, hrelativeuskey: isize, phnewuskey: *mut isize, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegCreateUSKeyW(pwzpath: super::super::Foundation::PWSTR, samdesired: u32, hrelativeuskey: isize, phnewuskey: *mut isize, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteEmptyUSKeyA(huskey: isize, pszsubkey: super::super::Foundation::PSTR, delregflags: SHREGDEL_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteEmptyUSKeyW(huskey: isize, pwzsubkey: super::super::Foundation::PWSTR, delregflags: SHREGDEL_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteUSValueA(huskey: isize, pszvalue: super::super::Foundation::PSTR, delregflags: SHREGDEL_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteUSValueW(huskey: isize, pwzvalue: super::super::Foundation::PWSTR, delregflags: SHREGDEL_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SHRegDuplicateHKey(hkey: super::super::System::Registry::HKEY) -> super::super::System::Registry::HKEY;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSKeyA(huskey: isize, dwindex: u32, pszname: super::super::Foundation::PSTR, pcchname: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSKeyW(huskey: isize, dwindex: u32, pwzname: super::super::Foundation::PWSTR, pcchname: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSValueA(huskey: isize, dwindex: u32, pszvaluename: super::super::Foundation::PSTR, pcchvaluename: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSValueW(huskey: isize, dwindex: u32, pszvaluename: super::super::Foundation::PWSTR, pcchvaluename: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetBoolUSValueA(pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, fignorehkcu: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetBoolUSValueW(pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, fignorehkcu: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetIntW(hk: super::super::System::Registry::HKEY, pwzkey: super::super::Foundation::PWSTR, idefault: i32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetPathA(hkey: super::super::System::Registry::HKEY, pcszsubkey: super::super::Foundation::PSTR, pcszvalue: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetPathW(hkey: super::super::System::Registry::HKEY, pcszsubkey: super::super::Foundation::PWSTR, pcszvalue: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetUSValueA(pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, fignorehkcu: super::super::Foundation::BOOL, pvdefaultdata: *const ::core::ffi::c_void, dwdefaultdatasize: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetUSValueW(pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, fignorehkcu: super::super::Foundation::BOOL, pvdefaultdata: *const ::core::ffi::c_void, dwdefaultdatasize: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetValueA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, srrfflags: i32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetValueFromHKCUHKLM(pwszkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR, srrfflags: i32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetValueW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, srrfflags: i32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegOpenUSKeyA(pszpath: super::super::Foundation::PSTR, samdesired: u32, hrelativeuskey: isize, phnewuskey: *mut isize, fignorehkcu: super::super::Foundation::BOOL) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegOpenUSKeyW(pwzpath: super::super::Foundation::PWSTR, samdesired: u32, hrelativeuskey: isize, phnewuskey: *mut isize, fignorehkcu: super::super::Foundation::BOOL) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryInfoUSKeyA(huskey: isize, pcsubkeys: *mut u32, pcchmaxsubkeylen: *mut u32, pcvalues: *mut u32, pcchmaxvaluenamelen: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryInfoUSKeyW(huskey: isize, pcsubkeys: *mut u32, pcchmaxsubkeylen: *mut u32, pcvalues: *mut u32, pcchmaxvaluenamelen: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryUSValueA(huskey: isize, pszvalue: super::super::Foundation::PSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, fignorehkcu: super::super::Foundation::BOOL, pvdefaultdata: *const ::core::ffi::c_void, dwdefaultdatasize: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryUSValueW(huskey: isize, pszvalue: super::super::Foundation::PWSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, fignorehkcu: super::super::Foundation::BOOL, pvdefaultdata: *const ::core::ffi::c_void, dwdefaultdatasize: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegSetPathA(hkey: super::super::System::Registry::HKEY, pcszsubkey: super::super::Foundation::PSTR, pcszvalue: super::super::Foundation::PSTR, pcszpath: super::super::Foundation::PSTR, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegSetPathW(hkey: super::super::System::Registry::HKEY, pcszsubkey: super::super::Foundation::PWSTR, pcszvalue: super::super::Foundation::PWSTR, pcszpath: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegSetUSValueA(pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegSetUSValueW(pwzsubkey: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegWriteUSValueA(huskey: isize, pszvalue: super::super::Foundation::PSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegWriteUSValueW(huskey: isize, pwzvalue: super::super::Foundation::PWSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::super::Foundation::LSTATUS;
    pub fn SHReleaseThreadRef() -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRemoveLocalizedName(pszpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SHReplaceFromPropSheetExtArray(hpsxa: HPSXA, upageid: u32, lpfnreplacewith: super::Controls::LPFNSVADDPROPSHEETPAGE, lparam: super::super::Foundation::LPARAM) -> u32;
    pub fn SHResolveLibrary(psilibrary: IShellItem) -> ::windows_sys::core::HRESULT;
    pub fn SHRestricted(rest: RESTRICTIONS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSendMessageBroadcastA(umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSendMessageBroadcastW(umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetDefaultProperties(hwnd: super::super::Foundation::HWND, psi: IShellItem, dwfileopflags: u32, pfops: IFileOperationProgressSink) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetFolderPathA(csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszpath: super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetFolderPathW(csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn SHSetInstanceExplorer(punk: ::windows_sys::core::IUnknown);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetKnownFolderPath(rfid: *const ::windows_sys::core::GUID, dwflags: u32, htoken: super::super::Foundation::HANDLE, pszpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetLocalizedName(pszpath: super::super::Foundation::PWSTR, pszresmodule: super::super::Foundation::PWSTR, idsres: i32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn SHSetTemporaryPropertyForItem(psi: IShellItem, propkey: *const PropertiesSystem::PROPERTYKEY, propvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_sys::core::HRESULT;
    pub fn SHSetThreadRef(punk: ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetUnreadMailCountW(pszmailaddress: super::super::Foundation::PWSTR, dwcount: u32, pszshellexecutecommand: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHSetValueA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHSetValueW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHShellFolderView_Message(hwndmain: super::super::Foundation::HWND, umsg: u32, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHShowManageLibraryUI(psilibrary: IShellItem, hwndowner: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pszinstruction: super::super::Foundation::PWSTR, lmdoptions: LIBRARYMANAGEDIALOGOPTIONS) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHSimpleIDListFromPath(pszpath: super::super::Foundation::PWSTR) -> *mut Common::ITEMIDLIST;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHSkipJunction(pbc: super::super::System::Com::IBindCtx, pclsid: *const ::windows_sys::core::GUID) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStartNetConnectionDialogW(hwnd: super::super::Foundation::HWND, pszremotename: super::super::Foundation::PWSTR, dwtype: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStrDupA(psz: super::super::Foundation::PSTR, ppwsz: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStrDupW(psz: super::super::Foundation::PWSTR, ppwsz: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStripMneumonicA(pszmenu: super::super::Foundation::PSTR) -> super::super::Foundation::CHAR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStripMneumonicW(pszmenu: super::super::Foundation::PWSTR) -> u16;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHTestTokenMembership(htoken: super::super::Foundation::HANDLE, ulrid: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUnicodeToAnsi(pwszsrc: super::super::Foundation::PWSTR, pszdst: super::super::Foundation::PSTR, cchbuf: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUnicodeToUnicode(pwzsrc: super::super::Foundation::PWSTR, pwzdst: super::super::Foundation::PWSTR, cwchbuf: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUnlockShared(pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUpdateImageA(pszhashitem: super::super::Foundation::PSTR, iindex: i32, uflags: u32, iimageindex: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUpdateImageW(pszhashitem: super::super::Foundation::PWSTR, iindex: i32, uflags: u32, iimageindex: i32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHValidateUNC(hwndowner: super::super::Foundation::HWND, pszfile: super::super::Foundation::PWSTR, fconnect: VALIDATEUNC_OPTION) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentProcessExplicitAppUserModelID(appid: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetMenuContextHelpId(param0: super::WindowsAndMessaging::HMENU, param1: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowContextHelpId(param0: super::super::Foundation::HWND, param1: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowSubclass(hwnd: super::super::Foundation::HWND, pfnsubclass: SUBCLASSPROC, uidsubclass: usize, dwrefdata: usize) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ShellAboutA(hwnd: super::super::Foundation::HWND, szapp: super::super::Foundation::PSTR, szotherstuff: super::super::Foundation::PSTR, hicon: super::WindowsAndMessaging::HICON) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ShellAboutW(hwnd: super::super::Foundation::HWND, szapp: super::super::Foundation::PWSTR, szotherstuff: super::super::Foundation::PWSTR, hicon: super::WindowsAndMessaging::HICON) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellExecuteA(hwnd: super::super::Foundation::HWND, lpoperation: super::super::Foundation::PSTR, lpfile: super::super::Foundation::PSTR, lpparameters: super::super::Foundation::PSTR, lpdirectory: super::super::Foundation::PSTR, nshowcmd: i32) -> super::super::Foundation::HINSTANCE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ShellExecuteExA(pexecinfo: *mut SHELLEXECUTEINFOA) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ShellExecuteExW(pexecinfo: *mut SHELLEXECUTEINFOW) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellExecuteW(hwnd: super::super::Foundation::HWND, lpoperation: super::super::Foundation::PWSTR, lpfile: super::super::Foundation::PWSTR, lpparameters: super::super::Foundation::PWSTR, lpdirectory: super::super::Foundation::PWSTR, nshowcmd: i32) -> super::super::Foundation::HINSTANCE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellMessageBoxA(happinst: super::super::Foundation::HINSTANCE, hwnd: super::super::Foundation::HWND, lpctext: super::super::Foundation::PSTR, lpctitle: super::super::Foundation::PSTR, fustyle: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellMessageBoxW(happinst: super::super::Foundation::HINSTANCE, hwnd: super::super::Foundation::HWND, lpctext: super::super::Foundation::PWSTR, lpctitle: super::super::Foundation::PWSTR, fustyle: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_GetCachedImageIndex(pwsziconpath: super::super::Foundation::PWSTR, iiconindex: i32, uiconflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_GetCachedImageIndexA(psziconpath: super::super::Foundation::PSTR, iiconindex: i32, uiconflags: u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_GetCachedImageIndexW(psziconpath: super::super::Foundation::PWSTR, iiconindex: i32, uiconflags: u32) -> i32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn Shell_GetImageLists(phiml: *mut super::Controls::HIMAGELIST, phimlsmall: *mut super::Controls::HIMAGELIST) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn Shell_MergeMenus(hmdst: super::WindowsAndMessaging::HMENU, hmsrc: super::WindowsAndMessaging::HMENU, uinsert: u32, uidadjust: u32, uidadjustmax: u32, uflags: MM_FLAGS) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn Shell_NotifyIconA(dwmessage: NOTIFY_ICON_MESSAGE, lpdata: *const NOTIFYICONDATAA) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_NotifyIconGetRect(identifier: *const NOTIFYICONIDENTIFIER, iconlocation: *mut super::super::Foundation::RECT) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn Shell_NotifyIconW(dwmessage: NOTIFY_ICON_MESSAGE, lpdata: *const NOTIFYICONDATAW) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SignalFileOpen(pidl: *const Common::ITEMIDLIST) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_Urlmon"))]
    pub fn SoftwareUpdateMessageBox(hwnd: super::super::Foundation::HWND, pszdistunit: super::super::Foundation::PWSTR, dwflags: u32, psdi: *mut super::super::System::Com::Urlmon::SOFTDISTINFO) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn StgMakeUniqueName(pstgparent: super::super::System::Com::StructuredStorage::IStorage, pszfilespec: super::super::Foundation::PWSTR, grfmode: u32, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnA(pszstr: super::super::Foundation::PSTR, pszset: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnIA(pszstr: super::super::Foundation::PSTR, pszset: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnIW(pszstr: super::super::Foundation::PWSTR, pszset: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnW(pszstr: super::super::Foundation::PWSTR, pszset: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatBuffA(pszdest: super::super::Foundation::PSTR, pszsrc: super::super::Foundation::PSTR, cchdestbuffsize: i32) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatBuffW(pszdest: super::super::Foundation::PWSTR, pszsrc: super::super::Foundation::PWSTR, cchdestbuffsize: i32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatChainW(pszdst: super::super::Foundation::PWSTR, cchdst: u32, ichat: u32, pszsrc: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrA(pszstart: super::super::Foundation::PSTR, wmatch: u16) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrIA(pszstart: super::super::Foundation::PSTR, wmatch: u16) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrIW(pszstart: super::super::Foundation::PWSTR, wmatch: u16) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrNIW(pszstart: super::super::Foundation::PWSTR, wmatch: u16, cchmax: u32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrNW(pszstart: super::super::Foundation::PWSTR, wmatch: u16, cchmax: u32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrW(pszstart: super::super::Foundation::PWSTR, wmatch: u16) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpCA(pszstr1: super::super::Foundation::PSTR, pszstr2: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpCW(pszstr1: super::super::Foundation::PWSTR, pszstr2: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpICA(pszstr1: super::super::Foundation::PSTR, pszstr2: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpICW(pszstr1: super::super::Foundation::PWSTR, pszstr2: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpIW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpLogicalW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNA(psz1: super::super::Foundation::PSTR, psz2: super::super::Foundation::PSTR, nchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNCA(pszstr1: super::super::Foundation::PSTR, pszstr2: super::super::Foundation::PSTR, nchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNCW(pszstr1: super::super::Foundation::PWSTR, pszstr2: super::super::Foundation::PWSTR, nchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNIA(psz1: super::super::Foundation::PSTR, psz2: super::super::Foundation::PSTR, nchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNICA(pszstr1: super::super::Foundation::PSTR, pszstr2: super::super::Foundation::PSTR, nchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNICW(pszstr1: super::super::Foundation::PWSTR, pszstr2: super::super::Foundation::PWSTR, nchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNIW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR, nchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR, nchar: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCpyNW(pszdst: super::super::Foundation::PWSTR, pszsrc: super::super::Foundation::PWSTR, cchmax: i32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCpyW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrDupA(pszsrch: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrDupW(pszsrch: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSize64A(qdw: i64, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSizeA(dw: u32, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSizeEx(ull: u64, flags: SFBS_FLAGS, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSizeW(qdw: i64, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatKBSizeA(qdw: i64, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatKBSizeW(qdw: i64, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFromTimeIntervalA(pszout: super::super::Foundation::PSTR, cchmax: u32, dwtimems: u32, digits: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFromTimeIntervalW(pszout: super::super::Foundation::PWSTR, cchmax: u32, dwtimems: u32, digits: i32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrIsIntlEqualA(fcasesens: super::super::Foundation::BOOL, pszstring1: super::super::Foundation::PSTR, pszstring2: super::super::Foundation::PSTR, nchar: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrIsIntlEqualW(fcasesens: super::super::Foundation::BOOL, pszstring1: super::super::Foundation::PWSTR, pszstring2: super::super::Foundation::PWSTR, nchar: i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrNCatA(psz1: super::super::Foundation::PSTR, psz2: super::super::Foundation::PSTR, cchmax: i32) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrNCatW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR, cchmax: i32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrPBrkA(psz: super::super::Foundation::PSTR, pszset: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrPBrkW(psz: super::super::Foundation::PWSTR, pszset: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrA(pszstart: super::super::Foundation::PSTR, pszend: super::super::Foundation::PSTR, wmatch: u16) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrIA(pszstart: super::super::Foundation::PSTR, pszend: super::super::Foundation::PSTR, wmatch: u16) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrIW(pszstart: super::super::Foundation::PWSTR, pszend: super::super::Foundation::PWSTR, wmatch: u16) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrW(pszstart: super::super::Foundation::PWSTR, pszend: super::super::Foundation::PWSTR, wmatch: u16) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRStrIA(pszsource: super::super::Foundation::PSTR, pszlast: super::super::Foundation::PSTR, pszsrch: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRStrIW(pszsource: super::super::Foundation::PWSTR, pszlast: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToBSTR(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, pbstr: *mut super::super::Foundation::BSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToBufA(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToBufW(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToStrA(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, ppsz: *mut super::super::Foundation::PSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToStrW(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, ppsz: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrSpnA(psz: super::super::Foundation::PSTR, pszset: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrSpnW(psz: super::super::Foundation::PWSTR, pszset: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrA(pszfirst: super::super::Foundation::PSTR, pszsrch: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrIA(pszfirst: super::super::Foundation::PSTR, pszsrch: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrIW(pszfirst: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrNIW(pszfirst: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR, cchmax: u32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrNW(pszfirst: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR, cchmax: u32) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrW(pszfirst: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToInt64ExA(pszstring: super::super::Foundation::PSTR, dwflags: i32, pllret: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToInt64ExW(pszstring: super::super::Foundation::PWSTR, dwflags: i32, pllret: *mut i64) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntA(pszsrc: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntExA(pszstring: super::super::Foundation::PSTR, dwflags: i32, piret: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntExW(pszstring: super::super::Foundation::PWSTR, dwflags: i32, piret: *mut i32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntW(pszsrc: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrTrimA(psz: super::super::Foundation::PSTR, psztrimchars: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrTrimW(psz: super::super::Foundation::PWSTR, psztrimchars: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadUserProfile(htoken: super::super::Foundation::HANDLE, hprofile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    pub fn UnregisterAppConstrainedChangeNotification(registration: *mut _APPCONSTRAIN_REGISTRATION);
    pub fn UnregisterAppStateChangeNotification(registration: *mut _APPSTATE_REGISTRATION);
    pub fn UnregisterScaleChangeEvent(dwcookie: usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlApplySchemeA(pszin: super::super::Foundation::PSTR, pszout: super::super::Foundation::PSTR, pcchout: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlApplySchemeW(pszin: super::super::Foundation::PWSTR, pszout: super::super::Foundation::PWSTR, pcchout: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCanonicalizeA(pszurl: super::super::Foundation::PSTR, pszcanonicalized: super::super::Foundation::PSTR, pcchcanonicalized: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCanonicalizeW(pszurl: super::super::Foundation::PWSTR, pszcanonicalized: super::super::Foundation::PWSTR, pcchcanonicalized: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCombineA(pszbase: super::super::Foundation::PSTR, pszrelative: super::super::Foundation::PSTR, pszcombined: super::super::Foundation::PSTR, pcchcombined: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCombineW(pszbase: super::super::Foundation::PWSTR, pszrelative: super::super::Foundation::PWSTR, pszcombined: super::super::Foundation::PWSTR, pcchcombined: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCompareA(psz1: super::super::Foundation::PSTR, psz2: super::super::Foundation::PSTR, fignoreslash: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCompareW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR, fignoreslash: super::super::Foundation::BOOL) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCreateFromPathA(pszpath: super::super::Foundation::PSTR, pszurl: super::super::Foundation::PSTR, pcchurl: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCreateFromPathW(pszpath: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlEscapeA(pszurl: super::super::Foundation::PSTR, pszescaped: super::super::Foundation::PSTR, pcchescaped: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlEscapeW(pszurl: super::super::Foundation::PWSTR, pszescaped: super::super::Foundation::PWSTR, pcchescaped: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlFixupW(pcszurl: super::super::Foundation::PWSTR, psztranslatedurl: super::super::Foundation::PWSTR, cchmax: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetLocationA(pszurl: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetLocationW(pszurl: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetPartA(pszin: super::super::Foundation::PSTR, pszout: super::super::Foundation::PSTR, pcchout: *mut u32, dwpart: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetPartW(pszin: super::super::Foundation::PWSTR, pszout: super::super::Foundation::PWSTR, pcchout: *mut u32, dwpart: u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlHashA(pszurl: super::super::Foundation::PSTR, pbhash: *mut u8, cbhash: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlHashW(pszurl: super::super::Foundation::PWSTR, pbhash: *mut u8, cbhash: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsA(pszurl: super::super::Foundation::PSTR, urlis: URLIS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsNoHistoryA(pszurl: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsNoHistoryW(pszurl: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsOpaqueA(pszurl: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsOpaqueW(pszurl: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsW(pszurl: super::super::Foundation::PWSTR, urlis: URLIS) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlUnescapeA(pszurl: super::super::Foundation::PSTR, pszunescaped: super::super::Foundation::PSTR, pcchunescaped: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlUnescapeW(pszurl: super::super::Foundation::PWSTR, pszunescaped: super::super::Foundation::PWSTR, pcchunescaped: *mut u32, dwflags: u32) -> ::windows_sys::core::HRESULT;
    pub fn WhichPlatform() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Win32DeleteFile(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHelpA(hwndmain: super::super::Foundation::HWND, lpszhelp: super::super::Foundation::PSTR, ucommand: u32, dwdata: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHelpW(hwndmain: super::super::Foundation::HWND, lpszhelp: super::super::Foundation::PWSTR, ucommand: u32, dwdata: usize) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteCabinetState(pcs: *const CABINETSTATE) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn wnsprintfA(pszdest: super::super::Foundation::PSTR, cchdest: i32, pszfmt: super::super::Foundation::PSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn wnsprintfW(pszdest: super::super::Foundation::PWSTR, cchdest: i32, pszfmt: super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvnsprintfA(pszdest: super::super::Foundation::PSTR, cchdest: i32, pszfmt: super::super::Foundation::PSTR, arglist: *const i8) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvnsprintfW(pszdest: super::super::Foundation::PWSTR, cchdest: i32, pszfmt: super::super::Foundation::PWSTR, arglist: *const i8) -> i32;
}
#[repr(C)]
pub struct AASHELLMENUFILENAME(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AASHELLMENUITEM(i32);
pub const ABE_BOTTOM: u32 = 3u32;
pub const ABE_LEFT: u32 = 0u32;
pub const ABE_RIGHT: u32 = 2u32;
pub const ABE_TOP: u32 = 1u32;
pub const ABM_ACTIVATE: u32 = 6u32;
pub const ABM_GETAUTOHIDEBAR: u32 = 7u32;
pub const ABM_GETAUTOHIDEBAREX: u32 = 11u32;
pub const ABM_GETSTATE: u32 = 4u32;
pub const ABM_GETTASKBARPOS: u32 = 5u32;
pub const ABM_NEW: u32 = 0u32;
pub const ABM_QUERYPOS: u32 = 2u32;
pub const ABM_REMOVE: u32 = 1u32;
pub const ABM_SETAUTOHIDEBAR: u32 = 8u32;
pub const ABM_SETAUTOHIDEBAREX: u32 = 12u32;
pub const ABM_SETPOS: u32 = 3u32;
pub const ABM_SETSTATE: u32 = 10u32;
pub const ABM_WINDOWPOSCHANGED: u32 = 9u32;
pub const ABN_FULLSCREENAPP: u32 = 2u32;
pub const ABN_POSCHANGED: u32 = 1u32;
pub const ABN_STATECHANGE: u32 = 0u32;
pub const ABN_WINDOWARRANGE: u32 = 3u32;
pub const ABS_ALWAYSONTOP: u32 = 2u32;
pub const ABS_AUTOHIDE: u32 = 1u32;
pub const ACDD_VISIBLE: u32 = 1u32;
#[repr(C)]
pub struct ACENUMOPTION(i32);
#[repr(C)]
pub struct ACTIVATEOPTIONS(i32);
pub const ADDURL_SILENT: u32 = 1u32;
#[repr(C)]
pub struct ADJACENT_DISPLAY_EDGES(i32);
pub const AD_APPLY_BUFFERED_REFRESH: u32 = 16u32;
pub const AD_APPLY_DYNAMICREFRESH: u32 = 32u32;
pub const AD_APPLY_FORCE: u32 = 8u32;
pub const AD_APPLY_HTMLGEN: u32 = 2u32;
pub const AD_APPLY_REFRESH: u32 = 4u32;
pub const AD_APPLY_SAVE: u32 = 1u32;
pub const AD_GETWP_BMP: u32 = 0u32;
pub const AD_GETWP_IMAGE: u32 = 1u32;
pub const AD_GETWP_LAST_APPLIED: u32 = 2u32;
#[repr(C)]
pub struct AHE_TYPE(i32);
#[repr(C)]
pub struct AHTYPE(i32);
#[repr(C)]
pub struct APPACTIONFLAGS(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct APPBARDATA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct APPBARDATA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct APPCATEGORYINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct APPCATEGORYINFOLIST(i32);
#[repr(C)]
pub struct APPDOCLISTTYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct APPINFODATA(i32);
#[repr(C)]
pub struct APPINFODATAFLAGS(i32);
#[repr(C)]
pub struct APPLET_PROC(i32);
#[repr(C)]
pub struct APPLICATION_VIEW_MIN_WIDTH(i32);
#[repr(C)]
pub struct APPLICATION_VIEW_ORIENTATION(i32);
#[repr(C)]
pub struct APPLICATION_VIEW_SIZE_PREFERENCE(i32);
#[repr(C)]
pub struct APPLICATION_VIEW_STATE(i32);
pub const APPNAMEBUFFERLEN: u32 = 40u32;
pub const ARCONTENT_AUDIOCD: u32 = 4u32;
pub const ARCONTENT_AUTOPLAYMUSIC: u32 = 256u32;
pub const ARCONTENT_AUTOPLAYPIX: u32 = 128u32;
pub const ARCONTENT_AUTOPLAYVIDEO: u32 = 512u32;
pub const ARCONTENT_AUTORUNINF: u32 = 2u32;
pub const ARCONTENT_BLANKBD: u32 = 8192u32;
pub const ARCONTENT_BLANKCD: u32 = 16u32;
pub const ARCONTENT_BLANKDVD: u32 = 32u32;
pub const ARCONTENT_BLURAY: u32 = 16384u32;
pub const ARCONTENT_CAMERASTORAGE: u32 = 32768u32;
pub const ARCONTENT_CUSTOMEVENT: u32 = 65536u32;
pub const ARCONTENT_DVDAUDIO: u32 = 4096u32;
pub const ARCONTENT_DVDMOVIE: u32 = 8u32;
pub const ARCONTENT_MASK: u32 = 131070u32;
pub const ARCONTENT_NONE: u32 = 0u32;
pub const ARCONTENT_PHASE_FINAL: u32 = 1073741824u32;
pub const ARCONTENT_PHASE_MASK: u32 = 1879048192u32;
pub const ARCONTENT_PHASE_PRESNIFF: u32 = 268435456u32;
pub const ARCONTENT_PHASE_SNIFFING: u32 = 536870912u32;
pub const ARCONTENT_PHASE_UNKNOWN: u32 = 0u32;
pub const ARCONTENT_SVCD: u32 = 2048u32;
pub const ARCONTENT_UNKNOWNCONTENT: u32 = 64u32;
pub const ARCONTENT_VCD: u32 = 1024u32;
#[repr(C)]
pub struct ASSOCCLASS(i32);
#[repr(C)]
pub struct ASSOCDATA(i32);
#[repr(C)]
pub struct ASSOCENUM(i32);
pub const ASSOCF_APP_TO_APP: i32 = 65536i32;
pub const ASSOCF_IGNOREBASECLASS: i32 = 512i32;
pub const ASSOCF_INIT_BYEXENAME: i32 = 2i32;
pub const ASSOCF_INIT_DEFAULTTOFOLDER: i32 = 8i32;
pub const ASSOCF_INIT_DEFAULTTOSTAR: i32 = 4i32;
pub const ASSOCF_INIT_FIXED_PROGID: i32 = 2048i32;
pub const ASSOCF_INIT_FOR_FILE: i32 = 8192i32;
pub const ASSOCF_INIT_IGNOREUNKNOWN: i32 = 1024i32;
pub const ASSOCF_INIT_NOREMAPCLSID: i32 = 1i32;
pub const ASSOCF_IS_FULL_URI: i32 = 16384i32;
pub const ASSOCF_IS_PROTOCOL: i32 = 4096i32;
pub const ASSOCF_NOFIXUPS: i32 = 256i32;
pub const ASSOCF_NONE: i32 = 0i32;
pub const ASSOCF_NOTRUNCATE: i32 = 32i32;
pub const ASSOCF_NOUSERSETTINGS: i32 = 16i32;
pub const ASSOCF_OPEN_BYEXENAME: i32 = 2i32;
pub const ASSOCF_PER_MACHINE_ONLY: i32 = 32768i32;
pub const ASSOCF_REMAPRUNDLL: i32 = 128i32;
pub const ASSOCF_VERIFY: i32 = 64i32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[repr(C)]
pub struct ASSOCIATIONELEMENT(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[repr(C)]
pub struct ASSOCIATIONELEMENT(i32);
#[repr(C)]
pub struct ASSOCIATIONLEVEL(i32);
#[repr(C)]
pub struct ASSOCIATIONTYPE(i32);
#[repr(C)]
pub struct ASSOCKEY(i32);
#[repr(C)]
pub struct ASSOCSTR(i32);
#[repr(C)]
pub struct ASSOC_FILTER(i32);
#[repr(C)]
pub struct ATTACHMENT_ACTION(i32);
#[repr(C)]
pub struct ATTACHMENT_PROMPT(i32);
#[repr(C)]
pub struct AUTOCOMPLETELISTOPTIONS(i32);
#[repr(C)]
pub struct AUTOCOMPLETEOPTIONS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct AUTO_SCROLL_DATA(i32);
#[repr(C)]
pub struct AccessibilityDockingService(i32);
#[repr(C)]
pub struct AlphabeticalCategorizer(i32);
#[repr(C)]
pub struct AppShellVerbHandler(i32);
#[repr(C)]
pub struct AppStartupLink(i32);
#[repr(C)]
pub struct AppVisibility(i32);
#[repr(C)]
pub struct ApplicationActivationManager(i32);
#[repr(C)]
pub struct ApplicationAssociationRegistration(i32);
#[repr(C)]
pub struct ApplicationAssociationRegistrationUI(i32);
#[repr(C)]
pub struct ApplicationDesignModeSettings(i32);
#[repr(C)]
pub struct ApplicationDestinations(i32);
#[repr(C)]
pub struct ApplicationDocumentLists(i32);
#[repr(C)]
pub struct AttachmentServices(i32);
#[cfg(feature = "Win32_UI_Shell_Common")]
#[repr(C)]
pub struct BANDINFOSFB(i32);
#[repr(C)]
pub struct BANDSITECID(i32);
#[repr(C)]
pub struct BANDSITEINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct BANNER_NOTIFICATION(i32);
#[repr(C)]
pub struct BANNER_NOTIFICATION_EVENT(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct BASEBROWSERDATALH(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct BASEBROWSERDATAXP(i32);
#[repr(C)]
pub struct BFFCALLBACK(i32);
pub const BFFM_ENABLEOK: u32 = 1125u32;
pub const BFFM_INITIALIZED: u32 = 1u32;
pub const BFFM_IUNKNOWN: u32 = 5u32;
pub const BFFM_SELCHANGED: u32 = 2u32;
pub const BFFM_SETEXPANDED: u32 = 1130u32;
pub const BFFM_SETOKTEXT: u32 = 1129u32;
pub const BFFM_SETSELECTION: u32 = 1127u32;
pub const BFFM_SETSELECTIONA: u32 = 1126u32;
pub const BFFM_SETSELECTIONW: u32 = 1127u32;
pub const BFFM_SETSTATUSTEXT: u32 = 1128u32;
pub const BFFM_SETSTATUSTEXTA: u32 = 1124u32;
pub const BFFM_SETSTATUSTEXTW: u32 = 1128u32;
pub const BFFM_VALIDATEFAILED: u32 = 4u32;
pub const BFFM_VALIDATEFAILEDA: u32 = 3u32;
pub const BFFM_VALIDATEFAILEDW: u32 = 4u32;
pub const BHID_AssociationArray: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3198807831,
    data2: 33521,
    data3: 20320,
    data4: [146, 132, 79, 141, 183, 92, 59, 233],
};
pub const BHID_DataObject: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3099639199,
    data2: 60708,
    data3: 17756,
    data4: [131, 230, 213, 57, 12, 79, 232, 196],
};
pub const BHID_EnumAssocHandlers: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3098217372, data2: 49900, data3: 20346, data4: [145, 141, 49, 73, 0, 230, 40, 10] };
pub const BHID_EnumItems: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2499151129,
    data2: 10320,
    data3: 18724,
    data4: [170, 90, 209, 94, 132, 134, 128, 57],
};
pub const BHID_FilePlaceholder: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2256002283, data2: 43744, data3: 16389, data4: [141, 61, 84, 127, 168, 82, 248, 37] };
pub const BHID_Filter: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 953190264,
    data2: 62807,
    data3: 18064,
    data4: [158, 191, 186, 84, 112, 106, 216, 247],
};
pub const BHID_LinkTargetItem: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 964813352, data2: 62809, data3: 4563, data4: [142, 58, 0, 192, 79, 104, 55, 213] };
pub const BHID_PropertyStore: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 59040164, data2: 5411, data3: 17308, data4: [164, 200, 171, 145, 16, 82, 245, 134] };
pub const BHID_RandomAccessStream: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4050635067,
    data2: 30638,
    data3: 19710,
    data4: [189, 167, 168, 102, 238, 166, 135, 141],
};
pub const BHID_SFObject: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 964813348, data2: 62809, data3: 4563, data4: [142, 58, 0, 192, 79, 104, 55, 213] };
pub const BHID_SFUIObject: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 964813349, data2: 62809, data3: 4563, data4: [142, 58, 0, 192, 79, 104, 55, 213] };
pub const BHID_SFViewObject: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 964813350, data2: 62809, data3: 4563, data4: [142, 58, 0, 192, 79, 104, 55, 213] };
pub const BHID_Storage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 964813351, data2: 62809, data3: 4563, data4: [142, 58, 0, 192, 79, 104, 55, 213] };
pub const BHID_StorageEnum: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1176610019, data2: 61654, data3: 18291, data4: [138, 156, 70, 231, 123, 23, 72, 64] };
pub const BHID_StorageItem: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1078862089,
    data2: 30674,
    data3: 18073,
    data4: [165, 160, 79, 223, 16, 219, 152, 55],
};
pub const BHID_Stream: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 485209003,
    data2: 31760,
    data3: 18842,
    data4: [164, 23, 146, 202, 22, 196, 203, 131],
};
pub const BHID_ThumbnailHandler: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2066638090,
    data2: 36384,
    data3: 20298,
    data4: [176, 158, 101, 151, 175, 199, 47, 176],
};
pub const BHID_Transfer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3588441761, data2: 63315, data3: 18738, data4: [180, 3, 69, 116, 128, 14, 36, 152] };
pub const BIF_BROWSEFILEJUNCTIONS: u32 = 65536u32;
pub const BIF_BROWSEFORCOMPUTER: u32 = 4096u32;
pub const BIF_BROWSEFORPRINTER: u32 = 8192u32;
pub const BIF_BROWSEINCLUDEFILES: u32 = 16384u32;
pub const BIF_BROWSEINCLUDEURLS: u32 = 128u32;
pub const BIF_DONTGOBELOWDOMAIN: u32 = 2u32;
pub const BIF_EDITBOX: u32 = 16u32;
pub const BIF_NEWDIALOGSTYLE: u32 = 64u32;
pub const BIF_NONEWFOLDERBUTTON: u32 = 512u32;
pub const BIF_NOTRANSLATETARGETS: u32 = 1024u32;
pub const BIF_RETURNFSANCESTORS: u32 = 8u32;
pub const BIF_RETURNONLYFSDIRS: u32 = 1u32;
pub const BIF_SHAREABLE: u32 = 32768u32;
pub const BIF_STATUSTEXT: u32 = 4u32;
pub const BIF_UAHINT: u32 = 256u32;
pub const BIF_VALIDATE: u32 = 32u32;
pub const BIND_INTERRUPTABLE: u32 = 4294967295u32;
pub const BMICON_LARGE: i32 = 0i32;
pub const BMICON_SMALL: i32 = 1i32;
#[repr(C)]
pub struct BNSTATE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct BROWSEINFOA(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct BROWSEINFOW(i32);
pub const BSF_CANMAXIMIZE: u32 = 1024u32;
pub const BSF_DELEGATEDNAVIGATION: u32 = 65536u32;
pub const BSF_DONTSHOWNAVCANCELPAGE: u32 = 16384u32;
pub const BSF_FEEDNAVIGATION: u32 = 524288u32;
pub const BSF_FEEDSUBSCRIBED: u32 = 1048576u32;
pub const BSF_HTMLNAVCANCELED: u32 = 8192u32;
pub const BSF_MERGEDMENUS: u32 = 262144u32;
pub const BSF_NAVNOHISTORY: u32 = 4096u32;
pub const BSF_NOLOCALFILEWARNING: u32 = 16u32;
pub const BSF_REGISTERASDROPTARGET: u32 = 1u32;
pub const BSF_RESIZABLE: u32 = 512u32;
pub const BSF_SETNAVIGATABLECODEPAGE: u32 = 32768u32;
pub const BSF_THEATERMODE: u32 = 2u32;
pub const BSF_TOPBROWSER: u32 = 2048u32;
pub const BSF_TRUSTEDFORACTIVEX: u32 = 131072u32;
pub const BSF_UISETBYAUTOMATION: u32 = 256u32;
pub const BSIM_STATE: u32 = 1u32;
pub const BSIM_STYLE: u32 = 2u32;
pub const BSIS_ALWAYSGRIPPER: u32 = 2u32;
pub const BSIS_AUTOGRIPPER: u32 = 0u32;
pub const BSIS_FIXEDORDER: u32 = 1024u32;
pub const BSIS_LEFTALIGN: u32 = 4u32;
pub const BSIS_LOCKED: u32 = 256u32;
pub const BSIS_NOCAPTION: u32 = 64u32;
pub const BSIS_NOCONTEXTMENU: u32 = 16u32;
pub const BSIS_NODROPTARGET: u32 = 32u32;
pub const BSIS_NOGRIPPER: u32 = 1u32;
pub const BSIS_PREFERNOLINEBREAK: u32 = 128u32;
pub const BSIS_PRESERVEORDERDURINGLAYOUT: u32 = 512u32;
pub const BSIS_SINGLECLICK: u32 = 8u32;
pub const BSSF_NOTITLE: u32 = 2u32;
pub const BSSF_UNDELETEABLE: u32 = 4096u32;
pub const BSSF_VISIBLE: u32 = 1u32;
pub const BUFFLEN: u32 = 255u32;
#[repr(C)]
pub struct BrowserNavConstants(i32);
#[repr(C)]
pub struct CABINETSTATE(i32);
pub const CABINETSTATE_VERSION: u32 = 2u32;
pub const CAMERAROLL_E_NO_DOWNSAMPLING_REQUIRED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927456i32 as _);
#[repr(C)]
pub struct CATEGORYINFO_FLAGS(i32);
#[repr(C)]
pub struct CATEGORY_INFO(i32);
pub const CATID_BrowsableShellExt: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136336, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const CATID_BrowseInPlace: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136337, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const CATID_CommBand: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136340, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const CATID_DeskBand: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136338, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const CATID_FilePlaceholderMergeHandler: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1050450513,
    data2: 54442,
    data3: 18544,
    data4: [180, 124, 116, 36, 180, 145, 241, 204],
};
pub const CATID_InfoBand: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136339, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const CATID_LocationFactory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2522631505, data2: 35702, data3: 20055, data4: [128, 183, 86, 77, 46, 164, 181, 94] };
pub const CATID_LocationProvider: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 456959092, data2: 9748, data3: 16715, data4: [184, 19, 26, 206, 202, 62, 61, 216] };
pub const CATID_SearchableApplication: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 913058090,
    data2: 55731,
    data3: 19903,
    data4: [187, 112, 230, 46, 195, 208, 187, 191],
};
#[repr(C)]
pub struct CATSORT_FLAGS(i32);
pub const CDB2GVF_ADDSHIELD: u32 = 64u32;
pub const CDB2GVF_ALLOWPREVIEWPANE: u32 = 4u32;
pub const CDB2GVF_ISFILESAVE: u32 = 2u32;
pub const CDB2GVF_ISFOLDERPICKER: u32 = 32u32;
pub const CDB2GVF_NOINCLUDEITEM: u32 = 16u32;
pub const CDB2GVF_NOSELECTVERB: u32 = 8u32;
pub const CDB2GVF_SHOWALLFILES: u32 = 1u32;
pub const CDB2N_CONTEXTMENU_DONE: u32 = 1u32;
pub const CDB2N_CONTEXTMENU_START: u32 = 2u32;
pub const CDBOSC_KILLFOCUS: u32 = 1u32;
pub const CDBOSC_RENAME: u32 = 3u32;
pub const CDBOSC_SELCHANGE: u32 = 2u32;
pub const CDBOSC_SETFOCUS: u32 = 0u32;
pub const CDBOSC_STATECHANGE: u32 = 4u32;
#[repr(C)]
pub struct CDBURNINGEXTENSIONRET(i32);
#[repr(C)]
pub struct CDBurn(i32);
#[repr(C)]
pub struct CDCONTROLSTATEF(i32);
pub const CGID_DefView: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1257275152, data2: 53809, data3: 4560, data4: [185, 66, 0, 160, 201, 3, 18, 225] };
pub const CGID_Explorer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136400, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const CGID_ExplorerBarDoc: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136403, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const CGID_MENUDESKBAR: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1553926674, data2: 38302, data3: 4560, data4: [163, 164, 0, 160, 201, 8, 38, 54] };
pub const CGID_ShellDocView: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136401, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const CGID_ShellServiceObject: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136402, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const CGID_ShortCut: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2477164368, data2: 38170, data3: 4561, data4: [148, 111, 0, 0, 0, 0, 0, 0] };
#[repr(C)]
pub struct CIDA(i32);
#[repr(transparent)]
pub struct CIE4ConnectionPoint(pub *mut ::core::ffi::c_void);
pub const CLOSEPROPS_DISCARD: u32 = 1u32;
pub const CLOSEPROPS_NONE: u32 = 0u32;
pub const CLSID_ACLCustomMRU: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1765137299,
    data2: 8680,
    data3: 19660,
    data4: [190, 185, 159, 227, 199, 122, 41, 122],
};
pub const CLSID_ACLHistory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 12265316, data2: 27255, data3: 4560, data4: [165, 53, 0, 192, 79, 215, 208, 98] };
pub const CLSID_ACLMRU: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1733731905, data2: 56945, data3: 4560, data4: [131, 27, 0, 170, 0, 91, 67, 131] };
pub const CLSID_ACLMulti: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 12265317, data2: 27255, data3: 4560, data4: [165, 53, 0, 192, 79, 215, 208, 98] };
pub const CLSID_ACListISF: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 62928625, data2: 41350, data3: 4560, data4: [130, 74, 0, 170, 0, 91, 67, 131] };
pub const CLSID_ActiveDesktop: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1963230976, data2: 61215, data3: 4560, data4: [152, 136, 0, 96, 151, 222, 172, 249] };
pub const CLSID_AutoComplete: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 12265315, data2: 27255, data3: 4560, data4: [165, 53, 0, 192, 79, 215, 208, 98] };
pub const CLSID_CAnchorBrowsePropertyPage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611643, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CLSID_CDocBrowsePropertyPage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611636, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CLSID_CFSIconOverlayManager: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1672814465, data2: 51304, data3: 4560, data4: [153, 156, 0, 192, 79, 214, 85, 225] };
pub const CLSID_CImageBrowsePropertyPage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 810611635, data2: 39093, data3: 4559, data4: [187, 130, 0, 170, 0, 189, 206, 11] };
pub const CLSID_CURLSearchHook: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3485445632, data2: 6054, data3: 4560, data4: [153, 203, 0, 192, 79, 214, 68, 151] };
pub const CLSID_CUrlHistory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1010256448, data2: 47844, data3: 4559, data4: [191, 125, 0, 170, 0, 105, 70, 238] };
pub const CLSID_ControlPanel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 569122848, data2: 15082, data3: 4201, data4: [162, 221, 8, 0, 43, 48, 48, 157] };
pub const CLSID_DarwinAppPublisher: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3486304160, data2: 41602, data3: 4561, data4: [144, 130, 0, 96, 8, 5, 147, 130] };
pub const CLSID_DocHostUIHandler: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1884809554, data2: 48411, data3: 4561, data4: [137, 25, 0, 192, 79, 194, 200, 54] };
pub const CLSID_DragDropHelper: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1180116874, data2: 16667, data3: 4562, data4: [131, 154, 0, 192, 79, 217, 24, 208] };
pub const CLSID_FileTypes: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2962351424, data2: 33763, data3: 4559, data4: [167, 19, 0, 32, 175, 215, 151, 98] };
pub const CLSID_FolderItemsMultiLevel: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1405569062, data2: 43929, data3: 19763, data4: [172, 164, 49, 23, 245, 29, 55, 136] };
pub const CLSID_FolderShortcut: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 184209105, data2: 59432, data3: 4561, data4: [145, 135, 181, 50, 241, 233, 87, 93] };
pub const CLSID_HWShellExecute: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4290274655,
    data2: 33209,
    data3: 20430,
    data4: [184, 156, 154, 107, 167, 109, 19, 231],
};
pub const CLSID_ISFBand: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3626754736, data2: 22372, data3: 4560, data4: [169, 110, 0, 192, 79, 215, 5, 162] };
pub const CLSID_Internet: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2266780544, data2: 17056, data3: 4201, data4: [162, 234, 8, 0, 43, 48, 48, 157] };
pub const CLSID_InternetButtons: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 511273344, data2: 40133, data3: 4561, data4: [168, 63, 0, 192, 79, 201, 157, 97] };
pub const CLSID_InternetShortcut: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4226956096, data2: 58352, data3: 4123, data4: [132, 136, 0, 170, 0, 62, 86, 248] };
pub const CLSID_LinkColumnProvider: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 619794178, data2: 31516, data3: 4561, data4: [131, 143, 0, 0, 248, 4, 97, 207] };
pub const CLSID_MSOButtons: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 395261112, data2: 41602, data3: 4562, data4: [134, 197, 0, 192, 79, 142, 234, 153] };
pub const CLSID_MenuBand: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1531817510, data2: 47111, data3: 4560, data4: [152, 21, 0, 192, 79, 217, 25, 114] };
pub const CLSID_MenuBandSite: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3778999524, data2: 54002, data3: 4560, data4: [152, 22, 0, 192, 79, 217, 25, 114] };
pub const CLSID_MenuToolbarBase: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1085892112, data2: 46370, data3: 4561, data4: [179, 180, 0, 170, 0, 110, 253, 231] };
pub const CLSID_MyComputer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 550522848, data2: 15082, data3: 4201, data4: [162, 216, 8, 0, 43, 48, 48, 157] };
pub const CLSID_MyDocuments: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1158516666, data2: 44325, data3: 4560, data4: [152, 168, 8, 0, 54, 27, 17, 3] };
pub const CLSID_NetworkDomain: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1189111424, data2: 19440, data3: 4561, data4: [131, 238, 0, 160, 201, 13, 200, 73] };
pub const CLSID_NetworkServer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3226741392, data2: 19440, data3: 4561, data4: [131, 238, 0, 160, 201, 13, 200, 73] };
pub const CLSID_NetworkShare: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1420252352, data2: 19440, data3: 4561, data4: [131, 238, 0, 160, 201, 13, 200, 73] };
pub const CLSID_NewMenu: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3647578880, data2: 59391, data3: 4560, data4: [169, 59, 0, 160, 201, 15, 39, 25] };
pub const CLSID_Printers: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 573022848, data2: 15082, data3: 4201, data4: [162, 222, 8, 0, 43, 48, 48, 157] };
pub const CLSID_ProgressDialog: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4164434002, data2: 64723, data3: 4561, data4: [166, 185, 0, 96, 151, 223, 91, 212] };
pub const CLSID_QueryAssociations: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2691708157,
    data2: 27818,
    data3: 18772,
    data4: [172, 63, 151, 162, 114, 22, 249, 138],
};
pub const CLSID_QuickLinks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 240959265, data2: 53599, data3: 4560, data4: [131, 1, 0, 170, 0, 91, 67, 131] };
pub const CLSID_RecycleBin: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1684009024, data2: 20609, data3: 4123, data4: [159, 8, 0, 170, 0, 47, 149, 78] };
pub const CLSID_ShellFldSetExt: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1834161088, data2: 35938, data3: 4561, data4: [178, 205, 0, 96, 151, 223, 140, 17] };
pub const CLSID_ShellThumbnailDiskCache: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 515755904, data2: 41472, data3: 4560, data4: [163, 164, 0, 192, 79, 215, 6, 236] };
pub const CLSID_ToolbarExtButtons: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 753186264, data2: 41615, data3: 4562, data4: [134, 197, 0, 192, 79, 142, 234, 153] };
pub const CMDID_INTSHORTCUTCREATE: i32 = 1i32;
pub const CMF_ASYNCVERBSTATE: u32 = 1024u32;
pub const CMF_CANRENAME: u32 = 16u32;
pub const CMF_DEFAULTONLY: u32 = 1u32;
pub const CMF_DISABLEDVERBS: u32 = 512u32;
pub const CMF_DONOTPICKDEFAULT: u32 = 8192u32;
pub const CMF_EXPLORE: u32 = 4u32;
pub const CMF_EXTENDEDVERBS: u32 = 256u32;
pub const CMF_INCLUDESTATIC: u32 = 64u32;
pub const CMF_ITEMMENU: u32 = 128u32;
pub const CMF_NODEFAULT: u32 = 32u32;
pub const CMF_NORMAL: u32 = 0u32;
pub const CMF_NOVERBS: u32 = 8u32;
pub const CMF_OPTIMIZEFORINVOKE: u32 = 2048u32;
pub const CMF_RESERVED: u32 = 4294901760u32;
pub const CMF_SYNCCASCADEMENU: u32 = 4096u32;
pub const CMF_VERBSONLY: u32 = 2u32;
pub const CMIC_MASK_CONTROL_DOWN: u32 = 1073741824u32;
pub const CMIC_MASK_PTINVOKE: u32 = 536870912u32;
pub const CMIC_MASK_SHIFT_DOWN: u32 = 268435456u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CMINVOKECOMMANDINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CMINVOKECOMMANDINFOEX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CMINVOKECOMMANDINFOEX_REMOTE(i32);
#[repr(C)]
pub struct CM_COLUMNINFO(i32);
#[repr(C)]
pub struct CM_ENUM_FLAGS(i32);
#[repr(C)]
pub struct CM_MASK(i32);
#[repr(C)]
pub struct CM_SET_WIDTH_VALUE(i32);
#[repr(C)]
pub struct CM_STATE(i32);
pub const COMPONENT_DEFAULT_LEFT: u32 = 65535u32;
pub const COMPONENT_DEFAULT_TOP: u32 = 65535u32;
pub const COMPONENT_TOP: u32 = 1073741823u32;
pub const COMP_ELEM_CHECKED: u32 = 2u32;
pub const COMP_ELEM_CURITEMSTATE: u32 = 16384u32;
pub const COMP_ELEM_DIRTY: u32 = 4u32;
pub const COMP_ELEM_FRIENDLYNAME: u32 = 1024u32;
pub const COMP_ELEM_NOSCROLL: u32 = 8u32;
pub const COMP_ELEM_ORIGINAL_CSI: u32 = 4096u32;
pub const COMP_ELEM_POS_LEFT: u32 = 16u32;
pub const COMP_ELEM_POS_TOP: u32 = 32u32;
pub const COMP_ELEM_POS_ZINDEX: u32 = 256u32;
pub const COMP_ELEM_RESTORED_CSI: u32 = 8192u32;
pub const COMP_ELEM_SIZE_HEIGHT: u32 = 128u32;
pub const COMP_ELEM_SIZE_WIDTH: u32 = 64u32;
pub const COMP_ELEM_SOURCE: u32 = 512u32;
pub const COMP_ELEM_SUBSCRIBEDURL: u32 = 2048u32;
pub const COMP_ELEM_TYPE: u32 = 1u32;
pub const COMP_TYPE_CFHTML: u32 = 4u32;
pub const COMP_TYPE_CONTROL: u32 = 3u32;
pub const COMP_TYPE_HTMLDOC: u32 = 0u32;
pub const COMP_TYPE_MAX: u32 = 4u32;
pub const COMP_TYPE_PICTURE: u32 = 1u32;
pub const COMP_TYPE_WEBSITE: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CONFIRM_CONFLICT_ITEM(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CONFIRM_CONFLICT_RESULT_INFO(i32);
pub const COPYENGINE_E_ACCESSDENIED_READONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927681i32 as _);
pub const COPYENGINE_E_ACCESS_DENIED_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927710i32 as _);
pub const COPYENGINE_E_ACCESS_DENIED_SRC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927711i32 as _);
pub const COPYENGINE_E_ALREADY_EXISTS_FOLDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927700i32 as _);
pub const COPYENGINE_E_ALREADY_EXISTS_NORMAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927703i32 as _);
pub const COPYENGINE_E_ALREADY_EXISTS_READONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927702i32 as _);
pub const COPYENGINE_E_ALREADY_EXISTS_SYSTEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927701i32 as _);
pub const COPYENGINE_E_BLOCKED_BY_DLP_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927666i32 as _);
pub const COPYENGINE_E_BLOCKED_BY_EDP_FOR_REMOVABLE_DRIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927670i32 as _);
pub const COPYENGINE_E_BLOCKED_BY_EDP_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927672i32 as _);
pub const COPYENGINE_E_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927743i32 as _);
pub const COPYENGINE_E_CANNOT_MOVE_FROM_RECYCLE_BIN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927677i32 as _);
pub const COPYENGINE_E_CANNOT_MOVE_SHARED_FOLDER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927676i32 as _);
pub const COPYENGINE_E_CANT_REACH_SOURCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927691i32 as _);
pub const COPYENGINE_E_DEST_IS_RO_CD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927729i32 as _);
pub const COPYENGINE_E_DEST_IS_RO_DVD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927726i32 as _);
pub const COPYENGINE_E_DEST_IS_RW_CD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927728i32 as _);
pub const COPYENGINE_E_DEST_IS_RW_DVD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927725i32 as _);
pub const COPYENGINE_E_DEST_IS_R_CD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927727i32 as _);
pub const COPYENGINE_E_DEST_IS_R_DVD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927724i32 as _);
pub const COPYENGINE_E_DEST_SAME_TREE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927734i32 as _);
pub const COPYENGINE_E_DEST_SUBTREE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927735i32 as _);
pub const COPYENGINE_E_DIFF_DIR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927740i32 as _);
pub const COPYENGINE_E_DIR_NOT_EMPTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927683i32 as _);
pub const COPYENGINE_E_DISK_FULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927694i32 as _);
pub const COPYENGINE_E_DISK_FULL_CLEAN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927693i32 as _);
pub const COPYENGINE_E_EA_LOSS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927698i32 as _);
pub const COPYENGINE_E_EA_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927692i32 as _);
pub const COPYENGINE_E_ENCRYPTION_LOSS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927695i32 as _);
pub const COPYENGINE_E_FAT_MAX_IN_ROOT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927682i32 as _);
pub const COPYENGINE_E_FILE_IS_FLD_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927732i32 as _);
pub const COPYENGINE_E_FILE_TOO_LARGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927731i32 as _);
pub const COPYENGINE_E_FLD_IS_FILE_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927733i32 as _);
pub const COPYENGINE_E_INTERNET_ITEM_STORAGE_PROVIDER_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927675i32 as _);
pub const COPYENGINE_E_INTERNET_ITEM_STORAGE_PROVIDER_PAUSED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927674i32 as _);
pub const COPYENGINE_E_INTERNET_ITEM_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927678i32 as _);
pub const COPYENGINE_E_INVALID_FILES_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927716i32 as _);
pub const COPYENGINE_E_INVALID_FILES_SRC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927717i32 as _);
pub const COPYENGINE_E_MANY_SRC_1_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927739i32 as _);
pub const COPYENGINE_E_NET_DISCONNECT_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927706i32 as _);
pub const COPYENGINE_E_NET_DISCONNECT_SRC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927707i32 as _);
pub const COPYENGINE_E_NEWFILE_NAME_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927685i32 as _);
pub const COPYENGINE_E_NEWFOLDER_NAME_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927684i32 as _);
pub const COPYENGINE_E_PATH_NOT_FOUND_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927708i32 as _);
pub const COPYENGINE_E_PATH_NOT_FOUND_SRC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927709i32 as _);
pub const COPYENGINE_E_PATH_TOO_DEEP_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927714i32 as _);
pub const COPYENGINE_E_PATH_TOO_DEEP_SRC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927715i32 as _);
pub const COPYENGINE_E_PROPERTIES_LOSS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927696i32 as _);
pub const COPYENGINE_E_PROPERTY_LOSS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927697i32 as _);
pub const COPYENGINE_E_RECYCLE_BIN_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927686i32 as _);
pub const COPYENGINE_E_RECYCLE_FORCE_NUKE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927690i32 as _);
pub const COPYENGINE_E_RECYCLE_PATH_TOO_LONG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927688i32 as _);
pub const COPYENGINE_E_RECYCLE_SIZE_TOO_BIG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927689i32 as _);
pub const COPYENGINE_E_RECYCLE_UNKNOWN_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927691i32 as _);
pub const COPYENGINE_E_REDIRECTED_TO_WEBPAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927680i32 as _);
pub const COPYENGINE_E_REMOVABLE_FULL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927730i32 as _);
pub const COPYENGINE_E_REQUIRES_EDP_CONSENT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927673i32 as _);
pub const COPYENGINE_E_REQUIRES_EDP_CONSENT_FOR_REMOVABLE_DRIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927671i32 as _);
pub const COPYENGINE_E_REQUIRES_ELEVATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927742i32 as _);
pub const COPYENGINE_E_RMS_BLOCKED_BY_EDP_FOR_REMOVABLE_DRIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927668i32 as _);
pub const COPYENGINE_E_RMS_REQUIRES_EDP_CONSENT_FOR_REMOVABLE_DRIVE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927669i32 as _);
pub const COPYENGINE_E_ROOT_DIR_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927712i32 as _);
pub const COPYENGINE_E_ROOT_DIR_SRC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927713i32 as _);
pub const COPYENGINE_E_SAME_FILE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927741i32 as _);
pub const COPYENGINE_E_SERVER_BAD_FILE_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927679i32 as _);
pub const COPYENGINE_E_SHARING_VIOLATION_DEST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927704i32 as _);
pub const COPYENGINE_E_SHARING_VIOLATION_SRC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927705i32 as _);
pub const COPYENGINE_E_SILENT_FAIL_BY_DLP_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927665i32 as _);
pub const COPYENGINE_E_SRC_IS_RO_CD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927723i32 as _);
pub const COPYENGINE_E_SRC_IS_RO_DVD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927720i32 as _);
pub const COPYENGINE_E_SRC_IS_RW_CD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927722i32 as _);
pub const COPYENGINE_E_SRC_IS_RW_DVD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927719i32 as _);
pub const COPYENGINE_E_SRC_IS_R_CD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927721i32 as _);
pub const COPYENGINE_E_SRC_IS_R_DVD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927718i32 as _);
pub const COPYENGINE_E_STREAM_LOSS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927699i32 as _);
pub const COPYENGINE_E_USER_CANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927744i32 as _);
pub const COPYENGINE_E_WARNED_BY_DLP_POLICY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927667i32 as _);
pub const COPYENGINE_S_ALREADY_DONE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555914i32 as _);
pub const COPYENGINE_S_CLOSE_PROGRAM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555917i32 as _);
pub const COPYENGINE_S_COLLISIONRESOLVED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555918i32 as _);
pub const COPYENGINE_S_DONT_PROCESS_CHILDREN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555912i32 as _);
pub const COPYENGINE_S_KEEP_BOTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555916i32 as _);
pub const COPYENGINE_S_MERGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555910i32 as _);
pub const COPYENGINE_S_NOT_HANDLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555907i32 as _);
pub const COPYENGINE_S_PENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555915i32 as _);
pub const COPYENGINE_S_PROGRESS_PAUSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555919i32 as _);
pub const COPYENGINE_S_USER_IGNORED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555909i32 as _);
pub const COPYENGINE_S_USER_RETRY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555908i32 as _);
pub const COPYENGINE_S_YES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2555905i32 as _);
pub const CPFG_CREDENTIAL_PROVIDER_LABEL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 678150131, data2: 47828, data3: 17295, data4: [176, 7, 121, 183, 38, 124, 61, 72] };
pub const CPFG_CREDENTIAL_PROVIDER_LOGO: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 763590517, data2: 63181, data3: 17998, data4: [167, 69, 72, 47, 208, 180, 116, 147] };
pub const CPFG_LOGON_PASSWORD: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1617054970, data2: 42103, data3: 18353, data4: [138, 142, 58, 74, 25, 152, 24, 39] };
pub const CPFG_LOGON_USERNAME: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3658857448, data2: 38221, data3: 20435, data4: [176, 244, 31, 181, 185, 11, 23, 75] };
pub const CPFG_SMARTCARD_PIN: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1340417595,
    data2: 37249,
    data3: 18113,
    data4: [176, 164, 157, 237, 212, 219, 125, 234],
};
pub const CPFG_SMARTCARD_USERNAME: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1042206569, data2: 22156, data3: 19862, data4: [157, 89, 70, 68, 65, 116, 226, 214] };
pub const CPFG_STANDALONE_SUBMIT_BUTTON: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 192613080, data2: 52278, data3: 19801, data4: [128, 43, 130, 247, 20, 250, 112, 34] };
pub const CPFG_STYLE_LINK_AS_BUTTON: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 143631624,
    data2: 38054,
    data3: 17456,
    data4: [164, 203, 111, 198, 227, 192, 185, 226],
};
#[repr(C)]
pub struct CPLINFO(i32);
pub const CPLPAGE_DISPLAY_BACKGROUND: u32 = 1u32;
pub const CPLPAGE_KEYBOARD_SPEED: u32 = 1u32;
pub const CPLPAGE_MOUSE_BUTTONS: u32 = 1u32;
pub const CPLPAGE_MOUSE_PTRMOTION: u32 = 2u32;
pub const CPLPAGE_MOUSE_WHEEL: u32 = 3u32;
pub const CPL_DBLCLK: u32 = 5u32;
pub const CPL_DYNAMIC_RES: u32 = 0u32;
pub const CPL_EXIT: u32 = 7u32;
pub const CPL_GETCOUNT: u32 = 2u32;
pub const CPL_INIT: u32 = 1u32;
pub const CPL_INQUIRE: u32 = 3u32;
pub const CPL_NEWINQUIRE: u32 = 8u32;
pub const CPL_SELECT: u32 = 4u32;
pub const CPL_SETUP: u32 = 200u32;
pub const CPL_STARTWPARMS: u32 = 10u32;
pub const CPL_STARTWPARMSA: u32 = 9u32;
pub const CPL_STARTWPARMSW: u32 = 10u32;
pub const CPL_STOP: u32 = 6u32;
#[repr(C)]
pub struct CPVIEW(i32);
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_ACCOUNT_OPTIONS(i32);
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_CREDENTIAL_FIELD_OPTIONS(i32);
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_CREDENTIAL_SERIALIZATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_FIELD_DESCRIPTOR(i32);
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_FIELD_INTERACTIVE_STATE(i32);
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_FIELD_STATE(i32);
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_FIELD_TYPE(i32);
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_GET_SERIALIZATION_RESPONSE(i32);
pub const CREDENTIAL_PROVIDER_NO_DEFAULT: u32 = 4294967295u32;
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_STATUS_ICON(i32);
#[repr(C)]
pub struct CREDENTIAL_PROVIDER_USAGE_SCENARIO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct CSFV(i32);
pub const CSIDL_ADMINTOOLS: u32 = 48u32;
pub const CSIDL_ALTSTARTUP: u32 = 29u32;
pub const CSIDL_APPDATA: u32 = 26u32;
pub const CSIDL_BITBUCKET: u32 = 10u32;
pub const CSIDL_CDBURN_AREA: u32 = 59u32;
pub const CSIDL_COMMON_ADMINTOOLS: u32 = 47u32;
pub const CSIDL_COMMON_ALTSTARTUP: u32 = 30u32;
pub const CSIDL_COMMON_APPDATA: u32 = 35u32;
pub const CSIDL_COMMON_DESKTOPDIRECTORY: u32 = 25u32;
pub const CSIDL_COMMON_DOCUMENTS: u32 = 46u32;
pub const CSIDL_COMMON_FAVORITES: u32 = 31u32;
pub const CSIDL_COMMON_MUSIC: u32 = 53u32;
pub const CSIDL_COMMON_OEM_LINKS: u32 = 58u32;
pub const CSIDL_COMMON_PICTURES: u32 = 54u32;
pub const CSIDL_COMMON_PROGRAMS: u32 = 23u32;
pub const CSIDL_COMMON_STARTMENU: u32 = 22u32;
pub const CSIDL_COMMON_STARTUP: u32 = 24u32;
pub const CSIDL_COMMON_TEMPLATES: u32 = 45u32;
pub const CSIDL_COMMON_VIDEO: u32 = 55u32;
pub const CSIDL_COMPUTERSNEARME: u32 = 61u32;
pub const CSIDL_CONNECTIONS: u32 = 49u32;
pub const CSIDL_CONTROLS: u32 = 3u32;
pub const CSIDL_COOKIES: u32 = 33u32;
pub const CSIDL_DESKTOP: u32 = 0u32;
pub const CSIDL_DESKTOPDIRECTORY: u32 = 16u32;
pub const CSIDL_DRIVES: u32 = 17u32;
pub const CSIDL_FAVORITES: u32 = 6u32;
pub const CSIDL_FLAG_CREATE: u32 = 32768u32;
pub const CSIDL_FLAG_DONT_UNEXPAND: u32 = 8192u32;
pub const CSIDL_FLAG_DONT_VERIFY: u32 = 16384u32;
pub const CSIDL_FLAG_MASK: u32 = 65280u32;
pub const CSIDL_FLAG_NO_ALIAS: u32 = 4096u32;
pub const CSIDL_FLAG_PER_USER_INIT: u32 = 2048u32;
pub const CSIDL_FLAG_PFTI_TRACKTARGET: u32 = 16384u32;
pub const CSIDL_FONTS: u32 = 20u32;
pub const CSIDL_HISTORY: u32 = 34u32;
pub const CSIDL_INTERNET: u32 = 1u32;
pub const CSIDL_INTERNET_CACHE: u32 = 32u32;
pub const CSIDL_LOCAL_APPDATA: u32 = 28u32;
pub const CSIDL_MYDOCUMENTS: u32 = 5u32;
pub const CSIDL_MYMUSIC: u32 = 13u32;
pub const CSIDL_MYPICTURES: u32 = 39u32;
pub const CSIDL_MYVIDEO: u32 = 14u32;
pub const CSIDL_NETHOOD: u32 = 19u32;
pub const CSIDL_NETWORK: u32 = 18u32;
pub const CSIDL_PERSONAL: u32 = 5u32;
pub const CSIDL_PRINTERS: u32 = 4u32;
pub const CSIDL_PRINTHOOD: u32 = 27u32;
pub const CSIDL_PROFILE: u32 = 40u32;
pub const CSIDL_PROGRAMS: u32 = 2u32;
pub const CSIDL_PROGRAM_FILES: u32 = 38u32;
pub const CSIDL_PROGRAM_FILESX86: u32 = 42u32;
pub const CSIDL_PROGRAM_FILES_COMMON: u32 = 43u32;
pub const CSIDL_PROGRAM_FILES_COMMONX86: u32 = 44u32;
pub const CSIDL_RECENT: u32 = 8u32;
pub const CSIDL_RESOURCES: u32 = 56u32;
pub const CSIDL_RESOURCES_LOCALIZED: u32 = 57u32;
pub const CSIDL_SENDTO: u32 = 9u32;
pub const CSIDL_STARTMENU: u32 = 11u32;
pub const CSIDL_STARTUP: u32 = 7u32;
pub const CSIDL_SYSTEM: u32 = 37u32;
pub const CSIDL_SYSTEMX86: u32 = 41u32;
pub const CSIDL_TEMPLATES: u32 = 21u32;
pub const CSIDL_WINDOWS: u32 = 36u32;
#[repr(C)]
pub struct CScriptErrorList(i32);
pub const CTF_COINIT: i32 = 8i32;
pub const CTF_COINIT_MTA: i32 = 4096i32;
pub const CTF_COINIT_STA: i32 = 8i32;
pub const CTF_FREELIBANDEXIT: i32 = 16i32;
pub const CTF_INHERITWOW64: i32 = 256i32;
pub const CTF_INSIST: i32 = 1i32;
pub const CTF_KEYBOARD_LOCALE: i32 = 1024i32;
pub const CTF_NOADDREFLIB: i32 = 8192i32;
pub const CTF_OLEINITIALIZE: i32 = 2048i32;
pub const CTF_PROCESS_REF: i32 = 4i32;
pub const CTF_REF_COUNTED: i32 = 32i32;
pub const CTF_THREAD_REF: i32 = 2i32;
pub const CTF_UNUSED: i32 = 128i32;
pub const CTF_WAIT_ALLOWCOM: i32 = 64i32;
pub const CTF_WAIT_NO_REENTRANCY: i32 = 512i32;
#[repr(C)]
pub struct CommandStateChangeConstants(i32);
#[repr(C)]
pub struct ConflictFolder(i32);
#[repr(C)]
pub struct DATABLOCK_HEADER(i32);
#[repr(C)]
pub struct DATAOBJ_GET_ITEM_FLAGS(i32);
pub const DBCID_CLSIDOFBAR: i32 = 2i32;
pub const DBCID_EMPTY: i32 = 0i32;
pub const DBCID_GETBAR: i32 = 4i32;
pub const DBCID_ONDRAG: i32 = 1i32;
pub const DBCID_RESIZE: i32 = 3i32;
pub const DBCID_UPDATESIZE: i32 = 5i32;
pub const DBC_GS_IDEAL: u32 = 0u32;
pub const DBC_GS_SIZEDOWN: u32 = 1u32;
pub const DBC_HIDE: u32 = 0u32;
pub const DBC_SHOW: u32 = 1u32;
pub const DBC_SHOWOBSCURE: u32 = 2u32;
pub const DBIF_VIEWMODE_FLOATING: u32 = 2u32;
pub const DBIF_VIEWMODE_NORMAL: u32 = 0u32;
pub const DBIF_VIEWMODE_TRANSPARENT: u32 = 4u32;
pub const DBIF_VIEWMODE_VERTICAL: u32 = 1u32;
pub const DBIMF_ADDTOFRONT: u32 = 512u32;
pub const DBIMF_ALWAYSGRIPPER: u32 = 4096u32;
pub const DBIMF_BKCOLOR: u32 = 64u32;
pub const DBIMF_BREAK: u32 = 256u32;
pub const DBIMF_DEBOSSED: u32 = 32u32;
pub const DBIMF_FIXED: u32 = 1u32;
pub const DBIMF_FIXEDBMP: u32 = 4u32;
pub const DBIMF_NOGRIPPER: u32 = 2048u32;
pub const DBIMF_NOMARGINS: u32 = 8192u32;
pub const DBIMF_NORMAL: u32 = 0u32;
pub const DBIMF_TOPALIGN: u32 = 1024u32;
pub const DBIMF_UNDELETEABLE: u32 = 16u32;
pub const DBIMF_USECHEVRON: u32 = 128u32;
pub const DBIMF_VARIABLEHEIGHT: u32 = 8u32;
pub const DBIM_ACTUAL: u32 = 8u32;
pub const DBIM_BKCOLOR: u32 = 64u32;
pub const DBIM_INTEGRAL: u32 = 4u32;
pub const DBIM_MAXSIZE: u32 = 2u32;
pub const DBIM_MINSIZE: u32 = 1u32;
pub const DBIM_MODEFLAGS: u32 = 32u32;
pub const DBIM_TITLE: u32 = 16u32;
pub const DBPC_SELECTFIRST: u32 = 4294967295u32;
#[repr(C)]
pub struct DEFAULTSAVEFOLDERTYPE(i32);
#[repr(C)]
pub struct DEFAULT_FOLDER_MENU_RESTRICTIONS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct DEFCONTEXTMENU(i32);
#[repr(C)]
pub struct DEF_SHARE_ID(i32);
#[repr(C)]
pub struct DELEGATEITEMID(i32);
#[repr(C)]
pub struct DESKBANDCID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DESKBANDINFO(i32);
#[repr(C)]
pub struct DESKTOP_SLIDESHOW_DIRECTION(i32);
#[repr(C)]
pub struct DESKTOP_SLIDESHOW_OPTIONS(i32);
#[repr(C)]
pub struct DESKTOP_SLIDESHOW_STATE(i32);
#[repr(C)]
pub struct DESKTOP_WALLPAPER_POSITION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct DETAILSINFO(i32);
#[repr(transparent)]
pub struct DFConstraint(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DFMICS(i32);
#[repr(C)]
pub struct DFM_CMD(i32);
#[repr(C)]
pub struct DFM_MESSAGE_ID(i32);
pub const DISPID_BEGINDRAG: u32 = 204u32;
pub const DISPID_CHECKSTATECHANGED: u32 = 209u32;
pub const DISPID_COLUMNSCHANGED: u32 = 212u32;
pub const DISPID_CONTENTSCHANGED: u32 = 207u32;
pub const DISPID_CTRLMOUSEWHEEL: u32 = 213u32;
pub const DISPID_DEFAULTVERBINVOKED: u32 = 203u32;
pub const DISPID_ENTERPRESSED: u32 = 200u32;
pub const DISPID_ENTERPRISEIDCHANGED: u32 = 224u32;
pub const DISPID_EXPLORERWINDOWREADY: u32 = 221u32;
pub const DISPID_FILELISTENUMDONE: u32 = 201u32;
pub const DISPID_FILTERINVOKED: u32 = 218u32;
pub const DISPID_FOCUSCHANGED: u32 = 208u32;
pub const DISPID_FOLDERCHANGED: u32 = 217u32;
pub const DISPID_IADCCTL_DEFAULTCAT: u32 = 262u32;
pub const DISPID_IADCCTL_DIRTY: u32 = 256u32;
pub const DISPID_IADCCTL_FORCEX86: u32 = 259u32;
pub const DISPID_IADCCTL_ONDOMAIN: u32 = 261u32;
pub const DISPID_IADCCTL_PUBCAT: u32 = 257u32;
pub const DISPID_IADCCTL_SHOWPOSTSETUP: u32 = 260u32;
pub const DISPID_IADCCTL_SORT: u32 = 258u32;
pub const DISPID_ICONSIZECHANGED: u32 = 215u32;
pub const DISPID_INITIALENUMERATIONDONE: u32 = 223u32;
pub const DISPID_NOITEMSTATE_CHANGED: u32 = 206u32;
pub const DISPID_ORDERCHANGED: u32 = 210u32;
pub const DISPID_SEARCHCOMMAND_ABORT: u32 = 3u32;
pub const DISPID_SEARCHCOMMAND_COMPLETE: u32 = 2u32;
pub const DISPID_SEARCHCOMMAND_ERROR: u32 = 6u32;
pub const DISPID_SEARCHCOMMAND_PROGRESSTEXT: u32 = 5u32;
pub const DISPID_SEARCHCOMMAND_RESTORE: u32 = 7u32;
pub const DISPID_SEARCHCOMMAND_START: u32 = 1u32;
pub const DISPID_SEARCHCOMMAND_UPDATE: u32 = 4u32;
pub const DISPID_SELECTEDITEMCHANGED: u32 = 220u32;
pub const DISPID_SELECTIONCHANGED: u32 = 200u32;
pub const DISPID_SORTDONE: u32 = 214u32;
pub const DISPID_UPDATEIMAGE: u32 = 222u32;
pub const DISPID_VERBINVOKED: u32 = 202u32;
pub const DISPID_VIEWMODECHANGED: u32 = 205u32;
pub const DISPID_VIEWPAINTDONE: u32 = 211u32;
pub const DISPID_WORDWHEELEDITED: u32 = 219u32;
#[repr(C)]
pub struct DISPLAY_DEVICE_TYPE(i32);
pub const DLG_SCRNSAVECONFIGURE: u32 = 2003u32;
#[repr(C)]
pub struct DLLGETVERSIONPROC(i32);
#[repr(C)]
pub struct DLLVERSIONINFO(i32);
#[repr(C)]
pub struct DLLVERSIONINFO2(i32);
pub const DLLVER_BUILD_MASK: u64 = 4294901760u64;
pub const DLLVER_MAJOR_MASK: u64 = 18446462598732840960u64;
pub const DLLVER_MINOR_MASK: u64 = 281470681743360u64;
pub const DLLVER_PLATFORM_NT: u32 = 2u32;
pub const DLLVER_PLATFORM_WINDOWS: u32 = 1u32;
pub const DLLVER_QFE_MASK: u64 = 65535u64;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DRAGINFOA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DRAGINFOA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DRAGINFOW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DRAGINFOW(i32);
#[repr(C)]
pub struct DROPDESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DROPFILES(i32);
#[repr(C)]
pub struct DROPIMAGETYPE(i32);
#[repr(C)]
pub struct DSH_FLAGS(i32);
#[repr(transparent)]
pub struct DShellFolderViewEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DShellNameSpaceEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DShellWindowsEvents(pub *mut ::core::ffi::c_void);
pub const DVASPECT_COPY: u32 = 3u32;
pub const DVASPECT_LINK: u32 = 4u32;
pub const DVASPECT_SHORTNAME: u32 = 2u32;
pub const DWFAF_AUTOHIDE: u32 = 16u32;
pub const DWFAF_GROUP1: u32 = 2u32;
pub const DWFAF_GROUP2: u32 = 4u32;
pub const DWFAF_HIDDEN: u32 = 1u32;
pub const DWFRF_DELETECONFIGDATA: u32 = 1u32;
pub const DWFRF_NORMAL: u32 = 0u32;
#[repr(transparent)]
pub struct DWebBrowserEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DWebBrowserEvents2(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DefFolderMenu(i32);
#[repr(C)]
pub struct DesktopGadget(i32);
#[repr(C)]
pub struct DesktopWallpaper(i32);
#[repr(C)]
pub struct DestinationList(i32);
#[repr(C)]
pub struct DocPropShellExtension(i32);
#[repr(C)]
pub struct DriveSizeCategorizer(i32);
#[repr(C)]
pub struct DriveTypeCategorizer(i32);
#[repr(C)]
pub struct EC_HOST_UI_MODE(i32);
#[repr(C)]
pub struct EDGE_GESTURE_KIND(i32);
pub const EP_AdvQueryPane: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3035224971, data2: 13498, data3: 19513, data4: [181, 204, 22, 161, 189, 44, 65, 28] };
pub const EP_Commands: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3648280680,
    data2: 51807,
    data3: 19062,
    data4: [145, 205, 245, 161, 41, 251, 176, 118],
};
pub const EP_Commands_Organize: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1927812864, data2: 58348, data3: 18016, data4: [191, 36, 60, 59, 123, 100, 136, 6] };
pub const EP_Commands_View: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 569885485, data2: 61098, data3: 17307, data4: [187, 81, 55, 185, 111, 214, 169, 67] };
pub const EP_DetailsPane: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1135344011,
    data2: 35256,
    data3: 18221,
    data4: [185, 206, 230, 155, 130, 41, 240, 25],
};
pub const EP_NavPane: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3409013538, data2: 9719, data3: 17080, data4: [138, 9, 84, 13, 35, 164, 60, 47] };
pub const EP_PreviewPane: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2302436305, data2: 17864, data3: 19735, data4: [190, 25, 34, 59, 231, 27, 227, 101] };
pub const EP_QueryPane: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1706876495,
    data2: 20231,
    data3: 20263,
    data4: [131, 167, 26, 252, 164, 223, 125, 221],
};
pub const EP_Ribbon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3530892456, data2: 51698, data3: 18484, data4: [161, 6, 223, 136, 137, 253, 79, 55] };
pub const EP_StatusBar: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1711167182,
    data2: 23806,
    data3: 19396,
    data4: [173, 138, 122, 227, 254, 126, 143, 124],
};
pub const EXECUTE_E_LAUNCH_APPLICATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927487i32 as _);
#[repr(C)]
pub struct EXPLORER_BROWSER_FILL_FLAGS(i32);
#[repr(C)]
pub struct EXPLORER_BROWSER_OPTIONS(i32);
pub const EXP_DARWIN_ID_SIG: u32 = 2684354566u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EXP_DARWIN_LINK(i32);
#[repr(C)]
pub struct EXP_PROPERTYSTORAGE(i32);
pub const EXP_PROPERTYSTORAGE_SIG: u32 = 2684354569u32;
#[repr(C)]
pub struct EXP_SPECIAL_FOLDER(i32);
pub const EXP_SPECIAL_FOLDER_SIG: u32 = 2684354565u32;
pub const EXP_SZ_ICON_SIG: u32 = 2684354567u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct EXP_SZ_LINK(i32);
pub const EXP_SZ_LINK_SIG: u32 = 2684354561u32;
#[repr(C)]
pub struct EXTRASEARCH(i32);
pub const E_ACTIVATIONDENIED_SHELLERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927439i32 as _);
pub const E_ACTIVATIONDENIED_SHELLNOTREADY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927436i32 as _);
pub const E_ACTIVATIONDENIED_SHELLRESTART: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927438i32 as _);
pub const E_ACTIVATIONDENIED_UNEXPECTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927437i32 as _);
pub const E_ACTIVATIONDENIED_USERCLOSE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927440i32 as _);
pub const E_FILE_PLACEHOLDER_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927472i32 as _);
pub const E_FILE_PLACEHOLDER_SERVER_TIMED_OUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927470i32 as _);
pub const E_FILE_PLACEHOLDER_STORAGEPROVIDER_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927469i32 as _);
pub const E_FILE_PLACEHOLDER_VERSION_MISMATCH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927471i32 as _);
pub const E_FLAGS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217408i32 as _);
pub const E_IMAGEFEED_CHANGEDISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144926960i32 as _);
pub const E_NOTVALIDFORANIMATEDIMAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221503i32 as _);
pub const E_PREVIEWHANDLER_CORRUPT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2042494972i32 as _);
pub const E_PREVIEWHANDLER_DRM_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2042494975i32 as _);
pub const E_PREVIEWHANDLER_NOAUTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2042494974i32 as _);
pub const E_PREVIEWHANDLER_NOTFOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2042494973i32 as _);
pub const E_SHELL_EXTENSION_BLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144926975i32 as _);
pub const E_TILE_NOTIFICATIONS_PLATFORM_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927159i32 as _);
pub const E_USERTILE_CHANGEDISABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927215i32 as _);
pub const E_USERTILE_FILESIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927212i32 as _);
pub const E_USERTILE_LARGEORDYNAMIC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927214i32 as _);
pub const E_USERTILE_UNSUPPORTEDFILETYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927216i32 as _);
pub const E_USERTILE_VIDEOFRAMESIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927213i32 as _);
#[repr(C)]
pub struct EnumerableObjectCollection(i32);
#[repr(C)]
pub struct ExecuteFolder(i32);
#[repr(C)]
pub struct ExecuteUnknown(i32);
#[repr(C)]
pub struct ExplorerBrowser(i32);
pub const FCIDM_BROWSERFIRST: u32 = 40960u32;
pub const FCIDM_BROWSERLAST: u32 = 48896u32;
pub const FCIDM_GLOBALFIRST: u32 = 32768u32;
pub const FCIDM_GLOBALLAST: u32 = 40959u32;
pub const FCIDM_MENU_EDIT: u32 = 32832u32;
pub const FCIDM_MENU_EXPLORE: u32 = 33104u32;
pub const FCIDM_MENU_FAVORITES: u32 = 33136u32;
pub const FCIDM_MENU_FILE: u32 = 32768u32;
pub const FCIDM_MENU_FIND: u32 = 33088u32;
pub const FCIDM_MENU_HELP: u32 = 33024u32;
pub const FCIDM_MENU_TOOLS: u32 = 32960u32;
pub const FCIDM_MENU_TOOLS_SEP_GOTO: u32 = 32961u32;
pub const FCIDM_MENU_VIEW: u32 = 32896u32;
pub const FCIDM_MENU_VIEW_SEP_OPTIONS: u32 = 32897u32;
pub const FCIDM_SHVIEWFIRST: u32 = 0u32;
pub const FCIDM_SHVIEWLAST: u32 = 32767u32;
pub const FCIDM_STATUS: u32 = 40961u32;
pub const FCIDM_TOOLBAR: u32 = 40960u32;
pub const FCSM_CLSID: u32 = 8u32;
pub const FCSM_FLAGS: u32 = 64u32;
pub const FCSM_ICONFILE: u32 = 16u32;
pub const FCSM_INFOTIP: u32 = 4u32;
pub const FCSM_LOGO: u32 = 32u32;
pub const FCSM_VIEWID: u32 = 1u32;
pub const FCSM_WEBVIEWTEMPLATE: u32 = 2u32;
pub const FCS_FLAG_DRAGDROP: u32 = 2u32;
pub const FCS_FORCEWRITE: u32 = 2u32;
pub const FCS_READ: u32 = 1u32;
pub const FCT_ADDTOEND: u32 = 4u32;
pub const FCT_CONFIGABLE: u32 = 2u32;
pub const FCT_MERGE: u32 = 1u32;
pub const FCW_INTERNETBAR: u32 = 6u32;
pub const FCW_PROGRESS: u32 = 8u32;
pub const FCW_STATUS: u32 = 1u32;
pub const FCW_TOOLBAR: u32 = 2u32;
pub const FCW_TREE: u32 = 3u32;
#[repr(C)]
pub struct FDAP(i32);
#[repr(C)]
pub struct FDE_OVERWRITE_RESPONSE(i32);
#[repr(C)]
pub struct FDE_SHAREVIOLATION_RESPONSE(i32);
pub const FDTF_LONGDATE: u32 = 4u32;
pub const FDTF_LONGTIME: u32 = 8u32;
pub const FDTF_LTRDATE: u32 = 256u32;
pub const FDTF_NOAUTOREADINGORDER: u32 = 1024u32;
pub const FDTF_RELATIVE: u32 = 16u32;
pub const FDTF_RTLDATE: u32 = 512u32;
pub const FDTF_SHORTDATE: u32 = 2u32;
pub const FDTF_SHORTTIME: u32 = 1u32;
#[repr(C)]
pub struct FD_FLAGS(i32);
#[repr(C)]
pub struct FFFP_MODE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FILEDESCRIPTORA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FILEDESCRIPTORW(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FILEGROUPDESCRIPTORA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct FILEGROUPDESCRIPTORW(i32);
#[repr(C)]
pub struct FILETYPEATTRIBUTEFLAGS(i32);
#[repr(C)]
pub struct FILE_ATTRIBUTES_ARRAY(i32);
#[repr(C)]
pub struct FILE_OPERATION_FLAGS2(i32);
#[repr(C)]
pub struct FILE_USAGE_TYPE(i32);
#[repr(C)]
pub struct FLYOUT_PLACEMENT(i32);
pub const FMTID_Briefcase: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 848136993, data2: 30505, data3: 19452, data4: [149, 76, 144, 43, 50, 157, 86, 176] };
pub const FMTID_CustomImageProperties: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2127399694,
    data2: 49462,
    data3: 19099,
    data4: [148, 17, 78, 189, 102, 115, 204, 195],
};
pub const FMTID_DRM: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2930514404,
    data2: 35246,
    data3: 17672,
    data4: [185, 183, 187, 134, 122, 190, 226, 237],
};
pub const FMTID_Displaced: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2601995059, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] };
pub const FMTID_ImageProperties: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 347610529, data2: 309, data3: 19761, data4: [150, 217, 108, 191, 201, 103, 26, 153] };
pub const FMTID_InternetSite: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136353, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const FMTID_Intshcut: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 136352, data2: 0, data3: 0, data4: [192, 0, 0, 0, 0, 0, 0, 70] };
pub const FMTID_LibraryProperties: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1568061055,
    data2: 39741,
    data3: 17595,
    data4: [182, 174, 37, 218, 79, 99, 138, 103],
};
pub const FMTID_MUSIC: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1453537070, data2: 52892, data3: 4562, data4: [159, 14, 0, 96, 151, 198, 134, 246] };
pub const FMTID_Misc: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2601995060, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] };
pub const FMTID_Query: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1231625360, data2: 32279, data3: 4122, data4: [169, 28, 8, 0, 43, 46, 205, 169] };
pub const FMTID_ShellDetails: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 677604006, data2: 38205, data3: 4562, data4: [181, 214, 0, 192, 79, 217, 24, 208] };
pub const FMTID_Storage: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3072717104, data2: 18415, data3: 4122, data4: [165, 241, 2, 96, 140, 158, 235, 172] };
pub const FMTID_Volume: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2601995061, data2: 16639, data3: 4562, data4: [162, 126, 0, 192, 79, 195, 8, 113] };
pub const FMTID_WebView: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4062663808,
    data2: 63362,
    data3: 17041,
    data4: [189, 148, 241, 54, 147, 81, 58, 236],
};
pub const FOFX_ADDUNDORECORD: u32 = 536870912u32;
pub const FOFX_COPYASDOWNLOAD: u32 = 1073741824u32;
pub const FOFX_DONTDISPLAYDESTPATH: u32 = 134217728u32;
pub const FOFX_DONTDISPLAYLOCATIONS: u32 = 2147483648u32;
pub const FOFX_DONTDISPLAYSOURCEPATH: u32 = 67108864u32;
pub const FOFX_EARLYFAILURE: u32 = 1048576u32;
pub const FOFX_KEEPNEWERFILE: u32 = 4194304u32;
pub const FOFX_MOVEACLSACROSSVOLUMES: u32 = 33554432u32;
pub const FOFX_NOCOPYHOOKS: u32 = 8388608u32;
pub const FOFX_NOMINIMIZEBOX: u32 = 16777216u32;
pub const FOFX_NOSKIPJUNCTIONS: u32 = 65536u32;
pub const FOFX_PREFERHARDLINK: u32 = 131072u32;
pub const FOFX_PRESERVEFILEEXTENSIONS: u32 = 2097152u32;
pub const FOFX_RECYCLEONDELETE: u32 = 524288u32;
pub const FOFX_REQUIREELEVATION: u32 = 268435456u32;
pub const FOFX_SHOWELEVATIONPROMPT: u32 = 262144u32;
pub const FOF_ALLOWUNDO: u32 = 64u32;
pub const FOF_CONFIRMMOUSE: u32 = 2u32;
pub const FOF_FILESONLY: u32 = 128u32;
pub const FOF_MULTIDESTFILES: u32 = 1u32;
pub const FOF_NOCONFIRMATION: u32 = 16u32;
pub const FOF_NOCONFIRMMKDIR: u32 = 512u32;
pub const FOF_NOCOPYSECURITYATTRIBS: u32 = 2048u32;
pub const FOF_NOERRORUI: u32 = 1024u32;
pub const FOF_NORECURSEREPARSE: u32 = 32768u32;
pub const FOF_NORECURSION: u32 = 4096u32;
pub const FOF_NO_CONNECTED_ELEMENTS: u32 = 8192u32;
pub const FOF_RENAMEONCOLLISION: u32 = 8u32;
pub const FOF_SILENT: u32 = 4u32;
pub const FOF_SIMPLEPROGRESS: u32 = 256u32;
pub const FOF_WANTMAPPINGHANDLE: u32 = 32u32;
pub const FOF_WANTNUKEWARNING: u32 = 16384u32;
#[repr(C)]
pub struct FOLDERFLAGS(i32);
pub const FOLDERID_AccountPictures: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 9216177, data2: 21940, data3: 19542, data4: [184, 168, 77, 228, 178, 153, 211, 190] };
pub const FOLDERID_AddNewPrograms: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3730954609, data2: 24252, data3: 20226, data4: [163, 169, 108, 130, 137, 94, 92, 4] };
pub const FOLDERID_AdminTools: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1917776240,
    data2: 42029,
    data3: 20463,
    data4: [159, 38, 182, 14, 132, 111, 186, 79],
};
pub const FOLDERID_AllAppMods: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2060875929,
    data2: 26287,
    data3: 17338,
    data4: [145, 86, 106, 173, 66, 230, 197, 150],
};
pub const FOLDERID_AppCaptures: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3988848241,
    data2: 39128,
    data3: 20298,
    data4: [185, 32, 200, 220, 19, 60, 177, 101],
};
pub const FOLDERID_AppDataDesktop: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2999313017,
    data2: 31453,
    data3: 17311,
    data4: [178, 140, 196, 31, 225, 187, 246, 114],
};
pub const FOLDERID_AppDataDocuments: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2078369296,
    data2: 8063,
    data3: 17580,
    data4: [191, 240, 131, 225, 95, 47, 252, 161],
};
pub const FOLDERID_AppDataFavorites: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2096885692,
    data2: 56863,
    data3: 17834,
    data4: [184, 67, 165, 66, 172, 83, 108, 201],
};
pub const FOLDERID_AppDataProgramData: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1436369059, data2: 41014, data3: 16634, data4: [175, 97, 132, 203, 67, 10, 77, 52] };
pub const FOLDERID_AppUpdates: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2735066777,
    data2: 62759,
    data3: 18731,
    data4: [139, 26, 126, 118, 250, 152, 214, 228],
};
pub const FOLDERID_ApplicationShortcuts: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2744223617,
    data2: 58866,
    data3: 18576,
    data4: [179, 217, 167, 229, 67, 50, 50, 140],
};
pub const FOLDERID_AppsFolder: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 512184461, data2: 35266, data3: 17136, data4: [138, 126, 100, 90, 15, 80, 202, 88] };
pub const FOLDERID_CDBurning: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2656217872,
    data2: 63501,
    data3: 18911,
    data4: [172, 184, 67, 48, 245, 104, 120, 85],
};
pub const FOLDERID_CameraRoll: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2875177083, data2: 31970, data3: 20355, data4: [145, 93, 85, 8, 70, 201, 83, 123] };
pub const FOLDERID_CameraRollLibrary: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 723574645, data2: 7898, data3: 16441, data4: [128, 151, 56, 121, 130, 39, 213, 183] };
pub const FOLDERID_ChangeRemovePrograms: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3748816556, data2: 37492, data3: 18535, data4: [141, 85, 59, 214, 97, 222, 135, 45] };
pub const FOLDERID_CommonAdminTools: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3493351037,
    data2: 47811,
    data3: 18327,
    data4: [143, 20, 203, 162, 41, 179, 146, 181],
};
pub const FOLDERID_CommonOEMLinks: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3250250448,
    data2: 4319,
    data3: 17204,
    data4: [190, 221, 122, 162, 11, 34, 122, 157],
};
pub const FOLDERID_CommonPrograms: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 20567118,
    data2: 27390,
    data3: 18930,
    data4: [134, 144, 61, 175, 202, 230, 255, 184],
};
pub const FOLDERID_CommonStartMenu: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2752599833,
    data2: 54830,
    data3: 18717,
    data4: [170, 124, 231, 75, 139, 227, 176, 103],
};
pub const FOLDERID_CommonStartMenuPlaces: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2755692447, data2: 34720, data3: 20349, data4: [183, 0, 2, 7, 185, 102, 25, 74] };
pub const FOLDERID_CommonStartup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2191911477, data2: 55757, data3: 18373, data4: [150, 41, 225, 93, 47, 113, 78, 110] };
pub const FOLDERID_CommonTemplates: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3108124647,
    data2: 22444,
    data3: 17223,
    data4: [145, 81, 176, 140, 108, 50, 209, 247],
};
pub const FOLDERID_ComputerFolder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 180388732,
    data2: 48120,
    data3: 17706,
    data4: [133, 13, 121, 208, 142, 102, 124, 167],
};
pub const FOLDERID_ConflictFolder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1275001669,
    data2: 13437,
    data3: 16390,
    data4: [165, 190, 172, 12, 176, 86, 113, 146],
};
pub const FOLDERID_ConnectionsFolder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1863113003,
    data2: 11927,
    data3: 17873,
    data4: [136, 255, 176, 209, 134, 184, 222, 221],
};
pub const FOLDERID_Contacts: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1450723412,
    data2: 50891,
    data3: 17963,
    data4: [129, 105, 136, 227, 80, 172, 184, 130],
};
pub const FOLDERID_ControlPanelFolder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2192001771,
    data2: 44724,
    data3: 18012,
    data4: [160, 20, 208, 151, 238, 52, 109, 99],
};
pub const FOLDERID_Cookies: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 722433629, data2: 49385, data3: 16753, data4: [144, 142, 8, 166, 17, 184, 79, 246] };
pub const FOLDERID_CurrentAppMods: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1035209504,
    data2: 10800,
    data3: 19902,
    data4: [145, 126, 119, 29, 210, 29, 208, 153],
};
pub const FOLDERID_Desktop: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3032468538,
    data2: 56108,
    data3: 16972,
    data4: [176, 41, 127, 233, 154, 135, 198, 65],
};
pub const FOLDERID_DevelopmentFiles: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3689472142, data2: 12371, data3: 19388, data4: [177, 131, 42, 123, 43, 25, 30, 89] };
pub const FOLDERID_Device: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 472564188, data2: 17240, data3: 19308, data4: [151, 51, 175, 33, 21, 101, 118, 240] };
pub const FOLDERID_DeviceMetadataStore: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1558488553, data2: 58603, data3: 18333, data4: [184, 159, 19, 12, 2, 136, 97, 85] };
pub const FOLDERID_Documents: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4258503376, data2: 9103, data3: 18095, data4: [173, 180, 108, 133, 72, 3, 105, 199] };
pub const FOLDERID_DocumentsLibrary: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2064494973, data2: 40146, data3: 19091, data4: [151, 51, 70, 204, 137, 2, 46, 124] };
pub const FOLDERID_Downloads: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 927851152, data2: 4671, data3: 17765, data4: [145, 100, 57, 196, 146, 94, 70, 123] };
pub const FOLDERID_Favorites: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 393738081, data2: 26797, data3: 19850, data4: [135, 189, 48, 183, 89, 250, 51, 221] };
pub const FOLDERID_Fonts: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4246899895,
    data2: 44561,
    data3: 19171,
    data4: [134, 76, 22, 243, 145, 10, 184, 254],
};
pub const FOLDERID_GameTasks: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 89108065, data2: 19928, data3: 18311, data4: [128, 182, 9, 2, 32, 196, 183, 0] };
pub const FOLDERID_Games: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3401919514,
    data2: 46397,
    data3: 20188,
    data4: [146, 215, 107, 46, 138, 193, 148, 52],
};
pub const FOLDERID_History: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3655109179, data2: 46980, data3: 17198, data4: [167, 129, 90, 17, 48, 167, 89, 99] };
pub const FOLDERID_HomeGroup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1381141099, data2: 47587, data3: 19165, data4: [182, 13, 88, 140, 45, 186, 132, 45] };
pub const FOLDERID_HomeGroupCurrentUser: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2608117411, data2: 3581, data3: 20241, data4: [158, 120, 95, 120, 0, 242, 231, 114] };
pub const FOLDERID_ImplicitAppShortcuts: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3165988207, data2: 31222, data3: 19694, data4: [183, 37, 220, 52, 228, 2, 253, 70] };
pub const FOLDERID_InternetCache: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 891585000, data2: 13246, data3: 16977, data4: [186, 133, 96, 7, 202, 237, 207, 157] };
pub const FOLDERID_InternetFolder: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1302296692, data2: 19980, data3: 18692, data4: [150, 123, 64, 176, 210, 12, 62, 75] };
pub const FOLDERID_Libraries: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 457090524,
    data2: 46471,
    data3: 18310,
    data4: [180, 239, 189, 29, 195, 50, 174, 174],
};
pub const FOLDERID_Links: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3216627168,
    data2: 50857,
    data3: 16460,
    data4: [178, 178, 174, 109, 182, 175, 73, 104],
};
pub const FOLDERID_LocalAppData: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4055050117,
    data2: 28602,
    data3: 20431,
    data4: [157, 85, 123, 142, 127, 21, 112, 145],
};
pub const FOLDERID_LocalAppDataLow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2770379172, data2: 6016, data3: 20470, data4: [189, 24, 22, 115, 67, 197, 175, 22] };
pub const FOLDERID_LocalDocuments: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4096713427, data2: 37023, data3: 18695, data4: [136, 113, 76, 34, 252, 11, 247, 86] };
pub const FOLDERID_LocalDownloads: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2105798299, data2: 8772, data3: 20080, data4: [177, 245, 83, 147, 4, 42, 241, 228] };
pub const FOLDERID_LocalMusic: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2697370265, data2: 8648, data3: 18033, data4: [135, 3, 121, 52, 22, 47, 207, 29] };
pub const FOLDERID_LocalPictures: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 232587613, data2: 45164, data3: 17877, data4: [140, 76, 245, 151, 19, 133, 70, 57] };
pub const FOLDERID_LocalStorage: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3018524883,
    data2: 41459,
    data3: 18795,
    data4: [134, 90, 66, 181, 54, 205, 160, 236],
};
pub const FOLDERID_LocalVideos: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 891841128,
    data2: 15447,
    data3: 16801,
    data4: [187, 177, 14, 174, 115, 215, 108, 149],
};
pub const FOLDERID_LocalizedResourcesDir: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 704657246, data2: 8780, data3: 18910, data4: [184, 209, 68, 13, 247, 239, 61, 220] };
pub const FOLDERID_Music: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1272501617, data2: 27929, data3: 18643, data4: [190, 151, 66, 34, 32, 8, 14, 67] };
pub const FOLDERID_MusicLibrary: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 554871562, data2: 51306, data3: 20478, data4: [163, 104, 13, 233, 110, 71, 1, 46] };
pub const FOLDERID_NetHood: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3316367187,
    data2: 57727,
    data3: 16673,
    data4: [137, 0, 134, 98, 111, 194, 201, 115],
};
pub const FOLDERID_NetworkFolder: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3523997380, data2: 23720, data3: 18693, data4: [174, 59, 191, 37, 30, 160, 155, 83] };
pub const FOLDERID_Objects3D: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 834723109,
    data2: 37945,
    data3: 20242,
    data4: [191, 65, 127, 244, 237, 163, 135, 34],
};
pub const FOLDERID_OneDrive: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2771106374,
    data2: 59873,
    data3: 17247,
    data4: [179, 217, 40, 218, 166, 72, 192, 246],
};
pub const FOLDERID_OriginalImages: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 741785770,
    data2: 22546,
    data3: 19335,
    data4: [191, 208, 76, 208, 223, 177, 155, 57],
};
pub const FOLDERID_PhotoAlbums: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1775423376,
    data2: 64563,
    data3: 20407,
    data4: [154, 12, 235, 176, 240, 252, 180, 60],
};
pub const FOLDERID_Pictures: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 870482224, data2: 19998, data3: 18038, data4: [131, 90, 152, 57, 92, 59, 195, 187] };
pub const FOLDERID_PicturesLibrary: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2844831391, data2: 41019, data3: 20096, data4: [148, 188, 153, 18, 215, 80, 65, 4] };
pub const FOLDERID_Playlists: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3734159815, data2: 33663, data3: 20329, data4: [163, 187, 134, 230, 49, 32, 74, 35] };
pub const FOLDERID_PrintHood: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2457124237, data2: 53201, data3: 16835, data4: [179, 94, 177, 63, 85, 167, 88, 244] };
pub const FOLDERID_PrintersFolder: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1996246573, data2: 54957, data3: 17689, data4: [166, 99, 55, 189, 86, 6, 129, 133] };
pub const FOLDERID_Profile: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1584170383,
    data2: 3618,
    data3: 18272,
    data4: [154, 254, 234, 51, 23, 182, 113, 115],
};
pub const FOLDERID_ProgramData: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1655397762, data2: 64961, data3: 19907, data4: [169, 221, 7, 13, 29, 73, 93, 151] };
pub const FOLDERID_ProgramFiles: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2422105014,
    data2: 49599,
    data3: 18766,
    data4: [178, 156, 101, 183, 50, 211, 210, 26],
};
pub const FOLDERID_ProgramFilesCommon: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4159827205,
    data2: 40813,
    data3: 18338,
    data4: [170, 174, 41, 211, 23, 198, 240, 102],
};
pub const FOLDERID_ProgramFilesCommonX64: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1667618215,
    data2: 3853,
    data3: 17893,
    data4: [135, 246, 13, 165, 107, 106, 79, 125],
};
pub const FOLDERID_ProgramFilesCommonX86: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3734457636, data2: 55750, data3: 19774, data4: [191, 145, 244, 69, 81, 32, 185, 23] };
pub const FOLDERID_ProgramFilesX64: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1837142903, data2: 27376, data3: 17483, data4: [137, 87, 163, 119, 63, 2, 32, 14] };
pub const FOLDERID_ProgramFilesX86: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2086289647,
    data2: 41211,
    data3: 19452,
    data4: [135, 74, 192, 242, 224, 185, 250, 142],
};
pub const FOLDERID_Programs: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2810142071, data2: 11819, data3: 17603, data4: [166, 162, 171, 166, 1, 5, 74, 81] };
pub const FOLDERID_Public: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3755964066,
    data2: 51242,
    data3: 19811,
    data4: [144, 106, 86, 68, 172, 69, 115, 133],
};
pub const FOLDERID_PublicDesktop: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3299488781,
    data2: 61967,
    data3: 18531,
    data4: [175, 239, 248, 126, 242, 230, 186, 37],
};
pub const FOLDERID_PublicDocuments: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3980928175, data2: 56548, data3: 17832, data4: [129, 226, 252, 121, 101, 8, 54, 52] };
pub const FOLDERID_PublicDownloads: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1029983387, data2: 8120, data3: 20272, data4: [155, 69, 246, 112, 35, 95, 121, 192] };
pub const FOLDERID_PublicGameTasks: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3737068854,
    data2: 57768,
    data3: 19545,
    data4: [182, 162, 65, 69, 134, 71, 106, 234],
};
pub const FOLDERID_PublicLibraries: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1222309899, data2: 59087, data3: 20302, data4: [184, 0, 14, 105, 216, 78, 227, 132] };
pub const FOLDERID_PublicMusic: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 840235701,
    data2: 38743,
    data3: 17048,
    data4: [187, 97, 146, 169, 222, 170, 68, 255],
};
pub const FOLDERID_PublicPictures: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3068918662,
    data2: 26887,
    data3: 16700,
    data4: [154, 247, 79, 194, 171, 240, 124, 197],
};
pub const FOLDERID_PublicRingtones: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3847596896, data2: 5435, data3: 19735, data4: [159, 4, 165, 254, 153, 252, 21, 236] };
pub const FOLDERID_PublicUserTiles: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 75673452, data2: 2289, data3: 19508, data4: [140, 144, 225, 126, 201, 139, 30, 23] };
pub const FOLDERID_PublicVideos: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 603985978, data2: 24965, data3: 18939, data4: [162, 216, 74, 57, 42, 96, 43, 163] };
pub const FOLDERID_QuickLaunch: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1386541089,
    data2: 31605,
    data3: 18601,
    data4: [159, 107, 75, 135, 162, 16, 188, 143],
};
pub const FOLDERID_Recent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2924527745, data2: 60370, data3: 17290, data4: [134, 85, 138, 9, 46, 52, 152, 122] };
pub const FOLDERID_RecordedCalls: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 797655234,
    data2: 33773,
    data3: 18670,
    data4: [179, 131, 161, 241, 87, 236, 111, 154],
};
pub const FOLDERID_RecordedTVLibrary: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 443538338, data2: 62509, data3: 17240, data4: [167, 152, 183, 77, 116, 89, 38, 197] };
pub const FOLDERID_RecycleBinFolder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3075686470,
    data2: 16075,
    data3: 19480,
    data4: [190, 78, 100, 205, 76, 183, 214, 172],
};
pub const FOLDERID_ResourceDir: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2328955953,
    data2: 10971,
    data3: 17046,
    data4: [168, 247, 228, 112, 18, 50, 201, 114],
};
pub const FOLDERID_RetailDemo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 315934366, data2: 9389, data3: 18723, data4: [190, 25, 49, 50, 28, 67, 167, 103] };
pub const FOLDERID_Ringtones: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3362784331,
    data2: 62622,
    data3: 16678,
    data4: [169, 195, 181, 42, 31, 244, 17, 232],
};
pub const FOLDERID_RoamedTileImages: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2863191461,
    data2: 61910,
    data3: 16985,
    data4: [186, 168, 120, 231, 239, 96, 131, 94],
};
pub const FOLDERID_RoamingAppData: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1052149211,
    data2: 26105,
    data3: 19702,
    data4: [160, 58, 227, 239, 101, 114, 159, 61],
};
pub const FOLDERID_RoamingTiles: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 12385370, data2: 60820, data3: 20040, data4: [150, 161, 63, 98, 23, 242, 25, 144] };
pub const FOLDERID_SEARCH_CSC: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3996312646,
    data2: 12746,
    data3: 19130,
    data4: [129, 79, 165, 235, 210, 253, 109, 94],
};
pub const FOLDERID_SEARCH_MAPI: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2565606936,
    data2: 8344,
    data3: 19780,
    data4: [134, 68, 102, 151, 147, 21, 162, 129],
};
pub const FOLDERID_SampleMusic: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2991638120, data2: 62845, data3: 20193, data4: [166, 60, 41, 14, 231, 209, 170, 31] };
pub const FOLDERID_SamplePictures: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3297772864,
    data2: 9081,
    data3: 19573,
    data4: [132, 75, 100, 230, 250, 248, 113, 107],
};
pub const FOLDERID_SamplePlaylists: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 365586867,
    data2: 12526,
    data3: 18881,
    data4: [172, 225, 107, 94, 195, 114, 175, 181],
};
pub const FOLDERID_SampleVideos: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2241768852, data2: 11909, data3: 18605, data4: [167, 26, 9, 105, 203, 86, 166, 205] };
pub const FOLDERID_SavedGames: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1281110783,
    data2: 48029,
    data3: 17328,
    data4: [181, 180, 45, 114, 229, 78, 170, 164],
};
pub const FOLDERID_SavedPictures: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 991508610,
    data2: 54189,
    data3: 20139,
    data4: [150, 90, 105, 130, 157, 31, 181, 159],
};
pub const FOLDERID_SavedPicturesLibrary: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3797637138,
    data2: 48776,
    data3: 19417,
    data4: [148, 176, 41, 35, 52, 119, 182, 195],
};
pub const FOLDERID_SavedSearches: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2099067396, data2: 57019, data3: 16661, data4: [149, 207, 47, 41, 218, 41, 32, 218] };
pub const FOLDERID_Screenshots: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3082739329,
    data2: 57236,
    data3: 18050,
    data4: [167, 216, 87, 165, 38, 32, 184, 111],
};
pub const FOLDERID_SearchHistory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 223100342, data2: 931, data3: 17967, data4: [160, 230, 8, 146, 76, 65, 181, 212] };
pub const FOLDERID_SearchHome: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 419641297, data2: 47306, data3: 16673, data4: [166, 57, 109, 71, 45, 22, 151, 42] };
pub const FOLDERID_SearchTemplates: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2120444926,
    data2: 57257,
    data3: 19806,
    data4: [180, 86, 215, 179, 152, 81, 216, 169],
};
pub const FOLDERID_SendTo: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2307064684, data2: 10176, data3: 16459, data4: [143, 8, 16, 45, 16, 220, 253, 116] };
pub const FOLDERID_SidebarDefaultParts: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2067361364, data2: 40645, data3: 17152, data4: [190, 10, 36, 130, 235, 174, 26, 38] };
pub const FOLDERID_SidebarParts: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2807903790,
    data2: 20732,
    data3: 20407,
    data4: [172, 44, 168, 190, 170, 49, 68, 147],
};
pub const FOLDERID_SkyDrive: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2771106374,
    data2: 59873,
    data3: 17247,
    data4: [179, 217, 40, 218, 166, 72, 192, 246],
};
pub const FOLDERID_SkyDriveCameraRoll: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1987995665, data2: 18891, data3: 17011, data4: [135, 194, 32, 243, 85, 225, 8, 91] };
pub const FOLDERID_SkyDriveDocuments: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 618176036,
    data2: 12057,
    data3: 17716,
    data4: [157, 222, 106, 102, 113, 251, 184, 254],
};
pub const FOLDERID_SkyDriveMusic: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3287434654,
    data2: 32982,
    data3: 17884,
    data4: [191, 239, 31, 118, 159, 43, 231, 48],
};
pub const FOLDERID_SkyDrivePictures: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 865540533,
    data2: 35911,
    data3: 18580,
    data4: [148, 194, 216, 247, 122, 221, 68, 166],
};
pub const FOLDERID_StartMenu: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1650152387, data2: 43848, data3: 20161, data4: [186, 31, 161, 239, 65, 70, 252, 25] };
pub const FOLDERID_StartMenuAllPrograms: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4066575855, data2: 26952, data3: 16569, data4: [178, 85, 129, 69, 61, 9, 199, 133] };
pub const FOLDERID_Startup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3111985339, data2: 62570, data3: 19607, data4: [186, 16, 94, 54, 8, 67, 8, 84] };
pub const FOLDERID_SyncManagerFolder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1130793976,
    data2: 49486,
    data3: 18866,
    data4: [151, 201, 116, 119, 132, 215, 132, 183],
};
pub const FOLDERID_SyncResultsFolder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 681220675,
    data2: 48708,
    data3: 16471,
    data4: [164, 27, 88, 122, 118, 215, 231, 249],
};
pub const FOLDERID_SyncSetupFolder: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 253837624,
    data2: 45523,
    data3: 19088,
    data4: [187, 169, 39, 203, 192, 197, 56, 154],
};
pub const FOLDERID_System: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 448876151, data2: 743, data3: 20061, data4: [183, 68, 46, 177, 174, 81, 152, 183] };
pub const FOLDERID_SystemX86: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3595710896,
    data2: 45809,
    data3: 18519,
    data4: [164, 206, 168, 231, 198, 234, 125, 39],
};
pub const FOLDERID_Templates: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2788332520, data2: 26190, data3: 18651, data4: [160, 121, 223, 117, 158, 5, 9, 247] };
pub const FOLDERID_UserPinned: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2654573995, data2: 8092, data3: 20243, data4: [184, 39, 72, 178, 75, 108, 113, 116] };
pub const FOLDERID_UserProfiles: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 123916914,
    data2: 50442,
    data3: 19376,
    data4: [163, 130, 105, 125, 205, 114, 155, 128],
};
pub const FOLDERID_UserProgramFiles: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1557638882, data2: 8729, data3: 19047, data4: [184, 93, 108, 156, 225, 86, 96, 203] };
pub const FOLDERID_UserProgramFilesCommon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3166515287, data2: 51804, data3: 17954, data4: [180, 45, 188, 86, 219, 10, 229, 22] };
pub const FOLDERID_UsersFiles: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4090367868, data2: 18689, data3: 19148, data4: [134, 72, 213, 212, 75, 4, 239, 143] };
pub const FOLDERID_UsersLibraries: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2734838877,
    data2: 57087,
    data3: 17995,
    data4: [171, 232, 97, 200, 100, 141, 147, 155],
};
pub const FOLDERID_Videos: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 412654365,
    data2: 39349,
    data3: 17755,
    data4: [132, 28, 171, 124, 116, 228, 221, 252],
};
pub const FOLDERID_VideosLibrary: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1226740271,
    data2: 22083,
    data3: 19188,
    data4: [167, 235, 78, 122, 19, 141, 129, 116],
};
pub const FOLDERID_Windows: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4086035460, data2: 7491, data3: 17138, data4: [147, 5, 103, 222, 11, 40, 252, 35] };
#[repr(C)]
pub struct FOLDERLOGICALVIEWMODE(i32);
#[repr(C)]
pub struct FOLDERSETDATA(i32);
#[repr(C)]
pub struct FOLDERSETTINGS(i32);
pub const FOLDERTYPEID_AccountPictures: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3676986767,
    data2: 1766,
    data3: 16391,
    data4: [171, 166, 175, 135, 125, 82, 110, 166],
};
pub const FOLDERTYPEID_Communications: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2437373925,
    data2: 22635,
    data3: 20154,
    data4: [141, 117, 209, 116, 52, 184, 205, 246],
};
pub const FOLDERTYPEID_CompressedFolder: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2149662338, data2: 48381, data3: 19535, data4: [136, 23, 187, 39, 96, 18, 103, 169] };
pub const FOLDERTYPEID_Contacts: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3727388908,
    data2: 39927,
    data3: 19091,
    data4: [189, 61, 36, 63, 120, 129, 212, 146],
};
pub const FOLDERTYPEID_ControlPanelCategory: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3729720928, data2: 64016, data3: 19343, data4: [164, 148, 6, 139, 32, 178, 35, 7] };
pub const FOLDERTYPEID_ControlPanelClassic: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 204969203, data2: 46405, data3: 17322, data4: [163, 41, 195, 116, 48, 197, 141, 42] };
pub const FOLDERTYPEID_Documents: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2101991206,
    data2: 15393,
    data3: 20229,
    data4: [153, 170, 253, 194, 201, 71, 70, 86],
};
pub const FOLDERTYPEID_Downloads: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2287605870, data2: 42048, data3: 19162, data4: [129, 43, 219, 135, 27, 148, 34, 89] };
pub const FOLDERTYPEID_Games: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3062477008, data2: 30419, data3: 19643, data4: [135, 247, 88, 93, 14, 12, 224, 112] };
pub const FOLDERTYPEID_Generic: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1548691637,
    data2: 63593,
    data3: 20100,
    data4: [142, 96, 241, 29, 185, 124, 92, 199],
};
pub const FOLDERTYPEID_GenericLibrary: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1598991258,
    data2: 26675,
    data3: 20321,
    data4: [137, 157, 49, 207, 70, 151, 157, 73],
};
pub const FOLDERTYPEID_GenericSearchResults: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2145262110,
    data2: 35633,
    data3: 18853,
    data4: [147, 184, 107, 225, 76, 250, 73, 67],
};
pub const FOLDERTYPEID_Invalid: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1468037272, data2: 35919, data3: 17506, data4: [187, 99, 113, 4, 35, 128, 177, 9] };
pub const FOLDERTYPEID_Music: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2497109452, data2: 19048, data3: 16757, data4: [163, 116, 189, 88, 74, 81, 11, 120] };
pub const FOLDERTYPEID_NetworkExplorer: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 634135595, data2: 39548, data3: 20305, data4: [128, 224, 122, 41, 40, 254, 190, 66] };
pub const FOLDERTYPEID_OpenSearch: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2410649129,
    data2: 6528,
    data3: 18175,
    data4: [128, 35, 157, 206, 171, 156, 62, 227],
};
pub const FOLDERTYPEID_OtherUsers: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3006790912,
    data2: 40405,
    data3: 17973,
    data4: [166, 212, 218, 51, 253, 16, 43, 122],
};
pub const FOLDERTYPEID_Pictures: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3010006616,
    data2: 59745,
    data3: 16955,
    data4: [182, 135, 56, 110, 191, 216, 50, 57],
};
pub const FOLDERTYPEID_Printers: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 746307270,
    data2: 51268,
    data3: 18954,
    data4: [145, 250, 206, 246, 245, 156, 253, 161],
};
pub const FOLDERTYPEID_PublishedItems: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2133810070,
    data2: 65396,
    data3: 16858,
    data4: [175, 216, 28, 120, 165, 243, 174, 162],
};
pub const FOLDERTYPEID_RecordedTV: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1431806607,
    data2: 23974,
    data3: 20355,
    data4: [136, 9, 194, 201, 138, 17, 166, 250],
};
pub const FOLDERTYPEID_RecycleBin: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3604602884, data2: 52615, data3: 17451, data4: [157, 87, 94, 10, 235, 79, 111, 114] };
pub const FOLDERTYPEID_SavedGames: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3493212935,
    data2: 10443,
    data3: 16646,
    data4: [159, 35, 41, 86, 227, 229, 224, 231],
};
pub const FOLDERTYPEID_SearchConnector: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2552702446,
    data2: 28487,
    data3: 18334,
    data4: [180, 71, 129, 43, 250, 125, 46, 143],
};
pub const FOLDERTYPEID_SearchHome: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2202896964, data2: 2420, data3: 20182, data4: [134, 110, 242, 3, 216, 11, 56, 16] };
pub const FOLDERTYPEID_Searches: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 185311971, data2: 16479, data3: 16734, data4: [166, 238, 202, 214, 37, 32, 120, 83] };
pub const FOLDERTYPEID_SoftwareExplorer: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3597941019,
    data2: 21209,
    data3: 19975,
    data4: [131, 78, 103, 201, 134, 16, 243, 157],
};
pub const FOLDERTYPEID_StartMenu: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4018648267, data2: 62158, data3: 18309, data4: [134, 88, 76, 166, 198, 62, 56, 198] };
pub const FOLDERTYPEID_StorageProviderDocuments: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3714170214,
    data2: 28904,
    data3: 18653,
    data4: [150, 85, 101, 197, 225, 170, 194, 209],
};
pub const FOLDERTYPEID_StorageProviderGeneric: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1325525957, data2: 9093, data3: 16882, data4: [162, 142, 44, 92, 145, 251, 86, 224] };
pub const FOLDERTYPEID_StorageProviderMusic: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1731120510, data2: 44804, data3: 17305, data4: [135, 92, 2, 144, 132, 91, 98, 71] };
pub const FOLDERTYPEID_StorageProviderPictures: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1909867177,
    data2: 62129,
    data3: 17101,
    data4: [173, 146, 235, 147, 0, 199, 204, 10],
};
pub const FOLDERTYPEID_StorageProviderVideos: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1361661345,
    data2: 55217,
    data3: 18523,
    data4: [158, 154, 23, 207, 254, 51, 225, 135],
};
pub const FOLDERTYPEID_UserFiles: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3440363163,
    data2: 29154,
    data3: 18149,
    data4: [150, 144, 91, 205, 159, 87, 170, 179],
};
pub const FOLDERTYPEID_UsersLibraries: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3302592265, data2: 24868, data3: 20448, data4: [153, 66, 130, 100, 22, 8, 45, 169] };
pub const FOLDERTYPEID_Videos: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1604936711, data2: 32375, data3: 18492, data4: [172, 147, 105, 29, 5, 133, 13, 232] };
#[repr(C)]
pub struct FOLDERVIEWMODE(i32);
#[repr(C)]
pub struct FOLDERVIEWOPTIONS(i32);
#[repr(C)]
pub struct FOLDER_ENUM_MODE(i32);
pub const FO_COPY: u32 = 2u32;
pub const FO_DELETE: u32 = 3u32;
pub const FO_MOVE: u32 = 1u32;
pub const FO_RENAME: u32 = 4u32;
#[repr(C)]
pub struct FSCopyHandler(i32);
pub const FVSIF_CANVIEWIT: u32 = 1073741824u32;
pub const FVSIF_NEWFAILED: u32 = 134217728u32;
pub const FVSIF_NEWFILE: u32 = 2147483648u32;
pub const FVSIF_PINNED: u32 = 2u32;
pub const FVSIF_RECT: u32 = 1u32;
#[repr(C)]
pub struct FVTEXTTYPE(i32);
#[repr(C)]
pub struct FileOpenDialog(i32);
#[repr(C)]
pub struct FileOperation(i32);
#[repr(C)]
pub struct FileSaveDialog(i32);
#[repr(C)]
pub struct FileSearchBand(i32);
#[repr(transparent)]
pub struct Folder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Folder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Folder3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderItemVerb(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderItemVerbs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderItems2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct FolderItems3(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct FolderViewHost(i32);
#[repr(C)]
pub struct FrameworkInputPane(i32);
#[repr(C)]
pub struct FreeSpaceCategorizer(i32);
pub const GADOF_DIRTY: u32 = 1u32;
pub const GCS_HELPTEXT: u32 = 5u32;
pub const GCS_HELPTEXTA: u32 = 1u32;
pub const GCS_HELPTEXTW: u32 = 5u32;
pub const GCS_UNICODE: u32 = 4u32;
pub const GCS_VALIDATE: u32 = 6u32;
pub const GCS_VALIDATEA: u32 = 2u32;
pub const GCS_VALIDATEW: u32 = 6u32;
pub const GCS_VERB: u32 = 4u32;
pub const GCS_VERBA: u32 = 0u32;
pub const GCS_VERBICONW: u32 = 20u32;
pub const GCS_VERBW: u32 = 4u32;
pub const GCT_INVALID: u32 = 0u32;
pub const GCT_LFNCHAR: u32 = 1u32;
pub const GCT_SEPARATOR: u32 = 8u32;
pub const GCT_SHORTCHAR: u32 = 2u32;
pub const GCT_WILD: u32 = 4u32;
pub const GETPROPS_NONE: u32 = 0u32;
pub const GIL_ASYNC: u32 = 32u32;
pub const GIL_CHECKSHIELD: u32 = 512u32;
pub const GIL_DEFAULTICON: u32 = 64u32;
pub const GIL_DONTCACHE: u32 = 16u32;
pub const GIL_FORCENOSHIELD: u32 = 1024u32;
pub const GIL_FORSHELL: u32 = 2u32;
pub const GIL_FORSHORTCUT: u32 = 128u32;
pub const GIL_NOTFILENAME: u32 = 8u32;
pub const GIL_OPENICON: u32 = 1u32;
pub const GIL_PERCLASS: u32 = 4u32;
pub const GIL_PERINSTANCE: u32 = 2u32;
pub const GIL_SHIELD: u32 = 512u32;
pub const GIL_SIMULATEDOC: u32 = 1u32;
pub const GPFIDL_ALTNAME: i32 = 1i32;
pub const GPFIDL_DEFAULT: i32 = 0i32;
pub const GPFIDL_UNCPRINTER: i32 = 2i32;
#[repr(C)]
pub struct GenericCredentialProvider(i32);
#[repr(C)]
pub struct HDROP(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HELPINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HELPWININFOA(i32);
#[repr(C)]
pub struct HELPWININFOW(i32);
#[repr(C)]
pub struct HLBWIF_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HLBWINFO(i32);
#[repr(C)]
pub struct HLFNAMEF(i32);
#[repr(C)]
pub struct HLID_INFO(i32);
#[repr(C)]
pub struct HLINKGETREF(i32);
#[repr(C)]
pub struct HLINKMISC(i32);
#[repr(C)]
pub struct HLINKSETF(i32);
#[repr(C)]
pub struct HLINKWHICHMK(i32);
pub const HLINK_E_FIRST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147221248i32 as _);
pub const HLINK_S_DONTHIDE: i32 = 262400i32;
pub const HLINK_S_FIRST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262400i32 as _);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HLITEM(i32);
#[repr(C)]
pub struct HLNF(i32);
pub const HLNF_ALLOW_AUTONAVIGATE: u32 = 536870912u32;
pub const HLNF_CALLERUNTRUSTED: u32 = 2097152u32;
pub const HLNF_DISABLEWINDOWRESTRICTIONS: u32 = 8388608u32;
pub const HLNF_EXTERNALNAVIGATE: u32 = 268435456u32;
pub const HLNF_NEWWINDOWSMANAGED: u32 = 2147483648u32;
pub const HLNF_TRUSTEDFORACTIVEX: u32 = 4194304u32;
pub const HLNF_TRUSTFIRSTDOWNLOAD: u32 = 16777216u32;
pub const HLNF_UNTRUSTEDFORDOWNLOAD: u32 = 33554432u32;
#[repr(C)]
pub struct HLQF_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HLTBINFO(i32);
#[repr(C)]
pub struct HLTB_INFO(i32);
#[repr(C)]
pub struct HOMEGROUPSHARINGCHOICES(i32);
#[repr(C)]
pub struct HPSXA(i32);
#[repr(C)]
pub struct HideInputPaneAnimationCoordinator(i32);
#[repr(C)]
pub struct HomeGroup(i32);
#[repr(transparent)]
pub struct IACList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IACList2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessibilityDockingService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessibilityDockingServiceCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAccessibleObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActionProgress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IActionProgressDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppActivationUIInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppPublisher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppVisibility(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppVisibilityEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationActivationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationAssociationRegistration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationAssociationRegistrationUI(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationDesignModeSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationDesignModeSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationDestinations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IApplicationDocumentLists(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAssocHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAssocHandlerInvoker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAttachmentExecute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoComplete(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoComplete2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAutoCompleteDropDown(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBandHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBandSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBannerNotificationHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBanneredBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrowserFrameOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrowserService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrowserService2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrowserService3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBrowserService4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICDBurn(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICDBurnExt(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICategorizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICategoryProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColumnManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IColumnProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommDlgBrowser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommDlgBrowser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICommDlgBrowser3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IComputerInfoChangeNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IConnectableCredentialProviderCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContactManagerInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextMenu2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextMenu3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextMenuCB(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContextMenuSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICopyHookA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICopyHookW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreateProcessInputs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICreatingProcess(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderCredential(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderCredential2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderCredentialEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderCredentialEvents2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderCredentialWithFieldOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderSetUserArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderUser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICredentialProviderUserArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentWorkingDirectory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICustomDestinationList(pub *mut ::core::ffi::c_void);
pub const IDC_OFFLINE_HAND: u32 = 103u32;
pub const IDC_PANTOOL_HAND_CLOSED: u32 = 105u32;
pub const IDC_PANTOOL_HAND_OPEN: u32 = 104u32;
pub const IDD_WIZEXTN_FIRST: u32 = 20480u32;
pub const IDD_WIZEXTN_LAST: u32 = 20736u32;
pub const IDO_SHGIOI_DEFAULT: u64 = 4294967292u64;
pub const IDO_SHGIOI_LINK: u32 = 268435454u32;
pub const IDO_SHGIOI_SHARE: u32 = 268435455u32;
pub const IDO_SHGIOI_SLOWFILE: u64 = 4294967293u64;
pub const IDS_DESCRIPTION: u32 = 1u32;
pub const ID_APP: u32 = 100u32;
#[repr(transparent)]
pub struct IDataObjectAsyncCapability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataObjectProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDataTransferManagerInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDefaultExtractIconInit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDefaultFolderMenuInitialize(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDelegateFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDelegateItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeskBand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeskBand2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeskBandInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeskBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDeskBarClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesktopGadget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesktopWallpaper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDestinationStreamFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDocViewSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDockingWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDockingWindowFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDockingWindowSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragSourceHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDragSourceHelper2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDropTargetHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDynamicHWHandler(pub *mut ::core::ffi::c_void);
pub const IEIFLAG_ASPECT: u32 = 4u32;
pub const IEIFLAG_ASYNC: u32 = 1u32;
pub const IEIFLAG_CACHE: u32 = 2u32;
pub const IEIFLAG_GLEAM: u32 = 16u32;
pub const IEIFLAG_NOBORDER: u32 = 256u32;
pub const IEIFLAG_NOSTAMP: u32 = 128u32;
pub const IEIFLAG_OFFLINE: u32 = 8u32;
pub const IEIFLAG_ORIGSIZE: u32 = 64u32;
pub const IEIFLAG_QUALITY: u32 = 512u32;
pub const IEIFLAG_REFRESH: u32 = 1024u32;
pub const IEIFLAG_SCREEN: u32 = 32u32;
pub const IEIT_PRIORITY_NORMAL: u32 = 268435456u32;
pub const IEI_PRIORITY_MAX: u32 = 2147483647u32;
pub const IEI_PRIORITY_MIN: u32 = 0u32;
#[repr(C)]
pub struct IENamespaceTreeControl(i32);
#[repr(C)]
pub struct IEPDNFLAGS(i32);
#[repr(C)]
pub struct IESHORTCUTFLAGS(i32);
#[repr(transparent)]
pub struct IEnumACString(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumAssocHandlers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumExplorerCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumExtraSearch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumFullIDList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumHLITEM(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumIDList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumObjects(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumPublishedApps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumReadyCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumResources(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumShellItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSyncMgrConflict(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSyncMgrEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumSyncMgrSyncItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumTravelLogEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumerableView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExecuteCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExecuteCommandApplicationHostEnvironment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExecuteCommandHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExpDispSupport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExpDispSupportXP(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExplorerBrowser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExplorerBrowserEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExplorerCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExplorerCommandProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExplorerCommandState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExplorerPaneVisibility(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtensionServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtractIconA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtractIconW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtractImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IExtractImage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileDialog2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileDialogControlEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileDialogCustomize(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileDialogEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileIsInUse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOpenDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOperation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileOperationProgressSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSaveDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSearchBand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSyncMergeHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSystemBindData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileSystemBindData2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderBandPriv(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderFilterSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderViewHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderViewOC(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderViewOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFolderViewSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkInputPane(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFrameworkInputPaneHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetServiceIds(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHWEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHWEventHandler2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandlerActivationHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandlerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandlerInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHlink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHlinkBrowseContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHlinkFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHlinkSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHlinkTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHomeGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIOCancelInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIdentityName(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IImageRecompress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitializeCommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitializeNetworkFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitializeObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitializeWithBindCtx(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitializeWithItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitializeWithPropertyStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInitializeWithWindow(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputObject2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputObjectSite(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPaneAnimationCoordinator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPanelConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPanelInvocationConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInsertItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IItemNameLimits(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownFolderManager(pub *mut ::core::ffi::c_void);
pub const ILMM_IE4: u32 = 0u32;
#[repr(transparent)]
pub struct ILaunchSourceAppUserModelId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILaunchSourceViewSizePreference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILaunchTargetMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILaunchTargetViewSizePreference(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILaunchUIContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILaunchUIContextProvider(pub *mut ::core::ffi::c_void);
pub const IMM_ACC_DOCKING_E_DOCKOCCUPIED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927183i32 as _);
pub const IMM_ACC_DOCKING_E_INSUFFICIENTHEIGHT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927184i32 as _);
pub const IMSC_E_SHELL_COMPONENT_STARTUP_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927181i32 as _);
#[repr(transparent)]
pub struct IMenuBand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMenuPopup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IModalWindow(pub *mut ::core::ffi::c_void);
pub const INTERNET_MAX_PATH_LENGTH: u32 = 2048u32;
pub const INTERNET_MAX_SCHEME_LENGTH: u32 = 32u32;
#[repr(transparent)]
pub struct INameSpaceTreeAccessible(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INameSpaceTreeControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INameSpaceTreeControl2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INameSpaceTreeControlCustomDraw(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INameSpaceTreeControlDropHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INameSpaceTreeControlEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INameSpaceTreeControlFolderCapabilities(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INamedPropertyBag(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INamespaceWalk(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INamespaceWalkCB(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INamespaceWalkCB2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetworkFolderInternal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INewMenuClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INewShortcutHookA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INewShortcutHookW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INewWDEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INewWindowManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INotifyReplica(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjMgr(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectWithAppUserModelID(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectWithBackReferences(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectWithCancelEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectWithFolderEnumMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectWithProgID(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IObjectWithSelection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenControlPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOpenSearchSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOperationsProgressDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageDebugSettings2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPackageExecutionStateChangeNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IParentAndItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IParseAndCreateItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistFolder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistFolder3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPersistIDList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPreviewHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPreviewHandlerFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPreviewHandlerVisuals(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPreviewItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPreviousVersionsInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProfferService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IProgressDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPropertyKeyStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPublishedApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPublishedApp2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPublishingWizard(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryAssociations(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryCancelAutoPlay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryCodePage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryContinue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryContinueWithStatus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IQueryInfo(pub *mut ::core::ffi::c_void);
pub const IRTIR_TASK_FINISHED: u32 = 4u32;
pub const IRTIR_TASK_NOT_RUNNING: u32 = 0u32;
pub const IRTIR_TASK_PENDING: u32 = 3u32;
pub const IRTIR_TASK_RUNNING: u32 = 1u32;
pub const IRTIR_TASK_SUSPENDED: u32 = 2u32;
#[repr(transparent)]
pub struct IRegTreeItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRelatedItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRemoteComputer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResolveShellLink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResultsFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRunnableTask(pub *mut ::core::ffi::c_void);
pub const ISFBVIEWMODE_LARGEICONS: u32 = 2u32;
pub const ISFBVIEWMODE_LOGOS: u32 = 3u32;
pub const ISFBVIEWMODE_SMALLICONS: u32 = 1u32;
pub const ISFB_MASK_BKCOLOR: u32 = 2u32;
pub const ISFB_MASK_COLORS: u32 = 32u32;
pub const ISFB_MASK_IDLIST: u32 = 16u32;
pub const ISFB_MASK_SHELLFOLDER: u32 = 8u32;
pub const ISFB_MASK_STATE: u32 = 1u32;
pub const ISFB_MASK_VIEWMODE: u32 = 4u32;
pub const ISFB_STATE_ALLOWRENAME: u32 = 2u32;
pub const ISFB_STATE_BTNMINSIZE: u32 = 256u32;
pub const ISFB_STATE_CHANNELBAR: u32 = 16u32;
pub const ISFB_STATE_DEBOSSED: u32 = 1u32;
pub const ISFB_STATE_DEFAULT: u32 = 0u32;
pub const ISFB_STATE_FULLOPEN: u32 = 64u32;
pub const ISFB_STATE_NONAMESORT: u32 = 128u32;
pub const ISFB_STATE_NOSHOWTEXT: u32 = 4u32;
pub const ISFB_STATE_QLINKSMODE: u32 = 32u32;
pub const ISHCUTCMDID_COMMITHISTORY: i32 = 2i32;
pub const ISHCUTCMDID_DOWNLOADICON: i32 = 0i32;
pub const ISHCUTCMDID_INTSHORTCUTCREATE: i32 = 1i32;
pub const ISHCUTCMDID_SETUSERAWURL: i32 = 3i32;
pub const ISIOI_ICONFILE: u32 = 1u32;
pub const ISIOI_ICONINDEX: u32 = 2u32;
pub const IS_E_EXEC_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147213310i32 as _);
pub const IS_FULLSCREEN: u32 = 2u32;
pub const IS_NORMAL: u32 = 1u32;
pub const IS_SPLIT: u32 = 4u32;
#[repr(transparent)]
pub struct IScriptErrorList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchBoxInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISearchFolderItemFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedBitmap(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharingConfigurationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellBrowser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellChangeNotify(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellDispatch(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellDispatch2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellDispatch3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellDispatch4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellDispatch5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellDispatch6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellExtInit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellFavoritesNameSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellFolder2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellFolderBand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellFolderView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellFolderViewCB(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellFolderViewDual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellFolderViewDual2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellFolderViewDual3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellIconOverlay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellIconOverlayIdentifier(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellIconOverlayManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellImageData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellImageDataAbort(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellImageDataFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellItem2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellItemArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellItemFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellItemImageFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellItemResources(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellLibrary(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellLinkA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellLinkDataList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellLinkDual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellLinkDual2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellLinkW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellMenuCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellNameSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellPropSheetExt(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellRunDll(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellTaskScheduler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellUIHelper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellUIHelper2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellUIHelper3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellUIHelper4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellUIHelper5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellUIHelper6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellUIHelper7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellUIHelper8(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellUIHelper9(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellView2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellView3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IShellWindows(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISortColumnArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStartMenuPinnedList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderBanners(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderCopyHook(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStorageProviderPropertyHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamAsync(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStreamUnbufferedInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISuspensionDependencyManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrConflict(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrConflictFolder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrConflictItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrConflictPresenter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrConflictResolutionItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrConflictResolveInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrConflictStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrEnumItems(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrEventLinkUIOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrEventStore(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrHandlerCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrHandlerInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrRegister(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrResolutionHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrScheduleWizardUIOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrSessionCreator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrSyncCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrSyncItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrSyncItemContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrSyncItemInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrSyncResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrSynchronize(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrSynchronizeCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrSynchronizeInvoke(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISyncMgrUIOperation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ITEMSPACING(i32);
pub const ITSAT_DEFAULT_PRIORITY: u32 = 268435456u32;
pub const ITSAT_MAX_PRIORITY: u32 = 2147483647u32;
pub const ITSAT_MIN_PRIORITY: u32 = 0u32;
pub const ITSSFLAG_COMPLETE_ON_DESTROY: u32 = 0u32;
pub const ITSSFLAG_FLAGS_MASK: u32 = 3u32;
pub const ITSSFLAG_KILL_ON_DESTROY: u32 = 1u32;
#[repr(transparent)]
pub struct ITaskbarList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskbarList2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskbarList3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITaskbarList4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbnailCache(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbnailCachePrimer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbnailCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbnailHandlerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbnailProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbnailSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IThumbnailStreamCache(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITrackShellMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITranscodeImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransferAdviseSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransferDestination(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransferMediumItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITransferSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITravelEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITravelLog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITravelLogClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITravelLogEntry(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITravelLogStg(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITrayDeskBand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IURLSearchHook(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IURLSearchHook2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUniformResourceLocatorA(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUniformResourceLocatorW(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUpdateIDList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUseToBrowseItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserAccountChangeCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotification2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUserNotificationCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IViewStateIdentityItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualDesktopManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVisualProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebBrowser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebBrowser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebBrowserApp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebWizardExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebWizardHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebWizardHost2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWizardExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWizardSite(pub *mut ::core::ffi::c_void);
pub const Identity_LocalUserProvider: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2711114395, data2: 29455, data3: 16521, data4: [182, 70, 161, 37, 87, 245, 102, 94] };
#[repr(C)]
pub struct ImageProperties(i32);
#[repr(C)]
pub struct ImageRecompress(i32);
#[repr(C)]
pub struct ImageTranscode(i32);
#[repr(C)]
pub struct InputPanelConfiguration(i32);
#[repr(C)]
pub struct InternetExplorer(i32);
#[repr(C)]
pub struct InternetExplorerMedium(i32);
#[repr(C)]
pub struct InternetPrintOrdering(i32);
pub const ItemCount_Property_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2881444933,
    data2: 23756,
    data3: 18359,
    data4: [187, 78, 135, 203, 135, 187, 209, 98],
};
pub const ItemIndex_Property_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2459980762, data2: 10601, data3: 16417, data4: [191, 39, 81, 76, 252, 46, 74, 105] };
#[repr(C)]
pub struct KF_CATEGORY(i32);
#[repr(C)]
pub struct KNOWNDESTCATEGORY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct KNOWNFOLDER_DEFINITION(i32);
#[repr(C)]
pub struct KNOWN_FOLDER_FLAG(i32);
#[repr(C)]
pub struct KnownFolderManager(i32);
#[repr(C)]
pub struct LIBRARYFOLDERFILTER(i32);
#[repr(C)]
pub struct LIBRARYMANAGEDIALOGOPTIONS(i32);
#[repr(C)]
pub struct LIBRARYOPTIONFLAGS(i32);
#[repr(C)]
pub struct LIBRARYSAVEFLAGS(i32);
pub const LIBRARY_E_NO_ACCESSIBLE_LOCATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927231i32 as _);
pub const LIBRARY_E_NO_SAVE_LOCATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927232i32 as _);
pub const LINK_E_DELETE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927485i32 as _);
#[repr(C)]
pub struct LPFNDFMCALLBACK(i32);
#[repr(C)]
pub struct LPFNVIEWCALLBACK(i32);
#[repr(C)]
pub struct LocalThumbnailCache(i32);
pub const MAXFILELEN: u32 = 13u32;
pub const MAX_COLUMN_DESC_LEN: u32 = 128u32;
pub const MAX_COLUMN_NAME_LEN: u32 = 80u32;
pub const MAX_SYNCMGRHANDLERNAME: u32 = 32u32;
pub const MAX_SYNCMGRITEMNAME: u32 = 128u32;
pub const MAX_SYNCMGR_ID: u32 = 64u32;
pub const MAX_SYNCMGR_NAME: u32 = 128u32;
pub const MAX_SYNCMGR_PROGRESSTEXT: u32 = 260u32;
#[repr(C)]
pub struct MENUBANDHANDLERCID(i32);
#[repr(C)]
pub struct MENUPOPUPPOPUPFLAGS(i32);
#[repr(C)]
pub struct MENUPOPUPSELECT(i32);
#[repr(C)]
pub struct MERGE_UPDATE_STATUS(i32);
#[repr(C)]
pub struct MM_FLAGS(i32);
#[repr(C)]
pub struct MONITOR_APP_VISIBILITY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MULTIKEYHELPA(i32);
#[repr(C)]
pub struct MULTIKEYHELPW(i32);
#[repr(C)]
pub struct MailRecipient(i32);
#[repr(C)]
pub struct MergedCategorizer(i32);
#[repr(C)]
pub struct NAMESPACEWALKFLAG(i32);
#[repr(C)]
pub struct NATIVE_DISPLAY_ORIENTATION(i32);
pub const NCM_DISPLAYERRORTIP: u32 = 1028u32;
pub const NCM_GETADDRESS: u32 = 1025u32;
pub const NCM_GETALLOWTYPE: u32 = 1027u32;
pub const NCM_SETALLOWTYPE: u32 = 1026u32;
#[repr(C)]
pub struct NC_ADDRESS(i32);
pub const NETCACHE_E_NEGATIVE_CACHE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927488i32 as _);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct NEWCPLINFOA(i32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct NEWCPLINFOW(i32);
pub const NIIF_ERROR: u32 = 3u32;
pub const NIIF_ICON_MASK: u32 = 15u32;
pub const NIIF_INFO: u32 = 1u32;
pub const NIIF_LARGE_ICON: u32 = 32u32;
pub const NIIF_NONE: u32 = 0u32;
pub const NIIF_NOSOUND: u32 = 16u32;
pub const NIIF_RESPECT_QUIET_TIME: u32 = 128u32;
pub const NIIF_USER: u32 = 4u32;
pub const NIIF_WARNING: u32 = 2u32;
pub const NINF_KEY: u32 = 1u32;
pub const NIN_BALLOONHIDE: u32 = 1027u32;
pub const NIN_BALLOONSHOW: u32 = 1026u32;
pub const NIN_BALLOONTIMEOUT: u32 = 1028u32;
pub const NIN_BALLOONUSERCLICK: u32 = 1029u32;
pub const NIN_POPUPCLOSE: u32 = 1031u32;
pub const NIN_POPUPOPEN: u32 = 1030u32;
pub const NIN_SELECT: u32 = 1024u32;
pub const NIS_HIDDEN: u32 = 1u32;
pub const NIS_SHAREDICON: u32 = 2u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct NOTIFYICONDATAA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct NOTIFYICONDATAA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct NOTIFYICONDATAW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct NOTIFYICONDATAW(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NOTIFYICONIDENTIFIER(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NOTIFYICONIDENTIFIER(i32);
pub const NOTIFYICON_VERSION: u32 = 3u32;
pub const NOTIFYICON_VERSION_4: u32 = 4u32;
#[repr(C)]
pub struct NOTIFY_ICON_DATA_FLAGS(i32);
#[repr(C)]
pub struct NOTIFY_ICON_MESSAGE(i32);
#[repr(C)]
pub struct NPCredentialProvider(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_WNet"))]
#[repr(C)]
pub struct NRESARRAY(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[repr(C)]
pub struct NSTCCUSTOMDRAW(i32);
pub const NSTCDHPOS_ONTOP: i32 = -1i32;
#[repr(C)]
pub struct NSTCFOLDERCAPABILITIES(i32);
#[repr(C)]
pub struct NSTCGNI(i32);
#[repr(C)]
pub struct NSTCSTYLE2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Console"))]
#[repr(C)]
pub struct NT_CONSOLE_PROPS(i32);
pub const NT_CONSOLE_PROPS_SIG: u32 = 2684354562u32;
#[repr(C)]
pub struct NT_FE_CONSOLE_PROPS(i32);
pub const NT_FE_CONSOLE_PROPS_SIG: u32 = 2684354564u32;
pub const NUM_POINTS: u32 = 3u32;
#[repr(C)]
pub struct NWMF(i32);
#[repr(C)]
pub struct NamespaceTreeControl(i32);
#[repr(C)]
pub struct NamespaceWalker(i32);
#[repr(C)]
pub struct NetworkConnections(i32);
#[repr(C)]
pub struct NetworkExplorerFolder(i32);
#[repr(C)]
pub struct NetworkPlaces(i32);
#[repr(C)]
pub struct NewProcessCauseConstants(i32);
pub const OFASI_EDIT: u32 = 1u32;
pub const OFASI_OPENDESKTOP: u32 = 2u32;
pub const OFFLINE_STATUS_INCOMPLETE: u32 = 4u32;
pub const OFFLINE_STATUS_LOCAL: u32 = 1u32;
pub const OFFLINE_STATUS_REMOTE: u32 = 2u32;
pub const OF_CAP_CANCLOSE: u32 = 2u32;
pub const OF_CAP_CANSWITCHTO: u32 = 1u32;
pub const OI_ASYNC: u32 = 4294962926u32;
pub const OI_DEFAULT: u32 = 0u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPENASINFO(i32);
pub const OPENPROPS_INHIBITPIF: u32 = 32768u32;
pub const OPENPROPS_NONE: u32 = 0u32;
#[repr(C)]
pub struct OPEN_AS_INFO_FLAGS(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPEN_PRINTER_PROPS_INFOA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPEN_PRINTER_PROPS_INFOA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPEN_PRINTER_PROPS_INFOW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct OPEN_PRINTER_PROPS_INFOW(i32);
#[repr(C)]
pub struct OS(i32);
#[repr(C)]
pub struct OfflineFolderStatus(i32);
#[repr(C)]
pub struct OnexCredentialProvider(i32);
#[repr(C)]
pub struct OnexPlapSmartcardCredentialProvider(i32);
#[repr(C)]
pub struct OpenControlPanel(i32);
#[repr(C)]
pub struct PACKAGE_EXECUTION_STATE(i32);
pub const PANE_NAVIGATION: u32 = 5u32;
pub const PANE_NONE: u32 = 4294967295u32;
pub const PANE_OFFLINE: u32 = 2u32;
pub const PANE_PRINTER: u32 = 3u32;
pub const PANE_PRIVACY: u32 = 7u32;
pub const PANE_PROGRESS: u32 = 6u32;
pub const PANE_SSL: u32 = 4u32;
pub const PANE_ZONE: u32 = 1u32;
#[repr(C)]
pub struct PAPPCONSTRAIN_CHANGE_ROUTINE(i32);
#[repr(C)]
pub struct PAPPSTATE_CHANGE_ROUTINE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PARSEDURLA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PARSEDURLW(i32);
pub const PATHCCH_MAX_CCH: u32 = 32768u32;
#[repr(C)]
pub struct PATHCCH_OPTIONS(i32);
#[repr(C)]
pub struct PCS_RET(i32);
pub const PDTIMER_PAUSE: u32 = 2u32;
pub const PDTIMER_RESET: u32 = 1u32;
pub const PDTIMER_RESUME: u32 = 3u32;
#[cfg(feature = "Win32_UI_Shell_Common")]
#[repr(C)]
pub struct PERSIST_FOLDER_TARGET_INFO(i32);
#[repr(C)]
pub struct PFNCANSHAREFOLDERW(i32);
#[repr(C)]
pub struct PFNSHOWSHAREFOLDERUIW(i32);
pub const PIDASI_AVG_DATA_RATE: u32 = 4u32;
pub const PIDASI_CHANNEL_COUNT: u32 = 7u32;
pub const PIDASI_COMPRESSION: u32 = 10u32;
pub const PIDASI_FORMAT: u32 = 2u32;
pub const PIDASI_SAMPLE_RATE: u32 = 5u32;
pub const PIDASI_SAMPLE_SIZE: u32 = 6u32;
pub const PIDASI_STREAM_NAME: u32 = 9u32;
pub const PIDASI_STREAM_NUMBER: u32 = 8u32;
pub const PIDASI_TIMELENGTH: u32 = 3u32;
pub const PIDDRSI_DESCRIPTION: u32 = 3u32;
pub const PIDDRSI_PLAYCOUNT: u32 = 4u32;
pub const PIDDRSI_PLAYEXPIRES: u32 = 6u32;
pub const PIDDRSI_PLAYSTARTS: u32 = 5u32;
pub const PIDDRSI_PROTECTED: u32 = 2u32;
#[repr(C)]
pub struct PIDISF_FLAGS(i32);
#[repr(C)]
pub struct PIDISM_OPTIONS(i32);
#[repr(C)]
pub struct PIDISR_INFO(i32);
pub const PIDSI_ALBUM: u32 = 4u32;
pub const PIDSI_ARTIST: u32 = 2u32;
pub const PIDSI_COMMENT: u32 = 6u32;
pub const PIDSI_GENRE: u32 = 11u32;
pub const PIDSI_LYRICS: u32 = 12u32;
pub const PIDSI_SONGTITLE: u32 = 3u32;
pub const PIDSI_TRACK: u32 = 7u32;
pub const PIDSI_YEAR: u32 = 5u32;
pub const PIDVSI_COMPRESSION: u32 = 10u32;
pub const PIDVSI_DATA_RATE: u32 = 8u32;
pub const PIDVSI_FRAME_COUNT: u32 = 5u32;
pub const PIDVSI_FRAME_HEIGHT: u32 = 4u32;
pub const PIDVSI_FRAME_RATE: u32 = 6u32;
pub const PIDVSI_FRAME_WIDTH: u32 = 3u32;
pub const PIDVSI_SAMPLE_SIZE: u32 = 9u32;
pub const PIDVSI_STREAM_NAME: u32 = 2u32;
pub const PIDVSI_STREAM_NUMBER: u32 = 11u32;
pub const PIDVSI_TIMELENGTH: u32 = 7u32;
pub const PID_COMPUTERNAME: u32 = 5u32;
pub const PID_CONTROLPANEL_CATEGORY: u32 = 2u32;
pub const PID_DESCRIPTIONID: u32 = 2u32;
pub const PID_DISPLACED_DATE: u32 = 3u32;
pub const PID_DISPLACED_FROM: u32 = 2u32;
pub const PID_DISPLAY_PROPERTIES: u32 = 0u32;
pub const PID_FINDDATA: u32 = 0u32;
pub const PID_HTMLINFOTIPFILE: u32 = 5u32;
pub const PID_INTROTEXT: u32 = 1u32;
#[repr(C)]
pub struct PID_INTSITE(i32);
#[repr(C)]
pub struct PID_IS(i32);
pub const PID_LINK_TARGET: u32 = 2u32;
pub const PID_LINK_TARGET_TYPE: u32 = 3u32;
pub const PID_MISC_ACCESSCOUNT: u32 = 3u32;
pub const PID_MISC_OWNER: u32 = 4u32;
pub const PID_MISC_PICS: u32 = 6u32;
pub const PID_MISC_STATUS: u32 = 2u32;
pub const PID_NETRESOURCE: u32 = 1u32;
pub const PID_NETWORKLOCATION: u32 = 4u32;
pub const PID_QUERY_RANK: u32 = 2u32;
pub const PID_SHARE_CSC_STATUS: u32 = 2u32;
pub const PID_SYNC_COPY_IN: u32 = 2u32;
pub const PID_VOLUME_CAPACITY: u32 = 3u32;
pub const PID_VOLUME_FILESYSTEM: u32 = 4u32;
pub const PID_VOLUME_FREE: u32 = 2u32;
pub const PID_WHICHFOLDER: u32 = 3u32;
pub const PIFDEFFILESIZE: u32 = 80u32;
pub const PIFDEFPATHSIZE: u32 = 64u32;
pub const PIFMAXFILEPATH: u32 = 260u32;
pub const PIFNAMESIZE: u32 = 30u32;
pub const PIFPARAMSSIZE: u32 = 64u32;
pub const PIFSHDATASIZE: u32 = 64u32;
pub const PIFSHPROGSIZE: u32 = 64u32;
pub const PIFSTARTLOCSIZE: u32 = 63u32;
#[repr(C)]
pub struct PINLogonCredentialProvider(i32);
pub const PLATFORM_BROWSERONLY: u32 = 1u32;
pub const PLATFORM_IE3: u32 = 1u32;
pub const PLATFORM_INTEGRATED: u32 = 2u32;
pub const PLATFORM_UNKNOWN: u32 = 0u32;
pub const PMSF_DONT_STRIP_SPACES: u32 = 65536u32;
pub const PMSF_MULTIPLE: u32 = 1u32;
pub const PMSF_NORMAL: u32 = 0u32;
pub const PO_DELETE: u32 = 19u32;
pub const PO_PORTCHANGE: u32 = 32u32;
pub const PO_RENAME: u32 = 20u32;
pub const PO_REN_PORT: u32 = 52u32;
pub const PPCF_ADDARGUMENTS: u32 = 3u32;
pub const PPCF_ADDQUOTES: u32 = 1u32;
pub const PPCF_FORCEQUALIFY: u32 = 64u32;
pub const PPCF_LONGESTPOSSIBLE: u32 = 128u32;
pub const PPCF_NODIRECTORIES: u32 = 16u32;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct PREVIEWHANDLERFRAMEINFO(i32);
#[repr(C)]
pub struct PRF_FLAGS(i32);
pub const PRINTACTION_DOCUMENTDEFAULTS: u32 = 6u32;
pub const PRINTACTION_NETINSTALL: u32 = 2u32;
pub const PRINTACTION_NETINSTALLLINK: u32 = 3u32;
pub const PRINTACTION_OPEN: u32 = 0u32;
pub const PRINTACTION_OPENNETPRN: u32 = 5u32;
pub const PRINTACTION_PROPERTIES: u32 = 1u32;
pub const PRINTACTION_SERVERPROPERTIES: u32 = 7u32;
pub const PRINTACTION_TESTPAGE: u32 = 4u32;
pub const PRINT_PROP_FORCE_NAME: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROFILEINFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROFILEINFOW(i32);
pub const PROGDLG_AUTOTIME: u32 = 2u32;
pub const PROGDLG_MARQUEEPROGRESS: u32 = 32u32;
pub const PROGDLG_MODAL: u32 = 1u32;
pub const PROGDLG_NOCANCEL: u32 = 64u32;
pub const PROGDLG_NOMINIMIZE: u32 = 8u32;
pub const PROGDLG_NOPROGRESSBAR: u32 = 16u32;
pub const PROGDLG_NORMAL: u32 = 0u32;
pub const PROGDLG_NOTIME: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PUBAPPINFO(i32);
#[repr(C)]
pub struct PUBAPPINFOFLAGS(i32);
#[repr(C)]
pub struct PackageDebugSettings(i32);
#[repr(C)]
pub struct PasswordCredentialProvider(i32);
#[repr(C)]
pub struct PreviousVersions(i32);
#[repr(C)]
pub struct PropertiesUI(i32);
#[repr(C)]
pub struct PublishDropTarget(i32);
#[repr(C)]
pub struct PublishingWizard(i32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct QCMINFO(i32);
#[repr(C)]
pub struct QCMINFO_IDMAP(i32);
#[repr(C)]
pub struct QCMINFO_IDMAP_PLACEMENT(i32);
pub const QCMINFO_PLACE_AFTER: u32 = 1u32;
pub const QCMINFO_PLACE_BEFORE: u32 = 0u32;
#[repr(C)]
pub struct QITAB(i32);
#[repr(C)]
pub struct QITIPF_FLAGS(i32);
#[repr(C)]
pub struct QUERY_USER_NOTIFICATION_STATE(i32);
#[repr(C)]
pub struct QueryCancelAutoPlay(i32);
#[repr(C)]
pub struct RASProvider(i32);
#[repr(C)]
pub struct RESTRICTIONS(i32);
#[repr(C)]
pub struct RefreshConstants(i32);
pub const SBSP_ABSOLUTE: u32 = 0u32;
pub const SBSP_ACTIVATE_NOFOCUS: u32 = 524288u32;
pub const SBSP_ALLOW_AUTONAVIGATE: u32 = 65536u32;
pub const SBSP_CALLERUNTRUSTED: u32 = 8388608u32;
pub const SBSP_CREATENOHISTORY: u32 = 1048576u32;
pub const SBSP_DEFBROWSER: u32 = 0u32;
pub const SBSP_DEFMODE: u32 = 0u32;
pub const SBSP_EXPLOREMODE: u32 = 32u32;
pub const SBSP_FEEDNAVIGATION: u32 = 536870912u32;
pub const SBSP_HELPMODE: u32 = 64u32;
pub const SBSP_INITIATEDBYHLINKFRAME: u32 = 2147483648u32;
pub const SBSP_KEEPSAMETEMPLATE: u32 = 131072u32;
pub const SBSP_KEEPWORDWHEELTEXT: u32 = 262144u32;
pub const SBSP_NAVIGATEBACK: u32 = 16384u32;
pub const SBSP_NAVIGATEFORWARD: u32 = 32768u32;
pub const SBSP_NEWBROWSER: u32 = 2u32;
pub const SBSP_NOAUTOSELECT: u32 = 67108864u32;
pub const SBSP_NOTRANSFERHIST: u32 = 128u32;
pub const SBSP_OPENMODE: u32 = 16u32;
pub const SBSP_PARENT: u32 = 8192u32;
pub const SBSP_PLAYNOSOUND: u32 = 2097152u32;
pub const SBSP_REDIRECT: u32 = 1073741824u32;
pub const SBSP_RELATIVE: u32 = 4096u32;
pub const SBSP_SAMEBROWSER: u32 = 1u32;
pub const SBSP_TRUSTEDFORACTIVEX: u32 = 268435456u32;
pub const SBSP_TRUSTFIRSTDOWNLOAD: u32 = 16777216u32;
pub const SBSP_UNTRUSTEDFORDOWNLOAD: u32 = 33554432u32;
pub const SBSP_WRITENOHISTORY: u32 = 134217728u32;
#[repr(C)]
pub struct SCALE_CHANGE_FLAGS(i32);
pub const SCHEME_CREATE: u32 = 128u32;
pub const SCHEME_DISPLAY: u32 = 1u32;
pub const SCHEME_DONOTUSE: u32 = 64u32;
pub const SCHEME_EDIT: u32 = 2u32;
pub const SCHEME_GLOBAL: u32 = 8u32;
pub const SCHEME_LOCAL: u32 = 4u32;
pub const SCHEME_REFRESH: u32 = 16u32;
pub const SCHEME_UPDATE: u32 = 32u32;
#[repr(C)]
pub struct SCNRT_STATUS(i32);
pub const SCRM_VERIFYPW: u32 = 32768u32;
#[repr(C)]
pub struct SECURELOCKCODE(i32);
pub const SEE_MASK_ASYNCOK: u32 = 1048576u32;
pub const SEE_MASK_CLASSKEY: u32 = 3u32;
pub const SEE_MASK_CLASSNAME: u32 = 1u32;
pub const SEE_MASK_CONNECTNETDRV: u32 = 128u32;
pub const SEE_MASK_DEFAULT: u32 = 0u32;
pub const SEE_MASK_DOENVSUBST: u32 = 512u32;
pub const SEE_MASK_FLAG_DDEWAIT: u32 = 256u32;
pub const SEE_MASK_FLAG_HINST_IS_SITE: u32 = 134217728u32;
pub const SEE_MASK_FLAG_LOG_USAGE: u32 = 67108864u32;
pub const SEE_MASK_FLAG_NO_UI: u32 = 1024u32;
pub const SEE_MASK_HMONITOR: u32 = 2097152u32;
pub const SEE_MASK_HOTKEY: u32 = 32u32;
pub const SEE_MASK_ICON: u32 = 16u32;
pub const SEE_MASK_IDLIST: u32 = 4u32;
pub const SEE_MASK_INVOKEIDLIST: u32 = 12u32;
pub const SEE_MASK_NOASYNC: u32 = 256u32;
pub const SEE_MASK_NOCLOSEPROCESS: u32 = 64u32;
pub const SEE_MASK_NOQUERYCLASSSTORE: u32 = 16777216u32;
pub const SEE_MASK_NOZONECHECKS: u32 = 8388608u32;
pub const SEE_MASK_NO_CONSOLE: u32 = 32768u32;
pub const SEE_MASK_UNICODE: u32 = 16384u32;
pub const SEE_MASK_WAITFORINPUTIDLE: u32 = 33554432u32;
pub const SETPROPS_NONE: u32 = 0u32;
pub const SE_ERR_ACCESSDENIED: u32 = 5u32;
pub const SE_ERR_ASSOCINCOMPLETE: u32 = 27u32;
pub const SE_ERR_DDEBUSY: u32 = 30u32;
pub const SE_ERR_DDEFAIL: u32 = 29u32;
pub const SE_ERR_DDETIMEOUT: u32 = 28u32;
pub const SE_ERR_DLLNOTFOUND: u32 = 32u32;
pub const SE_ERR_FNF: u32 = 2u32;
pub const SE_ERR_NOASSOC: u32 = 31u32;
pub const SE_ERR_OOM: u32 = 8u32;
pub const SE_ERR_PNF: u32 = 3u32;
pub const SE_ERR_SHARE: u32 = 26u32;
pub const SFBID_PIDLCHANGED: i32 = 0i32;
#[repr(C)]
pub struct SFBS_FLAGS(i32);
pub const SFGAO_BROWSABLE: i32 = 134217728i32;
pub const SFGAO_CANCOPY: u32 = 1u32;
pub const SFGAO_CANDELETE: i32 = 32i32;
pub const SFGAO_CANLINK: u32 = 4u32;
pub const SFGAO_CANMONIKER: i32 = 4194304i32;
pub const SFGAO_CANMOVE: u32 = 2u32;
pub const SFGAO_CANRENAME: i32 = 16i32;
pub const SFGAO_CAPABILITYMASK: i32 = 375i32;
pub const SFGAO_COMPRESSED: i32 = 67108864i32;
pub const SFGAO_CONTENTSMASK: i32 = -2147483648i32;
pub const SFGAO_DISPLAYATTRMASK: i32 = 1032192i32;
pub const SFGAO_DROPTARGET: i32 = 256i32;
pub const SFGAO_ENCRYPTED: i32 = 8192i32;
pub const SFGAO_FILESYSANCESTOR: i32 = 268435456i32;
pub const SFGAO_FILESYSTEM: i32 = 1073741824i32;
pub const SFGAO_FOLDER: i32 = 536870912i32;
pub const SFGAO_GHOSTED: i32 = 32768i32;
pub const SFGAO_HASPROPSHEET: i32 = 64i32;
pub const SFGAO_HASSTORAGE: i32 = 4194304i32;
pub const SFGAO_HASSUBFOLDER: i32 = -2147483648i32;
pub const SFGAO_HIDDEN: i32 = 524288i32;
pub const SFGAO_ISSLOW: i32 = 16384i32;
pub const SFGAO_LINK: i32 = 65536i32;
pub const SFGAO_NEWCONTENT: i32 = 2097152i32;
pub const SFGAO_NONENUMERATED: i32 = 1048576i32;
pub const SFGAO_PKEYSFGAOMASK: i32 = -2130427904i32;
pub const SFGAO_PLACEHOLDER: i32 = 2048i32;
pub const SFGAO_READONLY: i32 = 262144i32;
pub const SFGAO_REMOVABLE: i32 = 33554432i32;
pub const SFGAO_SHARE: i32 = 131072i32;
pub const SFGAO_STORAGE: i32 = 8i32;
pub const SFGAO_STORAGEANCESTOR: i32 = 8388608i32;
pub const SFGAO_STORAGECAPMASK: i32 = 1891958792i32;
pub const SFGAO_STREAM: i32 = 4194304i32;
pub const SFGAO_SYSTEM: i32 = 4096i32;
pub const SFGAO_VALIDATE: i32 = 16777216i32;
pub const SFVM_ADDOBJECT: u32 = 3u32;
pub const SFVM_GETSELECTEDOBJECTS: u32 = 9u32;
#[repr(C)]
pub struct SFVM_HELPTOPIC_DATA(i32);
#[repr(C)]
pub struct SFVM_MESSAGE_ID(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[repr(C)]
pub struct SFVM_PROPPAGE_DATA(i32);
pub const SFVM_REARRANGE: u32 = 1u32;
pub const SFVM_REMOVEOBJECT: u32 = 6u32;
pub const SFVM_SETCLIPBOARD: u32 = 16u32;
pub const SFVM_SETITEMPOS: u32 = 14u32;
pub const SFVM_SETPOINTS: u32 = 23u32;
pub const SFVM_UPDATEOBJECT: u32 = 7u32;
pub const SFVSOC_INVALIDATE_ALL: u32 = 1u32;
pub const SFVSOC_NOSCROLL: u32 = 2u32;
#[repr(C)]
pub struct SFVS_SELECT(i32);
#[repr(C)]
pub struct SFV_CREATE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct SFV_SETITEMPOS(i32);
pub const SHACF_AUTOAPPEND_FORCE_OFF: u32 = 2147483648u32;
pub const SHACF_AUTOAPPEND_FORCE_ON: u32 = 1073741824u32;
pub const SHACF_AUTOSUGGEST_FORCE_OFF: u32 = 536870912u32;
pub const SHACF_AUTOSUGGEST_FORCE_ON: u32 = 268435456u32;
pub const SHACF_DEFAULT: u32 = 0u32;
pub const SHACF_FILESYSTEM: u32 = 1u32;
pub const SHACF_FILESYS_DIRS: u32 = 32u32;
pub const SHACF_FILESYS_ONLY: u32 = 16u32;
pub const SHACF_URLHISTORY: u32 = 2u32;
pub const SHACF_URLMRU: u32 = 4u32;
pub const SHACF_USETAB: u32 = 8u32;
pub const SHACF_VIRTUAL_NAMESPACE: u32 = 64u32;
#[repr(C)]
pub struct SHARD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHARDAPPIDINFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct SHARDAPPIDINFOIDLIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHARDAPPIDINFOLINK(i32);
#[repr(C)]
pub struct SHARE_ROLE(i32);
pub const SHCDF_UPDATEITEM: u32 = 1u32;
pub const SHCIDS_ALLFIELDS: i32 = -2147483648i32;
pub const SHCIDS_BITMASK: i32 = -65536i32;
pub const SHCIDS_CANONICALONLY: i32 = 268435456i32;
pub const SHCIDS_COLUMNMASK: i32 = 65535i32;
pub const SHCNEE_MSI_CHANGE: i32 = 4i32;
pub const SHCNEE_MSI_UNINSTALL: i32 = 5i32;
pub const SHCNEE_ORDERCHANGED: i32 = 2i32;
#[repr(C)]
pub struct SHCNE_ID(i32);
#[repr(C)]
pub struct SHCNF_FLAGS(i32);
#[repr(C)]
pub struct SHCNRF_SOURCE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHCOLUMNDATA(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
pub struct SHCOLUMNINFO(i32);
#[repr(C)]
pub struct SHCOLUMNINIT(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Threading"))]
#[repr(C)]
pub struct SHCREATEPROCESSINFOW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Threading"))]
#[repr(C)]
pub struct SHCREATEPROCESSINFOW(i32);
pub const SHC_E_SHELL_COMPONENT_STARTUP_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927180i32 as _);
#[repr(C)]
pub struct SHChangeDWORDAsIDList(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct SHChangeNotifyEntry(i32);
#[repr(C)]
pub struct SHChangeProductKeyAsIDList(i32);
#[repr(C)]
pub struct SHChangeUpdateImageIDList(i32);
#[repr(C)]
pub struct SHDESCRIPTIONID(i32);
#[repr(C)]
pub struct SHDID_ID(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct SHDRAGIMAGE(i32);
#[repr(C)]
pub struct SHELLBROWSERSHOWCONTROL(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[repr(C)]
pub struct SHELLEXECUTEINFOA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[repr(C)]
pub struct SHELLEXECUTEINFOA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[repr(C)]
pub struct SHELLEXECUTEINFOW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[repr(C)]
pub struct SHELLEXECUTEINFOW(i32);
#[repr(C)]
pub struct SHELLFLAGSTATE(i32);
#[repr(C)]
pub struct SHELLSTATEA(i32);
pub const SHELLSTATEVERSION_IE4: u32 = 9u32;
pub const SHELLSTATEVERSION_WIN2K: u32 = 10u32;
#[repr(C)]
pub struct SHELLSTATEW(i32);
pub const SHELL_E_WRONG_BITDEPTH: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927486i32 as _);
#[repr(C)]
pub struct SHELL_ITEM_RESOURCE(i32);
#[repr(C)]
pub struct SHELL_LINK_DATA_FLAGS(i32);
#[repr(C)]
pub struct SHELL_UI_COMPONENT(i32);
pub const SHERB_NOCONFIRMATION: u32 = 1u32;
pub const SHERB_NOPROGRESSUI: u32 = 2u32;
pub const SHERB_NOSOUND: u32 = 4u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct SHFILEINFOA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct SHFILEINFOA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct SHFILEINFOW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct SHFILEINFOW(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHFILEOPSTRUCTA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHFILEOPSTRUCTA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHFILEOPSTRUCTW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHFILEOPSTRUCTW(i32);
#[repr(C)]
pub struct SHFMT_ID(i32);
#[repr(C)]
pub struct SHFMT_OPT(i32);
#[repr(C)]
pub struct SHFMT_RET(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHFOLDERCUSTOMSETTINGS(i32);
#[repr(C)]
pub struct SHGDFIL_FORMAT(i32);
#[repr(C)]
pub struct SHGFI_FLAGS(i32);
#[repr(C)]
pub struct SHGFP_TYPE(i32);
#[repr(C)]
pub struct SHGLOBALCOUNTER(i32);
pub const SHGNLI_NOLNK: u64 = 8u64;
pub const SHGNLI_NOLOCNAME: u64 = 16u64;
pub const SHGNLI_NOUNIQUE: u64 = 4u64;
pub const SHGNLI_PIDL: u64 = 1u64;
pub const SHGNLI_PREFIXNAME: u64 = 2u64;
pub const SHGNLI_USEURLEXT: u64 = 32u64;
pub const SHGSI_ICONLOCATION: u32 = 0u32;
pub const SHGVSPB_ALLFOLDERS: u32 = 8u32;
pub const SHGVSPB_ALLUSERS: u32 = 2u32;
pub const SHGVSPB_INHERIT: u32 = 16u32;
pub const SHGVSPB_NOAUTODEFAULTS: u32 = 2147483648u32;
pub const SHGVSPB_PERFOLDER: u32 = 4u32;
pub const SHGVSPB_PERUSER: u32 = 1u32;
pub const SHGVSPB_ROAM: u32 = 32u32;
pub const SHHLNF_NOAUTOSELECT: u32 = 67108864u32;
pub const SHHLNF_WRITENOHISTORY: u32 = 134217728u32;
pub const SHIL_EXTRALARGE: u32 = 2u32;
pub const SHIL_JUMBO: u32 = 4u32;
pub const SHIL_LARGE: u32 = 0u32;
pub const SHIL_LAST: u32 = 4u32;
pub const SHIL_SMALL: u32 = 1u32;
pub const SHIL_SYSSMALL: u32 = 3u32;
pub const SHIMGDEC_DEFAULT: u32 = 0u32;
pub const SHIMGDEC_LOADFULL: u32 = 2u32;
pub const SHIMGDEC_THUMBNAIL: u32 = 1u32;
pub const SHIMSTCAPFLAG_LOCKABLE: u32 = 1u32;
pub const SHIMSTCAPFLAG_PURGEABLE: u32 = 2u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHNAMEMAPPINGA(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHNAMEMAPPINGA(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHNAMEMAPPINGW(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SHNAMEMAPPINGW(i32);
#[repr(C)]
pub struct SHOP_TYPE(i32);
pub const SHPPFW_ASKDIRCREATE: u32 = 2u32;
pub const SHPPFW_DIRCREATE: u32 = 1u32;
pub const SHPPFW_IGNOREFILENAME: u32 = 4u32;
pub const SHPPFW_MEDIACHECKONLY: u32 = 16u32;
pub const SHPPFW_NONE: u32 = 0u32;
pub const SHPPFW_NOWRITECHECK: u32 = 8u32;
pub const SHPWHF_ANYLOCATION: u32 = 256u32;
pub const SHPWHF_NOFILESELECTOR: u32 = 4u32;
pub const SHPWHF_NONETPLACECREATE: u32 = 2u32;
pub const SHPWHF_NORECOMPRESS: u32 = 1u32;
pub const SHPWHF_USEMRU: u32 = 8u32;
pub const SHPWHF_VALIDATEVIAWEBFOLDERS: u32 = 65536u32;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[repr(C)]
pub struct SHQUERYRBINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[repr(C)]
pub struct SHQUERYRBINFO(i32);
#[repr(C)]
pub struct SHREGDEL_FLAGS(i32);
#[repr(C)]
pub struct SHREGENUM_FLAGS(i32);
pub const SHREGSET_FORCE_HKCU: u32 = 2u32;
pub const SHREGSET_FORCE_HKLM: u32 = 8u32;
pub const SHREGSET_HKCU: u32 = 1u32;
pub const SHREGSET_HKLM: u32 = 4u32;
#[repr(C)]
pub struct SHSTOCKICONID(i32);
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct SHSTOCKICONINFO(i32);
#[cfg(any(target_arch = "x86",))]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct SHSTOCKICONINFO(i32);
#[repr(C)]
pub struct SIATTRIBFLAGS(i32);
pub const SID_CommandsPropertyBag: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1845768784,
    data2: 17430,
    data3: 18524,
    data4: [177, 67, 230, 42, 118, 13, 159, 229],
};
pub const SID_CtxQueryAssociations: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4205706304,
    data2: 46967,
    data3: 19305,
    data4: [170, 129, 119, 3, 94, 240, 230, 232],
};
pub const SID_DefView: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1829961344, data2: 30993, data3: 4559, data4: [149, 52, 0, 0, 192, 91, 174, 11] };
pub const SID_LaunchSourceAppUserModelId: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 753369104, data2: 29915, data3: 18620, data4: [156, 106, 16, 243, 114, 73, 87, 35] };
pub const SID_LaunchSourceViewSizePreference: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2153796754,
    data2: 26585,
    data3: 16719,
    data4: [175, 137, 161, 205, 241, 36, 43, 193],
};
pub const SID_LaunchTargetViewSizePreference: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 651895922, data2: 47031, data3: 16491, data4: [151, 2, 115, 10, 78, 32, 211, 191] };
pub const SID_MenuShellFolder: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2797698740, data2: 11621, data3: 4562, data4: [131, 143, 0, 192, 79, 217, 24, 208] };
pub const SID_SCommDlgBrowser: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2163409459, data2: 47071, data3: 4562, data4: [163, 59, 0, 96, 151, 223, 91, 212] };
pub const SID_SCommandBarState: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3114183260, data2: 14416, data3: 17408, data4: [188, 51, 44, 229, 52, 4, 139, 248] };
pub const SID_SGetViewFromViewDual: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2291831645,
    data2: 38686,
    data3: 19218,
    data4: [185, 12, 36, 223, 201, 225, 229, 232],
};
pub const SID_SInPlaceBrowser: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 489349163, data2: 13909, data3: 18124, data4: [182, 58, 40, 89, 136, 21, 59, 202] };
pub const SID_SMenuBandBKContextMenu: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 374062470, data2: 7437, data3: 19936, data4: [154, 59, 217, 114, 150, 71, 194, 184] };
pub const SID_SMenuBandBottom: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1950131812, data2: 3563, data3: 4561, data4: [152, 37, 0, 192, 79, 217, 25, 114] };
pub const SID_SMenuBandBottomSelected: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 375306996, data2: 27985, data3: 4562, data4: [131, 173, 0, 192, 79, 217, 24, 208] };
pub const SID_SMenuBandChild: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3986472992, data2: 2233, data3: 4561, data4: [152, 35, 0, 192, 79, 217, 25, 114] };
pub const SID_SMenuBandContextMenuModifier: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 961828980,
    data2: 29026,
    data3: 18014,
    data4: [183, 131, 42, 161, 135, 79, 239, 129],
};
pub const SID_SMenuBandParent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2351402732, data2: 16043, data3: 4561, data4: [140, 176, 0, 192, 79, 217, 24, 208] };
pub const SID_SMenuBandTop: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2492704784, data2: 60472, data3: 4560, data4: [188, 70, 0, 170, 0, 108, 226, 245] };
pub const SID_SMenuPopup: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3521621995, data2: 27182, data3: 4560, data4: [140, 120, 0, 192, 79, 217, 24, 180] };
pub const SID_SSearchBoxInfo: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 338537057,
    data2: 20843,
    data3: 18195,
    data4: [180, 156, 251, 152, 94, 248, 41, 152],
};
pub const SID_STopLevelBrowser: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1284947520, data2: 37212, data3: 4559, data4: [153, 211, 0, 170, 0, 74, 232, 55] };
pub const SID_STopWindow: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1239528704, data2: 17974, data3: 4563, data4: [151, 247, 0, 192, 79, 69, 208, 179] };
pub const SID_ShellExecuteNamedPropertyStore: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3951340962, data2: 255, data3: 18834, data4: [131, 36, 237, 92, 224, 97, 203, 41] };
pub const SID_URLExecutionContext: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4217343676,
    data2: 48054,
    data3: 19728,
    data4: [164, 97, 119, 114, 145, 160, 144, 48],
};
#[repr(C)]
pub struct SIGDN(i32);
#[repr(C)]
pub struct SIIGBF(i32);
pub const SIOM_ICONINDEX: u32 = 2u32;
pub const SIOM_OVERLAYINDEX: u32 = 1u32;
pub const SIOM_RESERVED_DEFAULT: u32 = 3u32;
pub const SIOM_RESERVED_LINK: u32 = 1u32;
pub const SIOM_RESERVED_SHARED: u32 = 0u32;
pub const SIOM_RESERVED_SLOWFILE: u32 = 2u32;
#[repr(C)]
pub struct SLGP_FLAGS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SLOWAPPINFO(i32);
#[repr(C)]
pub struct SLR_FLAGS(i32);
pub const SMAE_CONTRACTED: u32 = 2u32;
pub const SMAE_EXPANDED: u32 = 1u32;
pub const SMAE_USER: u32 = 4u32;
pub const SMAE_VALID: u32 = 7u32;
#[cfg(feature = "Win32_UI_Shell_Common")]
#[repr(C)]
pub struct SMCSHCHANGENOTIFYSTRUCT(i32);
pub const SMC_AUTOEXPANDCHANGE: u32 = 66u32;
pub const SMC_CHEVRONEXPAND: u32 = 25u32;
pub const SMC_CHEVRONGETTIP: u32 = 47u32;
pub const SMC_CREATE: u32 = 2u32;
pub const SMC_DEFAULTICON: u32 = 22u32;
pub const SMC_DEMOTE: u32 = 17u32;
pub const SMC_DISPLAYCHEVRONTIP: u32 = 42u32;
pub const SMC_EXITMENU: u32 = 3u32;
pub const SMC_GETAUTOEXPANDSTATE: u32 = 65u32;
pub const SMC_GETBKCONTEXTMENU: u32 = 68u32;
pub const SMC_GETCONTEXTMENUMODIFIER: u32 = 67u32;
pub const SMC_GETINFO: u32 = 5u32;
pub const SMC_GETOBJECT: u32 = 7u32;
pub const SMC_GETSFINFO: u32 = 6u32;
pub const SMC_GETSFOBJECT: u32 = 8u32;
pub const SMC_INITMENU: u32 = 1u32;
pub const SMC_NEWITEM: u32 = 23u32;
pub const SMC_OPEN: u32 = 69u32;
pub const SMC_PROMOTE: u32 = 18u32;
pub const SMC_REFRESH: u32 = 16u32;
pub const SMC_SETSFOBJECT: u32 = 45u32;
pub const SMC_SFDDRESTRICTED: u32 = 48u32;
pub const SMC_SFEXEC: u32 = 9u32;
pub const SMC_SFEXEC_MIDDLE: u32 = 49u32;
pub const SMC_SFSELECTITEM: u32 = 10u32;
pub const SMC_SHCHANGENOTIFY: u32 = 46u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct SMDATA(i32);
pub const SMDM_HMENU: u32 = 2u32;
pub const SMDM_SHELLFOLDER: u32 = 1u32;
pub const SMDM_TOOLBAR: u32 = 4u32;
#[repr(C)]
pub struct SMINFO(i32);
#[repr(C)]
pub struct SMINFOFLAGS(i32);
#[repr(C)]
pub struct SMINFOMASK(i32);
#[repr(C)]
pub struct SMINFOTYPE(i32);
pub const SMINIT_AUTOEXPAND: u32 = 256u32;
pub const SMINIT_AUTOTOOLTIP: u32 = 512u32;
pub const SMINIT_CACHED: u32 = 16u32;
pub const SMINIT_DEFAULT: u32 = 0u32;
pub const SMINIT_DROPONCONTAINER: u32 = 1024u32;
pub const SMINIT_HORIZONTAL: u32 = 536870912u32;
pub const SMINIT_RESTRICT_DRAGDROP: u32 = 2u32;
pub const SMINIT_TOPLEVEL: u32 = 4u32;
pub const SMINIT_VERTICAL: u32 = 268435456u32;
pub const SMINV_ID: u32 = 8u32;
pub const SMINV_REFRESH: u32 = 1u32;
pub const SMSET_BOTTOM: u32 = 536870912u32;
pub const SMSET_DONTOWN: u32 = 1u32;
pub const SMSET_TOP: u32 = 268435456u32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
pub struct SORTCOLUMN(i32);
#[repr(C)]
pub struct SORTDIRECTION(i32);
#[repr(C)]
pub struct SORT_ORDER_TYPE(i32);
#[repr(C)]
pub struct SPACTION(i32);
pub const SPMODE_BROWSER: u32 = 8u32;
pub const SPMODE_DBMON: u32 = 8192u32;
pub const SPMODE_DEBUGBREAK: u32 = 512u32;
pub const SPMODE_DEBUGOUT: u32 = 2u32;
pub const SPMODE_EVENT: u32 = 32u32;
pub const SPMODE_EVENTTRACE: u32 = 32768u32;
pub const SPMODE_FLUSH: u32 = 16u32;
pub const SPMODE_FORMATTEXT: u32 = 128u32;
pub const SPMODE_MEMWATCH: u32 = 4096u32;
pub const SPMODE_MSGTRACE: u32 = 1024u32;
pub const SPMODE_MSVM: u32 = 64u32;
pub const SPMODE_MULTISTOP: u32 = 16384u32;
pub const SPMODE_PERFTAGS: u32 = 2048u32;
pub const SPMODE_PROFILE: u32 = 256u32;
pub const SPMODE_SHELL: u32 = 1u32;
pub const SPMODE_TEST: u32 = 4u32;
#[repr(C)]
pub struct SPTEXT(i32);
pub const SRRF_NOEXPAND: u32 = 268435456u32;
pub const SRRF_NOVIRT: u32 = 1073741824u32;
pub const SRRF_RM_ANY: u32 = 0u32;
pub const SRRF_RM_NORMAL: u32 = 65536u32;
pub const SRRF_RM_SAFE: u32 = 131072u32;
pub const SRRF_RM_SAFENETWORK: u32 = 262144u32;
pub const SRRF_RT_ANY: u32 = 65535u32;
pub const SRRF_RT_REG_BINARY: u32 = 8u32;
pub const SRRF_RT_REG_DWORD: u32 = 16u32;
pub const SRRF_RT_REG_EXPAND_SZ: u32 = 4u32;
pub const SRRF_RT_REG_MULTI_SZ: u32 = 32u32;
pub const SRRF_RT_REG_NONE: u32 = 1u32;
pub const SRRF_RT_REG_QWORD: u32 = 64u32;
pub const SRRF_RT_REG_SZ: u32 = 2u32;
pub const SRRF_ZEROONFAILURE: u32 = 536870912u32;
#[repr(C)]
pub struct SSF_MASK(i32);
pub const SSM_CLEAR: u32 = 0u32;
pub const SSM_REFRESH: u32 = 2u32;
pub const SSM_SET: u32 = 1u32;
pub const SSM_UPDATE: u32 = 4u32;
#[repr(C)]
pub struct STGOP(i32);
pub const STIF_DEFAULT: i32 = 0i32;
pub const STIF_SUPPORT_HEX: i32 = 1i32;
#[repr(C)]
pub struct STORAGE_PROVIDER_FILE_FLAGS(i32);
pub const STORE_E_NEWER_VERSION_AVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2144927484i32 as _);
#[repr(C)]
pub struct STPFLAG(i32);
#[repr(C)]
pub struct SUBCLASSPROC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SV2CVW2_PARAMS(i32);
#[repr(C)]
pub struct SVUIA_STATUS(i32);
#[repr(C)]
pub struct SYNCMGRERRORFLAGS(i32);
#[repr(C)]
pub struct SYNCMGRFLAG(i32);
#[repr(C)]
pub struct SYNCMGRHANDLERFLAGS(i32);
pub const SYNCMGRHANDLERFLAG_MASK: u32 = 15u32;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct SYNCMGRHANDLERINFO(i32);
#[repr(C)]
pub struct SYNCMGRINVOKEFLAGS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct SYNCMGRITEM(i32);
#[repr(C)]
pub struct SYNCMGRITEMFLAGS(i32);
#[repr(C)]
pub struct SYNCMGRITEMSTATE(i32);
pub const SYNCMGRITEM_ITEMFLAGMASK: u32 = 127u32;
#[repr(C)]
pub struct SYNCMGRLOGERRORINFO(i32);
pub const SYNCMGRLOGERROR_ERRORFLAGS: u32 = 1u32;
pub const SYNCMGRLOGERROR_ERRORID: u32 = 2u32;
pub const SYNCMGRLOGERROR_ITEMID: u32 = 4u32;
#[repr(C)]
pub struct SYNCMGRLOGLEVEL(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct SYNCMGRPROGRESSITEM(i32);
pub const SYNCMGRPROGRESSITEM_MAXVALUE: u32 = 8u32;
pub const SYNCMGRPROGRESSITEM_PROGVALUE: u32 = 4u32;
pub const SYNCMGRPROGRESSITEM_STATUSTEXT: u32 = 1u32;
pub const SYNCMGRPROGRESSITEM_STATUSTYPE: u32 = 2u32;
#[repr(C)]
pub struct SYNCMGRREGISTERFLAGS(i32);
pub const SYNCMGRREGISTERFLAGS_MASK: u32 = 7u32;
#[repr(C)]
pub struct SYNCMGRSTATUS(i32);
#[repr(C)]
pub struct SYNCMGR_CANCEL_REQUEST(i32);
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
pub struct SYNCMGR_CONFLICT_ID_INFO(i32);
#[repr(C)]
pub struct SYNCMGR_CONFLICT_ITEM_TYPE(i32);
#[repr(C)]
pub struct SYNCMGR_CONTROL_FLAGS(i32);
#[repr(C)]
pub struct SYNCMGR_EVENT_FLAGS(i32);
#[repr(C)]
pub struct SYNCMGR_EVENT_LEVEL(i32);
#[repr(C)]
pub struct SYNCMGR_HANDLER_CAPABILITIES(i32);
#[repr(C)]
pub struct SYNCMGR_HANDLER_POLICIES(i32);
#[repr(C)]
pub struct SYNCMGR_HANDLER_TYPE(i32);
#[repr(C)]
pub struct SYNCMGR_ITEM_CAPABILITIES(i32);
#[repr(C)]
pub struct SYNCMGR_ITEM_POLICIES(i32);
pub const SYNCMGR_OBJECTID_BrowseContent: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1472968068,
    data2: 59828,
    data3: 18350,
    data4: [161, 32, 196, 223, 51, 53, 222, 226],
};
pub const SYNCMGR_OBJECTID_ConflictStore: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3615588852, data2: 9097, data3: 18404, data4: [169, 96, 96, 188, 194, 237, 147, 11] };
pub const SYNCMGR_OBJECTID_EventLinkClick: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 570670529, data2: 6897, data3: 16514, data4: [140, 48, 40, 57, 159, 65, 56, 76] };
pub const SYNCMGR_OBJECTID_EventStore: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1273967801,
    data2: 42886,
    data3: 16501,
    data4: [186, 136, 12, 43, 157, 137, 169, 143],
};
pub const SYNCMGR_OBJECTID_Icon: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1841071555, data2: 23815, data3: 19570, data4: [167, 119, 127, 236, 120, 7, 44, 6] };
pub const SYNCMGR_OBJECTID_QueryBeforeActivate: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3632453643,
    data2: 59306,
    data3: 18925,
    data4: [134, 183, 230, 225, 247, 20, 205, 254],
};
pub const SYNCMGR_OBJECTID_QueryBeforeDeactivate: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2700067458,
    data2: 24800,
    data3: 17934,
    data4: [147, 116, 234, 136, 81, 60, 252, 128],
};
pub const SYNCMGR_OBJECTID_QueryBeforeDelete: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4151063447, data2: 44979, data3: 17879, data4: [165, 159, 90, 73, 233, 5, 67, 126] };
pub const SYNCMGR_OBJECTID_QueryBeforeDisable: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3143591082, data2: 61444, data3: 20149, data4: [142, 77, 38, 117, 25, 102, 52, 76] };
pub const SYNCMGR_OBJECTID_QueryBeforeEnable: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 80476144,
    data2: 23531,
    data3: 19937,
    data4: [188, 144, 144, 131, 69, 196, 128, 246],
};
pub const SYNCMGR_OBJECTID_ShowSchedule: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3989238755,
    data2: 33857,
    data3: 16649,
    data4: [173, 243, 108, 28, 160, 183, 222, 71],
};
#[repr(C)]
pub struct SYNCMGR_PRESENTER_CHOICE(i32);
#[repr(C)]
pub struct SYNCMGR_PRESENTER_NEXT_STEP(i32);
#[repr(C)]
pub struct SYNCMGR_PROGRESS_STATUS(i32);
#[repr(C)]
pub struct SYNCMGR_RESOLUTION_ABILITIES(i32);
#[repr(C)]
pub struct SYNCMGR_RESOLUTION_FEEDBACK(i32);
#[repr(C)]
pub struct SYNCMGR_SYNC_CONTROL_FLAGS(i32);
#[repr(C)]
pub struct SYNCMGR_UPDATE_REASON(i32);
pub const S_SYNCMGR_CANCELALL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262660i32 as _);
pub const S_SYNCMGR_CANCELITEM: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262659i32 as _);
pub const S_SYNCMGR_ENUMITEMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262673i32 as _);
pub const S_SYNCMGR_ITEMDELETED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262672i32 as _);
pub const S_SYNCMGR_MISSINGITEMS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262657i32 as _);
pub const S_SYNCMGR_RETRYSYNC: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(262658i32 as _);
#[repr(C)]
pub struct ScheduledTasks(i32);
#[repr(C)]
pub struct SearchFolderItemFactory(i32);
#[repr(C)]
pub struct SecureLockIconConstants(i32);
pub const SelectedItemCount_Property_GUID: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2414024402,
    data2: 3666,
    data3: 17930,
    data4: [156, 30, 72, 242, 115, 212, 112, 163],
};
#[repr(C)]
pub struct ShFindChangeNotificationHandle(i32);
#[repr(C)]
pub struct SharedBitmap(i32);
#[repr(C)]
pub struct SharingConfigurationManager(i32);
#[repr(C)]
pub struct Shell(i32);
#[repr(C)]
pub struct ShellBrowserWindow(i32);
#[repr(C)]
pub struct ShellDesktop(i32);
#[repr(C)]
pub struct ShellDispatchInproc(i32);
#[repr(C)]
pub struct ShellFSFolder(i32);
#[repr(C)]
pub struct ShellFolderItem(i32);
#[repr(C)]
pub struct ShellFolderView(i32);
#[repr(C)]
pub struct ShellFolderViewOC(i32);
#[repr(C)]
pub struct ShellFolderViewOptions(i32);
#[repr(C)]
pub struct ShellImageDataFactory(i32);
#[repr(C)]
pub struct ShellItem(i32);
#[repr(C)]
pub struct ShellLibrary(i32);
#[repr(C)]
pub struct ShellLink(i32);
#[repr(C)]
pub struct ShellLinkObject(i32);
#[repr(C)]
pub struct ShellNameSpace(i32);
#[repr(C)]
pub struct ShellSpecialFolderConstants(i32);
#[repr(C)]
pub struct ShellUIHelper(i32);
#[repr(C)]
pub struct ShellWindowFindWindowOptions(i32);
#[repr(C)]
pub struct ShellWindowTypeConstants(i32);
#[repr(C)]
pub struct ShellWindows(i32);
#[repr(C)]
pub struct ShowInputPaneAnimationCoordinator(i32);
#[repr(C)]
pub struct SimpleConflictPresenter(i32);
#[repr(C)]
pub struct SizeCategorizer(i32);
#[repr(C)]
pub struct SmartcardCredentialProvider(i32);
#[repr(C)]
pub struct SmartcardPinProvider(i32);
#[repr(C)]
pub struct SmartcardReaderSelectionProvider(i32);
#[repr(C)]
pub struct SmartcardWinRTProvider(i32);
#[repr(C)]
pub struct StartMenuPin(i32);
#[repr(C)]
pub struct StorageProviderBanners(i32);
#[repr(C)]
pub struct SuspensionDependencyManager(i32);
#[repr(C)]
pub struct SyncMgr(i32);
#[repr(C)]
pub struct SyncMgrClient(i32);
#[repr(C)]
pub struct SyncMgrControl(i32);
#[repr(C)]
pub struct SyncMgrFolder(i32);
#[repr(C)]
pub struct SyncMgrScheduleWizard(i32);
#[repr(C)]
pub struct SyncResultsFolder(i32);
#[repr(C)]
pub struct SyncSetupFolder(i32);
pub const TBIF_APPEND: u32 = 0u32;
pub const TBIF_DEFAULT: u32 = 0u32;
pub const TBIF_INTERNETBAR: u32 = 65536u32;
pub const TBIF_NOTOOLBAR: u32 = 196608u32;
pub const TBIF_PREPEND: u32 = 1u32;
pub const TBIF_REPLACE: u32 = 2u32;
pub const TBIF_STANDARDTOOLBAR: u32 = 131072u32;
#[repr(C)]
pub struct TBINFO(i32);
#[repr(C)]
pub struct TBPFLAG(i32);
pub const THBN_CLICKED: u32 = 6144u32;
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[repr(C)]
pub struct THUMBBUTTON(i32);
#[repr(C)]
pub struct THUMBBUTTONFLAGS(i32);
#[repr(C)]
pub struct THUMBBUTTONMASK(i32);
pub const TITLEBARNAMELEN: u32 = 40u32;
#[repr(C)]
pub struct TI_FLAGS(i32);
#[repr(C)]
pub struct TLENUMF(i32);
pub const TLMENUF_BACK: u32 = 16u32;
pub const TLMENUF_FORE: u32 = 32u32;
pub const TLMENUF_INCLUDECURRENT: u32 = 1u32;
pub const TLOG_BACK: i32 = -1i32;
pub const TLOG_CURRENT: u32 = 0u32;
pub const TLOG_FORE: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[repr(C)]
pub struct TOOLBARITEM(i32);
#[repr(C)]
pub struct TaskbarList(i32);
#[repr(C)]
pub struct ThumbnailStreamCache(i32);
#[repr(C)]
pub struct ThumbnailStreamCacheOptions(i32);
#[repr(C)]
pub struct TimeCategorizer(i32);
#[repr(C)]
pub struct TrackShellMenu(i32);
#[repr(C)]
pub struct TrayBandSiteService(i32);
#[repr(C)]
pub struct TrayDeskBand(i32);
#[repr(C)]
pub struct UNDOCK_REASON(i32);
#[repr(C)]
pub struct URLIS(i32);
pub const URL_APPLY_DEFAULT: u32 = 1u32;
pub const URL_APPLY_FORCEAPPLY: u32 = 8u32;
pub const URL_APPLY_GUESSFILE: u32 = 4u32;
pub const URL_APPLY_GUESSSCHEME: u32 = 2u32;
pub const URL_BROWSER_MODE: u32 = 33554432u32;
pub const URL_CONVERT_IF_DOSPATH: u32 = 2097152u32;
pub const URL_DONT_ESCAPE_EXTRA_INFO: u32 = 33554432u32;
pub const URL_DONT_SIMPLIFY: u32 = 134217728u32;
pub const URL_DONT_UNESCAPE: u32 = 131072u32;
pub const URL_DONT_UNESCAPE_EXTRA_INFO: u32 = 33554432u32;
pub const URL_ESCAPE_ASCII_URI_COMPONENT: u32 = 524288u32;
pub const URL_ESCAPE_AS_UTF8: u32 = 262144u32;
pub const URL_ESCAPE_PERCENT: u32 = 4096u32;
pub const URL_ESCAPE_SEGMENT_ONLY: u32 = 8192u32;
pub const URL_ESCAPE_SPACES_ONLY: u32 = 67108864u32;
pub const URL_ESCAPE_UNSAFE: u32 = 536870912u32;
pub const URL_E_INVALID_SYNTAX: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217407i32 as _);
pub const URL_E_UNREGISTERED_PROTOCOL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147217406i32 as _);
pub const URL_FILE_USE_PATHURL: u32 = 65536u32;
pub const URL_INTERNAL_PATH: u32 = 8388608u32;
pub const URL_NO_META: u32 = 134217728u32;
#[repr(C)]
pub struct URL_PART(i32);
pub const URL_PARTFLAG_KEEPSCHEME: u32 = 1u32;
pub const URL_PLUGGABLE_PROTOCOL: u32 = 1073741824u32;
#[repr(C)]
pub struct URL_SCHEME(i32);
pub const URL_UNESCAPE: u32 = 268435456u32;
pub const URL_UNESCAPE_AS_UTF8: u32 = 262144u32;
pub const URL_UNESCAPE_HIGH_ANSI_ONLY: u32 = 4194304u32;
pub const URL_UNESCAPE_INPLACE: u32 = 1048576u32;
pub const URL_UNESCAPE_URI_COMPONENT: u32 = 262144u32;
pub const URL_WININET_COMPATIBILITY: u32 = 2147483648u32;
#[repr(C)]
pub struct UserNotification(i32);
#[repr(C)]
pub struct V1PasswordCredentialProvider(i32);
#[repr(C)]
pub struct V1SmartcardCredentialProvider(i32);
#[repr(C)]
pub struct V1WinBioCredentialProvider(i32);
#[repr(C)]
pub struct VALIDATEUNC_OPTION(i32);
pub const VID_Content: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 818070580, data2: 2185, data3: 19597, data4: [152, 93, 169, 247, 24, 48, 176, 169] };
pub const VID_Details: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 327055104, data2: 13683, data3: 4559, data4: [174, 105, 8, 0, 43, 46, 18, 98] };
pub const VID_LargeIcons: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 5755104, data2: 13683, data3: 4559, data4: [174, 105, 8, 0, 43, 46, 18, 98] };
pub const VID_List: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 236955104, data2: 13683, data3: 4559, data4: [174, 105, 8, 0, 43, 46, 18, 98] };
pub const VID_SmallIcons: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 143655104, data2: 13683, data3: 4559, data4: [174, 105, 8, 0, 43, 46, 18, 98] };
pub const VID_ThumbStrip: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2398070308,
    data2: 53737,
    data3: 17499,
    data4: [148, 183, 116, 251, 206, 46, 161, 26],
};
pub const VID_Thumbnails: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2347479696, data2: 21200, data3: 4560, data4: [183, 244, 0, 192, 79, 215, 6, 236] };
pub const VID_Tile: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1710302693,
    data2: 31713,
    data3: 18448,
    data4: [186, 157, 210, 113, 200, 67, 44, 227],
};
pub const VIEW_PRIORITY_CACHEHIT: u32 = 80u32;
pub const VIEW_PRIORITY_CACHEMISS: u32 = 48u32;
pub const VIEW_PRIORITY_DESPERATE: u32 = 16u32;
pub const VIEW_PRIORITY_INHERIT: u32 = 32u32;
pub const VIEW_PRIORITY_NONE: u32 = 0u32;
pub const VIEW_PRIORITY_RESTRICTED: u32 = 112u32;
pub const VIEW_PRIORITY_SHELLEXT: u32 = 64u32;
pub const VIEW_PRIORITY_SHELLEXT_ASBACKUP: u32 = 21u32;
pub const VIEW_PRIORITY_STALECACHEHIT: u32 = 69u32;
pub const VIEW_PRIORITY_USEASDEFAULT: u32 = 67u32;
#[repr(C)]
pub struct VPCOLORFLAGS(i32);
#[repr(C)]
pub struct VPWATERMARKFLAGS(i32);
#[repr(C)]
pub struct VaultProvider(i32);
#[repr(C)]
pub struct VirtualDesktopManager(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
#[repr(C)]
pub struct WINDOWDATA(i32);
pub const WM_CPL_LAUNCH: u32 = 2024u32;
pub const WM_CPL_LAUNCHED: u32 = 2025u32;
pub const WPSTYLE_CENTER: u32 = 0u32;
pub const WPSTYLE_CROPTOFIT: u32 = 4u32;
pub const WPSTYLE_KEEPASPECT: u32 = 3u32;
pub const WPSTYLE_MAX: u32 = 6u32;
pub const WPSTYLE_SPAN: u32 = 5u32;
pub const WPSTYLE_STRETCH: u32 = 2u32;
pub const WPSTYLE_TILE: u32 = 1u32;
#[repr(C)]
pub struct WTS_ALPHATYPE(i32);
#[repr(C)]
pub struct WTS_CACHEFLAGS(i32);
#[repr(C)]
pub struct WTS_CONTEXTFLAGS(i32);
pub const WTS_E_DATAFILEUNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147175932i32 as _);
pub const WTS_E_EXTRACTIONBLOCKED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147175930i32 as _);
pub const WTS_E_EXTRACTIONPENDING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147175931i32 as _);
pub const WTS_E_EXTRACTIONTIMEDOUT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147175935i32 as _);
pub const WTS_E_FAILEDEXTRACTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147175936i32 as _);
pub const WTS_E_FASTEXTRACTIONNOTSUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147175933i32 as _);
pub const WTS_E_NOSTORAGEPROVIDERTHUMBNAILHANDLER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147175929i32 as _);
pub const WTS_E_SURROGATEUNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2147175934i32 as _);
#[repr(C)]
pub struct WTS_FLAGS(i32);
#[repr(C)]
pub struct WTS_THUMBNAILID(i32);
#[repr(C)]
pub struct WebBrowser(i32);
#[repr(C)]
pub struct WebBrowser_V1(i32);
#[repr(C)]
pub struct WebWizardHost(i32);
#[repr(C)]
pub struct WinBioCredentialProvider(i32);
#[repr(C)]
pub struct _APPCONSTRAIN_REGISTRATION(i32);
#[repr(C)]
pub struct _APPSTATE_REGISTRATION(i32);
#[repr(C)]
pub struct _BROWSERFRAMEOPTIONS(i32);
#[repr(C)]
pub struct _CDBE_ACTIONS(i32);
#[repr(C)]
pub struct _EXPCMDFLAGS(i32);
#[repr(C)]
pub struct _EXPCMDSTATE(i32);
#[repr(C)]
pub struct _EXPLORERPANESTATE(i32);
#[repr(C)]
pub struct _EXPPS(i32);
#[repr(C)]
pub struct _FILEOPENDIALOGOPTIONS(i32);
#[repr(C)]
pub struct _HLSHORTCUTF__NOREDEF10(i32);
#[repr(C)]
pub struct _HLSR_NOREDEF10(i32);
#[repr(C)]
pub struct _HLTRANSLATEF_NOREDEF10(i32);
#[repr(C)]
pub struct _KF_DEFINITION_FLAGS(i32);
#[repr(C)]
pub struct _KF_REDIRECTION_CAPABILITIES(i32);
#[repr(C)]
pub struct _KF_REDIRECT_FLAGS(i32);
#[repr(C)]
pub struct _NMCII_FLAGS(i32);
#[repr(C)]
pub struct _NMCSAEI_FLAGS(i32);
#[repr(C)]
pub struct _NSTCECLICKTYPE(i32);
#[repr(C)]
pub struct _NSTCEHITTEST(i32);
#[repr(C)]
pub struct _NSTCITEMSTATE(i32);
#[repr(C)]
pub struct _NSTCROOTSTYLE(i32);
#[repr(C)]
pub struct _NSTCSTYLE(i32);
#[repr(C)]
pub struct _OPPROGDLGF(i32);
#[repr(C)]
pub struct _PDMODE(i32);
#[repr(C)]
pub struct _SHCONTF(i32);
#[repr(C)]
pub struct _SHGDNF(i32);
#[repr(C)]
pub struct _SICHINTF(i32);
#[repr(C)]
pub struct _SPBEGINF(i32);
#[repr(C)]
pub struct _SPINITF(i32);
#[repr(C)]
pub struct _SV3CVW3_FLAGS(i32);
#[repr(C)]
pub struct _SVGIO(i32);
#[repr(C)]
pub struct _SVSIF(i32);
#[repr(C)]
pub struct _TRANSFER_ADVISE_STATE(i32);
#[repr(C)]
pub struct _TRANSFER_SOURCE_FLAGS(i32);
#[repr(C)]
pub struct iurl_invokecommand_flags(i32);
#[repr(C)]
pub struct iurl_seturl_flags(i32);
#[repr(C)]
pub struct mimeassociationdialog_in_flags(i32);
#[repr(C)]
pub struct translateurl_in_flags(i32);
#[repr(C)]
pub struct urlassociationdialog_in_flags(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct urlinvokecommandinfoA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct urlinvokecommandinfoW(i32);
