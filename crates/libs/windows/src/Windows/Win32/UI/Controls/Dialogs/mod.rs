#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChooseColorA(param0: *mut CHOOSECOLORA) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn ChooseColorA ( param0 : *mut CHOOSECOLORA ) -> super::super::super::Foundation:: BOOL );
    ChooseColorA(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChooseColorW(param0: *mut CHOOSECOLORW) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn ChooseColorW ( param0 : *mut CHOOSECOLORW ) -> super::super::super::Foundation:: BOOL );
    ChooseColorW(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ChooseFontA(param0: *mut CHOOSEFONTA) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn ChooseFontA ( param0 : *mut CHOOSEFONTA ) -> super::super::super::Foundation:: BOOL );
    ChooseFontA(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn ChooseFontW(param0: *mut CHOOSEFONTW) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn ChooseFontW ( param0 : *mut CHOOSEFONTW ) -> super::super::super::Foundation:: BOOL );
    ChooseFontW(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[inline]
pub unsafe fn CommDlgExtendedError() -> COMMON_DLG_ERRORS {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn CommDlgExtendedError ( ) -> COMMON_DLG_ERRORS );
    CommDlgExtendedError()
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn FindTextA ( param0 : *mut FINDREPLACEA ) -> super::super::super::Foundation:: HWND );
    FindTextA(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn FindTextW ( param0 : *mut FINDREPLACEW ) -> super::super::super::Foundation:: HWND );
    FindTextW(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[inline]
pub unsafe fn GetFileTitleA<P0>(param0: P0, buf: &mut [u8]) -> i16
where
    P0: ::windows::core::IntoParam<::windows::core::PCSTR>,
{
    ::windows_targets::link ! ( "comdlg32.dll""system" fn GetFileTitleA ( param0 : ::windows::core::PCSTR , buf : ::windows::core::PSTR , cchsize : u16 ) -> i16 );
    GetFileTitleA(param0.into_param().abi(), ::core::mem::transmute(buf.as_ptr()), buf.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[inline]
pub unsafe fn GetFileTitleW<P0>(param0: P0, buf: &mut [u16]) -> i16
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "comdlg32.dll""system" fn GetFileTitleW ( param0 : ::windows::core::PCWSTR , buf : ::windows::core::PWSTR , cchsize : u16 ) -> i16 );
    GetFileTitleW(param0.into_param().abi(), ::core::mem::transmute(buf.as_ptr()), buf.len() as _)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn GetOpenFileNameA ( param0 : *mut OPENFILENAMEA ) -> super::super::super::Foundation:: BOOL );
    GetOpenFileNameA(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetOpenFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn GetOpenFileNameW ( param0 : *mut OPENFILENAMEW ) -> super::super::super::Foundation:: BOOL );
    GetOpenFileNameW(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSaveFileNameA(param0: *mut OPENFILENAMEA) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn GetSaveFileNameA ( param0 : *mut OPENFILENAMEA ) -> super::super::super::Foundation:: BOOL );
    GetSaveFileNameA(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSaveFileNameW(param0: *mut OPENFILENAMEW) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn GetSaveFileNameW ( param0 : *mut OPENFILENAMEW ) -> super::super::super::Foundation:: BOOL );
    GetSaveFileNameW(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PageSetupDlgA(param0: *mut PAGESETUPDLGA) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn PageSetupDlgA ( param0 : *mut PAGESETUPDLGA ) -> super::super::super::Foundation:: BOOL );
    PageSetupDlgA(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PageSetupDlgW(param0: *mut PAGESETUPDLGW) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn PageSetupDlgW ( param0 : *mut PAGESETUPDLGW ) -> super::super::super::Foundation:: BOOL );
    PageSetupDlgW(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintDlgA(ppd: *mut PRINTDLGA) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn PrintDlgA ( ppd : *mut PRINTDLGA ) -> super::super::super::Foundation:: BOOL );
    PrintDlgA(ppd)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintDlgExA(ppd: *mut PRINTDLGEXA) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn PrintDlgExA ( ppd : *mut PRINTDLGEXA ) -> ::windows::core::HRESULT );
    PrintDlgExA(ppd).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintDlgExW(ppd: *mut PRINTDLGEXW) -> ::windows::core::Result<()> {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn PrintDlgExW ( ppd : *mut PRINTDLGEXW ) -> ::windows::core::HRESULT );
    PrintDlgExW(ppd).ok()
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
#[inline]
pub unsafe fn PrintDlgW(ppd: *mut PRINTDLGW) -> super::super::super::Foundation::BOOL {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn PrintDlgW ( ppd : *mut PRINTDLGW ) -> super::super::super::Foundation:: BOOL );
    PrintDlgW(ppd)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceTextA(param0: *mut FINDREPLACEA) -> super::super::super::Foundation::HWND {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn ReplaceTextA ( param0 : *mut FINDREPLACEA ) -> super::super::super::Foundation:: HWND );
    ReplaceTextA(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReplaceTextW(param0: *mut FINDREPLACEW) -> super::super::super::Foundation::HWND {
    ::windows_targets::link ! ( "comdlg32.dll""system" fn ReplaceTextW ( param0 : *mut FINDREPLACEW ) -> super::super::super::Foundation:: HWND );
    ReplaceTextW(param0)
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
pub struct IPrintDialogCallback(::windows::core::IUnknown);
impl IPrintDialogCallback {
    pub unsafe fn InitDone(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InitDone)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SelectionChange(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SelectionChange)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HandleMessage<P0, P1, P2>(&self, hdlg: P0, umsg: u32, wparam: P1, lparam: P2, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::super::Foundation::HWND>,
        P1: ::windows::core::IntoParam<super::super::super::Foundation::WPARAM>,
        P2: ::windows::core::IntoParam<super::super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).HandleMessage)(::windows::core::Interface::as_raw(self), hdlg.into_param().abi(), umsg, wparam.into_param().abi(), lparam.into_param().abi(), presult).ok()
    }
}
::windows::imp::interface_hierarchy!(IPrintDialogCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPrintDialogCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDialogCallback {}
impl ::core::fmt::Debug for IPrintDialogCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDialogCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintDialogCallback {
    type Vtable = IPrintDialogCallback_Vtbl;
}
impl ::core::clone::Clone for IPrintDialogCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintDialogCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5852a2c3_6530_11d1_b6a3_0000f8757bf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InitDone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HandleMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hdlg: super::super::super::Foundation::HWND, umsg: u32, wparam: super::super::super::Foundation::WPARAM, lparam: super::super::super::Foundation::LPARAM, presult: *mut super::super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HandleMessage: usize,
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
pub struct IPrintDialogServices(::windows::core::IUnknown);
impl IPrintDialogServices {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetCurrentDevMode(&self, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentDevMode)(::windows::core::Interface::as_raw(self), pdevmode, pcbsize).ok()
    }
    pub unsafe fn GetCurrentPrinterName(&self, pprintername: ::windows::core::PWSTR, pcchsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentPrinterName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pprintername), pcchsize).ok()
    }
    pub unsafe fn GetCurrentPortName(&self, pportname: ::windows::core::PWSTR, pcchsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCurrentPortName)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pportname), pcchsize).ok()
    }
}
::windows::imp::interface_hierarchy!(IPrintDialogServices, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPrintDialogServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPrintDialogServices {}
impl ::core::fmt::Debug for IPrintDialogServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPrintDialogServices").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPrintDialogServices {
    type Vtable = IPrintDialogServices_Vtbl;
}
impl ::core::clone::Clone for IPrintDialogServices {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrintDialogServices {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x509aaeda_5639_11d1_b6a1_0000f8757bf9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDialogServices_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetCurrentDevMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdevmode: *mut super::super::super::Graphics::Gdi::DEVMODEA, pcbsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetCurrentDevMode: usize,
    pub GetCurrentPrinterName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprintername: ::windows::core::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetCurrentPortName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pportname: ::windows::core::PWSTR, pcchsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDM_FIRST: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDM_GETFILEPATH: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDM_GETFOLDERIDLIST: u32 = 1127u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDM_GETFOLDERPATH: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDM_GETSPEC: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDM_HIDECONTROL: u32 = 1129u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDM_LAST: u32 = 1224u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDM_SETCONTROLTEXT: u32 = 1128u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDM_SETDEFEXT: u32 = 1130u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDN_FILEOK: u32 = 4294966690u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDN_FOLDERCHANGE: u32 = 4294966693u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDN_HELP: u32 = 4294966691u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDN_INCLUDEITEM: u32 = 4294966688u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDN_INITDONE: u32 = 4294966695u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDN_SELCHANGE: u32 = 4294966694u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDN_SHAREVIOLATION: u32 = 4294966692u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDN_TYPECHANGE: u32 = 4294966689u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CD_LBSELADD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CD_LBSELCHANGE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CD_LBSELNOITEMS: i32 = -1i32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CD_LBSELSUB: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOROKSTRING: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_ColorOK");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOROKSTRINGA: ::windows::core::PCSTR = ::windows::core::s!("commdlg_ColorOK");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOROKSTRINGW: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_ColorOK");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_ADD: u32 = 712u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_BLUE: u32 = 708u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_BLUEACCEL: u32 = 728u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_BOX1: u32 = 720u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_CURRENT: u32 = 709u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_CUSTOM1: u32 = 721u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_ELEMENT: u32 = 716u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_GREEN: u32 = 707u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_GREENACCEL: u32 = 727u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_HUE: u32 = 703u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_HUEACCEL: u32 = 723u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_HUESCROLL: u32 = 700u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_LUM: u32 = 705u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_LUMACCEL: u32 = 725u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_LUMSCROLL: u32 = 702u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_MIX: u32 = 719u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_PALETTE: u32 = 718u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_RAINBOW: u32 = 710u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_RED: u32 = 706u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_REDACCEL: u32 = 726u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_SAMPLES: u32 = 717u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_SAT: u32 = 704u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_SATACCEL: u32 = 724u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_SATSCROLL: u32 = 701u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_SAVE: u32 = 711u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_SCHEMES: u32 = 715u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_SOLID: u32 = 713u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_SOLID_LEFT: u32 = 730u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_SOLID_RIGHT: u32 = 731u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const COLOR_TUNE: u32 = 714u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const DLG_COLOR: u32 = 10u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const DN_DEFAULTPRN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FILEOKSTRING: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_FileNameOK");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FILEOKSTRINGA: ::windows::core::PCSTR = ::windows::core::s!("commdlg_FileNameOK");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FILEOKSTRINGW: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_FileNameOK");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FINDMSGSTRING: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_FindReplace");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FINDMSGSTRINGA: ::windows::core::PCSTR = ::windows::core::s!("commdlg_FindReplace");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FINDMSGSTRINGW: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_FindReplace");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FRM_FIRST: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FRM_LAST: u32 = 1224u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FRM_SETOPERATIONRESULT: u32 = 1124u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FRM_SETOPERATIONRESULTTEXT: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_NOWRAPAROUND: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_RAW: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_SHOWWRAPAROUND: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_WRAPAROUND: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const HELPMSGSTRING: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_help");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const HELPMSGSTRINGA: ::windows::core::PCSTR = ::windows::core::s!("commdlg_help");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const HELPMSGSTRINGW: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_help");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const LBSELCHSTRING: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_LBSelChangedNotify");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const LBSELCHSTRINGA: ::windows::core::PCSTR = ::windows::core::s!("commdlg_LBSelChangedNotify");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const LBSELCHSTRINGW: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_LBSelChangedNotify");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const NUM_BASIC_COLORS: u32 = 48u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const NUM_CUSTOM_COLORS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_SHAREFALLTHROUGH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_SHARENOWARN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_SHAREWARN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_RESULT_APPLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_RESULT_CANCEL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_RESULT_PRINT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PS_OPENTYPE_FONTTYPE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const SETRGBSTRING: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_SetRGBColor");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const SETRGBSTRINGA: ::windows::core::PCSTR = ::windows::core::s!("commdlg_SetRGBColor");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const SETRGBSTRINGW: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_SetRGBColor");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const SHAREVISTRING: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_ShareViolation");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const SHAREVISTRINGA: ::windows::core::PCSTR = ::windows::core::s!("commdlg_ShareViolation");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const SHAREVISTRINGW: ::windows::core::PCWSTR = ::windows::core::w!("commdlg_ShareViolation");
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const START_PAGE_GENERAL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const SYMBOL_FONTTYPE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const TT_OPENTYPE_FONTTYPE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const TYPE1_FONTTYPE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const WM_CHOOSEFONT_GETLOGFONT: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const WM_CHOOSEFONT_SETFLAGS: u32 = 1126u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const WM_CHOOSEFONT_SETLOGFONT: u32 = 1125u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const WM_PSD_ENVSTAMPRECT: u32 = 1029u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const WM_PSD_FULLPAGERECT: u32 = 1025u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const WM_PSD_GREEKTEXTRECT: u32 = 1028u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const WM_PSD_MARGINRECT: u32 = 1027u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const WM_PSD_MINMARGINRECT: u32 = 1026u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const WM_PSD_YAFULLPAGERECT: u32 = 1030u32;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHOOSECOLOR_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CC_RGBINIT: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CC_FULLOPEN: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CC_PREVENTFULLOPEN: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CC_SHOWHELP: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CC_ENABLEHOOK: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CC_ENABLETEMPLATE: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CC_ENABLETEMPLATEHANDLE: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CC_SOLIDCOLOR: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CC_ANYCOLOR: CHOOSECOLOR_FLAGS = CHOOSECOLOR_FLAGS(256u32);
impl ::core::marker::Copy for CHOOSECOLOR_FLAGS {}
impl ::core::clone::Clone for CHOOSECOLOR_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHOOSECOLOR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CHOOSECOLOR_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CHOOSECOLOR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHOOSECOLOR_FLAGS").field(&self.0).finish()
    }
}
impl CHOOSECOLOR_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CHOOSECOLOR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHOOSECOLOR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHOOSECOLOR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHOOSECOLOR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHOOSECOLOR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHOOSEFONT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_APPLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_ANSIONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_BOTH: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_EFFECTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_ENABLEHOOK: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_ENABLETEMPLATE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_ENABLETEMPLATEHANDLE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_FIXEDPITCHONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_FORCEFONTEXIST: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_INACTIVEFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_INITTOLOGFONTSTRUCT: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_LIMITSIZE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_NOOEMFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_NOFACESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_NOSCRIPTSEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_NOSIMULATIONS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_NOSIZESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_NOSTYLESEL: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_NOVECTORFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_NOVERTFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_PRINTERFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_SCALABLEONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_SCREENFONTS: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_SCRIPTSONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_SELECTSCRIPT: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_SHOWHELP: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_TTONLY: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_USESTYLE: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CF_WYSIWYG: CHOOSEFONT_FLAGS = CHOOSEFONT_FLAGS(32768u32);
impl ::core::marker::Copy for CHOOSEFONT_FLAGS {}
impl ::core::clone::Clone for CHOOSEFONT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHOOSEFONT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CHOOSEFONT_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CHOOSEFONT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHOOSEFONT_FLAGS").field(&self.0).finish()
    }
}
impl CHOOSEFONT_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CHOOSEFONT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHOOSEFONT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHOOSEFONT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHOOSEFONT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHOOSEFONT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CHOOSEFONT_FONT_TYPE(pub u16);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const BOLD_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(256u16);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const ITALIC_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(512u16);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PRINTER_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(16384u16);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const REGULAR_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(1024u16);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const SCREEN_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(8192u16);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const SIMULATED_FONTTYPE: CHOOSEFONT_FONT_TYPE = CHOOSEFONT_FONT_TYPE(32768u16);
impl ::core::marker::Copy for CHOOSEFONT_FONT_TYPE {}
impl ::core::clone::Clone for CHOOSEFONT_FONT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CHOOSEFONT_FONT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CHOOSEFONT_FONT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CHOOSEFONT_FONT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CHOOSEFONT_FONT_TYPE").field(&self.0).finish()
    }
}
impl CHOOSEFONT_FONT_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for CHOOSEFONT_FONT_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CHOOSEFONT_FONT_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CHOOSEFONT_FONT_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CHOOSEFONT_FONT_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CHOOSEFONT_FONT_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct COMMON_DLG_ERRORS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_DIALOGFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(65535u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_GENERALCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_STRUCTSIZE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_INITIALIZATION: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_NOTEMPLATE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(3u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_NOHINSTANCE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_LOADSTRFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(5u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_FINDRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(6u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_LOADRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(7u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_LOCKRESFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_MEMALLOCFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(9u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_MEMLOCKFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(10u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_NOHOOK: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(11u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CDERR_REGISTERMSGFAIL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_PRINTERCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_SETUPFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4097u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_PARSEFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4098u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_RETDEFFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4099u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_LOADDRVFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4100u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_GETDEVMODEFAIL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4101u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_INITFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4102u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_NODEVICES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4103u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_NODEFAULTPRN: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4104u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_DNDMMISMATCH: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4105u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_CREATEICFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4106u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_PRINTERNOTFOUND: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4107u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PDERR_DEFAULTDIFFERENT: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(4108u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CFERR_CHOOSEFONTCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CFERR_NOFONTS: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8193u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CFERR_MAXLESSTHANMIN: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(8194u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FNERR_FILENAMECODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12288u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FNERR_SUBCLASSFAILURE: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12289u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FNERR_INVALIDFILENAME: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12290u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FNERR_BUFFERTOOSMALL: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(12291u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FRERR_FINDREPLACECODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FRERR_BUFFERLENGTHZERO: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(16385u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const CCERR_CHOOSECOLORCODES: COMMON_DLG_ERRORS = COMMON_DLG_ERRORS(20480u32);
impl ::core::marker::Copy for COMMON_DLG_ERRORS {}
impl ::core::clone::Clone for COMMON_DLG_ERRORS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMMON_DLG_ERRORS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for COMMON_DLG_ERRORS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for COMMON_DLG_ERRORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMMON_DLG_ERRORS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FINDREPLACE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_DIALOGTERM: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_DOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_ENABLEHOOK: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_ENABLETEMPLATE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_ENABLETEMPLATEHANDLE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_FINDNEXT: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_HIDEUPDOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_HIDEMATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_HIDEWHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_MATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_NOMATCHCASE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_NOUPDOWN: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_NOWHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_REPLACE: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_REPLACEALL: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_SHOWHELP: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const FR_WHOLEWORD: FINDREPLACE_FLAGS = FINDREPLACE_FLAGS(2u32);
impl ::core::marker::Copy for FINDREPLACE_FLAGS {}
impl ::core::clone::Clone for FINDREPLACE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FINDREPLACE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for FINDREPLACE_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for FINDREPLACE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FINDREPLACE_FLAGS").field(&self.0).finish()
    }
}
impl FINDREPLACE_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for FINDREPLACE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FINDREPLACE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FINDREPLACE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FINDREPLACE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FINDREPLACE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPEN_FILENAME_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_READONLY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_OVERWRITEPROMPT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_HIDEREADONLY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_NOCHANGEDIR: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_SHOWHELP: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_ENABLEHOOK: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_ENABLETEMPLATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_ENABLETEMPLATEHANDLE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_NOVALIDATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_ALLOWMULTISELECT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_EXTENSIONDIFFERENT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_PATHMUSTEXIST: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_FILEMUSTEXIST: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_CREATEPROMPT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_SHAREAWARE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_NOREADONLYRETURN: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_NOTESTFILECREATE: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_NONETWORKBUTTON: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_NOLONGNAMES: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_EXPLORER: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_NODEREFERENCELINKS: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_LONGNAMES: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_ENABLEINCLUDENOTIFY: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_ENABLESIZING: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_DONTADDTORECENT: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_FORCESHOWHIDDEN: OPEN_FILENAME_FLAGS = OPEN_FILENAME_FLAGS(268435456u32);
impl ::core::marker::Copy for OPEN_FILENAME_FLAGS {}
impl ::core::clone::Clone for OPEN_FILENAME_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPEN_FILENAME_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPEN_FILENAME_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPEN_FILENAME_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPEN_FILENAME_FLAGS").field(&self.0).finish()
    }
}
impl OPEN_FILENAME_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for OPEN_FILENAME_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPEN_FILENAME_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPEN_FILENAME_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPEN_FILENAME_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPEN_FILENAME_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct OPEN_FILENAME_FLAGS_EX(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_EX_NONE: OPEN_FILENAME_FLAGS_EX = OPEN_FILENAME_FLAGS_EX(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const OFN_EX_NOPLACESBAR: OPEN_FILENAME_FLAGS_EX = OPEN_FILENAME_FLAGS_EX(1u32);
impl ::core::marker::Copy for OPEN_FILENAME_FLAGS_EX {}
impl ::core::clone::Clone for OPEN_FILENAME_FLAGS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for OPEN_FILENAME_FLAGS_EX {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for OPEN_FILENAME_FLAGS_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for OPEN_FILENAME_FLAGS_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OPEN_FILENAME_FLAGS_EX").field(&self.0).finish()
    }
}
impl OPEN_FILENAME_FLAGS_EX {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for OPEN_FILENAME_FLAGS_EX {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for OPEN_FILENAME_FLAGS_EX {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for OPEN_FILENAME_FLAGS_EX {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for OPEN_FILENAME_FLAGS_EX {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for OPEN_FILENAME_FLAGS_EX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PAGESETUPDLG_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_DEFAULTMINMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_DISABLEMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_DISABLEORIENTATION: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_DISABLEPAGEPAINTING: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_DISABLEPAPER: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_DISABLEPRINTER: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_ENABLEPAGEPAINTHOOK: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_ENABLEPAGESETUPHOOK: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_ENABLEPAGESETUPTEMPLATE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_ENABLEPAGESETUPTEMPLATEHANDLE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_INHUNDREDTHSOFMILLIMETERS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_INTHOUSANDTHSOFINCHES: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_INWININIINTLMEASURE: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_MARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_MINMARGINS: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_NONETWORKBUTTON: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_NOWARNING: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_RETURNDEFAULT: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PSD_SHOWHELP: PAGESETUPDLG_FLAGS = PAGESETUPDLG_FLAGS(2048u32);
impl ::core::marker::Copy for PAGESETUPDLG_FLAGS {}
impl ::core::clone::Clone for PAGESETUPDLG_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PAGESETUPDLG_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PAGESETUPDLG_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PAGESETUPDLG_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PAGESETUPDLG_FLAGS").field(&self.0).finish()
    }
}
impl PAGESETUPDLG_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PAGESETUPDLG_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PAGESETUPDLG_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PAGESETUPDLG_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PAGESETUPDLG_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PAGESETUPDLG_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PRINTDLGEX_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_ALLPAGES: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_COLLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_CURRENTPAGE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_DISABLEPRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_ENABLEPRINTTEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_ENABLEPRINTTEMPLATEHANDLE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_EXCLUSIONFLAGS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_HIDEPRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_NOCURRENTPAGE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8388608u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_NOPAGENUMS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_NOSELECTION: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_NOWARNING: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_PAGENUMS: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_PRINTTOFILE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_RETURNDC: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_RETURNDEFAULT: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_RETURNIC: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_SELECTION: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_USEDEVMODECOPIES: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_USEDEVMODECOPIESANDCOLLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_USELARGETEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_ENABLEPRINTHOOK: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_ENABLESETUPHOOK: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_ENABLESETUPTEMPLATE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_ENABLESETUPTEMPLATEHANDLE: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_NONETWORKBUTTON: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_PRINTSETUP: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
pub const PD_SHOWHELP: PRINTDLGEX_FLAGS = PRINTDLGEX_FLAGS(2048u32);
impl ::core::marker::Copy for PRINTDLGEX_FLAGS {}
impl ::core::clone::Clone for PRINTDLGEX_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PRINTDLGEX_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for PRINTDLGEX_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for PRINTDLGEX_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINTDLGEX_FLAGS").field(&self.0).finish()
    }
}
impl PRINTDLGEX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for PRINTDLGEX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PRINTDLGEX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PRINTDLGEX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PRINTDLGEX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PRINTDLGEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CHOOSECOLORA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CHOOSECOLORA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHOOSECOLORA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CHOOSECOLORW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct CHOOSECOLORW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HWND,
    pub rgbResult: super::super::super::Foundation::COLORREF,
    pub lpCustColors: *mut super::super::super::Foundation::COLORREF,
    pub Flags: CHOOSECOLOR_FLAGS,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCCHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CHOOSECOLORW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CHOOSECOLORW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for CHOOSECOLORW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CHOOSECOLORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpszStyle: ::windows::core::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for CHOOSEFONTA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTA,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpszStyle: ::windows::core::PSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for CHOOSEFONTA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHOOSEFONTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpszStyle: ::windows::core::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for CHOOSEFONTW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct CHOOSEFONTW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub lpLogFont: *mut super::super::super::Graphics::Gdi::LOGFONTW,
    pub iPointSize: i32,
    pub Flags: CHOOSEFONT_FLAGS,
    pub rgbColors: super::super::super::Foundation::COLORREF,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPCFHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpszStyle: ::windows::core::PWSTR,
    pub nFontType: CHOOSEFONT_FONT_TYPE,
    pub ___MISSING_ALIGNMENT__: u16,
    pub nSizeMin: i32,
    pub nSizeMax: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for CHOOSEFONTW {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for CHOOSEFONTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for CHOOSEFONTW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for CHOOSEFONTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for DEVNAMES {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for DEVNAMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for DEVNAMES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for DEVNAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(target_arch = "x86")]
pub struct DEVNAMES {
    pub wDriverOffset: u16,
    pub wDeviceOffset: u16,
    pub wOutputOffset: u16,
    pub wDefault: u16,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for DEVNAMES {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for DEVNAMES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for DEVNAMES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for DEVNAMES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: ::windows::core::PSTR,
    pub lpstrReplaceWith: ::windows::core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FINDREPLACEA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDREPLACEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: ::windows::core::PSTR,
    pub lpstrReplaceWith: ::windows::core::PSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FINDREPLACEA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDREPLACEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: ::windows::core::PWSTR,
    pub lpstrReplaceWith: ::windows::core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FINDREPLACEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDREPLACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct FINDREPLACEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub Flags: FINDREPLACE_FLAGS,
    pub lpstrFindWhat: ::windows::core::PWSTR,
    pub lpstrReplaceWith: ::windows::core::PWSTR,
    pub wFindWhatLen: u16,
    pub wReplaceWithLen: u16,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPFRHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for FINDREPLACEW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for FINDREPLACEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for FINDREPLACEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FINDREPLACEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: ::windows::core::PSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OFNOTIFYA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub pszFile: ::windows::core::PSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OFNOTIFYA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OFNOTIFYEXA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXA {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEA,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OFNOTIFYEXA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OFNOTIFYEXW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYEXW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub psf: *mut ::core::ffi::c_void,
    pub pidl: *mut ::core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYEXW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OFNOTIFYEXW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: ::windows::core::PWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OFNOTIFYW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OFNOTIFYW {
    pub hdr: super::NMHDR,
    pub lpOFN: *mut OPENFILENAMEW,
    pub pszFile: ::windows::core::PWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OFNOTIFYW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OFNOTIFYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OFNOTIFYW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OFNOTIFYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpstrFilter: ::windows::core::PCSTR,
    pub lpstrCustomFilter: ::windows::core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: ::windows::core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: ::windows::core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: ::windows::core::PCSTR,
    pub lpstrTitle: ::windows::core::PCSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: ::windows::core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENFILENAMEA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpstrFilter: ::windows::core::PCSTR,
    pub lpstrCustomFilter: ::windows::core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: ::windows::core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: ::windows::core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: ::windows::core::PCSTR,
    pub lpstrTitle: ::windows::core::PCSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: ::windows::core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENFILENAMEA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAMEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpstrFilter: ::windows::core::PCWSTR,
    pub lpstrCustomFilter: ::windows::core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: ::windows::core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: ::windows::core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: ::windows::core::PCWSTR,
    pub lpstrTitle: ::windows::core::PCWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: ::windows::core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENFILENAMEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAMEW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpstrFilter: ::windows::core::PCWSTR,
    pub lpstrCustomFilter: ::windows::core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: ::windows::core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: ::windows::core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: ::windows::core::PCWSTR,
    pub lpstrTitle: ::windows::core::PCWSTR,
    pub Flags: OPEN_FILENAME_FLAGS,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: ::windows::core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dwReserved: u32,
    pub FlagsEx: OPEN_FILENAME_FLAGS_EX,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAMEW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENFILENAMEW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAMEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpstrFilter: ::windows::core::PCSTR,
    pub lpstrCustomFilter: ::windows::core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: ::windows::core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: ::windows::core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: ::windows::core::PCSTR,
    pub lpstrTitle: ::windows::core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: ::windows::core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4A {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENFILENAME_NT4A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4A {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpstrFilter: ::windows::core::PCSTR,
    pub lpstrCustomFilter: ::windows::core::PSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: ::windows::core::PSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: ::windows::core::PSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: ::windows::core::PCSTR,
    pub lpstrTitle: ::windows::core::PCSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: ::windows::core::PCSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::windows::core::PCSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4A {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENFILENAME_NT4A {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAME_NT4A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpstrFilter: ::windows::core::PCWSTR,
    pub lpstrCustomFilter: ::windows::core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: ::windows::core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: ::windows::core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: ::windows::core::PCWSTR,
    pub lpstrTitle: ::windows::core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: ::windows::core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4W {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENFILENAME_NT4W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct OPENFILENAME_NT4W {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpstrFilter: ::windows::core::PCWSTR,
    pub lpstrCustomFilter: ::windows::core::PWSTR,
    pub nMaxCustFilter: u32,
    pub nFilterIndex: u32,
    pub lpstrFile: ::windows::core::PWSTR,
    pub nMaxFile: u32,
    pub lpstrFileTitle: ::windows::core::PWSTR,
    pub nMaxFileTitle: u32,
    pub lpstrInitialDir: ::windows::core::PCWSTR,
    pub lpstrTitle: ::windows::core::PCWSTR,
    pub Flags: u32,
    pub nFileOffset: u16,
    pub nFileExtension: u16,
    pub lpstrDefExt: ::windows::core::PCWSTR,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnHook: LPOFNHOOKPROC,
    pub lpTemplateName: ::windows::core::PCWSTR,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for OPENFILENAME_NT4W {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for OPENFILENAME_NT4W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for OPENFILENAME_NT4W {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for OPENFILENAME_NT4W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: ::windows::core::PCSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PAGESETUPDLGA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESETUPDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: ::windows::core::PCSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGA {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PAGESETUPDLGA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESETUPDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: ::windows::core::PCWSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PAGESETUPDLGW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESETUPDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
pub struct PAGESETUPDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub Flags: PAGESETUPDLG_FLAGS,
    pub ptPaperSize: super::super::super::Foundation::POINT,
    pub rtMinMargin: super::super::super::Foundation::RECT,
    pub rtMargin: super::super::super::Foundation::RECT,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPageSetupHook: LPPAGESETUPHOOK,
    pub lpfnPagePaintHook: LPPAGEPAINTHOOK,
    pub lpPageSetupTemplateName: ::windows::core::PCWSTR,
    pub hPageSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PAGESETUPDLGW {}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PAGESETUPDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for PAGESETUPDLGW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PAGESETUPDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: ::windows::core::PCSTR,
    pub lpSetupTemplateName: ::windows::core::PCSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGA {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for PRINTDLGA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: ::windows::core::PCSTR,
    pub lpSetupTemplateName: ::windows::core::PCSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGA {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for PRINTDLGA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpPrintTemplateName: ::windows::core::PCSTR,
    pub lpCallback: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGEXA {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for PRINTDLGEXA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXA {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpPrintTemplateName: ::windows::core::PCSTR,
    pub lpCallback: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for PRINTDLGEXA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpPrintTemplateName: ::windows::core::PCWSTR,
    pub lpCallback: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGEXW {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for PRINTDLGEXW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGEXW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub Flags2: u32,
    pub ExclusionFlags: u32,
    pub nPageRanges: u32,
    pub nMaxPageRanges: u32,
    pub lpPageRanges: *mut PRINTPAGERANGE,
    pub nMinPage: u32,
    pub nMaxPage: u32,
    pub nCopies: u32,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lpPrintTemplateName: ::windows::core::PCWSTR,
    pub lpCallback: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
    pub nPropertyPages: u32,
    pub lphPropertyPages: *mut super::HPROPSHEETPAGE,
    pub nStartPage: u32,
    pub dwResultAction: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for PRINTDLGEXW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: ::windows::core::PCWSTR,
    pub lpSetupTemplateName: ::windows::core::PCWSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGW {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for PRINTDLGW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
pub struct PRINTDLGW {
    pub lStructSize: u32,
    pub hwndOwner: super::super::super::Foundation::HWND,
    pub hDevMode: super::super::super::Foundation::HGLOBAL,
    pub hDevNames: super::super::super::Foundation::HGLOBAL,
    pub hDC: super::super::super::Graphics::Gdi::HDC,
    pub Flags: PRINTDLGEX_FLAGS,
    pub nFromPage: u16,
    pub nToPage: u16,
    pub nMinPage: u16,
    pub nMaxPage: u16,
    pub nCopies: u16,
    pub hInstance: super::super::super::Foundation::HMODULE,
    pub lCustData: super::super::super::Foundation::LPARAM,
    pub lpfnPrintHook: LPPRINTHOOKPROC,
    pub lpfnSetupHook: LPSETUPHOOKPROC,
    pub lpPrintTemplateName: ::windows::core::PCWSTR,
    pub lpSetupTemplateName: ::windows::core::PCWSTR,
    pub hPrintTemplate: super::super::super::Foundation::HGLOBAL,
    pub hSetupTemplate: super::super::super::Foundation::HGLOBAL,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::marker::Copy for PRINTDLGW {}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::clone::Clone for PRINTDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::TypeKind for PRINTDLGW {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for PRINTDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for PRINTPAGERANGE {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for PRINTPAGERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::windows::core::TypeKind for PRINTPAGERANGE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::default::Default for PRINTPAGERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`*"]
#[cfg(target_arch = "x86")]
pub struct PRINTPAGERANGE {
    pub nFromPage: u32,
    pub nToPage: u32,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for PRINTPAGERANGE {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for PRINTPAGERANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(target_arch = "x86")]
impl ::windows::core::TypeKind for PRINTPAGERANGE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(target_arch = "x86")]
impl ::core::default::Default for PRINTPAGERANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPCCHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPCFHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPFRHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPOFNHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPPAGEPAINTHOOK = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPPAGESETUPHOOK = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPPRINTHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[doc = "*Required features: `\"Win32_UI_Controls_Dialogs\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPSETUPHOOKPROC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::super::Foundation::HWND, param1: u32, param2: super::super::super::Foundation::WPARAM, param3: super::super::super::Foundation::LPARAM) -> usize>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
