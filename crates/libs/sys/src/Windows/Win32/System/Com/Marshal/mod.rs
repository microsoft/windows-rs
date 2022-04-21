#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::BSTR);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::BSTR);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::BSTR) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::BSTR) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::BSTR) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CLIPFORMAT_UserFree(param0: *const u32, param1: *const u16);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CLIPFORMAT_UserFree64(param0: *const u32, param1: *const u16);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CLIPFORMAT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CLIPFORMAT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const u16) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CLIPFORMAT_UserSize(param0: *const u32, param1: u32, param2: *const u16) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CLIPFORMAT_UserSize64(param0: *const u32, param1: u32, param2: *const u16) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CLIPFORMAT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CLIPFORMAT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut u16) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CoGetMarshalSizeMax(pulsize: *mut u32, riid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CoGetStandardMarshal(riid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32, ppmarshal: *mut IMarshal) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CoGetStdMarshalEx(punkouter: ::windows_sys::core::IUnknown, smexflags: u32, ppunkinner: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CoMarshalHresult(pstm: super::IStream, hresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CoMarshalInterThreadInterfaceInStream(riid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, ppstm: *mut super::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CoMarshalInterface(pstm: super::IStream, riid: *const ::windows_sys::core::GUID, punk: ::windows_sys::core::IUnknown, dwdestcontext: u32, pvdestcontext: *const ::core::ffi::c_void, mshlflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CoReleaseMarshalData(pstm: super::IStream) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CoUnmarshalHresult(pstm: super::IStream, phresult: *mut ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn CoUnmarshalInterface(pstm: super::IStream, riid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HACCEL);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HACCEL) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HACCEL) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HBITMAP);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HBITMAP) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HBITMAP) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HDC);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HDC) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HDC) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HDC) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn HGLOBAL_UserFree(param0: *const u32, param1: *const isize);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn HGLOBAL_UserFree64(param0: *const u32, param1: *const isize);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn HGLOBAL_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn HGLOBAL_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const isize) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn HGLOBAL_UserSize(param0: *const u32, param1: u32, param2: *const isize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn HGLOBAL_UserSize64(param0: *const u32, param1: u32, param2: *const isize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn HGLOBAL_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn HGLOBAL_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut isize) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HICON);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HICON) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HICON) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserFree(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserFree64(param0: *const u32, param1: *const super::super::super::UI::WindowsAndMessaging::HMENU);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::UI::WindowsAndMessaging::HMENU) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::UI::WindowsAndMessaging::HMENU) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserFree(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserFree64(param0: *const u32, param1: *const super::super::super::Graphics::Gdi::HPALETTE);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Graphics::Gdi::HPALETTE) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Graphics::Gdi::HPALETTE) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserFree(param0: *const u32, param1: *const super::super::super::Foundation::HWND);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserFree64(param0: *const u32, param1: *const super::super::super::Foundation::HWND);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::super::super::Foundation::HWND) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserSize(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserSize64(param0: *const u32, param1: u32, param2: *const super::super::super::Foundation::HWND) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::super::super::Foundation::HWND) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn LPSAFEARRAY_UserFree(param0: *const u32, param1: *const *const super::SAFEARRAY);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn LPSAFEARRAY_UserFree64(param0: *const u32, param1: *const *const super::SAFEARRAY);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn LPSAFEARRAY_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn LPSAFEARRAY_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const super::SAFEARRAY) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn LPSAFEARRAY_UserSize(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn LPSAFEARRAY_UserSize64(param0: *const u32, param1: u32, param2: *const *const super::SAFEARRAY) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn LPSAFEARRAY_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn LPSAFEARRAY_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut super::SAFEARRAY) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn SNB_UserFree(param0: *const u32, param1: *const *const *const u16);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn SNB_UserFree64(param0: *const u32, param1: *const *const *const u16);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn SNB_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn SNB_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const *const *const u16) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn SNB_UserSize(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn SNB_UserSize64(param0: *const u32, param1: u32, param2: *const *const *const u16) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn SNB_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
    pub fn SNB_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut *mut *mut u16) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserFree(param0: *const u32, param1: *const super::STGMEDIUM);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserFree64(param0: *const u32, param1: *const super::STGMEDIUM);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::STGMEDIUM) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserSize(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserSize64(param0: *const u32, param1: u32, param2: *const super::STGMEDIUM) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com_StructuredStorage\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::STGMEDIUM) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserFree(param0: *const u32, param1: *const super::VARIANT);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserFree64(param0: *const u32, param1: *const super::VARIANT);
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const super::VARIANT) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserSize(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserSize64(param0: *const u32, param1: u32, param2: *const super::VARIANT) -> u32;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8;
    #[doc = "*Required features: `\"Win32_System_Com_Marshal\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut super::VARIANT) -> *mut u8;
}
pub type IMarshal = *mut ::core::ffi::c_void;
pub type IMarshal2 = *mut ::core::ffi::c_void;
pub type IMarshalingStream = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
pub type STDMSHLFLAGS = i32;
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
pub const SMEXF_SERVER: STDMSHLFLAGS = 1i32;
#[doc = "*Required features: `\"Win32_System_Com_Marshal\"`*"]
pub const SMEXF_HANDLER: STDMSHLFLAGS = 2i32;
