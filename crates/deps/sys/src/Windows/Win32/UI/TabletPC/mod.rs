#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const ALT_BREAKS_SAME: i32 = 0i32;
pub const ALT_BREAKS_UNIQUE: i32 = 1i32;
pub const ALT_BREAKS_FULL: i32 = 2i32;
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
pub const rtfFlat: i32 = 0i32;
pub const rtfThreeD: i32 = 1i32;
pub const BEST_COMPLETE: u32 = 2u32;
pub const rtfNoBorder: i32 = 0i32;
pub const rtfFixedSingle: i32 = 1i32;
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
pub const CFL_STRONG: i32 = 0i32;
pub const CFL_INTERMEDIATE: i32 = 1i32;
pub const CFL_POOR: i32 = 2i32;
pub const CorrectionMode_NotVisible: i32 = 0i32;
pub const CorrectionMode_PreInsertion: i32 = 1i32;
pub const CorrectionMode_PostInsertionCollapsed: i32 = 2i32;
pub const CorrectionMode_PostInsertionExpanded: i32 = 3i32;
pub const CorrectionPosition_Auto: i32 = 0i32;
pub const CorrectionPosition_Bottom: i32 = 1i32;
pub const CorrectionPosition_Top: i32 = 2i32;
pub const DISPID_IStrokes: i32 = 1i32;
pub const DISPID_IExtendedProperties: i32 = 2i32;
pub const DISPID_IGetBoundingBox: i32 = 3i32;
pub const DISPID_IDeleteStrokes: i32 = 4i32;
pub const DISPID_IDeleteStroke: i32 = 5i32;
pub const DISPID_IExtractStrokes: i32 = 6i32;
pub const DISPID_IExtractWithRectangle: i32 = 7i32;
pub const DISPID_IDirty: i32 = 8i32;
pub const DISPID_ICustomStrokes: i32 = 9i32;
pub const DISPID_IClone: i32 = 10i32;
pub const DISPID_IHitTestCircle: i32 = 11i32;
pub const DISPID_IHitTestWithRectangle: i32 = 12i32;
pub const DISPID_IHitTestWithLasso: i32 = 13i32;
pub const DISPID_INearestPoint: i32 = 14i32;
pub const DISPID_ICreateStrokes: i32 = 15i32;
pub const DISPID_ICreateStroke: i32 = 16i32;
pub const DISPID_IAddStrokesAtRectangle: i32 = 17i32;
pub const DISPID_IClip: i32 = 18i32;
pub const DISPID_ISave: i32 = 19i32;
pub const DISPID_ILoad: i32 = 20i32;
pub const DISPID_ICreateStrokeFromPoints: i32 = 21i32;
pub const DISPID_IClipboardCopyWithRectangle: i32 = 22i32;
pub const DISPID_IClipboardCopy: i32 = 23i32;
pub const DISPID_ICanPaste: i32 = 24i32;
pub const DISPID_IClipboardPaste: i32 = 25i32;
pub const DISPID_ICEnabled: i32 = 1i32;
pub const DISPID_ICHwnd: i32 = 2i32;
pub const DISPID_ICPaint: i32 = 3i32;
pub const DISPID_ICText: i32 = 4i32;
pub const DISPID_ICDefaultDrawingAttributes: i32 = 5i32;
pub const DISPID_ICRenderer: i32 = 6i32;
pub const DISPID_ICInk: i32 = 7i32;
pub const DISPID_ICAutoRedraw: i32 = 8i32;
pub const DISPID_ICCollectingInk: i32 = 9i32;
pub const DISPID_ICSetEventInterest: i32 = 10i32;
pub const DISPID_ICGetEventInterest: i32 = 11i32;
pub const DISPID_IOEditingMode: i32 = 12i32;
pub const DISPID_IOSelection: i32 = 13i32;
pub const DISPID_IOAttachMode: i32 = 14i32;
pub const DISPID_IOHitTestSelection: i32 = 15i32;
pub const DISPID_IODraw: i32 = 16i32;
pub const DISPID_IPPicture: i32 = 17i32;
pub const DISPID_IPSizeMode: i32 = 18i32;
pub const DISPID_IPBackColor: i32 = 19i32;
pub const DISPID_ICCursors: i32 = 20i32;
pub const DISPID_ICMarginX: i32 = 21i32;
pub const DISPID_ICMarginY: i32 = 22i32;
pub const DISPID_ICSetWindowInputRectangle: i32 = 23i32;
pub const DISPID_ICGetWindowInputRectangle: i32 = 24i32;
pub const DISPID_ICTablet: i32 = 25i32;
pub const DISPID_ICSetAllTabletsMode: i32 = 26i32;
pub const DISPID_ICSetSingleTabletIntegratedMode: i32 = 27i32;
pub const DISPID_ICCollectionMode: i32 = 28i32;
pub const DISPID_ICSetGestureStatus: i32 = 29i32;
pub const DISPID_ICGetGestureStatus: i32 = 30i32;
pub const DISPID_ICDynamicRendering: i32 = 31i32;
pub const DISPID_ICDesiredPacketDescription: i32 = 32i32;
pub const DISPID_IOEraserMode: i32 = 33i32;
pub const DISPID_IOEraserWidth: i32 = 34i32;
pub const DISPID_ICMouseIcon: i32 = 35i32;
pub const DISPID_ICMousePointer: i32 = 36i32;
pub const DISPID_IPInkEnabled: i32 = 37i32;
pub const DISPID_ICSupportHighContrastInk: i32 = 38i32;
pub const DISPID_IOSupportHighContrastSelectionUI: i32 = 39i32;
pub const DISPID_ICEStroke: i32 = 1i32;
pub const DISPID_ICECursorDown: i32 = 2i32;
pub const DISPID_ICENewPackets: i32 = 3i32;
pub const DISPID_ICENewInAirPackets: i32 = 4i32;
pub const DISPID_ICECursorButtonDown: i32 = 5i32;
pub const DISPID_ICECursorButtonUp: i32 = 6i32;
pub const DISPID_ICECursorInRange: i32 = 7i32;
pub const DISPID_ICECursorOutOfRange: i32 = 8i32;
pub const DISPID_ICESystemGesture: i32 = 9i32;
pub const DISPID_ICEGesture: i32 = 10i32;
pub const DISPID_ICETabletAdded: i32 = 11i32;
pub const DISPID_ICETabletRemoved: i32 = 12i32;
pub const DISPID_IOEPainting: i32 = 13i32;
pub const DISPID_IOEPainted: i32 = 14i32;
pub const DISPID_IOESelectionChanging: i32 = 15i32;
pub const DISPID_IOESelectionChanged: i32 = 16i32;
pub const DISPID_IOESelectionMoving: i32 = 17i32;
pub const DISPID_IOESelectionMoved: i32 = 18i32;
pub const DISPID_IOESelectionResizing: i32 = 19i32;
pub const DISPID_IOESelectionResized: i32 = 20i32;
pub const DISPID_IOEStrokesDeleting: i32 = 21i32;
pub const DISPID_IOEStrokesDeleted: i32 = 22i32;
pub const DISPID_IPEChangeUICues: i32 = 23i32;
pub const DISPID_IPEClick: i32 = 24i32;
pub const DISPID_IPEDblClick: i32 = 25i32;
pub const DISPID_IPEInvalidated: i32 = 26i32;
pub const DISPID_IPEMouseDown: i32 = 27i32;
pub const DISPID_IPEMouseEnter: i32 = 28i32;
pub const DISPID_IPEMouseHover: i32 = 29i32;
pub const DISPID_IPEMouseLeave: i32 = 30i32;
pub const DISPID_IPEMouseMove: i32 = 31i32;
pub const DISPID_IPEMouseUp: i32 = 32i32;
pub const DISPID_IPEMouseWheel: i32 = 33i32;
pub const DISPID_IPESizeModeChanged: i32 = 34i32;
pub const DISPID_IPEStyleChanged: i32 = 35i32;
pub const DISPID_IPESystemColorsChanged: i32 = 36i32;
pub const DISPID_IPEKeyDown: i32 = 37i32;
pub const DISPID_IPEKeyPress: i32 = 38i32;
pub const DISPID_IPEKeyUp: i32 = 39i32;
pub const DISPID_IPEResize: i32 = 40i32;
pub const DISPID_IPESizeChanged: i32 = 41i32;
pub const DISPID_ICsrName: i32 = 0i32;
pub const DISPID_ICsrId: i32 = 1i32;
pub const DISPID_ICsrDrawingAttributes: i32 = 2i32;
pub const DISPID_ICsrButtons: i32 = 3i32;
pub const DISPID_ICsrInverted: i32 = 4i32;
pub const DISPID_ICsrTablet: i32 = 5i32;
pub const DISPID_ICBName: i32 = 0i32;
pub const DISPID_ICBId: i32 = 1i32;
pub const DISPID_ICBState: i32 = 2i32;
pub const DISPID_ICBs_NewEnum: i32 = -4i32;
pub const DISPID_ICBsItem: i32 = 0i32;
pub const DISPID_ICBsCount: i32 = 1i32;
pub const DISPID_ICs_NewEnum: i32 = -4i32;
pub const DISPID_ICsItem: i32 = 0i32;
pub const DISPID_ICsCount: i32 = 1i32;
pub const DISPID_ICSs_NewEnum: i32 = -4i32;
pub const DISPID_ICSsItem: i32 = 0i32;
pub const DISPID_ICSsCount: i32 = 1i32;
pub const DISPID_ICSsAdd: i32 = 2i32;
pub const DISPID_ICSsRemove: i32 = 3i32;
pub const DISPID_ICSsClear: i32 = 4i32;
pub const DISPID_IInkDivider_Strokes: i32 = 1i32;
pub const DISPID_IInkDivider_RecognizerContext: i32 = 2i32;
pub const DISPID_IInkDivider_LineHeight: i32 = 3i32;
pub const DISPID_IInkDivider_Divide: i32 = 4i32;
pub const DISPID_IInkDivisionResult_Strokes: i32 = 1i32;
pub const DISPID_IInkDivisionResult_ResultByType: i32 = 2i32;
pub const DISPID_IInkDivisionUnit_Strokes: i32 = 1i32;
pub const DISPID_IInkDivisionUnit_DivisionType: i32 = 2i32;
pub const DISPID_IInkDivisionUnit_RecognizedString: i32 = 3i32;
pub const DISPID_IInkDivisionUnit_RotationTransform: i32 = 4i32;
pub const DISPID_IInkDivisionUnits_NewEnum: i32 = -4i32;
pub const DISPID_IInkDivisionUnits_Item: i32 = 0i32;
pub const DISPID_IInkDivisionUnits_Count: i32 = 1i32;
pub const DISPID_DAHeight: i32 = 1i32;
pub const DISPID_DAColor: i32 = 2i32;
pub const DISPID_DAWidth: i32 = 3i32;
pub const DISPID_DAFitToCurve: i32 = 4i32;
pub const DISPID_DAIgnorePressure: i32 = 5i32;
pub const DISPID_DAAntiAliased: i32 = 6i32;
pub const DISPID_DATransparency: i32 = 7i32;
pub const DISPID_DARasterOperation: i32 = 8i32;
pub const DISPID_DAPenTip: i32 = 9i32;
pub const DISPID_DAClone: i32 = 10i32;
pub const DISPID_DAExtendedProperties: i32 = 11i32;
pub const DISPID_Text: i32 = 0i32;
pub const DISPID_TextRTF: i32 = 1i32;
pub const DISPID_Hwnd: i32 = 2i32;
pub const DISPID_DisableNoScroll: i32 = 3i32;
pub const DISPID_Locked: i32 = 4i32;
pub const DISPID_Enabled: i32 = 5i32;
pub const DISPID_MaxLength: i32 = 6i32;
pub const DISPID_MultiLine: i32 = 7i32;
pub const DISPID_ScrollBars: i32 = 8i32;
pub const DISPID_RTSelStart: i32 = 9i32;
pub const DISPID_RTSelLength: i32 = 10i32;
pub const DISPID_RTSelText: i32 = 11i32;
pub const DISPID_SelAlignment: i32 = 12i32;
pub const DISPID_SelBold: i32 = 13i32;
pub const DISPID_SelCharOffset: i32 = 14i32;
pub const DISPID_SelColor: i32 = 15i32;
pub const DISPID_SelFontName: i32 = 16i32;
pub const DISPID_SelFontSize: i32 = 17i32;
pub const DISPID_SelItalic: i32 = 18i32;
pub const DISPID_SelRTF: i32 = 19i32;
pub const DISPID_SelUnderline: i32 = 20i32;
pub const DISPID_DragIcon: i32 = 21i32;
pub const DISPID_Status: i32 = 22i32;
pub const DISPID_UseMouseForInput: i32 = 23i32;
pub const DISPID_InkMode: i32 = 24i32;
pub const DISPID_InkInsertMode: i32 = 25i32;
pub const DISPID_RecoTimeout: i32 = 26i32;
pub const DISPID_DrawAttr: i32 = 27i32;
pub const DISPID_Recognizer: i32 = 28i32;
pub const DISPID_Factoid: i32 = 29i32;
pub const DISPID_SelInk: i32 = 30i32;
pub const DISPID_SelInksDisplayMode: i32 = 31i32;
pub const DISPID_Recognize: i32 = 32i32;
pub const DISPID_GetGestStatus: i32 = 33i32;
pub const DISPID_SetGestStatus: i32 = 34i32;
pub const DISPID_Refresh: i32 = 35i32;
pub const DISPID_IeeChange: i32 = 1i32;
pub const DISPID_IeeSelChange: i32 = 2i32;
pub const DISPID_IeeKeyDown: i32 = 3i32;
pub const DISPID_IeeKeyUp: i32 = 4i32;
pub const DISPID_IeeMouseUp: i32 = 5i32;
pub const DISPID_IeeMouseDown: i32 = 6i32;
pub const DISPID_IeeKeyPress: i32 = 7i32;
pub const DISPID_IeeDblClick: i32 = 8i32;
pub const DISPID_IeeClick: i32 = 9i32;
pub const DISPID_IeeMouseMove: i32 = 10i32;
pub const DISPID_IeeCursorDown: i32 = 21i32;
pub const DISPID_IeeStroke: i32 = 22i32;
pub const DISPID_IeeGesture: i32 = 23i32;
pub const DISPID_IeeRecognitionResult: i32 = 24i32;
pub const DISPID_IEInkAdded: i32 = 1i32;
pub const DISPID_IEInkDeleted: i32 = 2i32;
pub const DISPID_IEPs_NewEnum: i32 = -4i32;
pub const DISPID_IEPsItem: i32 = 0i32;
pub const DISPID_IEPsCount: i32 = 1i32;
pub const DISPID_IEPsAdd: i32 = 2i32;
pub const DISPID_IEPsRemove: i32 = 3i32;
pub const DISPID_IEPsClear: i32 = 4i32;
pub const DISPID_IEPsDoesPropertyExist: i32 = 5i32;
pub const DISPID_IEPGuid: i32 = 1i32;
pub const DISPID_IEPData: i32 = 2i32;
pub const DISPID_IGId: i32 = 0i32;
pub const DISPID_IGGetHotPoint: i32 = 1i32;
pub const DISPID_IGConfidence: i32 = 2i32;
pub const DISPID_InkRecoAlternate_String: i32 = 1i32;
pub const DISPID_InkRecoAlternate_LineNumber: i32 = 2i32;
pub const DISPID_InkRecoAlternate_Baseline: i32 = 3i32;
pub const DISPID_InkRecoAlternate_Midline: i32 = 4i32;
pub const DISPID_InkRecoAlternate_Ascender: i32 = 5i32;
pub const DISPID_InkRecoAlternate_Descender: i32 = 6i32;
pub const DISPID_InkRecoAlternate_Confidence: i32 = 7i32;
pub const DISPID_InkRecoAlternate_Strokes: i32 = 8i32;
pub const DISPID_InkRecoAlternate_GetStrokesFromStrokeRanges: i32 = 9i32;
pub const DISPID_InkRecoAlternate_GetStrokesFromTextRange: i32 = 10i32;
pub const DISPID_InkRecoAlternate_GetTextRangeFromStrokes: i32 = 11i32;
pub const DISPID_InkRecoAlternate_GetPropertyValue: i32 = 12i32;
pub const DISPID_InkRecoAlternate_LineAlternates: i32 = 13i32;
pub const DISPID_InkRecoAlternate_ConfidenceAlternates: i32 = 14i32;
pub const DISPID_InkRecoAlternate_AlternatesWithConstantPropertyValues: i32 = 15i32;
pub const DISPID_IRecoCtx_Strokes: i32 = 1i32;
pub const DISPID_IRecoCtx_CharacterAutoCompletionMode: i32 = 2i32;
pub const DISPID_IRecoCtx_Factoid: i32 = 3i32;
pub const DISPID_IRecoCtx_WordList: i32 = 4i32;
pub const DISPID_IRecoCtx_Recognizer: i32 = 5i32;
pub const DISPID_IRecoCtx_Guide: i32 = 6i32;
pub const DISPID_IRecoCtx_Flags: i32 = 7i32;
pub const DISPID_IRecoCtx_PrefixText: i32 = 8i32;
pub const DISPID_IRecoCtx_SuffixText: i32 = 9i32;
pub const DISPID_IRecoCtx_StopRecognition: i32 = 10i32;
pub const DISPID_IRecoCtx_Clone: i32 = 11i32;
pub const DISPID_IRecoCtx_Recognize: i32 = 12i32;
pub const DISPID_IRecoCtx_StopBackgroundRecognition: i32 = 13i32;
pub const DISPID_IRecoCtx_EndInkInput: i32 = 14i32;
pub const DISPID_IRecoCtx_BackgroundRecognize: i32 = 15i32;
pub const DISPID_IRecoCtx_BackgroundRecognizeWithAlternates: i32 = 16i32;
pub const DISPID_IRecoCtx_IsStringSupported: i32 = 17i32;
pub const DISPID_IRecoCtx2_EnabledUnicodeRanges: i32 = 0i32;
pub const DISPID_InkRecognitionAlternates_NewEnum: i32 = -4i32;
pub const DISPID_InkRecognitionAlternates_Item: i32 = 0i32;
pub const DISPID_InkRecognitionAlternates_Count: i32 = 1i32;
pub const DISPID_InkRecognitionAlternates_Strokes: i32 = 2i32;
pub const DISPID_IRERecognitionWithAlternates: i32 = 1i32;
pub const DISPID_IRERecognition: i32 = 2i32;
pub const DISPID_InkRecognitionResult_TopString: i32 = 1i32;
pub const DISPID_InkRecognitionResult_TopAlternate: i32 = 2i32;
pub const DISPID_InkRecognitionResult_Strokes: i32 = 3i32;
pub const DISPID_InkRecognitionResult_TopConfidence: i32 = 4i32;
pub const DISPID_InkRecognitionResult_AlternatesFromSelection: i32 = 5i32;
pub const DISPID_InkRecognitionResult_ModifyTopAlternate: i32 = 6i32;
pub const DISPID_InkRecognitionResult_SetResultOnStrokes: i32 = 7i32;
pub const DISPID_RecoClsid: i32 = 1i32;
pub const DISPID_RecoName: i32 = 2i32;
pub const DISPID_RecoVendor: i32 = 3i32;
pub const DISPID_RecoCapabilities: i32 = 4i32;
pub const DISPID_RecoLanguageID: i32 = 5i32;
pub const DISPID_RecoPreferredPacketDescription: i32 = 6i32;
pub const DISPID_RecoCreateRecognizerContext: i32 = 7i32;
pub const DISPID_RecoSupportedProperties: i32 = 8i32;
pub const DISPID_RecoId: i32 = 0i32;
pub const DISPID_RecoUnicodeRanges: i32 = 1i32;
pub const DISPID_IRGWritingBox: i32 = 1i32;
pub const DISPID_IRGDrawnBox: i32 = 2i32;
pub const DISPID_IRGRows: i32 = 3i32;
pub const DISPID_IRGColumns: i32 = 4i32;
pub const DISPID_IRGMidline: i32 = 5i32;
pub const DISPID_IRGGuideData: i32 = 6i32;
pub const DISPID_IRecos_NewEnum: i32 = -4i32;
pub const DISPID_IRecosItem: i32 = 0i32;
pub const DISPID_IRecosCount: i32 = 1i32;
pub const DISPID_IRecosGetDefaultRecognizer: i32 = 2i32;
pub const DISPID_IRTop: i32 = 1i32;
pub const DISPID_IRLeft: i32 = 2i32;
pub const DISPID_IRBottom: i32 = 3i32;
pub const DISPID_IRRight: i32 = 4i32;
pub const DISPID_IRGetRectangle: i32 = 5i32;
pub const DISPID_IRSetRectangle: i32 = 6i32;
pub const DISPID_IRData: i32 = 7i32;
pub const DISPID_IRGetViewTransform: i32 = 1i32;
pub const DISPID_IRSetViewTransform: i32 = 2i32;
pub const DISPID_IRGetObjectTransform: i32 = 3i32;
pub const DISPID_IRSetObjectTransform: i32 = 4i32;
pub const DISPID_IRDraw: i32 = 5i32;
pub const DISPID_IRDrawStroke: i32 = 6i32;
pub const DISPID_IRPixelToInkSpace: i32 = 7i32;
pub const DISPID_IRInkSpaceToPixel: i32 = 8i32;
pub const DISPID_IRPixelToInkSpaceFromPoints: i32 = 9i32;
pub const DISPID_IRInkSpaceToPixelFromPoints: i32 = 10i32;
pub const DISPID_IRMeasure: i32 = 11i32;
pub const DISPID_IRMeasureStroke: i32 = 12i32;
pub const DISPID_IRMove: i32 = 13i32;
pub const DISPID_IRRotate: i32 = 14i32;
pub const DISPID_IRScale: i32 = 15i32;
pub const DISPID_ISDInkIndex: i32 = 1i32;
pub const DISPID_ISDID: i32 = 2i32;
pub const DISPID_ISDGetBoundingBox: i32 = 3i32;
pub const DISPID_ISDDrawingAttributes: i32 = 4i32;
pub const DISPID_ISDFindIntersections: i32 = 5i32;
pub const DISPID_ISDGetRectangleIntersections: i32 = 6i32;
pub const DISPID_ISDClip: i32 = 7i32;
pub const DISPID_ISDHitTestCircle: i32 = 8i32;
pub const DISPID_ISDNearestPoint: i32 = 9i32;
pub const DISPID_ISDSplit: i32 = 10i32;
pub const DISPID_ISDExtendedProperties: i32 = 11i32;
pub const DISPID_ISDInk: i32 = 12i32;
pub const DISPID_ISDBezierPoints: i32 = 13i32;
pub const DISPID_ISDPolylineCusps: i32 = 14i32;
pub const DISPID_ISDBezierCusps: i32 = 15i32;
pub const DISPID_ISDSelfIntersections: i32 = 16i32;
pub const DISPID_ISDPacketCount: i32 = 17i32;
pub const DISPID_ISDPacketSize: i32 = 18i32;
pub const DISPID_ISDPacketDescription: i32 = 19i32;
pub const DISPID_ISDDeleted: i32 = 20i32;
pub const DISPID_ISDGetPacketDescriptionPropertyMetrics: i32 = 21i32;
pub const DISPID_ISDGetPoints: i32 = 22i32;
pub const DISPID_ISDSetPoints: i32 = 23i32;
pub const DISPID_ISDGetPacketData: i32 = 24i32;
pub const DISPID_ISDGetPacketValuesByProperty: i32 = 25i32;
pub const DISPID_ISDSetPacketValuesByProperty: i32 = 26i32;
pub const DISPID_ISDGetFlattenedBezierPoints: i32 = 27i32;
pub const DISPID_ISDScaleToRectangle: i32 = 28i32;
pub const DISPID_ISDTransform: i32 = 29i32;
pub const DISPID_ISDMove: i32 = 30i32;
pub const DISPID_ISDRotate: i32 = 31i32;
pub const DISPID_ISDShear: i32 = 32i32;
pub const DISPID_ISDScale: i32 = 33i32;
pub const DISPID_ISs_NewEnum: i32 = -4i32;
pub const DISPID_ISsItem: i32 = 0i32;
pub const DISPID_ISsCount: i32 = 1i32;
pub const DISPID_ISsValid: i32 = 2i32;
pub const DISPID_ISsInk: i32 = 3i32;
pub const DISPID_ISsAdd: i32 = 4i32;
pub const DISPID_ISsAddStrokes: i32 = 5i32;
pub const DISPID_ISsRemove: i32 = 6i32;
pub const DISPID_ISsRemoveStrokes: i32 = 7i32;
pub const DISPID_ISsToString: i32 = 8i32;
pub const DISPID_ISsModifyDrawingAttributes: i32 = 9i32;
pub const DISPID_ISsGetBoundingBox: i32 = 10i32;
pub const DISPID_ISsScaleToRectangle: i32 = 11i32;
pub const DISPID_ISsTransform: i32 = 12i32;
pub const DISPID_ISsMove: i32 = 13i32;
pub const DISPID_ISsRotate: i32 = 14i32;
pub const DISPID_ISsShear: i32 = 15i32;
pub const DISPID_ISsScale: i32 = 16i32;
pub const DISPID_ISsClip: i32 = 17i32;
pub const DISPID_ISsRecognitionResult: i32 = 18i32;
pub const DISPID_ISsRemoveRecognitionResult: i32 = 19i32;
pub const DISPID_ITName: i32 = 0i32;
pub const DISPID_ITPlugAndPlayId: i32 = 1i32;
pub const DISPID_ITPropertyMetrics: i32 = 2i32;
pub const DISPID_ITIsPacketPropertySupported: i32 = 3i32;
pub const DISPID_ITMaximumInputRectangle: i32 = 4i32;
pub const DISPID_ITHardwareCapabilities: i32 = 5i32;
pub const DISPID_IT2DeviceKind: i32 = 0i32;
pub const DISPID_IT3IsMultiTouch: i32 = 0i32;
pub const DISPID_IT3MaximumCursors: i32 = 1i32;
pub const DISPID_ITs_NewEnum: i32 = -4i32;
pub const DISPID_ITsItem: i32 = 0i32;
pub const DISPID_ITsDefaultTablet: i32 = 1i32;
pub const DISPID_ITsCount: i32 = 2i32;
pub const DISPID_ITsIsPacketPropertySupported: i32 = 3i32;
pub const DISPID_ITReset: i32 = 1i32;
pub const DISPID_ITTranslate: i32 = 2i32;
pub const DISPID_ITRotate: i32 = 3i32;
pub const DISPID_ITReflect: i32 = 4i32;
pub const DISPID_ITShear: i32 = 5i32;
pub const DISPID_ITScale: i32 = 6i32;
pub const DISPID_ITeM11: i32 = 7i32;
pub const DISPID_ITeM12: i32 = 8i32;
pub const DISPID_ITeM21: i32 = 9i32;
pub const DISPID_ITeM22: i32 = 10i32;
pub const DISPID_ITeDx: i32 = 11i32;
pub const DISPID_ITeDy: i32 = 12i32;
pub const DISPID_ITGetTransform: i32 = 13i32;
pub const DISPID_ITSetTransform: i32 = 14i32;
pub const DISPID_ITData: i32 = 15i32;
pub const DISPID_InkWordList_AddWord: i32 = 0i32;
pub const DISPID_InkWordList_RemoveWord: i32 = 1i32;
pub const DISPID_InkWordList_Merge: i32 = 2i32;
pub const DISPID_InkWordList2_AddWords: i32 = 3i32;
pub const DISPID_MICInsert: i32 = 0i32;
pub const DISPID_MICClose: i32 = 1i32;
pub const DISPID_MICPaint: i32 = 2i32;
pub const DISPID_MICClear: i32 = 3i32;
pub const DISPID_PIPAttachedEditWindow: i32 = 0i32;
pub const DISPID_PIPFactoid: i32 = 1i32;
pub const DISPID_PIPCurrentPanel: i32 = 2i32;
pub const DISPID_PIPDefaultPanel: i32 = 3i32;
pub const DISPID_PIPVisible: i32 = 4i32;
pub const DISPID_PIPTop: i32 = 5i32;
pub const DISPID_PIPLeft: i32 = 6i32;
pub const DISPID_PIPWidth: i32 = 7i32;
pub const DISPID_PIPHeight: i32 = 8i32;
pub const DISPID_PIPMoveTo: i32 = 9i32;
pub const DISPID_PIPCommitPendingInput: i32 = 10i32;
pub const DISPID_PIPRefresh: i32 = 11i32;
pub const DISPID_PIPBusy: i32 = 12i32;
pub const DISPID_PIPVerticalOffset: i32 = 13i32;
pub const DISPID_PIPHorizontalOffset: i32 = 14i32;
pub const DISPID_PIPEnableTsf: i32 = 15i32;
pub const DISPID_PIPAutoShow: i32 = 16i32;
pub const DISPID_PIPEVisibleChanged: i32 = 0i32;
pub const DISPID_PIPEPanelChanged: i32 = 1i32;
pub const DISPID_PIPEInputFailed: i32 = 2i32;
pub const DISPID_PIPEPanelMoving: i32 = 3i32;
pub const DISPID_SEStrokesAdded: i32 = 1i32;
pub const DISPID_SEStrokesRemoved: i32 = 2i32;
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
pub const EventMask_InPlaceStateChanging: i32 = 1i32;
pub const EventMask_InPlaceStateChanged: i32 = 2i32;
pub const EventMask_InPlaceSizeChanging: i32 = 4i32;
pub const EventMask_InPlaceSizeChanged: i32 = 8i32;
pub const EventMask_InputAreaChanging: i32 = 16i32;
pub const EventMask_InputAreaChanged: i32 = 32i32;
pub const EventMask_CorrectionModeChanging: i32 = 64i32;
pub const EventMask_CorrectionModeChanged: i32 = 128i32;
pub const EventMask_InPlaceVisibilityChanging: i32 = 256i32;
pub const EventMask_InPlaceVisibilityChanged: i32 = 512i32;
pub const EventMask_TextInserting: i32 = 1024i32;
pub const EventMask_TextInserted: i32 = 2048i32;
pub const EventMask_All: i32 = 4095i32;
pub const FACILITY_INK: u32 = 40u32;
pub const FLICKACTION_COMMANDCODE_NULL: i32 = 0i32;
pub const FLICKACTION_COMMANDCODE_SCROLL: i32 = 1i32;
pub const FLICKACTION_COMMANDCODE_APPCOMMAND: i32 = 2i32;
pub const FLICKACTION_COMMANDCODE_CUSTOMKEY: i32 = 3i32;
pub const FLICKACTION_COMMANDCODE_KEYMODIFIER: i32 = 4i32;
pub const FLICKDIRECTION_MIN: i32 = 0i32;
pub const FLICKDIRECTION_RIGHT: i32 = 0i32;
pub const FLICKDIRECTION_UPRIGHT: i32 = 1i32;
pub const FLICKDIRECTION_UP: i32 = 2i32;
pub const FLICKDIRECTION_UPLEFT: i32 = 3i32;
pub const FLICKDIRECTION_LEFT: i32 = 4i32;
pub const FLICKDIRECTION_DOWNLEFT: i32 = 5i32;
pub const FLICKDIRECTION_DOWN: i32 = 6i32;
pub const FLICKDIRECTION_DOWNRIGHT: i32 = 7i32;
pub const FLICKDIRECTION_INVALID: i32 = 8i32;
pub const FLICKMODE_MIN: i32 = 0i32;
pub const FLICKMODE_OFF: i32 = 0i32;
pub const FLICKMODE_ON: i32 = 1i32;
pub const FLICKMODE_LEARNING: i32 = 2i32;
pub const FLICKMODE_MAX: i32 = 2i32;
pub const FLICKMODE_DEFAULT: i32 = 1i32;
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
pub type HRECOALT = isize;
pub type HRECOCONTEXT = isize;
pub type HRECOGNIZER = isize;
pub type HRECOLATTICE = isize;
pub type HRECOWORDLIST = isize;
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
pub const InPlaceDirection_Auto: i32 = 0i32;
pub const InPlaceDirection_Bottom: i32 = 1i32;
pub const InPlaceDirection_Top: i32 = 2i32;
pub const InPlaceState_Auto: i32 = 0i32;
pub const InPlaceState_HoverTarget: i32 = 1i32;
pub const InPlaceState_Expanded: i32 = 2i32;
pub const Ink: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 333335106,
    data2: 36129,
    data3: 19598,
    data4: [191, 156, 143, 105, 203, 6, 143, 202],
};
pub const IAG_AllGestures: i32 = 0i32;
pub const IAG_NoGesture: i32 = 61440i32;
pub const IAG_Scratchout: i32 = 61441i32;
pub const IAG_Triangle: i32 = 61442i32;
pub const IAG_Square: i32 = 61443i32;
pub const IAG_Star: i32 = 61444i32;
pub const IAG_Check: i32 = 61445i32;
pub const IAG_Curlicue: i32 = 61456i32;
pub const IAG_DoubleCurlicue: i32 = 61457i32;
pub const IAG_Circle: i32 = 61472i32;
pub const IAG_DoubleCircle: i32 = 61473i32;
pub const IAG_SemiCircleLeft: i32 = 61480i32;
pub const IAG_SemiCircleRight: i32 = 61481i32;
pub const IAG_ChevronUp: i32 = 61488i32;
pub const IAG_ChevronDown: i32 = 61489i32;
pub const IAG_ChevronLeft: i32 = 61490i32;
pub const IAG_ChevronRight: i32 = 61491i32;
pub const IAG_ArrowUp: i32 = 61496i32;
pub const IAG_ArrowDown: i32 = 61497i32;
pub const IAG_ArrowLeft: i32 = 61498i32;
pub const IAG_ArrowRight: i32 = 61499i32;
pub const IAG_Up: i32 = 61528i32;
pub const IAG_Down: i32 = 61529i32;
pub const IAG_Left: i32 = 61530i32;
pub const IAG_Right: i32 = 61531i32;
pub const IAG_UpDown: i32 = 61536i32;
pub const IAG_DownUp: i32 = 61537i32;
pub const IAG_LeftRight: i32 = 61538i32;
pub const IAG_RightLeft: i32 = 61539i32;
pub const IAG_UpLeftLong: i32 = 61540i32;
pub const IAG_UpRightLong: i32 = 61541i32;
pub const IAG_DownLeftLong: i32 = 61542i32;
pub const IAG_DownRightLong: i32 = 61543i32;
pub const IAG_UpLeft: i32 = 61544i32;
pub const IAG_UpRight: i32 = 61545i32;
pub const IAG_DownLeft: i32 = 61546i32;
pub const IAG_DownRight: i32 = 61547i32;
pub const IAG_LeftUp: i32 = 61548i32;
pub const IAG_LeftDown: i32 = 61549i32;
pub const IAG_RightUp: i32 = 61550i32;
pub const IAG_RightDown: i32 = 61551i32;
pub const IAG_Exclamation: i32 = 61604i32;
pub const IAG_Tap: i32 = 61680i32;
pub const IAG_DoubleTap: i32 = 61681i32;
pub const IBBM_Default: i32 = 0i32;
pub const IBBM_NoCurveFit: i32 = 1i32;
pub const IBBM_CurveFit: i32 = 2i32;
pub const IBBM_PointsOnly: i32 = 3i32;
pub const IBBM_Union: i32 = 4i32;
pub const ICF_None: i32 = 0i32;
pub const ICF_InkSerializedFormat: i32 = 1i32;
pub const ICF_SketchInk: i32 = 2i32;
pub const ICF_TextInk: i32 = 6i32;
pub const ICF_EnhancedMetafile: i32 = 8i32;
pub const ICF_Metafile: i32 = 32i32;
pub const ICF_Bitmap: i32 = 64i32;
pub const ICF_PasteMask: i32 = 7i32;
pub const ICF_CopyMask: i32 = 127i32;
pub const ICF_Default: i32 = 127i32;
pub const ICB_Copy: i32 = 0i32;
pub const ICB_Cut: i32 = 1i32;
pub const ICB_ExtractOnly: i32 = 48i32;
pub const ICB_DelayedCopy: i32 = 32i32;
pub const ICB_Default: i32 = 0i32;
pub const ICM_InkOnly: i32 = 0i32;
pub const ICM_GestureOnly: i32 = 1i32;
pub const ICM_InkAndGesture: i32 = 2i32;
pub const InkCollector: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1140528467,
    data2: 44404,
    data3: 20200,
    data4: [136, 228, 62, 109, 170, 201, 21, 219],
};
pub const InkCollectorClipInkToMargin: i32 = 0i32;
pub const InkCollectorDefaultMargin: i32 = -2147483648i32;
pub const ICEI_DefaultEvents: i32 = -1i32;
pub const ICEI_CursorDown: i32 = 0i32;
pub const ICEI_Stroke: i32 = 1i32;
pub const ICEI_NewPackets: i32 = 2i32;
pub const ICEI_NewInAirPackets: i32 = 3i32;
pub const ICEI_CursorButtonDown: i32 = 4i32;
pub const ICEI_CursorButtonUp: i32 = 5i32;
pub const ICEI_CursorInRange: i32 = 6i32;
pub const ICEI_CursorOutOfRange: i32 = 7i32;
pub const ICEI_SystemGesture: i32 = 8i32;
pub const ICEI_TabletAdded: i32 = 9i32;
pub const ICEI_TabletRemoved: i32 = 10i32;
pub const ICEI_MouseDown: i32 = 11i32;
pub const ICEI_MouseMove: i32 = 12i32;
pub const ICEI_MouseUp: i32 = 13i32;
pub const ICEI_MouseWheel: i32 = 14i32;
pub const ICEI_DblClick: i32 = 15i32;
pub const ICEI_AllEvents: i32 = 16i32;
pub const ICBS_Unavailable: i32 = 0i32;
pub const ICBS_Up: i32 = 1i32;
pub const ICBS_Down: i32 = 2i32;
pub const InkDisp: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2474383924,
    data2: 5405,
    data3: 17936,
    data4: [156, 166, 168, 204, 155, 219, 93, 131],
};
pub const IDM_Ink: i32 = 0i32;
pub const IDM_Text: i32 = 1i32;
pub const InkDivider: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2287269536,
    data2: 18051,
    data3: 19175,
    data4: [145, 145, 117, 47, 230, 70, 18, 195],
};
pub const IDT_Segment: i32 = 0i32;
pub const IDT_Line: i32 = 1i32;
pub const IDT_Paragraph: i32 = 2i32;
pub const IDT_Drawing: i32 = 3i32;
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
pub const IES_Idle: i32 = 0i32;
pub const IES_Collecting: i32 = 1i32;
pub const IES_Recognizing: i32 = 2i32;
pub const IEF_CopyFromOriginal: i32 = 0i32;
pub const IEF_RemoveFromOriginal: i32 = 1i32;
pub const IEF_Default: i32 = 1i32;
pub const IEM_InsertText: i32 = 0i32;
pub const IEM_InsertInk: i32 = 1i32;
pub const InkMaxTransparencyValue: i32 = 255i32;
pub const InkMinTransparencyValue: i32 = 0i32;
pub const IEM_Disabled: i32 = 0i32;
pub const IEM_Ink: i32 = 1i32;
pub const IEM_InkAndGesture: i32 = 2i32;
pub const IMF_Left: i32 = 1i32;
pub const IMF_Right: i32 = 2i32;
pub const IMF_Middle: i32 = 4i32;
pub const IMP_Default: i32 = 0i32;
pub const IMP_Arrow: i32 = 1i32;
pub const IMP_Crosshair: i32 = 2i32;
pub const IMP_Ibeam: i32 = 3i32;
pub const IMP_SizeNESW: i32 = 4i32;
pub const IMP_SizeNS: i32 = 5i32;
pub const IMP_SizeNWSE: i32 = 6i32;
pub const IMP_SizeWE: i32 = 7i32;
pub const IMP_UpArrow: i32 = 8i32;
pub const IMP_Hourglass: i32 = 9i32;
pub const IMP_NoDrop: i32 = 10i32;
pub const IMP_ArrowHourglass: i32 = 11i32;
pub const IMP_ArrowQuestion: i32 = 12i32;
pub const IMP_SizeAll: i32 = 13i32;
pub const IMP_Hand: i32 = 14i32;
pub const IMP_Custom: i32 = 99i32;
pub const InkOverlay: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1708131910,
    data2: 52707,
    data3: 19080,
    data4: [145, 99, 103, 105, 240, 241, 169, 125],
};
pub const IOAM_Behind: i32 = 0i32;
pub const IOAM_InFront: i32 = 1i32;
pub const IOEM_Ink: i32 = 0i32;
pub const IOEM_Delete: i32 = 1i32;
pub const IOEM_Select: i32 = 2i32;
pub const IOERM_StrokeErase: i32 = 0i32;
pub const IOERM_PointErase: i32 = 1i32;
pub const IPT_Ball: i32 = 0i32;
pub const IPT_Rectangle: i32 = 1i32;
pub const IPCM_Default: i32 = 0i32;
pub const IPCM_MaximumCompression: i32 = 1i32;
pub const IPCM_NoCompression: i32 = 2i32;
pub const IPF_InkSerializedFormat: i32 = 0i32;
pub const IPF_Base64InkSerializedFormat: i32 = 1i32;
pub const IPF_GIF: i32 = 2i32;
pub const IPF_Base64GIF: i32 = 3i32;
pub const InkPicture: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 77718867, data2: 65078, data3: 20446, data4: [134, 94, 52, 65, 148, 230, 148, 36] };
pub const IPSM_AutoSize: i32 = 0i32;
pub const IPSM_CenterImage: i32 = 1i32;
pub const IPSM_Normal: i32 = 2i32;
pub const IPSM_StretchImage: i32 = 3i32;
pub const IRO_Black: i32 = 1i32;
pub const IRO_NotMergePen: i32 = 2i32;
pub const IRO_MaskNotPen: i32 = 3i32;
pub const IRO_NotCopyPen: i32 = 4i32;
pub const IRO_MaskPenNot: i32 = 5i32;
pub const IRO_Not: i32 = 6i32;
pub const IRO_XOrPen: i32 = 7i32;
pub const IRO_NotMaskPen: i32 = 8i32;
pub const IRO_MaskPen: i32 = 9i32;
pub const IRO_NotXOrPen: i32 = 10i32;
pub const IRO_NoOperation: i32 = 11i32;
pub const IRO_MergeNotPen: i32 = 12i32;
pub const IRO_CopyPen: i32 = 13i32;
pub const IRO_MergePenNot: i32 = 14i32;
pub const IRO_MergePen: i32 = 15i32;
pub const IRO_White: i32 = 16i32;
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
pub const IRAS_Start: i32 = 0i32;
pub const IRAS_DefaultCount: i32 = 10i32;
pub const IRAS_All: i32 = -1i32;
pub const IRC_Strong: i32 = 0i32;
pub const IRC_Intermediate: i32 = 1i32;
pub const IRC_Poor: i32 = 2i32;
pub const IRM_None: i32 = 0i32;
pub const IRM_WordModeOnly: i32 = 1i32;
pub const IRM_Coerce: i32 = 2i32;
pub const IRM_TopInkBreaksOnly: i32 = 4i32;
pub const IRM_PrefixOk: i32 = 8i32;
pub const IRM_LineMode: i32 = 16i32;
pub const IRM_DisablePersonalization: i32 = 32i32;
pub const IRM_AutoSpace: i32 = 64i32;
pub const IRM_Max: i32 = 128i32;
pub const IRS_NoError: i32 = 0i32;
pub const IRS_Interrupted: i32 = 1i32;
pub const IRS_ProcessFailed: i32 = 2i32;
pub const IRS_InkAddedFailed: i32 = 4i32;
pub const IRS_SetAutoCompletionModeFailed: i32 = 8i32;
pub const IRS_SetStrokesFailed: i32 = 16i32;
pub const IRS_SetGuideFailed: i32 = 32i32;
pub const IRS_SetFlagsFailed: i32 = 64i32;
pub const IRS_SetFactoidFailed: i32 = 128i32;
pub const IRS_SetPrefixSuffixFailed: i32 = 256i32;
pub const IRS_SetWordListFailed: i32 = 512i32;
pub const IRC_DontCare: i32 = 1i32;
pub const IRC_Object: i32 = 2i32;
pub const IRC_FreeInput: i32 = 4i32;
pub const IRC_LinedInput: i32 = 8i32;
pub const IRC_BoxedInput: i32 = 16i32;
pub const IRC_CharacterAutoCompletionInput: i32 = 32i32;
pub const IRC_RightAndDown: i32 = 64i32;
pub const IRC_LeftAndDown: i32 = 128i32;
pub const IRC_DownAndLeft: i32 = 256i32;
pub const IRC_DownAndRight: i32 = 512i32;
pub const IRC_ArbitraryAngle: i32 = 1024i32;
pub const IRC_Lattice: i32 = 2048i32;
pub const IRC_AdviseInkChange: i32 = 4096i32;
pub const IRC_StrokeReorder: i32 = 8192i32;
pub const IRC_Personalizable: i32 = 16384i32;
pub const IRC_PrefersArbitraryAngle: i32 = 32768i32;
pub const IRC_PrefersParagraphBreaking: i32 = 65536i32;
pub const IRC_PrefersSegmentation: i32 = 131072i32;
pub const IRC_Cursive: i32 = 262144i32;
pub const IRC_TextPrediction: i32 = 524288i32;
pub const IRC_Alpha: i32 = 1048576i32;
pub const IRC_Beta: i32 = 2097152i32;
pub const IRCACM_Full: i32 = 0i32;
pub const IRCACM_Prefix: i32 = 1i32;
pub const IRCACM_Random: i32 = 2i32;
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
pub const ISC_FirstElement: i32 = 0i32;
pub const ISC_AllElements: i32 = -1i32;
pub const IKM_Shift: i32 = 1i32;
pub const IKM_Control: i32 = 2i32;
pub const IKM_Alt: i32 = 4i32;
pub const InkStrokes: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1223987644, data2: 9230, data3: 18528, data4: [176, 121, 161, 233, 77, 61, 44, 134] };
pub const ISG_Tap: i32 = 16i32;
pub const ISG_DoubleTap: i32 = 17i32;
pub const ISG_RightTap: i32 = 18i32;
pub const ISG_Drag: i32 = 19i32;
pub const ISG_RightDrag: i32 = 20i32;
pub const ISG_HoldEnter: i32 = 21i32;
pub const ISG_HoldLeave: i32 = 22i32;
pub const ISG_HoverEnter: i32 = 23i32;
pub const ISG_HoverLeave: i32 = 24i32;
pub const ISG_Flick: i32 = 31i32;
pub const InkTablets: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1850723090, data2: 20746, data3: 19776, data4: [147, 4, 29, 161, 10, 233, 20, 124] };
pub const InkTransform: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3822442812, data2: 5731, data3: 19064, data4: [161, 167, 34, 55, 93, 254, 186, 238] };
pub const InkWordList: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2649247892,
    data2: 63263,
    data3: 17649,
    data4: [132, 113, 21, 162, 250, 118, 252, 243],
};
pub const InteractionMode_InPlace: i32 = 0i32;
pub const InteractionMode_Floating: i32 = 1i32;
pub const InteractionMode_DockedTop: i32 = 2i32;
pub const InteractionMode_DockedBottom: i32 = 3i32;
pub const KEYMODIFIER_CONTROL: i32 = 1i32;
pub const KEYMODIFIER_MENU: i32 = 2i32;
pub const KEYMODIFIER_SHIFT: i32 = 4i32;
pub const KEYMODIFIER_WIN: i32 = 8i32;
pub const KEYMODIFIER_ALTGR: i32 = 16i32;
pub const KEYMODIFIER_EXT: i32 = 32i32;
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
pub const LM_BASELINE: i32 = 0i32;
pub const LM_MIDLINE: i32 = 1i32;
pub const LM_ASCENDER: i32 = 2i32;
pub const LM_DESCENDER: i32 = 3i32;
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
pub const MICUIELEMENT_BUTTON_WRITE: i32 = 1i32;
pub const MICUIELEMENT_BUTTON_ERASE: i32 = 2i32;
pub const MICUIELEMENT_BUTTON_CORRECT: i32 = 4i32;
pub const MICUIELEMENT_BUTTON_CLEAR: i32 = 8i32;
pub const MICUIELEMENT_BUTTON_UNDO: i32 = 16i32;
pub const MICUIELEMENT_BUTTON_REDO: i32 = 32i32;
pub const MICUIELEMENT_BUTTON_INSERT: i32 = 64i32;
pub const MICUIELEMENT_BUTTON_CANCEL: i32 = 128i32;
pub const MICUIELEMENT_INKPANEL_BACKGROUND: i32 = 256i32;
pub const MICUIELEMENT_RESULTPANEL_BACKGROUND: i32 = 512i32;
pub const MICUIELEMENTSTATE_NORMAL: i32 = 1i32;
pub const MICUIELEMENTSTATE_HOT: i32 = 2i32;
pub const MICUIELEMENTSTATE_PRESSED: i32 = 3i32;
pub const MICUIELEMENTSTATE_DISABLED: i32 = 4i32;
pub const MathInputControl: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3311501676,
    data2: 5336,
    data3: 16528,
    data4: [131, 12, 152, 217, 148, 178, 28, 123],
};
pub const NO_BUTTON: i32 = 0i32;
pub const LEFT_BUTTON: i32 = 1i32;
pub const RIGHT_BUTTON: i32 = 2i32;
pub const MIDDLE_BUTTON: i32 = 4i32;
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
pub const PROPERTY_UNITS_DEFAULT: i32 = 0i32;
pub const PROPERTY_UNITS_INCHES: i32 = 1i32;
pub const PROPERTY_UNITS_CENTIMETERS: i32 = 2i32;
pub const PROPERTY_UNITS_DEGREES: i32 = 3i32;
pub const PROPERTY_UNITS_RADIANS: i32 = 4i32;
pub const PROPERTY_UNITS_SECONDS: i32 = 5i32;
pub const PROPERTY_UNITS_POUNDS: i32 = 6i32;
pub const PROPERTY_UNITS_GRAMS: i32 = 7i32;
pub const PROPERTY_UNITS_SILINEAR: i32 = 8i32;
pub const PROPERTY_UNITS_SIROTATION: i32 = 9i32;
pub const PROPERTY_UNITS_ENGLINEAR: i32 = 10i32;
pub const PROPERTY_UNITS_ENGROTATION: i32 = 11i32;
pub const PROPERTY_UNITS_SLUGS: i32 = 12i32;
pub const PROPERTY_UNITS_KELVIN: i32 = 13i32;
pub const PROPERTY_UNITS_FAHRENHEIT: i32 = 14i32;
pub const PROPERTY_UNITS_AMPERE: i32 = 15i32;
pub const PROPERTY_UNITS_CANDELA: i32 = 16i32;
pub const PanelInputArea_Auto: i32 = 0i32;
pub const PanelInputArea_Keyboard: i32 = 1i32;
pub const PanelInputArea_WritingPad: i32 = 2i32;
pub const PanelInputArea_CharacterPad: i32 = 3i32;
pub const PT_Default: i32 = 0i32;
pub const PT_Inactive: i32 = 1i32;
pub const PT_Handwriting: i32 = 2i32;
pub const PT_Keyboard: i32 = 3i32;
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
pub const RTSDI_AllData: i32 = -1i32;
pub const RTSDI_None: i32 = 0i32;
pub const RTSDI_Error: i32 = 1i32;
pub const RTSDI_RealTimeStylusEnabled: i32 = 2i32;
pub const RTSDI_RealTimeStylusDisabled: i32 = 4i32;
pub const RTSDI_StylusNew: i32 = 8i32;
pub const RTSDI_StylusInRange: i32 = 16i32;
pub const RTSDI_InAirPackets: i32 = 32i32;
pub const RTSDI_StylusOutOfRange: i32 = 64i32;
pub const RTSDI_StylusDown: i32 = 128i32;
pub const RTSDI_Packets: i32 = 256i32;
pub const RTSDI_StylusUp: i32 = 512i32;
pub const RTSDI_StylusButtonUp: i32 = 1024i32;
pub const RTSDI_StylusButtonDown: i32 = 2048i32;
pub const RTSDI_SystemEvents: i32 = 4096i32;
pub const RTSDI_TabletAdded: i32 = 8192i32;
pub const RTSDI_TabletRemoved: i32 = 16384i32;
pub const RTSDI_CustomStylusDataAdded: i32 = 32768i32;
pub const RTSDI_UpdateMapping: i32 = 65536i32;
pub const RTSDI_DefaultEvents: i32 = 37766i32;
pub const RTSLT_ObjLock: i32 = 1i32;
pub const RTSLT_SyncEventLock: i32 = 2i32;
pub const RTSLT_AsyncEventLock: i32 = 4i32;
pub const RTSLT_ExcludeCallback: i32 = 8i32;
pub const RTSLT_SyncObjLock: i32 = 11i32;
pub const RTSLT_AsyncObjLock: i32 = 13i32;
pub const SAFE_PARTIAL: u32 = 1u32;
pub const SCROLLDIRECTION_UP: i32 = 0i32;
pub const SCROLLDIRECTION_DOWN: i32 = 1i32;
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
pub const rtfNone: i32 = 0i32;
pub const rtfHorizontal: i32 = 1i32;
pub const rtfVertical: i32 = 2i32;
pub const rtfBoth: i32 = 3i32;
pub const rtfLeft: i32 = 0i32;
pub const rtfRight: i32 = 1i32;
pub const rtfCenter: i32 = 2i32;
pub const SHR_None: i32 = 0i32;
pub const SHR_NW: i32 = 1i32;
pub const SHR_SE: i32 = 2i32;
pub const SHR_NE: i32 = 3i32;
pub const SHR_SW: i32 = 4i32;
pub const SHR_E: i32 = 5i32;
pub const SHR_W: i32 = 6i32;
pub const SHR_N: i32 = 7i32;
pub const SHR_S: i32 = 8i32;
pub const SHR_Selection: i32 = 9i32;
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
pub const SyncStylusQueue: i32 = 1i32;
pub const AsyncStylusQueueImmediate: i32 = 2i32;
pub const AsyncStylusQueue: i32 = 3i32;
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
pub const TDK_Mouse: i32 = 0i32;
pub const TDK_Pen: i32 = 1i32;
pub const TDK_Touch: i32 = 2i32;
pub const THWC_Integrated: i32 = 1i32;
pub const THWC_CursorMustTouch: i32 = 2i32;
pub const THWC_HardProximity: i32 = 4i32;
pub const THWC_CursorsHavePhysicalIds: i32 = 8i32;
pub const TPMU_Default: i32 = 0i32;
pub const TPMU_Inches: i32 = 1i32;
pub const TPMU_Centimeters: i32 = 2i32;
pub const TPMU_Degrees: i32 = 3i32;
pub const TPMU_Radians: i32 = 4i32;
pub const TPMU_Seconds: i32 = 5i32;
pub const TPMU_Pounds: i32 = 6i32;
pub const TPMU_Grams: i32 = 7i32;
pub const TextInputPanel: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4189161943, data2: 8843, data3: 20267, data4: [134, 80, 185, 127, 89, 224, 44, 140] };
pub const TipAutoCompleteClient: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2155617900,
    data2: 7424,
    data3: 17727,
    data4: [185, 32, 182, 27, 183, 205, 217, 151],
};
pub const InPlace: i32 = 0i32;
pub const Floating: i32 = 1i32;
pub const DockedTop: i32 = 2i32;
pub const DockedBottom: i32 = 3i32;
pub const Closed: i32 = 4i32;
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
pub const TCF_ALLOW_RECOGNITION: i32 = 1i32;
pub const TCF_FORCE_RECOGNITION: i32 = 2i32;
pub const IMF_FONT_SELECTED_IN_HDC: i32 = 1i32;
pub const IMF_ITALIC: i32 = 2i32;
pub const IMF_BOLD: i32 = 4i32;
pub const RECO_TYPE_WSTRING: i32 = 0i32;
pub const RECO_TYPE_WCHAR: i32 = 1i32;
