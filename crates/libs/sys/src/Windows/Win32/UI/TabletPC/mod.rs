#[cfg(feature = "Win32_Graphics_Gdi")]
windows_link::link!("inkobjcore.dll" "system" fn AddStroke(hrc : HRECOCONTEXT, ppacketdesc : *const PACKET_DESCRIPTION, cbpacket : u32, ppacket : *const u8, pxform : *const super::super::Graphics::Gdi::XFORM) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn AddWordsToWordList(hwl : HRECOWORDLIST, pwcwords : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn AdviseInkChange(hrc : HRECOCONTEXT, bnewstroke : windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn CloneContext(hrc : HRECOCONTEXT, pclonehrc : *mut HRECOCONTEXT) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn CreateContext(hrec : HRECOGNIZER, phrc : *mut HRECOCONTEXT) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn CreateRecognizer(pclsid : *mut windows_sys::core::GUID, phrec : *mut HRECOGNIZER) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn DestroyAlternate(hrcalt : HRECOALT) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn DestroyContext(hrc : HRECOCONTEXT) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn DestroyRecognizer(hrec : HRECOGNIZER) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn DestroyWordList(hwl : HRECOWORDLIST) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn EndInkInput(hrc : HRECOCONTEXT) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetAllRecognizers(recognizerclsids : *mut *mut windows_sys::core::GUID, count : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetBestResultString(hrc : HRECOCONTEXT, pcsize : *mut u32, pwcbestresult : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetContextPreferenceFlags(hrc : HRECOCONTEXT, pdwcontextpreferenceflags : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetContextPropertyList(hrc : HRECOCONTEXT, pcproperties : *mut u32, ppropertyguids : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetContextPropertyValue(hrc : HRECOCONTEXT, pguid : *mut windows_sys::core::GUID, pcbsize : *mut u32, pproperty : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetEnabledUnicodeRanges(hrc : HRECOCONTEXT, pcranges : *mut u32, pcr : *mut CHARACTER_RANGE) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetGuide(hrc : HRECOCONTEXT, pguide : *mut RECO_GUIDE, piindex : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetLatticePtr(hrc : HRECOCONTEXT, pplattice : *mut *mut RECO_LATTICE) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetLeftSeparator(hrc : HRECOCONTEXT, pcsize : *mut u32, pwcleftseparator : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetPreferredPacketDescription(hrec : HRECOGNIZER, ppacketdescription : *mut PACKET_DESCRIPTION) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetRecoAttributes(hrec : HRECOGNIZER, precoattrs : *mut RECO_ATTRS) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetResultPropertyList(hrec : HRECOGNIZER, ppropertycount : *mut u32, ppropertyguid : *mut windows_sys::core::GUID) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetRightSeparator(hrc : HRECOCONTEXT, pcsize : *mut u32, pwcrightseparator : windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn GetUnicodeRanges(hrec : HRECOGNIZER, pcranges : *mut u32, pcr : *mut CHARACTER_RANGE) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn IsStringSupported(hrc : HRECOCONTEXT, wcstring : u32, pwcstring : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn LoadCachedAttributes(clsid : windows_sys::core::GUID, precoattributes : *mut RECO_ATTRS) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn MakeWordList(hrec : HRECOGNIZER, pbuffer : windows_sys::core::PCWSTR, phwl : *mut HRECOWORDLIST) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn Process(hrc : HRECOCONTEXT, pbpartialprocessing : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn ResetContext(hrc : HRECOCONTEXT) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn SetCACMode(hrc : HRECOCONTEXT, imode : i32) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn SetContextPropertyValue(hrc : HRECOCONTEXT, pguid : *mut windows_sys::core::GUID, cbsize : u32, pproperty : *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn SetEnabledUnicodeRanges(hrc : HRECOCONTEXT, cranges : u32, pcr : *mut CHARACTER_RANGE) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn SetFactoid(hrc : HRECOCONTEXT, cwcfactoid : u32, pwcfactoid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn SetFlags(hrc : HRECOCONTEXT, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn SetGuide(hrc : HRECOCONTEXT, pguide : *const RECO_GUIDE, iindex : u32) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn SetTextContext(hrc : HRECOCONTEXT, cwcbefore : u32, pwcbefore : windows_sys::core::PCWSTR, cwcafter : u32, pwcafter : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("inkobjcore.dll" "system" fn SetWordList(hrc : HRECOCONTEXT, hwl : HRECOWORDLIST) -> windows_sys::core::HRESULT);
pub type ALT_BREAKS = i32;
pub const ALT_BREAKS_FULL: ALT_BREAKS = 2;
pub const ALT_BREAKS_SAME: ALT_BREAKS = 0;
pub const ALT_BREAKS_UNIQUE: ALT_BREAKS = 1;
pub const ASYNC_RECO_ADDSTROKE_FAILED: u32 = 4;
pub const ASYNC_RECO_INTERRUPTED: u32 = 1;
pub const ASYNC_RECO_PROCESS_FAILED: u32 = 2;
pub const ASYNC_RECO_RESETCONTEXT_FAILED: u32 = 16;
pub const ASYNC_RECO_SETCACMODE_FAILED: u32 = 8;
pub const ASYNC_RECO_SETFACTOID_FAILED: u32 = 128;
pub const ASYNC_RECO_SETFLAGS_FAILED: u32 = 64;
pub const ASYNC_RECO_SETGUIDE_FAILED: u32 = 32;
pub const ASYNC_RECO_SETTEXTCONTEXT_FAILED: u32 = 256;
pub const ASYNC_RECO_SETWORDLIST_FAILED: u32 = 512;
pub type AppearanceConstants = i32;
pub const AsyncStylusQueue: StylusQueue = 3;
pub const AsyncStylusQueueImmediate: StylusQueue = 2;
pub const BEST_COMPLETE: u32 = 2;
pub type BorderStyleConstants = i32;
pub const CAC_FULL: u32 = 0;
pub const CAC_PREFIX: u32 = 1;
pub const CAC_RANDOM: u32 = 2;
pub const CFL_INTERMEDIATE: CONFIDENCE_LEVEL = 1;
pub const CFL_POOR: CONFIDENCE_LEVEL = 2;
pub const CFL_STRONG: CONFIDENCE_LEVEL = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CHARACTER_RANGE {
    pub wcLow: u16,
    pub cChars: u16,
}
pub type CONFIDENCE_LEVEL = i32;
pub const Closed: VisualState = 4;
pub type CorrectionMode = i32;
pub const CorrectionMode_NotVisible: CorrectionMode = 0;
pub const CorrectionMode_PostInsertionCollapsed: CorrectionMode = 2;
pub const CorrectionMode_PostInsertionExpanded: CorrectionMode = 3;
pub const CorrectionMode_PreInsertion: CorrectionMode = 1;
pub type CorrectionPosition = i32;
pub const CorrectionPosition_Auto: CorrectionPosition = 0;
pub const CorrectionPosition_Bottom: CorrectionPosition = 1;
pub const CorrectionPosition_Top: CorrectionPosition = 2;
pub const DISPID_DAAntiAliased: DISPID_InkDrawingAttributes = 6;
pub const DISPID_DAClone: DISPID_InkDrawingAttributes = 10;
pub const DISPID_DAColor: DISPID_InkDrawingAttributes = 2;
pub const DISPID_DAExtendedProperties: DISPID_InkDrawingAttributes = 11;
pub const DISPID_DAFitToCurve: DISPID_InkDrawingAttributes = 4;
pub const DISPID_DAHeight: DISPID_InkDrawingAttributes = 1;
pub const DISPID_DAIgnorePressure: DISPID_InkDrawingAttributes = 5;
pub const DISPID_DAPenTip: DISPID_InkDrawingAttributes = 9;
pub const DISPID_DARasterOperation: DISPID_InkDrawingAttributes = 8;
pub const DISPID_DATransparency: DISPID_InkDrawingAttributes = 7;
pub const DISPID_DAWidth: DISPID_InkDrawingAttributes = 3;
pub const DISPID_DisableNoScroll: DISPID_InkEdit = 3;
pub const DISPID_DragIcon: DISPID_InkEdit = 21;
pub const DISPID_DrawAttr: DISPID_InkEdit = 27;
pub const DISPID_Enabled: DISPID_InkEdit = 5;
pub const DISPID_Factoid: DISPID_InkEdit = 29;
pub const DISPID_GetGestStatus: DISPID_InkEdit = 33;
pub const DISPID_Hwnd: DISPID_InkEdit = 2;
pub const DISPID_IAddStrokesAtRectangle: DISPID_Ink = 17;
pub const DISPID_ICAutoRedraw: DISPID_InkCollector = 8;
pub const DISPID_ICBId: DISPID_InkCursorButton = 1;
pub const DISPID_ICBName: DISPID_InkCursorButton = 0;
pub const DISPID_ICBState: DISPID_InkCursorButton = 2;
pub const DISPID_ICBsCount: DISPID_InkCursorButtons = 1;
pub const DISPID_ICBsItem: DISPID_InkCursorButtons = 0;
pub const DISPID_ICBs_NewEnum: DISPID_InkCursorButtons = -4;
pub const DISPID_ICCollectingInk: DISPID_InkCollector = 9;
pub const DISPID_ICCollectionMode: DISPID_InkCollector = 28;
pub const DISPID_ICCursors: DISPID_InkCollector = 20;
pub const DISPID_ICDefaultDrawingAttributes: DISPID_InkCollector = 5;
pub const DISPID_ICDesiredPacketDescription: DISPID_InkCollector = 32;
pub const DISPID_ICDynamicRendering: DISPID_InkCollector = 31;
pub const DISPID_ICECursorButtonDown: DISPID_InkCollectorEvent = 5;
pub const DISPID_ICECursorButtonUp: DISPID_InkCollectorEvent = 6;
pub const DISPID_ICECursorDown: DISPID_InkCollectorEvent = 2;
pub const DISPID_ICECursorInRange: DISPID_InkCollectorEvent = 7;
pub const DISPID_ICECursorOutOfRange: DISPID_InkCollectorEvent = 8;
pub const DISPID_ICEGesture: DISPID_InkCollectorEvent = 10;
pub const DISPID_ICENewInAirPackets: DISPID_InkCollectorEvent = 4;
pub const DISPID_ICENewPackets: DISPID_InkCollectorEvent = 3;
pub const DISPID_ICEStroke: DISPID_InkCollectorEvent = 1;
pub const DISPID_ICESystemGesture: DISPID_InkCollectorEvent = 9;
pub const DISPID_ICETabletAdded: DISPID_InkCollectorEvent = 11;
pub const DISPID_ICETabletRemoved: DISPID_InkCollectorEvent = 12;
pub const DISPID_ICEnabled: DISPID_InkCollector = 1;
pub const DISPID_ICGetEventInterest: DISPID_InkCollector = 11;
pub const DISPID_ICGetGestureStatus: DISPID_InkCollector = 30;
pub const DISPID_ICGetWindowInputRectangle: DISPID_InkCollector = 24;
pub const DISPID_ICHwnd: DISPID_InkCollector = 2;
pub const DISPID_ICInk: DISPID_InkCollector = 7;
pub const DISPID_ICMarginX: DISPID_InkCollector = 21;
pub const DISPID_ICMarginY: DISPID_InkCollector = 22;
pub const DISPID_ICMouseIcon: DISPID_InkCollector = 35;
pub const DISPID_ICMousePointer: DISPID_InkCollector = 36;
pub const DISPID_ICPaint: DISPID_InkCollector = 3;
pub const DISPID_ICRenderer: DISPID_InkCollector = 6;
pub const DISPID_ICSetAllTabletsMode: DISPID_InkCollector = 26;
pub const DISPID_ICSetEventInterest: DISPID_InkCollector = 10;
pub const DISPID_ICSetGestureStatus: DISPID_InkCollector = 29;
pub const DISPID_ICSetSingleTabletIntegratedMode: DISPID_InkCollector = 27;
pub const DISPID_ICSetWindowInputRectangle: DISPID_InkCollector = 23;
pub const DISPID_ICSsAdd: DISPID_InkCustomStrokes = 2;
pub const DISPID_ICSsClear: DISPID_InkCustomStrokes = 4;
pub const DISPID_ICSsCount: DISPID_InkCustomStrokes = 1;
pub const DISPID_ICSsItem: DISPID_InkCustomStrokes = 0;
pub const DISPID_ICSsRemove: DISPID_InkCustomStrokes = 3;
pub const DISPID_ICSs_NewEnum: DISPID_InkCustomStrokes = -4;
pub const DISPID_ICSupportHighContrastInk: DISPID_InkCollector = 38;
pub const DISPID_ICTablet: DISPID_InkCollector = 25;
pub const DISPID_ICText: DISPID_InkCollector = 4;
pub const DISPID_ICanPaste: DISPID_Ink = 24;
pub const DISPID_IClip: DISPID_Ink = 18;
pub const DISPID_IClipboardCopy: DISPID_Ink = 23;
pub const DISPID_IClipboardCopyWithRectangle: DISPID_Ink = 22;
pub const DISPID_IClipboardPaste: DISPID_Ink = 25;
pub const DISPID_IClone: DISPID_Ink = 10;
pub const DISPID_ICreateStroke: DISPID_Ink = 16;
pub const DISPID_ICreateStrokeFromPoints: DISPID_Ink = 21;
pub const DISPID_ICreateStrokes: DISPID_Ink = 15;
pub const DISPID_ICsCount: DISPID_InkCursors = 1;
pub const DISPID_ICsItem: DISPID_InkCursors = 0;
pub const DISPID_ICs_NewEnum: DISPID_InkCursors = -4;
pub const DISPID_ICsrButtons: DISPID_InkCursor = 3;
pub const DISPID_ICsrDrawingAttributes: DISPID_InkCursor = 2;
pub const DISPID_ICsrId: DISPID_InkCursor = 1;
pub const DISPID_ICsrInverted: DISPID_InkCursor = 4;
pub const DISPID_ICsrName: DISPID_InkCursor = 0;
pub const DISPID_ICsrTablet: DISPID_InkCursor = 5;
pub const DISPID_ICustomStrokes: DISPID_Ink = 9;
pub const DISPID_IDeleteStroke: DISPID_Ink = 5;
pub const DISPID_IDeleteStrokes: DISPID_Ink = 4;
pub const DISPID_IDirty: DISPID_Ink = 8;
pub const DISPID_IEInkAdded: DISPID_InkEvent = 1;
pub const DISPID_IEInkDeleted: DISPID_InkEvent = 2;
pub const DISPID_IEPData: DISPID_InkExtendedProperty = 2;
pub const DISPID_IEPGuid: DISPID_InkExtendedProperty = 1;
pub const DISPID_IEPsAdd: DISPID_InkExtendedProperties = 2;
pub const DISPID_IEPsClear: DISPID_InkExtendedProperties = 4;
pub const DISPID_IEPsCount: DISPID_InkExtendedProperties = 1;
pub const DISPID_IEPsDoesPropertyExist: DISPID_InkExtendedProperties = 5;
pub const DISPID_IEPsItem: DISPID_InkExtendedProperties = 0;
pub const DISPID_IEPsRemove: DISPID_InkExtendedProperties = 3;
pub const DISPID_IEPs_NewEnum: DISPID_InkExtendedProperties = -4;
pub const DISPID_IExtendedProperties: DISPID_Ink = 2;
pub const DISPID_IExtractStrokes: DISPID_Ink = 6;
pub const DISPID_IExtractWithRectangle: DISPID_Ink = 7;
pub const DISPID_IGConfidence: DISPID_InkGesture = 2;
pub const DISPID_IGGetHotPoint: DISPID_InkGesture = 1;
pub const DISPID_IGId: DISPID_InkGesture = 0;
pub const DISPID_IGetBoundingBox: DISPID_Ink = 3;
pub const DISPID_IHitTestCircle: DISPID_Ink = 11;
pub const DISPID_IHitTestWithLasso: DISPID_Ink = 13;
pub const DISPID_IHitTestWithRectangle: DISPID_Ink = 12;
pub const DISPID_IInkDivider_Divide: DISPID_InkDivider = 4;
pub const DISPID_IInkDivider_LineHeight: DISPID_InkDivider = 3;
pub const DISPID_IInkDivider_RecognizerContext: DISPID_InkDivider = 2;
pub const DISPID_IInkDivider_Strokes: DISPID_InkDivider = 1;
pub const DISPID_IInkDivisionResult_ResultByType: DISPID_InkDivisionResult = 2;
pub const DISPID_IInkDivisionResult_Strokes: DISPID_InkDivisionResult = 1;
pub const DISPID_IInkDivisionUnit_DivisionType: DISPID_InkDivisionUnit = 2;
pub const DISPID_IInkDivisionUnit_RecognizedString: DISPID_InkDivisionUnit = 3;
pub const DISPID_IInkDivisionUnit_RotationTransform: DISPID_InkDivisionUnit = 4;
pub const DISPID_IInkDivisionUnit_Strokes: DISPID_InkDivisionUnit = 1;
pub const DISPID_IInkDivisionUnits_Count: DISPID_InkDivisionUnits = 1;
pub const DISPID_IInkDivisionUnits_Item: DISPID_InkDivisionUnits = 0;
pub const DISPID_IInkDivisionUnits_NewEnum: DISPID_InkDivisionUnits = -4;
pub const DISPID_ILoad: DISPID_Ink = 20;
pub const DISPID_INearestPoint: DISPID_Ink = 14;
pub const DISPID_IOAttachMode: DISPID_InkCollector = 14;
pub const DISPID_IODraw: DISPID_InkCollector = 16;
pub const DISPID_IOEPainted: DISPID_InkCollectorEvent = 14;
pub const DISPID_IOEPainting: DISPID_InkCollectorEvent = 13;
pub const DISPID_IOESelectionChanged: DISPID_InkCollectorEvent = 16;
pub const DISPID_IOESelectionChanging: DISPID_InkCollectorEvent = 15;
pub const DISPID_IOESelectionMoved: DISPID_InkCollectorEvent = 18;
pub const DISPID_IOESelectionMoving: DISPID_InkCollectorEvent = 17;
pub const DISPID_IOESelectionResized: DISPID_InkCollectorEvent = 20;
pub const DISPID_IOESelectionResizing: DISPID_InkCollectorEvent = 19;
pub const DISPID_IOEStrokesDeleted: DISPID_InkCollectorEvent = 22;
pub const DISPID_IOEStrokesDeleting: DISPID_InkCollectorEvent = 21;
pub const DISPID_IOEditingMode: DISPID_InkCollector = 12;
pub const DISPID_IOEraserMode: DISPID_InkCollector = 33;
pub const DISPID_IOEraserWidth: DISPID_InkCollector = 34;
pub const DISPID_IOHitTestSelection: DISPID_InkCollector = 15;
pub const DISPID_IOSelection: DISPID_InkCollector = 13;
pub const DISPID_IOSupportHighContrastSelectionUI: DISPID_InkCollector = 39;
pub const DISPID_IPBackColor: DISPID_InkCollector = 19;
pub const DISPID_IPEChangeUICues: DISPID_InkCollectorEvent = 23;
pub const DISPID_IPEClick: DISPID_InkCollectorEvent = 24;
pub const DISPID_IPEDblClick: DISPID_InkCollectorEvent = 25;
pub const DISPID_IPEInvalidated: DISPID_InkCollectorEvent = 26;
pub const DISPID_IPEKeyDown: DISPID_InkCollectorEvent = 37;
pub const DISPID_IPEKeyPress: DISPID_InkCollectorEvent = 38;
pub const DISPID_IPEKeyUp: DISPID_InkCollectorEvent = 39;
pub const DISPID_IPEMouseDown: DISPID_InkCollectorEvent = 27;
pub const DISPID_IPEMouseEnter: DISPID_InkCollectorEvent = 28;
pub const DISPID_IPEMouseHover: DISPID_InkCollectorEvent = 29;
pub const DISPID_IPEMouseLeave: DISPID_InkCollectorEvent = 30;
pub const DISPID_IPEMouseMove: DISPID_InkCollectorEvent = 31;
pub const DISPID_IPEMouseUp: DISPID_InkCollectorEvent = 32;
pub const DISPID_IPEMouseWheel: DISPID_InkCollectorEvent = 33;
pub const DISPID_IPEResize: DISPID_InkCollectorEvent = 40;
pub const DISPID_IPESizeChanged: DISPID_InkCollectorEvent = 41;
pub const DISPID_IPESizeModeChanged: DISPID_InkCollectorEvent = 34;
pub const DISPID_IPEStyleChanged: DISPID_InkCollectorEvent = 35;
pub const DISPID_IPESystemColorsChanged: DISPID_InkCollectorEvent = 36;
pub const DISPID_IPInkEnabled: DISPID_InkCollector = 37;
pub const DISPID_IPPicture: DISPID_InkCollector = 17;
pub const DISPID_IPSizeMode: DISPID_InkCollector = 18;
pub const DISPID_IRBottom: DISPID_InkRectangle = 3;
pub const DISPID_IRData: DISPID_InkRectangle = 7;
pub const DISPID_IRDraw: DISPID_InkRenderer = 5;
pub const DISPID_IRDrawStroke: DISPID_InkRenderer = 6;
pub const DISPID_IRERecognition: DISPID_InkRecognitionEvent = 2;
pub const DISPID_IRERecognitionWithAlternates: DISPID_InkRecognitionEvent = 1;
pub const DISPID_IRGColumns: DISPID_InkRecognizerGuide = 4;
pub const DISPID_IRGDrawnBox: DISPID_InkRecognizerGuide = 2;
pub const DISPID_IRGGuideData: DISPID_InkRecognizerGuide = 6;
pub const DISPID_IRGMidline: DISPID_InkRecognizerGuide = 5;
pub const DISPID_IRGRows: DISPID_InkRecognizerGuide = 3;
pub const DISPID_IRGWritingBox: DISPID_InkRecognizerGuide = 1;
pub const DISPID_IRGetObjectTransform: DISPID_InkRenderer = 3;
pub const DISPID_IRGetRectangle: DISPID_InkRectangle = 5;
pub const DISPID_IRGetViewTransform: DISPID_InkRenderer = 1;
pub const DISPID_IRInkSpaceToPixel: DISPID_InkRenderer = 8;
pub const DISPID_IRInkSpaceToPixelFromPoints: DISPID_InkRenderer = 10;
pub const DISPID_IRLeft: DISPID_InkRectangle = 2;
pub const DISPID_IRMeasure: DISPID_InkRenderer = 11;
pub const DISPID_IRMeasureStroke: DISPID_InkRenderer = 12;
pub const DISPID_IRMove: DISPID_InkRenderer = 13;
pub const DISPID_IRPixelToInkSpace: DISPID_InkRenderer = 7;
pub const DISPID_IRPixelToInkSpaceFromPoints: DISPID_InkRenderer = 9;
pub const DISPID_IRRight: DISPID_InkRectangle = 4;
pub const DISPID_IRRotate: DISPID_InkRenderer = 14;
pub const DISPID_IRScale: DISPID_InkRenderer = 15;
pub const DISPID_IRSetObjectTransform: DISPID_InkRenderer = 4;
pub const DISPID_IRSetRectangle: DISPID_InkRectangle = 6;
pub const DISPID_IRSetViewTransform: DISPID_InkRenderer = 2;
pub const DISPID_IRTop: DISPID_InkRectangle = 1;
pub const DISPID_IRecoCtx2_EnabledUnicodeRanges: DISPID_InkRecoContext2 = 0;
pub const DISPID_IRecoCtx_BackgroundRecognize: DISPID_InkRecoContext = 15;
pub const DISPID_IRecoCtx_BackgroundRecognizeWithAlternates: DISPID_InkRecoContext = 16;
pub const DISPID_IRecoCtx_CharacterAutoCompletionMode: DISPID_InkRecoContext = 2;
pub const DISPID_IRecoCtx_Clone: DISPID_InkRecoContext = 11;
pub const DISPID_IRecoCtx_EndInkInput: DISPID_InkRecoContext = 14;
pub const DISPID_IRecoCtx_Factoid: DISPID_InkRecoContext = 3;
pub const DISPID_IRecoCtx_Flags: DISPID_InkRecoContext = 7;
pub const DISPID_IRecoCtx_Guide: DISPID_InkRecoContext = 6;
pub const DISPID_IRecoCtx_IsStringSupported: DISPID_InkRecoContext = 17;
pub const DISPID_IRecoCtx_PrefixText: DISPID_InkRecoContext = 8;
pub const DISPID_IRecoCtx_Recognize: DISPID_InkRecoContext = 12;
pub const DISPID_IRecoCtx_Recognizer: DISPID_InkRecoContext = 5;
pub const DISPID_IRecoCtx_StopBackgroundRecognition: DISPID_InkRecoContext = 13;
pub const DISPID_IRecoCtx_StopRecognition: DISPID_InkRecoContext = 10;
pub const DISPID_IRecoCtx_Strokes: DISPID_InkRecoContext = 1;
pub const DISPID_IRecoCtx_SuffixText: DISPID_InkRecoContext = 9;
pub const DISPID_IRecoCtx_WordList: DISPID_InkRecoContext = 4;
pub const DISPID_IRecosCount: DISPID_InkRecognizers = 1;
pub const DISPID_IRecosGetDefaultRecognizer: DISPID_InkRecognizers = 2;
pub const DISPID_IRecosItem: DISPID_InkRecognizers = 0;
pub const DISPID_IRecos_NewEnum: DISPID_InkRecognizers = -4;
pub const DISPID_ISDBezierCusps: DISPID_InkStrokeDisp = 15;
pub const DISPID_ISDBezierPoints: DISPID_InkStrokeDisp = 13;
pub const DISPID_ISDClip: DISPID_InkStrokeDisp = 7;
pub const DISPID_ISDDeleted: DISPID_InkStrokeDisp = 20;
pub const DISPID_ISDDrawingAttributes: DISPID_InkStrokeDisp = 4;
pub const DISPID_ISDExtendedProperties: DISPID_InkStrokeDisp = 11;
pub const DISPID_ISDFindIntersections: DISPID_InkStrokeDisp = 5;
pub const DISPID_ISDGetBoundingBox: DISPID_InkStrokeDisp = 3;
pub const DISPID_ISDGetFlattenedBezierPoints: DISPID_InkStrokeDisp = 27;
pub const DISPID_ISDGetPacketData: DISPID_InkStrokeDisp = 24;
pub const DISPID_ISDGetPacketDescriptionPropertyMetrics: DISPID_InkStrokeDisp = 21;
pub const DISPID_ISDGetPacketValuesByProperty: DISPID_InkStrokeDisp = 25;
pub const DISPID_ISDGetPoints: DISPID_InkStrokeDisp = 22;
pub const DISPID_ISDGetRectangleIntersections: DISPID_InkStrokeDisp = 6;
pub const DISPID_ISDHitTestCircle: DISPID_InkStrokeDisp = 8;
pub const DISPID_ISDID: DISPID_InkStrokeDisp = 2;
pub const DISPID_ISDInk: DISPID_InkStrokeDisp = 12;
pub const DISPID_ISDInkIndex: DISPID_InkStrokeDisp = 1;
pub const DISPID_ISDMove: DISPID_InkStrokeDisp = 30;
pub const DISPID_ISDNearestPoint: DISPID_InkStrokeDisp = 9;
pub const DISPID_ISDPacketCount: DISPID_InkStrokeDisp = 17;
pub const DISPID_ISDPacketDescription: DISPID_InkStrokeDisp = 19;
pub const DISPID_ISDPacketSize: DISPID_InkStrokeDisp = 18;
pub const DISPID_ISDPolylineCusps: DISPID_InkStrokeDisp = 14;
pub const DISPID_ISDRotate: DISPID_InkStrokeDisp = 31;
pub const DISPID_ISDScale: DISPID_InkStrokeDisp = 33;
pub const DISPID_ISDScaleToRectangle: DISPID_InkStrokeDisp = 28;
pub const DISPID_ISDSelfIntersections: DISPID_InkStrokeDisp = 16;
pub const DISPID_ISDSetPacketValuesByProperty: DISPID_InkStrokeDisp = 26;
pub const DISPID_ISDSetPoints: DISPID_InkStrokeDisp = 23;
pub const DISPID_ISDShear: DISPID_InkStrokeDisp = 32;
pub const DISPID_ISDSplit: DISPID_InkStrokeDisp = 10;
pub const DISPID_ISDTransform: DISPID_InkStrokeDisp = 29;
pub const DISPID_ISave: DISPID_Ink = 19;
pub const DISPID_ISsAdd: DISPID_InkStrokes = 4;
pub const DISPID_ISsAddStrokes: DISPID_InkStrokes = 5;
pub const DISPID_ISsClip: DISPID_InkStrokes = 17;
pub const DISPID_ISsCount: DISPID_InkStrokes = 1;
pub const DISPID_ISsGetBoundingBox: DISPID_InkStrokes = 10;
pub const DISPID_ISsInk: DISPID_InkStrokes = 3;
pub const DISPID_ISsItem: DISPID_InkStrokes = 0;
pub const DISPID_ISsModifyDrawingAttributes: DISPID_InkStrokes = 9;
pub const DISPID_ISsMove: DISPID_InkStrokes = 13;
pub const DISPID_ISsRecognitionResult: DISPID_InkStrokes = 18;
pub const DISPID_ISsRemove: DISPID_InkStrokes = 6;
pub const DISPID_ISsRemoveRecognitionResult: DISPID_InkStrokes = 19;
pub const DISPID_ISsRemoveStrokes: DISPID_InkStrokes = 7;
pub const DISPID_ISsRotate: DISPID_InkStrokes = 14;
pub const DISPID_ISsScale: DISPID_InkStrokes = 16;
pub const DISPID_ISsScaleToRectangle: DISPID_InkStrokes = 11;
pub const DISPID_ISsShear: DISPID_InkStrokes = 15;
pub const DISPID_ISsToString: DISPID_InkStrokes = 8;
pub const DISPID_ISsTransform: DISPID_InkStrokes = 12;
pub const DISPID_ISsValid: DISPID_InkStrokes = 2;
pub const DISPID_ISs_NewEnum: DISPID_InkStrokes = -4;
pub const DISPID_IStrokes: DISPID_Ink = 1;
pub const DISPID_IT2DeviceKind: DISPID_InkTablet2 = 0;
pub const DISPID_IT3IsMultiTouch: DISPID_InkTablet3 = 0;
pub const DISPID_IT3MaximumCursors: DISPID_InkTablet3 = 1;
pub const DISPID_ITData: DISPID_InkTransform = 15;
pub const DISPID_ITGetTransform: DISPID_InkTransform = 13;
pub const DISPID_ITHardwareCapabilities: DISPID_InkTablet = 5;
pub const DISPID_ITIsPacketPropertySupported: DISPID_InkTablet = 3;
pub const DISPID_ITMaximumInputRectangle: DISPID_InkTablet = 4;
pub const DISPID_ITName: DISPID_InkTablet = 0;
pub const DISPID_ITPlugAndPlayId: DISPID_InkTablet = 1;
pub const DISPID_ITPropertyMetrics: DISPID_InkTablet = 2;
pub const DISPID_ITReflect: DISPID_InkTransform = 4;
pub const DISPID_ITReset: DISPID_InkTransform = 1;
pub const DISPID_ITRotate: DISPID_InkTransform = 3;
pub const DISPID_ITScale: DISPID_InkTransform = 6;
pub const DISPID_ITSetTransform: DISPID_InkTransform = 14;
pub const DISPID_ITShear: DISPID_InkTransform = 5;
pub const DISPID_ITTranslate: DISPID_InkTransform = 2;
pub const DISPID_ITeDx: DISPID_InkTransform = 11;
pub const DISPID_ITeDy: DISPID_InkTransform = 12;
pub const DISPID_ITeM11: DISPID_InkTransform = 7;
pub const DISPID_ITeM12: DISPID_InkTransform = 8;
pub const DISPID_ITeM21: DISPID_InkTransform = 9;
pub const DISPID_ITeM22: DISPID_InkTransform = 10;
pub const DISPID_ITsCount: DISPID_InkTablets = 2;
pub const DISPID_ITsDefaultTablet: DISPID_InkTablets = 1;
pub const DISPID_ITsIsPacketPropertySupported: DISPID_InkTablets = 3;
pub const DISPID_ITsItem: DISPID_InkTablets = 0;
pub const DISPID_ITs_NewEnum: DISPID_InkTablets = -4;
pub const DISPID_IeeChange: DISPID_InkEditEvents = 1;
pub const DISPID_IeeClick: DISPID_InkEditEvents = 9;
pub const DISPID_IeeCursorDown: DISPID_InkEditEvents = 21;
pub const DISPID_IeeDblClick: DISPID_InkEditEvents = 8;
pub const DISPID_IeeGesture: DISPID_InkEditEvents = 23;
pub const DISPID_IeeKeyDown: DISPID_InkEditEvents = 3;
pub const DISPID_IeeKeyPress: DISPID_InkEditEvents = 7;
pub const DISPID_IeeKeyUp: DISPID_InkEditEvents = 4;
pub const DISPID_IeeMouseDown: DISPID_InkEditEvents = 6;
pub const DISPID_IeeMouseMove: DISPID_InkEditEvents = 10;
pub const DISPID_IeeMouseUp: DISPID_InkEditEvents = 5;
pub const DISPID_IeeRecognitionResult: DISPID_InkEditEvents = 24;
pub const DISPID_IeeSelChange: DISPID_InkEditEvents = 2;
pub const DISPID_IeeStroke: DISPID_InkEditEvents = 22;
pub type DISPID_Ink = i32;
pub type DISPID_InkCollector = i32;
pub type DISPID_InkCollectorEvent = i32;
pub type DISPID_InkCursor = i32;
pub type DISPID_InkCursorButton = i32;
pub type DISPID_InkCursorButtons = i32;
pub type DISPID_InkCursors = i32;
pub type DISPID_InkCustomStrokes = i32;
pub type DISPID_InkDivider = i32;
pub type DISPID_InkDivisionResult = i32;
pub type DISPID_InkDivisionUnit = i32;
pub type DISPID_InkDivisionUnits = i32;
pub type DISPID_InkDrawingAttributes = i32;
pub type DISPID_InkEdit = i32;
pub type DISPID_InkEditEvents = i32;
pub type DISPID_InkEvent = i32;
pub type DISPID_InkExtendedProperties = i32;
pub type DISPID_InkExtendedProperty = i32;
pub type DISPID_InkGesture = i32;
pub const DISPID_InkInsertMode: DISPID_InkEdit = 25;
pub const DISPID_InkMode: DISPID_InkEdit = 24;
pub type DISPID_InkRecoAlternate = i32;
pub const DISPID_InkRecoAlternate_AlternatesWithConstantPropertyValues: DISPID_InkRecoAlternate = 15;
pub const DISPID_InkRecoAlternate_Ascender: DISPID_InkRecoAlternate = 5;
pub const DISPID_InkRecoAlternate_Baseline: DISPID_InkRecoAlternate = 3;
pub const DISPID_InkRecoAlternate_Confidence: DISPID_InkRecoAlternate = 7;
pub const DISPID_InkRecoAlternate_ConfidenceAlternates: DISPID_InkRecoAlternate = 14;
pub const DISPID_InkRecoAlternate_Descender: DISPID_InkRecoAlternate = 6;
pub const DISPID_InkRecoAlternate_GetPropertyValue: DISPID_InkRecoAlternate = 12;
pub const DISPID_InkRecoAlternate_GetStrokesFromStrokeRanges: DISPID_InkRecoAlternate = 9;
pub const DISPID_InkRecoAlternate_GetStrokesFromTextRange: DISPID_InkRecoAlternate = 10;
pub const DISPID_InkRecoAlternate_GetTextRangeFromStrokes: DISPID_InkRecoAlternate = 11;
pub const DISPID_InkRecoAlternate_LineAlternates: DISPID_InkRecoAlternate = 13;
pub const DISPID_InkRecoAlternate_LineNumber: DISPID_InkRecoAlternate = 2;
pub const DISPID_InkRecoAlternate_Midline: DISPID_InkRecoAlternate = 4;
pub const DISPID_InkRecoAlternate_String: DISPID_InkRecoAlternate = 1;
pub const DISPID_InkRecoAlternate_Strokes: DISPID_InkRecoAlternate = 8;
pub type DISPID_InkRecoContext = i32;
pub type DISPID_InkRecoContext2 = i32;
pub type DISPID_InkRecognitionAlternates = i32;
pub const DISPID_InkRecognitionAlternates_Count: DISPID_InkRecognitionAlternates = 1;
pub const DISPID_InkRecognitionAlternates_Item: DISPID_InkRecognitionAlternates = 0;
pub const DISPID_InkRecognitionAlternates_NewEnum: DISPID_InkRecognitionAlternates = -4;
pub const DISPID_InkRecognitionAlternates_Strokes: DISPID_InkRecognitionAlternates = 2;
pub type DISPID_InkRecognitionEvent = i32;
pub type DISPID_InkRecognitionResult = i32;
pub const DISPID_InkRecognitionResult_AlternatesFromSelection: DISPID_InkRecognitionResult = 5;
pub const DISPID_InkRecognitionResult_ModifyTopAlternate: DISPID_InkRecognitionResult = 6;
pub const DISPID_InkRecognitionResult_SetResultOnStrokes: DISPID_InkRecognitionResult = 7;
pub const DISPID_InkRecognitionResult_Strokes: DISPID_InkRecognitionResult = 3;
pub const DISPID_InkRecognitionResult_TopAlternate: DISPID_InkRecognitionResult = 2;
pub const DISPID_InkRecognitionResult_TopConfidence: DISPID_InkRecognitionResult = 4;
pub const DISPID_InkRecognitionResult_TopString: DISPID_InkRecognitionResult = 1;
pub type DISPID_InkRecognizer = i32;
pub type DISPID_InkRecognizer2 = i32;
pub type DISPID_InkRecognizerGuide = i32;
pub type DISPID_InkRecognizers = i32;
pub type DISPID_InkRectangle = i32;
pub type DISPID_InkRenderer = i32;
pub type DISPID_InkStrokeDisp = i32;
pub type DISPID_InkStrokes = i32;
pub type DISPID_InkTablet = i32;
pub type DISPID_InkTablet2 = i32;
pub type DISPID_InkTablet3 = i32;
pub type DISPID_InkTablets = i32;
pub type DISPID_InkTransform = i32;
pub type DISPID_InkWordList = i32;
pub type DISPID_InkWordList2 = i32;
pub const DISPID_InkWordList2_AddWords: DISPID_InkWordList2 = 3;
pub const DISPID_InkWordList_AddWord: DISPID_InkWordList = 0;
pub const DISPID_InkWordList_Merge: DISPID_InkWordList = 2;
pub const DISPID_InkWordList_RemoveWord: DISPID_InkWordList = 1;
pub const DISPID_Locked: DISPID_InkEdit = 4;
pub const DISPID_MICClear: DISPID_MathInputControlEvents = 3;
pub const DISPID_MICClose: DISPID_MathInputControlEvents = 1;
pub const DISPID_MICInsert: DISPID_MathInputControlEvents = 0;
pub const DISPID_MICPaint: DISPID_MathInputControlEvents = 2;
pub type DISPID_MathInputControlEvents = i32;
pub const DISPID_MaxLength: DISPID_InkEdit = 6;
pub const DISPID_MultiLine: DISPID_InkEdit = 7;
pub const DISPID_PIPAttachedEditWindow: DISPID_PenInputPanel = 0;
pub const DISPID_PIPAutoShow: DISPID_PenInputPanel = 16;
pub const DISPID_PIPBusy: DISPID_PenInputPanel = 12;
pub const DISPID_PIPCommitPendingInput: DISPID_PenInputPanel = 10;
pub const DISPID_PIPCurrentPanel: DISPID_PenInputPanel = 2;
pub const DISPID_PIPDefaultPanel: DISPID_PenInputPanel = 3;
pub const DISPID_PIPEInputFailed: DISPID_PenInputPanelEvents = 2;
pub const DISPID_PIPEPanelChanged: DISPID_PenInputPanelEvents = 1;
pub const DISPID_PIPEPanelMoving: DISPID_PenInputPanelEvents = 3;
pub const DISPID_PIPEVisibleChanged: DISPID_PenInputPanelEvents = 0;
pub const DISPID_PIPEnableTsf: DISPID_PenInputPanel = 15;
pub const DISPID_PIPFactoid: DISPID_PenInputPanel = 1;
pub const DISPID_PIPHeight: DISPID_PenInputPanel = 8;
pub const DISPID_PIPHorizontalOffset: DISPID_PenInputPanel = 14;
pub const DISPID_PIPLeft: DISPID_PenInputPanel = 6;
pub const DISPID_PIPMoveTo: DISPID_PenInputPanel = 9;
pub const DISPID_PIPRefresh: DISPID_PenInputPanel = 11;
pub const DISPID_PIPTop: DISPID_PenInputPanel = 5;
pub const DISPID_PIPVerticalOffset: DISPID_PenInputPanel = 13;
pub const DISPID_PIPVisible: DISPID_PenInputPanel = 4;
pub const DISPID_PIPWidth: DISPID_PenInputPanel = 7;
pub type DISPID_PenInputPanel = i32;
pub type DISPID_PenInputPanelEvents = i32;
pub const DISPID_RTSelLength: DISPID_InkEdit = 10;
pub const DISPID_RTSelStart: DISPID_InkEdit = 9;
pub const DISPID_RTSelText: DISPID_InkEdit = 11;
pub const DISPID_RecoCapabilities: DISPID_InkRecognizer = 4;
pub const DISPID_RecoClsid: DISPID_InkRecognizer = 1;
pub const DISPID_RecoCreateRecognizerContext: DISPID_InkRecognizer = 7;
pub const DISPID_RecoId: DISPID_InkRecognizer2 = 0;
pub const DISPID_RecoLanguageID: DISPID_InkRecognizer = 5;
pub const DISPID_RecoName: DISPID_InkRecognizer = 2;
pub const DISPID_RecoPreferredPacketDescription: DISPID_InkRecognizer = 6;
pub const DISPID_RecoSupportedProperties: DISPID_InkRecognizer = 8;
pub const DISPID_RecoTimeout: DISPID_InkEdit = 26;
pub const DISPID_RecoUnicodeRanges: DISPID_InkRecognizer2 = 1;
pub const DISPID_RecoVendor: DISPID_InkRecognizer = 3;
pub const DISPID_Recognize: DISPID_InkEdit = 32;
pub const DISPID_Recognizer: DISPID_InkEdit = 28;
pub const DISPID_Refresh: DISPID_InkEdit = 35;
pub const DISPID_SEStrokesAdded: DISPID_StrokeEvent = 1;
pub const DISPID_SEStrokesRemoved: DISPID_StrokeEvent = 2;
pub const DISPID_ScrollBars: DISPID_InkEdit = 8;
pub const DISPID_SelAlignment: DISPID_InkEdit = 12;
pub const DISPID_SelBold: DISPID_InkEdit = 13;
pub const DISPID_SelCharOffset: DISPID_InkEdit = 14;
pub const DISPID_SelColor: DISPID_InkEdit = 15;
pub const DISPID_SelFontName: DISPID_InkEdit = 16;
pub const DISPID_SelFontSize: DISPID_InkEdit = 17;
pub const DISPID_SelInk: DISPID_InkEdit = 30;
pub const DISPID_SelInksDisplayMode: DISPID_InkEdit = 31;
pub const DISPID_SelItalic: DISPID_InkEdit = 18;
pub const DISPID_SelRTF: DISPID_InkEdit = 19;
pub const DISPID_SelUnderline: DISPID_InkEdit = 20;
pub const DISPID_SetGestStatus: DISPID_InkEdit = 34;
pub const DISPID_Status: DISPID_InkEdit = 22;
pub type DISPID_StrokeEvent = i32;
pub const DISPID_Text: DISPID_InkEdit = 0;
pub const DISPID_TextRTF: DISPID_InkEdit = 1;
pub const DISPID_UseMouseForInput: DISPID_InkEdit = 23;
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct DYNAMIC_RENDERER_CACHED_DATA {
    pub strokeId: i32,
    pub dynamicRenderer: core::mem::ManuallyDrop<*mut core::ffi::c_void>,
}
impl Default for DYNAMIC_RENDERER_CACHED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DockedBottom: VisualState = 3;
pub const DockedTop: VisualState = 2;
pub const DynamicRenderer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xecd32aea_746f_4dcb_bf68_082757faff18);
pub const EM_GETDRAWATTR: u32 = 1541;
pub const EM_GETFACTOID: u32 = 1549;
pub const EM_GETGESTURESTATUS: u32 = 1545;
pub const EM_GETINKINSERTMODE: u32 = 1539;
pub const EM_GETINKMODE: u32 = 1537;
pub const EM_GETMOUSEICON: u32 = 1553;
pub const EM_GETMOUSEPOINTER: u32 = 1555;
pub const EM_GETRECOGNIZER: u32 = 1547;
pub const EM_GETRECOTIMEOUT: u32 = 1543;
pub const EM_GETSELINK: u32 = 1551;
pub const EM_GETSELINKDISPLAYMODE: u32 = 1562;
pub const EM_GETSTATUS: u32 = 1557;
pub const EM_GETUSEMOUSEFORINPUT: u32 = 1559;
pub const EM_RECOGNIZE: u32 = 1558;
pub const EM_SETDRAWATTR: u32 = 1542;
pub const EM_SETFACTOID: u32 = 1550;
pub const EM_SETGESTURESTATUS: u32 = 1546;
pub const EM_SETINKINSERTMODE: u32 = 1540;
pub const EM_SETINKMODE: u32 = 1538;
pub const EM_SETMOUSEICON: u32 = 1554;
pub const EM_SETMOUSEPOINTER: u32 = 1556;
pub const EM_SETRECOGNIZER: u32 = 1548;
pub const EM_SETRECOTIMEOUT: u32 = 1544;
pub const EM_SETSELINK: u32 = 1552;
pub const EM_SETSELINKDISPLAYMODE: u32 = 1561;
pub const EM_SETUSEMOUSEFORINPUT: u32 = 1560;
pub type EventMask = i32;
pub const EventMask_All: EventMask = 4095;
pub const EventMask_CorrectionModeChanged: EventMask = 128;
pub const EventMask_CorrectionModeChanging: EventMask = 64;
pub const EventMask_InPlaceSizeChanged: EventMask = 8;
pub const EventMask_InPlaceSizeChanging: EventMask = 4;
pub const EventMask_InPlaceStateChanged: EventMask = 2;
pub const EventMask_InPlaceStateChanging: EventMask = 1;
pub const EventMask_InPlaceVisibilityChanged: EventMask = 512;
pub const EventMask_InPlaceVisibilityChanging: EventMask = 256;
pub const EventMask_InputAreaChanged: EventMask = 32;
pub const EventMask_InputAreaChanging: EventMask = 16;
pub const EventMask_TextInserted: EventMask = 2048;
pub const EventMask_TextInserting: EventMask = 1024;
pub const FACILITY_INK: u32 = 40;
pub const FACTOID_BOPOMOFO: windows_sys::core::PCWSTR = windows_sys::core::w!("BOPOMOFO");
pub const FACTOID_CHINESESIMPLECOMMON: windows_sys::core::PCWSTR = windows_sys::core::w!("CHS_COMMON");
pub const FACTOID_CHINESETRADITIONALCOMMON: windows_sys::core::PCWSTR = windows_sys::core::w!("CHT_COMMON");
pub const FACTOID_CURRENCY: windows_sys::core::PCWSTR = windows_sys::core::w!("CURRENCY");
pub const FACTOID_DATE: windows_sys::core::PCWSTR = windows_sys::core::w!("DATE");
pub const FACTOID_DEFAULT: windows_sys::core::PCWSTR = windows_sys::core::w!("DEFAULT");
pub const FACTOID_DIGIT: windows_sys::core::PCWSTR = windows_sys::core::w!("DIGIT");
pub const FACTOID_EMAIL: windows_sys::core::PCWSTR = windows_sys::core::w!("EMAIL");
pub const FACTOID_FILENAME: windows_sys::core::PCWSTR = windows_sys::core::w!("FILENAME");
pub const FACTOID_HANGULCOMMON: windows_sys::core::PCWSTR = windows_sys::core::w!("HANGUL_COMMON");
pub const FACTOID_HANGULRARE: windows_sys::core::PCWSTR = windows_sys::core::w!("HANGUL_RARE");
pub const FACTOID_HIRAGANA: windows_sys::core::PCWSTR = windows_sys::core::w!("HIRAGANA");
pub const FACTOID_JAMO: windows_sys::core::PCWSTR = windows_sys::core::w!("JAMO");
pub const FACTOID_JAPANESECOMMON: windows_sys::core::PCWSTR = windows_sys::core::w!("JPN_COMMON");
pub const FACTOID_KANJICOMMON: windows_sys::core::PCWSTR = windows_sys::core::w!("KANJI_COMMON");
pub const FACTOID_KANJIRARE: windows_sys::core::PCWSTR = windows_sys::core::w!("KANJI_RARE");
pub const FACTOID_KATAKANA: windows_sys::core::PCWSTR = windows_sys::core::w!("KATAKANA");
pub const FACTOID_KOREANCOMMON: windows_sys::core::PCWSTR = windows_sys::core::w!("KOR_COMMON");
pub const FACTOID_LOWERCHAR: windows_sys::core::PCWSTR = windows_sys::core::w!("LOWERCHAR");
pub const FACTOID_NONE: windows_sys::core::PCWSTR = windows_sys::core::w!("NONE");
pub const FACTOID_NUMBER: windows_sys::core::PCWSTR = windows_sys::core::w!("NUMBER");
pub const FACTOID_NUMBERSIMPLE: windows_sys::core::PCWSTR = windows_sys::core::w!("NUMSIMPLE");
pub const FACTOID_ONECHAR: windows_sys::core::PCWSTR = windows_sys::core::w!("ONECHAR");
pub const FACTOID_PERCENT: windows_sys::core::PCWSTR = windows_sys::core::w!("PERCENT");
pub const FACTOID_POSTALCODE: windows_sys::core::PCWSTR = windows_sys::core::w!("POSTALCODE");
pub const FACTOID_PUNCCHAR: windows_sys::core::PCWSTR = windows_sys::core::w!("PUNCCHAR");
pub const FACTOID_SYSTEMDICTIONARY: windows_sys::core::PCWSTR = windows_sys::core::w!("SYSDICT");
pub const FACTOID_TELEPHONE: windows_sys::core::PCWSTR = windows_sys::core::w!("TELEPHONE");
pub const FACTOID_TIME: windows_sys::core::PCWSTR = windows_sys::core::w!("TIME");
pub const FACTOID_UPPERCHAR: windows_sys::core::PCWSTR = windows_sys::core::w!("UPPERCHAR");
pub const FACTOID_WEB: windows_sys::core::PCWSTR = windows_sys::core::w!("WEB");
pub const FACTOID_WORDLIST: windows_sys::core::PCWSTR = windows_sys::core::w!("WORDLIST");
pub type FLICKACTION_COMMANDCODE = i32;
pub const FLICKACTION_COMMANDCODE_APPCOMMAND: FLICKACTION_COMMANDCODE = 2;
pub const FLICKACTION_COMMANDCODE_CUSTOMKEY: FLICKACTION_COMMANDCODE = 3;
pub const FLICKACTION_COMMANDCODE_KEYMODIFIER: FLICKACTION_COMMANDCODE = 4;
pub const FLICKACTION_COMMANDCODE_NULL: FLICKACTION_COMMANDCODE = 0;
pub const FLICKACTION_COMMANDCODE_SCROLL: FLICKACTION_COMMANDCODE = 1;
pub type FLICKDIRECTION = i32;
pub const FLICKDIRECTION_DOWN: FLICKDIRECTION = 6;
pub const FLICKDIRECTION_DOWNLEFT: FLICKDIRECTION = 5;
pub const FLICKDIRECTION_DOWNRIGHT: FLICKDIRECTION = 7;
pub const FLICKDIRECTION_INVALID: FLICKDIRECTION = 8;
pub const FLICKDIRECTION_LEFT: FLICKDIRECTION = 4;
pub const FLICKDIRECTION_MIN: FLICKDIRECTION = 0;
pub const FLICKDIRECTION_RIGHT: FLICKDIRECTION = 0;
pub const FLICKDIRECTION_UP: FLICKDIRECTION = 2;
pub const FLICKDIRECTION_UPLEFT: FLICKDIRECTION = 3;
pub const FLICKDIRECTION_UPRIGHT: FLICKDIRECTION = 1;
pub type FLICKMODE = i32;
pub const FLICKMODE_DEFAULT: FLICKMODE = 1;
pub const FLICKMODE_LEARNING: FLICKMODE = 2;
pub const FLICKMODE_MAX: FLICKMODE = 2;
pub const FLICKMODE_MIN: FLICKMODE = 0;
pub const FLICKMODE_OFF: FLICKMODE = 0;
pub const FLICKMODE_ON: FLICKMODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLICK_DATA {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLICK_POINT {
    pub _bitfield: i32,
}
pub const FLICK_WM_HANDLED_MASK: u32 = 1;
pub const Floating: VisualState = 1;
pub const GESTURE_ARROW_DOWN: u32 = 61497;
pub const GESTURE_ARROW_LEFT: u32 = 61498;
pub const GESTURE_ARROW_RIGHT: u32 = 61499;
pub const GESTURE_ARROW_UP: u32 = 61496;
pub const GESTURE_ASTERISK: u32 = 61608;
pub const GESTURE_BRACE_LEFT: u32 = 61674;
pub const GESTURE_BRACE_OVER: u32 = 61672;
pub const GESTURE_BRACE_RIGHT: u32 = 61675;
pub const GESTURE_BRACE_UNDER: u32 = 61673;
pub const GESTURE_BRACKET_LEFT: u32 = 61670;
pub const GESTURE_BRACKET_OVER: u32 = 61668;
pub const GESTURE_BRACKET_RIGHT: u32 = 61671;
pub const GESTURE_BRACKET_UNDER: u32 = 61669;
pub const GESTURE_BULLET: u32 = 61450;
pub const GESTURE_BULLET_CROSS: u32 = 61451;
pub const GESTURE_CHECK: u32 = 61445;
pub const GESTURE_CHEVRON_DOWN: u32 = 61489;
pub const GESTURE_CHEVRON_LEFT: u32 = 61490;
pub const GESTURE_CHEVRON_RIGHT: u32 = 61491;
pub const GESTURE_CHEVRON_UP: u32 = 61488;
pub const GESTURE_CIRCLE: u32 = 61472;
pub const GESTURE_CIRCLE_CIRCLE: u32 = 61475;
pub const GESTURE_CIRCLE_CROSS: u32 = 61477;
pub const GESTURE_CIRCLE_LINE_HORZ: u32 = 61479;
pub const GESTURE_CIRCLE_LINE_VERT: u32 = 61478;
pub const GESTURE_CIRCLE_TAP: u32 = 61474;
pub const GESTURE_CLOSEUP: u32 = 61455;
pub const GESTURE_CROSS: u32 = 61447;
pub const GESTURE_CURLICUE: u32 = 61456;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GESTURE_DATA {
    pub gestureId: i32,
    pub recoConfidence: i32,
    pub strokeCount: i32,
}
pub const GESTURE_DIAGONAL_LEFTDOWN: u32 = 61534;
pub const GESTURE_DIAGONAL_LEFTUP: u32 = 61532;
pub const GESTURE_DIAGONAL_RIGHTDOWN: u32 = 61535;
pub const GESTURE_DIAGONAL_RIGHTUP: u32 = 61533;
pub const GESTURE_DIGIT_0: u32 = 61594;
pub const GESTURE_DIGIT_1: u32 = 61595;
pub const GESTURE_DIGIT_2: u32 = 61596;
pub const GESTURE_DIGIT_3: u32 = 61597;
pub const GESTURE_DIGIT_4: u32 = 61598;
pub const GESTURE_DIGIT_5: u32 = 61599;
pub const GESTURE_DIGIT_6: u32 = 61600;
pub const GESTURE_DIGIT_7: u32 = 61601;
pub const GESTURE_DIGIT_8: u32 = 61602;
pub const GESTURE_DIGIT_9: u32 = 61603;
pub const GESTURE_DOLLAR: u32 = 61607;
pub const GESTURE_DOUBLE_ARROW_DOWN: u32 = 61501;
pub const GESTURE_DOUBLE_ARROW_LEFT: u32 = 61502;
pub const GESTURE_DOUBLE_ARROW_RIGHT: u32 = 61503;
pub const GESTURE_DOUBLE_ARROW_UP: u32 = 61500;
pub const GESTURE_DOUBLE_CIRCLE: u32 = 61473;
pub const GESTURE_DOUBLE_CURLICUE: u32 = 61457;
pub const GESTURE_DOUBLE_DOWN: u32 = 61625;
pub const GESTURE_DOUBLE_LEFT: u32 = 61626;
pub const GESTURE_DOUBLE_RIGHT: u32 = 61627;
pub const GESTURE_DOUBLE_TAP: u32 = 61681;
pub const GESTURE_DOUBLE_UP: u32 = 61624;
pub const GESTURE_DOWN: u32 = 61529;
pub const GESTURE_DOWN_ARROW_LEFT: u32 = 61506;
pub const GESTURE_DOWN_ARROW_RIGHT: u32 = 61507;
pub const GESTURE_DOWN_LEFT: u32 = 61546;
pub const GESTURE_DOWN_LEFT_LONG: u32 = 61542;
pub const GESTURE_DOWN_RIGHT: u32 = 61547;
pub const GESTURE_DOWN_RIGHT_LONG: u32 = 61543;
pub const GESTURE_DOWN_UP: u32 = 61537;
pub const GESTURE_EXCLAMATION: u32 = 61604;
pub const GESTURE_INFINITY: u32 = 61446;
pub const GESTURE_LEFT: u32 = 61530;
pub const GESTURE_LEFT_ARROW_DOWN: u32 = 61509;
pub const GESTURE_LEFT_ARROW_UP: u32 = 61508;
pub const GESTURE_LEFT_DOWN: u32 = 61549;
pub const GESTURE_LEFT_RIGHT: u32 = 61538;
pub const GESTURE_LEFT_UP: u32 = 61548;
pub const GESTURE_LETTER_A: u32 = 61568;
pub const GESTURE_LETTER_B: u32 = 61569;
pub const GESTURE_LETTER_C: u32 = 61570;
pub const GESTURE_LETTER_D: u32 = 61571;
pub const GESTURE_LETTER_E: u32 = 61572;
pub const GESTURE_LETTER_F: u32 = 61573;
pub const GESTURE_LETTER_G: u32 = 61574;
pub const GESTURE_LETTER_H: u32 = 61575;
pub const GESTURE_LETTER_I: u32 = 61576;
pub const GESTURE_LETTER_J: u32 = 61577;
pub const GESTURE_LETTER_K: u32 = 61578;
pub const GESTURE_LETTER_L: u32 = 61579;
pub const GESTURE_LETTER_M: u32 = 61580;
pub const GESTURE_LETTER_N: u32 = 61581;
pub const GESTURE_LETTER_O: u32 = 61582;
pub const GESTURE_LETTER_P: u32 = 61583;
pub const GESTURE_LETTER_Q: u32 = 61584;
pub const GESTURE_LETTER_R: u32 = 61585;
pub const GESTURE_LETTER_S: u32 = 61586;
pub const GESTURE_LETTER_T: u32 = 61587;
pub const GESTURE_LETTER_U: u32 = 61588;
pub const GESTURE_LETTER_V: u32 = 61589;
pub const GESTURE_LETTER_W: u32 = 61590;
pub const GESTURE_LETTER_X: u32 = 61591;
pub const GESTURE_LETTER_Y: u32 = 61592;
pub const GESTURE_LETTER_Z: u32 = 61593;
pub const GESTURE_NULL: u32 = 61440;
pub const GESTURE_OPENUP: u32 = 61454;
pub const GESTURE_PARAGRAPH: u32 = 61448;
pub const GESTURE_PLUS: u32 = 61609;
pub const GESTURE_QUAD_TAP: u32 = 61683;
pub const GESTURE_QUESTION: u32 = 61605;
pub const GESTURE_RECTANGLE: u32 = 61458;
pub const GESTURE_RIGHT: u32 = 61531;
pub const GESTURE_RIGHT_ARROW_DOWN: u32 = 61511;
pub const GESTURE_RIGHT_ARROW_UP: u32 = 61510;
pub const GESTURE_RIGHT_DOWN: u32 = 61551;
pub const GESTURE_RIGHT_LEFT: u32 = 61539;
pub const GESTURE_RIGHT_UP: u32 = 61550;
pub const GESTURE_SCRATCHOUT: u32 = 61441;
pub const GESTURE_SECTION: u32 = 61449;
pub const GESTURE_SEMICIRCLE_LEFT: u32 = 61480;
pub const GESTURE_SEMICIRCLE_RIGHT: u32 = 61481;
pub const GESTURE_SHARP: u32 = 61606;
pub const GESTURE_SQUARE: u32 = 61443;
pub const GESTURE_SQUIGGLE: u32 = 61452;
pub const GESTURE_STAR: u32 = 61444;
pub const GESTURE_SWAP: u32 = 61453;
pub const GESTURE_TAP: u32 = 61680;
pub const GESTURE_TRIANGLE: u32 = 61442;
pub const GESTURE_TRIPLE_DOWN: u32 = 61629;
pub const GESTURE_TRIPLE_LEFT: u32 = 61630;
pub const GESTURE_TRIPLE_RIGHT: u32 = 61631;
pub const GESTURE_TRIPLE_TAP: u32 = 61682;
pub const GESTURE_TRIPLE_UP: u32 = 61628;
pub const GESTURE_UP: u32 = 61528;
pub const GESTURE_UP_ARROW_LEFT: u32 = 61504;
pub const GESTURE_UP_ARROW_RIGHT: u32 = 61505;
pub const GESTURE_UP_DOWN: u32 = 61536;
pub const GESTURE_UP_LEFT: u32 = 61544;
pub const GESTURE_UP_LEFT_LONG: u32 = 61540;
pub const GESTURE_UP_RIGHT: u32 = 61545;
pub const GESTURE_UP_RIGHT_LONG: u32 = 61541;
pub type GET_DANDIDATE_FLAGS = i32;
pub const GUID_DYNAMIC_RENDERER_CACHED_DATA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbf531b92_25bf_4a95_89ad_0e476b34b4f5);
pub const GUID_GESTURE_DATA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x41e4ec0f_26aa_455a_9aa5_2cd36cf63fb9);
pub const GUID_PACKETPROPERTY_GUID_ALTITUDE_ORIENTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x82dec5c7_f6ba_4906_894f_66d68dfc456c);
pub const GUID_PACKETPROPERTY_GUID_AZIMUTH_ORIENTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x029123b4_8828_410b_b250_a0536595e5dc);
pub const GUID_PACKETPROPERTY_GUID_BUTTON_PRESSURE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8b7fefc4_96aa_4bfe_ac26_8a5f0be07bf5);
pub const GUID_PACKETPROPERTY_GUID_DEVICE_CONTACT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x02585b91_049b_4750_9615_df8948ab3c9c);
pub const GUID_PACKETPROPERTY_GUID_FINGERCONTACTCONFIDENCE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe706c804_57f0_4f00_8a0c_853d57789be9);
pub const GUID_PACKETPROPERTY_GUID_HEIGHT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe61858d2_e447_4218_9d3f_18865c203df4);
pub const GUID_PACKETPROPERTY_GUID_NORMAL_PRESSURE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7307502d_f9f4_4e18_b3f2_2ce1b1a3610c);
pub const GUID_PACKETPROPERTY_GUID_PACKET_STATUS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6e0e07bf_afe7_4cf7_87d1_af6446208418);
pub const GUID_PACKETPROPERTY_GUID_PITCH_ROTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7f7e57b7_be37_4be1_a356_7a84160e1893);
pub const GUID_PACKETPROPERTY_GUID_ROLL_ROTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5d5d5e56_6ba9_4c5b_9fb0_851c91714e56);
pub const GUID_PACKETPROPERTY_GUID_SERIAL_NUMBER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x78a81b56_0935_4493_baae_00541a8a16c4);
pub const GUID_PACKETPROPERTY_GUID_TANGENT_PRESSURE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6da4488b_5244_41ec_905b_32d89ab80809);
pub const GUID_PACKETPROPERTY_GUID_TIMER_TICK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x436510c5_fed3_45d1_8b76_71d3ea7a829d);
pub const GUID_PACKETPROPERTY_GUID_TWIST_ORIENTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0d324960_13b2_41e4_ace6_7ae9d43d2d3b);
pub const GUID_PACKETPROPERTY_GUID_WIDTH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbaabe94d_2712_48f5_be9d_8f8b5ea0711a);
pub const GUID_PACKETPROPERTY_GUID_X: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x598a6a8f_52c0_4ba0_93af_af357411a561);
pub const GUID_PACKETPROPERTY_GUID_X_TILT_ORIENTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa8d07b3a_8bf0_40b0_95a9_b80a6bb787bf);
pub const GUID_PACKETPROPERTY_GUID_Y: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb53f9f75_04e0_4498_a7ee_c30dbb5a9011);
pub const GUID_PACKETPROPERTY_GUID_YAW_ROTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6a849980_7c3a_45b7_aa82_90a262950e89);
pub const GUID_PACKETPROPERTY_GUID_Y_TILT_ORIENTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0e932389_1d77_43af_ac00_5b950d6d4b2d);
pub const GUID_PACKETPROPERTY_GUID_Z: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x735adb30_0ebb_4788_a0e4_0f316490055d);
pub const GestureRecognizer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea30c654_c62c_441f_ac00_95f9a196782c);
pub type HRECOALT = *mut core::ffi::c_void;
pub type HRECOCONTEXT = *mut core::ffi::c_void;
pub type HRECOGNIZER = *mut core::ffi::c_void;
pub type HRECOLATTICE = *mut core::ffi::c_void;
pub type HRECOWORDLIST = *mut core::ffi::c_void;
pub const HandwrittenTextInsertion: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9f074ee2_e6e9_4d8a_a047_eb5b5c3c55da);
pub const IAG_AllGestures: InkApplicationGesture = 0;
pub const IAG_ArrowDown: InkApplicationGesture = 61497;
pub const IAG_ArrowLeft: InkApplicationGesture = 61498;
pub const IAG_ArrowRight: InkApplicationGesture = 61499;
pub const IAG_ArrowUp: InkApplicationGesture = 61496;
pub const IAG_Check: InkApplicationGesture = 61445;
pub const IAG_ChevronDown: InkApplicationGesture = 61489;
pub const IAG_ChevronLeft: InkApplicationGesture = 61490;
pub const IAG_ChevronRight: InkApplicationGesture = 61491;
pub const IAG_ChevronUp: InkApplicationGesture = 61488;
pub const IAG_Circle: InkApplicationGesture = 61472;
pub const IAG_Curlicue: InkApplicationGesture = 61456;
pub const IAG_DoubleCircle: InkApplicationGesture = 61473;
pub const IAG_DoubleCurlicue: InkApplicationGesture = 61457;
pub const IAG_DoubleTap: InkApplicationGesture = 61681;
pub const IAG_Down: InkApplicationGesture = 61529;
pub const IAG_DownLeft: InkApplicationGesture = 61546;
pub const IAG_DownLeftLong: InkApplicationGesture = 61542;
pub const IAG_DownRight: InkApplicationGesture = 61547;
pub const IAG_DownRightLong: InkApplicationGesture = 61543;
pub const IAG_DownUp: InkApplicationGesture = 61537;
pub const IAG_Exclamation: InkApplicationGesture = 61604;
pub const IAG_Left: InkApplicationGesture = 61530;
pub const IAG_LeftDown: InkApplicationGesture = 61549;
pub const IAG_LeftRight: InkApplicationGesture = 61538;
pub const IAG_LeftUp: InkApplicationGesture = 61548;
pub const IAG_NoGesture: InkApplicationGesture = 61440;
pub const IAG_Right: InkApplicationGesture = 61531;
pub const IAG_RightDown: InkApplicationGesture = 61551;
pub const IAG_RightLeft: InkApplicationGesture = 61539;
pub const IAG_RightUp: InkApplicationGesture = 61550;
pub const IAG_Scratchout: InkApplicationGesture = 61441;
pub const IAG_SemiCircleLeft: InkApplicationGesture = 61480;
pub const IAG_SemiCircleRight: InkApplicationGesture = 61481;
pub const IAG_Square: InkApplicationGesture = 61443;
pub const IAG_Star: InkApplicationGesture = 61444;
pub const IAG_Tap: InkApplicationGesture = 61680;
pub const IAG_Triangle: InkApplicationGesture = 61442;
pub const IAG_Up: InkApplicationGesture = 61528;
pub const IAG_UpDown: InkApplicationGesture = 61536;
pub const IAG_UpLeft: InkApplicationGesture = 61544;
pub const IAG_UpLeftLong: InkApplicationGesture = 61540;
pub const IAG_UpRight: InkApplicationGesture = 61545;
pub const IAG_UpRightLong: InkApplicationGesture = 61541;
pub const IBBM_CurveFit: InkBoundingBoxMode = 2;
pub const IBBM_Default: InkBoundingBoxMode = 0;
pub const IBBM_NoCurveFit: InkBoundingBoxMode = 1;
pub const IBBM_PointsOnly: InkBoundingBoxMode = 3;
pub const IBBM_Union: InkBoundingBoxMode = 4;
pub const ICBS_Down: InkCursorButtonState = 2;
pub const ICBS_Unavailable: InkCursorButtonState = 0;
pub const ICBS_Up: InkCursorButtonState = 1;
pub const ICB_Copy: InkClipboardModes = 0;
pub const ICB_Cut: InkClipboardModes = 1;
pub const ICB_Default: InkClipboardModes = 0;
pub const ICB_DelayedCopy: InkClipboardModes = 32;
pub const ICB_ExtractOnly: InkClipboardModes = 48;
pub const ICEI_AllEvents: InkCollectorEventInterest = 16;
pub const ICEI_CursorButtonDown: InkCollectorEventInterest = 4;
pub const ICEI_CursorButtonUp: InkCollectorEventInterest = 5;
pub const ICEI_CursorDown: InkCollectorEventInterest = 0;
pub const ICEI_CursorInRange: InkCollectorEventInterest = 6;
pub const ICEI_CursorOutOfRange: InkCollectorEventInterest = 7;
pub const ICEI_DblClick: InkCollectorEventInterest = 15;
pub const ICEI_DefaultEvents: InkCollectorEventInterest = -1;
pub const ICEI_MouseDown: InkCollectorEventInterest = 11;
pub const ICEI_MouseMove: InkCollectorEventInterest = 12;
pub const ICEI_MouseUp: InkCollectorEventInterest = 13;
pub const ICEI_MouseWheel: InkCollectorEventInterest = 14;
pub const ICEI_NewInAirPackets: InkCollectorEventInterest = 3;
pub const ICEI_NewPackets: InkCollectorEventInterest = 2;
pub const ICEI_Stroke: InkCollectorEventInterest = 1;
pub const ICEI_SystemGesture: InkCollectorEventInterest = 8;
pub const ICEI_TabletAdded: InkCollectorEventInterest = 9;
pub const ICEI_TabletRemoved: InkCollectorEventInterest = 10;
pub const ICF_Bitmap: InkClipboardFormats = 64;
pub const ICF_CopyMask: InkClipboardFormats = 127;
pub const ICF_Default: InkClipboardFormats = 127;
pub const ICF_EnhancedMetafile: InkClipboardFormats = 8;
pub const ICF_InkSerializedFormat: InkClipboardFormats = 1;
pub const ICF_Metafile: InkClipboardFormats = 32;
pub const ICF_None: InkClipboardFormats = 0;
pub const ICF_PasteMask: InkClipboardFormats = 7;
pub const ICF_SketchInk: InkClipboardFormats = 2;
pub const ICF_TextInk: InkClipboardFormats = 6;
pub const ICM_GestureOnly: InkCollectionMode = 1;
pub const ICM_InkAndGesture: InkCollectionMode = 2;
pub const ICM_InkOnly: InkCollectionMode = 0;
pub const IDM_Ink: InkDisplayMode = 0;
pub const IDM_Text: InkDisplayMode = 1;
pub const IDT_Drawing: InkDivisionType = 3;
pub const IDT_Line: InkDivisionType = 1;
pub const IDT_Paragraph: InkDivisionType = 2;
pub const IDT_Segment: InkDivisionType = 0;
pub const IECN_GESTURE: u32 = 2050;
pub const IECN_RECOGNITIONRESULT: u32 = 2051;
pub const IECN_STROKE: u32 = 2049;
pub const IECN__BASE: u32 = 2048;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Controls"))]
pub struct IEC_GESTUREINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: core::mem::ManuallyDrop<*mut core::ffi::c_void>,
    pub Strokes: core::mem::ManuallyDrop<*mut core::ffi::c_void>,
    pub Gestures: super::super::System::Variant::VARIANT,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Controls"))]
