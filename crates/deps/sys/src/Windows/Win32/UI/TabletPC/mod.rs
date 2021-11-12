#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn AddStroke(hrc: HRECOCONTEXT, ppacketdesc: *const PACKET_DESCRIPTION, cbpacket: u32, ppacket: *const u8, pxform: *const super::super::Graphics::Gdi::XFORM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddWordsToWordList(hwl: HRECOWORDLIST, pwcwords: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdviseInkChange(hrc: HRECOCONTEXT, bnewstroke: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn CreateContext(hrec: HRECOGNIZER, phrc: *mut HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn CreateRecognizer(pclsid: *mut ::windows_sys::core::GUID, phrec: *mut HRECOGNIZER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn DestroyContext(hrc: HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn DestroyRecognizer(hrec: HRECOGNIZER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn DestroyWordList(hwl: HRECOWORDLIST) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn EndInkInput(hrc: HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetAllRecognizers(recognizerclsids: *mut *mut ::windows_sys::core::GUID, count: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBestResultString(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcbestresult: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetLatticePtr(hrc: HRECOCONTEXT, pplattice: *mut *mut RECO_LATTICE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLeftSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcleftseparator: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetRecoAttributes(hrec: HRECOGNIZER, precoattrs: *mut RECO_ATTRS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetResultPropertyList(hrec: HRECOGNIZER, ppropertycount: *mut u32, ppropertyguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRightSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcrightseparator: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn GetUnicodeRanges(hrec: HRECOGNIZER, pcranges: *mut u32, pcr: *mut CHARACTER_RANGE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsStringSupported(hrc: HRECOCONTEXT, wcstring: u32, pwcstring: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn LoadCachedAttributes(clsid: ::windows_sys::core::GUID, precoattributes: *mut RECO_ATTRS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeWordList(hrec: HRECOGNIZER, pbuffer: super::super::Foundation::PWSTR, phwl: *mut HRECOWORDLIST) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process(hrc: HRECOCONTEXT, pbpartialprocessing: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetEnabledUnicodeRanges(hrc: HRECOCONTEXT, cranges: u32, pcr: *mut CHARACTER_RANGE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFactoid(hrc: HRECOCONTEXT, cwcfactoid: u32, pwcfactoid: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetFlags(hrc: HRECOCONTEXT, dwflags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetGuide(hrc: HRECOCONTEXT, pguide: *const RECO_GUIDE, iindex: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTextContext(hrc: HRECOCONTEXT, cwcbefore: u32, pwcbefore: super::super::Foundation::PWSTR, cwcafter: u32, pwcafter: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub fn SetWordList(hrc: HRECOCONTEXT, hwl: HRECOWORDLIST) -> ::windows_sys::core::HRESULT;
}
pub struct ALT_BREAKS(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_ADDSTROKE_FAILED: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_INTERRUPTED: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_PROCESS_FAILED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_RESETCONTEXT_FAILED: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_SETCACMODE_FAILED: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_SETFACTOID_FAILED: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_SETFLAGS_FAILED: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_SETGUIDE_FAILED: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_SETTEXTCONTEXT_FAILED: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const ASYNC_RECO_SETWORDLIST_FAILED: u32 = 512u32;
pub struct AppearanceConstants(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const BEST_COMPLETE: u32 = 2u32;
pub struct BorderStyleConstants(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const CAC_FULL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const CAC_PREFIX: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const CAC_RANDOM: u32 = 2u32;
pub struct CHARACTER_RANGE(i32);
pub struct CONFIDENCE_LEVEL(i32);
pub struct CorrectionMode(i32);
pub struct CorrectionPosition(i32);
pub struct DISPID_Ink(i32);
pub struct DISPID_InkCollector(i32);
pub struct DISPID_InkCollectorEvent(i32);
pub struct DISPID_InkCursor(i32);
pub struct DISPID_InkCursorButton(i32);
pub struct DISPID_InkCursorButtons(i32);
pub struct DISPID_InkCursors(i32);
pub struct DISPID_InkCustomStrokes(i32);
pub struct DISPID_InkDivider(i32);
pub struct DISPID_InkDivisionResult(i32);
pub struct DISPID_InkDivisionUnit(i32);
pub struct DISPID_InkDivisionUnits(i32);
pub struct DISPID_InkDrawingAttributes(i32);
pub struct DISPID_InkEdit(i32);
pub struct DISPID_InkEditEvents(i32);
pub struct DISPID_InkEvent(i32);
pub struct DISPID_InkExtendedProperties(i32);
pub struct DISPID_InkExtendedProperty(i32);
pub struct DISPID_InkGesture(i32);
pub struct DISPID_InkRecoAlternate(i32);
pub struct DISPID_InkRecoContext(i32);
pub struct DISPID_InkRecoContext2(i32);
pub struct DISPID_InkRecognitionAlternates(i32);
pub struct DISPID_InkRecognitionEvent(i32);
pub struct DISPID_InkRecognitionResult(i32);
pub struct DISPID_InkRecognizer(i32);
pub struct DISPID_InkRecognizer2(i32);
pub struct DISPID_InkRecognizerGuide(i32);
pub struct DISPID_InkRecognizers(i32);
pub struct DISPID_InkRectangle(i32);
pub struct DISPID_InkRenderer(i32);
pub struct DISPID_InkStrokeDisp(i32);
pub struct DISPID_InkStrokes(i32);
pub struct DISPID_InkTablet(i32);
pub struct DISPID_InkTablet2(i32);
pub struct DISPID_InkTablet3(i32);
pub struct DISPID_InkTablets(i32);
pub struct DISPID_InkTransform(i32);
pub struct DISPID_InkWordList(i32);
pub struct DISPID_InkWordList2(i32);
pub struct DISPID_MathInputControlEvents(i32);
pub struct DISPID_PenInputPanel(i32);
pub struct DISPID_PenInputPanelEvents(i32);
pub struct DISPID_StrokeEvent(i32);
pub struct DYNAMIC_RENDERER_CACHED_DATA(i32);
pub struct DynamicRenderer(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETDRAWATTR: u32 = 1541u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETFACTOID: u32 = 1549u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETGESTURESTATUS: u32 = 1545u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETINKINSERTMODE: u32 = 1539u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETINKMODE: u32 = 1537u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETMOUSEICON: u32 = 1553u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETMOUSEPOINTER: u32 = 1555u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETRECOGNIZER: u32 = 1547u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETRECOTIMEOUT: u32 = 1543u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETSELINK: u32 = 1551u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETSELINKDISPLAYMODE: u32 = 1562u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETSTATUS: u32 = 1557u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_GETUSEMOUSEFORINPUT: u32 = 1559u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_RECOGNIZE: u32 = 1558u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETDRAWATTR: u32 = 1542u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETFACTOID: u32 = 1550u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETGESTURESTATUS: u32 = 1546u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETINKINSERTMODE: u32 = 1540u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETINKMODE: u32 = 1538u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETMOUSEICON: u32 = 1554u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETMOUSEPOINTER: u32 = 1556u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETRECOGNIZER: u32 = 1548u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETRECOTIMEOUT: u32 = 1544u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETSELINK: u32 = 1552u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETSELINKDISPLAYMODE: u32 = 1561u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const EM_SETUSEMOUSEFORINPUT: u32 = 1560u32;
pub struct EventMask(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const FACILITY_INK: u32 = 40u32;
pub struct FLICKACTION_COMMANDCODE(i32);
pub struct FLICKDIRECTION(i32);
pub struct FLICKMODE(i32);
pub struct FLICK_DATA(i32);
pub struct FLICK_POINT(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const FLICK_WM_HANDLED_MASK: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_ARROW_DOWN: u32 = 61497u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_ARROW_LEFT: u32 = 61498u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_ARROW_RIGHT: u32 = 61499u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_ARROW_UP: u32 = 61496u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_ASTERISK: u32 = 61608u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BRACE_LEFT: u32 = 61674u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BRACE_OVER: u32 = 61672u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BRACE_RIGHT: u32 = 61675u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BRACE_UNDER: u32 = 61673u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BRACKET_LEFT: u32 = 61670u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BRACKET_OVER: u32 = 61668u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BRACKET_RIGHT: u32 = 61671u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BRACKET_UNDER: u32 = 61669u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BULLET: u32 = 61450u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_BULLET_CROSS: u32 = 61451u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CHECK: u32 = 61445u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CHEVRON_DOWN: u32 = 61489u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CHEVRON_LEFT: u32 = 61490u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CHEVRON_RIGHT: u32 = 61491u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CHEVRON_UP: u32 = 61488u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CIRCLE: u32 = 61472u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CIRCLE_CIRCLE: u32 = 61475u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CIRCLE_CROSS: u32 = 61477u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CIRCLE_LINE_HORZ: u32 = 61479u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CIRCLE_LINE_VERT: u32 = 61478u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CIRCLE_TAP: u32 = 61474u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CLOSEUP: u32 = 61455u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CROSS: u32 = 61447u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_CURLICUE: u32 = 61456u32;
pub struct GESTURE_DATA(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIAGONAL_LEFTDOWN: u32 = 61534u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIAGONAL_LEFTUP: u32 = 61532u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIAGONAL_RIGHTDOWN: u32 = 61535u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIAGONAL_RIGHTUP: u32 = 61533u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_0: u32 = 61594u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_1: u32 = 61595u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_2: u32 = 61596u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_3: u32 = 61597u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_4: u32 = 61598u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_5: u32 = 61599u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_6: u32 = 61600u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_7: u32 = 61601u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_8: u32 = 61602u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DIGIT_9: u32 = 61603u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOLLAR: u32 = 61607u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_ARROW_DOWN: u32 = 61501u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_ARROW_LEFT: u32 = 61502u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_ARROW_RIGHT: u32 = 61503u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_ARROW_UP: u32 = 61500u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_CIRCLE: u32 = 61473u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_CURLICUE: u32 = 61457u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_DOWN: u32 = 61625u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_LEFT: u32 = 61626u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_RIGHT: u32 = 61627u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_TAP: u32 = 61681u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOUBLE_UP: u32 = 61624u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOWN: u32 = 61529u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOWN_ARROW_LEFT: u32 = 61506u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOWN_ARROW_RIGHT: u32 = 61507u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOWN_LEFT: u32 = 61546u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOWN_LEFT_LONG: u32 = 61542u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOWN_RIGHT: u32 = 61547u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOWN_RIGHT_LONG: u32 = 61543u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_DOWN_UP: u32 = 61537u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_EXCLAMATION: u32 = 61604u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_INFINITY: u32 = 61446u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LEFT: u32 = 61530u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LEFT_ARROW_DOWN: u32 = 61509u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LEFT_ARROW_UP: u32 = 61508u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LEFT_DOWN: u32 = 61549u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LEFT_RIGHT: u32 = 61538u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LEFT_UP: u32 = 61548u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_A: u32 = 61568u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_B: u32 = 61569u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_C: u32 = 61570u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_D: u32 = 61571u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_E: u32 = 61572u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_F: u32 = 61573u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_G: u32 = 61574u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_H: u32 = 61575u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_I: u32 = 61576u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_J: u32 = 61577u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_K: u32 = 61578u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_L: u32 = 61579u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_M: u32 = 61580u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_N: u32 = 61581u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_O: u32 = 61582u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_P: u32 = 61583u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_Q: u32 = 61584u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_R: u32 = 61585u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_S: u32 = 61586u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_T: u32 = 61587u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_U: u32 = 61588u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_V: u32 = 61589u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_W: u32 = 61590u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_X: u32 = 61591u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_Y: u32 = 61592u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_LETTER_Z: u32 = 61593u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_NULL: u32 = 61440u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_OPENUP: u32 = 61454u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_PARAGRAPH: u32 = 61448u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_PLUS: u32 = 61609u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_QUAD_TAP: u32 = 61683u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_QUESTION: u32 = 61605u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_RECTANGLE: u32 = 61458u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_RIGHT: u32 = 61531u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_RIGHT_ARROW_DOWN: u32 = 61511u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_RIGHT_ARROW_UP: u32 = 61510u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_RIGHT_DOWN: u32 = 61551u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_RIGHT_LEFT: u32 = 61539u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_RIGHT_UP: u32 = 61550u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_SCRATCHOUT: u32 = 61441u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_SECTION: u32 = 61449u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_SEMICIRCLE_LEFT: u32 = 61480u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_SEMICIRCLE_RIGHT: u32 = 61481u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_SHARP: u32 = 61606u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_SQUARE: u32 = 61443u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_SQUIGGLE: u32 = 61452u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_STAR: u32 = 61444u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_SWAP: u32 = 61453u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_TAP: u32 = 61680u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_TRIANGLE: u32 = 61442u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_TRIPLE_DOWN: u32 = 61629u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_TRIPLE_LEFT: u32 = 61630u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_TRIPLE_RIGHT: u32 = 61631u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_TRIPLE_TAP: u32 = 61682u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_TRIPLE_UP: u32 = 61628u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_UP: u32 = 61528u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_UP_ARROW_LEFT: u32 = 61504u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_UP_ARROW_RIGHT: u32 = 61505u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_UP_DOWN: u32 = 61536u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_UP_LEFT: u32 = 61544u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_UP_LEFT_LONG: u32 = 61540u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_UP_RIGHT: u32 = 61545u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const GESTURE_UP_RIGHT_LONG: u32 = 61541u32;
pub const GUID_DYNAMIC_RENDERER_CACHED_DATA: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3209894802, data2: 9663, data3: 19093, data4: [137, 173, 14, 71, 107, 52, 180, 245] };
pub const GUID_GESTURE_DATA: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1105521679,
    data2: 9898,
    data3: 17754,
    data4: [154, 165, 44, 211, 108, 246, 63, 185],
};
pub const GUID_PACKETPROPERTY_GUID_ALTITUDE_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2195637703,
    data2: 63162,
    data3: 18694,
    data4: [137, 79, 102, 214, 141, 252, 69, 108],
};
pub const GUID_PACKETPROPERTY_GUID_AZIMUTH_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 43066292, data2: 34856, data3: 16651, data4: [178, 80, 160, 83, 101, 149, 229, 220] };
pub const GUID_PACKETPROPERTY_GUID_BUTTON_PRESSURE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2340417476,
    data2: 38570,
    data3: 19454,
    data4: [172, 38, 138, 95, 11, 224, 123, 245],
};
pub const GUID_PACKETPROPERTY_GUID_DEVICE_CONTACT_ID: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 39345041, data2: 1179, data3: 18256, data4: [150, 21, 223, 137, 72, 171, 60, 156] };
pub const GUID_PACKETPROPERTY_GUID_FINGERCONTACTCONFIDENCE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3875981316,
    data2: 22512,
    data3: 20224,
    data4: [138, 12, 133, 61, 87, 120, 155, 233],
};
pub const GUID_PACKETPROPERTY_GUID_HEIGHT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3860355282, data2: 58439, data3: 16920, data4: [157, 63, 24, 134, 92, 32, 61, 244] };
pub const GUID_PACKETPROPERTY_GUID_NORMAL_PRESSURE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1929859117,
    data2: 63988,
    data3: 19992,
    data4: [179, 242, 44, 225, 177, 163, 97, 12],
};
pub const GUID_PACKETPROPERTY_GUID_PACKET_STATUS: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1846413247,
    data2: 45031,
    data3: 19703,
    data4: [135, 209, 175, 100, 70, 32, 132, 24],
};
pub const GUID_PACKETPROPERTY_GUID_PITCH_ROTATION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2138986423, data2: 48695, data3: 19425, data4: [163, 86, 122, 132, 22, 14, 24, 147] };
pub const GUID_PACKETPROPERTY_GUID_ROLL_ROTATION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1566400086,
    data2: 27561,
    data3: 19547,
    data4: [159, 176, 133, 28, 145, 113, 78, 86],
};
pub const GUID_PACKETPROPERTY_GUID_SERIAL_NUMBER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2024282966, data2: 2357, data3: 17555, data4: [186, 174, 0, 84, 26, 138, 22, 196] };
pub const GUID_PACKETPROPERTY_GUID_TANGENT_PRESSURE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1839483019, data2: 21060, data3: 16876, data4: [144, 91, 50, 216, 154, 184, 8, 9] };
pub const GUID_PACKETPROPERTY_GUID_TIMER_TICK: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1130696901,
    data2: 65235,
    data3: 17873,
    data4: [139, 118, 113, 211, 234, 122, 130, 157],
};
pub const GUID_PACKETPROPERTY_GUID_TWIST_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 221399392, data2: 5042, data3: 16868, data4: [172, 230, 122, 233, 212, 61, 45, 59] };
pub const GUID_PACKETPROPERTY_GUID_WIDTH: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3131828557,
    data2: 10002,
    data3: 18677,
    data4: [190, 157, 143, 139, 94, 160, 113, 26],
};
pub const GUID_PACKETPROPERTY_GUID_X: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1502243471,
    data2: 21184,
    data3: 19360,
    data4: [147, 175, 175, 53, 116, 17, 165, 97],
};
pub const GUID_PACKETPROPERTY_GUID_X_TILT_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2832235322,
    data2: 35824,
    data3: 16560,
    data4: [149, 169, 184, 10, 107, 183, 135, 191],
};
pub const GUID_PACKETPROPERTY_GUID_Y: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3040845685, data2: 1248, data3: 17560, data4: [167, 238, 195, 13, 187, 90, 144, 17] };
pub const GUID_PACKETPROPERTY_GUID_YAW_ROTATION: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1787074944,
    data2: 31802,
    data3: 17847,
    data4: [170, 130, 144, 162, 98, 149, 14, 137],
};
pub const GUID_PACKETPROPERTY_GUID_Y_TILT_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 244523913, data2: 7543, data3: 17327, data4: [172, 0, 91, 149, 13, 109, 75, 45] };
pub const GUID_PACKETPROPERTY_GUID_Z: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1935334192, data2: 3771, data3: 18312, data4: [160, 228, 15, 49, 100, 144, 5, 93] };
pub struct GestureRecognizer(i32);
pub struct HRECOALT(i32);
pub struct HRECOCONTEXT(i32);
pub struct HRECOGNIZER(i32);
pub struct HRECOLATTICE(i32);
pub struct HRECOWORDLIST(i32);
pub struct HandwrittenTextInsertion(i32);
#[repr(transparent)]
pub struct IDynamicRenderer(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IECN_GESTURE: u32 = 2050u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IECN_RECOGNITIONRESULT: u32 = 2051u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IECN_STROKE: u32 = 2049u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IECN__BASE: u32 = 2048u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Controls"))]
pub struct IEC_GESTUREINFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct IEC_RECOGNITIONRESULTINFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct IEC_STROKEINFO(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IEC__BASE: u32 = 1536u32;
#[repr(transparent)]
pub struct IGestureRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHandwrittenTextInsertion(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInk(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkCursor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkCursorButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkCursorButtons(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkCursors(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkCustomStrokes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDisp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDivider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDivisionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDivisionUnit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDivisionUnits(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkDrawingAttributes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkEdit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkExtendedProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkExtendedProperty(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkGesture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkLineInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkOverlay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkPicture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognitionAlternate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognitionAlternates(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognitionResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognizer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognizer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognizerContext(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognizerContext2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognizerGuide(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRecognizers(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRectangle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkRenderer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokeDisp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkStrokes(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkTablet(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkTablet2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkTablet3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkTablets(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkWordList(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkWordList2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInputPanelWindowHandle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMathInputControl(pub *mut ::core::ffi::c_void);
pub struct INKMETRIC(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IP_CURSOR_DOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IP_INVERTED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IP_MARGIN: u32 = 4u32;
#[repr(transparent)]
pub struct IPenInputPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRealTimeStylus(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRealTimeStylus2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRealTimeStylus3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRealTimeStylusSynchronization(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISketchInk(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStrokeBuilder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStylusAsyncPlugin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStylusPlugin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStylusSyncPlugin(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextInputPanel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextInputPanelEventSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITextInputPanelRunInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITipAutoCompleteClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITipAutoCompleteProvider(pub *mut ::core::ffi::c_void);
pub struct InPlaceDirection(i32);
pub struct InPlaceState(i32);
pub struct Ink(i32);
pub struct InkApplicationGesture(i32);
pub struct InkBoundingBoxMode(i32);
pub struct InkClipboardFormats(i32);
pub struct InkClipboardModes(i32);
pub struct InkCollectionMode(i32);
pub struct InkCollector(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const InkCollectorClipInkToMargin: i32 = 0i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const InkCollectorDefaultMargin: i32 = -2147483648i32;
pub struct InkCollectorEventInterest(i32);
pub struct InkCursorButtonState(i32);
pub struct InkDisp(i32);
pub struct InkDisplayMode(i32);
pub struct InkDivider(i32);
pub struct InkDivisionType(i32);
pub struct InkDrawingAttributes(i32);
pub struct InkEdit(i32);
pub struct InkEditStatus(i32);
pub struct InkExtractFlags(i32);
pub struct InkInsertMode(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const InkMaxTransparencyValue: i32 = 255i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const InkMinTransparencyValue: i32 = 0i32;
pub struct InkMode(i32);
pub struct InkMouseButton(i32);
pub struct InkMousePointer(i32);
pub struct InkOverlay(i32);
pub struct InkOverlayAttachMode(i32);
pub struct InkOverlayEditingMode(i32);
pub struct InkOverlayEraserMode(i32);
pub struct InkPenTip(i32);
pub struct InkPersistenceCompressionMode(i32);
pub struct InkPersistenceFormat(i32);
pub struct InkPicture(i32);
pub struct InkPictureSizeMode(i32);
pub struct InkRasterOperation(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct InkRecoGuide(i32);
pub struct InkRecognitionAlternatesSelection(i32);
pub struct InkRecognitionConfidence(i32);
pub struct InkRecognitionModes(i32);
pub struct InkRecognitionStatus(i32);
pub struct InkRecognizerCapabilities(i32);
pub struct InkRecognizerCharacterAutoCompletionMode(i32);
pub struct InkRecognizerContext(i32);
pub struct InkRecognizerGuide(i32);
pub struct InkRecognizers(i32);
pub struct InkRectangle(i32);
pub struct InkRenderer(i32);
pub struct InkSelectionConstants(i32);
pub struct InkShiftKeyModifierFlags(i32);
pub struct InkStrokes(i32);
pub struct InkSystemGesture(i32);
pub struct InkTablets(i32);
pub struct InkTransform(i32);
pub struct InkWordList(i32);
pub struct InteractionMode(i32);
pub struct KEYMODIFIER(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct LATTICE_METRICS(i32);
pub struct LINE_METRICS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct LINE_SEGMENT(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const MAX_FRIENDLYNAME: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const MAX_LANGUAGES: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const MAX_PACKET_BUTTON_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const MAX_PACKET_PROPERTY_COUNT: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const MAX_VENDORNAME: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const MICROSOFT_TIP_COMBOBOXLIST_PROPERTY: &'static str = "Microsoft TIP ComboBox List Window Identifier";
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const MICROSOFT_TIP_NO_INSERT_BUTTON_PROPERTY: &'static str = "Microsoft TIP No Insert Option";
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const MICROSOFT_TIP_OPENING_MSG: &'static str = "TabletInputPanelOpening";
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const MICROSOFT_URL_EXPERIENCE_PROPERTY: &'static str = "Microsoft TIP URL Experience";
pub struct MICUIELEMENT(i32);
pub struct MICUIELEMENTSTATE(i32);
pub struct MathInputControl(i32);
pub struct MouseButton(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const NUM_FLICK_DIRECTIONS: u32 = 8u32;
pub struct PACKET_DESCRIPTION(i32);
pub struct PACKET_PROPERTY(i32);
pub struct PROPERTY_METRICS(i32);
pub struct PROPERTY_UNITS(i32);
pub struct PanelInputArea(i32);
pub struct PanelType(i32);
pub struct PenInputPanel(i32);
pub struct PenInputPanel_Internal(i32);
pub struct PfnRecoCallback(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOCONF_HIGHCONFIDENCE: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOCONF_LOWCONFIDENCE: i32 = -1i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOCONF_MEDIUMCONFIDENCE: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOCONF_NOTSET: u32 = 128u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOFLAG_AUTOSPACE: u32 = 64u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOFLAG_COERCE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOFLAG_DISABLEPERSONALIZATION: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOFLAG_LINEMODE: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOFLAG_PREFIXOK: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOFLAG_SINGLESEG: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RECOFLAG_WORDMODE: u32 = 1u32;
pub struct RECO_ATTRS(i32);
pub struct RECO_GUIDE(i32);
pub struct RECO_LATTICE(i32);
pub struct RECO_LATTICE_COLUMN(i32);
pub struct RECO_LATTICE_ELEMENT(i32);
pub struct RECO_LATTICE_PROPERTIES(i32);
pub struct RECO_LATTICE_PROPERTY(i32);
pub struct RECO_RANGE(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_ADVISEINKCHANGE: i32 = 4096i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_ARBITRARY_ANGLE: i32 = 1024i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_BOXED_INPUT: i32 = 16i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_CAC_INPUT: i32 = 32i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_DONTCARE: i32 = 1i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_DOWN_AND_LEFT: i32 = 256i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_DOWN_AND_RIGHT: i32 = 512i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_FREE_INPUT: i32 = 4i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_LATTICE: i32 = 2048i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_LEFT_AND_DOWN: i32 = 128i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_LINED_INPUT: i32 = 8i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_OBJECT: i32 = 2i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_PERFORMSLINEBREAKING: i32 = 65536i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_PERSONALIZABLE: i32 = 16384i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_REQUIRESSEGMENTATIONBREAKING: i32 = 131072i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_RIGHT_AND_DOWN: i32 = 64i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const RF_STROKEREORDER: i32 = 8192i32;
pub struct RealTimeStylus(i32);
pub struct RealTimeStylusDataInterest(i32);
pub struct RealTimeStylusLockType(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const SAFE_PARTIAL: u32 = 1u32;
pub struct SCROLLDIRECTION(i32);
pub struct STROKE_RANGE(i32);
pub struct SYSTEM_EVENT_DATA(i32);
pub struct ScrollBarsConstants(i32);
pub struct SelAlignmentConstants(i32);
pub struct SelectionHitResult(i32);
pub struct SketchInk(i32);
pub struct StrokeBuilder(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct StylusInfo(i32);
pub struct StylusQueue(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_DISABLE_FLICKFALLBACKKEYS: u32 = 1048576u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_DISABLE_FLICKS: u32 = 65536u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_DISABLE_PENBARRELFEEDBACK: u32 = 16u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_DISABLE_PENTAPFEEDBACK: u32 = 8u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_DISABLE_PRESSANDHOLD: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_DISABLE_SMOOTHSCROLLING: u32 = 524288u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_DISABLE_TOUCHSWITCH: u32 = 32768u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_DISABLE_TOUCHUIFORCEOFF: u32 = 512u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_DISABLE_TOUCHUIFORCEON: u32 = 256u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_ENABLE_FLICKLEARNINGMODE: u32 = 262144u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_ENABLE_FLICKSONCONTEXT: u32 = 131072u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const TABLET_ENABLE_MULTITOUCHDATA: u32 = 16777216u32;
pub struct TabletDeviceKind(i32);
pub struct TabletHardwareCapabilities(i32);
pub struct TabletPropertyMetricUnit(i32);
pub struct TextInputPanel(i32);
pub struct TipAutoCompleteClient(i32);
pub struct VisualState(i32);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const WM_TABLET_ADDED: u32 = 712u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const WM_TABLET_DEFBASE: u32 = 704u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const WM_TABLET_DELETED: u32 = 713u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const WM_TABLET_FLICK: u32 = 715u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const WM_TABLET_MAXOFFSET: u32 = 32u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const WM_TABLET_QUERYSYSTEMGESTURESTATUS: u32 = 716u32;
#[repr(transparent)]
pub struct _IInkCollectorEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _IInkEditEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _IInkEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _IInkOverlayEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _IInkPictureEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _IInkRecognitionEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _IInkStrokesEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _IMathInputControlEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct _IPenInputPanelEvents(pub *mut ::core::ffi::c_void);
pub struct enumGetCandidateFlags(i32);
pub struct enumINKMETRIC_FLAGS(i32);
pub struct enumRECO_TYPE(i32);
