windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserFree(param0 : *const u32, param1 : *const windows_sys::core::BSTR));
windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserFree64(param0 : *const u32, param1 : *const windows_sys::core::BSTR));
windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const windows_sys::core::BSTR) -> *mut u8);
windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const windows_sys::core::BSTR) -> *mut u8);
windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserSize(param0 : *const u32, param1 : u32, param2 : *const windows_sys::core::BSTR) -> u32);
windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserSize64(param0 : *const u32, param1 : u32, param2 : *const windows_sys::core::BSTR) -> u32);
windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
windows_targets::link!("oleaut32.dll" "system" fn BSTR_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut windows_sys::core::BSTR) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserFree(param0 : *const u32, param1 : *const u16));
windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserFree64(param0 : *const u32, param1 : *const u16));
windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const u16) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const u16) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserSize(param0 : *const u32, param1 : u32, param2 : *const u16) -> u32);
windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserSize64(param0 : *const u32, param1 : u32, param2 : *const u16) -> u32);
windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut u16) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn CLIPFORMAT_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut u16) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn CoGetMarshalSizeMax(pulsize : *mut u32, riid : *const windows_sys::core::GUID, punk : * mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoGetStandardMarshal(riid : *const windows_sys::core::GUID, punk : * mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32, ppmarshal : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoGetStdMarshalEx(punkouter : * mut core::ffi::c_void, smexflags : u32, ppunkinner : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoMarshalHresult(pstm : * mut core::ffi::c_void, hresult : windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoMarshalInterThreadInterfaceInStream(riid : *const windows_sys::core::GUID, punk : * mut core::ffi::c_void, ppstm : *mut * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoMarshalInterface(pstm : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, punk : * mut core::ffi::c_void, dwdestcontext : u32, pvdestcontext : *const core::ffi::c_void, mshlflags : u32) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoReleaseMarshalData(pstm : * mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoUnmarshalHresult(pstm : * mut core::ffi::c_void, phresult : *mut windows_sys::core::HRESULT) -> windows_sys::core::HRESULT);
windows_targets::link!("ole32.dll" "system" fn CoUnmarshalInterface(pstm : * mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HACCEL_UserFree(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL));
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HACCEL_UserFree64(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL));
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HACCEL_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HACCEL_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HACCEL_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL) -> u32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HACCEL_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HACCEL) -> u32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HACCEL_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HACCEL) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HACCEL_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HACCEL) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserFree(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HBITMAP));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserFree64(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HBITMAP));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HBITMAP) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HBITMAP) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HBITMAP) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HBITMAP) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HBITMAP) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HBITMAP_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HBITMAP) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HDC_UserFree(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HDC));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HDC_UserFree64(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HDC));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HDC_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HDC) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HDC_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HDC) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HDC_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HDC) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HDC_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HDC) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HDC_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HDC) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HDC_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HDC) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserFree(param0 : *const u32, param1 : *const super::super::super::Foundation:: HGLOBAL));
windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserFree64(param0 : *const u32, param1 : *const super::super::super::Foundation:: HGLOBAL));
windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Foundation:: HGLOBAL) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Foundation:: HGLOBAL) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Foundation:: HGLOBAL) -> u32);
windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Foundation:: HGLOBAL) -> u32);
windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Foundation:: HGLOBAL) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn HGLOBAL_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Foundation:: HGLOBAL) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HICON_UserFree(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HICON));
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HICON_UserFree64(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HICON));
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HICON_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HICON_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HICON_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON) -> u32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HICON_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HICON) -> u32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HICON_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HICON) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HICON_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HICON) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HMENU_UserFree(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HMENU));
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HMENU_UserFree64(param0 : *const u32, param1 : *const super::super::super::UI::WindowsAndMessaging:: HMENU));
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HMENU_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HMENU_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HMENU_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU) -> u32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HMENU_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::UI::WindowsAndMessaging:: HMENU) -> u32);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HMENU_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HMENU) -> *mut u8);
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
windows_targets::link!("ole32.dll" "system" fn HMENU_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::UI::WindowsAndMessaging:: HMENU) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserFree(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HPALETTE));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserFree64(param0 : *const u32, param1 : *const super::super::super::Graphics::Gdi:: HPALETTE));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HPALETTE) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Graphics::Gdi:: HPALETTE) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HPALETTE) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Graphics::Gdi:: HPALETTE) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HPALETTE) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn HPALETTE_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Graphics::Gdi:: HPALETTE) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn HWND_UserFree(param0 : *const u32, param1 : *const super::super::super::Foundation:: HWND));
windows_targets::link!("ole32.dll" "system" fn HWND_UserFree64(param0 : *const u32, param1 : *const super::super::super::Foundation:: HWND));
windows_targets::link!("ole32.dll" "system" fn HWND_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Foundation:: HWND) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn HWND_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super::super::super::Foundation:: HWND) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn HWND_UserSize(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Foundation:: HWND) -> u32);
windows_targets::link!("ole32.dll" "system" fn HWND_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super::super::super::Foundation:: HWND) -> u32);
windows_targets::link!("ole32.dll" "system" fn HWND_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Foundation:: HWND) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn HWND_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super::super::super::Foundation:: HWND) -> *mut u8);
windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserFree(param0 : *const u32, param1 : *const *const super:: SAFEARRAY));
windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserFree64(param0 : *const u32, param1 : *const *const super:: SAFEARRAY));
windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const *const super:: SAFEARRAY) -> *mut u8);
windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const *const super:: SAFEARRAY) -> *mut u8);
windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserSize(param0 : *const u32, param1 : u32, param2 : *const *const super:: SAFEARRAY) -> u32);
windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserSize64(param0 : *const u32, param1 : u32, param2 : *const *const super:: SAFEARRAY) -> u32);
windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut *mut super:: SAFEARRAY) -> *mut u8);
windows_targets::link!("oleaut32.dll" "system" fn LPSAFEARRAY_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut *mut super:: SAFEARRAY) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn SNB_UserFree(param0 : *const u32, param1 : *const *const *const u16));
windows_targets::link!("ole32.dll" "system" fn SNB_UserFree64(param0 : *const u32, param1 : *const *const *const u16));
windows_targets::link!("ole32.dll" "system" fn SNB_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const *const *const u16) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn SNB_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const *const *const u16) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn SNB_UserSize(param0 : *const u32, param1 : u32, param2 : *const *const *const u16) -> u32);
windows_targets::link!("ole32.dll" "system" fn SNB_UserSize64(param0 : *const u32, param1 : u32, param2 : *const *const *const u16) -> u32);
windows_targets::link!("ole32.dll" "system" fn SNB_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut *mut *mut u16) -> *mut u8);
windows_targets::link!("ole32.dll" "system" fn SNB_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut *mut *mut u16) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserFree(param0 : *const u32, param1 : *const super:: STGMEDIUM));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserFree64(param0 : *const u32, param1 : *const super:: STGMEDIUM));
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserMarshal(param0 : *const u32, param1 : *mut u8, param2 : *const super:: STGMEDIUM) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserMarshal64(param0 : *const u32, param1 : *mut u8, param2 : *const super:: STGMEDIUM) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserSize(param0 : *const u32, param1 : u32, param2 : *const super:: STGMEDIUM) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserSize64(param0 : *const u32, param1 : u32, param2 : *const super:: STGMEDIUM) -> u32);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserUnmarshal(param0 : *const u32, param1 : *const u8, param2 : *mut super:: STGMEDIUM) -> *mut u8);
#[cfg(feature = "Win32_Graphics_Gdi")]
windows_targets::link!("ole32.dll" "system" fn STGMEDIUM_UserUnmarshal64(param0 : *const u32, param1 : *const u8, param2 : *mut super:: STGMEDIUM) -> *mut u8);
pub const SMEXF_HANDLER: STDMSHLFLAGS = 2i32;
pub const SMEXF_SERVER: STDMSHLFLAGS = 1i32;
pub type STDMSHLFLAGS = i32;
