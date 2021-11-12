#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn AddStroke(hrc: HRECOCONTEXT, ppacketdesc: *const PACKET_DESCRIPTION, cbpacket: u32, ppacket: *const u8, pxform: *const super::super::Graphics::Gdi::XFORM) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddWordsToWordList(hwl: HRECOWORDLIST, pwcwords: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdviseInkChange(hrc: HRECOCONTEXT, bnewstroke: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn CreateContext(hrec: HRECOGNIZER, phrc: *mut HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
    pub fn CreateRecognizer(pclsid: *mut ::windows_sys::core::GUID, phrec: *mut HRECOGNIZER) -> ::windows_sys::core::HRESULT;
    pub fn DestroyContext(hrc: HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
    pub fn DestroyRecognizer(hrec: HRECOGNIZER) -> ::windows_sys::core::HRESULT;
    pub fn DestroyWordList(hwl: HRECOWORDLIST) -> ::windows_sys::core::HRESULT;
    pub fn EndInkInput(hrc: HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
    pub fn GetAllRecognizers(recognizerclsids: *mut *mut ::windows_sys::core::GUID, count: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBestResultString(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcbestresult: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn GetLatticePtr(hrc: HRECOCONTEXT, pplattice: *mut *mut RECO_LATTICE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLeftSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcleftseparator: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn GetRecoAttributes(hrec: HRECOGNIZER, precoattrs: *mut RECO_ATTRS) -> ::windows_sys::core::HRESULT;
    pub fn GetResultPropertyList(hrec: HRECOGNIZER, ppropertycount: *mut u32, ppropertyguid: *mut ::windows_sys::core::GUID) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRightSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcrightseparator: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn GetUnicodeRanges(hrec: HRECOGNIZER, pcranges: *mut u32, pcr: *mut CHARACTER_RANGE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsStringSupported(hrc: HRECOCONTEXT, wcstring: u32, pwcstring: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn LoadCachedAttributes(clsid: ::windows_sys::core::GUID, precoattributes: *mut RECO_ATTRS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeWordList(hrec: HRECOGNIZER, pbuffer: super::super::Foundation::PWSTR, phwl: *mut HRECOWORDLIST) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process(hrc: HRECOCONTEXT, pbpartialprocessing: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn SetEnabledUnicodeRanges(hrc: HRECOCONTEXT, cranges: u32, pcr: *mut CHARACTER_RANGE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetFactoid(hrc: HRECOCONTEXT, cwcfactoid: u32, pwcfactoid: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn SetFlags(hrc: HRECOCONTEXT, dwflags: u32) -> ::windows_sys::core::HRESULT;
    pub fn SetGuide(hrc: HRECOCONTEXT, pguide: *const RECO_GUIDE, iindex: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTextContext(hrc: HRECOCONTEXT, cwcbefore: u32, pwcbefore: super::super::Foundation::PWSTR, cwcafter: u32, pwcafter: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn SetWordList(hrc: HRECOCONTEXT, hwl: HRECOWORDLIST) -> ::windows_sys::core::HRESULT;
}
#[repr(transparent)]
pub struct ALT_BREAKS(pub i32);
pub const ALT_BREAKS_SAME: ALT_BREAKS = ALT_BREAKS(0i32);
pub const ALT_BREAKS_UNIQUE: ALT_BREAKS = ALT_BREAKS(1i32);
pub const ALT_BREAKS_FULL: ALT_BREAKS = ALT_BREAKS(2i32);
impl ::core::marker::Copy for ALT_BREAKS {}
impl ::core::clone::Clone for ALT_BREAKS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ASYNC_RECO_ADDSTROKE_FAILED: u32 = 4u32;
pub const ASYNC_RECO_INTERRUPTED: u32 = 1u32;
pub const ASYNC_RECO_PROCESS_FAILED: u32 = 2u32;
pub const ASYNC_RECO_RESETCONTEXT_FAILED: u32 = 16u32;
pub const ASYNC_RECO_SETCACMODE_FAILED: u32 = 8u32;
pub const ASYNC_RECO_SETFACTOID_FAILED: u32 = 128u32;
pub const ASYNC_RECO_SETFLAGS_FAILED: u32 = 64u32;
pub const ASYNC_RECO_SETGUIDE_FAILED: u32 = 32u32;
pub const ASYNC_RECO_SETTEXTCONTEXT_FAILED: u32 = 256u32;
pub const ASYNC_RECO_SETWORDLIST_FAILED: u32 = 512u32;
#[repr(transparent)]
pub struct AppearanceConstants(pub i32);
pub const rtfFlat: AppearanceConstants = AppearanceConstants(0i32);
pub const rtfThreeD: AppearanceConstants = AppearanceConstants(1i32);
impl ::core::marker::Copy for AppearanceConstants {}
impl ::core::clone::Clone for AppearanceConstants {
    fn clone(&self) -> Self {
        *self
    }
}
pub const BEST_COMPLETE: u32 = 2u32;
#[repr(transparent)]
pub struct BorderStyleConstants(pub i32);
pub const rtfNoBorder: BorderStyleConstants = BorderStyleConstants(0i32);
pub const rtfFixedSingle: BorderStyleConstants = BorderStyleConstants(1i32);
impl ::core::marker::Copy for BorderStyleConstants {}
impl ::core::clone::Clone for BorderStyleConstants {
    fn clone(&self) -> Self {
        *self
    }
}
pub const CAC_FULL: u32 = 0u32;
pub const CAC_PREFIX: u32 = 1u32;
pub const CAC_RANDOM: u32 = 2u32;
#[repr(C)]
pub struct CHARACTER_RANGE {
    pub wcLow: u16,
    pub cChars: u16,
}
impl ::core::marker::Copy for CHARACTER_RANGE {}
impl ::core::clone::Clone for CHARACTER_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CONFIDENCE_LEVEL(pub i32);
pub const CFL_STRONG: CONFIDENCE_LEVEL = CONFIDENCE_LEVEL(0i32);
pub const CFL_INTERMEDIATE: CONFIDENCE_LEVEL = CONFIDENCE_LEVEL(1i32);
pub const CFL_POOR: CONFIDENCE_LEVEL = CONFIDENCE_LEVEL(2i32);
impl ::core::marker::Copy for CONFIDENCE_LEVEL {}
impl ::core::clone::Clone for CONFIDENCE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CorrectionMode(pub i32);
pub const CorrectionMode_NotVisible: CorrectionMode = CorrectionMode(0i32);
pub const CorrectionMode_PreInsertion: CorrectionMode = CorrectionMode(1i32);
pub const CorrectionMode_PostInsertionCollapsed: CorrectionMode = CorrectionMode(2i32);
pub const CorrectionMode_PostInsertionExpanded: CorrectionMode = CorrectionMode(3i32);
impl ::core::marker::Copy for CorrectionMode {}
impl ::core::clone::Clone for CorrectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CorrectionPosition(pub i32);
pub const CorrectionPosition_Auto: CorrectionPosition = CorrectionPosition(0i32);
pub const CorrectionPosition_Bottom: CorrectionPosition = CorrectionPosition(1i32);
pub const CorrectionPosition_Top: CorrectionPosition = CorrectionPosition(2i32);
impl ::core::marker::Copy for CorrectionPosition {}
impl ::core::clone::Clone for CorrectionPosition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_Ink(pub i32);
pub const DISPID_IStrokes: DISPID_Ink = DISPID_Ink(1i32);
pub const DISPID_IExtendedProperties: DISPID_Ink = DISPID_Ink(2i32);
pub const DISPID_IGetBoundingBox: DISPID_Ink = DISPID_Ink(3i32);
pub const DISPID_IDeleteStrokes: DISPID_Ink = DISPID_Ink(4i32);
pub const DISPID_IDeleteStroke: DISPID_Ink = DISPID_Ink(5i32);
pub const DISPID_IExtractStrokes: DISPID_Ink = DISPID_Ink(6i32);
pub const DISPID_IExtractWithRectangle: DISPID_Ink = DISPID_Ink(7i32);
pub const DISPID_IDirty: DISPID_Ink = DISPID_Ink(8i32);
pub const DISPID_ICustomStrokes: DISPID_Ink = DISPID_Ink(9i32);
pub const DISPID_IClone: DISPID_Ink = DISPID_Ink(10i32);
pub const DISPID_IHitTestCircle: DISPID_Ink = DISPID_Ink(11i32);
pub const DISPID_IHitTestWithRectangle: DISPID_Ink = DISPID_Ink(12i32);
pub const DISPID_IHitTestWithLasso: DISPID_Ink = DISPID_Ink(13i32);
pub const DISPID_INearestPoint: DISPID_Ink = DISPID_Ink(14i32);
pub const DISPID_ICreateStrokes: DISPID_Ink = DISPID_Ink(15i32);
pub const DISPID_ICreateStroke: DISPID_Ink = DISPID_Ink(16i32);
pub const DISPID_IAddStrokesAtRectangle: DISPID_Ink = DISPID_Ink(17i32);
pub const DISPID_IClip: DISPID_Ink = DISPID_Ink(18i32);
pub const DISPID_ISave: DISPID_Ink = DISPID_Ink(19i32);
pub const DISPID_ILoad: DISPID_Ink = DISPID_Ink(20i32);
pub const DISPID_ICreateStrokeFromPoints: DISPID_Ink = DISPID_Ink(21i32);
pub const DISPID_IClipboardCopyWithRectangle: DISPID_Ink = DISPID_Ink(22i32);
pub const DISPID_IClipboardCopy: DISPID_Ink = DISPID_Ink(23i32);
pub const DISPID_ICanPaste: DISPID_Ink = DISPID_Ink(24i32);
pub const DISPID_IClipboardPaste: DISPID_Ink = DISPID_Ink(25i32);
impl ::core::marker::Copy for DISPID_Ink {}
impl ::core::clone::Clone for DISPID_Ink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkCollector(pub i32);
pub const DISPID_ICEnabled: DISPID_InkCollector = DISPID_InkCollector(1i32);
pub const DISPID_ICHwnd: DISPID_InkCollector = DISPID_InkCollector(2i32);
pub const DISPID_ICPaint: DISPID_InkCollector = DISPID_InkCollector(3i32);
pub const DISPID_ICText: DISPID_InkCollector = DISPID_InkCollector(4i32);
pub const DISPID_ICDefaultDrawingAttributes: DISPID_InkCollector = DISPID_InkCollector(5i32);
pub const DISPID_ICRenderer: DISPID_InkCollector = DISPID_InkCollector(6i32);
pub const DISPID_ICInk: DISPID_InkCollector = DISPID_InkCollector(7i32);
pub const DISPID_ICAutoRedraw: DISPID_InkCollector = DISPID_InkCollector(8i32);
pub const DISPID_ICCollectingInk: DISPID_InkCollector = DISPID_InkCollector(9i32);
pub const DISPID_ICSetEventInterest: DISPID_InkCollector = DISPID_InkCollector(10i32);
pub const DISPID_ICGetEventInterest: DISPID_InkCollector = DISPID_InkCollector(11i32);
pub const DISPID_IOEditingMode: DISPID_InkCollector = DISPID_InkCollector(12i32);
pub const DISPID_IOSelection: DISPID_InkCollector = DISPID_InkCollector(13i32);
pub const DISPID_IOAttachMode: DISPID_InkCollector = DISPID_InkCollector(14i32);
pub const DISPID_IOHitTestSelection: DISPID_InkCollector = DISPID_InkCollector(15i32);
pub const DISPID_IODraw: DISPID_InkCollector = DISPID_InkCollector(16i32);
pub const DISPID_IPPicture: DISPID_InkCollector = DISPID_InkCollector(17i32);
pub const DISPID_IPSizeMode: DISPID_InkCollector = DISPID_InkCollector(18i32);
pub const DISPID_IPBackColor: DISPID_InkCollector = DISPID_InkCollector(19i32);
pub const DISPID_ICCursors: DISPID_InkCollector = DISPID_InkCollector(20i32);
pub const DISPID_ICMarginX: DISPID_InkCollector = DISPID_InkCollector(21i32);
pub const DISPID_ICMarginY: DISPID_InkCollector = DISPID_InkCollector(22i32);
pub const DISPID_ICSetWindowInputRectangle: DISPID_InkCollector = DISPID_InkCollector(23i32);
pub const DISPID_ICGetWindowInputRectangle: DISPID_InkCollector = DISPID_InkCollector(24i32);
pub const DISPID_ICTablet: DISPID_InkCollector = DISPID_InkCollector(25i32);
pub const DISPID_ICSetAllTabletsMode: DISPID_InkCollector = DISPID_InkCollector(26i32);
pub const DISPID_ICSetSingleTabletIntegratedMode: DISPID_InkCollector = DISPID_InkCollector(27i32);
pub const DISPID_ICCollectionMode: DISPID_InkCollector = DISPID_InkCollector(28i32);
pub const DISPID_ICSetGestureStatus: DISPID_InkCollector = DISPID_InkCollector(29i32);
pub const DISPID_ICGetGestureStatus: DISPID_InkCollector = DISPID_InkCollector(30i32);
pub const DISPID_ICDynamicRendering: DISPID_InkCollector = DISPID_InkCollector(31i32);
pub const DISPID_ICDesiredPacketDescription: DISPID_InkCollector = DISPID_InkCollector(32i32);
pub const DISPID_IOEraserMode: DISPID_InkCollector = DISPID_InkCollector(33i32);
pub const DISPID_IOEraserWidth: DISPID_InkCollector = DISPID_InkCollector(34i32);
pub const DISPID_ICMouseIcon: DISPID_InkCollector = DISPID_InkCollector(35i32);
pub const DISPID_ICMousePointer: DISPID_InkCollector = DISPID_InkCollector(36i32);
pub const DISPID_IPInkEnabled: DISPID_InkCollector = DISPID_InkCollector(37i32);
pub const DISPID_ICSupportHighContrastInk: DISPID_InkCollector = DISPID_InkCollector(38i32);
pub const DISPID_IOSupportHighContrastSelectionUI: DISPID_InkCollector = DISPID_InkCollector(39i32);
impl ::core::marker::Copy for DISPID_InkCollector {}
impl ::core::clone::Clone for DISPID_InkCollector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkCollectorEvent(pub i32);
pub const DISPID_ICEStroke: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(1i32);
pub const DISPID_ICECursorDown: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(2i32);
pub const DISPID_ICENewPackets: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(3i32);
pub const DISPID_ICENewInAirPackets: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(4i32);
pub const DISPID_ICECursorButtonDown: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(5i32);
pub const DISPID_ICECursorButtonUp: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(6i32);
pub const DISPID_ICECursorInRange: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(7i32);
pub const DISPID_ICECursorOutOfRange: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(8i32);
pub const DISPID_ICESystemGesture: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(9i32);
pub const DISPID_ICEGesture: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(10i32);
pub const DISPID_ICETabletAdded: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(11i32);
pub const DISPID_ICETabletRemoved: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(12i32);
pub const DISPID_IOEPainting: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(13i32);
pub const DISPID_IOEPainted: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(14i32);
pub const DISPID_IOESelectionChanging: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(15i32);
pub const DISPID_IOESelectionChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(16i32);
pub const DISPID_IOESelectionMoving: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(17i32);
pub const DISPID_IOESelectionMoved: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(18i32);
pub const DISPID_IOESelectionResizing: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(19i32);
pub const DISPID_IOESelectionResized: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(20i32);
pub const DISPID_IOEStrokesDeleting: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(21i32);
pub const DISPID_IOEStrokesDeleted: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(22i32);
pub const DISPID_IPEChangeUICues: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(23i32);
pub const DISPID_IPEClick: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(24i32);
pub const DISPID_IPEDblClick: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(25i32);
pub const DISPID_IPEInvalidated: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(26i32);
pub const DISPID_IPEMouseDown: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(27i32);
pub const DISPID_IPEMouseEnter: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(28i32);
pub const DISPID_IPEMouseHover: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(29i32);
pub const DISPID_IPEMouseLeave: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(30i32);
pub const DISPID_IPEMouseMove: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(31i32);
pub const DISPID_IPEMouseUp: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(32i32);
pub const DISPID_IPEMouseWheel: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(33i32);
pub const DISPID_IPESizeModeChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(34i32);
pub const DISPID_IPEStyleChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(35i32);
pub const DISPID_IPESystemColorsChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(36i32);
pub const DISPID_IPEKeyDown: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(37i32);
pub const DISPID_IPEKeyPress: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(38i32);
pub const DISPID_IPEKeyUp: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(39i32);
pub const DISPID_IPEResize: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(40i32);
pub const DISPID_IPESizeChanged: DISPID_InkCollectorEvent = DISPID_InkCollectorEvent(41i32);
impl ::core::marker::Copy for DISPID_InkCollectorEvent {}
impl ::core::clone::Clone for DISPID_InkCollectorEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkCursor(pub i32);
pub const DISPID_ICsrName: DISPID_InkCursor = DISPID_InkCursor(0i32);
pub const DISPID_ICsrId: DISPID_InkCursor = DISPID_InkCursor(1i32);
pub const DISPID_ICsrDrawingAttributes: DISPID_InkCursor = DISPID_InkCursor(2i32);
pub const DISPID_ICsrButtons: DISPID_InkCursor = DISPID_InkCursor(3i32);
pub const DISPID_ICsrInverted: DISPID_InkCursor = DISPID_InkCursor(4i32);
pub const DISPID_ICsrTablet: DISPID_InkCursor = DISPID_InkCursor(5i32);
impl ::core::marker::Copy for DISPID_InkCursor {}
impl ::core::clone::Clone for DISPID_InkCursor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkCursorButton(pub i32);
pub const DISPID_ICBName: DISPID_InkCursorButton = DISPID_InkCursorButton(0i32);
pub const DISPID_ICBId: DISPID_InkCursorButton = DISPID_InkCursorButton(1i32);
pub const DISPID_ICBState: DISPID_InkCursorButton = DISPID_InkCursorButton(2i32);
impl ::core::marker::Copy for DISPID_InkCursorButton {}
impl ::core::clone::Clone for DISPID_InkCursorButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkCursorButtons(pub i32);
pub const DISPID_ICBs_NewEnum: DISPID_InkCursorButtons = DISPID_InkCursorButtons(-4i32);
pub const DISPID_ICBsItem: DISPID_InkCursorButtons = DISPID_InkCursorButtons(0i32);
pub const DISPID_ICBsCount: DISPID_InkCursorButtons = DISPID_InkCursorButtons(1i32);
impl ::core::marker::Copy for DISPID_InkCursorButtons {}
impl ::core::clone::Clone for DISPID_InkCursorButtons {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkCursors(pub i32);
pub const DISPID_ICs_NewEnum: DISPID_InkCursors = DISPID_InkCursors(-4i32);
pub const DISPID_ICsItem: DISPID_InkCursors = DISPID_InkCursors(0i32);
pub const DISPID_ICsCount: DISPID_InkCursors = DISPID_InkCursors(1i32);
impl ::core::marker::Copy for DISPID_InkCursors {}
impl ::core::clone::Clone for DISPID_InkCursors {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkCustomStrokes(pub i32);
pub const DISPID_ICSs_NewEnum: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(-4i32);
pub const DISPID_ICSsItem: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(0i32);
pub const DISPID_ICSsCount: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(1i32);
pub const DISPID_ICSsAdd: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(2i32);
pub const DISPID_ICSsRemove: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(3i32);
pub const DISPID_ICSsClear: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(4i32);
impl ::core::marker::Copy for DISPID_InkCustomStrokes {}
impl ::core::clone::Clone for DISPID_InkCustomStrokes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkDivider(pub i32);
pub const DISPID_IInkDivider_Strokes: DISPID_InkDivider = DISPID_InkDivider(1i32);
pub const DISPID_IInkDivider_RecognizerContext: DISPID_InkDivider = DISPID_InkDivider(2i32);
pub const DISPID_IInkDivider_LineHeight: DISPID_InkDivider = DISPID_InkDivider(3i32);
pub const DISPID_IInkDivider_Divide: DISPID_InkDivider = DISPID_InkDivider(4i32);
impl ::core::marker::Copy for DISPID_InkDivider {}
impl ::core::clone::Clone for DISPID_InkDivider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkDivisionResult(pub i32);
pub const DISPID_IInkDivisionResult_Strokes: DISPID_InkDivisionResult = DISPID_InkDivisionResult(1i32);
pub const DISPID_IInkDivisionResult_ResultByType: DISPID_InkDivisionResult = DISPID_InkDivisionResult(2i32);
impl ::core::marker::Copy for DISPID_InkDivisionResult {}
impl ::core::clone::Clone for DISPID_InkDivisionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkDivisionUnit(pub i32);
pub const DISPID_IInkDivisionUnit_Strokes: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(1i32);
pub const DISPID_IInkDivisionUnit_DivisionType: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(2i32);
pub const DISPID_IInkDivisionUnit_RecognizedString: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(3i32);
pub const DISPID_IInkDivisionUnit_RotationTransform: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(4i32);
impl ::core::marker::Copy for DISPID_InkDivisionUnit {}
impl ::core::clone::Clone for DISPID_InkDivisionUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkDivisionUnits(pub i32);
pub const DISPID_IInkDivisionUnits_NewEnum: DISPID_InkDivisionUnits = DISPID_InkDivisionUnits(-4i32);
pub const DISPID_IInkDivisionUnits_Item: DISPID_InkDivisionUnits = DISPID_InkDivisionUnits(0i32);
pub const DISPID_IInkDivisionUnits_Count: DISPID_InkDivisionUnits = DISPID_InkDivisionUnits(1i32);
impl ::core::marker::Copy for DISPID_InkDivisionUnits {}
impl ::core::clone::Clone for DISPID_InkDivisionUnits {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkDrawingAttributes(pub i32);
pub const DISPID_DAHeight: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(1i32);
pub const DISPID_DAColor: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(2i32);
pub const DISPID_DAWidth: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(3i32);
pub const DISPID_DAFitToCurve: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(4i32);
pub const DISPID_DAIgnorePressure: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(5i32);
pub const DISPID_DAAntiAliased: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(6i32);
pub const DISPID_DATransparency: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(7i32);
pub const DISPID_DARasterOperation: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(8i32);
pub const DISPID_DAPenTip: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(9i32);
pub const DISPID_DAClone: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(10i32);
pub const DISPID_DAExtendedProperties: DISPID_InkDrawingAttributes = DISPID_InkDrawingAttributes(11i32);
impl ::core::marker::Copy for DISPID_InkDrawingAttributes {}
impl ::core::clone::Clone for DISPID_InkDrawingAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkEdit(pub i32);
pub const DISPID_Text: DISPID_InkEdit = DISPID_InkEdit(0i32);
pub const DISPID_TextRTF: DISPID_InkEdit = DISPID_InkEdit(1i32);
pub const DISPID_Hwnd: DISPID_InkEdit = DISPID_InkEdit(2i32);
pub const DISPID_DisableNoScroll: DISPID_InkEdit = DISPID_InkEdit(3i32);
pub const DISPID_Locked: DISPID_InkEdit = DISPID_InkEdit(4i32);
pub const DISPID_Enabled: DISPID_InkEdit = DISPID_InkEdit(5i32);
pub const DISPID_MaxLength: DISPID_InkEdit = DISPID_InkEdit(6i32);
pub const DISPID_MultiLine: DISPID_InkEdit = DISPID_InkEdit(7i32);
pub const DISPID_ScrollBars: DISPID_InkEdit = DISPID_InkEdit(8i32);
pub const DISPID_RTSelStart: DISPID_InkEdit = DISPID_InkEdit(9i32);
pub const DISPID_RTSelLength: DISPID_InkEdit = DISPID_InkEdit(10i32);
pub const DISPID_RTSelText: DISPID_InkEdit = DISPID_InkEdit(11i32);
pub const DISPID_SelAlignment: DISPID_InkEdit = DISPID_InkEdit(12i32);
pub const DISPID_SelBold: DISPID_InkEdit = DISPID_InkEdit(13i32);
pub const DISPID_SelCharOffset: DISPID_InkEdit = DISPID_InkEdit(14i32);
pub const DISPID_SelColor: DISPID_InkEdit = DISPID_InkEdit(15i32);
pub const DISPID_SelFontName: DISPID_InkEdit = DISPID_InkEdit(16i32);
pub const DISPID_SelFontSize: DISPID_InkEdit = DISPID_InkEdit(17i32);
pub const DISPID_SelItalic: DISPID_InkEdit = DISPID_InkEdit(18i32);
pub const DISPID_SelRTF: DISPID_InkEdit = DISPID_InkEdit(19i32);
pub const DISPID_SelUnderline: DISPID_InkEdit = DISPID_InkEdit(20i32);
pub const DISPID_DragIcon: DISPID_InkEdit = DISPID_InkEdit(21i32);
pub const DISPID_Status: DISPID_InkEdit = DISPID_InkEdit(22i32);
pub const DISPID_UseMouseForInput: DISPID_InkEdit = DISPID_InkEdit(23i32);
pub const DISPID_InkMode: DISPID_InkEdit = DISPID_InkEdit(24i32);
pub const DISPID_InkInsertMode: DISPID_InkEdit = DISPID_InkEdit(25i32);
pub const DISPID_RecoTimeout: DISPID_InkEdit = DISPID_InkEdit(26i32);
pub const DISPID_DrawAttr: DISPID_InkEdit = DISPID_InkEdit(27i32);
pub const DISPID_Recognizer: DISPID_InkEdit = DISPID_InkEdit(28i32);
pub const DISPID_Factoid: DISPID_InkEdit = DISPID_InkEdit(29i32);
pub const DISPID_SelInk: DISPID_InkEdit = DISPID_InkEdit(30i32);
pub const DISPID_SelInksDisplayMode: DISPID_InkEdit = DISPID_InkEdit(31i32);
pub const DISPID_Recognize: DISPID_InkEdit = DISPID_InkEdit(32i32);
pub const DISPID_GetGestStatus: DISPID_InkEdit = DISPID_InkEdit(33i32);
pub const DISPID_SetGestStatus: DISPID_InkEdit = DISPID_InkEdit(34i32);
pub const DISPID_Refresh: DISPID_InkEdit = DISPID_InkEdit(35i32);
impl ::core::marker::Copy for DISPID_InkEdit {}
impl ::core::clone::Clone for DISPID_InkEdit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkEditEvents(pub i32);
pub const DISPID_IeeChange: DISPID_InkEditEvents = DISPID_InkEditEvents(1i32);
pub const DISPID_IeeSelChange: DISPID_InkEditEvents = DISPID_InkEditEvents(2i32);
pub const DISPID_IeeKeyDown: DISPID_InkEditEvents = DISPID_InkEditEvents(3i32);
pub const DISPID_IeeKeyUp: DISPID_InkEditEvents = DISPID_InkEditEvents(4i32);
pub const DISPID_IeeMouseUp: DISPID_InkEditEvents = DISPID_InkEditEvents(5i32);
pub const DISPID_IeeMouseDown: DISPID_InkEditEvents = DISPID_InkEditEvents(6i32);
pub const DISPID_IeeKeyPress: DISPID_InkEditEvents = DISPID_InkEditEvents(7i32);
pub const DISPID_IeeDblClick: DISPID_InkEditEvents = DISPID_InkEditEvents(8i32);
pub const DISPID_IeeClick: DISPID_InkEditEvents = DISPID_InkEditEvents(9i32);
pub const DISPID_IeeMouseMove: DISPID_InkEditEvents = DISPID_InkEditEvents(10i32);
pub const DISPID_IeeCursorDown: DISPID_InkEditEvents = DISPID_InkEditEvents(21i32);
pub const DISPID_IeeStroke: DISPID_InkEditEvents = DISPID_InkEditEvents(22i32);
pub const DISPID_IeeGesture: DISPID_InkEditEvents = DISPID_InkEditEvents(23i32);
pub const DISPID_IeeRecognitionResult: DISPID_InkEditEvents = DISPID_InkEditEvents(24i32);
impl ::core::marker::Copy for DISPID_InkEditEvents {}
impl ::core::clone::Clone for DISPID_InkEditEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkEvent(pub i32);
pub const DISPID_IEInkAdded: DISPID_InkEvent = DISPID_InkEvent(1i32);
pub const DISPID_IEInkDeleted: DISPID_InkEvent = DISPID_InkEvent(2i32);
impl ::core::marker::Copy for DISPID_InkEvent {}
impl ::core::clone::Clone for DISPID_InkEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkExtendedProperties(pub i32);
pub const DISPID_IEPs_NewEnum: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(-4i32);
pub const DISPID_IEPsItem: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(0i32);
pub const DISPID_IEPsCount: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(1i32);
pub const DISPID_IEPsAdd: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(2i32);
pub const DISPID_IEPsRemove: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(3i32);
pub const DISPID_IEPsClear: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(4i32);
pub const DISPID_IEPsDoesPropertyExist: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(5i32);
impl ::core::marker::Copy for DISPID_InkExtendedProperties {}
impl ::core::clone::Clone for DISPID_InkExtendedProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkExtendedProperty(pub i32);
pub const DISPID_IEPGuid: DISPID_InkExtendedProperty = DISPID_InkExtendedProperty(1i32);
pub const DISPID_IEPData: DISPID_InkExtendedProperty = DISPID_InkExtendedProperty(2i32);
impl ::core::marker::Copy for DISPID_InkExtendedProperty {}
impl ::core::clone::Clone for DISPID_InkExtendedProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkGesture(pub i32);
pub const DISPID_IGId: DISPID_InkGesture = DISPID_InkGesture(0i32);
pub const DISPID_IGGetHotPoint: DISPID_InkGesture = DISPID_InkGesture(1i32);
pub const DISPID_IGConfidence: DISPID_InkGesture = DISPID_InkGesture(2i32);
impl ::core::marker::Copy for DISPID_InkGesture {}
impl ::core::clone::Clone for DISPID_InkGesture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecoAlternate(pub i32);
pub const DISPID_InkRecoAlternate_String: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(1i32);
pub const DISPID_InkRecoAlternate_LineNumber: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(2i32);
pub const DISPID_InkRecoAlternate_Baseline: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(3i32);
pub const DISPID_InkRecoAlternate_Midline: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(4i32);
pub const DISPID_InkRecoAlternate_Ascender: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(5i32);
pub const DISPID_InkRecoAlternate_Descender: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(6i32);
pub const DISPID_InkRecoAlternate_Confidence: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(7i32);
pub const DISPID_InkRecoAlternate_Strokes: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(8i32);
pub const DISPID_InkRecoAlternate_GetStrokesFromStrokeRanges: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(9i32);
pub const DISPID_InkRecoAlternate_GetStrokesFromTextRange: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(10i32);
pub const DISPID_InkRecoAlternate_GetTextRangeFromStrokes: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(11i32);
pub const DISPID_InkRecoAlternate_GetPropertyValue: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(12i32);
pub const DISPID_InkRecoAlternate_LineAlternates: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(13i32);
pub const DISPID_InkRecoAlternate_ConfidenceAlternates: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(14i32);
pub const DISPID_InkRecoAlternate_AlternatesWithConstantPropertyValues: DISPID_InkRecoAlternate = DISPID_InkRecoAlternate(15i32);
impl ::core::marker::Copy for DISPID_InkRecoAlternate {}
impl ::core::clone::Clone for DISPID_InkRecoAlternate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecoContext(pub i32);
pub const DISPID_IRecoCtx_Strokes: DISPID_InkRecoContext = DISPID_InkRecoContext(1i32);
pub const DISPID_IRecoCtx_CharacterAutoCompletionMode: DISPID_InkRecoContext = DISPID_InkRecoContext(2i32);
pub const DISPID_IRecoCtx_Factoid: DISPID_InkRecoContext = DISPID_InkRecoContext(3i32);
pub const DISPID_IRecoCtx_WordList: DISPID_InkRecoContext = DISPID_InkRecoContext(4i32);
pub const DISPID_IRecoCtx_Recognizer: DISPID_InkRecoContext = DISPID_InkRecoContext(5i32);
pub const DISPID_IRecoCtx_Guide: DISPID_InkRecoContext = DISPID_InkRecoContext(6i32);
pub const DISPID_IRecoCtx_Flags: DISPID_InkRecoContext = DISPID_InkRecoContext(7i32);
pub const DISPID_IRecoCtx_PrefixText: DISPID_InkRecoContext = DISPID_InkRecoContext(8i32);
pub const DISPID_IRecoCtx_SuffixText: DISPID_InkRecoContext = DISPID_InkRecoContext(9i32);
pub const DISPID_IRecoCtx_StopRecognition: DISPID_InkRecoContext = DISPID_InkRecoContext(10i32);
pub const DISPID_IRecoCtx_Clone: DISPID_InkRecoContext = DISPID_InkRecoContext(11i32);
pub const DISPID_IRecoCtx_Recognize: DISPID_InkRecoContext = DISPID_InkRecoContext(12i32);
pub const DISPID_IRecoCtx_StopBackgroundRecognition: DISPID_InkRecoContext = DISPID_InkRecoContext(13i32);
pub const DISPID_IRecoCtx_EndInkInput: DISPID_InkRecoContext = DISPID_InkRecoContext(14i32);
pub const DISPID_IRecoCtx_BackgroundRecognize: DISPID_InkRecoContext = DISPID_InkRecoContext(15i32);
pub const DISPID_IRecoCtx_BackgroundRecognizeWithAlternates: DISPID_InkRecoContext = DISPID_InkRecoContext(16i32);
pub const DISPID_IRecoCtx_IsStringSupported: DISPID_InkRecoContext = DISPID_InkRecoContext(17i32);
impl ::core::marker::Copy for DISPID_InkRecoContext {}
impl ::core::clone::Clone for DISPID_InkRecoContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecoContext2(pub i32);
pub const DISPID_IRecoCtx2_EnabledUnicodeRanges: DISPID_InkRecoContext2 = DISPID_InkRecoContext2(0i32);
impl ::core::marker::Copy for DISPID_InkRecoContext2 {}
impl ::core::clone::Clone for DISPID_InkRecoContext2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecognitionAlternates(pub i32);
pub const DISPID_InkRecognitionAlternates_NewEnum: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(-4i32);
pub const DISPID_InkRecognitionAlternates_Item: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(0i32);
pub const DISPID_InkRecognitionAlternates_Count: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(1i32);
pub const DISPID_InkRecognitionAlternates_Strokes: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(2i32);
impl ::core::marker::Copy for DISPID_InkRecognitionAlternates {}
impl ::core::clone::Clone for DISPID_InkRecognitionAlternates {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecognitionEvent(pub i32);
pub const DISPID_IRERecognitionWithAlternates: DISPID_InkRecognitionEvent = DISPID_InkRecognitionEvent(1i32);
pub const DISPID_IRERecognition: DISPID_InkRecognitionEvent = DISPID_InkRecognitionEvent(2i32);
impl ::core::marker::Copy for DISPID_InkRecognitionEvent {}
impl ::core::clone::Clone for DISPID_InkRecognitionEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecognitionResult(pub i32);
pub const DISPID_InkRecognitionResult_TopString: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(1i32);
pub const DISPID_InkRecognitionResult_TopAlternate: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(2i32);
pub const DISPID_InkRecognitionResult_Strokes: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(3i32);
pub const DISPID_InkRecognitionResult_TopConfidence: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(4i32);
pub const DISPID_InkRecognitionResult_AlternatesFromSelection: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(5i32);
pub const DISPID_InkRecognitionResult_ModifyTopAlternate: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(6i32);
pub const DISPID_InkRecognitionResult_SetResultOnStrokes: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(7i32);
impl ::core::marker::Copy for DISPID_InkRecognitionResult {}
impl ::core::clone::Clone for DISPID_InkRecognitionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecognizer(pub i32);
pub const DISPID_RecoClsid: DISPID_InkRecognizer = DISPID_InkRecognizer(1i32);
pub const DISPID_RecoName: DISPID_InkRecognizer = DISPID_InkRecognizer(2i32);
pub const DISPID_RecoVendor: DISPID_InkRecognizer = DISPID_InkRecognizer(3i32);
pub const DISPID_RecoCapabilities: DISPID_InkRecognizer = DISPID_InkRecognizer(4i32);
pub const DISPID_RecoLanguageID: DISPID_InkRecognizer = DISPID_InkRecognizer(5i32);
pub const DISPID_RecoPreferredPacketDescription: DISPID_InkRecognizer = DISPID_InkRecognizer(6i32);
pub const DISPID_RecoCreateRecognizerContext: DISPID_InkRecognizer = DISPID_InkRecognizer(7i32);
pub const DISPID_RecoSupportedProperties: DISPID_InkRecognizer = DISPID_InkRecognizer(8i32);
impl ::core::marker::Copy for DISPID_InkRecognizer {}
impl ::core::clone::Clone for DISPID_InkRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecognizer2(pub i32);
pub const DISPID_RecoId: DISPID_InkRecognizer2 = DISPID_InkRecognizer2(0i32);
pub const DISPID_RecoUnicodeRanges: DISPID_InkRecognizer2 = DISPID_InkRecognizer2(1i32);
impl ::core::marker::Copy for DISPID_InkRecognizer2 {}
impl ::core::clone::Clone for DISPID_InkRecognizer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecognizerGuide(pub i32);
pub const DISPID_IRGWritingBox: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(1i32);
pub const DISPID_IRGDrawnBox: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(2i32);
pub const DISPID_IRGRows: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(3i32);
pub const DISPID_IRGColumns: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(4i32);
pub const DISPID_IRGMidline: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(5i32);
pub const DISPID_IRGGuideData: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(6i32);
impl ::core::marker::Copy for DISPID_InkRecognizerGuide {}
impl ::core::clone::Clone for DISPID_InkRecognizerGuide {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRecognizers(pub i32);
pub const DISPID_IRecos_NewEnum: DISPID_InkRecognizers = DISPID_InkRecognizers(-4i32);
pub const DISPID_IRecosItem: DISPID_InkRecognizers = DISPID_InkRecognizers(0i32);
pub const DISPID_IRecosCount: DISPID_InkRecognizers = DISPID_InkRecognizers(1i32);
pub const DISPID_IRecosGetDefaultRecognizer: DISPID_InkRecognizers = DISPID_InkRecognizers(2i32);
impl ::core::marker::Copy for DISPID_InkRecognizers {}
impl ::core::clone::Clone for DISPID_InkRecognizers {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRectangle(pub i32);
pub const DISPID_IRTop: DISPID_InkRectangle = DISPID_InkRectangle(1i32);
pub const DISPID_IRLeft: DISPID_InkRectangle = DISPID_InkRectangle(2i32);
pub const DISPID_IRBottom: DISPID_InkRectangle = DISPID_InkRectangle(3i32);
pub const DISPID_IRRight: DISPID_InkRectangle = DISPID_InkRectangle(4i32);
pub const DISPID_IRGetRectangle: DISPID_InkRectangle = DISPID_InkRectangle(5i32);
pub const DISPID_IRSetRectangle: DISPID_InkRectangle = DISPID_InkRectangle(6i32);
pub const DISPID_IRData: DISPID_InkRectangle = DISPID_InkRectangle(7i32);
impl ::core::marker::Copy for DISPID_InkRectangle {}
impl ::core::clone::Clone for DISPID_InkRectangle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkRenderer(pub i32);
pub const DISPID_IRGetViewTransform: DISPID_InkRenderer = DISPID_InkRenderer(1i32);
pub const DISPID_IRSetViewTransform: DISPID_InkRenderer = DISPID_InkRenderer(2i32);
pub const DISPID_IRGetObjectTransform: DISPID_InkRenderer = DISPID_InkRenderer(3i32);
pub const DISPID_IRSetObjectTransform: DISPID_InkRenderer = DISPID_InkRenderer(4i32);
pub const DISPID_IRDraw: DISPID_InkRenderer = DISPID_InkRenderer(5i32);
pub const DISPID_IRDrawStroke: DISPID_InkRenderer = DISPID_InkRenderer(6i32);
pub const DISPID_IRPixelToInkSpace: DISPID_InkRenderer = DISPID_InkRenderer(7i32);
pub const DISPID_IRInkSpaceToPixel: DISPID_InkRenderer = DISPID_InkRenderer(8i32);
pub const DISPID_IRPixelToInkSpaceFromPoints: DISPID_InkRenderer = DISPID_InkRenderer(9i32);
pub const DISPID_IRInkSpaceToPixelFromPoints: DISPID_InkRenderer = DISPID_InkRenderer(10i32);
pub const DISPID_IRMeasure: DISPID_InkRenderer = DISPID_InkRenderer(11i32);
pub const DISPID_IRMeasureStroke: DISPID_InkRenderer = DISPID_InkRenderer(12i32);
pub const DISPID_IRMove: DISPID_InkRenderer = DISPID_InkRenderer(13i32);
pub const DISPID_IRRotate: DISPID_InkRenderer = DISPID_InkRenderer(14i32);
pub const DISPID_IRScale: DISPID_InkRenderer = DISPID_InkRenderer(15i32);
impl ::core::marker::Copy for DISPID_InkRenderer {}
impl ::core::clone::Clone for DISPID_InkRenderer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkStrokeDisp(pub i32);
pub const DISPID_ISDInkIndex: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(1i32);
pub const DISPID_ISDID: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(2i32);
pub const DISPID_ISDGetBoundingBox: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(3i32);
pub const DISPID_ISDDrawingAttributes: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(4i32);
pub const DISPID_ISDFindIntersections: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(5i32);
pub const DISPID_ISDGetRectangleIntersections: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(6i32);
pub const DISPID_ISDClip: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(7i32);
pub const DISPID_ISDHitTestCircle: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(8i32);
pub const DISPID_ISDNearestPoint: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(9i32);
pub const DISPID_ISDSplit: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(10i32);
pub const DISPID_ISDExtendedProperties: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(11i32);
pub const DISPID_ISDInk: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(12i32);
pub const DISPID_ISDBezierPoints: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(13i32);
pub const DISPID_ISDPolylineCusps: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(14i32);
pub const DISPID_ISDBezierCusps: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(15i32);
pub const DISPID_ISDSelfIntersections: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(16i32);
pub const DISPID_ISDPacketCount: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(17i32);
pub const DISPID_ISDPacketSize: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(18i32);
pub const DISPID_ISDPacketDescription: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(19i32);
pub const DISPID_ISDDeleted: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(20i32);
pub const DISPID_ISDGetPacketDescriptionPropertyMetrics: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(21i32);
pub const DISPID_ISDGetPoints: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(22i32);
pub const DISPID_ISDSetPoints: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(23i32);
pub const DISPID_ISDGetPacketData: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(24i32);
pub const DISPID_ISDGetPacketValuesByProperty: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(25i32);
pub const DISPID_ISDSetPacketValuesByProperty: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(26i32);
pub const DISPID_ISDGetFlattenedBezierPoints: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(27i32);
pub const DISPID_ISDScaleToRectangle: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(28i32);
pub const DISPID_ISDTransform: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(29i32);
pub const DISPID_ISDMove: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(30i32);
pub const DISPID_ISDRotate: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(31i32);
pub const DISPID_ISDShear: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(32i32);
pub const DISPID_ISDScale: DISPID_InkStrokeDisp = DISPID_InkStrokeDisp(33i32);
impl ::core::marker::Copy for DISPID_InkStrokeDisp {}
impl ::core::clone::Clone for DISPID_InkStrokeDisp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkStrokes(pub i32);
pub const DISPID_ISs_NewEnum: DISPID_InkStrokes = DISPID_InkStrokes(-4i32);
pub const DISPID_ISsItem: DISPID_InkStrokes = DISPID_InkStrokes(0i32);
pub const DISPID_ISsCount: DISPID_InkStrokes = DISPID_InkStrokes(1i32);
pub const DISPID_ISsValid: DISPID_InkStrokes = DISPID_InkStrokes(2i32);
pub const DISPID_ISsInk: DISPID_InkStrokes = DISPID_InkStrokes(3i32);
pub const DISPID_ISsAdd: DISPID_InkStrokes = DISPID_InkStrokes(4i32);
pub const DISPID_ISsAddStrokes: DISPID_InkStrokes = DISPID_InkStrokes(5i32);
pub const DISPID_ISsRemove: DISPID_InkStrokes = DISPID_InkStrokes(6i32);
pub const DISPID_ISsRemoveStrokes: DISPID_InkStrokes = DISPID_InkStrokes(7i32);
pub const DISPID_ISsToString: DISPID_InkStrokes = DISPID_InkStrokes(8i32);
pub const DISPID_ISsModifyDrawingAttributes: DISPID_InkStrokes = DISPID_InkStrokes(9i32);
pub const DISPID_ISsGetBoundingBox: DISPID_InkStrokes = DISPID_InkStrokes(10i32);
pub const DISPID_ISsScaleToRectangle: DISPID_InkStrokes = DISPID_InkStrokes(11i32);
pub const DISPID_ISsTransform: DISPID_InkStrokes = DISPID_InkStrokes(12i32);
pub const DISPID_ISsMove: DISPID_InkStrokes = DISPID_InkStrokes(13i32);
pub const DISPID_ISsRotate: DISPID_InkStrokes = DISPID_InkStrokes(14i32);
pub const DISPID_ISsShear: DISPID_InkStrokes = DISPID_InkStrokes(15i32);
pub const DISPID_ISsScale: DISPID_InkStrokes = DISPID_InkStrokes(16i32);
pub const DISPID_ISsClip: DISPID_InkStrokes = DISPID_InkStrokes(17i32);
pub const DISPID_ISsRecognitionResult: DISPID_InkStrokes = DISPID_InkStrokes(18i32);
pub const DISPID_ISsRemoveRecognitionResult: DISPID_InkStrokes = DISPID_InkStrokes(19i32);
impl ::core::marker::Copy for DISPID_InkStrokes {}
impl ::core::clone::Clone for DISPID_InkStrokes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkTablet(pub i32);
pub const DISPID_ITName: DISPID_InkTablet = DISPID_InkTablet(0i32);
pub const DISPID_ITPlugAndPlayId: DISPID_InkTablet = DISPID_InkTablet(1i32);
pub const DISPID_ITPropertyMetrics: DISPID_InkTablet = DISPID_InkTablet(2i32);
pub const DISPID_ITIsPacketPropertySupported: DISPID_InkTablet = DISPID_InkTablet(3i32);
pub const DISPID_ITMaximumInputRectangle: DISPID_InkTablet = DISPID_InkTablet(4i32);
pub const DISPID_ITHardwareCapabilities: DISPID_InkTablet = DISPID_InkTablet(5i32);
impl ::core::marker::Copy for DISPID_InkTablet {}
impl ::core::clone::Clone for DISPID_InkTablet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkTablet2(pub i32);
pub const DISPID_IT2DeviceKind: DISPID_InkTablet2 = DISPID_InkTablet2(0i32);
impl ::core::marker::Copy for DISPID_InkTablet2 {}
impl ::core::clone::Clone for DISPID_InkTablet2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkTablet3(pub i32);
pub const DISPID_IT3IsMultiTouch: DISPID_InkTablet3 = DISPID_InkTablet3(0i32);
pub const DISPID_IT3MaximumCursors: DISPID_InkTablet3 = DISPID_InkTablet3(1i32);
impl ::core::marker::Copy for DISPID_InkTablet3 {}
impl ::core::clone::Clone for DISPID_InkTablet3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkTablets(pub i32);
pub const DISPID_ITs_NewEnum: DISPID_InkTablets = DISPID_InkTablets(-4i32);
pub const DISPID_ITsItem: DISPID_InkTablets = DISPID_InkTablets(0i32);
pub const DISPID_ITsDefaultTablet: DISPID_InkTablets = DISPID_InkTablets(1i32);
pub const DISPID_ITsCount: DISPID_InkTablets = DISPID_InkTablets(2i32);
pub const DISPID_ITsIsPacketPropertySupported: DISPID_InkTablets = DISPID_InkTablets(3i32);
impl ::core::marker::Copy for DISPID_InkTablets {}
impl ::core::clone::Clone for DISPID_InkTablets {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkTransform(pub i32);
pub const DISPID_ITReset: DISPID_InkTransform = DISPID_InkTransform(1i32);
pub const DISPID_ITTranslate: DISPID_InkTransform = DISPID_InkTransform(2i32);
pub const DISPID_ITRotate: DISPID_InkTransform = DISPID_InkTransform(3i32);
pub const DISPID_ITReflect: DISPID_InkTransform = DISPID_InkTransform(4i32);
pub const DISPID_ITShear: DISPID_InkTransform = DISPID_InkTransform(5i32);
pub const DISPID_ITScale: DISPID_InkTransform = DISPID_InkTransform(6i32);
pub const DISPID_ITeM11: DISPID_InkTransform = DISPID_InkTransform(7i32);
pub const DISPID_ITeM12: DISPID_InkTransform = DISPID_InkTransform(8i32);
pub const DISPID_ITeM21: DISPID_InkTransform = DISPID_InkTransform(9i32);
pub const DISPID_ITeM22: DISPID_InkTransform = DISPID_InkTransform(10i32);
pub const DISPID_ITeDx: DISPID_InkTransform = DISPID_InkTransform(11i32);
pub const DISPID_ITeDy: DISPID_InkTransform = DISPID_InkTransform(12i32);
pub const DISPID_ITGetTransform: DISPID_InkTransform = DISPID_InkTransform(13i32);
pub const DISPID_ITSetTransform: DISPID_InkTransform = DISPID_InkTransform(14i32);
pub const DISPID_ITData: DISPID_InkTransform = DISPID_InkTransform(15i32);
impl ::core::marker::Copy for DISPID_InkTransform {}
impl ::core::clone::Clone for DISPID_InkTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkWordList(pub i32);
pub const DISPID_InkWordList_AddWord: DISPID_InkWordList = DISPID_InkWordList(0i32);
pub const DISPID_InkWordList_RemoveWord: DISPID_InkWordList = DISPID_InkWordList(1i32);
pub const DISPID_InkWordList_Merge: DISPID_InkWordList = DISPID_InkWordList(2i32);
impl ::core::marker::Copy for DISPID_InkWordList {}
impl ::core::clone::Clone for DISPID_InkWordList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_InkWordList2(pub i32);
pub const DISPID_InkWordList2_AddWords: DISPID_InkWordList2 = DISPID_InkWordList2(3i32);
impl ::core::marker::Copy for DISPID_InkWordList2 {}
impl ::core::clone::Clone for DISPID_InkWordList2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_MathInputControlEvents(pub i32);
pub const DISPID_MICInsert: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(0i32);
pub const DISPID_MICClose: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(1i32);
pub const DISPID_MICPaint: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(2i32);
pub const DISPID_MICClear: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(3i32);
impl ::core::marker::Copy for DISPID_MathInputControlEvents {}
impl ::core::clone::Clone for DISPID_MathInputControlEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_PenInputPanel(pub i32);
pub const DISPID_PIPAttachedEditWindow: DISPID_PenInputPanel = DISPID_PenInputPanel(0i32);
pub const DISPID_PIPFactoid: DISPID_PenInputPanel = DISPID_PenInputPanel(1i32);
pub const DISPID_PIPCurrentPanel: DISPID_PenInputPanel = DISPID_PenInputPanel(2i32);
pub const DISPID_PIPDefaultPanel: DISPID_PenInputPanel = DISPID_PenInputPanel(3i32);
pub const DISPID_PIPVisible: DISPID_PenInputPanel = DISPID_PenInputPanel(4i32);
pub const DISPID_PIPTop: DISPID_PenInputPanel = DISPID_PenInputPanel(5i32);
pub const DISPID_PIPLeft: DISPID_PenInputPanel = DISPID_PenInputPanel(6i32);
pub const DISPID_PIPWidth: DISPID_PenInputPanel = DISPID_PenInputPanel(7i32);
pub const DISPID_PIPHeight: DISPID_PenInputPanel = DISPID_PenInputPanel(8i32);
pub const DISPID_PIPMoveTo: DISPID_PenInputPanel = DISPID_PenInputPanel(9i32);
pub const DISPID_PIPCommitPendingInput: DISPID_PenInputPanel = DISPID_PenInputPanel(10i32);
pub const DISPID_PIPRefresh: DISPID_PenInputPanel = DISPID_PenInputPanel(11i32);
pub const DISPID_PIPBusy: DISPID_PenInputPanel = DISPID_PenInputPanel(12i32);
pub const DISPID_PIPVerticalOffset: DISPID_PenInputPanel = DISPID_PenInputPanel(13i32);
pub const DISPID_PIPHorizontalOffset: DISPID_PenInputPanel = DISPID_PenInputPanel(14i32);
pub const DISPID_PIPEnableTsf: DISPID_PenInputPanel = DISPID_PenInputPanel(15i32);
pub const DISPID_PIPAutoShow: DISPID_PenInputPanel = DISPID_PenInputPanel(16i32);
impl ::core::marker::Copy for DISPID_PenInputPanel {}
impl ::core::clone::Clone for DISPID_PenInputPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_PenInputPanelEvents(pub i32);
pub const DISPID_PIPEVisibleChanged: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(0i32);
pub const DISPID_PIPEPanelChanged: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(1i32);
pub const DISPID_PIPEInputFailed: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(2i32);
pub const DISPID_PIPEPanelMoving: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(3i32);
impl ::core::marker::Copy for DISPID_PenInputPanelEvents {}
impl ::core::clone::Clone for DISPID_PenInputPanelEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DISPID_StrokeEvent(pub i32);
pub const DISPID_SEStrokesAdded: DISPID_StrokeEvent = DISPID_StrokeEvent(1i32);
pub const DISPID_SEStrokesRemoved: DISPID_StrokeEvent = DISPID_StrokeEvent(2i32);
impl ::core::marker::Copy for DISPID_StrokeEvent {}
impl ::core::clone::Clone for DISPID_StrokeEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DYNAMIC_RENDERER_CACHED_DATA {
    pub strokeId: i32,
    pub dynamicRenderer: IDynamicRenderer,
}
impl ::core::marker::Copy for DYNAMIC_RENDERER_CACHED_DATA {}
impl ::core::clone::Clone for DYNAMIC_RENDERER_CACHED_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DynamicRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3973262058, data2: 29807, data3: 19915, data4: [191, 104, 8, 39, 87, 250, 255, 24] };
pub const EM_GETDRAWATTR: u32 = 1541u32;
pub const EM_GETFACTOID: u32 = 1549u32;
pub const EM_GETGESTURESTATUS: u32 = 1545u32;
pub const EM_GETINKINSERTMODE: u32 = 1539u32;
pub const EM_GETINKMODE: u32 = 1537u32;
pub const EM_GETMOUSEICON: u32 = 1553u32;
pub const EM_GETMOUSEPOINTER: u32 = 1555u32;
pub const EM_GETRECOGNIZER: u32 = 1547u32;
pub const EM_GETRECOTIMEOUT: u32 = 1543u32;
pub const EM_GETSELINK: u32 = 1551u32;
pub const EM_GETSELINKDISPLAYMODE: u32 = 1562u32;
pub const EM_GETSTATUS: u32 = 1557u32;
pub const EM_GETUSEMOUSEFORINPUT: u32 = 1559u32;
pub const EM_RECOGNIZE: u32 = 1558u32;
pub const EM_SETDRAWATTR: u32 = 1542u32;
pub const EM_SETFACTOID: u32 = 1550u32;
pub const EM_SETGESTURESTATUS: u32 = 1546u32;
pub const EM_SETINKINSERTMODE: u32 = 1540u32;
pub const EM_SETINKMODE: u32 = 1538u32;
pub const EM_SETMOUSEICON: u32 = 1554u32;
pub const EM_SETMOUSEPOINTER: u32 = 1556u32;
pub const EM_SETRECOGNIZER: u32 = 1548u32;
pub const EM_SETRECOTIMEOUT: u32 = 1544u32;
pub const EM_SETSELINK: u32 = 1552u32;
pub const EM_SETSELINKDISPLAYMODE: u32 = 1561u32;
pub const EM_SETUSEMOUSEFORINPUT: u32 = 1560u32;
#[repr(transparent)]
pub struct EventMask(pub i32);
pub const EventMask_InPlaceStateChanging: EventMask = EventMask(1i32);
pub const EventMask_InPlaceStateChanged: EventMask = EventMask(2i32);
pub const EventMask_InPlaceSizeChanging: EventMask = EventMask(4i32);
pub const EventMask_InPlaceSizeChanged: EventMask = EventMask(8i32);
pub const EventMask_InputAreaChanging: EventMask = EventMask(16i32);
pub const EventMask_InputAreaChanged: EventMask = EventMask(32i32);
pub const EventMask_CorrectionModeChanging: EventMask = EventMask(64i32);
pub const EventMask_CorrectionModeChanged: EventMask = EventMask(128i32);
pub const EventMask_InPlaceVisibilityChanging: EventMask = EventMask(256i32);
pub const EventMask_InPlaceVisibilityChanged: EventMask = EventMask(512i32);
pub const EventMask_TextInserting: EventMask = EventMask(1024i32);
pub const EventMask_TextInserted: EventMask = EventMask(2048i32);
pub const EventMask_All: EventMask = EventMask(4095i32);
impl ::core::marker::Copy for EventMask {}
impl ::core::clone::Clone for EventMask {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FACILITY_INK: u32 = 40u32;
#[repr(transparent)]
pub struct FLICKACTION_COMMANDCODE(pub i32);
pub const FLICKACTION_COMMANDCODE_NULL: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(0i32);
pub const FLICKACTION_COMMANDCODE_SCROLL: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(1i32);
pub const FLICKACTION_COMMANDCODE_APPCOMMAND: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(2i32);
pub const FLICKACTION_COMMANDCODE_CUSTOMKEY: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(3i32);
pub const FLICKACTION_COMMANDCODE_KEYMODIFIER: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(4i32);
impl ::core::marker::Copy for FLICKACTION_COMMANDCODE {}
impl ::core::clone::Clone for FLICKACTION_COMMANDCODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FLICKDIRECTION(pub i32);
pub const FLICKDIRECTION_MIN: FLICKDIRECTION = FLICKDIRECTION(0i32);
pub const FLICKDIRECTION_RIGHT: FLICKDIRECTION = FLICKDIRECTION(0i32);
pub const FLICKDIRECTION_UPRIGHT: FLICKDIRECTION = FLICKDIRECTION(1i32);
pub const FLICKDIRECTION_UP: FLICKDIRECTION = FLICKDIRECTION(2i32);
pub const FLICKDIRECTION_UPLEFT: FLICKDIRECTION = FLICKDIRECTION(3i32);
pub const FLICKDIRECTION_LEFT: FLICKDIRECTION = FLICKDIRECTION(4i32);
pub const FLICKDIRECTION_DOWNLEFT: FLICKDIRECTION = FLICKDIRECTION(5i32);
pub const FLICKDIRECTION_DOWN: FLICKDIRECTION = FLICKDIRECTION(6i32);
pub const FLICKDIRECTION_DOWNRIGHT: FLICKDIRECTION = FLICKDIRECTION(7i32);
pub const FLICKDIRECTION_INVALID: FLICKDIRECTION = FLICKDIRECTION(8i32);
impl ::core::marker::Copy for FLICKDIRECTION {}
impl ::core::clone::Clone for FLICKDIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct FLICKMODE(pub i32);
pub const FLICKMODE_MIN: FLICKMODE = FLICKMODE(0i32);
pub const FLICKMODE_OFF: FLICKMODE = FLICKMODE(0i32);
pub const FLICKMODE_ON: FLICKMODE = FLICKMODE(1i32);
pub const FLICKMODE_LEARNING: FLICKMODE = FLICKMODE(2i32);
pub const FLICKMODE_MAX: FLICKMODE = FLICKMODE(2i32);
pub const FLICKMODE_DEFAULT: FLICKMODE = FLICKMODE(1i32);
impl ::core::marker::Copy for FLICKMODE {}
impl ::core::clone::Clone for FLICKMODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct FLICK_DATA {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for FLICK_DATA {}
impl ::core::clone::Clone for FLICK_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct FLICK_POINT {
    pub _bitfield: i32,
}
impl ::core::marker::Copy for FLICK_POINT {}
impl ::core::clone::Clone for FLICK_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FLICK_WM_HANDLED_MASK: u32 = 1u32;
pub const GESTURE_ARROW_DOWN: u32 = 61497u32;
pub const GESTURE_ARROW_LEFT: u32 = 61498u32;
pub const GESTURE_ARROW_RIGHT: u32 = 61499u32;
pub const GESTURE_ARROW_UP: u32 = 61496u32;
pub const GESTURE_ASTERISK: u32 = 61608u32;
pub const GESTURE_BRACE_LEFT: u32 = 61674u32;
pub const GESTURE_BRACE_OVER: u32 = 61672u32;
pub const GESTURE_BRACE_RIGHT: u32 = 61675u32;
pub const GESTURE_BRACE_UNDER: u32 = 61673u32;
pub const GESTURE_BRACKET_LEFT: u32 = 61670u32;
pub const GESTURE_BRACKET_OVER: u32 = 61668u32;
pub const GESTURE_BRACKET_RIGHT: u32 = 61671u32;
pub const GESTURE_BRACKET_UNDER: u32 = 61669u32;
pub const GESTURE_BULLET: u32 = 61450u32;
pub const GESTURE_BULLET_CROSS: u32 = 61451u32;
pub const GESTURE_CHECK: u32 = 61445u32;
pub const GESTURE_CHEVRON_DOWN: u32 = 61489u32;
pub const GESTURE_CHEVRON_LEFT: u32 = 61490u32;
pub const GESTURE_CHEVRON_RIGHT: u32 = 61491u32;
pub const GESTURE_CHEVRON_UP: u32 = 61488u32;
pub const GESTURE_CIRCLE: u32 = 61472u32;
pub const GESTURE_CIRCLE_CIRCLE: u32 = 61475u32;
pub const GESTURE_CIRCLE_CROSS: u32 = 61477u32;
pub const GESTURE_CIRCLE_LINE_HORZ: u32 = 61479u32;
pub const GESTURE_CIRCLE_LINE_VERT: u32 = 61478u32;
pub const GESTURE_CIRCLE_TAP: u32 = 61474u32;
pub const GESTURE_CLOSEUP: u32 = 61455u32;
pub const GESTURE_CROSS: u32 = 61447u32;
pub const GESTURE_CURLICUE: u32 = 61456u32;
#[repr(C)]
pub struct GESTURE_DATA {
    pub gestureId: i32,
    pub recoConfidence: i32,
    pub strokeCount: i32,
}
impl ::core::marker::Copy for GESTURE_DATA {}
impl ::core::clone::Clone for GESTURE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
pub const GESTURE_DIAGONAL_LEFTDOWN: u32 = 61534u32;
pub const GESTURE_DIAGONAL_LEFTUP: u32 = 61532u32;
pub const GESTURE_DIAGONAL_RIGHTDOWN: u32 = 61535u32;
pub const GESTURE_DIAGONAL_RIGHTUP: u32 = 61533u32;
pub const GESTURE_DIGIT_0: u32 = 61594u32;
pub const GESTURE_DIGIT_1: u32 = 61595u32;
pub const GESTURE_DIGIT_2: u32 = 61596u32;
pub const GESTURE_DIGIT_3: u32 = 61597u32;
pub const GESTURE_DIGIT_4: u32 = 61598u32;
pub const GESTURE_DIGIT_5: u32 = 61599u32;
pub const GESTURE_DIGIT_6: u32 = 61600u32;
pub const GESTURE_DIGIT_7: u32 = 61601u32;
pub const GESTURE_DIGIT_8: u32 = 61602u32;
pub const GESTURE_DIGIT_9: u32 = 61603u32;
pub const GESTURE_DOLLAR: u32 = 61607u32;
pub const GESTURE_DOUBLE_ARROW_DOWN: u32 = 61501u32;
pub const GESTURE_DOUBLE_ARROW_LEFT: u32 = 61502u32;
pub const GESTURE_DOUBLE_ARROW_RIGHT: u32 = 61503u32;
pub const GESTURE_DOUBLE_ARROW_UP: u32 = 61500u32;
pub const GESTURE_DOUBLE_CIRCLE: u32 = 61473u32;
pub const GESTURE_DOUBLE_CURLICUE: u32 = 61457u32;
pub const GESTURE_DOUBLE_DOWN: u32 = 61625u32;
pub const GESTURE_DOUBLE_LEFT: u32 = 61626u32;
pub const GESTURE_DOUBLE_RIGHT: u32 = 61627u32;
pub const GESTURE_DOUBLE_TAP: u32 = 61681u32;
pub const GESTURE_DOUBLE_UP: u32 = 61624u32;
pub const GESTURE_DOWN: u32 = 61529u32;
pub const GESTURE_DOWN_ARROW_LEFT: u32 = 61506u32;
pub const GESTURE_DOWN_ARROW_RIGHT: u32 = 61507u32;
pub const GESTURE_DOWN_LEFT: u32 = 61546u32;
pub const GESTURE_DOWN_LEFT_LONG: u32 = 61542u32;
pub const GESTURE_DOWN_RIGHT: u32 = 61547u32;
pub const GESTURE_DOWN_RIGHT_LONG: u32 = 61543u32;
pub const GESTURE_DOWN_UP: u32 = 61537u32;
pub const GESTURE_EXCLAMATION: u32 = 61604u32;
pub const GESTURE_INFINITY: u32 = 61446u32;
pub const GESTURE_LEFT: u32 = 61530u32;
pub const GESTURE_LEFT_ARROW_DOWN: u32 = 61509u32;
pub const GESTURE_LEFT_ARROW_UP: u32 = 61508u32;
pub const GESTURE_LEFT_DOWN: u32 = 61549u32;
pub const GESTURE_LEFT_RIGHT: u32 = 61538u32;
pub const GESTURE_LEFT_UP: u32 = 61548u32;
pub const GESTURE_LETTER_A: u32 = 61568u32;
pub const GESTURE_LETTER_B: u32 = 61569u32;
pub const GESTURE_LETTER_C: u32 = 61570u32;
pub const GESTURE_LETTER_D: u32 = 61571u32;
pub const GESTURE_LETTER_E: u32 = 61572u32;
pub const GESTURE_LETTER_F: u32 = 61573u32;
pub const GESTURE_LETTER_G: u32 = 61574u32;
pub const GESTURE_LETTER_H: u32 = 61575u32;
pub const GESTURE_LETTER_I: u32 = 61576u32;
pub const GESTURE_LETTER_J: u32 = 61577u32;
pub const GESTURE_LETTER_K: u32 = 61578u32;
pub const GESTURE_LETTER_L: u32 = 61579u32;
pub const GESTURE_LETTER_M: u32 = 61580u32;
pub const GESTURE_LETTER_N: u32 = 61581u32;
pub const GESTURE_LETTER_O: u32 = 61582u32;
pub const GESTURE_LETTER_P: u32 = 61583u32;
pub const GESTURE_LETTER_Q: u32 = 61584u32;
pub const GESTURE_LETTER_R: u32 = 61585u32;
pub const GESTURE_LETTER_S: u32 = 61586u32;
pub const GESTURE_LETTER_T: u32 = 61587u32;
pub const GESTURE_LETTER_U: u32 = 61588u32;
pub const GESTURE_LETTER_V: u32 = 61589u32;
pub const GESTURE_LETTER_W: u32 = 61590u32;
pub const GESTURE_LETTER_X: u32 = 61591u32;
pub const GESTURE_LETTER_Y: u32 = 61592u32;
pub const GESTURE_LETTER_Z: u32 = 61593u32;
pub const GESTURE_NULL: u32 = 61440u32;
pub const GESTURE_OPENUP: u32 = 61454u32;
pub const GESTURE_PARAGRAPH: u32 = 61448u32;
pub const GESTURE_PLUS: u32 = 61609u32;
pub const GESTURE_QUAD_TAP: u32 = 61683u32;
pub const GESTURE_QUESTION: u32 = 61605u32;
pub const GESTURE_RECTANGLE: u32 = 61458u32;
pub const GESTURE_RIGHT: u32 = 61531u32;
pub const GESTURE_RIGHT_ARROW_DOWN: u32 = 61511u32;
pub const GESTURE_RIGHT_ARROW_UP: u32 = 61510u32;
pub const GESTURE_RIGHT_DOWN: u32 = 61551u32;
pub const GESTURE_RIGHT_LEFT: u32 = 61539u32;
pub const GESTURE_RIGHT_UP: u32 = 61550u32;
pub const GESTURE_SCRATCHOUT: u32 = 61441u32;
pub const GESTURE_SECTION: u32 = 61449u32;
pub const GESTURE_SEMICIRCLE_LEFT: u32 = 61480u32;
pub const GESTURE_SEMICIRCLE_RIGHT: u32 = 61481u32;
pub const GESTURE_SHARP: u32 = 61606u32;
pub const GESTURE_SQUARE: u32 = 61443u32;
pub const GESTURE_SQUIGGLE: u32 = 61452u32;
pub const GESTURE_STAR: u32 = 61444u32;
pub const GESTURE_SWAP: u32 = 61453u32;
pub const GESTURE_TAP: u32 = 61680u32;
pub const GESTURE_TRIANGLE: u32 = 61442u32;
pub const GESTURE_TRIPLE_DOWN: u32 = 61629u32;
pub const GESTURE_TRIPLE_LEFT: u32 = 61630u32;
pub const GESTURE_TRIPLE_RIGHT: u32 = 61631u32;
pub const GESTURE_TRIPLE_TAP: u32 = 61682u32;
pub const GESTURE_TRIPLE_UP: u32 = 61628u32;
pub const GESTURE_UP: u32 = 61528u32;
pub const GESTURE_UP_ARROW_LEFT: u32 = 61504u32;
pub const GESTURE_UP_ARROW_RIGHT: u32 = 61505u32;
pub const GESTURE_UP_DOWN: u32 = 61536u32;
pub const GESTURE_UP_LEFT: u32 = 61544u32;
pub const GESTURE_UP_LEFT_LONG: u32 = 61540u32;
pub const GESTURE_UP_RIGHT: u32 = 61545u32;
pub const GESTURE_UP_RIGHT_LONG: u32 = 61541u32;
pub const GUID_DYNAMIC_RENDERER_CACHED_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3209894802, data2: 9663, data3: 19093, data4: [137, 173, 14, 71, 107, 52, 180, 245] };
pub const GUID_GESTURE_DATA: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1105521679,
    data2: 9898,
    data3: 17754,
    data4: [154, 165, 44, 211, 108, 246, 63, 185],
};
pub const GUID_PACKETPROPERTY_GUID_ALTITUDE_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2195637703,
    data2: 63162,
    data3: 18694,
    data4: [137, 79, 102, 214, 141, 252, 69, 108],
};
pub const GUID_PACKETPROPERTY_GUID_AZIMUTH_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 43066292, data2: 34856, data3: 16651, data4: [178, 80, 160, 83, 101, 149, 229, 220] };
pub const GUID_PACKETPROPERTY_GUID_BUTTON_PRESSURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2340417476,
    data2: 38570,
    data3: 19454,
    data4: [172, 38, 138, 95, 11, 224, 123, 245],
};
pub const GUID_PACKETPROPERTY_GUID_DEVICE_CONTACT_ID: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 39345041, data2: 1179, data3: 18256, data4: [150, 21, 223, 137, 72, 171, 60, 156] };
pub const GUID_PACKETPROPERTY_GUID_FINGERCONTACTCONFIDENCE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3875981316,
    data2: 22512,
    data3: 20224,
    data4: [138, 12, 133, 61, 87, 120, 155, 233],
};
pub const GUID_PACKETPROPERTY_GUID_HEIGHT: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3860355282, data2: 58439, data3: 16920, data4: [157, 63, 24, 134, 92, 32, 61, 244] };
pub const GUID_PACKETPROPERTY_GUID_NORMAL_PRESSURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1929859117,
    data2: 63988,
    data3: 19992,
    data4: [179, 242, 44, 225, 177, 163, 97, 12],
};
pub const GUID_PACKETPROPERTY_GUID_PACKET_STATUS: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1846413247,
    data2: 45031,
    data3: 19703,
    data4: [135, 209, 175, 100, 70, 32, 132, 24],
};
pub const GUID_PACKETPROPERTY_GUID_PITCH_ROTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2138986423, data2: 48695, data3: 19425, data4: [163, 86, 122, 132, 22, 14, 24, 147] };
pub const GUID_PACKETPROPERTY_GUID_ROLL_ROTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1566400086,
    data2: 27561,
    data3: 19547,
    data4: [159, 176, 133, 28, 145, 113, 78, 86],
};
pub const GUID_PACKETPROPERTY_GUID_SERIAL_NUMBER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2024282966, data2: 2357, data3: 17555, data4: [186, 174, 0, 84, 26, 138, 22, 196] };
pub const GUID_PACKETPROPERTY_GUID_TANGENT_PRESSURE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1839483019, data2: 21060, data3: 16876, data4: [144, 91, 50, 216, 154, 184, 8, 9] };
pub const GUID_PACKETPROPERTY_GUID_TIMER_TICK: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1130696901,
    data2: 65235,
    data3: 17873,
    data4: [139, 118, 113, 211, 234, 122, 130, 157],
};
pub const GUID_PACKETPROPERTY_GUID_TWIST_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 221399392, data2: 5042, data3: 16868, data4: [172, 230, 122, 233, 212, 61, 45, 59] };
pub const GUID_PACKETPROPERTY_GUID_WIDTH: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3131828557,
    data2: 10002,
    data3: 18677,
    data4: [190, 157, 143, 139, 94, 160, 113, 26],
};
pub const GUID_PACKETPROPERTY_GUID_X: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1502243471,
    data2: 21184,
    data3: 19360,
    data4: [147, 175, 175, 53, 116, 17, 165, 97],
};
pub const GUID_PACKETPROPERTY_GUID_X_TILT_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2832235322,
    data2: 35824,
    data3: 16560,
    data4: [149, 169, 184, 10, 107, 183, 135, 191],
};
pub const GUID_PACKETPROPERTY_GUID_Y: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3040845685, data2: 1248, data3: 17560, data4: [167, 238, 195, 13, 187, 90, 144, 17] };
pub const GUID_PACKETPROPERTY_GUID_YAW_ROTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1787074944,
    data2: 31802,
    data3: 17847,
    data4: [170, 130, 144, 162, 98, 149, 14, 137],
};
pub const GUID_PACKETPROPERTY_GUID_Y_TILT_ORIENTATION: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 244523913, data2: 7543, data3: 17327, data4: [172, 0, 91, 149, 13, 109, 75, 45] };
pub const GUID_PACKETPROPERTY_GUID_Z: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1935334192, data2: 3771, data3: 18312, data4: [160, 228, 15, 49, 100, 144, 5, 93] };
pub const GestureRecognizer: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3929065044,
    data2: 50732,
    data3: 17439,
    data4: [172, 0, 149, 249, 161, 150, 120, 44],
};
#[repr(C)]
pub struct HRECOALT(pub isize);
impl ::core::marker::Copy for HRECOALT {}
impl ::core::clone::Clone for HRECOALT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HRECOCONTEXT(pub isize);
impl ::core::marker::Copy for HRECOCONTEXT {}
impl ::core::clone::Clone for HRECOCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HRECOGNIZER(pub isize);
impl ::core::marker::Copy for HRECOGNIZER {}
impl ::core::clone::Clone for HRECOGNIZER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HRECOLATTICE(pub isize);
impl ::core::marker::Copy for HRECOLATTICE {}
impl ::core::clone::Clone for HRECOLATTICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HRECOWORDLIST(pub isize);
impl ::core::marker::Copy for HRECOWORDLIST {}
impl ::core::clone::Clone for HRECOWORDLIST {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HandwrittenTextInsertion: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2668056290, data2: 59113, data3: 19850, data4: [160, 71, 235, 91, 92, 60, 85, 218] };
#[repr(transparent)]
pub struct IDynamicRenderer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDynamicRenderer {}
impl ::core::clone::Clone for IDynamicRenderer {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IECN_GESTURE: u32 = 2050u32;
pub const IECN_RECOGNITIONRESULT: u32 = 2051u32;
pub const IECN_STROKE: u32 = 2049u32;
pub const IECN__BASE: u32 = 2048u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Controls"))]
pub struct IEC_GESTUREINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: IInkCursor,
    pub Strokes: IInkStrokes,
    pub Gestures: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for IEC_GESTUREINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for IEC_GESTUREINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct IEC_RECOGNITIONRESULTINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub RecognitionResult: IInkRecognitionResult,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for IEC_RECOGNITIONRESULTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for IEC_RECOGNITIONRESULTINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
pub struct IEC_STROKEINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: IInkCursor,
    pub Stroke: IInkStrokeDisp,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::marker::Copy for IEC_STROKEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::core::clone::Clone for IEC_STROKEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IEC__BASE: u32 = 1536u32;
#[repr(transparent)]
pub struct IGestureRecognizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGestureRecognizer {}
impl ::core::clone::Clone for IGestureRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHandwrittenTextInsertion(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHandwrittenTextInsertion {}
impl ::core::clone::Clone for IHandwrittenTextInsertion {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInk(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInk {}
impl ::core::clone::Clone for IInk {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkCollector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkCollector {}
impl ::core::clone::Clone for IInkCollector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkCursor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkCursor {}
impl ::core::clone::Clone for IInkCursor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkCursorButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkCursorButton {}
impl ::core::clone::Clone for IInkCursorButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkCursorButtons(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkCursorButtons {}
impl ::core::clone::Clone for IInkCursorButtons {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkCursors(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkCursors {}
impl ::core::clone::Clone for IInkCursors {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkCustomStrokes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkCustomStrokes {}
impl ::core::clone::Clone for IInkCustomStrokes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDisp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDisp {}
impl ::core::clone::Clone for IInkDisp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDivider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDivider {}
impl ::core::clone::Clone for IInkDivider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDivisionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDivisionResult {}
impl ::core::clone::Clone for IInkDivisionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDivisionUnit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDivisionUnit {}
impl ::core::clone::Clone for IInkDivisionUnit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDivisionUnits(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDivisionUnits {}
impl ::core::clone::Clone for IInkDivisionUnits {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkDrawingAttributes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkDrawingAttributes {}
impl ::core::clone::Clone for IInkDrawingAttributes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkEdit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkEdit {}
impl ::core::clone::Clone for IInkEdit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkExtendedProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkExtendedProperties {}
impl ::core::clone::Clone for IInkExtendedProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkExtendedProperty(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkExtendedProperty {}
impl ::core::clone::Clone for IInkExtendedProperty {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkGesture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkGesture {}
impl ::core::clone::Clone for IInkGesture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkLineInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkLineInfo {}
impl ::core::clone::Clone for IInkLineInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkOverlay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkOverlay {}
impl ::core::clone::Clone for IInkOverlay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkPicture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkPicture {}
impl ::core::clone::Clone for IInkPicture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognitionAlternate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognitionAlternate {}
impl ::core::clone::Clone for IInkRecognitionAlternate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognitionAlternates(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognitionAlternates {}
impl ::core::clone::Clone for IInkRecognitionAlternates {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognitionResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognitionResult {}
impl ::core::clone::Clone for IInkRecognitionResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognizer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognizer {}
impl ::core::clone::Clone for IInkRecognizer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognizer2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognizer2 {}
impl ::core::clone::Clone for IInkRecognizer2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognizerContext(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognizerContext {}
impl ::core::clone::Clone for IInkRecognizerContext {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognizerContext2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognizerContext2 {}
impl ::core::clone::Clone for IInkRecognizerContext2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognizerGuide(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognizerGuide {}
impl ::core::clone::Clone for IInkRecognizerGuide {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRecognizers(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRecognizers {}
impl ::core::clone::Clone for IInkRecognizers {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRectangle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRectangle {}
impl ::core::clone::Clone for IInkRectangle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkRenderer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkRenderer {}
impl ::core::clone::Clone for IInkRenderer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokeDisp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokeDisp {}
impl ::core::clone::Clone for IInkStrokeDisp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkStrokes(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkStrokes {}
impl ::core::clone::Clone for IInkStrokes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkTablet(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkTablet {}
impl ::core::clone::Clone for IInkTablet {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkTablet2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkTablet2 {}
impl ::core::clone::Clone for IInkTablet2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkTablet3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkTablet3 {}
impl ::core::clone::Clone for IInkTablet3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkTablets(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkTablets {}
impl ::core::clone::Clone for IInkTablets {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkTransform(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkTransform {}
impl ::core::clone::Clone for IInkTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkWordList(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkWordList {}
impl ::core::clone::Clone for IInkWordList {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkWordList2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkWordList2 {}
impl ::core::clone::Clone for IInkWordList2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInputPanelWindowHandle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInputPanelWindowHandle {}
impl ::core::clone::Clone for IInputPanelWindowHandle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMathInputControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMathInputControl {}
impl ::core::clone::Clone for IMathInputControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct INKMETRIC {
    pub iHeight: i32,
    pub iFontAscent: i32,
    pub iFontDescent: i32,
    pub dwFlags: u32,
    pub color: u32,
}
impl ::core::marker::Copy for INKMETRIC {}
impl ::core::clone::Clone for INKMETRIC {
    fn clone(&self) -> Self {
        *self
    }
}
pub const IP_CURSOR_DOWN: u32 = 1u32;
pub const IP_INVERTED: u32 = 2u32;
pub const IP_MARGIN: u32 = 4u32;
#[repr(transparent)]
pub struct IPenInputPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPenInputPanel {}
impl ::core::clone::Clone for IPenInputPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRealTimeStylus(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRealTimeStylus {}
impl ::core::clone::Clone for IRealTimeStylus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRealTimeStylus2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRealTimeStylus2 {}
impl ::core::clone::Clone for IRealTimeStylus2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRealTimeStylus3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRealTimeStylus3 {}
impl ::core::clone::Clone for IRealTimeStylus3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRealTimeStylusSynchronization(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRealTimeStylusSynchronization {}
impl ::core::clone::Clone for IRealTimeStylusSynchronization {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISketchInk(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISketchInk {}
impl ::core::clone::Clone for ISketchInk {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStrokeBuilder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStrokeBuilder {}
impl ::core::clone::Clone for IStrokeBuilder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStylusAsyncPlugin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStylusAsyncPlugin {}
impl ::core::clone::Clone for IStylusAsyncPlugin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStylusPlugin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStylusPlugin {}
impl ::core::clone::Clone for IStylusPlugin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStylusSyncPlugin(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStylusSyncPlugin {}
impl ::core::clone::Clone for IStylusSyncPlugin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextInputPanel(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextInputPanel {}
impl ::core::clone::Clone for ITextInputPanel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextInputPanelEventSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextInputPanelEventSink {}
impl ::core::clone::Clone for ITextInputPanelEventSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITextInputPanelRunInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITextInputPanelRunInfo {}
impl ::core::clone::Clone for ITextInputPanelRunInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITipAutoCompleteClient(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITipAutoCompleteClient {}
impl ::core::clone::Clone for ITipAutoCompleteClient {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITipAutoCompleteProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITipAutoCompleteProvider {}
impl ::core::clone::Clone for ITipAutoCompleteProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InPlaceDirection(pub i32);
pub const InPlaceDirection_Auto: InPlaceDirection = InPlaceDirection(0i32);
pub const InPlaceDirection_Bottom: InPlaceDirection = InPlaceDirection(1i32);
pub const InPlaceDirection_Top: InPlaceDirection = InPlaceDirection(2i32);
impl ::core::marker::Copy for InPlaceDirection {}
impl ::core::clone::Clone for InPlaceDirection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InPlaceState(pub i32);
pub const InPlaceState_Auto: InPlaceState = InPlaceState(0i32);
pub const InPlaceState_HoverTarget: InPlaceState = InPlaceState(1i32);
pub const InPlaceState_Expanded: InPlaceState = InPlaceState(2i32);
impl ::core::marker::Copy for InPlaceState {}
impl ::core::clone::Clone for InPlaceState {
    fn clone(&self) -> Self {
        *self
    }
}
pub const Ink: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 333335106,
    data2: 36129,
    data3: 19598,
    data4: [191, 156, 143, 105, 203, 6, 143, 202],
};
#[repr(transparent)]
pub struct InkApplicationGesture(pub i32);
pub const IAG_AllGestures: InkApplicationGesture = InkApplicationGesture(0i32);
pub const IAG_NoGesture: InkApplicationGesture = InkApplicationGesture(61440i32);
pub const IAG_Scratchout: InkApplicationGesture = InkApplicationGesture(61441i32);
pub const IAG_Triangle: InkApplicationGesture = InkApplicationGesture(61442i32);
pub const IAG_Square: InkApplicationGesture = InkApplicationGesture(61443i32);
pub const IAG_Star: InkApplicationGesture = InkApplicationGesture(61444i32);
pub const IAG_Check: InkApplicationGesture = InkApplicationGesture(61445i32);
pub const IAG_Curlicue: InkApplicationGesture = InkApplicationGesture(61456i32);
pub const IAG_DoubleCurlicue: InkApplicationGesture = InkApplicationGesture(61457i32);
pub const IAG_Circle: InkApplicationGesture = InkApplicationGesture(61472i32);
pub const IAG_DoubleCircle: InkApplicationGesture = InkApplicationGesture(61473i32);
pub const IAG_SemiCircleLeft: InkApplicationGesture = InkApplicationGesture(61480i32);
pub const IAG_SemiCircleRight: InkApplicationGesture = InkApplicationGesture(61481i32);
pub const IAG_ChevronUp: InkApplicationGesture = InkApplicationGesture(61488i32);
pub const IAG_ChevronDown: InkApplicationGesture = InkApplicationGesture(61489i32);
pub const IAG_ChevronLeft: InkApplicationGesture = InkApplicationGesture(61490i32);
pub const IAG_ChevronRight: InkApplicationGesture = InkApplicationGesture(61491i32);
pub const IAG_ArrowUp: InkApplicationGesture = InkApplicationGesture(61496i32);
pub const IAG_ArrowDown: InkApplicationGesture = InkApplicationGesture(61497i32);
pub const IAG_ArrowLeft: InkApplicationGesture = InkApplicationGesture(61498i32);
pub const IAG_ArrowRight: InkApplicationGesture = InkApplicationGesture(61499i32);
pub const IAG_Up: InkApplicationGesture = InkApplicationGesture(61528i32);
pub const IAG_Down: InkApplicationGesture = InkApplicationGesture(61529i32);
pub const IAG_Left: InkApplicationGesture = InkApplicationGesture(61530i32);
pub const IAG_Right: InkApplicationGesture = InkApplicationGesture(61531i32);
pub const IAG_UpDown: InkApplicationGesture = InkApplicationGesture(61536i32);
pub const IAG_DownUp: InkApplicationGesture = InkApplicationGesture(61537i32);
pub const IAG_LeftRight: InkApplicationGesture = InkApplicationGesture(61538i32);
pub const IAG_RightLeft: InkApplicationGesture = InkApplicationGesture(61539i32);
pub const IAG_UpLeftLong: InkApplicationGesture = InkApplicationGesture(61540i32);
pub const IAG_UpRightLong: InkApplicationGesture = InkApplicationGesture(61541i32);
pub const IAG_DownLeftLong: InkApplicationGesture = InkApplicationGesture(61542i32);
pub const IAG_DownRightLong: InkApplicationGesture = InkApplicationGesture(61543i32);
pub const IAG_UpLeft: InkApplicationGesture = InkApplicationGesture(61544i32);
pub const IAG_UpRight: InkApplicationGesture = InkApplicationGesture(61545i32);
pub const IAG_DownLeft: InkApplicationGesture = InkApplicationGesture(61546i32);
pub const IAG_DownRight: InkApplicationGesture = InkApplicationGesture(61547i32);
pub const IAG_LeftUp: InkApplicationGesture = InkApplicationGesture(61548i32);
pub const IAG_LeftDown: InkApplicationGesture = InkApplicationGesture(61549i32);
pub const IAG_RightUp: InkApplicationGesture = InkApplicationGesture(61550i32);
pub const IAG_RightDown: InkApplicationGesture = InkApplicationGesture(61551i32);
pub const IAG_Exclamation: InkApplicationGesture = InkApplicationGesture(61604i32);
pub const IAG_Tap: InkApplicationGesture = InkApplicationGesture(61680i32);
pub const IAG_DoubleTap: InkApplicationGesture = InkApplicationGesture(61681i32);
impl ::core::marker::Copy for InkApplicationGesture {}
impl ::core::clone::Clone for InkApplicationGesture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkBoundingBoxMode(pub i32);
pub const IBBM_Default: InkBoundingBoxMode = InkBoundingBoxMode(0i32);
pub const IBBM_NoCurveFit: InkBoundingBoxMode = InkBoundingBoxMode(1i32);
pub const IBBM_CurveFit: InkBoundingBoxMode = InkBoundingBoxMode(2i32);
pub const IBBM_PointsOnly: InkBoundingBoxMode = InkBoundingBoxMode(3i32);
pub const IBBM_Union: InkBoundingBoxMode = InkBoundingBoxMode(4i32);
impl ::core::marker::Copy for InkBoundingBoxMode {}
impl ::core::clone::Clone for InkBoundingBoxMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkClipboardFormats(pub i32);
pub const ICF_None: InkClipboardFormats = InkClipboardFormats(0i32);
pub const ICF_InkSerializedFormat: InkClipboardFormats = InkClipboardFormats(1i32);
pub const ICF_SketchInk: InkClipboardFormats = InkClipboardFormats(2i32);
pub const ICF_TextInk: InkClipboardFormats = InkClipboardFormats(6i32);
pub const ICF_EnhancedMetafile: InkClipboardFormats = InkClipboardFormats(8i32);
pub const ICF_Metafile: InkClipboardFormats = InkClipboardFormats(32i32);
pub const ICF_Bitmap: InkClipboardFormats = InkClipboardFormats(64i32);
pub const ICF_PasteMask: InkClipboardFormats = InkClipboardFormats(7i32);
pub const ICF_CopyMask: InkClipboardFormats = InkClipboardFormats(127i32);
pub const ICF_Default: InkClipboardFormats = InkClipboardFormats(127i32);
impl ::core::marker::Copy for InkClipboardFormats {}
impl ::core::clone::Clone for InkClipboardFormats {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkClipboardModes(pub i32);
pub const ICB_Copy: InkClipboardModes = InkClipboardModes(0i32);
pub const ICB_Cut: InkClipboardModes = InkClipboardModes(1i32);
pub const ICB_ExtractOnly: InkClipboardModes = InkClipboardModes(48i32);
pub const ICB_DelayedCopy: InkClipboardModes = InkClipboardModes(32i32);
pub const ICB_Default: InkClipboardModes = InkClipboardModes(0i32);
impl ::core::marker::Copy for InkClipboardModes {}
impl ::core::clone::Clone for InkClipboardModes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkCollectionMode(pub i32);
pub const ICM_InkOnly: InkCollectionMode = InkCollectionMode(0i32);
pub const ICM_GestureOnly: InkCollectionMode = InkCollectionMode(1i32);
pub const ICM_InkAndGesture: InkCollectionMode = InkCollectionMode(2i32);
impl ::core::marker::Copy for InkCollectionMode {}
impl ::core::clone::Clone for InkCollectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkCollector: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1140528467,
    data2: 44404,
    data3: 20200,
    data4: [136, 228, 62, 109, 170, 201, 21, 219],
};
pub const InkCollectorClipInkToMargin: i32 = 0i32;
pub const InkCollectorDefaultMargin: i32 = -2147483648i32;
#[repr(transparent)]
pub struct InkCollectorEventInterest(pub i32);
pub const ICEI_DefaultEvents: InkCollectorEventInterest = InkCollectorEventInterest(-1i32);
pub const ICEI_CursorDown: InkCollectorEventInterest = InkCollectorEventInterest(0i32);
pub const ICEI_Stroke: InkCollectorEventInterest = InkCollectorEventInterest(1i32);
pub const ICEI_NewPackets: InkCollectorEventInterest = InkCollectorEventInterest(2i32);
pub const ICEI_NewInAirPackets: InkCollectorEventInterest = InkCollectorEventInterest(3i32);
pub const ICEI_CursorButtonDown: InkCollectorEventInterest = InkCollectorEventInterest(4i32);
pub const ICEI_CursorButtonUp: InkCollectorEventInterest = InkCollectorEventInterest(5i32);
pub const ICEI_CursorInRange: InkCollectorEventInterest = InkCollectorEventInterest(6i32);
pub const ICEI_CursorOutOfRange: InkCollectorEventInterest = InkCollectorEventInterest(7i32);
pub const ICEI_SystemGesture: InkCollectorEventInterest = InkCollectorEventInterest(8i32);
pub const ICEI_TabletAdded: InkCollectorEventInterest = InkCollectorEventInterest(9i32);
pub const ICEI_TabletRemoved: InkCollectorEventInterest = InkCollectorEventInterest(10i32);
pub const ICEI_MouseDown: InkCollectorEventInterest = InkCollectorEventInterest(11i32);
pub const ICEI_MouseMove: InkCollectorEventInterest = InkCollectorEventInterest(12i32);
pub const ICEI_MouseUp: InkCollectorEventInterest = InkCollectorEventInterest(13i32);
pub const ICEI_MouseWheel: InkCollectorEventInterest = InkCollectorEventInterest(14i32);
pub const ICEI_DblClick: InkCollectorEventInterest = InkCollectorEventInterest(15i32);
pub const ICEI_AllEvents: InkCollectorEventInterest = InkCollectorEventInterest(16i32);
impl ::core::marker::Copy for InkCollectorEventInterest {}
impl ::core::clone::Clone for InkCollectorEventInterest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkCursorButtonState(pub i32);
pub const ICBS_Unavailable: InkCursorButtonState = InkCursorButtonState(0i32);
pub const ICBS_Up: InkCursorButtonState = InkCursorButtonState(1i32);
pub const ICBS_Down: InkCursorButtonState = InkCursorButtonState(2i32);
impl ::core::marker::Copy for InkCursorButtonState {}
impl ::core::clone::Clone for InkCursorButtonState {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkDisp: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2474383924,
    data2: 5405,
    data3: 17936,
    data4: [156, 166, 168, 204, 155, 219, 93, 131],
};
#[repr(transparent)]
pub struct InkDisplayMode(pub i32);
pub const IDM_Ink: InkDisplayMode = InkDisplayMode(0i32);
pub const IDM_Text: InkDisplayMode = InkDisplayMode(1i32);
impl ::core::marker::Copy for InkDisplayMode {}
impl ::core::clone::Clone for InkDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkDivider: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2287269536,
    data2: 18051,
    data3: 19175,
    data4: [145, 145, 117, 47, 230, 70, 18, 195],
};
#[repr(transparent)]
pub struct InkDivisionType(pub i32);
pub const IDT_Segment: InkDivisionType = InkDivisionType(0i32);
pub const IDT_Line: InkDivisionType = InkDivisionType(1i32);
pub const IDT_Paragraph: InkDivisionType = InkDivisionType(2i32);
pub const IDT_Drawing: InkDivisionType = InkDivisionType(3i32);
impl ::core::marker::Copy for InkDivisionType {}
impl ::core::clone::Clone for InkDivisionType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkDrawingAttributes: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3636408994,
    data2: 1445,
    data3: 17603,
    data4: [179, 170, 94, 128, 172, 125, 37, 118],
};
pub const InkEdit: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3855243765,
    data2: 22468,
    data3: 19928,
    data4: [155, 214, 29, 238, 237, 210, 122, 244],
};
#[repr(transparent)]
pub struct InkEditStatus(pub i32);
pub const IES_Idle: InkEditStatus = InkEditStatus(0i32);
pub const IES_Collecting: InkEditStatus = InkEditStatus(1i32);
pub const IES_Recognizing: InkEditStatus = InkEditStatus(2i32);
impl ::core::marker::Copy for InkEditStatus {}
impl ::core::clone::Clone for InkEditStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkExtractFlags(pub i32);
pub const IEF_CopyFromOriginal: InkExtractFlags = InkExtractFlags(0i32);
pub const IEF_RemoveFromOriginal: InkExtractFlags = InkExtractFlags(1i32);
pub const IEF_Default: InkExtractFlags = InkExtractFlags(1i32);
impl ::core::marker::Copy for InkExtractFlags {}
impl ::core::clone::Clone for InkExtractFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkInsertMode(pub i32);
pub const IEM_InsertText: InkInsertMode = InkInsertMode(0i32);
pub const IEM_InsertInk: InkInsertMode = InkInsertMode(1i32);
impl ::core::marker::Copy for InkInsertMode {}
impl ::core::clone::Clone for InkInsertMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkMaxTransparencyValue: i32 = 255i32;
pub const InkMinTransparencyValue: i32 = 0i32;
#[repr(transparent)]
pub struct InkMode(pub i32);
pub const IEM_Disabled: InkMode = InkMode(0i32);
pub const IEM_Ink: InkMode = InkMode(1i32);
pub const IEM_InkAndGesture: InkMode = InkMode(2i32);
impl ::core::marker::Copy for InkMode {}
impl ::core::clone::Clone for InkMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkMouseButton(pub i32);
pub const IMF_Left: InkMouseButton = InkMouseButton(1i32);
pub const IMF_Right: InkMouseButton = InkMouseButton(2i32);
pub const IMF_Middle: InkMouseButton = InkMouseButton(4i32);
impl ::core::marker::Copy for InkMouseButton {}
impl ::core::clone::Clone for InkMouseButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkMousePointer(pub i32);
pub const IMP_Default: InkMousePointer = InkMousePointer(0i32);
pub const IMP_Arrow: InkMousePointer = InkMousePointer(1i32);
pub const IMP_Crosshair: InkMousePointer = InkMousePointer(2i32);
pub const IMP_Ibeam: InkMousePointer = InkMousePointer(3i32);
pub const IMP_SizeNESW: InkMousePointer = InkMousePointer(4i32);
pub const IMP_SizeNS: InkMousePointer = InkMousePointer(5i32);
pub const IMP_SizeNWSE: InkMousePointer = InkMousePointer(6i32);
pub const IMP_SizeWE: InkMousePointer = InkMousePointer(7i32);
pub const IMP_UpArrow: InkMousePointer = InkMousePointer(8i32);
pub const IMP_Hourglass: InkMousePointer = InkMousePointer(9i32);
pub const IMP_NoDrop: InkMousePointer = InkMousePointer(10i32);
pub const IMP_ArrowHourglass: InkMousePointer = InkMousePointer(11i32);
pub const IMP_ArrowQuestion: InkMousePointer = InkMousePointer(12i32);
pub const IMP_SizeAll: InkMousePointer = InkMousePointer(13i32);
pub const IMP_Hand: InkMousePointer = InkMousePointer(14i32);
pub const IMP_Custom: InkMousePointer = InkMousePointer(99i32);
impl ::core::marker::Copy for InkMousePointer {}
impl ::core::clone::Clone for InkMousePointer {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkOverlay: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1708131910,
    data2: 52707,
    data3: 19080,
    data4: [145, 99, 103, 105, 240, 241, 169, 125],
};
#[repr(transparent)]
pub struct InkOverlayAttachMode(pub i32);
pub const IOAM_Behind: InkOverlayAttachMode = InkOverlayAttachMode(0i32);
pub const IOAM_InFront: InkOverlayAttachMode = InkOverlayAttachMode(1i32);
impl ::core::marker::Copy for InkOverlayAttachMode {}
impl ::core::clone::Clone for InkOverlayAttachMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkOverlayEditingMode(pub i32);
pub const IOEM_Ink: InkOverlayEditingMode = InkOverlayEditingMode(0i32);
pub const IOEM_Delete: InkOverlayEditingMode = InkOverlayEditingMode(1i32);
pub const IOEM_Select: InkOverlayEditingMode = InkOverlayEditingMode(2i32);
impl ::core::marker::Copy for InkOverlayEditingMode {}
impl ::core::clone::Clone for InkOverlayEditingMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkOverlayEraserMode(pub i32);
pub const IOERM_StrokeErase: InkOverlayEraserMode = InkOverlayEraserMode(0i32);
pub const IOERM_PointErase: InkOverlayEraserMode = InkOverlayEraserMode(1i32);
impl ::core::marker::Copy for InkOverlayEraserMode {}
impl ::core::clone::Clone for InkOverlayEraserMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPenTip(pub i32);
pub const IPT_Ball: InkPenTip = InkPenTip(0i32);
pub const IPT_Rectangle: InkPenTip = InkPenTip(1i32);
impl ::core::marker::Copy for InkPenTip {}
impl ::core::clone::Clone for InkPenTip {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPersistenceCompressionMode(pub i32);
pub const IPCM_Default: InkPersistenceCompressionMode = InkPersistenceCompressionMode(0i32);
pub const IPCM_MaximumCompression: InkPersistenceCompressionMode = InkPersistenceCompressionMode(1i32);
pub const IPCM_NoCompression: InkPersistenceCompressionMode = InkPersistenceCompressionMode(2i32);
impl ::core::marker::Copy for InkPersistenceCompressionMode {}
impl ::core::clone::Clone for InkPersistenceCompressionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkPersistenceFormat(pub i32);
pub const IPF_InkSerializedFormat: InkPersistenceFormat = InkPersistenceFormat(0i32);
pub const IPF_Base64InkSerializedFormat: InkPersistenceFormat = InkPersistenceFormat(1i32);
pub const IPF_GIF: InkPersistenceFormat = InkPersistenceFormat(2i32);
pub const IPF_Base64GIF: InkPersistenceFormat = InkPersistenceFormat(3i32);
impl ::core::marker::Copy for InkPersistenceFormat {}
impl ::core::clone::Clone for InkPersistenceFormat {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkPicture: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 77718867, data2: 65078, data3: 20446, data4: [134, 94, 52, 65, 148, 230, 148, 36] };
#[repr(transparent)]
pub struct InkPictureSizeMode(pub i32);
pub const IPSM_AutoSize: InkPictureSizeMode = InkPictureSizeMode(0i32);
pub const IPSM_CenterImage: InkPictureSizeMode = InkPictureSizeMode(1i32);
pub const IPSM_Normal: InkPictureSizeMode = InkPictureSizeMode(2i32);
pub const IPSM_StretchImage: InkPictureSizeMode = InkPictureSizeMode(3i32);
impl ::core::marker::Copy for InkPictureSizeMode {}
impl ::core::clone::Clone for InkPictureSizeMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRasterOperation(pub i32);
pub const IRO_Black: InkRasterOperation = InkRasterOperation(1i32);
pub const IRO_NotMergePen: InkRasterOperation = InkRasterOperation(2i32);
pub const IRO_MaskNotPen: InkRasterOperation = InkRasterOperation(3i32);
pub const IRO_NotCopyPen: InkRasterOperation = InkRasterOperation(4i32);
pub const IRO_MaskPenNot: InkRasterOperation = InkRasterOperation(5i32);
pub const IRO_Not: InkRasterOperation = InkRasterOperation(6i32);
pub const IRO_XOrPen: InkRasterOperation = InkRasterOperation(7i32);
pub const IRO_NotMaskPen: InkRasterOperation = InkRasterOperation(8i32);
pub const IRO_MaskPen: InkRasterOperation = InkRasterOperation(9i32);
pub const IRO_NotXOrPen: InkRasterOperation = InkRasterOperation(10i32);
pub const IRO_NoOperation: InkRasterOperation = InkRasterOperation(11i32);
pub const IRO_MergeNotPen: InkRasterOperation = InkRasterOperation(12i32);
pub const IRO_CopyPen: InkRasterOperation = InkRasterOperation(13i32);
pub const IRO_MergePenNot: InkRasterOperation = InkRasterOperation(14i32);
pub const IRO_MergePen: InkRasterOperation = InkRasterOperation(15i32);
pub const IRO_White: InkRasterOperation = InkRasterOperation(16i32);
impl ::core::marker::Copy for InkRasterOperation {}
impl ::core::clone::Clone for InkRasterOperation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct InkRecoGuide {
    pub rectWritingBox: super::super::Foundation::RECT,
    pub rectDrawnBox: super::super::Foundation::RECT,
    pub cRows: i32,
    pub cColumns: i32,
    pub midline: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for InkRecoGuide {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for InkRecoGuide {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognitionAlternatesSelection(pub i32);
pub const IRAS_Start: InkRecognitionAlternatesSelection = InkRecognitionAlternatesSelection(0i32);
pub const IRAS_DefaultCount: InkRecognitionAlternatesSelection = InkRecognitionAlternatesSelection(10i32);
pub const IRAS_All: InkRecognitionAlternatesSelection = InkRecognitionAlternatesSelection(-1i32);
impl ::core::marker::Copy for InkRecognitionAlternatesSelection {}
impl ::core::clone::Clone for InkRecognitionAlternatesSelection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognitionConfidence(pub i32);
pub const IRC_Strong: InkRecognitionConfidence = InkRecognitionConfidence(0i32);
pub const IRC_Intermediate: InkRecognitionConfidence = InkRecognitionConfidence(1i32);
pub const IRC_Poor: InkRecognitionConfidence = InkRecognitionConfidence(2i32);
impl ::core::marker::Copy for InkRecognitionConfidence {}
impl ::core::clone::Clone for InkRecognitionConfidence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognitionModes(pub i32);
pub const IRM_None: InkRecognitionModes = InkRecognitionModes(0i32);
pub const IRM_WordModeOnly: InkRecognitionModes = InkRecognitionModes(1i32);
pub const IRM_Coerce: InkRecognitionModes = InkRecognitionModes(2i32);
pub const IRM_TopInkBreaksOnly: InkRecognitionModes = InkRecognitionModes(4i32);
pub const IRM_PrefixOk: InkRecognitionModes = InkRecognitionModes(8i32);
pub const IRM_LineMode: InkRecognitionModes = InkRecognitionModes(16i32);
pub const IRM_DisablePersonalization: InkRecognitionModes = InkRecognitionModes(32i32);
pub const IRM_AutoSpace: InkRecognitionModes = InkRecognitionModes(64i32);
pub const IRM_Max: InkRecognitionModes = InkRecognitionModes(128i32);
impl ::core::marker::Copy for InkRecognitionModes {}
impl ::core::clone::Clone for InkRecognitionModes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognitionStatus(pub i32);
pub const IRS_NoError: InkRecognitionStatus = InkRecognitionStatus(0i32);
pub const IRS_Interrupted: InkRecognitionStatus = InkRecognitionStatus(1i32);
pub const IRS_ProcessFailed: InkRecognitionStatus = InkRecognitionStatus(2i32);
pub const IRS_InkAddedFailed: InkRecognitionStatus = InkRecognitionStatus(4i32);
pub const IRS_SetAutoCompletionModeFailed: InkRecognitionStatus = InkRecognitionStatus(8i32);
pub const IRS_SetStrokesFailed: InkRecognitionStatus = InkRecognitionStatus(16i32);
pub const IRS_SetGuideFailed: InkRecognitionStatus = InkRecognitionStatus(32i32);
pub const IRS_SetFlagsFailed: InkRecognitionStatus = InkRecognitionStatus(64i32);
pub const IRS_SetFactoidFailed: InkRecognitionStatus = InkRecognitionStatus(128i32);
pub const IRS_SetPrefixSuffixFailed: InkRecognitionStatus = InkRecognitionStatus(256i32);
pub const IRS_SetWordListFailed: InkRecognitionStatus = InkRecognitionStatus(512i32);
impl ::core::marker::Copy for InkRecognitionStatus {}
impl ::core::clone::Clone for InkRecognitionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognizerCapabilities(pub i32);
pub const IRC_DontCare: InkRecognizerCapabilities = InkRecognizerCapabilities(1i32);
pub const IRC_Object: InkRecognizerCapabilities = InkRecognizerCapabilities(2i32);
pub const IRC_FreeInput: InkRecognizerCapabilities = InkRecognizerCapabilities(4i32);
pub const IRC_LinedInput: InkRecognizerCapabilities = InkRecognizerCapabilities(8i32);
pub const IRC_BoxedInput: InkRecognizerCapabilities = InkRecognizerCapabilities(16i32);
pub const IRC_CharacterAutoCompletionInput: InkRecognizerCapabilities = InkRecognizerCapabilities(32i32);
pub const IRC_RightAndDown: InkRecognizerCapabilities = InkRecognizerCapabilities(64i32);
pub const IRC_LeftAndDown: InkRecognizerCapabilities = InkRecognizerCapabilities(128i32);
pub const IRC_DownAndLeft: InkRecognizerCapabilities = InkRecognizerCapabilities(256i32);
pub const IRC_DownAndRight: InkRecognizerCapabilities = InkRecognizerCapabilities(512i32);
pub const IRC_ArbitraryAngle: InkRecognizerCapabilities = InkRecognizerCapabilities(1024i32);
pub const IRC_Lattice: InkRecognizerCapabilities = InkRecognizerCapabilities(2048i32);
pub const IRC_AdviseInkChange: InkRecognizerCapabilities = InkRecognizerCapabilities(4096i32);
pub const IRC_StrokeReorder: InkRecognizerCapabilities = InkRecognizerCapabilities(8192i32);
pub const IRC_Personalizable: InkRecognizerCapabilities = InkRecognizerCapabilities(16384i32);
pub const IRC_PrefersArbitraryAngle: InkRecognizerCapabilities = InkRecognizerCapabilities(32768i32);
pub const IRC_PrefersParagraphBreaking: InkRecognizerCapabilities = InkRecognizerCapabilities(65536i32);
pub const IRC_PrefersSegmentation: InkRecognizerCapabilities = InkRecognizerCapabilities(131072i32);
pub const IRC_Cursive: InkRecognizerCapabilities = InkRecognizerCapabilities(262144i32);
pub const IRC_TextPrediction: InkRecognizerCapabilities = InkRecognizerCapabilities(524288i32);
pub const IRC_Alpha: InkRecognizerCapabilities = InkRecognizerCapabilities(1048576i32);
pub const IRC_Beta: InkRecognizerCapabilities = InkRecognizerCapabilities(2097152i32);
impl ::core::marker::Copy for InkRecognizerCapabilities {}
impl ::core::clone::Clone for InkRecognizerCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkRecognizerCharacterAutoCompletionMode(pub i32);
pub const IRCACM_Full: InkRecognizerCharacterAutoCompletionMode = InkRecognizerCharacterAutoCompletionMode(0i32);
pub const IRCACM_Prefix: InkRecognizerCharacterAutoCompletionMode = InkRecognizerCharacterAutoCompletionMode(1i32);
pub const IRCACM_Random: InkRecognizerCharacterAutoCompletionMode = InkRecognizerCharacterAutoCompletionMode(2i32);
impl ::core::marker::Copy for InkRecognizerCharacterAutoCompletionMode {}
impl ::core::clone::Clone for InkRecognizerCharacterAutoCompletionMode {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkRecognizerContext: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2864998967,
    data2: 37417,
    data3: 20416,
    data4: [140, 206, 68, 151, 86, 155, 244, 209],
};
pub const InkRecognizerGuide: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2272319809,
    data2: 42554,
    data3: 18033,
    data4: [163, 117, 40, 85, 161, 142, 186, 115],
};
pub const InkRecognizers: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2681530376, data2: 63206, data3: 20069, data4: [152, 211, 170, 57, 5, 76, 18, 85] };
pub const InkRectangle: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1135637286, data2: 43744, data3: 19298, data4: [168, 61, 95, 215, 104, 183, 53, 60] };
pub const InkRenderer: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2619131620,
    data2: 55275,
    data3: 20203,
    data4: [144, 145, 21, 167, 200, 121, 30, 217],
};
#[repr(transparent)]
pub struct InkSelectionConstants(pub i32);
pub const ISC_FirstElement: InkSelectionConstants = InkSelectionConstants(0i32);
pub const ISC_AllElements: InkSelectionConstants = InkSelectionConstants(-1i32);
impl ::core::marker::Copy for InkSelectionConstants {}
impl ::core::clone::Clone for InkSelectionConstants {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkShiftKeyModifierFlags(pub i32);
pub const IKM_Shift: InkShiftKeyModifierFlags = InkShiftKeyModifierFlags(1i32);
pub const IKM_Control: InkShiftKeyModifierFlags = InkShiftKeyModifierFlags(2i32);
pub const IKM_Alt: InkShiftKeyModifierFlags = InkShiftKeyModifierFlags(4i32);
impl ::core::marker::Copy for InkShiftKeyModifierFlags {}
impl ::core::clone::Clone for InkShiftKeyModifierFlags {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkStrokes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1223987644, data2: 9230, data3: 18528, data4: [176, 121, 161, 233, 77, 61, 44, 134] };
#[repr(transparent)]
pub struct InkSystemGesture(pub i32);
pub const ISG_Tap: InkSystemGesture = InkSystemGesture(16i32);
pub const ISG_DoubleTap: InkSystemGesture = InkSystemGesture(17i32);
pub const ISG_RightTap: InkSystemGesture = InkSystemGesture(18i32);
pub const ISG_Drag: InkSystemGesture = InkSystemGesture(19i32);
pub const ISG_RightDrag: InkSystemGesture = InkSystemGesture(20i32);
pub const ISG_HoldEnter: InkSystemGesture = InkSystemGesture(21i32);
pub const ISG_HoldLeave: InkSystemGesture = InkSystemGesture(22i32);
pub const ISG_HoverEnter: InkSystemGesture = InkSystemGesture(23i32);
pub const ISG_HoverLeave: InkSystemGesture = InkSystemGesture(24i32);
pub const ISG_Flick: InkSystemGesture = InkSystemGesture(31i32);
impl ::core::marker::Copy for InkSystemGesture {}
impl ::core::clone::Clone for InkSystemGesture {
    fn clone(&self) -> Self {
        *self
    }
}
pub const InkTablets: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1850723090, data2: 20746, data3: 19776, data4: [147, 4, 29, 161, 10, 233, 20, 124] };
pub const InkTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3822442812, data2: 5731, data3: 19064, data4: [161, 167, 34, 55, 93, 254, 186, 238] };
pub const InkWordList: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2649247892,
    data2: 63263,
    data3: 17649,
    data4: [132, 113, 21, 162, 250, 118, 252, 243],
};
#[repr(transparent)]
pub struct InteractionMode(pub i32);
pub const InteractionMode_InPlace: InteractionMode = InteractionMode(0i32);
pub const InteractionMode_Floating: InteractionMode = InteractionMode(1i32);
pub const InteractionMode_DockedTop: InteractionMode = InteractionMode(2i32);
pub const InteractionMode_DockedBottom: InteractionMode = InteractionMode(3i32);
impl ::core::marker::Copy for InteractionMode {}
impl ::core::clone::Clone for InteractionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct KEYMODIFIER(pub i32);
pub const KEYMODIFIER_CONTROL: KEYMODIFIER = KEYMODIFIER(1i32);
pub const KEYMODIFIER_MENU: KEYMODIFIER = KEYMODIFIER(2i32);
pub const KEYMODIFIER_SHIFT: KEYMODIFIER = KEYMODIFIER(4i32);
pub const KEYMODIFIER_WIN: KEYMODIFIER = KEYMODIFIER(8i32);
pub const KEYMODIFIER_ALTGR: KEYMODIFIER = KEYMODIFIER(16i32);
pub const KEYMODIFIER_EXT: KEYMODIFIER = KEYMODIFIER(32i32);
impl ::core::marker::Copy for KEYMODIFIER {}
impl ::core::clone::Clone for KEYMODIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LATTICE_METRICS {
    pub lsBaseline: LINE_SEGMENT,
    pub iMidlineOffset: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LATTICE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LATTICE_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct LINE_METRICS(pub i32);
pub const LM_BASELINE: LINE_METRICS = LINE_METRICS(0i32);
pub const LM_MIDLINE: LINE_METRICS = LINE_METRICS(1i32);
pub const LM_ASCENDER: LINE_METRICS = LINE_METRICS(2i32);
pub const LM_DESCENDER: LINE_METRICS = LINE_METRICS(3i32);
impl ::core::marker::Copy for LINE_METRICS {}
impl ::core::clone::Clone for LINE_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct LINE_SEGMENT {
    pub PtA: super::super::Foundation::POINT,
    pub PtB: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LINE_SEGMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LINE_SEGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MAX_FRIENDLYNAME: u32 = 64u32;
pub const MAX_LANGUAGES: u32 = 64u32;
pub const MAX_PACKET_BUTTON_COUNT: u32 = 32u32;
pub const MAX_PACKET_PROPERTY_COUNT: u32 = 32u32;
pub const MAX_VENDORNAME: u32 = 32u32;
pub const MICROSOFT_TIP_COMBOBOXLIST_PROPERTY: &'static str = "Microsoft TIP ComboBox List Window Identifier";
pub const MICROSOFT_TIP_NO_INSERT_BUTTON_PROPERTY: &'static str = "Microsoft TIP No Insert Option";
pub const MICROSOFT_TIP_OPENING_MSG: &'static str = "TabletInputPanelOpening";
pub const MICROSOFT_URL_EXPERIENCE_PROPERTY: &'static str = "Microsoft TIP URL Experience";
#[repr(transparent)]
pub struct MICUIELEMENT(pub i32);
pub const MICUIELEMENT_BUTTON_WRITE: MICUIELEMENT = MICUIELEMENT(1i32);
pub const MICUIELEMENT_BUTTON_ERASE: MICUIELEMENT = MICUIELEMENT(2i32);
pub const MICUIELEMENT_BUTTON_CORRECT: MICUIELEMENT = MICUIELEMENT(4i32);
pub const MICUIELEMENT_BUTTON_CLEAR: MICUIELEMENT = MICUIELEMENT(8i32);
pub const MICUIELEMENT_BUTTON_UNDO: MICUIELEMENT = MICUIELEMENT(16i32);
pub const MICUIELEMENT_BUTTON_REDO: MICUIELEMENT = MICUIELEMENT(32i32);
pub const MICUIELEMENT_BUTTON_INSERT: MICUIELEMENT = MICUIELEMENT(64i32);
pub const MICUIELEMENT_BUTTON_CANCEL: MICUIELEMENT = MICUIELEMENT(128i32);
pub const MICUIELEMENT_INKPANEL_BACKGROUND: MICUIELEMENT = MICUIELEMENT(256i32);
pub const MICUIELEMENT_RESULTPANEL_BACKGROUND: MICUIELEMENT = MICUIELEMENT(512i32);
impl ::core::marker::Copy for MICUIELEMENT {}
impl ::core::clone::Clone for MICUIELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MICUIELEMENTSTATE(pub i32);
pub const MICUIELEMENTSTATE_NORMAL: MICUIELEMENTSTATE = MICUIELEMENTSTATE(1i32);
pub const MICUIELEMENTSTATE_HOT: MICUIELEMENTSTATE = MICUIELEMENTSTATE(2i32);
pub const MICUIELEMENTSTATE_PRESSED: MICUIELEMENTSTATE = MICUIELEMENTSTATE(3i32);
pub const MICUIELEMENTSTATE_DISABLED: MICUIELEMENTSTATE = MICUIELEMENTSTATE(4i32);
impl ::core::marker::Copy for MICUIELEMENTSTATE {}
impl ::core::clone::Clone for MICUIELEMENTSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MathInputControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3311501676,
    data2: 5336,
    data3: 16528,
    data4: [131, 12, 152, 217, 148, 178, 28, 123],
};
#[repr(transparent)]
pub struct MouseButton(pub i32);
pub const NO_BUTTON: MouseButton = MouseButton(0i32);
pub const LEFT_BUTTON: MouseButton = MouseButton(1i32);
pub const RIGHT_BUTTON: MouseButton = MouseButton(2i32);
pub const MIDDLE_BUTTON: MouseButton = MouseButton(4i32);
impl ::core::marker::Copy for MouseButton {}
impl ::core::clone::Clone for MouseButton {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NUM_FLICK_DIRECTIONS: u32 = 8u32;
#[repr(C)]
pub struct PACKET_DESCRIPTION {
    pub cbPacketSize: u32,
    pub cPacketProperties: u32,
    pub pPacketProperties: *mut PACKET_PROPERTY,
    pub cButtons: u32,
    pub pguidButtons: *mut ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for PACKET_DESCRIPTION {}
impl ::core::clone::Clone for PACKET_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PACKET_PROPERTY {
    pub guid: ::windows_sys::core::GUID,
    pub PropertyMetrics: PROPERTY_METRICS,
}
impl ::core::marker::Copy for PACKET_PROPERTY {}
impl ::core::clone::Clone for PACKET_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PROPERTY_METRICS {
    pub nLogicalMin: i32,
    pub nLogicalMax: i32,
    pub Units: PROPERTY_UNITS,
    pub fResolution: f32,
}
impl ::core::marker::Copy for PROPERTY_METRICS {}
impl ::core::clone::Clone for PROPERTY_METRICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PROPERTY_UNITS(pub i32);
pub const PROPERTY_UNITS_DEFAULT: PROPERTY_UNITS = PROPERTY_UNITS(0i32);
pub const PROPERTY_UNITS_INCHES: PROPERTY_UNITS = PROPERTY_UNITS(1i32);
pub const PROPERTY_UNITS_CENTIMETERS: PROPERTY_UNITS = PROPERTY_UNITS(2i32);
pub const PROPERTY_UNITS_DEGREES: PROPERTY_UNITS = PROPERTY_UNITS(3i32);
pub const PROPERTY_UNITS_RADIANS: PROPERTY_UNITS = PROPERTY_UNITS(4i32);
pub const PROPERTY_UNITS_SECONDS: PROPERTY_UNITS = PROPERTY_UNITS(5i32);
pub const PROPERTY_UNITS_POUNDS: PROPERTY_UNITS = PROPERTY_UNITS(6i32);
pub const PROPERTY_UNITS_GRAMS: PROPERTY_UNITS = PROPERTY_UNITS(7i32);
pub const PROPERTY_UNITS_SILINEAR: PROPERTY_UNITS = PROPERTY_UNITS(8i32);
pub const PROPERTY_UNITS_SIROTATION: PROPERTY_UNITS = PROPERTY_UNITS(9i32);
pub const PROPERTY_UNITS_ENGLINEAR: PROPERTY_UNITS = PROPERTY_UNITS(10i32);
pub const PROPERTY_UNITS_ENGROTATION: PROPERTY_UNITS = PROPERTY_UNITS(11i32);
pub const PROPERTY_UNITS_SLUGS: PROPERTY_UNITS = PROPERTY_UNITS(12i32);
pub const PROPERTY_UNITS_KELVIN: PROPERTY_UNITS = PROPERTY_UNITS(13i32);
pub const PROPERTY_UNITS_FAHRENHEIT: PROPERTY_UNITS = PROPERTY_UNITS(14i32);
pub const PROPERTY_UNITS_AMPERE: PROPERTY_UNITS = PROPERTY_UNITS(15i32);
pub const PROPERTY_UNITS_CANDELA: PROPERTY_UNITS = PROPERTY_UNITS(16i32);
impl ::core::marker::Copy for PROPERTY_UNITS {}
impl ::core::clone::Clone for PROPERTY_UNITS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PanelInputArea(pub i32);
pub const PanelInputArea_Auto: PanelInputArea = PanelInputArea(0i32);
pub const PanelInputArea_Keyboard: PanelInputArea = PanelInputArea(1i32);
pub const PanelInputArea_WritingPad: PanelInputArea = PanelInputArea(2i32);
pub const PanelInputArea_CharacterPad: PanelInputArea = PanelInputArea(3i32);
impl ::core::marker::Copy for PanelInputArea {}
impl ::core::clone::Clone for PanelInputArea {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PanelType(pub i32);
pub const PT_Default: PanelType = PanelType(0i32);
pub const PT_Inactive: PanelType = PanelType(1i32);
pub const PT_Handwriting: PanelType = PanelType(2i32);
pub const PT_Keyboard: PanelType = PanelType(3i32);
impl ::core::marker::Copy for PanelType {}
impl ::core::clone::Clone for PanelType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PenInputPanel: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4148487318,
    data2: 7002,
    data3: 18590,
    data4: [129, 220, 251, 215, 172, 98, 152, 168],
};
pub const PenInputPanel_Internal: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2150309817, data2: 1387, data3: 18208, data4: [176, 204, 128, 210, 59, 113, 23, 30] };
pub type PfnRecoCallback = unsafe extern "system" fn(param0: u32, param1: *mut u8, param2: HRECOCONTEXT) -> ::windows_sys::core::HRESULT;
pub const RECOCONF_HIGHCONFIDENCE: u32 = 1u32;
pub const RECOCONF_LOWCONFIDENCE: i32 = -1i32;
pub const RECOCONF_MEDIUMCONFIDENCE: u32 = 0u32;
pub const RECOCONF_NOTSET: u32 = 128u32;
pub const RECOFLAG_AUTOSPACE: u32 = 64u32;
pub const RECOFLAG_COERCE: u32 = 2u32;
pub const RECOFLAG_DISABLEPERSONALIZATION: u32 = 32u32;
pub const RECOFLAG_LINEMODE: u32 = 16u32;
pub const RECOFLAG_PREFIXOK: u32 = 8u32;
pub const RECOFLAG_SINGLESEG: u32 = 4u32;
pub const RECOFLAG_WORDMODE: u32 = 1u32;
#[repr(C)]
pub struct RECO_ATTRS {
    pub dwRecoCapabilityFlags: u32,
    pub awcVendorName: [u16; 32],
    pub awcFriendlyName: [u16; 64],
    pub awLanguageId: [u16; 64],
}
impl ::core::marker::Copy for RECO_ATTRS {}
impl ::core::clone::Clone for RECO_ATTRS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RECO_GUIDE {
    pub xOrigin: i32,
    pub yOrigin: i32,
    pub cxBox: i32,
    pub cyBox: i32,
    pub cxBase: i32,
    pub cyBase: i32,
    pub cHorzBox: i32,
    pub cVertBox: i32,
    pub cyMid: i32,
}
impl ::core::marker::Copy for RECO_GUIDE {}
impl ::core::clone::Clone for RECO_GUIDE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RECO_LATTICE {
    pub ulColumnCount: u32,
    pub pLatticeColumns: *mut RECO_LATTICE_COLUMN,
    pub ulPropertyCount: u32,
    pub pGuidProperties: *mut ::windows_sys::core::GUID,
    pub ulBestResultColumnCount: u32,
    pub pulBestResultColumns: *mut u32,
    pub pulBestResultIndexes: *mut u32,
}
impl ::core::marker::Copy for RECO_LATTICE {}
impl ::core::clone::Clone for RECO_LATTICE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RECO_LATTICE_COLUMN {
    pub key: u32,
    pub cpProp: RECO_LATTICE_PROPERTIES,
    pub cStrokes: u32,
    pub pStrokes: *mut u32,
    pub cLatticeElements: u32,
    pub pLatticeElements: *mut RECO_LATTICE_ELEMENT,
}
impl ::core::marker::Copy for RECO_LATTICE_COLUMN {}
impl ::core::clone::Clone for RECO_LATTICE_COLUMN {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RECO_LATTICE_ELEMENT {
    pub score: i32,
    pub r#type: u16,
    pub pData: *mut u8,
    pub ulNextColumn: u32,
    pub ulStrokeNumber: u32,
    pub epProp: RECO_LATTICE_PROPERTIES,
}
impl ::core::marker::Copy for RECO_LATTICE_ELEMENT {}
impl ::core::clone::Clone for RECO_LATTICE_ELEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RECO_LATTICE_PROPERTIES {
    pub cProperties: u32,
    pub apProps: *mut *mut RECO_LATTICE_PROPERTY,
}
impl ::core::marker::Copy for RECO_LATTICE_PROPERTIES {}
impl ::core::clone::Clone for RECO_LATTICE_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RECO_LATTICE_PROPERTY {
    pub guidProperty: ::windows_sys::core::GUID,
    pub cbPropertyValue: u16,
    pub pPropertyValue: *mut u8,
}
impl ::core::marker::Copy for RECO_LATTICE_PROPERTY {}
impl ::core::clone::Clone for RECO_LATTICE_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct RECO_RANGE {
    pub iwcBegin: u32,
    pub cCount: u32,
}
impl ::core::marker::Copy for RECO_RANGE {}
impl ::core::clone::Clone for RECO_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RF_ADVISEINKCHANGE: i32 = 4096i32;
pub const RF_ARBITRARY_ANGLE: i32 = 1024i32;
pub const RF_BOXED_INPUT: i32 = 16i32;
pub const RF_CAC_INPUT: i32 = 32i32;
pub const RF_DONTCARE: i32 = 1i32;
pub const RF_DOWN_AND_LEFT: i32 = 256i32;
pub const RF_DOWN_AND_RIGHT: i32 = 512i32;
pub const RF_FREE_INPUT: i32 = 4i32;
pub const RF_LATTICE: i32 = 2048i32;
pub const RF_LEFT_AND_DOWN: i32 = 128i32;
pub const RF_LINED_INPUT: i32 = 8i32;
pub const RF_OBJECT: i32 = 2i32;
pub const RF_PERFORMSLINEBREAKING: i32 = 65536i32;
pub const RF_PERSONALIZABLE: i32 = 16384i32;
pub const RF_REQUIRESSEGMENTATIONBREAKING: i32 = 131072i32;
pub const RF_RIGHT_AND_DOWN: i32 = 64i32;
pub const RF_STROKEREORDER: i32 = 8192i32;
pub const RealTimeStylus: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3798677101,
    data2: 63896,
    data3: 17358,
    data4: [131, 111, 203, 109, 144, 68, 50, 176],
};
#[repr(transparent)]
pub struct RealTimeStylusDataInterest(pub i32);
pub const RTSDI_AllData: RealTimeStylusDataInterest = RealTimeStylusDataInterest(-1i32);
pub const RTSDI_None: RealTimeStylusDataInterest = RealTimeStylusDataInterest(0i32);
pub const RTSDI_Error: RealTimeStylusDataInterest = RealTimeStylusDataInterest(1i32);
pub const RTSDI_RealTimeStylusEnabled: RealTimeStylusDataInterest = RealTimeStylusDataInterest(2i32);
pub const RTSDI_RealTimeStylusDisabled: RealTimeStylusDataInterest = RealTimeStylusDataInterest(4i32);
pub const RTSDI_StylusNew: RealTimeStylusDataInterest = RealTimeStylusDataInterest(8i32);
pub const RTSDI_StylusInRange: RealTimeStylusDataInterest = RealTimeStylusDataInterest(16i32);
pub const RTSDI_InAirPackets: RealTimeStylusDataInterest = RealTimeStylusDataInterest(32i32);
pub const RTSDI_StylusOutOfRange: RealTimeStylusDataInterest = RealTimeStylusDataInterest(64i32);
pub const RTSDI_StylusDown: RealTimeStylusDataInterest = RealTimeStylusDataInterest(128i32);
pub const RTSDI_Packets: RealTimeStylusDataInterest = RealTimeStylusDataInterest(256i32);
pub const RTSDI_StylusUp: RealTimeStylusDataInterest = RealTimeStylusDataInterest(512i32);
pub const RTSDI_StylusButtonUp: RealTimeStylusDataInterest = RealTimeStylusDataInterest(1024i32);
pub const RTSDI_StylusButtonDown: RealTimeStylusDataInterest = RealTimeStylusDataInterest(2048i32);
pub const RTSDI_SystemEvents: RealTimeStylusDataInterest = RealTimeStylusDataInterest(4096i32);
pub const RTSDI_TabletAdded: RealTimeStylusDataInterest = RealTimeStylusDataInterest(8192i32);
pub const RTSDI_TabletRemoved: RealTimeStylusDataInterest = RealTimeStylusDataInterest(16384i32);
pub const RTSDI_CustomStylusDataAdded: RealTimeStylusDataInterest = RealTimeStylusDataInterest(32768i32);
pub const RTSDI_UpdateMapping: RealTimeStylusDataInterest = RealTimeStylusDataInterest(65536i32);
pub const RTSDI_DefaultEvents: RealTimeStylusDataInterest = RealTimeStylusDataInterest(37766i32);
impl ::core::marker::Copy for RealTimeStylusDataInterest {}
impl ::core::clone::Clone for RealTimeStylusDataInterest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RealTimeStylusLockType(pub i32);
pub const RTSLT_ObjLock: RealTimeStylusLockType = RealTimeStylusLockType(1i32);
pub const RTSLT_SyncEventLock: RealTimeStylusLockType = RealTimeStylusLockType(2i32);
pub const RTSLT_AsyncEventLock: RealTimeStylusLockType = RealTimeStylusLockType(4i32);
pub const RTSLT_ExcludeCallback: RealTimeStylusLockType = RealTimeStylusLockType(8i32);
pub const RTSLT_SyncObjLock: RealTimeStylusLockType = RealTimeStylusLockType(11i32);
pub const RTSLT_AsyncObjLock: RealTimeStylusLockType = RealTimeStylusLockType(13i32);
impl ::core::marker::Copy for RealTimeStylusLockType {}
impl ::core::clone::Clone for RealTimeStylusLockType {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SAFE_PARTIAL: u32 = 1u32;
#[repr(transparent)]
pub struct SCROLLDIRECTION(pub i32);
pub const SCROLLDIRECTION_UP: SCROLLDIRECTION = SCROLLDIRECTION(0i32);
pub const SCROLLDIRECTION_DOWN: SCROLLDIRECTION = SCROLLDIRECTION(1i32);
impl ::core::marker::Copy for SCROLLDIRECTION {}
impl ::core::clone::Clone for SCROLLDIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct STROKE_RANGE {
    pub iStrokeBegin: u32,
    pub iStrokeEnd: u32,
}
impl ::core::marker::Copy for STROKE_RANGE {}
impl ::core::clone::Clone for STROKE_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SYSTEM_EVENT_DATA {
    pub bModifier: u8,
    pub wKey: u16,
    pub xPos: i32,
    pub yPos: i32,
    pub bCursorMode: u8,
    pub dwButtonState: u32,
}
impl ::core::marker::Copy for SYSTEM_EVENT_DATA {}
impl ::core::clone::Clone for SYSTEM_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ScrollBarsConstants(pub i32);
pub const rtfNone: ScrollBarsConstants = ScrollBarsConstants(0i32);
pub const rtfHorizontal: ScrollBarsConstants = ScrollBarsConstants(1i32);
pub const rtfVertical: ScrollBarsConstants = ScrollBarsConstants(2i32);
pub const rtfBoth: ScrollBarsConstants = ScrollBarsConstants(3i32);
impl ::core::marker::Copy for ScrollBarsConstants {}
impl ::core::clone::Clone for ScrollBarsConstants {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SelAlignmentConstants(pub i32);
pub const rtfLeft: SelAlignmentConstants = SelAlignmentConstants(0i32);
pub const rtfRight: SelAlignmentConstants = SelAlignmentConstants(1i32);
pub const rtfCenter: SelAlignmentConstants = SelAlignmentConstants(2i32);
impl ::core::marker::Copy for SelAlignmentConstants {}
impl ::core::clone::Clone for SelAlignmentConstants {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SelectionHitResult(pub i32);
pub const SHR_None: SelectionHitResult = SelectionHitResult(0i32);
pub const SHR_NW: SelectionHitResult = SelectionHitResult(1i32);
pub const SHR_SE: SelectionHitResult = SelectionHitResult(2i32);
pub const SHR_NE: SelectionHitResult = SelectionHitResult(3i32);
pub const SHR_SW: SelectionHitResult = SelectionHitResult(4i32);
pub const SHR_E: SelectionHitResult = SelectionHitResult(5i32);
pub const SHR_W: SelectionHitResult = SelectionHitResult(6i32);
pub const SHR_N: SelectionHitResult = SelectionHitResult(7i32);
pub const SHR_S: SelectionHitResult = SelectionHitResult(8i32);
pub const SHR_Selection: SelectionHitResult = SelectionHitResult(9i32);
impl ::core::marker::Copy for SelectionHitResult {}
impl ::core::clone::Clone for SelectionHitResult {
    fn clone(&self) -> Self {
        *self
    }
}
pub const SketchInk: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4029223041,
    data2: 59516,
    data3: 19975,
    data4: [151, 218, 160, 160, 55, 97, 229, 134],
};
pub const StrokeBuilder: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3893415655,
    data2: 28241,
    data3: 19632,
    data4: [170, 58, 11, 152, 91, 112, 218, 247],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct StylusInfo {
    pub tcid: u32,
    pub cid: u32,
    pub bIsInvertedCursor: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for StylusInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for StylusInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StylusQueue(pub i32);
pub const SyncStylusQueue: StylusQueue = StylusQueue(1i32);
pub const AsyncStylusQueueImmediate: StylusQueue = StylusQueue(2i32);
pub const AsyncStylusQueue: StylusQueue = StylusQueue(3i32);
impl ::core::marker::Copy for StylusQueue {}
impl ::core::clone::Clone for StylusQueue {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TABLET_DISABLE_FLICKFALLBACKKEYS: u32 = 1048576u32;
pub const TABLET_DISABLE_FLICKS: u32 = 65536u32;
pub const TABLET_DISABLE_PENBARRELFEEDBACK: u32 = 16u32;
pub const TABLET_DISABLE_PENTAPFEEDBACK: u32 = 8u32;
pub const TABLET_DISABLE_PRESSANDHOLD: u32 = 1u32;
pub const TABLET_DISABLE_SMOOTHSCROLLING: u32 = 524288u32;
pub const TABLET_DISABLE_TOUCHSWITCH: u32 = 32768u32;
pub const TABLET_DISABLE_TOUCHUIFORCEOFF: u32 = 512u32;
pub const TABLET_DISABLE_TOUCHUIFORCEON: u32 = 256u32;
pub const TABLET_ENABLE_FLICKLEARNINGMODE: u32 = 262144u32;
pub const TABLET_ENABLE_FLICKSONCONTEXT: u32 = 131072u32;
pub const TABLET_ENABLE_MULTITOUCHDATA: u32 = 16777216u32;
#[repr(transparent)]
pub struct TabletDeviceKind(pub i32);
pub const TDK_Mouse: TabletDeviceKind = TabletDeviceKind(0i32);
pub const TDK_Pen: TabletDeviceKind = TabletDeviceKind(1i32);
pub const TDK_Touch: TabletDeviceKind = TabletDeviceKind(2i32);
impl ::core::marker::Copy for TabletDeviceKind {}
impl ::core::clone::Clone for TabletDeviceKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TabletHardwareCapabilities(pub i32);
pub const THWC_Integrated: TabletHardwareCapabilities = TabletHardwareCapabilities(1i32);
pub const THWC_CursorMustTouch: TabletHardwareCapabilities = TabletHardwareCapabilities(2i32);
pub const THWC_HardProximity: TabletHardwareCapabilities = TabletHardwareCapabilities(4i32);
pub const THWC_CursorsHavePhysicalIds: TabletHardwareCapabilities = TabletHardwareCapabilities(8i32);
impl ::core::marker::Copy for TabletHardwareCapabilities {}
impl ::core::clone::Clone for TabletHardwareCapabilities {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TabletPropertyMetricUnit(pub i32);
pub const TPMU_Default: TabletPropertyMetricUnit = TabletPropertyMetricUnit(0i32);
pub const TPMU_Inches: TabletPropertyMetricUnit = TabletPropertyMetricUnit(1i32);
pub const TPMU_Centimeters: TabletPropertyMetricUnit = TabletPropertyMetricUnit(2i32);
pub const TPMU_Degrees: TabletPropertyMetricUnit = TabletPropertyMetricUnit(3i32);
pub const TPMU_Radians: TabletPropertyMetricUnit = TabletPropertyMetricUnit(4i32);
pub const TPMU_Seconds: TabletPropertyMetricUnit = TabletPropertyMetricUnit(5i32);
pub const TPMU_Pounds: TabletPropertyMetricUnit = TabletPropertyMetricUnit(6i32);
pub const TPMU_Grams: TabletPropertyMetricUnit = TabletPropertyMetricUnit(7i32);
impl ::core::marker::Copy for TabletPropertyMetricUnit {}
impl ::core::clone::Clone for TabletPropertyMetricUnit {
    fn clone(&self) -> Self {
        *self
    }
}
pub const TextInputPanel: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4189161943, data2: 8843, data3: 20267, data4: [134, 80, 185, 127, 89, 224, 44, 140] };
pub const TipAutoCompleteClient: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2155617900,
    data2: 7424,
    data3: 17727,
    data4: [185, 32, 182, 27, 183, 205, 217, 151],
};
#[repr(transparent)]
pub struct VisualState(pub i32);
pub const InPlace: VisualState = VisualState(0i32);
pub const Floating: VisualState = VisualState(1i32);
pub const DockedTop: VisualState = VisualState(2i32);
pub const DockedBottom: VisualState = VisualState(3i32);
pub const Closed: VisualState = VisualState(4i32);
impl ::core::marker::Copy for VisualState {}
impl ::core::clone::Clone for VisualState {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WM_TABLET_ADDED: u32 = 712u32;
pub const WM_TABLET_DEFBASE: u32 = 704u32;
pub const WM_TABLET_DELETED: u32 = 713u32;
pub const WM_TABLET_FLICK: u32 = 715u32;
pub const WM_TABLET_MAXOFFSET: u32 = 32u32;
pub const WM_TABLET_QUERYSYSTEMGESTURESTATUS: u32 = 716u32;
#[repr(transparent)]
pub struct _IInkCollectorEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _IInkCollectorEvents {}
impl ::core::clone::Clone for _IInkCollectorEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _IInkEditEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _IInkEditEvents {}
impl ::core::clone::Clone for _IInkEditEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _IInkEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _IInkEvents {}
impl ::core::clone::Clone for _IInkEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _IInkOverlayEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _IInkOverlayEvents {}
impl ::core::clone::Clone for _IInkOverlayEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _IInkPictureEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _IInkPictureEvents {}
impl ::core::clone::Clone for _IInkPictureEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _IInkRecognitionEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _IInkRecognitionEvents {}
impl ::core::clone::Clone for _IInkRecognitionEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _IInkStrokesEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _IInkStrokesEvents {}
impl ::core::clone::Clone for _IInkStrokesEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _IMathInputControlEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _IMathInputControlEvents {}
impl ::core::clone::Clone for _IMathInputControlEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _IPenInputPanelEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _IPenInputPanelEvents {}
impl ::core::clone::Clone for _IPenInputPanelEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct enumGetCandidateFlags(pub i32);
pub const TCF_ALLOW_RECOGNITION: enumGetCandidateFlags = enumGetCandidateFlags(1i32);
pub const TCF_FORCE_RECOGNITION: enumGetCandidateFlags = enumGetCandidateFlags(2i32);
impl ::core::marker::Copy for enumGetCandidateFlags {}
impl ::core::clone::Clone for enumGetCandidateFlags {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct enumINKMETRIC_FLAGS(pub i32);
pub const IMF_FONT_SELECTED_IN_HDC: enumINKMETRIC_FLAGS = enumINKMETRIC_FLAGS(1i32);
pub const IMF_ITALIC: enumINKMETRIC_FLAGS = enumINKMETRIC_FLAGS(2i32);
pub const IMF_BOLD: enumINKMETRIC_FLAGS = enumINKMETRIC_FLAGS(4i32);
impl ::core::marker::Copy for enumINKMETRIC_FLAGS {}
impl ::core::clone::Clone for enumINKMETRIC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct enumRECO_TYPE(pub i32);
pub const RECO_TYPE_WSTRING: enumRECO_TYPE = enumRECO_TYPE(0i32);
pub const RECO_TYPE_WCHAR: enumRECO_TYPE = enumRECO_TYPE(1i32);
impl ::core::marker::Copy for enumRECO_TYPE {}
impl ::core::clone::Clone for enumRECO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
