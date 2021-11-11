#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_UI_Shell_Common")]
pub mod Common;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub mod PropertiesSystem;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn AssocCreate(clsid: ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocCreateForClasses(rgclasses: *const ASSOCIATIONELEMENT, cclasses: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_UI_Shell_Common`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn AssocGetDetailsOfPropKey(psf: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST, pkey: *const PropertiesSystem::PROPERTYKEY, pv: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pffoundpropkey: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn AssocGetPerceivedType(pszext: super::super::Foundation::PWSTR, ptype: *mut Common::PERCEIVED, pflag: *mut u32, ppsztype: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssocIsDangerous(pszassoc: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryKeyA(flags: u32, key: ASSOCKEY, pszassoc: super::super::Foundation::PSTR, pszextra: super::super::Foundation::PSTR, phkeyout: *mut super::super::System::Registry::HKEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryKeyW(flags: u32, key: ASSOCKEY, pszassoc: super::super::Foundation::PWSTR, pszextra: super::super::Foundation::PWSTR, phkeyout: *mut super::super::System::Registry::HKEY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssocQueryStringA(flags: u32, str: ASSOCSTR, pszassoc: super::super::Foundation::PSTR, pszextra: super::super::Foundation::PSTR, pszout: super::super::Foundation::PSTR, pcchout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryStringByKeyA(flags: u32, str: ASSOCSTR, hkassoc: super::super::System::Registry::HKEY, pszextra: super::super::Foundation::PSTR, pszout: super::super::Foundation::PSTR, pcchout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryStringByKeyW(flags: u32, str: ASSOCSTR, hkassoc: super::super::System::Registry::HKEY, pszextra: super::super::Foundation::PWSTR, pszout: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssocQueryStringW(flags: u32, str: ASSOCSTR, pszassoc: super::super::Foundation::PWSTR, pszextra: super::super::Foundation::PWSTR, pszout: super::super::Foundation::PWSTR, pcchout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
    pub fn CDefFolderMenu_Create2(pidlfolder: *const Common::ITEMIDLIST, hwnd: super::super::Foundation::HWND, cidl: u32, apidl: *const *const Common::ITEMIDLIST, psf: ::windows::runtime::RawPtr, pfn: ::windows::runtime::RawPtr, nkeys: u32, ahkeys: *const super::super::System::Registry::HKEY, ppcm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn CIDLData_CreateFromIDArray(pidlfolder: *const Common::ITEMIDLIST, cidl: u32, apidl: *const *const Common::ITEMIDLIST, ppdtobj: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChrCmpIA(w1: u16, w2: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChrCmpIW(w1: u16, w2: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorAdjustLuma(clrrgb: u32, n: i32, fscale: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn ColorHLSToRGB(whue: u16, wluminance: u16, wsaturation: u16) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn ColorRGBToHLS(clrrgb: u32, pwhue: *mut u16, pwluminance: *mut u16, pwsaturation: *mut u16);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommandLineToArgvW(lpcmdline: super::super::Foundation::PWSTR, pnumargs: *mut i32) -> *mut super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ConnectToConnectionPoint(punk: ::windows::runtime::RawPtr, riidevent: *const ::windows::runtime::GUID, fconnect: super::super::Foundation::BOOL, punktarget: ::windows::runtime::RawPtr, pdwcookie: *mut u32, ppcpout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateProfile(pszusersid: super::super::Foundation::PWSTR, pszusername: super::super::Foundation::PWSTR, pszprofilepath: super::super::Foundation::PWSTR, cchprofilepath: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_AutoScroll(hwnd: super::super::Foundation::HWND, pad: *mut AUTO_SCROLL_DATA, pptnow: *const super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_DragEnterEx(hwndtarget: super::super::Foundation::HWND, ptstart: super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DAD_DragEnterEx2(hwndtarget: super::super::Foundation::HWND, ptstart: super::super::Foundation::POINT, pdtobject: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_DragLeave() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_DragMove(pt: super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DAD_SetDragImage(him: super::Controls::HIMAGELIST, pptoffset: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_ShowDragImage(fshow: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefSubclassProc(hwnd: super::super::Foundation::HWND, umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteProfileA(lpsidstring: super::super::Foundation::PSTR, lpprofilepath: super::super::Foundation::PSTR, lpcomputername: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteProfileW(lpsidstring: super::super::Foundation::PWSTR, lpprofilepath: super::super::Foundation::PWSTR, lpcomputername: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoEnvironmentSubstA(pszsrc: super::super::Foundation::PSTR, cchsrc: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoEnvironmentSubstW(pszsrc: super::super::Foundation::PWSTR, cchsrc: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragAcceptFiles(hwnd: super::super::Foundation::HWND, faccept: super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn DragFinish(hdrop: HDROP);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragQueryFileA(hdrop: HDROP, ifile: u32, lpszfile: super::super::Foundation::PSTR, cch: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragQueryFileW(hdrop: HDROP, ifile: u32, lpszfile: super::super::Foundation::PWSTR, cch: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragQueryPoint(hdrop: HDROP, ppt: *mut super::super::Foundation::POINT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn DriveType(idrive: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn DuplicateIcon(hinst: super::super::Foundation::HINSTANCE, hicon: super::WindowsAndMessaging::HICON) -> super::WindowsAndMessaging::HICON;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconA(hinst: super::super::Foundation::HINSTANCE, psziconpath: super::super::Foundation::PSTR, piicon: *mut u16) -> super::WindowsAndMessaging::HICON;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconExA(hinst: super::super::Foundation::HINSTANCE, psziconpath: super::super::Foundation::PSTR, piiconindex: *mut u16, piiconid: *mut u16) -> super::WindowsAndMessaging::HICON;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconExW(hinst: super::super::Foundation::HINSTANCE, psziconpath: super::super::Foundation::PWSTR, piiconindex: *mut u16, piiconid: *mut u16) -> super::WindowsAndMessaging::HICON;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconW(hinst: super::super::Foundation::HINSTANCE, psziconpath: super::super::Foundation::PWSTR, piicon: *mut u16) -> super::WindowsAndMessaging::HICON;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconA(hinst: super::super::Foundation::HINSTANCE, pszexefilename: super::super::Foundation::PSTR, niconindex: u32) -> super::WindowsAndMessaging::HICON;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconExA(lpszfile: super::super::Foundation::PSTR, niconindex: i32, phiconlarge: *mut super::WindowsAndMessaging::HICON, phiconsmall: *mut super::WindowsAndMessaging::HICON, nicons: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconExW(lpszfile: super::super::Foundation::PWSTR, niconindex: i32, phiconlarge: *mut super::WindowsAndMessaging::HICON, phiconsmall: *mut super::WindowsAndMessaging::HICON, nicons: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconW(hinst: super::super::Foundation::HINSTANCE, pszexefilename: super::super::Foundation::PWSTR, niconindex: u32) -> super::WindowsAndMessaging::HICON;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableA(lpfile: super::super::Foundation::PSTR, lpdirectory: super::super::Foundation::PSTR, lpresult: super::super::Foundation::PSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableW(lpfile: super::super::Foundation::PWSTR, lpdirectory: super::super::Foundation::PWSTR, lpresult: super::super::Foundation::PWSTR) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAcceptLanguagesA(pszlanguages: super::super::Foundation::PSTR, pcchlanguages: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAcceptLanguagesW(pszlanguages: super::super::Foundation::PWSTR, pcchlanguages: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAllUsersProfileDirectoryA(lpprofiledir: super::super::Foundation::PSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAllUsersProfileDirectoryW(lpprofiledir: super::super::Foundation::PWSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentProcessExplicitAppUserModelID(appid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultUserProfileDirectoryA(lpprofiledir: super::super::Foundation::PSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultUserProfileDirectoryW(lpprofiledir: super::super::Foundation::PWSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn GetDpiForShellUIComponent(param0: SHELL_UI_COMPONENT) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileNameFromBrowse(hwnd: super::super::Foundation::HWND, pszfilepath: super::super::Foundation::PWSTR, cchfilepath: u32, pszworkingdir: super::super::Foundation::PWSTR, pszdefext: super::super::Foundation::PWSTR, pszfilters: super::super::Foundation::PWSTR, psztitle: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn GetMenuContextHelpId(param0: super::WindowsAndMessaging::HMENU) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn GetMenuPosFromID(hmenu: super::WindowsAndMessaging::HMENU, id: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileType(dwflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfilesDirectoryA(lpprofiledir: super::super::Foundation::PSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfilesDirectoryW(lpprofiledir: super::super::Foundation::PWSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn GetScaleFactorForDevice(devicetype: DISPLAY_DEVICE_TYPE) -> Common::DEVICE_SCALE_FACTOR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Shell_Common"))]
    pub fn GetScaleFactorForMonitor(hmon: super::super::Graphics::Gdi::HMONITOR, pscale: *mut Common::DEVICE_SCALE_FACTOR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserProfileDirectoryA(htoken: super::super::Foundation::HANDLE, lpprofiledir: super::super::Foundation::PSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserProfileDirectoryW(htoken: super::super::Foundation::HANDLE, lpprofiledir: super::super::Foundation::PWSTR, lpcchsize: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowContextHelpId(param0: super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowSubclass(hwnd: super::super::Foundation::HWND, pfnsubclass: ::windows::runtime::RawPtr, uidsubclass: usize, pdwrefdata: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserFree(param0: *const u32, param1: *const super::super::Graphics::Gdi::HMONITOR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserFree64(param0: *const u32, param1: *const super::super::Graphics::Gdi::HMONITOR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HMONITOR) -> *mut u8;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::Graphics::Gdi::HMONITOR) -> *mut u8;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserSize(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HMONITOR) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::Graphics::Gdi::HMONITOR) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HMONITOR) -> *mut u8;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::Graphics::Gdi::HMONITOR) -> *mut u8;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn HashData(pbdata: *const u8, cbdata: u32, pbhash: *mut u8, cbhash: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn HlinkClone(pihl: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, pihlsiteforclone: ::windows::runtime::RawPtr, dwsitedata: u32, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn HlinkCreateBrowseContext(piunkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateExtensionServices(pwzadditionalheaders: super::super::Foundation::PWSTR, phwnd: super::super::Foundation::HWND, pszusername: super::super::Foundation::PWSTR, pszpassword: super::super::Foundation::PWSTR, piunkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkCreateFromData(pidataobj: ::windows::runtime::RawPtr, pihlsite: ::windows::runtime::RawPtr, dwsitedata: u32, piunkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkCreateFromMoniker(pimktrgt: ::windows::runtime::RawPtr, pwzlocation: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR, pihlsite: ::windows::runtime::RawPtr, dwsitedata: u32, piunkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateFromString(pwztarget: super::super::Foundation::PWSTR, pwzlocation: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR, pihlsite: ::windows::runtime::RawPtr, dwsitedata: u32, piunkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateShortcut(grfhlshortcutf: u32, pihl: ::windows::runtime::RawPtr, pwzdir: super::super::Foundation::PWSTR, pwzfilename: super::super::Foundation::PWSTR, ppwzshortcutfile: *mut super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkCreateShortcutFromMoniker(grfhlshortcutf: u32, pimktarget: ::windows::runtime::RawPtr, pwzlocation: super::super::Foundation::PWSTR, pwzdir: super::super::Foundation::PWSTR, pwzfilename: super::super::Foundation::PWSTR, ppwzshortcutfile: *mut super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateShortcutFromString(grfhlshortcutf: u32, pwztarget: super::super::Foundation::PWSTR, pwzlocation: super::super::Foundation::PWSTR, pwzdir: super::super::Foundation::PWSTR, pwzfilename: super::super::Foundation::PWSTR, ppwzshortcutfile: *mut super::super::Foundation::PWSTR, dwreserved: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkGetSpecialReference(ureference: u32, ppwzreference: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkGetValueFromParams(pwzparams: super::super::Foundation::PWSTR, pwzname: super::super::Foundation::PWSTR, ppwzvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkIsShortcut(pwzfilename: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkNavigate(pihl: ::windows::runtime::RawPtr, pihlframe: ::windows::runtime::RawPtr, grfhlnf: u32, pbc: ::windows::runtime::RawPtr, pibsc: ::windows::runtime::RawPtr, pihlbc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkNavigateToStringReference(pwztarget: super::super::Foundation::PWSTR, pwzlocation: super::super::Foundation::PWSTR, pihlsite: ::windows::runtime::RawPtr, dwsitedata: u32, pihlframe: ::windows::runtime::RawPtr, grfhlnf: u32, pibc: ::windows::runtime::RawPtr, pibsc: ::windows::runtime::RawPtr, pihlbc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkOnNavigate(pihlframe: ::windows::runtime::RawPtr, pihlbc: ::windows::runtime::RawPtr, grfhlnf: u32, pimktarget: ::windows::runtime::RawPtr, pwzlocation: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR, puhlid: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkOnRenameDocument(dwreserved: u32, pihlbc: ::windows::runtime::RawPtr, pimkold: ::windows::runtime::RawPtr, pimknew: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkParseDisplayName(pibc: ::windows::runtime::RawPtr, pwzdisplayname: super::super::Foundation::PWSTR, fnoforceabs: super::super::Foundation::BOOL, pccheaten: *mut u32, ppimk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkPreprocessMoniker(pibc: ::windows::runtime::RawPtr, pimkin: ::windows::runtime::RawPtr, ppimkout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkQueryCreateFromData(pidataobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkResolveMonikerForData(pimkreference: ::windows::runtime::RawPtr, reserved: u32, pibc: ::windows::runtime::RawPtr, cfmtetc: u32, rgfmtetc: *mut super::super::System::Com::FORMATETC, pibsc: ::windows::runtime::RawPtr, pimkbase: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkResolveShortcut(pwzshortcutfilename: super::super::Foundation::PWSTR, pihlsite: ::windows::runtime::RawPtr, dwsitedata: u32, piunkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkResolveShortcutToMoniker(pwzshortcutfilename: super::super::Foundation::PWSTR, ppimktarget: *mut ::windows::runtime::RawPtr, ppwzlocation: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkResolveShortcutToString(pwzshortcutfilename: super::super::Foundation::PWSTR, ppwztarget: *mut super::super::Foundation::PWSTR, ppwzlocation: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkResolveStringForData(pwzreference: super::super::Foundation::PWSTR, reserved: u32, pibc: ::windows::runtime::RawPtr, cfmtetc: u32, rgfmtetc: *mut super::super::System::Com::FORMATETC, pibsc: ::windows::runtime::RawPtr, pimkbase: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSetSpecialReference(ureference: u32, pwzreference: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkTranslateURL(pwzurl: super::super::Foundation::PWSTR, grfflags: u32, ppwztranslatedurl: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkUpdateStackItem(pihlframe: ::windows::runtime::RawPtr, pihlbc: ::windows::runtime::RawPtr, uhlid: u32, pimktrgt: ::windows::runtime::RawPtr, pwzlocation: super::super::Foundation::PWSTR, pwzfriendlyname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILAppendID(pidl: *const Common::ITEMIDLIST, pmkid: *const Common::SHITEMID, fappend: super::super::Foundation::BOOL) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILClone(pidl: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILCloneFirst(pidl: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILCombine(pidl1: *const Common::ITEMIDLIST, pidl2: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILCreateFromPathA(pszpath: super::super::Foundation::PSTR) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILCreateFromPathW(pszpath: super::super::Foundation::PWSTR) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILFindChild(pidlparent: *const Common::ITEMIDLIST, pidlchild: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILFindLastID(pidl: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILFree(pidl: *const Common::ITEMIDLIST);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILGetNext(pidl: *const Common::ITEMIDLIST) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILGetSize(pidl: *const Common::ITEMIDLIST) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILIsEqual(pidl1: *const Common::ITEMIDLIST, pidl2: *const Common::ITEMIDLIST) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILIsParent(pidl1: *const Common::ITEMIDLIST, pidl2: *const Common::ITEMIDLIST, fimmediate: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn ILLoadFromStreamEx(pstm: ::windows::runtime::RawPtr, pidl: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILRemoveLastID(pidl: *mut Common::ITEMIDLIST) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn ILSaveToStream(pstm: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Copy(pstmfrom: ::windows::runtime::RawPtr, pstmto: ::windows::runtime::RawPtr, cb: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Read(pstm: ::windows::runtime::RawPtr, pv: *mut ::core::ffi::c_void, cb: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn IStream_ReadPidl(pstm: ::windows::runtime::RawPtr, ppidlout: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn IStream_ReadStr(pstm: ::windows::runtime::RawPtr, ppsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Reset(pstm: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Size(pstm: ::windows::runtime::RawPtr, pui: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Write(pstm: ::windows::runtime::RawPtr, pv: *const ::core::ffi::c_void, cb: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn IStream_WritePidl(pstm: ::windows::runtime::RawPtr, pidlwrite: *const Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn IStream_WriteStr(pstm: ::windows::runtime::RawPtr, psz: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_AtomicRelease(ppunk: *mut *mut ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_GetSite(punk: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IUnknown_GetWindow(punk: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_QueryService(punk: ::windows::runtime::RawPtr, guidservice: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppvout: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_Set(ppunk: *mut ::windows::runtime::RawPtr, punk: ::windows::runtime::RawPtr);
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_SetSite(punk: ::windows::runtime::RawPtr, punksite: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportPrivacySettings(pszfilename: super::super::Foundation::PWSTR, pfparseprivacypreferences: *mut super::super::Foundation::BOOL, pfparsepersiterules: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitNetworkAddressControl() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntlStrEqWorkerA(fcasesens: super::super::Foundation::BOOL, lpstring1: super::super::Foundation::PSTR, lpstring2: super::super::Foundation::PSTR, nchar: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntlStrEqWorkerW(fcasesens: super::super::Foundation::BOOL, lpstring1: super::super::Foundation::PWSTR, lpstring2: super::super::Foundation::PWSTR, nchar: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharSpaceA(wch: super::super::Foundation::CHAR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharSpaceW(wch: u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsInternetESCEnabled() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLFNDriveA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLFNDriveW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IsNetDrive(idrive: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsOS(dwos: OS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUserAnAdmin() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadUserProfileA(htoken: super::super::Foundation::HANDLE, lpprofileinfo: *mut PROFILEINFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadUserProfileW(htoken: super::super::Foundation::HANDLE, lpprofileinfo: *mut PROFILEINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleSaveToStreamEx(piunk: ::windows::runtime::RawPtr, pistm: ::windows::runtime::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn OpenRegStream(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseURLA(pcszurl: super::super::Foundation::PSTR, ppu: *mut PARSEDURLA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseURLW(pcszurl: super::super::Foundation::PWSTR, ppu: *mut PARSEDURLW) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddBackslashA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddBackslashW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddExtensionA(pszpath: super::super::Foundation::PSTR, pszext: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddExtensionW(pszpath: super::super::Foundation::PWSTR, pszext: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAllocCanonicalize(pszpathin: super::super::Foundation::PWSTR, dwflags: u32, ppszpathout: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAllocCombine(pszpathin: super::super::Foundation::PWSTR, pszmore: super::super::Foundation::PWSTR, dwflags: u32, ppszpathout: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAppendA(pszpath: super::super::Foundation::PSTR, pszmore: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAppendW(pszpath: super::super::Foundation::PWSTR, pszmore: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathBuildRootA(pszroot: super::super::Foundation::PSTR, idrive: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathBuildRootW(pszroot: super::super::Foundation::PWSTR, idrive: i32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCanonicalizeA(pszbuf: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCanonicalizeW(pszbuf: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAddBackslash(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAddBackslashEx(pszpath: super::super::Foundation::PWSTR, cchpath: usize, ppszend: *mut super::super::Foundation::PWSTR, pcchremaining: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAddExtension(pszpath: super::super::Foundation::PWSTR, cchpath: usize, pszext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAppend(pszpath: super::super::Foundation::PWSTR, cchpath: usize, pszmore: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAppendEx(pszpath: super::super::Foundation::PWSTR, cchpath: usize, pszmore: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCanonicalize(pszpathout: super::super::Foundation::PWSTR, cchpathout: usize, pszpathin: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCanonicalizeEx(pszpathout: super::super::Foundation::PWSTR, cchpathout: usize, pszpathin: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCombine(pszpathout: super::super::Foundation::PWSTR, cchpathout: usize, pszpathin: super::super::Foundation::PWSTR, pszmore: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCombineEx(pszpathout: super::super::Foundation::PWSTR, cchpathout: usize, pszpathin: super::super::Foundation::PWSTR, pszmore: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchFindExtension(pszpath: super::super::Foundation::PWSTR, cchpath: usize, ppszext: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchIsRoot(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveBackslash(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveBackslashEx(pszpath: super::super::Foundation::PWSTR, cchpath: usize, ppszend: *mut super::super::Foundation::PWSTR, pcchremaining: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveExtension(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveFileSpec(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRenameExtension(pszpath: super::super::Foundation::PWSTR, cchpath: usize, pszext: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchSkipRoot(pszpath: super::super::Foundation::PWSTR, ppszrootend: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchStripPrefix(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchStripToRoot(pszpath: super::super::Foundation::PWSTR, cchpath: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCleanupSpec(pszdir: super::super::Foundation::PWSTR, pszspec: super::super::Foundation::PWSTR) -> PCS_RET;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCombineA(pszdest: super::super::Foundation::PSTR, pszdir: super::super::Foundation::PSTR, pszfile: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCombineW(pszdest: super::super::Foundation::PWSTR, pszdir: super::super::Foundation::PWSTR, pszfile: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCommonPrefixA(pszfile1: super::super::Foundation::PSTR, pszfile2: super::super::Foundation::PSTR, achpath: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCommonPrefixW(pszfile1: super::super::Foundation::PWSTR, pszfile2: super::super::Foundation::PWSTR, achpath: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PathCompactPathA(hdc: super::super::Graphics::Gdi::HDC, pszpath: super::super::Foundation::PSTR, dx: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCompactPathExA(pszout: super::super::Foundation::PSTR, pszsrc: super::super::Foundation::PSTR, cchmax: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCompactPathExW(pszout: super::super::Foundation::PWSTR, pszsrc: super::super::Foundation::PWSTR, cchmax: u32, dwflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PathCompactPathW(hdc: super::super::Graphics::Gdi::HDC, pszpath: super::super::Foundation::PWSTR, dx: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCreateFromUrlA(pszurl: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR, pcchpath: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCreateFromUrlAlloc(pszin: super::super::Foundation::PWSTR, ppszout: *mut super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCreateFromUrlW(pszurl: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR, pcchpath: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFileExistsA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFileExistsW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindExtensionA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindExtensionW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindFileNameA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindFileNameW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindNextComponentA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindNextComponentW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindOnPathA(pszpath: super::super::Foundation::PSTR, ppszotherdirs: *const *const i8) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindOnPathW(pszpath: super::super::Foundation::PWSTR, ppszotherdirs: *const *const u16) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindSuffixArrayA(pszpath: super::super::Foundation::PSTR, apszsuffix: *const super::super::Foundation::PSTR, iarraysize: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindSuffixArrayW(pszpath: super::super::Foundation::PWSTR, apszsuffix: *const super::super::Foundation::PWSTR, iarraysize: i32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetArgsA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetArgsW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn PathGetCharTypeA(ch: u8) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn PathGetCharTypeW(ch: u16) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetDriveNumberA(pszpath: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetDriveNumberW(pszpath: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetShortPath(pszlongpath: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsContentTypeA(pszpath: super::super::Foundation::PSTR, pszcontenttype: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsContentTypeW(pszpath: super::super::Foundation::PWSTR, pszcontenttype: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryEmptyA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryEmptyW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsExe(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsFileSpecA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsFileSpecW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsLFNFileSpecA(pszname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsLFNFileSpecW(pszname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsNetworkPathA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsNetworkPathW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsPrefixA(pszprefix: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsPrefixW(pszprefix: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRelativeA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRelativeW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRootA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRootW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSameRootA(pszpath1: super::super::Foundation::PSTR, pszpath2: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSameRootW(pszpath1: super::super::Foundation::PWSTR, pszpath2: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSlowA(pszfile: super::super::Foundation::PSTR, dwattr: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSlowW(pszfile: super::super::Foundation::PWSTR, dwattr: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSystemFolderA(pszpath: super::super::Foundation::PSTR, dwattrb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSystemFolderW(pszpath: super::super::Foundation::PWSTR, dwattrb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCEx(pszpath: super::super::Foundation::PWSTR, ppszserver: *mut super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerShareA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerShareW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsURLA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsURLW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakePrettyA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakePrettyW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakeSystemFolderA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakeSystemFolderW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakeUniqueName(pszuniquename: super::super::Foundation::PWSTR, cchmax: u32, psztemplate: super::super::Foundation::PWSTR, pszlongplate: super::super::Foundation::PWSTR, pszdir: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecA(pszfile: super::super::Foundation::PSTR, pszspec: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecExA(pszfile: super::super::Foundation::PSTR, pszspec: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecExW(pszfile: super::super::Foundation::PWSTR, pszspec: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecW(pszfile: super::super::Foundation::PWSTR, pszspec: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathParseIconLocationA(psziconfile: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathParseIconLocationW(psziconfile: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathQualify(psz: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathQuoteSpacesA(lpsz: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathQuoteSpacesW(lpsz: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRelativePathToA(pszpath: super::super::Foundation::PSTR, pszfrom: super::super::Foundation::PSTR, dwattrfrom: u32, pszto: super::super::Foundation::PSTR, dwattrto: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRelativePathToW(pszpath: super::super::Foundation::PWSTR, pszfrom: super::super::Foundation::PWSTR, dwattrfrom: u32, pszto: super::super::Foundation::PWSTR, dwattrto: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveArgsA(pszpath: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveArgsW(pszpath: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBackslashA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBackslashW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBlanksA(pszpath: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBlanksW(pszpath: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveExtensionA(pszpath: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveExtensionW(pszpath: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveFileSpecA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveFileSpecW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRenameExtensionA(pszpath: super::super::Foundation::PSTR, pszext: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRenameExtensionW(pszpath: super::super::Foundation::PWSTR, pszext: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathResolve(pszpath: super::super::Foundation::PWSTR, dirs: *const *const u16, fflags: PRF_FLAGS) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSearchAndQualifyA(pszpath: super::super::Foundation::PSTR, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSearchAndQualifyW(pszpath: super::super::Foundation::PWSTR, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSetDlgItemPathA(hdlg: super::super::Foundation::HWND, id: i32, pszpath: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSetDlgItemPathW(hdlg: super::super::Foundation::HWND, id: i32, pszpath: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSkipRootA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSkipRootW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripPathA(pszpath: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripPathW(pszpath: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripToRootA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripToRootW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnExpandEnvStringsA(pszpath: super::super::Foundation::PSTR, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnExpandEnvStringsW(pszpath: super::super::Foundation::PWSTR, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUndecorateA(pszpath: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUndecorateW(pszpath: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnmakeSystemFolderA(pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnmakeSystemFolderW(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnquoteSpacesA(lpsz: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnquoteSpacesW(lpsz: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathYetAnotherMakeUniqueName(pszuniquename: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR, pszshort: super::super::Foundation::PWSTR, pszfilespec: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PickIconDlg(hwnd: super::super::Foundation::HWND, psziconpath: super::super::Foundation::PWSTR, cchiconpath: u32, piiconindex: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn QISearch(that: *mut ::core::ffi::c_void, pqit: *const QITAB, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadCabinetState(pcs: *mut CABINETSTATE, clength: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RealDriveType(idrive: i32, foktohitnet: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppConstrainedChangeNotification(routine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, registration: *mut *mut _APPCONSTRAIN_REGISTRATION) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppStateChangeNotification(routine: ::windows::runtime::RawPtr, context: *const ::core::ffi::c_void, registration: *mut *mut _APPSTATE_REGISTRATION) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterScaleChangeEvent(hevent: super::super::Foundation::HANDLE, pdwcookie: *mut usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterScaleChangeNotifications(displaydevice: DISPLAY_DEVICE_TYPE, hwndnotify: super::super::Foundation::HWND, umsgnotify: u32, pdwcookie: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveWindowSubclass(hwnd: super::super::Foundation::HWND, pfnsubclass: ::windows::runtime::RawPtr, uidsubclass: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestartDialog(hwnd: super::super::Foundation::HWND, pszprompt: super::super::Foundation::PWSTR, dwreturn: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestartDialogEx(hwnd: super::super::Foundation::HWND, pszprompt: super::super::Foundation::PWSTR, dwreturn: u32, dwreasoncode: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn RevokeScaleChangeNotifications(displaydevice: DISPLAY_DEVICE_TYPE, dwcookie: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SHAddFromPropSheetExtArray(hpsxa: HPSXA, lpfnaddpage: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHAddToRecentDocs(uflags: u32, pv: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHAlloc(cb: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAllocShared(pvdata: *const ::core::ffi::c_void, dwsize: u32, dwprocessid: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAnsiToAnsi(pszsrc: super::super::Foundation::PSTR, pszdst: super::super::Foundation::PSTR, cchbuf: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAnsiToUnicode(pszsrc: super::super::Foundation::PSTR, pwszdst: super::super::Foundation::PWSTR, cwchbuf: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAppBarMessage(dwmessage: u32, pdata: *mut APPBARDATA) -> usize;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAssocEnumHandlers(pszextra: super::super::Foundation::PWSTR, affilter: ASSOC_FILTER, ppenumhandler: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAssocEnumHandlersForProtocolByApplication(protocol: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, enumhandlers: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAutoComplete(hwndedit: super::super::Foundation::HWND, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHBindToFolderIDListParent(psfroot: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void, ppidllast: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBindToFolderIDListParentEx(psfroot: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST, ppbc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void, ppidllast: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBindToObject(psf: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST, pbc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHBindToParent(pidl: *const Common::ITEMIDLIST, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void, ppidllast: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBrowseForFolderA(lpbi: *const ::core::mem::ManuallyDrop<BROWSEINFOA>) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBrowseForFolderW(lpbi: *const ::core::mem::ManuallyDrop<BROWSEINFOW>) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCLSIDFromString(psz: super::super::Foundation::PWSTR, pclsid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHChangeNotification_Lock(hchange: super::super::Foundation::HANDLE, dwprocid: u32, pppidl: *mut *mut *mut Common::ITEMIDLIST, plevent: *mut i32) -> ShFindChangeNotificationHandle;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHChangeNotification_Unlock(hlock: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHChangeNotify(weventid: SHCNE_ID, uflags: SHCNF_FLAGS, dwitem1: *const ::core::ffi::c_void, dwitem2: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHChangeNotifyDeregister(ulid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHChangeNotifyRegister(hwnd: super::super::Foundation::HWND, fsources: SHCNRF_SOURCE, fevents: i32, wmsg: u32, centries: i32, pshcne: *const SHChangeNotifyEntry) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHChangeNotifyRegisterThread(status: SCNRT_STATUS);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCloneSpecialIDList(hwnd: super::super::Foundation::HWND, csidl: i32, fcreate: super::super::Foundation::BOOL) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCoCreateInstance(pszclsid: super::super::Foundation::PWSTR, pclsid: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHCopyKeyA(hkeysrc: super::super::System::Registry::HKEY, pszsrcsubkey: super::super::Foundation::PSTR, hkeydest: super::super::System::Registry::HKEY, freserved: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHCopyKeyW(hkeysrc: super::super::System::Registry::HKEY, pszsrcsubkey: super::super::Foundation::PWSTR, hkeydest: super::super::System::Registry::HKEY, freserved: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateAssociationRegistration(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCreateDataObject(pidlfolder: *const Common::ITEMIDLIST, cidl: u32, apidl: *const *const Common::ITEMIDLIST, pdtinner: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCreateDefaultContextMenu(pdcm: *const ::core::mem::ManuallyDrop<DEFCONTEXTMENU>, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateDefaultExtractIcon(riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateDefaultPropertiesOp(psi: ::windows::runtime::RawPtr, ppfileop: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCreateDirectory(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SHCreateDirectoryExA(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PSTR, psa: *const super::super::Security::SECURITY_ATTRIBUTES) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SHCreateDirectoryExW(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PWSTR, psa: *const super::super::Security::SECURITY_ATTRIBUTES) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCreateFileExtractIconW(pszfile: super::super::Foundation::PWSTR, dwfileattributes: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateItemFromIDList(pidl: *const Common::ITEMIDLIST, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateItemFromParsingName(pszpath: super::super::Foundation::PWSTR, pbc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateItemFromRelativeName(psiparent: ::windows::runtime::RawPtr, pszname: super::super::Foundation::PWSTR, pbc: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCreateItemInKnownFolder(kfid: *const ::windows::runtime::GUID, dwkfflags: u32, pszitem: super::super::Foundation::PWSTR, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateItemWithParent(pidlparent: *const Common::ITEMIDLIST, psfparent: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST, riid: *const ::windows::runtime::GUID, ppvitem: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateMemStream(pinit: *const u8, cbinit: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Threading"))]
    pub fn SHCreateProcessAsUserW(pscpi: *mut SHCREATEPROCESSINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHCreatePropSheetExtArray(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, max_iface: u32) -> HPSXA;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateQueryCancelAutoPlayMoniker(ppmoniker: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateShellFolderView(pcsfv: *const ::core::mem::ManuallyDrop<SFV_CREATE>, ppsv: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCreateShellFolderViewEx(pcsfv: *const ::core::mem::ManuallyDrop<CSFV>, ppsv: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateShellItem(pidlparent: *const Common::ITEMIDLIST, psfparent: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST, ppsi: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateShellItemArray(pidlparent: *const Common::ITEMIDLIST, psf: ::windows::runtime::RawPtr, cidl: u32, ppidl: *const *const Common::ITEMIDLIST, ppsiitemarray: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateShellItemArrayFromDataObject(pdo: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateShellItemArrayFromIDLists(cidl: u32, rgpidl: *const *const Common::ITEMIDLIST, ppsiitemarray: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateShellItemArrayFromShellItem(psi: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SHCreateShellPalette(hdc: super::super::Graphics::Gdi::HDC) -> super::super::Graphics::Gdi::HPALETTE;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateStdEnumFmtEtc(cfmt: u32, afmt: *const super::super::System::Com::FORMATETC, ppenumformatetc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateStreamOnFileA(pszfile: super::super::Foundation::PSTR, grfmode: u32, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateStreamOnFileEx(pszfile: super::super::Foundation::PWSTR, grfmode: u32, dwattributes: u32, fcreate: super::super::Foundation::BOOL, pstmtemplate: ::windows::runtime::RawPtr, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateStreamOnFileW(pszfile: super::super::Foundation::PWSTR, grfmode: u32, ppstm: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn SHCreateThread(pfnthreadproc: ::windows::runtime::RawPtr, pdata: *const ::core::ffi::c_void, flags: u32, pfncallback: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateThreadRef(pcref: *mut i32, ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn SHCreateThreadWithHandle(pfnthreadproc: ::windows::runtime::RawPtr, pdata: *const ::core::ffi::c_void, flags: u32, pfncallback: ::windows::runtime::RawPtr, phandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHDefExtractIconA(psziconfile: super::super::Foundation::PSTR, iindex: i32, uflags: u32, phiconlarge: *mut super::WindowsAndMessaging::HICON, phiconsmall: *mut super::WindowsAndMessaging::HICON, niconsize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHDefExtractIconW(psziconfile: super::super::Foundation::PWSTR, iindex: i32, uflags: u32, phiconlarge: *mut super::WindowsAndMessaging::HICON, phiconsmall: *mut super::WindowsAndMessaging::HICON, niconsize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteEmptyKeyA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteEmptyKeyW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteKeyA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteKeyW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteValueA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteValueW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHDestroyPropSheetExtArray(hpsxa: HPSXA);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn SHDoDragDrop(hwnd: super::super::Foundation::HWND, pdata: ::windows::runtime::RawPtr, pdsrc: ::windows::runtime::RawPtr, dweffect: u32, pdweffect: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHEmptyRecycleBinA(hwnd: super::super::Foundation::HWND, pszrootpath: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHEmptyRecycleBinW(hwnd: super::super::Foundation::HWND, pszrootpath: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumKeyExA(hkey: super::super::System::Registry::HKEY, dwindex: u32, pszname: super::super::Foundation::PSTR, pcchname: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumKeyExW(hkey: super::super::System::Registry::HKEY, dwindex: u32, pszname: super::super::Foundation::PWSTR, pcchname: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumValueA(hkey: super::super::System::Registry::HKEY, dwindex: u32, pszvaluename: super::super::Foundation::PSTR, pcchvaluename: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumValueW(hkey: super::super::System::Registry::HKEY, dwindex: u32, pszvaluename: super::super::Foundation::PWSTR, pcchvaluename: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumerateUnreadMailAccountsW(hkeyuser: super::super::System::Registry::HKEY, dwindex: u32, pszmailaddress: super::super::Foundation::PWSTR, cchmailaddress: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHEvaluateSystemCommandTemplate(pszcmdtemplate: super::super::Foundation::PWSTR, ppszapplication: *mut super::super::Foundation::PWSTR, ppszcommandline: *mut super::super::Foundation::PWSTR, ppszparameters: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFileOperationA(lpfileop: *mut SHFILEOPSTRUCTA) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFileOperationW(lpfileop: *mut SHFILEOPSTRUCTW) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHFindFiles(pidlfolder: *const Common::ITEMIDLIST, pidlsavefile: *const Common::ITEMIDLIST) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHFind_InitMenuPopup(hmenu: super::WindowsAndMessaging::HMENU, hwndowner: super::super::Foundation::HWND, idcmdfirst: u32, idcmdlast: u32) -> ::core::option::Option<IContextMenu>;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHFlushSFCache();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFormatDateTimeA(pft: *const super::super::Foundation::FILETIME, pdwflags: *mut u32, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFormatDateTimeW(pft: *const super::super::Foundation::FILETIME, pdwflags: *mut u32, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFormatDrive(hwnd: super::super::Foundation::HWND, drive: u32, fmtid: SHFMT_ID, options: SHFMT_OPT) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHFree(pv: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFreeNameMappings(hnamemappings: super::super::Foundation::HANDLE);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFreeShared(hdata: super::super::Foundation::HANDLE, dwprocessid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHGetAttributesFromDataObject(pdo: ::windows::runtime::RawPtr, dwattributemask: u32, pdwattributes: *mut u32, pcitems: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetDataFromIDListA(psf: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST, nformat: SHGDFIL_FORMAT, pv: *mut ::core::ffi::c_void, cb: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetDataFromIDListW(psf: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST, nformat: SHGDFIL_FORMAT, pv: *mut ::core::ffi::c_void, cb: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetDesktopFolder(ppshf: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetDiskFreeSpaceExA(pszdirectoryname: super::super::Foundation::PSTR, pulfreebytesavailabletocaller: *mut u64, pultotalnumberofbytes: *mut u64, pultotalnumberoffreebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetDiskFreeSpaceExW(pszdirectoryname: super::super::Foundation::PWSTR, pulfreebytesavailabletocaller: *mut u64, pultotalnumberofbytes: *mut u64, pultotalnumberoffreebytes: *mut u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetDriveMedia(pszdrive: super::super::Foundation::PWSTR, pdwmediacontent: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHGetFileInfoA(pszpath: super::super::Foundation::PSTR, dwfileattributes: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, psfi: *mut SHFILEINFOA, cbfileinfo: u32, uflags: SHGFI_FLAGS) -> usize;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHGetFileInfoW(pszpath: super::super::Foundation::PWSTR, dwfileattributes: super::super::Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES, psfi: *mut SHFILEINFOW, cbfileinfo: u32, uflags: SHGFI_FLAGS) -> usize;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetFolderLocation(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, ppidl: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathA(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszpath: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathAndSubDirA(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszsubdir: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathAndSubDirW(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszsubdir: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathW(hwnd: super::super::Foundation::HWND, csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetIDListFromObject(punk: ::windows::runtime::RawPtr, ppidl: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetIconOverlayIndexA(psziconpath: super::super::Foundation::PSTR, iiconindex: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetIconOverlayIndexW(psziconpath: super::super::Foundation::PWSTR, iiconindex: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetImageList(iimagelist: i32, riid: *const ::windows::runtime::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetInstanceExplorer(ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetInverseCMAP(pbmap: *mut u8, cbmap: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHGetItemFromDataObject(pdtobj: ::windows::runtime::RawPtr, dwflags: DATAOBJ_GET_ITEM_FLAGS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetItemFromObject(punk: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetKnownFolderIDList(rfid: *const ::windows::runtime::GUID, dwflags: u32, htoken: super::super::Foundation::HANDLE, ppidl: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetKnownFolderItem(rfid: *const ::windows::runtime::GUID, flags: KNOWN_FOLDER_FLAG, htoken: super::super::Foundation::HANDLE, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetKnownFolderPath(rfid: *const ::windows::runtime::GUID, dwflags: u32, htoken: super::super::Foundation::HANDLE, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetLocalizedName(pszpath: super::super::Foundation::PWSTR, pszresmodule: super::super::Foundation::PWSTR, cch: u32, pidsres: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHGetMalloc(ppmalloc: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetNameFromIDList(pidl: *const Common::ITEMIDLIST, sigdnname: SIGDN, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetNewLinkInfoA(pszlinkto: super::super::Foundation::PSTR, pszdir: super::super::Foundation::PSTR, pszname: super::super::Foundation::PSTR, pfmustcopy: *mut super::super::Foundation::BOOL, uflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetNewLinkInfoW(pszlinkto: super::super::Foundation::PWSTR, pszdir: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, pfmustcopy: *mut super::super::Foundation::BOOL, uflags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetPathFromIDListA(pidl: *const Common::ITEMIDLIST, pszpath: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetPathFromIDListEx(pidl: *const Common::ITEMIDLIST, pszpath: super::super::Foundation::PWSTR, cchpath: u32, uopts: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetPathFromIDListW(pidl: *const Common::ITEMIDLIST, pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetRealIDL(psf: ::windows::runtime::RawPtr, pidlsimple: *const Common::ITEMIDLIST, ppidlreal: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSetFolderCustomSettings(pfcs: *mut SHFOLDERCUSTOMSETTINGS, pszpath: super::super::Foundation::PWSTR, dwreadwrite: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSetSettings(lpss: *mut SHELLSTATEA, dwmask: SSF_MASK, bset: super::super::Foundation::BOOL);
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetSettings(psfs: *mut SHELLFLAGSTATE, dwmask: u32);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetSpecialFolderLocation(hwnd: super::super::Foundation::HWND, csidl: i32, ppidl: *mut *mut Common::ITEMIDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSpecialFolderPathA(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PSTR, csidl: i32, fcreate: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSpecialFolderPathW(hwnd: super::super::Foundation::HWND, pszpath: super::super::Foundation::PWSTR, csidl: i32, fcreate: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn SHGetStockIconInfo(siid: SHSTOCKICONID, uflags: u32, psii: *mut SHSTOCKICONINFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn SHGetTemporaryPropertyForItem(psi: ::windows::runtime::RawPtr, propkey: *const PropertiesSystem::PROPERTYKEY, ppropvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetThreadRef(ppunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHGetUnreadMailCountW(hkeyuser: super::super::System::Registry::HKEY, pszmailaddress: super::super::Foundation::PWSTR, pdwcount: *mut u32, pfiletime: *mut super::super::Foundation::FILETIME, pszshellexecutecommand: super::super::Foundation::PWSTR, cchshellexecutecommand: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHGetValueA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHGetValueW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetViewStatePropertyBag(pidl: *const Common::ITEMIDLIST, pszbagname: super::super::Foundation::PWSTR, dwflags: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGlobalCounterDecrement(id: SHGLOBALCOUNTER) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGlobalCounterGetValue(id: SHGLOBALCOUNTER) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGlobalCounterIncrement(id: SHGLOBALCOUNTER) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHHandleUpdateImage(pidlextra: *const Common::ITEMIDLIST) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHILCreateFromPath(pszpath: super::super::Foundation::PWSTR, ppidl: *mut *mut Common::ITEMIDLIST, rgfinout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHInvokePrinterCommandA(hwnd: super::super::Foundation::HWND, uaction: u32, lpbuf1: super::super::Foundation::PSTR, lpbuf2: super::super::Foundation::PSTR, fmodal: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHInvokePrinterCommandW(hwnd: super::super::Foundation::HWND, uaction: u32, lpbuf1: super::super::Foundation::PWSTR, lpbuf2: super::super::Foundation::PWSTR, fmodal: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHIsFileAvailableOffline(pwszpath: super::super::Foundation::PWSTR, pdwstatus: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHIsLowMemoryMachine(dwtype: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHLimitInputEdit(hwndedit: super::super::Foundation::HWND, psf: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHLoadInProc(rclsid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHLoadIndirectString(pszsource: super::super::Foundation::PWSTR, pszoutbuf: super::super::Foundation::PWSTR, cchoutbuf: u32, ppvreserved: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHLoadNonloadedIconOverlayIdentifiers() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHLockShared(hdata: super::super::Foundation::HANDLE, dwprocessid: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHMapPIDLToSystemImageListIndex(pshf: ::windows::runtime::RawPtr, pidl: *const Common::ITEMIDLIST, piindexsel: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHMessageBoxCheckA(hwnd: super::super::Foundation::HWND, psztext: super::super::Foundation::PSTR, pszcaption: super::super::Foundation::PSTR, utype: u32, idefault: i32, pszregval: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHMessageBoxCheckW(hwnd: super::super::Foundation::HWND, psztext: super::super::Foundation::PWSTR, pszcaption: super::super::Foundation::PWSTR, utype: u32, idefault: i32, pszregval: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHMultiFileProperties(pdtobj: ::windows::runtime::RawPtr, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHObjectProperties(hwnd: super::super::Foundation::HWND, shopobjecttype: SHOP_TYPE, pszobjectname: super::super::Foundation::PWSTR, pszpropertypage: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHOpenFolderAndSelectItems(pidlfolder: *const Common::ITEMIDLIST, cidl: u32, apidl: *const *const Common::ITEMIDLIST, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenPropSheetW(pszcaption: super::super::Foundation::PWSTR, ahkeys: *const super::super::System::Registry::HKEY, ckeys: u32, pclsiddefault: *const ::windows::runtime::GUID, pdtobj: ::windows::runtime::RawPtr, psb: ::windows::runtime::RawPtr, pstartpage: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStream2A(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStream2W(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStreamA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStreamW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, grfmode: u32) -> ::core::option::Option<super::super::System::Com::IStream>;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHOpenWithDialog(hwndparent: super::super::Foundation::HWND, poainfo: *const OPENASINFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHParseDisplayName(pszname: super::super::Foundation::PWSTR, pbc: ::windows::runtime::RawPtr, ppidl: *mut *mut Common::ITEMIDLIST, sfgaoin: u32, psfgaoout: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHPathPrepareForWriteA(hwnd: super::super::Foundation::HWND, punkenablemodless: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHPathPrepareForWriteW(hwnd: super::super::Foundation::HWND, punkenablemodless: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryInfoKeyA(hkey: super::super::System::Registry::HKEY, pcsubkeys: *mut u32, pcchmaxsubkeylen: *mut u32, pcvalues: *mut u32, pcchmaxvaluenamelen: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryInfoKeyW(hkey: super::super::System::Registry::HKEY, pcsubkeys: *mut u32, pcchmaxsubkeylen: *mut u32, pcvalues: *mut u32, pcchmaxvaluenamelen: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHQueryRecycleBinA(pszrootpath: super::super::Foundation::PSTR, pshqueryrbinfo: *mut SHQUERYRBINFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHQueryRecycleBinW(pszrootpath: super::super::Foundation::PWSTR, pshqueryrbinfo: *mut SHQUERYRBINFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHQueryUserNotificationState(pquns: *mut QUERY_USER_NOTIFICATION_STATE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryValueExA(hkey: super::super::System::Registry::HKEY, pszvalue: super::super::Foundation::PSTR, pdwreserved: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryValueExW(hkey: super::super::System::Registry::HKEY, pszvalue: super::super::Foundation::PWSTR, pdwreserved: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegCloseUSKey(huskey: isize) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegCreateUSKeyA(pszpath: super::super::Foundation::PSTR, samdesired: u32, hrelativeuskey: isize, phnewuskey: *mut isize, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegCreateUSKeyW(pwzpath: super::super::Foundation::PWSTR, samdesired: u32, hrelativeuskey: isize, phnewuskey: *mut isize, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteEmptyUSKeyA(huskey: isize, pszsubkey: super::super::Foundation::PSTR, delregflags: SHREGDEL_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteEmptyUSKeyW(huskey: isize, pwzsubkey: super::super::Foundation::PWSTR, delregflags: SHREGDEL_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteUSValueA(huskey: isize, pszvalue: super::super::Foundation::PSTR, delregflags: SHREGDEL_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteUSValueW(huskey: isize, pwzvalue: super::super::Foundation::PWSTR, delregflags: SHREGDEL_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SHRegDuplicateHKey(hkey: super::super::System::Registry::HKEY) -> super::super::System::Registry::HKEY;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSKeyA(huskey: isize, dwindex: u32, pszname: super::super::Foundation::PSTR, pcchname: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSKeyW(huskey: isize, dwindex: u32, pwzname: super::super::Foundation::PWSTR, pcchname: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSValueA(huskey: isize, dwindex: u32, pszvaluename: super::super::Foundation::PSTR, pcchvaluename: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSValueW(huskey: isize, dwindex: u32, pszvaluename: super::super::Foundation::PWSTR, pcchvaluename: *mut u32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetBoolUSValueA(pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, fignorehkcu: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetBoolUSValueW(pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, fignorehkcu: super::super::Foundation::BOOL, fdefault: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetIntW(hk: super::super::System::Registry::HKEY, pwzkey: super::super::Foundation::PWSTR, idefault: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetPathA(hkey: super::super::System::Registry::HKEY, pcszsubkey: super::super::Foundation::PSTR, pcszvalue: super::super::Foundation::PSTR, pszpath: super::super::Foundation::PSTR, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetPathW(hkey: super::super::System::Registry::HKEY, pcszsubkey: super::super::Foundation::PWSTR, pcszvalue: super::super::Foundation::PWSTR, pszpath: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetUSValueA(pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, fignorehkcu: super::super::Foundation::BOOL, pvdefaultdata: *const ::core::ffi::c_void, dwdefaultdatasize: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetUSValueW(pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, fignorehkcu: super::super::Foundation::BOOL, pvdefaultdata: *const ::core::ffi::c_void, dwdefaultdatasize: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetValueA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, srrfflags: i32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetValueFromHKCUHKLM(pwszkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR, srrfflags: i32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetValueW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, srrfflags: i32, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegOpenUSKeyA(pszpath: super::super::Foundation::PSTR, samdesired: u32, hrelativeuskey: isize, phnewuskey: *mut isize, fignorehkcu: super::super::Foundation::BOOL) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegOpenUSKeyW(pwzpath: super::super::Foundation::PWSTR, samdesired: u32, hrelativeuskey: isize, phnewuskey: *mut isize, fignorehkcu: super::super::Foundation::BOOL) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryInfoUSKeyA(huskey: isize, pcsubkeys: *mut u32, pcchmaxsubkeylen: *mut u32, pcvalues: *mut u32, pcchmaxvaluenamelen: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryInfoUSKeyW(huskey: isize, pcsubkeys: *mut u32, pcchmaxsubkeylen: *mut u32, pcvalues: *mut u32, pcchmaxvaluenamelen: *mut u32, enumregflags: SHREGENUM_FLAGS) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryUSValueA(huskey: isize, pszvalue: super::super::Foundation::PSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, fignorehkcu: super::super::Foundation::BOOL, pvdefaultdata: *const ::core::ffi::c_void, dwdefaultdatasize: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryUSValueW(huskey: isize, pszvalue: super::super::Foundation::PWSTR, pdwtype: *mut u32, pvdata: *mut ::core::ffi::c_void, pcbdata: *mut u32, fignorehkcu: super::super::Foundation::BOOL, pvdefaultdata: *const ::core::ffi::c_void, dwdefaultdatasize: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegSetPathA(hkey: super::super::System::Registry::HKEY, pcszsubkey: super::super::Foundation::PSTR, pcszvalue: super::super::Foundation::PSTR, pcszpath: super::super::Foundation::PSTR, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegSetPathW(hkey: super::super::System::Registry::HKEY, pcszsubkey: super::super::Foundation::PWSTR, pcszvalue: super::super::Foundation::PWSTR, pcszpath: super::super::Foundation::PWSTR, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegSetUSValueA(pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegSetUSValueW(pwzsubkey: super::super::Foundation::PWSTR, pwzvalue: super::super::Foundation::PWSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegWriteUSValueA(huskey: isize, pszvalue: super::super::Foundation::PSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegWriteUSValueW(huskey: isize, pwzvalue: super::super::Foundation::PWSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32, dwflags: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHReleaseThreadRef() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRemoveLocalizedName(pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SHReplaceFromPropSheetExtArray(hpsxa: HPSXA, upageid: u32, lpfnreplacewith: ::windows::runtime::RawPtr, lparam: super::super::Foundation::LPARAM) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHResolveLibrary(psilibrary: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHRestricted(rest: RESTRICTIONS) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSendMessageBroadcastA(umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSendMessageBroadcastW(umsg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetDefaultProperties(hwnd: super::super::Foundation::HWND, psi: ::windows::runtime::RawPtr, dwfileopflags: u32, pfops: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetFolderPathA(csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszpath: super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetFolderPathW(csidl: i32, htoken: super::super::Foundation::HANDLE, dwflags: u32, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHSetInstanceExplorer(punk: ::windows::runtime::RawPtr);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetKnownFolderPath(rfid: *const ::windows::runtime::GUID, dwflags: u32, htoken: super::super::Foundation::HANDLE, pszpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetLocalizedName(pszpath: super::super::Foundation::PWSTR, pszresmodule: super::super::Foundation::PWSTR, idsres: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn SHSetTemporaryPropertyForItem(psi: ::windows::runtime::RawPtr, propkey: *const PropertiesSystem::PROPERTYKEY, propvar: *const ::core::mem::ManuallyDrop<super::super::System::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHSetThreadRef(punk: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetUnreadMailCountW(pszmailaddress: super::super::Foundation::PWSTR, dwcount: u32, pszshellexecutecommand: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHSetValueA(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PSTR, pszvalue: super::super::Foundation::PSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHSetValueW(hkey: super::super::System::Registry::HKEY, pszsubkey: super::super::Foundation::PWSTR, pszvalue: super::super::Foundation::PWSTR, dwtype: u32, pvdata: *const ::core::ffi::c_void, cbdata: u32) -> super::super::Foundation::LSTATUS;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHShellFolderView_Message(hwndmain: super::super::Foundation::HWND, umsg: u32, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHShowManageLibraryUI(psilibrary: ::windows::runtime::RawPtr, hwndowner: super::super::Foundation::HWND, psztitle: super::super::Foundation::PWSTR, pszinstruction: super::super::Foundation::PWSTR, lmdoptions: LIBRARYMANAGEDIALOGOPTIONS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHSimpleIDListFromPath(pszpath: super::super::Foundation::PWSTR) -> *mut Common::ITEMIDLIST;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHSkipJunction(pbc: ::windows::runtime::RawPtr, pclsid: *const ::windows::runtime::GUID) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStartNetConnectionDialogW(hwnd: super::super::Foundation::HWND, pszremotename: super::super::Foundation::PWSTR, dwtype: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStrDupA(psz: super::super::Foundation::PSTR, ppwsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStrDupW(psz: super::super::Foundation::PWSTR, ppwsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStripMneumonicA(pszmenu: super::super::Foundation::PSTR) -> super::super::Foundation::CHAR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStripMneumonicW(pszmenu: super::super::Foundation::PWSTR) -> u16;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHTestTokenMembership(htoken: super::super::Foundation::HANDLE, ulrid: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUnicodeToAnsi(pwszsrc: super::super::Foundation::PWSTR, pszdst: super::super::Foundation::PSTR, cchbuf: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUnicodeToUnicode(pwzsrc: super::super::Foundation::PWSTR, pwzdst: super::super::Foundation::PWSTR, cwchbuf: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUnlockShared(pvdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUpdateImageA(pszhashitem: super::super::Foundation::PSTR, iindex: i32, uflags: u32, iimageindex: i32);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUpdateImageW(pszhashitem: super::super::Foundation::PWSTR, iindex: i32, uflags: u32, iimageindex: i32);
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHValidateUNC(hwndowner: super::super::Foundation::HWND, pszfile: super::super::Foundation::PWSTR, fconnect: VALIDATEUNC_OPTION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentProcessExplicitAppUserModelID(appid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetMenuContextHelpId(param0: super::WindowsAndMessaging::HMENU, param1: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowContextHelpId(param0: super::super::Foundation::HWND, param1: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowSubclass(hwnd: super::super::Foundation::HWND, pfnsubclass: ::windows::runtime::RawPtr, uidsubclass: usize, dwrefdata: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ShellAboutA(hwnd: super::super::Foundation::HWND, szapp: super::super::Foundation::PSTR, szotherstuff: super::super::Foundation::PSTR, hicon: super::WindowsAndMessaging::HICON) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ShellAboutW(hwnd: super::super::Foundation::HWND, szapp: super::super::Foundation::PWSTR, szotherstuff: super::super::Foundation::PWSTR, hicon: super::WindowsAndMessaging::HICON) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellExecuteA(hwnd: super::super::Foundation::HWND, lpoperation: super::super::Foundation::PSTR, lpfile: super::super::Foundation::PSTR, lpparameters: super::super::Foundation::PSTR, lpdirectory: super::super::Foundation::PSTR, nshowcmd: i32) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ShellExecuteExA(pexecinfo: *mut SHELLEXECUTEINFOA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ShellExecuteExW(pexecinfo: *mut SHELLEXECUTEINFOW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellExecuteW(hwnd: super::super::Foundation::HWND, lpoperation: super::super::Foundation::PWSTR, lpfile: super::super::Foundation::PWSTR, lpparameters: super::super::Foundation::PWSTR, lpdirectory: super::super::Foundation::PWSTR, nshowcmd: i32) -> super::super::Foundation::HINSTANCE;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellMessageBoxA(happinst: super::super::Foundation::HINSTANCE, hwnd: super::super::Foundation::HWND, lpctext: super::super::Foundation::PSTR, lpctitle: super::super::Foundation::PSTR, fustyle: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellMessageBoxW(happinst: super::super::Foundation::HINSTANCE, hwnd: super::super::Foundation::HWND, lpctext: super::super::Foundation::PWSTR, lpctitle: super::super::Foundation::PWSTR, fustyle: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_GetCachedImageIndex(pwsziconpath: super::super::Foundation::PWSTR, iiconindex: i32, uiconflags: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_GetCachedImageIndexA(psziconpath: super::super::Foundation::PSTR, iiconindex: i32, uiconflags: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_GetCachedImageIndexW(psziconpath: super::super::Foundation::PWSTR, iiconindex: i32, uiconflags: u32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn Shell_GetImageLists(phiml: *mut super::Controls::HIMAGELIST, phimlsmall: *mut super::Controls::HIMAGELIST) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn Shell_MergeMenus(hmdst: super::WindowsAndMessaging::HMENU, hmsrc: super::WindowsAndMessaging::HMENU, uinsert: u32, uidadjust: u32, uidadjustmax: u32, uflags: MM_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn Shell_NotifyIconA(dwmessage: NOTIFY_ICON_MESSAGE, lpdata: *const NOTIFYICONDATAA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_NotifyIconGetRect(identifier: *const NOTIFYICONIDENTIFIER, iconlocation: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn Shell_NotifyIconW(dwmessage: NOTIFY_ICON_MESSAGE, lpdata: *const NOTIFYICONDATAW) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SignalFileOpen(pidl: *const Common::ITEMIDLIST) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com_Urlmon`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_Urlmon"))]
    pub fn SoftwareUpdateMessageBox(hwnd: super::super::Foundation::HWND, pszdistunit: super::super::Foundation::PWSTR, dwflags: u32, psdi: *mut super::super::System::Com::Urlmon::SOFTDISTINFO) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn StgMakeUniqueName(pstgparent: ::windows::runtime::RawPtr, pszfilespec: super::super::Foundation::PWSTR, grfmode: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnA(pszstr: super::super::Foundation::PSTR, pszset: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnIA(pszstr: super::super::Foundation::PSTR, pszset: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnIW(pszstr: super::super::Foundation::PWSTR, pszset: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnW(pszstr: super::super::Foundation::PWSTR, pszset: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatBuffA(pszdest: super::super::Foundation::PSTR, pszsrc: super::super::Foundation::PSTR, cchdestbuffsize: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatBuffW(pszdest: super::super::Foundation::PWSTR, pszsrc: super::super::Foundation::PWSTR, cchdestbuffsize: i32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatChainW(pszdst: super::super::Foundation::PWSTR, cchdst: u32, ichat: u32, pszsrc: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrA(pszstart: super::super::Foundation::PSTR, wmatch: u16) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrIA(pszstart: super::super::Foundation::PSTR, wmatch: u16) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrIW(pszstart: super::super::Foundation::PWSTR, wmatch: u16) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrNIW(pszstart: super::super::Foundation::PWSTR, wmatch: u16, cchmax: u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrNW(pszstart: super::super::Foundation::PWSTR, wmatch: u16, cchmax: u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrW(pszstart: super::super::Foundation::PWSTR, wmatch: u16) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpCA(pszstr1: super::super::Foundation::PSTR, pszstr2: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpCW(pszstr1: super::super::Foundation::PWSTR, pszstr2: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpICA(pszstr1: super::super::Foundation::PSTR, pszstr2: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpICW(pszstr1: super::super::Foundation::PWSTR, pszstr2: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpIW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpLogicalW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNA(psz1: super::super::Foundation::PSTR, psz2: super::super::Foundation::PSTR, nchar: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNCA(pszstr1: super::super::Foundation::PSTR, pszstr2: super::super::Foundation::PSTR, nchar: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNCW(pszstr1: super::super::Foundation::PWSTR, pszstr2: super::super::Foundation::PWSTR, nchar: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNIA(psz1: super::super::Foundation::PSTR, psz2: super::super::Foundation::PSTR, nchar: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNICA(pszstr1: super::super::Foundation::PSTR, pszstr2: super::super::Foundation::PSTR, nchar: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNICW(pszstr1: super::super::Foundation::PWSTR, pszstr2: super::super::Foundation::PWSTR, nchar: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNIW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR, nchar: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR, nchar: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCpyNW(pszdst: super::super::Foundation::PWSTR, pszsrc: super::super::Foundation::PWSTR, cchmax: i32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCpyW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrDupA(pszsrch: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrDupW(pszsrch: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSize64A(qdw: i64, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSizeA(dw: u32, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSizeEx(ull: u64, flags: SFBS_FLAGS, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSizeW(qdw: i64, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatKBSizeA(qdw: i64, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatKBSizeW(qdw: i64, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFromTimeIntervalA(pszout: super::super::Foundation::PSTR, cchmax: u32, dwtimems: u32, digits: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFromTimeIntervalW(pszout: super::super::Foundation::PWSTR, cchmax: u32, dwtimems: u32, digits: i32) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrIsIntlEqualA(fcasesens: super::super::Foundation::BOOL, pszstring1: super::super::Foundation::PSTR, pszstring2: super::super::Foundation::PSTR, nchar: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrIsIntlEqualW(fcasesens: super::super::Foundation::BOOL, pszstring1: super::super::Foundation::PWSTR, pszstring2: super::super::Foundation::PWSTR, nchar: i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrNCatA(psz1: super::super::Foundation::PSTR, psz2: super::super::Foundation::PSTR, cchmax: i32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrNCatW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR, cchmax: i32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrPBrkA(psz: super::super::Foundation::PSTR, pszset: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrPBrkW(psz: super::super::Foundation::PWSTR, pszset: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrA(pszstart: super::super::Foundation::PSTR, pszend: super::super::Foundation::PSTR, wmatch: u16) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrIA(pszstart: super::super::Foundation::PSTR, pszend: super::super::Foundation::PSTR, wmatch: u16) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrIW(pszstart: super::super::Foundation::PWSTR, pszend: super::super::Foundation::PWSTR, wmatch: u16) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrW(pszstart: super::super::Foundation::PWSTR, pszend: super::super::Foundation::PWSTR, wmatch: u16) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRStrIA(pszsource: super::super::Foundation::PSTR, pszlast: super::super::Foundation::PSTR, pszsrch: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRStrIW(pszsource: super::super::Foundation::PWSTR, pszlast: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToBSTR(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, pbstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToBufA(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, pszbuf: super::super::Foundation::PSTR, cchbuf: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToBufW(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, pszbuf: super::super::Foundation::PWSTR, cchbuf: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToStrA(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, ppsz: *mut super::super::Foundation::PSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToStrW(pstr: *mut Common::STRRET, pidl: *const Common::ITEMIDLIST, ppsz: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrSpnA(psz: super::super::Foundation::PSTR, pszset: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrSpnW(psz: super::super::Foundation::PWSTR, pszset: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrA(pszfirst: super::super::Foundation::PSTR, pszsrch: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrIA(pszfirst: super::super::Foundation::PSTR, pszsrch: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrIW(pszfirst: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrNIW(pszfirst: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR, cchmax: u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrNW(pszfirst: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR, cchmax: u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrW(pszfirst: super::super::Foundation::PWSTR, pszsrch: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToInt64ExA(pszstring: super::super::Foundation::PSTR, dwflags: i32, pllret: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToInt64ExW(pszstring: super::super::Foundation::PWSTR, dwflags: i32, pllret: *mut i64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntA(pszsrc: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntExA(pszstring: super::super::Foundation::PSTR, dwflags: i32, piret: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntExW(pszstring: super::super::Foundation::PWSTR, dwflags: i32, piret: *mut i32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntW(pszsrc: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrTrimA(psz: super::super::Foundation::PSTR, psztrimchars: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrTrimW(psz: super::super::Foundation::PWSTR, psztrimchars: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadUserProfile(htoken: super::super::Foundation::HANDLE, hprofile: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn UnregisterAppConstrainedChangeNotification(registration: *mut _APPCONSTRAIN_REGISTRATION);
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn UnregisterAppStateChangeNotification(registration: *mut _APPSTATE_REGISTRATION);
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn UnregisterScaleChangeEvent(dwcookie: usize) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlApplySchemeA(pszin: super::super::Foundation::PSTR, pszout: super::super::Foundation::PSTR, pcchout: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlApplySchemeW(pszin: super::super::Foundation::PWSTR, pszout: super::super::Foundation::PWSTR, pcchout: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCanonicalizeA(pszurl: super::super::Foundation::PSTR, pszcanonicalized: super::super::Foundation::PSTR, pcchcanonicalized: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCanonicalizeW(pszurl: super::super::Foundation::PWSTR, pszcanonicalized: super::super::Foundation::PWSTR, pcchcanonicalized: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCombineA(pszbase: super::super::Foundation::PSTR, pszrelative: super::super::Foundation::PSTR, pszcombined: super::super::Foundation::PSTR, pcchcombined: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCombineW(pszbase: super::super::Foundation::PWSTR, pszrelative: super::super::Foundation::PWSTR, pszcombined: super::super::Foundation::PWSTR, pcchcombined: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCompareA(psz1: super::super::Foundation::PSTR, psz2: super::super::Foundation::PSTR, fignoreslash: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCompareW(psz1: super::super::Foundation::PWSTR, psz2: super::super::Foundation::PWSTR, fignoreslash: super::super::Foundation::BOOL) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCreateFromPathA(pszpath: super::super::Foundation::PSTR, pszurl: super::super::Foundation::PSTR, pcchurl: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCreateFromPathW(pszpath: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR, pcchurl: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlEscapeA(pszurl: super::super::Foundation::PSTR, pszescaped: super::super::Foundation::PSTR, pcchescaped: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlEscapeW(pszurl: super::super::Foundation::PWSTR, pszescaped: super::super::Foundation::PWSTR, pcchescaped: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlFixupW(pcszurl: super::super::Foundation::PWSTR, psztranslatedurl: super::super::Foundation::PWSTR, cchmax: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetLocationA(pszurl: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetLocationW(pszurl: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetPartA(pszin: super::super::Foundation::PSTR, pszout: super::super::Foundation::PSTR, pcchout: *mut u32, dwpart: u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetPartW(pszin: super::super::Foundation::PWSTR, pszout: super::super::Foundation::PWSTR, pcchout: *mut u32, dwpart: u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlHashA(pszurl: super::super::Foundation::PSTR, pbhash: *mut u8, cbhash: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlHashW(pszurl: super::super::Foundation::PWSTR, pbhash: *mut u8, cbhash: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsA(pszurl: super::super::Foundation::PSTR, urlis: URLIS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsNoHistoryA(pszurl: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsNoHistoryW(pszurl: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsOpaqueA(pszurl: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsOpaqueW(pszurl: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsW(pszurl: super::super::Foundation::PWSTR, urlis: URLIS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlUnescapeA(pszurl: super::super::Foundation::PSTR, pszunescaped: super::super::Foundation::PSTR, pcchunescaped: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlUnescapeW(pszurl: super::super::Foundation::PWSTR, pszunescaped: super::super::Foundation::PWSTR, pcchunescaped: *mut u32, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn WhichPlatform() -> u32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Win32DeleteFile(pszpath: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHelpA(hwndmain: super::super::Foundation::HWND, lpszhelp: super::super::Foundation::PSTR, ucommand: u32, dwdata: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHelpW(hwndmain: super::super::Foundation::HWND, lpszhelp: super::super::Foundation::PWSTR, ucommand: u32, dwdata: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteCabinetState(pcs: *const CABINETSTATE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wnsprintfA(pszdest: super::super::Foundation::PSTR, cchdest: i32, pszfmt: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wnsprintfW(pszdest: super::super::Foundation::PWSTR, cchdest: i32, pszfmt: super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvnsprintfA(pszdest: super::super::Foundation::PSTR, cchdest: i32, pszfmt: super::super::Foundation::PSTR, arglist: *const i8) -> i32;
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvnsprintfW(pszdest: super::super::Foundation::PWSTR, cchdest: i32, pszfmt: super::super::Foundation::PWSTR, arglist: *const i8) -> i32;
}
