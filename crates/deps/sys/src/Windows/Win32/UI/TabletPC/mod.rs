#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn AddStroke(hrc: HRECOCONTEXT, ppacketdesc: *const PACKET_DESCRIPTION, cbpacket: u32, ppacket: *const u8, pxform: *const super::super::Graphics::Gdi::XFORM) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddWordsToWordList(hwl: HRECOWORDLIST, pwcwords: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdviseInkChange(hrc: HRECOCONTEXT, bnewstroke: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn CreateContext(hrec: HRECOGNIZER, phrc: *mut HRECOCONTEXT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn CreateRecognizer(pclsid: *mut ::windows::runtime::GUID, phrec: *mut HRECOGNIZER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn DestroyContext(hrc: HRECOCONTEXT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn DestroyRecognizer(hrec: HRECOGNIZER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn DestroyWordList(hwl: HRECOWORDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn EndInkInput(hrc: HRECOCONTEXT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetAllRecognizers(recognizerclsids: *mut *mut ::windows::runtime::GUID, count: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBestResultString(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcbestresult: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetLatticePtr(hrc: HRECOCONTEXT, pplattice: *mut *mut RECO_LATTICE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLeftSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcleftseparator: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetRecoAttributes(hrec: HRECOGNIZER, precoattrs: *mut RECO_ATTRS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetResultPropertyList(hrec: HRECOGNIZER, ppropertycount: *mut u32, ppropertyguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRightSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcrightseparator: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetUnicodeRanges(hrec: HRECOGNIZER, pcranges: *mut u32, pcr: *mut CHARACTER_RANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsStringSupported(hrc: HRECOCONTEXT, wcstring: u32, pwcstring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn LoadCachedAttributes(clsid: ::windows::runtime::GUID, precoattributes: *mut RECO_ATTRS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeWordList(hrec: HRECOGNIZER, pbuffer: super::super::Foundation::PWSTR, phwl: *mut HRECOWORDLIST) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process(hrc: HRECOCONTEXT, pbpartialprocessing: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetEnabledUnicodeRanges(hrc: HRECOCONTEXT, cranges: u32, pcr: *mut CHARACTER_RANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFactoid(hrc: HRECOCONTEXT, cwcfactoid: u32, pwcfactoid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetFlags(hrc: HRECOCONTEXT, dwflags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetGuide(hrc: HRECOCONTEXT, pguide: *const RECO_GUIDE, iindex: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTextContext(hrc: HRECOCONTEXT, cwcbefore: u32, pwcbefore: super::super::Foundation::PWSTR, cwcafter: u32, pwcafter: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetWordList(hrc: HRECOCONTEXT, hwl: HRECOWORDLIST) -> ::windows::runtime::HRESULT;
}
