#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BSTR_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CLIPFORMAT_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CLIPFORMAT_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CLIPFORMAT_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CLIPFORMAT_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CLIPFORMAT_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CLIPFORMAT_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CLIPFORMAT_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CLIPFORMAT_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CoGetMarshalSizeMax();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CoGetStandardMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CoGetStdMarshalEx();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CoMarshalHresult();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CoMarshalInterThreadInterfaceInStream();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CoMarshalInterface();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CoReleaseMarshalData();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CoUnmarshalHresult();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn CoUnmarshalInterface();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HACCEL_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HBITMAP_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HDC_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn HGLOBAL_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn HGLOBAL_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn HGLOBAL_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn HGLOBAL_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn HGLOBAL_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn HGLOBAL_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn HGLOBAL_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn HGLOBAL_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HICON_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn HMENU_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HPALETTE_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HWND_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn LPSAFEARRAY_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn LPSAFEARRAY_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn LPSAFEARRAY_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn LPSAFEARRAY_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn LPSAFEARRAY_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn LPSAFEARRAY_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn LPSAFEARRAY_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn LPSAFEARRAY_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn SNB_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn SNB_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn SNB_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn SNB_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn SNB_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn SNB_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn SNB_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`*"]
    pub fn SNB_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn STGMEDIUM_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserFree();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserFree64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserMarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserSize();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserSize64();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Com_Marshal`, `Win32_Foundation`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub fn VARIANT_UserUnmarshal64();
}