impl Clone for IEC_GESTUREINFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant", feature = "Win32_UI_Controls"))]
impl Default for IEC_GESTUREINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
#[derive(Clone, Debug, PartialEq)]
pub struct IEC_RECOGNITIONRESULTINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub RecognitionResult: core::mem::ManuallyDrop<*mut core::ffi::c_void>,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl Default for IEC_RECOGNITIONRESULTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
#[derive(Clone, Debug, PartialEq)]
pub struct IEC_STROKEINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: core::mem::ManuallyDrop<*mut core::ffi::c_void>,
    pub Stroke: core::mem::ManuallyDrop<*mut core::ffi::c_void>,
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Controls"))]
impl Default for IEC_STROKEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IEC__BASE: u32 = 1536;
pub const IEF_CopyFromOriginal: InkExtractFlags = 0;
pub const IEF_Default: InkExtractFlags = 1;
pub const IEF_RemoveFromOriginal: InkExtractFlags = 1;
pub const IEM_Disabled: InkMode = 0;
pub const IEM_Ink: InkMode = 1;
pub const IEM_InkAndGesture: InkMode = 2;
pub const IEM_InsertInk: InkInsertMode = 1;
pub const IEM_InsertText: InkInsertMode = 0;
pub const IES_Collecting: InkEditStatus = 1;
pub const IES_Idle: InkEditStatus = 0;
pub const IES_Recognizing: InkEditStatus = 2;
pub const IKM_Alt: InkShiftKeyModifierFlags = 4;
pub const IKM_Control: InkShiftKeyModifierFlags = 2;
pub const IKM_Shift: InkShiftKeyModifierFlags = 1;
pub const IMF_BOLD: INK_METRIC_FLAGS = 4;
pub const IMF_FONT_SELECTED_IN_HDC: INK_METRIC_FLAGS = 1;
pub const IMF_ITALIC: INK_METRIC_FLAGS = 2;
pub const IMF_Left: InkMouseButton = 1;
pub const IMF_Middle: InkMouseButton = 4;
pub const IMF_Right: InkMouseButton = 2;
pub const IMP_Arrow: InkMousePointer = 1;
pub const IMP_ArrowHourglass: InkMousePointer = 11;
pub const IMP_ArrowQuestion: InkMousePointer = 12;
pub const IMP_Crosshair: InkMousePointer = 2;
pub const IMP_Custom: InkMousePointer = 99;
pub const IMP_Default: InkMousePointer = 0;
pub const IMP_Hand: InkMousePointer = 14;
pub const IMP_Hourglass: InkMousePointer = 9;
pub const IMP_Ibeam: InkMousePointer = 3;
pub const IMP_NoDrop: InkMousePointer = 10;
pub const IMP_SizeAll: InkMousePointer = 13;
pub const IMP_SizeNESW: InkMousePointer = 4;
pub const IMP_SizeNS: InkMousePointer = 5;
pub const IMP_SizeNWSE: InkMousePointer = 6;
pub const IMP_SizeWE: InkMousePointer = 7;
pub const IMP_UpArrow: InkMousePointer = 8;
pub const INKEDIT_CLASS: windows_sys::core::PCWSTR = windows_sys::core::w!("INKEDIT");
pub const INKEDIT_CLASSW: windows_sys::core::PCWSTR = windows_sys::core::w!("INKEDIT");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INKMETRIC {
    pub iHeight: i32,
    pub iFontAscent: i32,
    pub iFontDescent: i32,
    pub dwFlags: u32,
    pub color: super::super::Foundation::COLORREF,
}
pub const INKRECOGNITIONPROPERTY_BOXNUMBER: windows_sys::core::PCWSTR = windows_sys::core::w!("{2C243E3A-F733-4EB6-B1F8-B5DC5C2C4CDA}");
pub const INKRECOGNITIONPROPERTY_CONFIDENCELEVEL: windows_sys::core::PCWSTR = windows_sys::core::w!("{7DFE11A7-FB5D-4958-8765-154ADF0D833F}");
pub const INKRECOGNITIONPROPERTY_HOTPOINT: windows_sys::core::PCWSTR = windows_sys::core::w!("{CA6F40DC-5292-452a-91FB-2181C0BEC0DE}");
pub const INKRECOGNITIONPROPERTY_LINEMETRICS: windows_sys::core::PCWSTR = windows_sys::core::w!("{8CC24B27-30A9-4b96-9056-2D3A90DA0727}");
pub const INKRECOGNITIONPROPERTY_LINENUMBER: windows_sys::core::PCWSTR = windows_sys::core::w!("{DBF29F2C-5289-4BE8-B3D8-6EF63246253E}");
pub const INKRECOGNITIONPROPERTY_MAXIMUMSTROKECOUNT: windows_sys::core::PCWSTR = windows_sys::core::w!("{BF0EEC4E-4B7D-47a9-8CFA-234DD24BD22A}");
pub const INKRECOGNITIONPROPERTY_POINTSPERINCH: windows_sys::core::PCWSTR = windows_sys::core::w!("{7ED16B76-889C-468e-8276-0021B770187E}");
pub const INKRECOGNITIONPROPERTY_SEGMENTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("{B3C0FE6C-FB51-4164-BA2F-844AF8F983DA}");
pub type INK_METRIC_FLAGS = i32;
pub const INK_SERIALIZED_FORMAT: windows_sys::core::PCWSTR = windows_sys::core::w!("Ink Serialized Format");
pub const IOAM_Behind: InkOverlayAttachMode = 0;
pub const IOAM_InFront: InkOverlayAttachMode = 1;
pub const IOEM_Delete: InkOverlayEditingMode = 1;
pub const IOEM_Ink: InkOverlayEditingMode = 0;
pub const IOEM_Select: InkOverlayEditingMode = 2;
pub const IOERM_PointErase: InkOverlayEraserMode = 1;
pub const IOERM_StrokeErase: InkOverlayEraserMode = 0;
pub const IPCM_Default: InkPersistenceCompressionMode = 0;
pub const IPCM_MaximumCompression: InkPersistenceCompressionMode = 1;
pub const IPCM_NoCompression: InkPersistenceCompressionMode = 2;
pub const IPF_Base64GIF: InkPersistenceFormat = 3;
pub const IPF_Base64InkSerializedFormat: InkPersistenceFormat = 1;
pub const IPF_GIF: InkPersistenceFormat = 2;
pub const IPF_InkSerializedFormat: InkPersistenceFormat = 0;
pub const IPSM_AutoSize: InkPictureSizeMode = 0;
pub const IPSM_CenterImage: InkPictureSizeMode = 1;
pub const IPSM_Normal: InkPictureSizeMode = 2;
pub const IPSM_StretchImage: InkPictureSizeMode = 3;
pub const IPT_Ball: InkPenTip = 0;
pub const IPT_Rectangle: InkPenTip = 1;
pub const IP_CURSOR_DOWN: u32 = 1;
pub const IP_INVERTED: u32 = 2;
pub const IP_MARGIN: u32 = 4;
pub const IRAS_All: InkRecognitionAlternatesSelection = -1;
pub const IRAS_DefaultCount: InkRecognitionAlternatesSelection = 10;
pub const IRAS_Start: InkRecognitionAlternatesSelection = 0;
pub const IRCACM_Full: InkRecognizerCharacterAutoCompletionMode = 0;
pub const IRCACM_Prefix: InkRecognizerCharacterAutoCompletionMode = 1;
pub const IRCACM_Random: InkRecognizerCharacterAutoCompletionMode = 2;
pub const IRC_AdviseInkChange: InkRecognizerCapabilities = 4096;
pub const IRC_Alpha: InkRecognizerCapabilities = 1048576;
pub const IRC_ArbitraryAngle: InkRecognizerCapabilities = 1024;
pub const IRC_Beta: InkRecognizerCapabilities = 2097152;
pub const IRC_BoxedInput: InkRecognizerCapabilities = 16;
pub const IRC_CharacterAutoCompletionInput: InkRecognizerCapabilities = 32;
pub const IRC_Cursive: InkRecognizerCapabilities = 262144;
pub const IRC_DontCare: InkRecognizerCapabilities = 1;
pub const IRC_DownAndLeft: InkRecognizerCapabilities = 256;
pub const IRC_DownAndRight: InkRecognizerCapabilities = 512;
pub const IRC_FreeInput: InkRecognizerCapabilities = 4;
pub const IRC_Intermediate: InkRecognitionConfidence = 1;
pub const IRC_Lattice: InkRecognizerCapabilities = 2048;
pub const IRC_LeftAndDown: InkRecognizerCapabilities = 128;
pub const IRC_LinedInput: InkRecognizerCapabilities = 8;
pub const IRC_Object: InkRecognizerCapabilities = 2;
pub const IRC_Personalizable: InkRecognizerCapabilities = 16384;
pub const IRC_Poor: InkRecognitionConfidence = 2;
pub const IRC_PrefersArbitraryAngle: InkRecognizerCapabilities = 32768;
pub const IRC_PrefersParagraphBreaking: InkRecognizerCapabilities = 65536;
pub const IRC_PrefersSegmentation: InkRecognizerCapabilities = 131072;
pub const IRC_RightAndDown: InkRecognizerCapabilities = 64;
pub const IRC_StrokeReorder: InkRecognizerCapabilities = 8192;
pub const IRC_Strong: InkRecognitionConfidence = 0;
pub const IRC_TextPrediction: InkRecognizerCapabilities = 524288;
pub const IRM_AutoSpace: InkRecognitionModes = 64;
pub const IRM_Coerce: InkRecognitionModes = 2;
pub const IRM_DisablePersonalization: InkRecognitionModes = 32;
pub const IRM_LineMode: InkRecognitionModes = 16;
pub const IRM_Max: InkRecognitionModes = 128;
pub const IRM_None: InkRecognitionModes = 0;
pub const IRM_PrefixOk: InkRecognitionModes = 8;
pub const IRM_TopInkBreaksOnly: InkRecognitionModes = 4;
pub const IRM_WordModeOnly: InkRecognitionModes = 1;
pub const IRO_Black: InkRasterOperation = 1;
pub const IRO_CopyPen: InkRasterOperation = 13;
pub const IRO_MaskNotPen: InkRasterOperation = 3;
pub const IRO_MaskPen: InkRasterOperation = 9;
pub const IRO_MaskPenNot: InkRasterOperation = 5;
pub const IRO_MergeNotPen: InkRasterOperation = 12;
pub const IRO_MergePen: InkRasterOperation = 15;
pub const IRO_MergePenNot: InkRasterOperation = 14;
pub const IRO_NoOperation: InkRasterOperation = 11;
pub const IRO_Not: InkRasterOperation = 6;
pub const IRO_NotCopyPen: InkRasterOperation = 4;
pub const IRO_NotMaskPen: InkRasterOperation = 8;
pub const IRO_NotMergePen: InkRasterOperation = 2;
pub const IRO_NotXOrPen: InkRasterOperation = 10;
pub const IRO_White: InkRasterOperation = 16;
pub const IRO_XOrPen: InkRasterOperation = 7;
pub const IRS_InkAddedFailed: InkRecognitionStatus = 4;
pub const IRS_Interrupted: InkRecognitionStatus = 1;
pub const IRS_NoError: InkRecognitionStatus = 0;
pub const IRS_ProcessFailed: InkRecognitionStatus = 2;
pub const IRS_SetAutoCompletionModeFailed: InkRecognitionStatus = 8;
pub const IRS_SetFactoidFailed: InkRecognitionStatus = 128;
pub const IRS_SetFlagsFailed: InkRecognitionStatus = 64;
pub const IRS_SetGuideFailed: InkRecognitionStatus = 32;
pub const IRS_SetPrefixSuffixFailed: InkRecognitionStatus = 256;
pub const IRS_SetStrokesFailed: InkRecognitionStatus = 16;
pub const IRS_SetWordListFailed: InkRecognitionStatus = 512;
pub const ISC_AllElements: InkSelectionConstants = -1;
pub const ISC_FirstElement: InkSelectionConstants = 0;
pub const ISG_DoubleTap: InkSystemGesture = 17;
pub const ISG_Drag: InkSystemGesture = 19;
pub const ISG_Flick: InkSystemGesture = 31;
pub const ISG_HoldEnter: InkSystemGesture = 21;
pub const ISG_HoldLeave: InkSystemGesture = 22;
pub const ISG_HoverEnter: InkSystemGesture = 23;
pub const ISG_HoverLeave: InkSystemGesture = 24;
pub const ISG_RightDrag: InkSystemGesture = 20;
pub const ISG_RightTap: InkSystemGesture = 18;
pub const ISG_Tap: InkSystemGesture = 16;
pub const InPlace: VisualState = 0;
pub type InPlaceDirection = i32;
pub const InPlaceDirection_Auto: InPlaceDirection = 0;
pub const InPlaceDirection_Bottom: InPlaceDirection = 1;
pub const InPlaceDirection_Top: InPlaceDirection = 2;
pub type InPlaceState = i32;
pub const InPlaceState_Auto: InPlaceState = 0;
pub const InPlaceState_Expanded: InPlaceState = 2;
pub const InPlaceState_HoverTarget: InPlaceState = 1;
pub const Ink: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x13de4a42_8d21_4c8e_bf9c_8f69cb068fca);
pub type InkApplicationGesture = i32;
pub type InkBoundingBoxMode = i32;
pub type InkClipboardFormats = i32;
pub type InkClipboardModes = i32;
pub type InkCollectionMode = i32;
pub const InkCollector: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43fb1553_ad74_4ee8_88e4_3e6daac915db);
pub const InkCollectorClipInkToMargin: i32 = 0;
pub const InkCollectorDefaultMargin: i32 = -2147483648;
pub type InkCollectorEventInterest = i32;
pub type InkCursorButtonState = i32;
pub const InkDisp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x937c1a34_151d_4610_9ca6_a8cc9bdb5d83);
pub type InkDisplayMode = i32;
pub const InkDivider: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8854f6a0_4683_4ae7_9191_752fe64612c3);
pub type InkDivisionType = i32;
pub const InkDrawingAttributes: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd8bf32a2_05a5_44c3_b3aa_5e80ac7d2576);
pub const InkEdit: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe5ca59f5_57c4_4dd8_9bd6_1deeedd27af4);
pub type InkEditStatus = i32;
pub type InkExtractFlags = i32;
pub type InkInsertMode = i32;
pub const InkMaxTransparencyValue: i32 = 255;
pub const InkMinTransparencyValue: i32 = 0;
pub type InkMode = i32;
pub type InkMouseButton = i32;
pub type InkMousePointer = i32;
pub const InkOverlay: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x65d00646_cde3_4a88_9163_6769f0f1a97d);
pub type InkOverlayAttachMode = i32;
pub type InkOverlayEditingMode = i32;
pub type InkOverlayEraserMode = i32;
pub type InkPenTip = i32;
pub type InkPersistenceCompressionMode = i32;
pub type InkPersistenceFormat = i32;
pub const InkPicture: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x04a1e553_fe36_4fde_865e_344194e69424);
pub type InkPictureSizeMode = i32;
pub type InkRasterOperation = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InkRecoGuide {
    pub rectWritingBox: super::super::Foundation::RECT,
    pub rectDrawnBox: super::super::Foundation::RECT,
    pub cRows: i32,
    pub cColumns: i32,
    pub midline: i32,
}
pub type InkRecognitionAlternatesSelection = i32;
pub type InkRecognitionConfidence = i32;
pub type InkRecognitionModes = i32;
pub type InkRecognitionStatus = i32;
pub type InkRecognizerCapabilities = i32;
pub type InkRecognizerCharacterAutoCompletionMode = i32;
pub const InkRecognizerContext: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaac46a37_9229_4fc0_8cce_4497569bf4d1);
pub const InkRecognizerGuide: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8770d941_a63a_4671_a375_2855a18eba73);
pub const InkRecognizers: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9fd4e808_f6e6_4e65_98d3_aa39054c1255);
pub const InkRectangle: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x43b07326_aae0_4b62_a83d_5fd768b7353c);
pub const InkRenderer: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9c1cc6e4_d7eb_4eeb_9091_15a7c8791ed9);
pub type InkSelectionConstants = i32;
pub type InkShiftKeyModifierFlags = i32;
pub const InkStrokes: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x48f491bc_240e_4860_b079_a1e94d3d2c86);
pub type InkSystemGesture = i32;
pub const InkTablets: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6e4fcb12_510a_4d40_9304_1da10ae9147c);
pub const InkTransform: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe3d5d93c_1663_4a78_a1a7_22375dfebaee);
pub const InkWordList: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9de85094_f71f_44f1_8471_15a2fa76fcf3);
pub type InteractionMode = i32;
pub const InteractionMode_DockedBottom: InteractionMode = 3;
pub const InteractionMode_DockedTop: InteractionMode = 2;
pub const InteractionMode_Floating: InteractionMode = 1;
pub const InteractionMode_InPlace: InteractionMode = 0;
pub type KEYMODIFIER = i32;
pub const KEYMODIFIER_ALTGR: KEYMODIFIER = 16;
pub const KEYMODIFIER_CONTROL: KEYMODIFIER = 1;
pub const KEYMODIFIER_EXT: KEYMODIFIER = 32;
pub const KEYMODIFIER_MENU: KEYMODIFIER = 2;
pub const KEYMODIFIER_SHIFT: KEYMODIFIER = 4;
pub const KEYMODIFIER_WIN: KEYMODIFIER = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LATTICE_METRICS {
    pub lsBaseline: LINE_SEGMENT,
    pub iMidlineOffset: i16,
}
pub const LEFT_BUTTON: MouseButton = 1;
pub type LINE_METRICS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LINE_SEGMENT {
    pub PtA: super::super::Foundation::POINT,
    pub PtB: super::super::Foundation::POINT,
}
pub const LM_ASCENDER: LINE_METRICS = 2;
pub const LM_BASELINE: LINE_METRICS = 0;
pub const LM_DESCENDER: LINE_METRICS = 3;
pub const LM_MIDLINE: LINE_METRICS = 1;
pub const MAX_FRIENDLYNAME: u32 = 64;
pub const MAX_LANGUAGES: u32 = 64;
pub const MAX_PACKET_BUTTON_COUNT: u32 = 32;
pub const MAX_PACKET_PROPERTY_COUNT: u32 = 32;
pub const MAX_VENDORNAME: u32 = 32;
pub const MICROSOFT_PENINPUT_PANEL_PROPERTY_T: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft PenInputPanel 1.5");
pub const MICROSOFT_TIP_COMBOBOXLIST_PROPERTY: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft TIP ComboBox List Window Identifier");
pub const MICROSOFT_TIP_NO_INSERT_BUTTON_PROPERTY: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft TIP No Insert Option");
pub const MICROSOFT_TIP_OPENING_MSG: windows_sys::core::PCWSTR = windows_sys::core::w!("TabletInputPanelOpening");
pub const MICROSOFT_URL_EXPERIENCE_PROPERTY: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft TIP URL Experience");
pub type MICUIELEMENT = i32;
pub type MICUIELEMENTSTATE = i32;
pub const MICUIELEMENTSTATE_DISABLED: MICUIELEMENTSTATE = 4;
pub const MICUIELEMENTSTATE_HOT: MICUIELEMENTSTATE = 2;
pub const MICUIELEMENTSTATE_NORMAL: MICUIELEMENTSTATE = 1;
pub const MICUIELEMENTSTATE_PRESSED: MICUIELEMENTSTATE = 3;
pub const MICUIELEMENT_BUTTON_CANCEL: MICUIELEMENT = 128;
pub const MICUIELEMENT_BUTTON_CLEAR: MICUIELEMENT = 8;
pub const MICUIELEMENT_BUTTON_CORRECT: MICUIELEMENT = 4;
pub const MICUIELEMENT_BUTTON_ERASE: MICUIELEMENT = 2;
pub const MICUIELEMENT_BUTTON_INSERT: MICUIELEMENT = 64;
pub const MICUIELEMENT_BUTTON_REDO: MICUIELEMENT = 32;
pub const MICUIELEMENT_BUTTON_UNDO: MICUIELEMENT = 16;
pub const MICUIELEMENT_BUTTON_WRITE: MICUIELEMENT = 1;
pub const MICUIELEMENT_INKPANEL_BACKGROUND: MICUIELEMENT = 256;
pub const MICUIELEMENT_RESULTPANEL_BACKGROUND: MICUIELEMENT = 512;
pub const MIDDLE_BUTTON: MouseButton = 4;
pub const MathInputControl: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc561816c_14d8_4090_830c_98d994b21c7b);
pub type MouseButton = i32;
pub const NO_BUTTON: MouseButton = 0;
pub const NUM_FLICK_DIRECTIONS: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PACKET_DESCRIPTION {
    pub cbPacketSize: u32,
    pub cPacketProperties: u32,
    pub pPacketProperties: *mut PACKET_PROPERTY,
    pub cButtons: u32,
    pub pguidButtons: *mut windows_sys::core::GUID,
}
impl Default for PACKET_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PACKET_PROPERTY {
    pub guid: windows_sys::core::GUID,
    pub PropertyMetrics: PROPERTY_METRICS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROPERTY_METRICS {
    pub nLogicalMin: i32,
    pub nLogicalMax: i32,
    pub Units: PROPERTY_UNITS,
    pub fResolution: f32,
}
pub type PROPERTY_UNITS = i32;
pub const PROPERTY_UNITS_AMPERE: PROPERTY_UNITS = 15;
pub const PROPERTY_UNITS_CANDELA: PROPERTY_UNITS = 16;
pub const PROPERTY_UNITS_CENTIMETERS: PROPERTY_UNITS = 2;
pub const PROPERTY_UNITS_DEFAULT: PROPERTY_UNITS = 0;
pub const PROPERTY_UNITS_DEGREES: PROPERTY_UNITS = 3;
pub const PROPERTY_UNITS_ENGLINEAR: PROPERTY_UNITS = 10;
pub const PROPERTY_UNITS_ENGROTATION: PROPERTY_UNITS = 11;
pub const PROPERTY_UNITS_FAHRENHEIT: PROPERTY_UNITS = 14;
pub const PROPERTY_UNITS_GRAMS: PROPERTY_UNITS = 7;
pub const PROPERTY_UNITS_INCHES: PROPERTY_UNITS = 1;
pub const PROPERTY_UNITS_KELVIN: PROPERTY_UNITS = 13;
pub const PROPERTY_UNITS_POUNDS: PROPERTY_UNITS = 6;
pub const PROPERTY_UNITS_RADIANS: PROPERTY_UNITS = 4;
pub const PROPERTY_UNITS_SECONDS: PROPERTY_UNITS = 5;
pub const PROPERTY_UNITS_SILINEAR: PROPERTY_UNITS = 8;
pub const PROPERTY_UNITS_SIROTATION: PROPERTY_UNITS = 9;
pub const PROPERTY_UNITS_SLUGS: PROPERTY_UNITS = 12;
pub const PT_Default: PanelType = 0;
pub const PT_Handwriting: PanelType = 2;
pub const PT_Inactive: PanelType = 1;
pub const PT_Keyboard: PanelType = 3;
pub type PanelInputArea = i32;
pub const PanelInputArea_Auto: PanelInputArea = 0;
pub const PanelInputArea_CharacterPad: PanelInputArea = 3;
pub const PanelInputArea_Keyboard: PanelInputArea = 1;
pub const PanelInputArea_WritingPad: PanelInputArea = 2;
pub type PanelType = i32;
pub const PenInputPanel: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf744e496_1b5a_489e_81dc_fbd7ac6298a8);
pub const PenInputPanel_Internal: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x802b1fb9_056b_4720_b0cc_80d23b71171e);
pub type PfnRecoCallback = Option<unsafe extern "system" fn(param0: u32, param1: *mut u8, param2: HRECOCONTEXT) -> windows_sys::core::HRESULT>;
pub const RECOCONF_HIGHCONFIDENCE: u32 = 1;
pub const RECOCONF_LOWCONFIDENCE: i32 = -1;
pub const RECOCONF_MEDIUMCONFIDENCE: u32 = 0;
pub const RECOCONF_NOTSET: u32 = 128;
pub const RECOFLAG_AUTOSPACE: u32 = 64;
pub const RECOFLAG_COERCE: u32 = 2;
pub const RECOFLAG_DISABLEPERSONALIZATION: u32 = 32;
pub const RECOFLAG_LINEMODE: u32 = 16;
pub const RECOFLAG_PREFIXOK: u32 = 8;
pub const RECOFLAG_SINGLESEG: u32 = 4;
pub const RECOFLAG_WORDMODE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RECO_ATTRS {
    pub dwRecoCapabilityFlags: u32,
    pub awcVendorName: [u16; 32],
    pub awcFriendlyName: [u16; 64],
    pub awLanguageId: [u16; 64],
}
impl Default for RECO_ATTRS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RECO_LATTICE {
    pub ulColumnCount: u32,
    pub pLatticeColumns: *mut RECO_LATTICE_COLUMN,
    pub ulPropertyCount: u32,
    pub pGuidProperties: *mut windows_sys::core::GUID,
    pub ulBestResultColumnCount: u32,
    pub pulBestResultColumns: *mut u32,
    pub pulBestResultIndexes: *mut u32,
}
impl Default for RECO_LATTICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RECO_LATTICE_COLUMN {
    pub key: u32,
    pub cpProp: RECO_LATTICE_PROPERTIES,
    pub cStrokes: u32,
    pub pStrokes: *mut u32,
    pub cLatticeElements: u32,
    pub pLatticeElements: *mut RECO_LATTICE_ELEMENT,
}
impl Default for RECO_LATTICE_COLUMN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RECO_LATTICE_ELEMENT {
    pub score: i32,
    pub r#type: u16,
    pub pData: *mut u8,
    pub ulNextColumn: u32,
    pub ulStrokeNumber: u32,
    pub epProp: RECO_LATTICE_PROPERTIES,
}
impl Default for RECO_LATTICE_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RECO_LATTICE_PROPERTIES {
    pub cProperties: u32,
    pub apProps: *mut *mut RECO_LATTICE_PROPERTY,
}
impl Default for RECO_LATTICE_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RECO_LATTICE_PROPERTY {
    pub guidProperty: windows_sys::core::GUID,
    pub cbPropertyValue: u16,
    pub pPropertyValue: *mut u8,
}
impl Default for RECO_LATTICE_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RECO_RANGE {
    pub iwcBegin: u32,
    pub cCount: u32,
}
pub type RECO_TYPE = i32;
pub const RECO_TYPE_WCHAR: RECO_TYPE = 1;
pub const RECO_TYPE_WSTRING: RECO_TYPE = 0;
pub const RF_ADVISEINKCHANGE: i32 = 4096;
pub const RF_ARBITRARY_ANGLE: i32 = 1024;
pub const RF_BOXED_INPUT: i32 = 16;
pub const RF_CAC_INPUT: i32 = 32;
pub const RF_DONTCARE: i32 = 1;
pub const RF_DOWN_AND_LEFT: i32 = 256;
pub const RF_DOWN_AND_RIGHT: i32 = 512;
pub const RF_FREE_INPUT: i32 = 4;
pub const RF_LATTICE: i32 = 2048;
pub const RF_LEFT_AND_DOWN: i32 = 128;
pub const RF_LINED_INPUT: i32 = 8;
pub const RF_OBJECT: i32 = 2;
pub const RF_PERFORMSLINEBREAKING: i32 = 65536;
pub const RF_PERSONALIZABLE: i32 = 16384;
pub const RF_REQUIRESSEGMENTATIONBREAKING: i32 = 131072;
pub const RF_RIGHT_AND_DOWN: i32 = 64;
pub const RF_STROKEREORDER: i32 = 8192;
pub const RIGHT_BUTTON: MouseButton = 2;
pub const RTSDI_AllData: RealTimeStylusDataInterest = -1;
pub const RTSDI_CustomStylusDataAdded: RealTimeStylusDataInterest = 32768;
pub const RTSDI_DefaultEvents: RealTimeStylusDataInterest = 37766;
pub const RTSDI_Error: RealTimeStylusDataInterest = 1;
pub const RTSDI_InAirPackets: RealTimeStylusDataInterest = 32;
pub const RTSDI_None: RealTimeStylusDataInterest = 0;
pub const RTSDI_Packets: RealTimeStylusDataInterest = 256;
pub const RTSDI_RealTimeStylusDisabled: RealTimeStylusDataInterest = 4;
pub const RTSDI_RealTimeStylusEnabled: RealTimeStylusDataInterest = 2;
pub const RTSDI_StylusButtonDown: RealTimeStylusDataInterest = 2048;
pub const RTSDI_StylusButtonUp: RealTimeStylusDataInterest = 1024;
pub const RTSDI_StylusDown: RealTimeStylusDataInterest = 128;
pub const RTSDI_StylusInRange: RealTimeStylusDataInterest = 16;
pub const RTSDI_StylusNew: RealTimeStylusDataInterest = 8;
pub const RTSDI_StylusOutOfRange: RealTimeStylusDataInterest = 64;
pub const RTSDI_StylusUp: RealTimeStylusDataInterest = 512;
pub const RTSDI_SystemEvents: RealTimeStylusDataInterest = 4096;
pub const RTSDI_TabletAdded: RealTimeStylusDataInterest = 8192;
pub const RTSDI_TabletRemoved: RealTimeStylusDataInterest = 16384;
pub const RTSDI_UpdateMapping: RealTimeStylusDataInterest = 65536;
pub const RTSLT_AsyncEventLock: RealTimeStylusLockType = 4;
pub const RTSLT_AsyncObjLock: RealTimeStylusLockType = 13;
pub const RTSLT_ExcludeCallback: RealTimeStylusLockType = 8;
pub const RTSLT_ObjLock: RealTimeStylusLockType = 1;
pub const RTSLT_SyncEventLock: RealTimeStylusLockType = 2;
pub const RTSLT_SyncObjLock: RealTimeStylusLockType = 11;
pub const RealTimeStylus: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe26b366d_f998_43ce_836f_cb6d904432b0);
pub type RealTimeStylusDataInterest = i32;
pub type RealTimeStylusLockType = i32;
pub const SAFE_PARTIAL: u32 = 1;
pub type SCROLLDIRECTION = i32;
pub const SCROLLDIRECTION_DOWN: SCROLLDIRECTION = 1;
pub const SCROLLDIRECTION_UP: SCROLLDIRECTION = 0;
pub const SHR_E: SelectionHitResult = 5;
pub const SHR_N: SelectionHitResult = 7;
pub const SHR_NE: SelectionHitResult = 3;
pub const SHR_NW: SelectionHitResult = 1;
pub const SHR_None: SelectionHitResult = 0;
pub const SHR_S: SelectionHitResult = 8;
pub const SHR_SE: SelectionHitResult = 2;
pub const SHR_SW: SelectionHitResult = 4;
pub const SHR_Selection: SelectionHitResult = 9;
pub const SHR_W: SelectionHitResult = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct STROKE_RANGE {
    pub iStrokeBegin: u32,
    pub iStrokeEnd: u32,
}
pub const STR_GUID_ALTITUDEORIENTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("{82DEC5C7-F6BA-4906-894F-66D68DFC456C}");
pub const STR_GUID_AZIMUTHORIENTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("{029123B4-8828-410B-B250-A0536595E5DC}");
pub const STR_GUID_BUTTONPRESSURE: windows_sys::core::PCWSTR = windows_sys::core::w!("{8B7FEFC4-96AA-4BFE-AC26-8A5F0BE07BF5}");
pub const STR_GUID_DEVICE_CONTACT_ID: windows_sys::core::PCWSTR = windows_sys::core::w!("{02585B91-049B-4750-9615-DF8948AB3C9C}");
pub const STR_GUID_FINGERCONTACTCONFIDENCE: windows_sys::core::PCWSTR = windows_sys::core::w!("{E706C804-57F0-4F00-8A0C-853D57789BE9}");
pub const STR_GUID_HEIGHT: windows_sys::core::PCWSTR = windows_sys::core::w!("{E61858D2-E447-4218-9D3F-18865C203DF4}");
pub const STR_GUID_NORMALPRESSURE: windows_sys::core::PCWSTR = windows_sys::core::w!("{7307502D-F9F4-4E18-B3F2-2CE1B1A3610C}");
pub const STR_GUID_PAKETSTATUS: windows_sys::core::PCWSTR = windows_sys::core::w!("{6E0E07BF-AFE7-4CF7-87D1-AF6446208418}");
pub const STR_GUID_PITCHROTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("{7F7E57B7-BE37-4BE1-A356-7A84160E1893}");
pub const STR_GUID_ROLLROTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("{5D5D5E56-6BA9-4C5B-9FB0-851C91714E56}");
pub const STR_GUID_SERIALNUMBER: windows_sys::core::PCWSTR = windows_sys::core::w!("{78A81B56-0935-4493-BAAE-00541A8A16C4}");
pub const STR_GUID_TANGENTPRESSURE: windows_sys::core::PCWSTR = windows_sys::core::w!("{6DA4488B-5244-41EC-905B-32D89AB80809}");
pub const STR_GUID_TIMERTICK: windows_sys::core::PCWSTR = windows_sys::core::w!("{436510C5-FED3-45D1-8B76-71D3EA7A829D}");
pub const STR_GUID_TWISTORIENTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("{0D324960-13B2-41E4-ACE6-7AE9D43D2D3B}");
pub const STR_GUID_WIDTH: windows_sys::core::PCWSTR = windows_sys::core::w!("{BAABE94D-2712-48F5-BE9D-8F8B5EA0711A}");
pub const STR_GUID_X: windows_sys::core::PCWSTR = windows_sys::core::w!("{598A6A8F-52C0-4BA0-93AF-AF357411A561}");
pub const STR_GUID_XTILTORIENTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("{A8D07B3A-8BF0-40B0-95A9-B80A6BB787BF}");
pub const STR_GUID_Y: windows_sys::core::PCWSTR = windows_sys::core::w!("{B53F9F75-04E0-4498-A7EE-C30DBB5A9011}");
pub const STR_GUID_YAWROTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("{6A849980-7C3A-45B7-AA82-90A262950E89}");
pub const STR_GUID_YTILTORIENTATION: windows_sys::core::PCWSTR = windows_sys::core::w!("{0E932389-1D77-43AF-AC00-5B950D6D4B2D}");
pub const STR_GUID_Z: windows_sys::core::PCWSTR = windows_sys::core::w!("{735ADB30-0EBB-4788-A0E4-0F316490055D}");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_EVENT_DATA {
    pub bModifier: u8,
    pub wKey: u16,
    pub xPos: i32,
    pub yPos: i32,
    pub bCursorMode: u8,
    pub dwButtonState: u32,
}
pub type ScrollBarsConstants = i32;
pub type SelAlignmentConstants = i32;
pub type SelectionHitResult = i32;
pub const SketchInk: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf0291081_e87c_4e07_97da_a0a03761e586);
pub const StrokeBuilder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe810cee7_6e51_4cb0_aa3a_0b985b70daf7);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StylusInfo {
    pub tcid: u32,
    pub cid: u32,
    pub bIsInvertedCursor: windows_sys::core::BOOL,
}
pub type StylusQueue = i32;
pub const SyncStylusQueue: StylusQueue = 1;
pub const TABLET_DISABLE_FLICKFALLBACKKEYS: u32 = 1048576;
pub const TABLET_DISABLE_FLICKS: u32 = 65536;
pub const TABLET_DISABLE_PENBARRELFEEDBACK: u32 = 16;
pub const TABLET_DISABLE_PENTAPFEEDBACK: u32 = 8;
pub const TABLET_DISABLE_PRESSANDHOLD: u32 = 1;
pub const TABLET_DISABLE_SMOOTHSCROLLING: u32 = 524288;
pub const TABLET_DISABLE_TOUCHSWITCH: u32 = 32768;
pub const TABLET_DISABLE_TOUCHUIFORCEOFF: u32 = 512;
pub const TABLET_DISABLE_TOUCHUIFORCEON: u32 = 256;
pub const TABLET_ENABLE_FLICKLEARNINGMODE: u32 = 262144;
pub const TABLET_ENABLE_FLICKSONCONTEXT: u32 = 131072;
pub const TABLET_ENABLE_MULTITOUCHDATA: u32 = 16777216;
pub const TCF_ALLOW_RECOGNITION: GET_DANDIDATE_FLAGS = 1;
pub const TCF_FORCE_RECOGNITION: GET_DANDIDATE_FLAGS = 2;
pub const TDK_Mouse: TabletDeviceKind = 0;
pub const TDK_Pen: TabletDeviceKind = 1;
pub const TDK_Touch: TabletDeviceKind = 2;
pub const THWC_CursorMustTouch: TabletHardwareCapabilities = 2;
pub const THWC_CursorsHavePhysicalIds: TabletHardwareCapabilities = 8;
pub const THWC_HardProximity: TabletHardwareCapabilities = 4;
pub const THWC_Integrated: TabletHardwareCapabilities = 1;
pub const TPMU_Centimeters: TabletPropertyMetricUnit = 2;
pub const TPMU_Default: TabletPropertyMetricUnit = 0;
pub const TPMU_Degrees: TabletPropertyMetricUnit = 3;
pub const TPMU_Grams: TabletPropertyMetricUnit = 7;
pub const TPMU_Inches: TabletPropertyMetricUnit = 1;
pub const TPMU_Pounds: TabletPropertyMetricUnit = 6;
pub const TPMU_Radians: TabletPropertyMetricUnit = 4;
pub const TPMU_Seconds: TabletPropertyMetricUnit = 5;
pub type TabletDeviceKind = i32;
pub type TabletHardwareCapabilities = i32;
pub type TabletPropertyMetricUnit = i32;
pub const TextInputPanel: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf9b189d7_228b_4f2b_8650_b97f59e02c8c);
pub const TipAutoCompleteClient: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x807c1e6c_1d00_453f_b920_b61bb7cdd997);
pub type VisualState = i32;
pub const WM_TABLET_ADDED: u32 = 712;
pub const WM_TABLET_DEFBASE: u32 = 704;
pub const WM_TABLET_DELETED: u32 = 713;
pub const WM_TABLET_FLICK: u32 = 715;
pub const WM_TABLET_MAXOFFSET: u32 = 32;
pub const WM_TABLET_QUERYSYSTEMGESTURESTATUS: u32 = 716;
pub const rtfBoth: ScrollBarsConstants = 3;
pub const rtfCenter: SelAlignmentConstants = 2;
pub const rtfFixedSingle: BorderStyleConstants = 1;
pub const rtfFlat: AppearanceConstants = 0;
pub const rtfHorizontal: ScrollBarsConstants = 1;
pub const rtfLeft: SelAlignmentConstants = 0;
pub const rtfNoBorder: BorderStyleConstants = 0;
pub const rtfNone: ScrollBarsConstants = 0;
pub const rtfRight: SelAlignmentConstants = 1;
pub const rtfThreeD: AppearanceConstants = 1;
pub const rtfVertical: ScrollBarsConstants = 2;
