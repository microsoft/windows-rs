#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ALT_BREAKS(pub i32);
pub const ALT_BREAKS_SAME: ALT_BREAKS = ALT_BREAKS(0i32);
pub const ALT_BREAKS_UNIQUE: ALT_BREAKS = ALT_BREAKS(1i32);
pub const ALT_BREAKS_FULL: ALT_BREAKS = ALT_BREAKS(2i32);
impl ::std::convert::From<i32> for ALT_BREAKS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ALT_BREAKS {
    type Abi = Self;
}
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
#[cfg(feature = "Win32_Graphics_Gdi")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Graphics_Gdi`*"]
#[inline]
pub unsafe fn AddStroke<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, ppacketdesc: *const PACKET_DESCRIPTION, cbpacket: u32, ppacket: *const u8, pxform: *const super::super::Graphics::Gdi::XFORM) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddStroke(hrc: HRECOCONTEXT, ppacketdesc: *const PACKET_DESCRIPTION, cbpacket: u32, ppacket: *const u8, pxform: *const super::super::Graphics::Gdi::XFORM) -> ::windows::runtime::HRESULT;
        }
        AddStroke(hrc.into_param().abi(), ::std::mem::transmute(ppacketdesc), ::std::mem::transmute(cbpacket), ::std::mem::transmute(ppacket), ::std::mem::transmute(pxform)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn AddWordsToWordList<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOWORDLIST>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hwl: Param0, pwcwords: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddWordsToWordList(hwl: HRECOWORDLIST, pwcwords: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        AddWordsToWordList(hwl.into_param().abi(), pwcwords.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn AdviseInkChange<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hrc: Param0, bnewstroke: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AdviseInkChange(hrc: HRECOCONTEXT, bnewstroke: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        AdviseInkChange(hrc.into_param().abi(), bnewstroke.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AppearanceConstants(pub i32);
pub const rtfFlat: AppearanceConstants = AppearanceConstants(0i32);
pub const rtfThreeD: AppearanceConstants = AppearanceConstants(1i32);
impl ::std::convert::From<i32> for AppearanceConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AppearanceConstants {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const BEST_COMPLETE: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BorderStyleConstants(pub i32);
pub const rtfNoBorder: BorderStyleConstants = BorderStyleConstants(0i32);
pub const rtfFixedSingle: BorderStyleConstants = BorderStyleConstants(1i32);
impl ::std::convert::From<i32> for BorderStyleConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BorderStyleConstants {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const CAC_FULL: u32 = 0u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const CAC_PREFIX: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const CAC_RANDOM: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct CHARACTER_RANGE {
    pub wcLow: u16,
    pub cChars: u16,
}
impl CHARACTER_RANGE {}
impl ::std::default::Default for CHARACTER_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for CHARACTER_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("CHARACTER_RANGE").field("wcLow", &self.wcLow).field("cChars", &self.cChars).finish()
    }
}
impl ::std::cmp::PartialEq for CHARACTER_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.wcLow == other.wcLow && self.cChars == other.cChars
    }
}
impl ::std::cmp::Eq for CHARACTER_RANGE {}
unsafe impl ::windows::runtime::Abi for CHARACTER_RANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONFIDENCE_LEVEL(pub i32);
pub const CFL_STRONG: CONFIDENCE_LEVEL = CONFIDENCE_LEVEL(0i32);
pub const CFL_INTERMEDIATE: CONFIDENCE_LEVEL = CONFIDENCE_LEVEL(1i32);
pub const CFL_POOR: CONFIDENCE_LEVEL = CONFIDENCE_LEVEL(2i32);
impl ::std::convert::From<i32> for CONFIDENCE_LEVEL {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONFIDENCE_LEVEL {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CorrectionMode(pub i32);
pub const CorrectionMode_NotVisible: CorrectionMode = CorrectionMode(0i32);
pub const CorrectionMode_PreInsertion: CorrectionMode = CorrectionMode(1i32);
pub const CorrectionMode_PostInsertionCollapsed: CorrectionMode = CorrectionMode(2i32);
pub const CorrectionMode_PostInsertionExpanded: CorrectionMode = CorrectionMode(3i32);
impl ::std::convert::From<i32> for CorrectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CorrectionMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CorrectionPosition(pub i32);
pub const CorrectionPosition_Auto: CorrectionPosition = CorrectionPosition(0i32);
pub const CorrectionPosition_Bottom: CorrectionPosition = CorrectionPosition(1i32);
pub const CorrectionPosition_Top: CorrectionPosition = CorrectionPosition(2i32);
impl ::std::convert::From<i32> for CorrectionPosition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CorrectionPosition {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn CreateContext<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOGNIZER>>(hrec: Param0, phrc: *mut HRECOCONTEXT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateContext(hrec: HRECOGNIZER, phrc: *mut HRECOCONTEXT) -> ::windows::runtime::HRESULT;
        }
        CreateContext(hrec.into_param().abi(), ::std::mem::transmute(phrc)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn CreateRecognizer(pclsid: *mut ::windows::runtime::GUID, phrec: *mut HRECOGNIZER) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRecognizer(pclsid: *mut ::windows::runtime::GUID, phrec: *mut HRECOGNIZER) -> ::windows::runtime::HRESULT;
        }
        CreateRecognizer(::std::mem::transmute(pclsid), ::std::mem::transmute(phrec)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_Ink {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_Ink {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkCollector {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkCollector {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkCollectorEvent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkCollectorEvent {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkCursor(pub i32);
pub const DISPID_ICsrName: DISPID_InkCursor = DISPID_InkCursor(0i32);
pub const DISPID_ICsrId: DISPID_InkCursor = DISPID_InkCursor(1i32);
pub const DISPID_ICsrDrawingAttributes: DISPID_InkCursor = DISPID_InkCursor(2i32);
pub const DISPID_ICsrButtons: DISPID_InkCursor = DISPID_InkCursor(3i32);
pub const DISPID_ICsrInverted: DISPID_InkCursor = DISPID_InkCursor(4i32);
pub const DISPID_ICsrTablet: DISPID_InkCursor = DISPID_InkCursor(5i32);
impl ::std::convert::From<i32> for DISPID_InkCursor {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkCursor {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkCursorButton(pub i32);
pub const DISPID_ICBName: DISPID_InkCursorButton = DISPID_InkCursorButton(0i32);
pub const DISPID_ICBId: DISPID_InkCursorButton = DISPID_InkCursorButton(1i32);
pub const DISPID_ICBState: DISPID_InkCursorButton = DISPID_InkCursorButton(2i32);
impl ::std::convert::From<i32> for DISPID_InkCursorButton {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkCursorButton {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkCursorButtons(pub i32);
pub const DISPID_ICBs_NewEnum: DISPID_InkCursorButtons = DISPID_InkCursorButtons(-4i32);
pub const DISPID_ICBsItem: DISPID_InkCursorButtons = DISPID_InkCursorButtons(0i32);
pub const DISPID_ICBsCount: DISPID_InkCursorButtons = DISPID_InkCursorButtons(1i32);
impl ::std::convert::From<i32> for DISPID_InkCursorButtons {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkCursorButtons {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkCursors(pub i32);
pub const DISPID_ICs_NewEnum: DISPID_InkCursors = DISPID_InkCursors(-4i32);
pub const DISPID_ICsItem: DISPID_InkCursors = DISPID_InkCursors(0i32);
pub const DISPID_ICsCount: DISPID_InkCursors = DISPID_InkCursors(1i32);
impl ::std::convert::From<i32> for DISPID_InkCursors {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkCursors {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkCustomStrokes(pub i32);
pub const DISPID_ICSs_NewEnum: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(-4i32);
pub const DISPID_ICSsItem: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(0i32);
pub const DISPID_ICSsCount: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(1i32);
pub const DISPID_ICSsAdd: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(2i32);
pub const DISPID_ICSsRemove: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(3i32);
pub const DISPID_ICSsClear: DISPID_InkCustomStrokes = DISPID_InkCustomStrokes(4i32);
impl ::std::convert::From<i32> for DISPID_InkCustomStrokes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkCustomStrokes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkDivider(pub i32);
pub const DISPID_IInkDivider_Strokes: DISPID_InkDivider = DISPID_InkDivider(1i32);
pub const DISPID_IInkDivider_RecognizerContext: DISPID_InkDivider = DISPID_InkDivider(2i32);
pub const DISPID_IInkDivider_LineHeight: DISPID_InkDivider = DISPID_InkDivider(3i32);
pub const DISPID_IInkDivider_Divide: DISPID_InkDivider = DISPID_InkDivider(4i32);
impl ::std::convert::From<i32> for DISPID_InkDivider {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkDivider {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkDivisionResult(pub i32);
pub const DISPID_IInkDivisionResult_Strokes: DISPID_InkDivisionResult = DISPID_InkDivisionResult(1i32);
pub const DISPID_IInkDivisionResult_ResultByType: DISPID_InkDivisionResult = DISPID_InkDivisionResult(2i32);
impl ::std::convert::From<i32> for DISPID_InkDivisionResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkDivisionResult {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkDivisionUnit(pub i32);
pub const DISPID_IInkDivisionUnit_Strokes: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(1i32);
pub const DISPID_IInkDivisionUnit_DivisionType: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(2i32);
pub const DISPID_IInkDivisionUnit_RecognizedString: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(3i32);
pub const DISPID_IInkDivisionUnit_RotationTransform: DISPID_InkDivisionUnit = DISPID_InkDivisionUnit(4i32);
impl ::std::convert::From<i32> for DISPID_InkDivisionUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkDivisionUnit {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkDivisionUnits(pub i32);
pub const DISPID_IInkDivisionUnits_NewEnum: DISPID_InkDivisionUnits = DISPID_InkDivisionUnits(-4i32);
pub const DISPID_IInkDivisionUnits_Item: DISPID_InkDivisionUnits = DISPID_InkDivisionUnits(0i32);
pub const DISPID_IInkDivisionUnits_Count: DISPID_InkDivisionUnits = DISPID_InkDivisionUnits(1i32);
impl ::std::convert::From<i32> for DISPID_InkDivisionUnits {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkDivisionUnits {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkDrawingAttributes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkDrawingAttributes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkEdit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkEdit {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkEditEvents {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkEditEvents {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkEvent(pub i32);
pub const DISPID_IEInkAdded: DISPID_InkEvent = DISPID_InkEvent(1i32);
pub const DISPID_IEInkDeleted: DISPID_InkEvent = DISPID_InkEvent(2i32);
impl ::std::convert::From<i32> for DISPID_InkEvent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkEvent {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkExtendedProperties(pub i32);
pub const DISPID_IEPs_NewEnum: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(-4i32);
pub const DISPID_IEPsItem: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(0i32);
pub const DISPID_IEPsCount: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(1i32);
pub const DISPID_IEPsAdd: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(2i32);
pub const DISPID_IEPsRemove: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(3i32);
pub const DISPID_IEPsClear: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(4i32);
pub const DISPID_IEPsDoesPropertyExist: DISPID_InkExtendedProperties = DISPID_InkExtendedProperties(5i32);
impl ::std::convert::From<i32> for DISPID_InkExtendedProperties {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkExtendedProperties {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkExtendedProperty(pub i32);
pub const DISPID_IEPGuid: DISPID_InkExtendedProperty = DISPID_InkExtendedProperty(1i32);
pub const DISPID_IEPData: DISPID_InkExtendedProperty = DISPID_InkExtendedProperty(2i32);
impl ::std::convert::From<i32> for DISPID_InkExtendedProperty {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkExtendedProperty {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkGesture(pub i32);
pub const DISPID_IGId: DISPID_InkGesture = DISPID_InkGesture(0i32);
pub const DISPID_IGGetHotPoint: DISPID_InkGesture = DISPID_InkGesture(1i32);
pub const DISPID_IGConfidence: DISPID_InkGesture = DISPID_InkGesture(2i32);
impl ::std::convert::From<i32> for DISPID_InkGesture {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkGesture {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkRecoAlternate {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecoAlternate {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkRecoContext {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecoContext {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkRecoContext2(pub i32);
pub const DISPID_IRecoCtx2_EnabledUnicodeRanges: DISPID_InkRecoContext2 = DISPID_InkRecoContext2(0i32);
impl ::std::convert::From<i32> for DISPID_InkRecoContext2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecoContext2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkRecognitionAlternates(pub i32);
pub const DISPID_InkRecognitionAlternates_NewEnum: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(-4i32);
pub const DISPID_InkRecognitionAlternates_Item: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(0i32);
pub const DISPID_InkRecognitionAlternates_Count: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(1i32);
pub const DISPID_InkRecognitionAlternates_Strokes: DISPID_InkRecognitionAlternates = DISPID_InkRecognitionAlternates(2i32);
impl ::std::convert::From<i32> for DISPID_InkRecognitionAlternates {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecognitionAlternates {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkRecognitionEvent(pub i32);
pub const DISPID_IRERecognitionWithAlternates: DISPID_InkRecognitionEvent = DISPID_InkRecognitionEvent(1i32);
pub const DISPID_IRERecognition: DISPID_InkRecognitionEvent = DISPID_InkRecognitionEvent(2i32);
impl ::std::convert::From<i32> for DISPID_InkRecognitionEvent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecognitionEvent {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkRecognitionResult(pub i32);
pub const DISPID_InkRecognitionResult_TopString: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(1i32);
pub const DISPID_InkRecognitionResult_TopAlternate: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(2i32);
pub const DISPID_InkRecognitionResult_Strokes: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(3i32);
pub const DISPID_InkRecognitionResult_TopConfidence: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(4i32);
pub const DISPID_InkRecognitionResult_AlternatesFromSelection: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(5i32);
pub const DISPID_InkRecognitionResult_ModifyTopAlternate: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(6i32);
pub const DISPID_InkRecognitionResult_SetResultOnStrokes: DISPID_InkRecognitionResult = DISPID_InkRecognitionResult(7i32);
impl ::std::convert::From<i32> for DISPID_InkRecognitionResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecognitionResult {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkRecognizer {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecognizer {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkRecognizer2(pub i32);
pub const DISPID_RecoId: DISPID_InkRecognizer2 = DISPID_InkRecognizer2(0i32);
pub const DISPID_RecoUnicodeRanges: DISPID_InkRecognizer2 = DISPID_InkRecognizer2(1i32);
impl ::std::convert::From<i32> for DISPID_InkRecognizer2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecognizer2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkRecognizerGuide(pub i32);
pub const DISPID_IRGWritingBox: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(1i32);
pub const DISPID_IRGDrawnBox: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(2i32);
pub const DISPID_IRGRows: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(3i32);
pub const DISPID_IRGColumns: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(4i32);
pub const DISPID_IRGMidline: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(5i32);
pub const DISPID_IRGGuideData: DISPID_InkRecognizerGuide = DISPID_InkRecognizerGuide(6i32);
impl ::std::convert::From<i32> for DISPID_InkRecognizerGuide {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecognizerGuide {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkRecognizers(pub i32);
pub const DISPID_IRecos_NewEnum: DISPID_InkRecognizers = DISPID_InkRecognizers(-4i32);
pub const DISPID_IRecosItem: DISPID_InkRecognizers = DISPID_InkRecognizers(0i32);
pub const DISPID_IRecosCount: DISPID_InkRecognizers = DISPID_InkRecognizers(1i32);
pub const DISPID_IRecosGetDefaultRecognizer: DISPID_InkRecognizers = DISPID_InkRecognizers(2i32);
impl ::std::convert::From<i32> for DISPID_InkRecognizers {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRecognizers {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkRectangle(pub i32);
pub const DISPID_IRTop: DISPID_InkRectangle = DISPID_InkRectangle(1i32);
pub const DISPID_IRLeft: DISPID_InkRectangle = DISPID_InkRectangle(2i32);
pub const DISPID_IRBottom: DISPID_InkRectangle = DISPID_InkRectangle(3i32);
pub const DISPID_IRRight: DISPID_InkRectangle = DISPID_InkRectangle(4i32);
pub const DISPID_IRGetRectangle: DISPID_InkRectangle = DISPID_InkRectangle(5i32);
pub const DISPID_IRSetRectangle: DISPID_InkRectangle = DISPID_InkRectangle(6i32);
pub const DISPID_IRData: DISPID_InkRectangle = DISPID_InkRectangle(7i32);
impl ::std::convert::From<i32> for DISPID_InkRectangle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRectangle {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkRenderer {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkRenderer {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkStrokeDisp {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkStrokeDisp {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkStrokes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkStrokes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkTablet(pub i32);
pub const DISPID_ITName: DISPID_InkTablet = DISPID_InkTablet(0i32);
pub const DISPID_ITPlugAndPlayId: DISPID_InkTablet = DISPID_InkTablet(1i32);
pub const DISPID_ITPropertyMetrics: DISPID_InkTablet = DISPID_InkTablet(2i32);
pub const DISPID_ITIsPacketPropertySupported: DISPID_InkTablet = DISPID_InkTablet(3i32);
pub const DISPID_ITMaximumInputRectangle: DISPID_InkTablet = DISPID_InkTablet(4i32);
pub const DISPID_ITHardwareCapabilities: DISPID_InkTablet = DISPID_InkTablet(5i32);
impl ::std::convert::From<i32> for DISPID_InkTablet {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkTablet {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkTablet2(pub i32);
pub const DISPID_IT2DeviceKind: DISPID_InkTablet2 = DISPID_InkTablet2(0i32);
impl ::std::convert::From<i32> for DISPID_InkTablet2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkTablet2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkTablet3(pub i32);
pub const DISPID_IT3IsMultiTouch: DISPID_InkTablet3 = DISPID_InkTablet3(0i32);
pub const DISPID_IT3MaximumCursors: DISPID_InkTablet3 = DISPID_InkTablet3(1i32);
impl ::std::convert::From<i32> for DISPID_InkTablet3 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkTablet3 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkTablets(pub i32);
pub const DISPID_ITs_NewEnum: DISPID_InkTablets = DISPID_InkTablets(-4i32);
pub const DISPID_ITsItem: DISPID_InkTablets = DISPID_InkTablets(0i32);
pub const DISPID_ITsDefaultTablet: DISPID_InkTablets = DISPID_InkTablets(1i32);
pub const DISPID_ITsCount: DISPID_InkTablets = DISPID_InkTablets(2i32);
pub const DISPID_ITsIsPacketPropertySupported: DISPID_InkTablets = DISPID_InkTablets(3i32);
impl ::std::convert::From<i32> for DISPID_InkTablets {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkTablets {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_InkTransform {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkTransform {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkWordList(pub i32);
pub const DISPID_InkWordList_AddWord: DISPID_InkWordList = DISPID_InkWordList(0i32);
pub const DISPID_InkWordList_RemoveWord: DISPID_InkWordList = DISPID_InkWordList(1i32);
pub const DISPID_InkWordList_Merge: DISPID_InkWordList = DISPID_InkWordList(2i32);
impl ::std::convert::From<i32> for DISPID_InkWordList {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkWordList {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_InkWordList2(pub i32);
pub const DISPID_InkWordList2_AddWords: DISPID_InkWordList2 = DISPID_InkWordList2(3i32);
impl ::std::convert::From<i32> for DISPID_InkWordList2 {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_InkWordList2 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_MathInputControlEvents(pub i32);
pub const DISPID_MICInsert: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(0i32);
pub const DISPID_MICClose: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(1i32);
pub const DISPID_MICPaint: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(2i32);
pub const DISPID_MICClear: DISPID_MathInputControlEvents = DISPID_MathInputControlEvents(3i32);
impl ::std::convert::From<i32> for DISPID_MathInputControlEvents {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_MathInputControlEvents {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for DISPID_PenInputPanel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_PenInputPanel {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_PenInputPanelEvents(pub i32);
pub const DISPID_PIPEVisibleChanged: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(0i32);
pub const DISPID_PIPEPanelChanged: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(1i32);
pub const DISPID_PIPEInputFailed: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(2i32);
pub const DISPID_PIPEPanelMoving: DISPID_PenInputPanelEvents = DISPID_PenInputPanelEvents(3i32);
impl ::std::convert::From<i32> for DISPID_PenInputPanelEvents {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_PenInputPanelEvents {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPID_StrokeEvent(pub i32);
pub const DISPID_SEStrokesAdded: DISPID_StrokeEvent = DISPID_StrokeEvent(1i32);
pub const DISPID_SEStrokesRemoved: DISPID_StrokeEvent = DISPID_StrokeEvent(2i32);
impl ::std::convert::From<i32> for DISPID_StrokeEvent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPID_StrokeEvent {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct DYNAMIC_RENDERER_CACHED_DATA {
    pub strokeId: i32,
    pub dynamicRenderer: ::std::option::Option<IDynamicRenderer>,
}
impl DYNAMIC_RENDERER_CACHED_DATA {}
impl ::std::default::Default for DYNAMIC_RENDERER_CACHED_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DYNAMIC_RENDERER_CACHED_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DYNAMIC_RENDERER_CACHED_DATA").field("strokeId", &self.strokeId).field("dynamicRenderer", &self.dynamicRenderer).finish()
    }
}
impl ::std::cmp::PartialEq for DYNAMIC_RENDERER_CACHED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.strokeId == other.strokeId && self.dynamicRenderer == other.dynamicRenderer
    }
}
impl ::std::cmp::Eq for DYNAMIC_RENDERER_CACHED_DATA {}
unsafe impl ::windows::runtime::Abi for DYNAMIC_RENDERER_CACHED_DATA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn DestroyContext<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyContext(hrc: HRECOCONTEXT) -> ::windows::runtime::HRESULT;
        }
        DestroyContext(hrc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn DestroyRecognizer<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOGNIZER>>(hrec: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyRecognizer(hrec: HRECOGNIZER) -> ::windows::runtime::HRESULT;
        }
        DestroyRecognizer(hrec.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn DestroyWordList<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOWORDLIST>>(hwl: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DestroyWordList(hwl: HRECOWORDLIST) -> ::windows::runtime::HRESULT;
        }
        DestroyWordList(hwl.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DynamicRenderer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3973262058, 29807, 19915, [191, 104, 8, 39, 87, 250, 255, 24]);
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
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn EndInkInput<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EndInkInput(hrc: HRECOCONTEXT) -> ::windows::runtime::HRESULT;
        }
        EndInkInput(hrc.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for EventMask {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EventMask {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const FACILITY_INK: u32 = 40u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FLICKACTION_COMMANDCODE(pub i32);
pub const FLICKACTION_COMMANDCODE_NULL: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(0i32);
pub const FLICKACTION_COMMANDCODE_SCROLL: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(1i32);
pub const FLICKACTION_COMMANDCODE_APPCOMMAND: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(2i32);
pub const FLICKACTION_COMMANDCODE_CUSTOMKEY: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(3i32);
pub const FLICKACTION_COMMANDCODE_KEYMODIFIER: FLICKACTION_COMMANDCODE = FLICKACTION_COMMANDCODE(4i32);
impl ::std::convert::From<i32> for FLICKACTION_COMMANDCODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FLICKACTION_COMMANDCODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for FLICKDIRECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FLICKDIRECTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FLICKMODE(pub i32);
pub const FLICKMODE_MIN: FLICKMODE = FLICKMODE(0i32);
pub const FLICKMODE_OFF: FLICKMODE = FLICKMODE(0i32);
pub const FLICKMODE_ON: FLICKMODE = FLICKMODE(1i32);
pub const FLICKMODE_LEARNING: FLICKMODE = FLICKMODE(2i32);
pub const FLICKMODE_MAX: FLICKMODE = FLICKMODE(2i32);
pub const FLICKMODE_DEFAULT: FLICKMODE = FLICKMODE(1i32);
impl ::std::convert::From<i32> for FLICKMODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FLICKMODE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct FLICK_DATA {
    pub _bitfield: i32,
}
impl FLICK_DATA {}
impl ::std::default::Default for FLICK_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FLICK_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FLICK_DATA").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for FLICK_DATA {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for FLICK_DATA {}
unsafe impl ::windows::runtime::Abi for FLICK_DATA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct FLICK_POINT {
    pub _bitfield: i32,
}
impl FLICK_POINT {}
impl ::std::default::Default for FLICK_POINT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FLICK_POINT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FLICK_POINT").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for FLICK_POINT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for FLICK_POINT {}
unsafe impl ::windows::runtime::Abi for FLICK_POINT {
    type Abi = Self;
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct GESTURE_DATA {
    pub gestureId: i32,
    pub recoConfidence: i32,
    pub strokeCount: i32,
}
impl GESTURE_DATA {}
impl ::std::default::Default for GESTURE_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GESTURE_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GESTURE_DATA").field("gestureId", &self.gestureId).field("recoConfidence", &self.recoConfidence).field("strokeCount", &self.strokeCount).finish()
    }
}
impl ::std::cmp::PartialEq for GESTURE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.gestureId == other.gestureId && self.recoConfidence == other.recoConfidence && self.strokeCount == other.strokeCount
    }
}
impl ::std::cmp::Eq for GESTURE_DATA {}
unsafe impl ::windows::runtime::Abi for GESTURE_DATA {
    type Abi = Self;
}
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
pub const GUID_DYNAMIC_RENDERER_CACHED_DATA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3209894802, 9663, 19093, [137, 173, 14, 71, 107, 52, 180, 245]);
pub const GUID_GESTURE_DATA: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1105521679, 9898, 17754, [154, 165, 44, 211, 108, 246, 63, 185]);
pub const GUID_PACKETPROPERTY_GUID_ALTITUDE_ORIENTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2195637703, 63162, 18694, [137, 79, 102, 214, 141, 252, 69, 108]);
pub const GUID_PACKETPROPERTY_GUID_AZIMUTH_ORIENTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(43066292, 34856, 16651, [178, 80, 160, 83, 101, 149, 229, 220]);
pub const GUID_PACKETPROPERTY_GUID_BUTTON_PRESSURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2340417476, 38570, 19454, [172, 38, 138, 95, 11, 224, 123, 245]);
pub const GUID_PACKETPROPERTY_GUID_DEVICE_CONTACT_ID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(39345041, 1179, 18256, [150, 21, 223, 137, 72, 171, 60, 156]);
pub const GUID_PACKETPROPERTY_GUID_FINGERCONTACTCONFIDENCE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3875981316, 22512, 20224, [138, 12, 133, 61, 87, 120, 155, 233]);
pub const GUID_PACKETPROPERTY_GUID_HEIGHT: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3860355282, 58439, 16920, [157, 63, 24, 134, 92, 32, 61, 244]);
pub const GUID_PACKETPROPERTY_GUID_NORMAL_PRESSURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1929859117, 63988, 19992, [179, 242, 44, 225, 177, 163, 97, 12]);
pub const GUID_PACKETPROPERTY_GUID_PACKET_STATUS: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1846413247, 45031, 19703, [135, 209, 175, 100, 70, 32, 132, 24]);
pub const GUID_PACKETPROPERTY_GUID_PITCH_ROTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2138986423, 48695, 19425, [163, 86, 122, 132, 22, 14, 24, 147]);
pub const GUID_PACKETPROPERTY_GUID_ROLL_ROTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1566400086, 27561, 19547, [159, 176, 133, 28, 145, 113, 78, 86]);
pub const GUID_PACKETPROPERTY_GUID_SERIAL_NUMBER: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2024282966, 2357, 17555, [186, 174, 0, 84, 26, 138, 22, 196]);
pub const GUID_PACKETPROPERTY_GUID_TANGENT_PRESSURE: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1839483019, 21060, 16876, [144, 91, 50, 216, 154, 184, 8, 9]);
pub const GUID_PACKETPROPERTY_GUID_TIMER_TICK: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1130696901, 65235, 17873, [139, 118, 113, 211, 234, 122, 130, 157]);
pub const GUID_PACKETPROPERTY_GUID_TWIST_ORIENTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(221399392, 5042, 16868, [172, 230, 122, 233, 212, 61, 45, 59]);
pub const GUID_PACKETPROPERTY_GUID_WIDTH: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3131828557, 10002, 18677, [190, 157, 143, 139, 94, 160, 113, 26]);
pub const GUID_PACKETPROPERTY_GUID_X: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1502243471, 21184, 19360, [147, 175, 175, 53, 116, 17, 165, 97]);
pub const GUID_PACKETPROPERTY_GUID_X_TILT_ORIENTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2832235322, 35824, 16560, [149, 169, 184, 10, 107, 183, 135, 191]);
pub const GUID_PACKETPROPERTY_GUID_Y: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3040845685, 1248, 17560, [167, 238, 195, 13, 187, 90, 144, 17]);
pub const GUID_PACKETPROPERTY_GUID_YAW_ROTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1787074944, 31802, 17847, [170, 130, 144, 162, 98, 149, 14, 137]);
pub const GUID_PACKETPROPERTY_GUID_Y_TILT_ORIENTATION: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(244523913, 7543, 17327, [172, 0, 91, 149, 13, 109, 75, 45]);
pub const GUID_PACKETPROPERTY_GUID_Z: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1935334192, 3771, 18312, [160, 228, 15, 49, 100, 144, 5, 93]);
pub const GestureRecognizer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3929065044, 50732, 17439, [172, 0, 149, 249, 161, 150, 120, 44]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn GetAllRecognizers(recognizerclsids: *mut *mut ::windows::runtime::GUID, count: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAllRecognizers(recognizerclsids: *mut *mut ::windows::runtime::GUID, count: *mut u32) -> ::windows::runtime::HRESULT;
        }
        GetAllRecognizers(::std::mem::transmute(recognizerclsids), ::std::mem::transmute(count)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetBestResultString<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pcsize: *mut u32, pwcbestresult: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetBestResultString(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcbestresult: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        GetBestResultString(hrc.into_param().abi(), ::std::mem::transmute(pcsize), ::std::mem::transmute(pwcbestresult)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn GetLatticePtr<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pplattice: *mut *mut RECO_LATTICE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLatticePtr(hrc: HRECOCONTEXT, pplattice: *mut *mut RECO_LATTICE) -> ::windows::runtime::HRESULT;
        }
        GetLatticePtr(hrc.into_param().abi(), ::std::mem::transmute(pplattice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetLeftSeparator<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pcsize: *mut u32, pwcleftseparator: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetLeftSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcleftseparator: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        GetLeftSeparator(hrc.into_param().abi(), ::std::mem::transmute(pcsize), ::std::mem::transmute(pwcleftseparator)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn GetRecoAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOGNIZER>>(hrec: Param0, precoattrs: *mut RECO_ATTRS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRecoAttributes(hrec: HRECOGNIZER, precoattrs: *mut RECO_ATTRS) -> ::windows::runtime::HRESULT;
        }
        GetRecoAttributes(hrec.into_param().abi(), ::std::mem::transmute(precoattrs)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn GetResultPropertyList<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOGNIZER>>(hrec: Param0, ppropertycount: *mut u32, ppropertyguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetResultPropertyList(hrec: HRECOGNIZER, ppropertycount: *mut u32, ppropertyguid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT;
        }
        GetResultPropertyList(hrec.into_param().abi(), ::std::mem::transmute(ppropertycount), ::std::mem::transmute(ppropertyguid)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn GetRightSeparator<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pcsize: *mut u32, pwcrightseparator: super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRightSeparator(hrc: HRECOCONTEXT, pcsize: *mut u32, pwcrightseparator: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        GetRightSeparator(hrc.into_param().abi(), ::std::mem::transmute(pcsize), ::std::mem::transmute(pwcrightseparator)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn GetUnicodeRanges<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOGNIZER>>(hrec: Param0, pcranges: *mut u32, pcr: *mut CHARACTER_RANGE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUnicodeRanges(hrec: HRECOGNIZER, pcranges: *mut u32, pcr: *mut CHARACTER_RANGE) -> ::windows::runtime::HRESULT;
        }
        GetUnicodeRanges(hrec.into_param().abi(), ::std::mem::transmute(pcranges), ::std::mem::transmute(pcr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HRECOALT(pub isize);
impl ::std::default::Default for HRECOALT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HRECOALT {}
unsafe impl ::windows::runtime::Abi for HRECOALT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HRECOCONTEXT(pub isize);
impl ::std::default::Default for HRECOCONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HRECOCONTEXT {}
unsafe impl ::windows::runtime::Abi for HRECOCONTEXT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HRECOGNIZER(pub isize);
impl ::std::default::Default for HRECOGNIZER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HRECOGNIZER {}
unsafe impl ::windows::runtime::Abi for HRECOGNIZER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HRECOLATTICE(pub isize);
impl ::std::default::Default for HRECOLATTICE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HRECOLATTICE {}
unsafe impl ::windows::runtime::Abi for HRECOLATTICE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HRECOWORDLIST(pub isize);
impl ::std::default::Default for HRECOWORDLIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HRECOWORDLIST {}
unsafe impl ::windows::runtime::Abi for HRECOWORDLIST {
    type Abi = Self;
}
pub const HandwrittenTextInsertion: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2668056290, 59113, 19850, [160, 71, 235, 91, 92, 60, 85, 218]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDynamicRenderer(pub ::windows::runtime::IUnknown);
impl IDynamicRenderer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, benabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), benabled.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn HWND(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE_PTR> {
        let mut result__: <super::super::Foundation::HANDLE_PTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE_PTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetHWND<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, hwnd: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn ClipRectangle(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetClipRectangle(&self, prccliprect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(prccliprect)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn ClipRegion(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE_PTR> {
        let mut result__: <super::super::Foundation::HANDLE_PTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE_PTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetClipRegion<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, hcliprgn: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), hcliprgn.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DrawingAttributes(&self) -> ::windows::runtime::Result<IInkDrawingAttributes> {
        let mut result__: <IInkDrawingAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_DrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, pida: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pida.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn DataCacheEnabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetDataCacheEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fcachedata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), fcachedata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ReleaseCachedData(&self, strokeid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(strokeid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Draw<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, hdc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), hdc.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDynamicRenderer {
    type Vtable = IDynamicRenderer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2692302478, 29029, 18169, [183, 175, 152, 173, 1, 169, 48, 9]);
}
impl ::std::convert::From<IDynamicRenderer> for ::windows::runtime::IUnknown {
    fn from(value: IDynamicRenderer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDynamicRenderer> for ::windows::runtime::IUnknown {
    fn from(value: &IDynamicRenderer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDynamicRenderer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDynamicRenderer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDynamicRenderer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, benabled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prccliprect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prccliprect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phcliprgn: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hcliprgn: super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppida: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pida: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfcachedata: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fcachedata: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokeid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IECN_GESTURE: u32 = 2050u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IECN_RECOGNITIONRESULT: u32 = 2051u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IECN_STROKE: u32 = 2049u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IECN__BASE: u32 = 2048u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Controls"))]
impl ::std::clone::Clone for IEC_GESTUREINFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Controls"))]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`, `Win32_UI_Controls`*"]
pub struct IEC_GESTUREINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: ::std::option::Option<IInkCursor>,
    pub Strokes: ::std::option::Option<IInkStrokes>,
    pub Gestures: super::super::System::Com::VARIANT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Controls"))]
impl IEC_GESTUREINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Controls"))]
impl ::std::default::Default for IEC_GESTUREINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::PartialEq for IEC_GESTUREINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::Eq for IEC_GESTUREINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::runtime::Abi for IEC_GESTUREINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_UI_Controls`*"]
pub struct IEC_RECOGNITIONRESULTINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub RecognitionResult: ::std::option::Option<IInkRecognitionResult>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl IEC_RECOGNITIONRESULTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::default::Default for IEC_RECOGNITIONRESULTINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::fmt::Debug for IEC_RECOGNITIONRESULTINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IEC_RECOGNITIONRESULTINFO").field("nmhdr", &self.nmhdr).field("RecognitionResult", &self.RecognitionResult).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::PartialEq for IEC_RECOGNITIONRESULTINFO {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.RecognitionResult == other.RecognitionResult
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::Eq for IEC_RECOGNITIONRESULTINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::runtime::Abi for IEC_RECOGNITIONRESULTINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_UI_Controls`*"]
pub struct IEC_STROKEINFO {
    pub nmhdr: super::Controls::NMHDR,
    pub Cursor: ::std::option::Option<IInkCursor>,
    pub Stroke: ::std::option::Option<IInkStrokeDisp>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl IEC_STROKEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::default::Default for IEC_STROKEINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::fmt::Debug for IEC_STROKEINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IEC_STROKEINFO").field("nmhdr", &self.nmhdr).field("Cursor", &self.Cursor).field("Stroke", &self.Stroke).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::PartialEq for IEC_STROKEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.nmhdr == other.nmhdr && self.Cursor == other.Cursor && self.Stroke == other.Stroke
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
impl ::std::cmp::Eq for IEC_STROKEINFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
unsafe impl ::windows::runtime::Abi for IEC_STROKEINFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IEC__BASE: u32 = 1536u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IGestureRecognizer(pub ::windows::runtime::IUnknown);
impl IGestureRecognizer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenabled: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fenabled.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MaxStrokeCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMaxStrokeCount(&self, cstrokes: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(cstrokes)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EnableGestures(&self, cgestures: u32, pgestures: *const i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(cgestures), ::std::mem::transmute(pgestures)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IGestureRecognizer {
    type Vtable = IGestureRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2929653867, 28756, 17891, [174, 34, 49, 116, 220, 136, 17, 183]);
}
impl ::std::convert::From<IGestureRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: IGestureRecognizer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGestureRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: &IGestureRecognizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IGestureRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGestureRecognizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenabled: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcstrokes: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cstrokes: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cgestures: u32, pgestures: *const i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IHandwrittenTextInsertion(pub ::windows::runtime::IUnknown);
impl IHandwrittenTextInsertion {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn InsertRecognitionResultsArray<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(psaalternates), ::std::mem::transmute(locale), falternatecontainsautospacinginformation.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InsertInkRecognitionResult<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRecognitionResult>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, piinkrecoresult: Param0, locale: u32, falternatecontainsautospacinginformation: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), piinkrecoresult.into_param().abi(), ::std::mem::transmute(locale), falternatecontainsautospacinginformation.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHandwrittenTextInsertion {
    type Vtable = IHandwrittenTextInsertion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1459481239, 60630, 17383, [170, 58, 129, 107, 231, 120, 88, 96]);
}
impl ::std::convert::From<IHandwrittenTextInsertion> for ::windows::runtime::IUnknown {
    fn from(value: IHandwrittenTextInsertion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IHandwrittenTextInsertion> for ::windows::runtime::IUnknown {
    fn from(value: &IHandwrittenTextInsertion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IHandwrittenTextInsertion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IHandwrittenTextInsertion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IHandwrittenTextInsertion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psaalternates: *const super::super::System::Com::SAFEARRAY, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piinkrecoresult: ::windows::runtime::RawPtr, locale: u32, falternatecontainsautospacinginformation: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInk(pub ::windows::runtime::IUnknown);
impl IInk {}
unsafe impl ::windows::runtime::Interface for IInk {
    type Vtable = IInk_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(66643217, 17313, 4563, [139, 182, 0, 128, 199, 214, 186, 213]);
}
impl ::std::convert::From<IInk> for ::windows::runtime::IUnknown {
    fn from(value: IInk) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInk> for ::windows::runtime::IUnknown {
    fn from(value: &IInk) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInk {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInk {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInk> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInk) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInk> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInk) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInk {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInk {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInk_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkCollector(pub ::windows::runtime::IUnknown);
impl IInkCollector {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn hWnd(&self) -> ::windows::runtime::Result<isize> {
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SethWnd(&self, newwindow: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(newwindow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEnabled(&self, collecting: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(collecting)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DefaultDrawingAttributes(&self) -> ::windows::runtime::Result<IInkDrawingAttributes> {
        let mut result__: <IInkDrawingAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_DefaultDrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, newattributes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), newattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Renderer(&self) -> ::windows::runtime::Result<IInkRenderer> {
        let mut result__: <IInkRenderer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRenderer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Renderer<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRenderer>>(&self, newinkrenderer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), newinkrenderer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Ink(&self) -> ::windows::runtime::Result<IInkDisp> {
        let mut result__: <IInkDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Ink<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDisp>>(&self, newink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), newink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AutoRedraw(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAutoRedraw(&self, autoredraw: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(autoredraw)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CollectingInk(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CollectionMode(&self) -> ::windows::runtime::Result<InkCollectionMode> {
        let mut result__: <InkCollectionMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkCollectionMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetCollectionMode(&self, mode: InkCollectionMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DynamicRendering(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDynamicRendering(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn DesiredPacketDescription(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetDesiredPacketDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, packetguids: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), packetguids.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn MouseIcon(&self) -> ::windows::runtime::Result<super::super::System::Ole::IPictureDisp> {
        let mut result__: <super::super::System::Ole::IPictureDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IPictureDisp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn SetMouseIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), mouseicon.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn putref_MouseIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), mouseicon.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MousePointer(&self) -> ::windows::runtime::Result<InkMousePointer> {
        let mut result__: <InkMousePointer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkMousePointer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(mousepointer)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Cursors(&self) -> ::windows::runtime::Result<IInkCursors> {
        let mut result__: <IInkCursors as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkCursors>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MarginX(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMarginX(&self, marginx: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(marginx)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MarginY(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMarginY(&self, marginy: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), ::std::mem::transmute(marginy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Tablet(&self) -> ::windows::runtime::Result<IInkTablet> {
        let mut result__: <IInkTablet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkTablet>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SupportHighContrastInk(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSupportHighContrastInk(&self, support: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(support)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(gesture), ::std::mem::transmute(listen)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(gesture), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetWindowInputRectangle(&self, windowinputrectangle: *mut ::std::option::Option<IInkRectangle>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(windowinputrectangle)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetWindowInputRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, windowinputrectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), windowinputrectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAllTabletsMode(&self, usemouseforinput: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), ::std::mem::transmute(usemouseforinput)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSingleTabletIntegratedMode<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTablet>>(&self, tablet: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), tablet.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventid), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventid), ::std::mem::transmute(listen)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkCollector {
    type Vtable = IInkCollector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4042285237, 35615, 19068, [137, 236, 136, 6, 146, 88, 138, 79]);
}
impl ::std::convert::From<IInkCollector> for ::windows::runtime::IUnknown {
    fn from(value: IInkCollector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkCollector> for ::windows::runtime::IUnknown {
    fn from(value: &IInkCollector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkCollector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkCollector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkCollector> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkCollector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkCollector> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkCollector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkCollector {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkCollector {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCollector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentwindow: *mut isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newwindow: isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collecting: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collecting: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newattributes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentinkrenderer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newinkrenderer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, autoredraw: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, autoredraw: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collecting: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: *mut InkCollectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: InkCollectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetguids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetguids: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mousepointer: *mut InkMousePointer) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mousepointer: InkMousePointer) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cursors: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginx: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginx: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginy: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginy: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, singletablet: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gesture: InkApplicationGesture, listen: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windowinputrectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windowinputrectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usemouseforinput: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tablet: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkCursor(pub ::windows::runtime::IUnknown);
impl IInkCursor {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Inverted(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DrawingAttributes(&self) -> ::windows::runtime::Result<IInkDrawingAttributes> {
        let mut result__: <IInkDrawingAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_DrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, attributes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), attributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Tablet(&self) -> ::windows::runtime::Result<IInkTablet> {
        let mut result__: <IInkTablet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkTablet>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Buttons(&self) -> ::windows::runtime::Result<IInkCursorButtons> {
        let mut result__: <IInkCursorButtons as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkCursorButtons>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkCursor {
    type Vtable = IInkCursor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2905654832, 16581, 17232, [132, 5, 156, 113, 1, 47, 197, 88]);
}
impl ::std::convert::From<IInkCursor> for ::windows::runtime::IUnknown {
    fn from(value: IInkCursor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkCursor> for ::windows::runtime::IUnknown {
    fn from(value: &IInkCursor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkCursor> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkCursor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkCursor> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkCursor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkCursor {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCursor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attributes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tablet: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, buttons: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkCursorButton(pub ::windows::runtime::IUnknown);
impl IInkCursorButton {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn State(&self) -> ::windows::runtime::Result<InkCursorButtonState> {
        let mut result__: <InkCursorButtonState as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkCursorButtonState>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkCursorButton {
    type Vtable = IInkCursorButton_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2247070743, 7513, 18866, [161, 60, 112, 44, 133, 67, 8, 148]);
}
impl ::std::convert::From<IInkCursorButton> for ::windows::runtime::IUnknown {
    fn from(value: IInkCursorButton) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkCursorButton> for ::windows::runtime::IUnknown {
    fn from(value: &IInkCursorButton) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkCursorButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkCursorButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkCursorButton> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkCursorButton) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkCursorButton> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkCursorButton) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkCursorButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkCursorButton {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCursorButton_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentstate: *mut InkCursorButtonState) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkCursorButtons(pub ::windows::runtime::IUnknown);
impl IInkCursorButtons {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, identifier: Param0) -> ::windows::runtime::Result<IInkCursorButton> {
        let mut result__: <IInkCursorButton as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), identifier.into_param().abi(), &mut result__).from_abi::<IInkCursorButton>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkCursorButtons {
    type Vtable = IInkCursorButtons_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(913427520, 46628, 18033, [159, 160, 219, 17, 157, 149, 45, 84]);
}
impl ::std::convert::From<IInkCursorButtons> for ::windows::runtime::IUnknown {
    fn from(value: IInkCursorButtons) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkCursorButtons> for ::windows::runtime::IUnknown {
    fn from(value: &IInkCursorButtons) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkCursorButtons {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkCursorButtons {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkCursorButtons> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkCursorButtons) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkCursorButtons> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkCursorButtons) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkCursorButtons {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkCursorButtons {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCursorButtons_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, button: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkCursors(pub ::windows::runtime::IUnknown);
impl IInkCursors {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<IInkCursor> {
        let mut result__: <IInkCursor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<IInkCursor>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkCursors {
    type Vtable = IInkCursors_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2722677164, 50840, 19974, [158, 92, 213, 127, 119, 199, 230, 71]);
}
impl ::std::convert::From<IInkCursors> for ::windows::runtime::IUnknown {
    fn from(value: IInkCursors) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkCursors> for ::windows::runtime::IUnknown {
    fn from(value: &IInkCursors) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkCursors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkCursors {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkCursors> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkCursors) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkCursors> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkCursors) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkCursors {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkCursors {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCursors_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, cursor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkCustomStrokes(pub ::windows::runtime::IUnknown);
impl IInkCustomStrokes {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, identifier: Param0) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), identifier.into_param().abi(), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, name: Param0, strokes: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), name.into_param().abi(), strokes.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, identifier: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), identifier.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkCustomStrokes {
    type Vtable = IInkCustomStrokes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2116266127, 49934, 16911, [155, 219, 40, 144, 37, 67, 240, 193]);
}
impl ::std::convert::From<IInkCustomStrokes> for ::windows::runtime::IUnknown {
    fn from(value: IInkCustomStrokes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkCustomStrokes> for ::windows::runtime::IUnknown {
    fn from(value: &IInkCustomStrokes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkCustomStrokes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkCustomStrokes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkCustomStrokes> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkCustomStrokes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkCustomStrokes> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkCustomStrokes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkCustomStrokes {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkCustomStrokes {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkCustomStrokes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, strokes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkDisp(pub ::windows::runtime::IUnknown);
impl IInkDisp {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Strokes(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ExtendedProperties(&self) -> ::windows::runtime::Result<IInkExtendedProperties> {
        let mut result__: <IInkExtendedProperties as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkExtendedProperties>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Dirty(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDirty(&self, dirty: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(dirty)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CustomStrokes(&self) -> ::windows::runtime::Result<IInkCustomStrokes> {
        let mut result__: <IInkCustomStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkCustomStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> ::windows::runtime::Result<IInkRectangle> {
        let mut result__: <IInkRectangle as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(boundingboxmode), &mut result__).from_abi::<IInkRectangle>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DeleteStrokes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), strokes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DeleteStroke<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokeDisp>>(&self, stroke: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), stroke.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ExtractStrokes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0, extractflags: InkExtractFlags) -> ::windows::runtime::Result<IInkDisp> {
        let mut result__: <IInkDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), strokes.into_param().abi(), ::std::mem::transmute(extractflags), &mut result__).from_abi::<IInkDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ExtractWithRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0, extractflags: InkExtractFlags) -> ::windows::runtime::Result<IInkDisp> {
        let mut result__: <IInkDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), rectangle.into_param().abi(), ::std::mem::transmute(extractflags), &mut result__).from_abi::<IInkDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Clip<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), rectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IInkDisp> {
        let mut result__: <IInkDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn HitTestCircle(&self, x: i32, y: i32, radius: f32) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(radius), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn HitTestWithRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, selectionrectangle: Param0, intersectpercent: f32) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), selectionrectangle.into_param().abi(), ::std::mem::transmute(intersectpercent), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn HitTestWithLasso<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, points: Param0, intersectpercent: f32, lassopoints: *mut super::super::System::Com::VARIANT, strokes: *mut ::std::option::Option<IInkStrokes>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), points.into_param().abi(), ::std::mem::transmute(intersectpercent), ::std::mem::transmute(lassopoints), ::std::mem::transmute(strokes)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn NearestPoint(&self, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut ::std::option::Option<IInkStrokeDisp>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(pointonstroke), ::std::mem::transmute(distancefrompacket), ::std::mem::transmute(stroke)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn CreateStrokes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, strokeids: Param0) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), strokeids.into_param().abi(), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AddStrokesAtRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>, Param1: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, sourcestrokes: Param0, targetrectangle: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), sourcestrokes.into_param().abi(), targetrectangle.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Save(&self, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(persistenceformat), ::std::mem::transmute(compressionmode), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), data.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn CreateStroke<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, packetdata: Param0, packetdescription: Param1) -> ::windows::runtime::Result<IInkStrokeDisp> {
        let mut result__: <IInkStrokeDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), packetdata.into_param().abi(), packetdescription.into_param().abi(), &mut result__).from_abi::<IInkStrokeDisp>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Com`*"]
    pub unsafe fn ClipboardCopyWithRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> ::windows::runtime::Result<super::super::System::Com::IDataObject> {
        let mut result__: <super::super::System::Com::IDataObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), rectangle.into_param().abi(), ::std::mem::transmute(clipboardformats), ::std::mem::transmute(clipboardmodes), &mut result__).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Com`*"]
    pub unsafe fn ClipboardCopy<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes) -> ::windows::runtime::Result<super::super::System::Com::IDataObject> {
        let mut result__: <super::super::System::Com::IDataObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), strokes.into_param().abi(), ::std::mem::transmute(clipboardformats), ::std::mem::transmute(clipboardmodes), &mut result__).from_abi::<super::super::System::Com::IDataObject>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Com`*"]
    pub unsafe fn CanPaste<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, dataobject: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), dataobject.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Com`*"]
    pub unsafe fn ClipboardPaste<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::System::Com::IDataObject>>(&self, x: i32, y: i32, dataobject: Param2) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), dataobject.into_param().abi(), &mut result__).from_abi::<IInkStrokes>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkDisp {
    type Vtable = IInkDisp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2637795232, 50402, 20429, [153, 115, 151, 92, 170, 244, 126, 166]);
}
impl ::std::convert::From<IInkDisp> for ::windows::runtime::IUnknown {
    fn from(value: IInkDisp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkDisp> for ::windows::runtime::IUnknown {
    fn from(value: &IInkDisp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkDisp> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkDisp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkDisp> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkDisp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDisp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, properties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dirty: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dirty: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppunkinkcustomstrokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stroke: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr, extractflags: InkExtractFlags, extractedink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, radius: f32, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selectionrectangle: ::windows::runtime::RawPtr, intersectpercent: f32, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, points: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, intersectpercent: f32, lassopoints: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, pointonstroke: *mut f32, distancefrompacket: *mut f32, stroke: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokeids: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcestrokes: ::windows::runtime::RawPtr, targetrectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, persistenceformat: InkPersistenceFormat, compressionmode: InkPersistenceCompressionMode, data: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetdata: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, packetdescription: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, stroke: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr, clipboardformats: InkClipboardFormats, clipboardmodes: InkClipboardModes, dataobject: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dataobject: ::windows::runtime::RawPtr, canpaste: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, dataobject: ::windows::runtime::RawPtr, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkDivider(pub ::windows::runtime::IUnknown);
impl IInkDivider {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Strokes(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Strokes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), strokes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RecognizerContext(&self) -> ::windows::runtime::Result<IInkRecognizerContext> {
        let mut result__: <IInkRecognizerContext as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognizerContext>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_RecognizerContext<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRecognizerContext>>(&self, recognizercontext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), recognizercontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn LineHeight(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetLineHeight(&self, lineheight: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(lineheight)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Divide(&self) -> ::windows::runtime::Result<IInkDivisionResult> {
        let mut result__: <IInkDivisionResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDivisionResult>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkDivider {
    type Vtable = IInkDivider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1574962181, 63908, 18001, [176, 197, 195, 23, 222, 253, 88, 185]);
}
impl ::std::convert::From<IInkDivider> for ::windows::runtime::IUnknown {
    fn from(value: IInkDivider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkDivider> for ::windows::runtime::IUnknown {
    fn from(value: &IInkDivider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkDivider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkDivider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkDivider> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkDivider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkDivider> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkDivider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkDivider {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkDivider {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDivider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognizercontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognizercontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lineheight: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lineheight: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkdivisionresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkDivisionResult(pub ::windows::runtime::IUnknown);
impl IInkDivisionResult {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Strokes(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ResultByType(&self, divisiontype: InkDivisionType) -> ::windows::runtime::Result<IInkDivisionUnits> {
        let mut result__: <IInkDivisionUnits as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(divisiontype), &mut result__).from_abi::<IInkDivisionUnits>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkDivisionResult {
    type Vtable = IInkDivisionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(767475879, 29895, 19256, [129, 235, 170, 142, 240, 194, 73, 0]);
}
impl ::std::convert::From<IInkDivisionResult> for ::windows::runtime::IUnknown {
    fn from(value: IInkDivisionResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkDivisionResult> for ::windows::runtime::IUnknown {
    fn from(value: &IInkDivisionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkDivisionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkDivisionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkDivisionResult> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkDivisionResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkDivisionResult> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkDivisionResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkDivisionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkDivisionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDivisionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, divisiontype: InkDivisionType, inkdivisionunits: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkDivisionUnit(pub ::windows::runtime::IUnknown);
impl IInkDivisionUnit {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Strokes(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DivisionType(&self) -> ::windows::runtime::Result<InkDivisionType> {
        let mut result__: <InkDivisionType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkDivisionType>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn RecognizedString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RotationTransform(&self) -> ::windows::runtime::Result<IInkTransform> {
        let mut result__: <IInkTransform as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkTransform>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkDivisionUnit {
    type Vtable = IInkDivisionUnit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2242831170, 18608, 16964, [157, 213, 30, 212, 53, 65, 15, 171]);
}
impl ::std::convert::From<IInkDivisionUnit> for ::windows::runtime::IUnknown {
    fn from(value: IInkDivisionUnit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkDivisionUnit> for ::windows::runtime::IUnknown {
    fn from(value: &IInkDivisionUnit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkDivisionUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkDivisionUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkDivisionUnit> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkDivisionUnit) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkDivisionUnit> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkDivisionUnit) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkDivisionUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkDivisionUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDivisionUnit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, divisiontype: *mut InkDivisionType) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recostring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rotationtransform: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkDivisionUnits(pub ::windows::runtime::IUnknown);
impl IInkDivisionUnits {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<IInkDivisionUnit> {
        let mut result__: <IInkDivisionUnit as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<IInkDivisionUnit>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkDivisionUnits {
    type Vtable = IInkDivisionUnits_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(464903618, 12748, 16693, [171, 130, 44, 102, 201, 240, 12, 65]);
}
impl ::std::convert::From<IInkDivisionUnits> for ::windows::runtime::IUnknown {
    fn from(value: IInkDivisionUnits) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkDivisionUnits> for ::windows::runtime::IUnknown {
    fn from(value: &IInkDivisionUnits) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkDivisionUnits {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkDivisionUnits {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkDivisionUnits> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkDivisionUnits) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkDivisionUnits> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkDivisionUnits) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkDivisionUnits {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkDivisionUnits {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDivisionUnits_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, inkdivisionunit: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkDrawingAttributes(pub ::windows::runtime::IUnknown);
impl IInkDrawingAttributes {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Color(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetColor(&self, newcolor: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(newcolor)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Width(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetWidth(&self, newwidth: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(newwidth)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Height(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetHeight(&self, newheight: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(newheight)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn FitToCurve(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetFitToCurve(&self, flag: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(flag)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn IgnorePressure(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetIgnorePressure(&self, flag: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(flag)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AntiAliased(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAntiAliased(&self, flag: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(flag)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Transparency(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetTransparency(&self, newtransparency: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(newtransparency)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RasterOperation(&self) -> ::windows::runtime::Result<InkRasterOperation> {
        let mut result__: <InkRasterOperation as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkRasterOperation>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetRasterOperation(&self, newrasteroperation: InkRasterOperation) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(newrasteroperation)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn PenTip(&self) -> ::windows::runtime::Result<InkPenTip> {
        let mut result__: <InkPenTip as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkPenTip>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetPenTip(&self, newpentip: InkPenTip) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(newpentip)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ExtendedProperties(&self) -> ::windows::runtime::Result<IInkExtendedProperties> {
        let mut result__: <IInkExtendedProperties as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkExtendedProperties>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IInkDrawingAttributes> {
        let mut result__: <IInkDrawingAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDrawingAttributes>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkDrawingAttributes {
    type Vtable = IInkDrawingAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3209796469, 2581, 17955, [173, 201, 192, 13, 67, 106, 128, 146]);
}
impl ::std::convert::From<IInkDrawingAttributes> for ::windows::runtime::IUnknown {
    fn from(value: IInkDrawingAttributes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkDrawingAttributes> for ::windows::runtime::IUnknown {
    fn from(value: &IInkDrawingAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkDrawingAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkDrawingAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkDrawingAttributes> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkDrawingAttributes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkDrawingAttributes> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkDrawingAttributes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkDrawingAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkDrawingAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentcolor: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newcolor: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentwidth: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newwidth: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentheight: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newheight: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flag: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flag: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flag: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flag: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flag: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, flag: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currenttransparency: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newtransparency: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentrasteroperation: *mut InkRasterOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newrasteroperation: InkRasterOperation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentpentip: *mut InkPenTip) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newpentip: InkPenTip) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, properties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, drawingattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkEdit(pub ::windows::runtime::IUnknown);
impl IInkEdit {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Status(&self) -> ::windows::runtime::Result<InkEditStatus> {
        let mut result__: <InkEditStatus as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkEditStatus>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn UseMouseForInput(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetUseMouseForInput(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn InkMode(&self) -> ::windows::runtime::Result<InkMode> {
        let mut result__: <InkMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetInkMode(&self, newval: InkMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn InkInsertMode(&self) -> ::windows::runtime::Result<InkInsertMode> {
        let mut result__: <InkInsertMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkInsertMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetInkInsertMode(&self, newval: InkInsertMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DrawingAttributes(&self) -> ::windows::runtime::Result<IInkDrawingAttributes> {
        let mut result__: <IInkDrawingAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_DrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RecognitionTimeout(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetRecognitionTimeout(&self, newval: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Recognizer(&self) -> ::windows::runtime::Result<IInkRecognizer> {
        let mut result__: <IInkRecognizer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognizer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Recognizer<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRecognizer>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Factoid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetFactoid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newval: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), newval.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelInks(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetSelInks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, selink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), selink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SelInksDisplayMode(&self) -> ::windows::runtime::Result<InkDisplayMode> {
        let mut result__: <InkDisplayMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkDisplayMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSelInksDisplayMode(&self, inkdisplaymode: InkDisplayMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(inkdisplaymode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Recognize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(gesture), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(gesture), ::std::mem::transmute(listen)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetBackColor(&self, clr: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(clr)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn BackColor(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Appearance(&self) -> ::windows::runtime::Result<AppearanceConstants> {
        let mut result__: <AppearanceConstants as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<AppearanceConstants>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAppearance(&self, pappearance: AppearanceConstants) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(pappearance)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn BorderStyle(&self) -> ::windows::runtime::Result<BorderStyleConstants> {
        let mut result__: <BorderStyleConstants as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<BorderStyleConstants>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetBorderStyle(&self, pborderstyle: BorderStyleConstants) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), ::std::mem::transmute(pborderstyle)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Hwnd(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn Font(&self) -> ::windows::runtime::Result<super::super::System::Ole::IFontDisp> {
        let mut result__: <super::super::System::Ole::IFontDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IFontDisp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn putref_Font<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IFontDisp>>(&self, ppfont: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ppfont.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Text(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrtext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), pbstrtext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn MouseIcon(&self) -> ::windows::runtime::Result<super::super::System::Ole::IPictureDisp> {
        let mut result__: <super::super::System::Ole::IPictureDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IPictureDisp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn SetMouseIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), mouseicon.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn putref_MouseIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), mouseicon.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MousePointer(&self) -> ::windows::runtime::Result<InkMousePointer> {
        let mut result__: <InkMousePointer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkMousePointer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), ::std::mem::transmute(mousepointer)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Locked(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetLocked(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEnabled(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MaxLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMaxLength(&self, lmaxlength: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self), ::std::mem::transmute(lmaxlength)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MultiLine(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMultiLine(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ScrollBars(&self) -> ::windows::runtime::Result<ScrollBarsConstants> {
        let mut result__: <ScrollBarsConstants as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ScrollBarsConstants>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetScrollBars(&self, newval: ScrollBarsConstants) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DisableNoScroll(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDisableNoScroll(&self, newval: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), ::std::mem::transmute(newval)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelAlignment(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetSelAlignment<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, pvarselalignment: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self), pvarselalignment.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelBold(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetSelBold<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, pvarselbold: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), pvarselbold.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelItalic(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).61)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetSelItalic<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, pvarselitalic: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).62)(::std::mem::transmute_copy(self), pvarselitalic.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelUnderline(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).63)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetSelUnderline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, pvarselunderline: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).64)(::std::mem::transmute_copy(self), pvarselunderline.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelColor(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).65)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetSelColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, pvarselcolor: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).66)(::std::mem::transmute_copy(self), pvarselcolor.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelFontName(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).67)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetSelFontName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, pvarselfontname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).68)(::std::mem::transmute_copy(self), pvarselfontname.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelFontSize(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).69)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetSelFontSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, pvarselfontsize: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).70)(::std::mem::transmute_copy(self), pvarselfontsize.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelCharOffset(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).71)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetSelCharOffset<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, pvarselcharoffset: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).72)(::std::mem::transmute_copy(self), pvarselcharoffset.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn TextRTF(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).73)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetTextRTF<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrtextrtf: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).74)(::std::mem::transmute_copy(self), pbstrtextrtf.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SelStart(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).75)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSelStart(&self, plselstart: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).76)(::std::mem::transmute_copy(self), ::std::mem::transmute(plselstart)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SelLength(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).77)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSelLength(&self, plsellength: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).78)(::std::mem::transmute_copy(self), ::std::mem::transmute(plsellength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SelText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).79)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetSelText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrseltext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).80)(::std::mem::transmute_copy(self), pbstrseltext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SelRTF(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).81)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetSelRTF<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, pbstrselrtf: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).82)(::std::mem::transmute_copy(self), pbstrselrtf.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).83)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkEdit {
    type Vtable = IInkEdit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4061297177, 64507, 19181, [132, 100, 63, 54, 215, 140, 254, 251]);
}
impl ::std::convert::From<IInkEdit> for ::windows::runtime::IUnknown {
    fn from(value: IInkEdit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkEdit> for ::windows::runtime::IUnknown {
    fn from(value: &IInkEdit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkEdit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkEdit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkEdit> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkEdit) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkEdit> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkEdit) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkEdit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkEdit {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkEdit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstatus: *mut InkEditStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut InkMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: InkMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut InkInsertMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: InkInsertMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pselink: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selink: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinkdisplaymode: *mut InkDisplayMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkdisplaymode: InkDisplayMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gesture: InkApplicationGesture, plisten: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gesture: InkApplicationGesture, listen: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, clr: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pclr: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pappearance: *mut AppearanceConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pappearance: AppearanceConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pborderstyle: *mut BorderStyleConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pborderstyle: BorderStyleConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pohhwnd: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfont: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mousepointer: *mut InkMousePointer) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mousepointer: InkMousePointer) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmaxlength: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lmaxlength: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut ScrollBarsConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: ScrollBarsConstants) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pval: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newval: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselalignment: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselalignment: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselbold: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselbold: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselitalic: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselitalic: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselunderline: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselunderline: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselcolor: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselcolor: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselfontname: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselfontname: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselfontsize: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselfontsize: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselcharoffset: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvarselcharoffset: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtextrtf: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrtextrtf: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plselstart: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plselstart: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsellength: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsellength: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrseltext: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrseltext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrselrtf: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrselrtf: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkExtendedProperties(pub ::windows::runtime::IUnknown);
impl IInkExtendedProperties {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Item<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, identifier: Param0) -> ::windows::runtime::Result<IInkExtendedProperty> {
        let mut result__: <IInkExtendedProperty as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), identifier.into_param().abi(), &mut result__).from_abi::<IInkExtendedProperty>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, guid: Param0, data: Param1) -> ::windows::runtime::Result<IInkExtendedProperty> {
        let mut result__: <IInkExtendedProperty as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), guid.into_param().abi(), data.into_param().abi(), &mut result__).from_abi::<IInkExtendedProperty>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, identifier: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), identifier.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn DoesPropertyExist<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, guid: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), guid.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkExtendedProperties {
    type Vtable = IInkExtendedProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2314381502, 38313, 17712, [139, 143, 136, 233, 113, 227, 226, 95]);
}
impl ::std::convert::From<IInkExtendedProperties> for ::windows::runtime::IUnknown {
    fn from(value: IInkExtendedProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkExtendedProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IInkExtendedProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkExtendedProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkExtendedProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkExtendedProperties> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkExtendedProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkExtendedProperties> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkExtendedProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkExtendedProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkExtendedProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkExtendedProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, item: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, data: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, inkextendedproperty: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, identifier: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, doespropertyexist: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkExtendedProperty(pub ::windows::runtime::IUnknown);
impl IInkExtendedProperty {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Guid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Data(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, data: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), data.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkExtendedProperty {
    type Vtable = IInkExtendedProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3678966281, 47043, 16669, [144, 246, 21, 72, 207, 255, 39, 30]);
}
impl ::std::convert::From<IInkExtendedProperty> for ::windows::runtime::IUnknown {
    fn from(value: IInkExtendedProperty) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkExtendedProperty> for ::windows::runtime::IUnknown {
    fn from(value: &IInkExtendedProperty) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkExtendedProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkExtendedProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkExtendedProperty> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkExtendedProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkExtendedProperty> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkExtendedProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkExtendedProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkExtendedProperty {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkExtendedProperty_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, guid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkGesture(pub ::windows::runtime::IUnknown);
impl IInkGesture {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Confidence(&self) -> ::windows::runtime::Result<InkRecognitionConfidence> {
        let mut result__: <InkRecognitionConfidence as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkRecognitionConfidence>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<InkApplicationGesture> {
        let mut result__: <InkApplicationGesture as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkApplicationGesture>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetHotPoint(&self, x: *mut i32, y: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkGesture {
    type Vtable = IInkGesture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1004276375, 1253, 20006, [184, 19, 24, 240, 82, 212, 29, 239]);
}
impl ::std::convert::From<IInkGesture> for ::windows::runtime::IUnknown {
    fn from(value: IInkGesture) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkGesture> for ::windows::runtime::IUnknown {
    fn from(value: &IInkGesture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkGesture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkGesture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkGesture> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkGesture) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkGesture> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkGesture) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkGesture {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkGesture {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkGesture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, confidence: *mut InkRecognitionConfidence) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut InkApplicationGesture) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: *mut i32, y: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkLineInfo(pub ::windows::runtime::IUnknown);
impl IInkLineInfo {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetFormat(&self, pim: *const INKMETRIC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pim)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetFormat(&self, pim: *const INKMETRIC) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pim)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetInkExtent(&self, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pim), ::std::mem::transmute(pnwidth)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn GetCandidate<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ncandidatenum: u32, pwcrecogword: Param1, pcwcrecogword: *const u32, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncandidatenum), pwcrecogword.into_param().abi(), ::std::mem::transmute(pcwcrecogword), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetCandidate<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ncandidatenum: u32, strrecogword: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ncandidatenum), strrecogword.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Recognize(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkLineInfo {
    type Vtable = IInkLineInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2619103958, 61999, 19940, [180, 83, 162, 204, 72, 46, 124, 51]);
}
impl ::std::convert::From<IInkLineInfo> for ::windows::runtime::IUnknown {
    fn from(value: IInkLineInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkLineInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IInkLineInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkLineInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkLineInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkLineInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pim: *const INKMETRIC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pim: *const INKMETRIC) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pim: *const INKMETRIC, pnwidth: *const u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncandidatenum: u32, pwcrecogword: super::super::Foundation::PWSTR, pcwcrecogword: *const u32, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ncandidatenum: u32, strrecogword: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkOverlay(pub ::windows::runtime::IUnknown);
impl IInkOverlay {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn hWnd(&self) -> ::windows::runtime::Result<isize> {
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SethWnd(&self, newwindow: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(newwindow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEnabled(&self, collecting: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(collecting)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DefaultDrawingAttributes(&self) -> ::windows::runtime::Result<IInkDrawingAttributes> {
        let mut result__: <IInkDrawingAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_DefaultDrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, newattributes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), newattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Renderer(&self) -> ::windows::runtime::Result<IInkRenderer> {
        let mut result__: <IInkRenderer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRenderer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Renderer<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRenderer>>(&self, newinkrenderer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), newinkrenderer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Ink(&self) -> ::windows::runtime::Result<IInkDisp> {
        let mut result__: <IInkDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Ink<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDisp>>(&self, newink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), newink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AutoRedraw(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAutoRedraw(&self, autoredraw: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(autoredraw)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CollectingInk(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CollectionMode(&self) -> ::windows::runtime::Result<InkCollectionMode> {
        let mut result__: <InkCollectionMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkCollectionMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetCollectionMode(&self, mode: InkCollectionMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DynamicRendering(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDynamicRendering(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn DesiredPacketDescription(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetDesiredPacketDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, packetguids: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), packetguids.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn MouseIcon(&self) -> ::windows::runtime::Result<super::super::System::Ole::IPictureDisp> {
        let mut result__: <super::super::System::Ole::IPictureDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IPictureDisp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn SetMouseIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), mouseicon.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn putref_MouseIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), mouseicon.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MousePointer(&self) -> ::windows::runtime::Result<InkMousePointer> {
        let mut result__: <InkMousePointer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkMousePointer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(mousepointer)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EditingMode(&self) -> ::windows::runtime::Result<InkOverlayEditingMode> {
        let mut result__: <InkOverlayEditingMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkOverlayEditingMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEditingMode(&self, editingmode: InkOverlayEditingMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(editingmode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Selection(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSelection<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, selection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), selection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EraserMode(&self) -> ::windows::runtime::Result<InkOverlayEraserMode> {
        let mut result__: <InkOverlayEraserMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkOverlayEraserMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEraserMode(&self, erasermode: InkOverlayEraserMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(erasermode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EraserWidth(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEraserWidth(&self, neweraserwidth: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(neweraserwidth)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AttachMode(&self) -> ::windows::runtime::Result<InkOverlayAttachMode> {
        let mut result__: <InkOverlayAttachMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkOverlayAttachMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAttachMode(&self, attachmode: InkOverlayAttachMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), ::std::mem::transmute(attachmode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Cursors(&self) -> ::windows::runtime::Result<IInkCursors> {
        let mut result__: <IInkCursors as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkCursors>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MarginX(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMarginX(&self, marginx: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), ::std::mem::transmute(marginx)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MarginY(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMarginY(&self, marginy: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(marginy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Tablet(&self) -> ::windows::runtime::Result<IInkTablet> {
        let mut result__: <IInkTablet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkTablet>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SupportHighContrastInk(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSupportHighContrastInk(&self, support: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), ::std::mem::transmute(support)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SupportHighContrastSelectionUI(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSupportHighContrastSelectionUI(&self, support: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self), ::std::mem::transmute(support)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn HitTestSelection(&self, x: i32, y: i32) -> ::windows::runtime::Result<SelectionHitResult> {
        let mut result__: <SelectionHitResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), &mut result__).from_abi::<SelectionHitResult>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Draw<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rect: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), rect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), ::std::mem::transmute(gesture), ::std::mem::transmute(listen)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self), ::std::mem::transmute(gesture), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetWindowInputRectangle(&self, windowinputrectangle: *mut ::std::option::Option<IInkRectangle>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self), ::std::mem::transmute(windowinputrectangle)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetWindowInputRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, windowinputrectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), windowinputrectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAllTabletsMode(&self, usemouseforinput: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), ::std::mem::transmute(usemouseforinput)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSingleTabletIntegratedMode<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTablet>>(&self, tablet: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self), tablet.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventid), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventid), ::std::mem::transmute(listen)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkOverlay {
    type Vtable = IInkOverlay_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3089778235, 49605, 17827, [153, 124, 222, 171, 86, 81, 182, 122]);
}
impl ::std::convert::From<IInkOverlay> for ::windows::runtime::IUnknown {
    fn from(value: IInkOverlay) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkOverlay> for ::windows::runtime::IUnknown {
    fn from(value: &IInkOverlay) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkOverlay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkOverlay {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkOverlay> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkOverlay) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkOverlay> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkOverlay) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkOverlay {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkOverlay {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkOverlay_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentwindow: *mut isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newwindow: isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collecting: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collecting: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newattributes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentinkrenderer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newinkrenderer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, autoredraw: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, autoredraw: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collecting: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: *mut InkCollectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: InkCollectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetguids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetguids: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mousepointer: *mut InkMousePointer) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mousepointer: InkMousePointer) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, editingmode: *mut InkOverlayEditingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, editingmode: InkOverlayEditingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, erasermode: *mut InkOverlayEraserMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, erasermode: InkOverlayEraserMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eraserwidth: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, neweraserwidth: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachmode: *mut InkOverlayAttachMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachmode: InkOverlayAttachMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cursors: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginx: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginx: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginy: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginy: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, singletablet: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rect: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gesture: InkApplicationGesture, listen: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windowinputrectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windowinputrectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usemouseforinput: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tablet: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkPicture(pub ::windows::runtime::IUnknown);
impl IInkPicture {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn hWnd(&self) -> ::windows::runtime::Result<isize> {
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<isize>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DefaultDrawingAttributes(&self) -> ::windows::runtime::Result<IInkDrawingAttributes> {
        let mut result__: <IInkDrawingAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_DefaultDrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, newattributes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), newattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Renderer(&self) -> ::windows::runtime::Result<IInkRenderer> {
        let mut result__: <IInkRenderer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRenderer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Renderer<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRenderer>>(&self, newinkrenderer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), newinkrenderer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Ink(&self) -> ::windows::runtime::Result<IInkDisp> {
        let mut result__: <IInkDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Ink<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDisp>>(&self, newink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), newink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AutoRedraw(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAutoRedraw(&self, autoredraw: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(autoredraw)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CollectingInk(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CollectionMode(&self) -> ::windows::runtime::Result<InkCollectionMode> {
        let mut result__: <InkCollectionMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkCollectionMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetCollectionMode(&self, mode: InkCollectionMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DynamicRendering(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDynamicRendering(&self, enabled: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(enabled)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn DesiredPacketDescription(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetDesiredPacketDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, packetguids: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), packetguids.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn MouseIcon(&self) -> ::windows::runtime::Result<super::super::System::Ole::IPictureDisp> {
        let mut result__: <super::super::System::Ole::IPictureDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IPictureDisp>(result__)
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn SetMouseIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), mouseicon.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn putref_MouseIcon<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, mouseicon: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), mouseicon.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MousePointer(&self) -> ::windows::runtime::Result<InkMousePointer> {
        let mut result__: <InkMousePointer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkMousePointer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMousePointer(&self, mousepointer: InkMousePointer) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(mousepointer)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EditingMode(&self) -> ::windows::runtime::Result<InkOverlayEditingMode> {
        let mut result__: <InkOverlayEditingMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkOverlayEditingMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEditingMode(&self, editingmode: InkOverlayEditingMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(editingmode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Selection(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSelection<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, selection: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), selection.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EraserMode(&self) -> ::windows::runtime::Result<InkOverlayEraserMode> {
        let mut result__: <InkOverlayEraserMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkOverlayEraserMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEraserMode(&self, erasermode: InkOverlayEraserMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(erasermode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EraserWidth(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEraserWidth(&self, neweraserwidth: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), ::std::mem::transmute(neweraserwidth)).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn putref_Picture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, ppicture: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ppicture.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn SetPicture<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Ole::IPictureDisp>>(&self, ppicture: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ppicture.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn Picture(&self) -> ::windows::runtime::Result<super::super::System::Ole::IPictureDisp> {
        let mut result__: <super::super::System::Ole::IPictureDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IPictureDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSizeMode(&self, smnewsizemode: InkPictureSizeMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(smnewsizemode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SizeMode(&self) -> ::windows::runtime::Result<InkPictureSizeMode> {
        let mut result__: <InkPictureSizeMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkPictureSizeMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetBackColor(&self, newcolor: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).41)(::std::mem::transmute_copy(self), ::std::mem::transmute(newcolor)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn BackColor(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).42)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Cursors(&self) -> ::windows::runtime::Result<IInkCursors> {
        let mut result__: <IInkCursors as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).43)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkCursors>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MarginX(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).44)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMarginX(&self, marginx: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).45)(::std::mem::transmute_copy(self), ::std::mem::transmute(marginx)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MarginY(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).46)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMarginY(&self, marginy: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).47)(::std::mem::transmute_copy(self), ::std::mem::transmute(marginy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Tablet(&self) -> ::windows::runtime::Result<IInkTablet> {
        let mut result__: <IInkTablet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).48)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkTablet>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SupportHighContrastInk(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).49)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSupportHighContrastInk(&self, support: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).50)(::std::mem::transmute_copy(self), ::std::mem::transmute(support)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SupportHighContrastSelectionUI(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).51)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSupportHighContrastSelectionUI(&self, support: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).52)(::std::mem::transmute_copy(self), ::std::mem::transmute(support)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn HitTestSelection(&self, x: i32, y: i32) -> ::windows::runtime::Result<SelectionHitResult> {
        let mut result__: <SelectionHitResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).53)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), &mut result__).from_abi::<SelectionHitResult>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetGestureStatus(&self, gesture: InkApplicationGesture, listen: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).54)(::std::mem::transmute_copy(self), ::std::mem::transmute(gesture), ::std::mem::transmute(listen)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetGestureStatus(&self, gesture: InkApplicationGesture) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).55)(::std::mem::transmute_copy(self), ::std::mem::transmute(gesture), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetWindowInputRectangle(&self, windowinputrectangle: *mut ::std::option::Option<IInkRectangle>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).56)(::std::mem::transmute_copy(self), ::std::mem::transmute(windowinputrectangle)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetWindowInputRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, windowinputrectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).57)(::std::mem::transmute_copy(self), windowinputrectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAllTabletsMode(&self, usemouseforinput: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).58)(::std::mem::transmute_copy(self), ::std::mem::transmute(usemouseforinput)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSingleTabletIntegratedMode<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTablet>>(&self, tablet: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).59)(::std::mem::transmute_copy(self), tablet.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetEventInterest(&self, eventid: InkCollectorEventInterest) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).60)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventid), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEventInterest(&self, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).61)(::std::mem::transmute_copy(self), ::std::mem::transmute(eventid), ::std::mem::transmute(listen)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn InkEnabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).62)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetInkEnabled(&self, collecting: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).63)(::std::mem::transmute_copy(self), ::std::mem::transmute(collecting)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).64)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetEnabled(&self, vbool: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).65)(::std::mem::transmute_copy(self), ::std::mem::transmute(vbool)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkPicture {
    type Vtable = IInkPicture_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3897975520, 14234, 16599, [155, 92, 117, 125, 35, 63, 153, 35]);
}
impl ::std::convert::From<IInkPicture> for ::windows::runtime::IUnknown {
    fn from(value: IInkPicture) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkPicture> for ::windows::runtime::IUnknown {
    fn from(value: &IInkPicture) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkPicture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkPicture {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkPicture> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkPicture) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkPicture> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkPicture) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkPicture {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkPicture {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPicture_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentwindow: *mut isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentattributes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newattributes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentinkrenderer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newinkrenderer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, autoredraw: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, autoredraw: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collecting: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: *mut InkCollectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: InkCollectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enabled: i16) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetguids: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetguids: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mouseicon: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mousepointer: *mut InkMousePointer) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mousepointer: InkMousePointer) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, editingmode: *mut InkOverlayEditingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, editingmode: InkOverlayEditingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, erasermode: *mut InkOverlayEraserMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, erasermode: InkOverlayEraserMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eraserwidth: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, neweraserwidth: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppicture: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppicture: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppicture: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, smnewsizemode: InkPictureSizeMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, smsizemode: *mut InkPictureSizeMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newcolor: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcolor: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cursors: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginx: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginx: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginy: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, marginy: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, singletablet: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, support: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, selarea: *mut SelectionHitResult) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gesture: InkApplicationGesture, listen: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, gesture: InkApplicationGesture, listening: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windowinputrectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, windowinputrectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, usemouseforinput: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tablet: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventid: InkCollectorEventInterest, listen: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventid: InkCollectorEventInterest, listen: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collecting: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, collecting: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbool: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vbool: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRecognitionAlternate(pub ::windows::runtime::IUnknown);
impl IInkRecognitionAlternate {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn String(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Confidence(&self) -> ::windows::runtime::Result<InkRecognitionConfidence> {
        let mut result__: <InkRecognitionConfidence as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkRecognitionConfidence>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Baseline(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Midline(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Ascender(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Descender(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn LineNumber(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Strokes(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn LineAlternates(&self) -> ::windows::runtime::Result<IInkRecognitionAlternates> {
        let mut result__: <IInkRecognitionAlternates as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognitionAlternates>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ConfidenceAlternates(&self) -> ::windows::runtime::Result<IInkRecognitionAlternates> {
        let mut result__: <IInkRecognitionAlternates as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognitionAlternates>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetStrokesFromStrokeRanges<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strokes.into_param().abi(), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetStrokesFromTextRange(&self, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut ::std::option::Option<IInkStrokes>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(selectionstart), ::std::mem::transmute(selectionlength), ::std::mem::transmute(getstrokesfromtextrange)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetTextRangeFromStrokes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), strokes.into_param().abi(), ::std::mem::transmute(selectionstart), ::std::mem::transmute(selectionlength)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn AlternatesWithConstantPropertyValues<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertytype: Param0) -> ::windows::runtime::Result<IInkRecognitionAlternates> {
        let mut result__: <IInkRecognitionAlternates as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), propertytype.into_param().abi(), &mut result__).from_abi::<IInkRecognitionAlternates>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetPropertyValue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertytype: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), propertytype.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkRecognitionAlternate {
    type Vtable = IInkRecognitionAlternate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3085328557, 30692, 17051, [173, 218, 135, 55, 128, 209, 252, 74]);
}
impl ::std::convert::From<IInkRecognitionAlternate> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognitionAlternate) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognitionAlternate> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognitionAlternate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognitionAlternate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRecognitionAlternate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRecognitionAlternate> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRecognitionAlternate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRecognitionAlternate> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRecognitionAlternate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRecognitionAlternate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRecognitionAlternate {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionAlternate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recostring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, confidence: *mut InkRecognitionConfidence) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, baseline: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, midline: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ascender: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, descender: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linenumber: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, linealternates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, confidencealternates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr, getstrokesfromstrokeranges: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selectionstart: *mut i32, selectionlength: *mut i32, getstrokesfromtextrange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr, selectionstart: *mut i32, selectionlength: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertytype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, alternateswithconstantpropertyvalues: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertytype: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, propertyvalue: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRecognitionAlternates(pub ::windows::runtime::IUnknown);
impl IInkRecognitionAlternates {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Strokes(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<IInkRecognitionAlternate> {
        let mut result__: <IInkRecognitionAlternate as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<IInkRecognitionAlternate>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkRecognitionAlternates {
    type Vtable = IInkRecognitionAlternates_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(678041215, 40729, 19553, [157, 83, 79, 7, 190, 98, 43, 132]);
}
impl ::std::convert::From<IInkRecognitionAlternates> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognitionAlternates) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognitionAlternates> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognitionAlternates) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognitionAlternates {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRecognitionAlternates {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRecognitionAlternates> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRecognitionAlternates) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRecognitionAlternates> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRecognitionAlternates) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRecognitionAlternates {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRecognitionAlternates {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionAlternates_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, inkrecoalternate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRecognitionResult(pub ::windows::runtime::IUnknown);
impl IInkRecognitionResult {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn TopString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn TopAlternate(&self) -> ::windows::runtime::Result<IInkRecognitionAlternate> {
        let mut result__: <IInkRecognitionAlternate as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognitionAlternate>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn TopConfidence(&self) -> ::windows::runtime::Result<InkRecognitionConfidence> {
        let mut result__: <InkRecognitionConfidence as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkRecognitionConfidence>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Strokes(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AlternatesFromSelection(&self, selectionstart: i32, selectionlength: i32, maximumalternates: i32) -> ::windows::runtime::Result<IInkRecognitionAlternates> {
        let mut result__: <IInkRecognitionAlternates as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(selectionstart), ::std::mem::transmute(selectionlength), ::std::mem::transmute(maximumalternates), &mut result__).from_abi::<IInkRecognitionAlternates>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ModifyTopAlternate<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRecognitionAlternate>>(&self, alternate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), alternate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetResultOnStrokes(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkRecognitionResult {
    type Vtable = IInkRecognitionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1002514856, 34509, 17837, [189, 232, 224, 211, 45, 97, 193, 109]);
}
impl ::std::convert::From<IInkRecognitionResult> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognitionResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognitionResult> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognitionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRecognitionResult> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRecognitionResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRecognitionResult> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRecognitionResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, topstring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, topalternate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, topconfidence: *mut InkRecognitionConfidence) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selectionstart: i32, selectionlength: i32, maximumalternates: i32, alternatesfromselection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alternate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRecognizer(pub ::windows::runtime::IUnknown);
impl IInkRecognizer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Vendor(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Capabilities(&self) -> ::windows::runtime::Result<InkRecognizerCapabilities> {
        let mut result__: <InkRecognizerCapabilities as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkRecognizerCapabilities>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn Languages(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SupportedProperties(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn PreferredPacketDescription(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CreateRecognizerContext(&self) -> ::windows::runtime::Result<IInkRecognizerContext> {
        let mut result__: <IInkRecognizerContext as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognizerContext>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkRecognizer {
    type Vtable = IInkRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2016147407, 843, 17302, [138, 50, 58, 24, 51, 207, 107, 86]);
}
impl ::std::convert::From<IInkRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognizer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRecognizer> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRecognizer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRecognizer> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRecognizer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, vendor: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilitiesflags: *mut InkRecognizerCapabilities) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languages: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, supportedproperties: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preferredpacketdescription: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRecognizer2(pub ::windows::runtime::IUnknown);
impl IInkRecognizer2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn UnicodeRanges(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkRecognizer2 {
    type Vtable = IInkRecognizer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1628443018, 14965, 19158, [178, 170, 4, 178, 183, 43, 190, 101]);
}
impl ::std::convert::From<IInkRecognizer2> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognizer2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognizer2> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognizer2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognizer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRecognizer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRecognizer2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRecognizer2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRecognizer2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRecognizer2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRecognizer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRecognizer2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbstrid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unicoderanges: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRecognizerContext(pub ::windows::runtime::IUnknown);
impl IInkRecognizerContext {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Strokes(&self) -> ::windows::runtime::Result<IInkStrokes> {
        let mut result__: <IInkStrokes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkStrokes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Strokes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), strokes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CharacterAutoCompletionMode(&self) -> ::windows::runtime::Result<InkRecognizerCharacterAutoCompletionMode> {
        let mut result__: <InkRecognizerCharacterAutoCompletionMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkRecognizerCharacterAutoCompletionMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetCharacterAutoCompletionMode(&self, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(mode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Factoid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetFactoid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, factoid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), factoid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Guide(&self) -> ::windows::runtime::Result<IInkRecognizerGuide> {
        let mut result__: <IInkRecognizerGuide as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognizerGuide>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Guide<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRecognizerGuide>>(&self, recognizerguide: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), recognizerguide.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn PrefixText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetPrefixText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, prefix: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), prefix.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SuffixText(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetSuffixText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, suffix: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), suffix.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RecognitionFlags(&self) -> ::windows::runtime::Result<InkRecognitionModes> {
        let mut result__: <InkRecognitionModes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkRecognitionModes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetRecognitionFlags(&self, modes: InkRecognitionModes) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(modes)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn WordList(&self) -> ::windows::runtime::Result<IInkWordList> {
        let mut result__: <IInkWordList as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkWordList>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_WordList<'a, Param0: ::windows::runtime::IntoParam<'a, IInkWordList>>(&self, wordlist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), wordlist.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Recognizer(&self) -> ::windows::runtime::Result<IInkRecognizer> {
        let mut result__: <IInkRecognizer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognizer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Recognize(&self, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut ::std::option::Option<IInkRecognitionResult>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(recognitionstatus), ::std::mem::transmute(recognitionresult)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn StopBackgroundRecognition(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EndInkInput(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn BackgroundRecognize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, customdata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), customdata.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn BackgroundRecognizeWithAlternates<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, customdata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), customdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IInkRecognizerContext> {
        let mut result__: <IInkRecognizerContext as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognizerContext>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn IsStringSupported<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, string: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), string.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkRecognizerContext {
    type Vtable = IInkRecognizerContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3331281657, 12963, 17957, [144, 108, 68, 252, 35, 180, 9, 88]);
}
impl ::std::convert::From<IInkRecognizerContext> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognizerContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognizerContext> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognizerContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognizerContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRecognizerContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRecognizerContext> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRecognizerContext) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRecognizerContext> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRecognizerContext) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRecognizerContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRecognizerContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: *mut InkRecognizerCharacterAutoCompletionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: InkRecognizerCharacterAutoCompletionMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, factoid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, factoid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognizerguide: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognizerguide: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prefix: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prefix: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, suffix: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, suffix: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modes: *mut InkRecognitionModes) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, modes: InkRecognitionModes) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wordlist: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, wordlist: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognizer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognitionstatus: *mut InkRecognitionStatus, recognitionresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customdata: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, customdata: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recocontext: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, string: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRecognizerContext2(pub ::windows::runtime::IUnknown);
impl IInkRecognizerContext2 {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn EnabledUnicodeRanges(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetEnabledUnicodeRanges<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, unicoderanges: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), unicoderanges.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkRecognizerContext2 {
    type Vtable = IInkRecognizerContext2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3606111023, 29656, 16526, [142, 159, 95, 234, 89, 44, 54, 63]);
}
impl ::std::convert::From<IInkRecognizerContext2> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognizerContext2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognizerContext2> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognizerContext2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognizerContext2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRecognizerContext2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRecognizerContext2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRecognizerContext2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRecognizerContext2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRecognizerContext2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRecognizerContext2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRecognizerContext2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerContext2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unicoderanges: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unicoderanges: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRecognizerGuide(pub ::windows::runtime::IUnknown);
impl IInkRecognizerGuide {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn WritingBox(&self) -> ::windows::runtime::Result<IInkRectangle> {
        let mut result__: <IInkRectangle as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRectangle>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetWritingBox<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), rectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DrawnBox(&self) -> ::windows::runtime::Result<IInkRectangle> {
        let mut result__: <IInkRectangle as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRectangle>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDrawnBox<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), rectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Rows(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetRows(&self, units: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(units)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Columns(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetColumns(&self, units: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(units)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Midline(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetMidline(&self, units: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(units)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn GuideData(&self) -> ::windows::runtime::Result<InkRecoGuide> {
        let mut result__: <InkRecoGuide as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InkRecoGuide>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetGuideData<'a, Param0: ::windows::runtime::IntoParam<'a, InkRecoGuide>>(&self, recoguide: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), recoguide.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkRecognizerGuide {
    type Vtable = IInkRecognizerGuide_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3644112391, 31620, 16904, [145, 54, 131, 194, 9, 148, 233, 5]);
}
impl ::std::convert::From<IInkRecognizerGuide> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognizerGuide) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognizerGuide> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognizerGuide) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognizerGuide {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRecognizerGuide {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRecognizerGuide> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRecognizerGuide) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRecognizerGuide> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRecognizerGuide) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRecognizerGuide {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRecognizerGuide {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerGuide_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, precoguide: *mut InkRecoGuide) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recoguide: InkRecoGuide) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRecognizers(pub ::windows::runtime::IUnknown);
impl IInkRecognizers {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetDefaultRecognizer(&self, lcid: i32) -> ::windows::runtime::Result<IInkRecognizer> {
        let mut result__: <IInkRecognizer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(lcid), &mut result__).from_abi::<IInkRecognizer>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<IInkRecognizer> {
        let mut result__: <IInkRecognizer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<IInkRecognizer>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkRecognizers {
    type Vtable = IInkRecognizers_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2630635282, 45239, 19083, [191, 88, 74, 236, 164, 232, 206, 253]);
}
impl ::std::convert::From<IInkRecognizers> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognizers) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognizers> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognizers) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognizers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRecognizers {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRecognizers> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRecognizers) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRecognizers> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRecognizers) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRecognizers {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRecognizers {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizers_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lcid: i32, defaultrecognizer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, inkrecognizer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRectangle(pub ::windows::runtime::IUnknown);
impl IInkRectangle {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Top(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetTop(&self, units: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(units)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Left(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetLeft(&self, units: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(units)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Bottom(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetBottom(&self, units: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(units)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Right(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetRight(&self, units: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(units)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Data(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, rect: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), rect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetRectangle(&self, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(top), ::std::mem::transmute(left), ::std::mem::transmute(bottom), ::std::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetRectangle(&self, top: i32, left: i32, bottom: i32, right: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(top), ::std::mem::transmute(left), ::std::mem::transmute(bottom), ::std::mem::transmute(right)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkRectangle {
    type Vtable = IInkRectangle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2543124354, 24689, 18199, [138, 139, 106, 199, 198, 74, 104, 110]);
}
impl ::std::convert::From<IInkRectangle> for ::windows::runtime::IUnknown {
    fn from(value: IInkRectangle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRectangle> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRectangle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRectangle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRectangle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRectangle> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRectangle) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRectangle> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRectangle) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRectangle {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRectangle {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRectangle_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, units: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rect: super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: *mut i32, left: *mut i32, bottom: *mut i32, right: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: i32, left: i32, bottom: i32, right: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkRenderer(pub ::windows::runtime::IUnknown);
impl IInkRenderer {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetViewTransform<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTransform>>(&self, viewtransform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), viewtransform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetViewTransform<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTransform>>(&self, viewtransform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), viewtransform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetObjectTransform<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTransform>>(&self, objecttransform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), objecttransform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetObjectTransform<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTransform>>(&self, objecttransform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), objecttransform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Draw<'a, Param1: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, hdc: isize, strokes: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(hdc), strokes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DrawStroke<'a, Param1: ::windows::runtime::IntoParam<'a, IInkStrokeDisp>, Param2: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, hdc: isize, stroke: Param1, drawingattributes: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(hdc), stroke.into_param().abi(), drawingattributes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn PixelToInkSpace(&self, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(hdc), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn InkSpaceToPixel(&self, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(hdcdisplay), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn PixelToInkSpaceFromPoints(&self, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(hdc), ::std::mem::transmute(points)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn InkSpaceToPixelFromPoints(&self, hdc: isize, points: *mut super::super::System::Com::VARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(hdc), ::std::mem::transmute(points)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Measure<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows::runtime::Result<IInkRectangle> {
        let mut result__: <IInkRectangle as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), strokes.into_param().abi(), &mut result__).from_abi::<IInkRectangle>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MeasureStroke<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokeDisp>, Param1: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, stroke: Param0, drawingattributes: Param1) -> ::windows::runtime::Result<IInkRectangle> {
        let mut result__: <IInkRectangle as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), stroke.into_param().abi(), drawingattributes.into_param().abi(), &mut result__).from_abi::<IInkRectangle>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalcomponent), ::std::mem::transmute(verticalcomponent)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(degrees), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalmultiplier), ::std::mem::transmute(verticalmultiplier), ::std::mem::transmute(applyonpenwidth)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkRenderer {
    type Vtable = IInkRenderer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3861215900, 46353, 20300, [168, 176, 167, 219, 201, 80, 107, 131]);
}
impl ::std::convert::From<IInkRenderer> for ::windows::runtime::IUnknown {
    fn from(value: IInkRenderer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRenderer> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRenderer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRenderer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkRenderer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkRenderer> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkRenderer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkRenderer> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkRenderer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkRenderer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkRenderer {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRenderer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewtransform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, viewtransform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, objecttransform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, objecttransform: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: isize, strokes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: isize, stroke: ::windows::runtime::RawPtr, drawingattributes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: isize, x: *mut i32, y: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdcdisplay: isize, x: *mut i32, y: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: isize, points: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hdc: isize, points: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr, rectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stroke: ::windows::runtime::RawPtr, drawingattributes: ::windows::runtime::RawPtr, rectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, degrees: f32, x: f32, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalmultiplier: f32, verticalmultiplier: f32, applyonpenwidth: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkStrokeDisp(pub ::windows::runtime::IUnknown);
impl IInkStrokeDisp {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ID(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn BezierPoints(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DrawingAttributes(&self) -> ::windows::runtime::Result<IInkDrawingAttributes> {
        let mut result__: <IInkDrawingAttributes as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDrawingAttributes>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_DrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, drawattrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), drawattrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Ink(&self) -> ::windows::runtime::Result<IInkDisp> {
        let mut result__: <IInkDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ExtendedProperties(&self) -> ::windows::runtime::Result<IInkExtendedProperties> {
        let mut result__: <IInkExtendedProperties as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkExtendedProperties>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn PolylineCusps(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn BezierCusps(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SelfIntersections(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn PacketCount(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn PacketSize(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn PacketDescription(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Deleted(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> ::windows::runtime::Result<IInkRectangle> {
        let mut result__: <IInkRectangle as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(boundingboxmode), &mut result__).from_abi::<IInkRectangle>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn FindIntersections<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, strokes: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), strokes.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetRectangleIntersections<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), rectangle.into_param().abi(), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Clip<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), rectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn HitTestCircle(&self, x: i32, y: i32, radius: f32) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(radius), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn NearestPoint(&self, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(x), ::std::mem::transmute(y), ::std::mem::transmute(distance), ::std::mem::transmute(point)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Split(&self, splitat: f32) -> ::windows::runtime::Result<IInkStrokeDisp> {
        let mut result__: <IInkStrokeDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(splitat), &mut result__).from_abi::<IInkStrokeDisp>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn GetPacketDescriptionPropertyMetrics<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), propertyname.into_param().abi(), ::std::mem::transmute(minimum), ::std::mem::transmute(maximum), ::std::mem::transmute(units), ::std::mem::transmute(resolution)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetPoints(&self, index: i32, count: i32) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), ::std::mem::transmute(count), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetPoints<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, points: Param0, index: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), points.into_param().abi(), ::std::mem::transmute(index), ::std::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetPacketData(&self, index: i32, count: i32) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), ::std::mem::transmute(count), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetPacketValuesByProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0, index: i32, count: i32) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), propertyname.into_param().abi(), ::std::mem::transmute(index), ::std::mem::transmute(count), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn SetPacketValuesByProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::System::Com::VARIANT>>(&self, bstrpropertyname: Param0, packetvalues: Param1, index: i32, count: i32) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), bstrpropertyname.into_param().abi(), packetvalues.into_param().abi(), ::std::mem::transmute(index), ::std::mem::transmute(count), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole_Automation`*"]
    pub unsafe fn GetFlattenedBezierPoints(&self, fittingerror: i32) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(fittingerror), &mut result__).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Transform<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTransform>>(&self, transform: Param0, applyonpenwidth: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), transform.into_param().abi(), ::std::mem::transmute(applyonpenwidth)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ScaleToRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), rectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalcomponent), ::std::mem::transmute(verticalcomponent)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), ::std::mem::transmute(degrees), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Shear(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalmultiplier), ::std::mem::transmute(verticalmultiplier)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalmultiplier), ::std::mem::transmute(verticalmultiplier)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkStrokeDisp {
    type Vtable = IInkStrokeDisp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1126445034, 37329, 19058, [150, 62, 251, 185, 24, 41, 207, 162]);
}
impl ::std::convert::From<IInkStrokeDisp> for ::windows::runtime::IUnknown {
    fn from(value: IInkStrokeDisp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkStrokeDisp> for ::windows::runtime::IUnknown {
    fn from(value: &IInkStrokeDisp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkStrokeDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkStrokeDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkStrokeDisp> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkStrokeDisp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkStrokeDisp> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkStrokeDisp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkStrokeDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkStrokeDisp {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeDisp_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, points: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, drawattrs: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, drawattrs: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, properties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cusps: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cusps: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, intersections: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plcount: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plsize: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetdescription: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, deleted: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boundingboxmode: InkBoundingBoxMode, rectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr, intersections: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr, intersections: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, radius: f32, intersects: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, x: i32, y: i32, distance: *mut f32, point: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, splitat: f32, newstroke: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, count: i32, points: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, points: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpointsset: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, count: i32, packetdata: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, index: i32, count: i32, packetvalues: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpropertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, packetvalues: ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, index: i32, count: i32, numberofpacketsset: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fittingerror: i32, flattenedbezierpoints: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: ::windows::runtime::RawPtr, applyonpenwidth: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, degrees: f32, x: f32, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkStrokes(pub ::windows::runtime::IUnknown);
impl IInkStrokes {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Ink(&self) -> ::windows::runtime::Result<IInkDisp> {
        let mut result__: <IInkDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RecognitionResult(&self) -> ::windows::runtime::Result<IInkRecognitionResult> {
        let mut result__: <IInkRecognitionResult as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRecognitionResult>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn ToString(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<IInkStrokeDisp> {
        let mut result__: <IInkStrokeDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<IInkStrokeDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokeDisp>>(&self, inkstroke: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), inkstroke.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AddStrokes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, inkstrokes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), inkstrokes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokeDisp>>(&self, inkstroke: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), inkstroke.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RemoveStrokes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkStrokes>>(&self, inkstrokes: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), inkstrokes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ModifyDrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDrawingAttributes>>(&self, drawattrs: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), drawattrs.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetBoundingBox(&self, boundingboxmode: InkBoundingBoxMode) -> ::windows::runtime::Result<IInkRectangle> {
        let mut result__: <IInkRectangle as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(boundingboxmode), &mut result__).from_abi::<IInkRectangle>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Transform<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTransform>>(&self, transform: Param0, applyonpenwidth: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), transform.into_param().abi(), ::std::mem::transmute(applyonpenwidth)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ScaleToRectangle<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), rectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Move(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalcomponent), ::std::mem::transmute(verticalcomponent)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(degrees), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Shear(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalmultiplier), ::std::mem::transmute(verticalmultiplier)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalmultiplier), ::std::mem::transmute(verticalmultiplier)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Clip<'a, Param0: ::windows::runtime::IntoParam<'a, IInkRectangle>>(&self, rectangle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), rectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RemoveRecognitionResult(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkStrokes {
    type Vtable = IInkStrokes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4059351512, 22794, 18787, [179, 174, 25, 53, 103, 27, 182, 243]);
}
impl ::std::convert::From<IInkStrokes> for ::windows::runtime::IUnknown {
    fn from(value: IInkStrokes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkStrokes> for ::windows::runtime::IUnknown {
    fn from(value: &IInkStrokes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkStrokes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkStrokes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkStrokes> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkStrokes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkStrokes> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkStrokes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkStrokes {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkStrokes {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognitionresult: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tostring: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, stroke: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkstroke: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkstrokes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkstroke: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkstrokes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, drawattrs: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boundingboxmode: InkBoundingBoxMode, boundingbox: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: ::windows::runtime::RawPtr, applyonpenwidth: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, degrees: f32, x: f32, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkTablet(pub ::windows::runtime::IUnknown);
impl IInkTablet {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn PlugAndPlayId(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MaximumInputRectangle(&self) -> ::windows::runtime::Result<IInkRectangle> {
        let mut result__: <IInkRectangle as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkRectangle>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn HardwareCapabilities(&self) -> ::windows::runtime::Result<TabletHardwareCapabilities> {
        let mut result__: <TabletHardwareCapabilities as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<TabletHardwareCapabilities>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn IsPacketPropertySupported<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, packetpropertyname: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), packetpropertyname.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyMetrics<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, propertyname: Param0, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), propertyname.into_param().abi(), ::std::mem::transmute(minimum), ::std::mem::transmute(maximum), ::std::mem::transmute(units), ::std::mem::transmute(resolution)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkTablet {
    type Vtable = IInkTablet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(769810090, 28408, 17109, [174, 233, 24, 91, 200, 27, 145, 45]);
}
impl ::std::convert::From<IInkTablet> for ::windows::runtime::IUnknown {
    fn from(value: IInkTablet) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkTablet> for ::windows::runtime::IUnknown {
    fn from(value: &IInkTablet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkTablet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkTablet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkTablet> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkTablet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkTablet> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkTablet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkTablet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkTablet {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkTablet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, rectangle: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, capabilities: *mut TabletHardwareCapabilities) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetpropertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, minimum: *mut i32, maximum: *mut i32, units: *mut TabletPropertyMetricUnit, resolution: *mut f32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkTablet2(pub ::windows::runtime::IUnknown);
impl IInkTablet2 {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DeviceKind(&self) -> ::windows::runtime::Result<TabletDeviceKind> {
        let mut result__: <TabletDeviceKind as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<TabletDeviceKind>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkTablet2 {
    type Vtable = IInkTablet2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2429098706, 64054, 18902, [149, 22, 206, 141, 87, 15, 111, 133]);
}
impl ::std::convert::From<IInkTablet2> for ::windows::runtime::IUnknown {
    fn from(value: IInkTablet2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkTablet2> for ::windows::runtime::IUnknown {
    fn from(value: &IInkTablet2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkTablet2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkTablet2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkTablet2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkTablet2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkTablet2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkTablet2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkTablet2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkTablet2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkTablet2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, kind: *mut TabletDeviceKind) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkTablet3(pub ::windows::runtime::IUnknown);
impl IInkTablet3 {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn IsMultiTouch(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MaximumCursors(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkTablet3 {
    type Vtable = IInkTablet3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2117155223, 4903, 16861, [140, 169, 121, 242, 75, 225, 114, 80]);
}
impl ::std::convert::From<IInkTablet3> for ::windows::runtime::IUnknown {
    fn from(value: IInkTablet3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkTablet3> for ::windows::runtime::IUnknown {
    fn from(value: &IInkTablet3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkTablet3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkTablet3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkTablet3> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkTablet3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkTablet3> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkTablet3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkTablet3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkTablet3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkTablet3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pismultitouch: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmaximumcursors: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkTablets(pub ::windows::runtime::IUnknown);
impl IInkTablets {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DefaultTablet(&self) -> ::windows::runtime::Result<IInkTablet> {
        let mut result__: <IInkTablet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkTablet>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Item(&self, index: i32) -> ::windows::runtime::Result<IInkTablet> {
        let mut result__: <IInkTablet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<IInkTablet>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn IsPacketPropertySupported<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, packetpropertyname: Param0) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), packetpropertyname.into_param().abi(), &mut result__).from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInkTablets {
    type Vtable = IInkTablets_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(287344345, 30585, 17717, [166, 153, 134, 43, 67, 172, 24, 99]);
}
impl ::std::convert::From<IInkTablets> for ::windows::runtime::IUnknown {
    fn from(value: IInkTablets) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkTablets> for ::windows::runtime::IUnknown {
    fn from(value: &IInkTablets) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkTablets {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkTablets {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkTablets> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkTablets) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkTablets> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkTablets) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkTablets {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkTablets {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkTablets_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, _newenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, defaulttablet: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: i32, tablet: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, packetpropertyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, supported: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkTransform(pub ::windows::runtime::IUnknown);
impl IInkTransform {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Translate(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalcomponent), ::std::mem::transmute(verticalcomponent)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Rotate(&self, degrees: f32, x: f32, y: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(degrees), ::std::mem::transmute(x), ::std::mem::transmute(y)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Reflect(&self, horizontally: i16, vertically: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontally), ::std::mem::transmute(vertically)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Shear(&self, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalcomponent), ::std::mem::transmute(verticalcomponent)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ScaleTransform(&self, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontalmultiplier), ::std::mem::transmute(verticalmultiplier)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetTransform(&self, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(em11), ::std::mem::transmute(em12), ::std::mem::transmute(em21), ::std::mem::transmute(em22), ::std::mem::transmute(edx), ::std::mem::transmute(edy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetTransform(&self, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(em11), ::std::mem::transmute(em12), ::std::mem::transmute(em21), ::std::mem::transmute(em22), ::std::mem::transmute(edx), ::std::mem::transmute(edy)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn eM11(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SeteM11(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn eM12(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SeteM12(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn eM21(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SeteM21(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn eM22(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SeteM22(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn eDx(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SeteDx(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn eDy(&self) -> ::windows::runtime::Result<f32> {
        let mut result__: <f32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<f32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SeteDy(&self, value: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn Data(&self) -> ::windows::runtime::Result<super::super::Graphics::Gdi::XFORM> {
        let mut result__: <super::super::Graphics::Gdi::XFORM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Gdi::XFORM>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn SetData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::XFORM>>(&self, xform: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), xform.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkTransform {
    type Vtable = IInkTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1633623363, 34563, 17765, [136, 226, 130, 1, 210, 236, 215, 183]);
}
impl ::std::convert::From<IInkTransform> for ::windows::runtime::IUnknown {
    fn from(value: IInkTransform) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkTransform> for ::windows::runtime::IUnknown {
    fn from(value: &IInkTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkTransform> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkTransform> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, degrees: f32, x: f32, y: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontally: i16, vertically: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalcomponent: f32, verticalcomponent: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontalmultiplier: f32, verticalmultiplier: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, em11: *mut f32, em12: *mut f32, em21: *mut f32, em22: *mut f32, edx: *mut f32, edy: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, em11: f32, em12: f32, em21: f32, em22: f32, edx: f32, edy: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xform: *mut super::super::Graphics::Gdi::XFORM) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xform: super::super::Graphics::Gdi::XFORM) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkWordList(pub ::windows::runtime::IUnknown);
impl IInkWordList {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn AddWord<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newword: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), newword.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn RemoveWord<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, removeword: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), removeword.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Merge<'a, Param0: ::windows::runtime::IntoParam<'a, IInkWordList>>(&self, mergewordlist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), mergewordlist.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkWordList {
    type Vtable = IInkWordList_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1991914641, 52015, 16491, [153, 97, 14, 12, 76, 218, 174, 242]);
}
impl ::std::convert::From<IInkWordList> for ::windows::runtime::IUnknown {
    fn from(value: IInkWordList) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkWordList> for ::windows::runtime::IUnknown {
    fn from(value: &IInkWordList) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkWordList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkWordList {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkWordList> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkWordList) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkWordList> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkWordList) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkWordList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkWordList {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWordList_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newword: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, removeword: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mergewordlist: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkWordList2(pub ::windows::runtime::IUnknown);
impl IInkWordList2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn AddWords<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, newwords: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), newwords.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInkWordList2 {
    type Vtable = IInkWordList2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(341058950, 4543, 20319, [182, 231, 73, 208, 116, 74, 171, 110]);
}
impl ::std::convert::From<IInkWordList2> for ::windows::runtime::IUnknown {
    fn from(value: IInkWordList2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkWordList2> for ::windows::runtime::IUnknown {
    fn from(value: &IInkWordList2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkWordList2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInkWordList2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IInkWordList2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IInkWordList2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IInkWordList2> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IInkWordList2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IInkWordList2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IInkWordList2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWordList2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newwords: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInputPanelWindowHandle(pub ::windows::runtime::IUnknown);
impl IInputPanelWindowHandle {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AttachedEditWindow32(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAttachedEditWindow32(&self, attachededitwindow: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(attachededitwindow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AttachedEditWindow64(&self) -> ::windows::runtime::Result<i64> {
        let mut result__: <i64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i64>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAttachedEditWindow64(&self, attachededitwindow: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(attachededitwindow)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IInputPanelWindowHandle {
    type Vtable = IInputPanelWindowHandle_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1257773127, 64964, 20419, [173, 11, 66, 36, 121, 193, 185, 53]);
}
impl ::std::convert::From<IInputPanelWindowHandle> for ::windows::runtime::IUnknown {
    fn from(value: IInputPanelWindowHandle) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInputPanelWindowHandle> for ::windows::runtime::IUnknown {
    fn from(value: &IInputPanelWindowHandle) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInputPanelWindowHandle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IInputPanelWindowHandle {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPanelWindowHandle_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachededitwindow: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachededitwindow: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachededitwindow: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachededitwindow: i64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IMathInputControl(pub ::windows::runtime::IUnknown);
impl IMathInputControl {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Show(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Hide(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn IsVisible(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetPosition(&self, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(right), ::std::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetPosition(&self, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(left), ::std::mem::transmute(top), ::std::mem::transmute(right), ::std::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Clear(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetCustomPaint(&self, element: i32, paint: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(element), ::std::mem::transmute(paint)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetCaptionText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, captiontext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), captiontext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn LoadInk<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDisp>>(&self, ink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetOwnerWindow(&self, ownerwindow: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(ownerwindow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EnableExtendedButtons(&self, extended: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(extended)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetPreviewHeight(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetPreviewHeight(&self, height: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EnableAutoGrow(&self, autogrow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(autogrow)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn AddFunctionName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, functionname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), functionname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn RemoveFunctionName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, functionname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), functionname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Ole")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Ole`*"]
    pub unsafe fn GetHoverIcon(&self) -> ::windows::runtime::Result<super::super::System::Ole::IPictureDisp> {
        let mut result__: <super::super::System::Ole::IPictureDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::System::Ole::IPictureDisp>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IMathInputControl {
    type Vtable = IMathInputControl_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3953530282, 64198, 18232, [186, 95, 255, 9, 233, 254, 71, 62]);
}
impl ::std::convert::From<IMathInputControl> for ::windows::runtime::IUnknown {
    fn from(value: IMathInputControl) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IMathInputControl> for ::windows::runtime::IUnknown {
    fn from(value: &IMathInputControl) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IMathInputControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IMathInputControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IMathInputControl> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IMathInputControl) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IMathInputControl> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IMathInputControl) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IMathInputControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IMathInputControl {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMathInputControl_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvbshown: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: *mut i32, top: *mut i32, right: *mut i32, bottom: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: i32, top: i32, right: i32, bottom: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, element: i32, paint: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, captiontext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ownerwindow: isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, extended: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, height: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, height: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, autogrow: i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, functionname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, functionname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Ole")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hoverimage: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole"))] usize,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct INKMETRIC {
    pub iHeight: i32,
    pub iFontAscent: i32,
    pub iFontDescent: i32,
    pub dwFlags: u32,
    pub color: u32,
}
impl INKMETRIC {}
impl ::std::default::Default for INKMETRIC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for INKMETRIC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("INKMETRIC").field("iHeight", &self.iHeight).field("iFontAscent", &self.iFontAscent).field("iFontDescent", &self.iFontDescent).field("dwFlags", &self.dwFlags).field("color", &self.color).finish()
    }
}
impl ::std::cmp::PartialEq for INKMETRIC {
    fn eq(&self, other: &Self) -> bool {
        self.iHeight == other.iHeight && self.iFontAscent == other.iFontAscent && self.iFontDescent == other.iFontDescent && self.dwFlags == other.dwFlags && self.color == other.color
    }
}
impl ::std::cmp::Eq for INKMETRIC {}
unsafe impl ::windows::runtime::Abi for INKMETRIC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IP_CURSOR_DOWN: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IP_INVERTED: u32 = 2u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const IP_MARGIN: u32 = 4u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IPenInputPanel(pub ::windows::runtime::IUnknown);
impl IPenInputPanel {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Busy(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Factoid(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetFactoid<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, factoid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), factoid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AttachedEditWindow(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAttachedEditWindow(&self, attachededitwindow: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(attachededitwindow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CurrentPanel(&self) -> ::windows::runtime::Result<PanelType> {
        let mut result__: <PanelType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PanelType>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetCurrentPanel(&self, currentpanel: PanelType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(currentpanel)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DefaultPanel(&self) -> ::windows::runtime::Result<PanelType> {
        let mut result__: <PanelType as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PanelType>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDefaultPanel(&self, defaultpanel: PanelType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(defaultpanel)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Visible(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetVisible(&self, visible: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(visible)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Top(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Left(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Width(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Height(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn VerticalOffset(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetVerticalOffset(&self, verticaloffset: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(verticaloffset)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn HorizontalOffset(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetHorizontalOffset(&self, horizontaloffset: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(horizontaloffset)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AutoShow(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetAutoShow(&self, autoshow: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(autoshow)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn MoveTo(&self, left: i32, top: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(left), ::std::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CommitPendingInput(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn EnableTsf(&self, enable: i16) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(enable)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPenInputPanel {
    type Vtable = IPenInputPanel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4202315907, 22343, 16448, [161, 130, 11, 14, 159, 212, 250, 199]);
}
impl ::std::convert::From<IPenInputPanel> for ::windows::runtime::IUnknown {
    fn from(value: IPenInputPanel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IPenInputPanel> for ::windows::runtime::IUnknown {
    fn from(value: &IPenInputPanel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPenInputPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPenInputPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IPenInputPanel> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: IPenInputPanel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IPenInputPanel> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &IPenInputPanel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for IPenInputPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &IPenInputPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenInputPanel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, busy: *mut i16) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, factoid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, factoid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachededitwindow: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachededitwindow: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentpanel: *mut PanelType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentpanel: PanelType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdefaultpanel: *mut PanelType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, defaultpanel: PanelType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visible: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visible: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, top: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, width: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, height: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, verticaloffset: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, verticaloffset: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontaloffset: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, horizontaloffset: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pautoshow: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, autoshow: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, left: i32, top: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, enable: i16) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRealTimeStylus(pub ::windows::runtime::IUnknown);
impl IRealTimeStylus {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Enabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn HWND(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE_PTR> {
        let mut result__: <super::super::Foundation::HANDLE_PTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE_PTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetHWND<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE_PTR>>(&self, hwnd: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn WindowInputRectangle(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetWindowInputRectangle(&self, prcwndinputrect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(prcwndinputrect)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AddStylusSyncPlugin<'a, Param1: ::windows::runtime::IntoParam<'a, IStylusSyncPlugin>>(&self, iindex: u32, piplugin: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex), piplugin.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RemoveStylusSyncPlugin(&self, iindex: u32, ppiplugin: *mut ::std::option::Option<IStylusSyncPlugin>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex), ::std::mem::transmute(ppiplugin)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RemoveAllStylusSyncPlugins(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetStylusSyncPlugin(&self, iindex: u32) -> ::windows::runtime::Result<IStylusSyncPlugin> {
        let mut result__: <IStylusSyncPlugin as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex), &mut result__).from_abi::<IStylusSyncPlugin>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetStylusSyncPluginCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AddStylusAsyncPlugin<'a, Param1: ::windows::runtime::IntoParam<'a, IStylusAsyncPlugin>>(&self, iindex: u32, piplugin: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex), piplugin.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RemoveStylusAsyncPlugin(&self, iindex: u32, ppiplugin: *mut ::std::option::Option<IStylusAsyncPlugin>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex), ::std::mem::transmute(ppiplugin)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RemoveAllStylusAsyncPlugins(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetStylusAsyncPlugin(&self, iindex: u32) -> ::windows::runtime::Result<IStylusAsyncPlugin> {
        let mut result__: <IStylusAsyncPlugin as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(iindex), &mut result__).from_abi::<IStylusAsyncPlugin>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetStylusAsyncPluginCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ChildRealTimeStylusPlugin(&self) -> ::windows::runtime::Result<IRealTimeStylus> {
        let mut result__: <IRealTimeStylus as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IRealTimeStylus>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_ChildRealTimeStylusPlugin<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirts: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), pirts.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AddCustomStylusDataToQueue(&self, sq: StylusQueue, pguidid: *const ::windows::runtime::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(sq), ::std::mem::transmute(pguidid), ::std::mem::transmute(cbdata), ::std::mem::transmute(pbdata)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ClearStylusQueues(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetAllTabletsMode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fusemouseforinput: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), fusemouseforinput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetSingleTabletMode<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTablet>>(&self, pitablet: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), pitablet.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetTablet(&self) -> ::windows::runtime::Result<IInkTablet> {
        let mut result__: <IInkTablet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkTablet>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetTabletContextIdFromTablet<'a, Param0: ::windows::runtime::IntoParam<'a, IInkTablet>>(&self, pitablet: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), pitablet.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetTabletFromTabletContextId(&self, tcid: u32) -> ::windows::runtime::Result<IInkTablet> {
        let mut result__: <IInkTablet as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(tcid), &mut result__).from_abi::<IInkTablet>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetAllTabletContextIds(&self, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(pctcidcount), ::std::mem::transmute(pptcids)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetStyluses(&self) -> ::windows::runtime::Result<IInkCursors> {
        let mut result__: <IInkCursors as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkCursors>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetStylusForId(&self, sid: u32) -> ::windows::runtime::Result<IInkCursor> {
        let mut result__: <IInkCursor as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), ::std::mem::transmute(sid), &mut result__).from_abi::<IInkCursor>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDesiredPacketDescription(&self, cproperties: u32, ppropertyguids: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(cproperties), ::std::mem::transmute(ppropertyguids)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetDesiredPacketDescription(&self, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcproperties), ::std::mem::transmute(pppropertyguids)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn GetPacketDescriptionData(&self, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(tcid), ::std::mem::transmute(pfinktodevicescalex), ::std::mem::transmute(pfinktodevicescaley), ::std::mem::transmute(pcpacketproperties), ::std::mem::transmute(pppacketproperties)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRealTimeStylus {
    type Vtable = IRealTimeStylus_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2830851362, 12612, 19067, [147, 205, 243, 74, 22, 190, 81, 58]);
}
impl ::std::convert::From<IRealTimeStylus> for ::windows::runtime::IUnknown {
    fn from(value: IRealTimeStylus) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRealTimeStylus> for ::windows::runtime::IUnknown {
    fn from(value: &IRealTimeStylus) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRealTimeStylus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRealTimeStylus {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRealTimeStylus_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HANDLE_PTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcwndinputrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcwndinputrect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32, piplugin: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32, ppiplugin: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32, ppiplugin: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcplugins: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32, piplugin: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32, ppiplugin: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iindex: u32, ppiplugin: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcplugins: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppirts: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirts: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sq: StylusQueue, pguidid: *const ::windows::runtime::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fusemouseforinput: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitablet: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppisingletablet: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitablet: ::windows::runtime::RawPtr, ptcid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tcid: u32, ppitablet: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctcidcount: *mut u32, pptcids: *mut *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiinkcursors: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sid: u32, ppiinkcursor: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cproperties: u32, ppropertyguids: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcproperties: *mut u32, pppropertyguids: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tcid: u32, pfinktodevicescalex: *mut f32, pfinktodevicescaley: *mut f32, pcpacketproperties: *mut u32, pppacketproperties: *mut *mut PACKET_PROPERTY) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRealTimeStylus2(pub ::windows::runtime::IUnknown);
impl IRealTimeStylus2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn FlicksEnabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetFlicksEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRealTimeStylus2 {
    type Vtable = IRealTimeStylus2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3052578509, 12665, 19006, [185, 196, 187, 88, 101, 150, 43, 226]);
}
impl ::std::convert::From<IRealTimeStylus2> for ::windows::runtime::IUnknown {
    fn from(value: IRealTimeStylus2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRealTimeStylus2> for ::windows::runtime::IUnknown {
    fn from(value: &IRealTimeStylus2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRealTimeStylus2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRealTimeStylus2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRealTimeStylus2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRealTimeStylus3(pub ::windows::runtime::IUnknown);
impl IRealTimeStylus3 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn MultiTouchEnabled(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetMultiTouchEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fenable: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fenable.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRealTimeStylus3 {
    type Vtable = IRealTimeStylus3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3607244963, 27014, 16465, [181, 122, 28, 246, 159, 77, 157, 181]);
}
impl ::std::convert::From<IRealTimeStylus3> for ::windows::runtime::IUnknown {
    fn from(value: IRealTimeStylus3) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRealTimeStylus3> for ::windows::runtime::IUnknown {
    fn from(value: &IRealTimeStylus3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRealTimeStylus3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRealTimeStylus3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRealTimeStylus3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfenable: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fenable: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRealTimeStylusSynchronization(pub ::windows::runtime::IUnknown);
impl IRealTimeStylusSynchronization {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AcquireLock(&self, lock: RealTimeStylusLockType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lock)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn ReleaseLock(&self, lock: RealTimeStylusLockType) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(lock)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRealTimeStylusSynchronization {
    type Vtable = IRealTimeStylusSynchronization_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2861034168, 43850, 19690, [181, 203, 70, 216, 76, 106, 37, 9]);
}
impl ::std::convert::From<IRealTimeStylusSynchronization> for ::windows::runtime::IUnknown {
    fn from(value: IRealTimeStylusSynchronization) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRealTimeStylusSynchronization> for ::windows::runtime::IUnknown {
    fn from(value: &IRealTimeStylusSynchronization) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRealTimeStylusSynchronization {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRealTimeStylusSynchronization {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRealTimeStylusSynchronization_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lock: RealTimeStylusLockType) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lock: RealTimeStylusLockType) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISketchInk(pub ::windows::runtime::IUnknown);
impl ISketchInk {}
unsafe impl ::windows::runtime::Interface for ISketchInk {
    type Vtable = ISketchInk_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3025548936, 39147, 17990, [178, 121, 68, 218, 20, 212, 87, 72]);
}
impl ::std::convert::From<ISketchInk> for ::windows::runtime::IUnknown {
    fn from(value: ISketchInk) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISketchInk> for ::windows::runtime::IUnknown {
    fn from(value: &ISketchInk) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISketchInk {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISketchInk {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<ISketchInk> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: ISketchInk) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&ISketchInk> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &ISketchInk) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for ISketchInk {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &ISketchInk {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISketchInk_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStrokeBuilder(pub ::windows::runtime::IUnknown);
impl IStrokeBuilder {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CreateStroke(&self, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::std::option::Option<IInkStrokeDisp>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(cpktbufflength),
            ::std::mem::transmute(ppackets),
            ::std::mem::transmute(cpacketproperties),
            ::std::mem::transmute(ppacketproperties),
            ::std::mem::transmute(finktodevicescalex),
            ::std::mem::transmute(finktodevicescaley),
            ::std::mem::transmute(ppiinkstroke),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn BeginStroke(&self, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::std::option::Option<IInkStrokeDisp>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(tcid),
            ::std::mem::transmute(sid),
            ::std::mem::transmute(ppacket),
            ::std::mem::transmute(cpacketproperties),
            ::std::mem::transmute(ppacketproperties),
            ::std::mem::transmute(finktodevicescalex),
            ::std::mem::transmute(finktodevicescaley),
            ::std::mem::transmute(ppiinkstroke),
        )
        .ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn AppendPackets(&self, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(tcid), ::std::mem::transmute(sid), ::std::mem::transmute(cpktbufflength), ::std::mem::transmute(ppackets)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn EndStroke(&self, tcid: u32, sid: u32, ppiinkstroke: *mut ::std::option::Option<IInkStrokeDisp>, pdirtyrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(tcid), ::std::mem::transmute(sid), ::std::mem::transmute(ppiinkstroke), ::std::mem::transmute(pdirtyrect)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Ink(&self) -> ::windows::runtime::Result<IInkDisp> {
        let mut result__: <IInkDisp as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IInkDisp>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn putref_Ink<'a, Param0: ::windows::runtime::IntoParam<'a, IInkDisp>>(&self, piinkobj: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), piinkobj.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IStrokeBuilder {
    type Vtable = IStrokeBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2784841261, 50251, 16530, [145, 119, 38, 9, 5, 235, 103, 43]);
}
impl ::std::convert::From<IStrokeBuilder> for ::windows::runtime::IUnknown {
    fn from(value: IStrokeBuilder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IStrokeBuilder> for ::windows::runtime::IUnknown {
    fn from(value: &IStrokeBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStrokeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStrokeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStrokeBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cpktbufflength: u32, ppackets: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tcid: u32, sid: u32, ppacket: *const i32, cpacketproperties: u32, ppacketproperties: *const PACKET_PROPERTY, finktodevicescalex: f32, finktodevicescaley: f32, ppiinkstroke: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tcid: u32, sid: u32, cpktbufflength: u32, ppackets: *const i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tcid: u32, sid: u32, ppiinkstroke: *mut ::windows::runtime::RawPtr, pdirtyrect: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppiinkobj: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piinkobj: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStylusAsyncPlugin(pub ::windows::runtime::IUnknown);
impl IStylusAsyncPlugin {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RealTimeStylusEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(ctcidcount), ::std::mem::transmute(ptcids)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RealTimeStylusDisabled<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(ctcidcount), ::std::mem::transmute(ptcids)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn StylusInRange<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(tcid), ::std::mem::transmute(sid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn StylusOutOfRange<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(tcid), ::std::mem::transmute(sid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusDown<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpropcountperpkt), ::std::mem::transmute(ppacket), ::std::mem::transmute(ppinoutpkt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusUp<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpropcountperpkt), ::std::mem::transmute(ppacket), ::std::mem::transmute(ppinoutpkt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusButtonDown<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(sid), ::std::mem::transmute(pguidstylusbutton), ::std::mem::transmute(pstyluspos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusButtonUp<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(sid), ::std::mem::transmute(pguidstylusbutton), ::std::mem::transmute(pstyluspos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InAirPackets<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpktcount), ::std::mem::transmute(cpktbufflength), ::std::mem::transmute(ppackets), ::std::mem::transmute(pcinoutpkts), ::std::mem::transmute(ppinoutpkts)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Packets<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpktcount), ::std::mem::transmute(cpktbufflength), ::std::mem::transmute(ppackets), ::std::mem::transmute(pcinoutpkts), ::std::mem::transmute(ppinoutpkts)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CustomStylusDataAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pguidid: *const ::windows::runtime::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pguidid), ::std::mem::transmute(cbdata), ::std::mem::transmute(pbdata)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SystemEvent<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>, Param4: ::windows::runtime::IntoParam<'a, SYSTEM_EVENT_DATA>>(&self, pirtssrc: Param0, tcid: u32, sid: u32, event: u16, eventdata: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(tcid), ::std::mem::transmute(sid), ::std::mem::transmute(event), eventdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn TabletAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>, Param1: ::windows::runtime::IntoParam<'a, IInkTablet>>(&self, pirtssrc: Param0, pitablet: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), pitablet.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn TabletRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, itabletindex: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(itabletindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Error<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>, Param1: ::windows::runtime::IntoParam<'a, IStylusPlugin>>(&self, pirtssrc: Param0, piplugin: Param1, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::runtime::HRESULT, lptrkey: *mut isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), piplugin.into_param().abi(), ::std::mem::transmute(datainterest), ::std::mem::transmute(hrerrorcode), ::std::mem::transmute(lptrkey)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn UpdateMapping<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DataInterest(&self) -> ::windows::runtime::Result<RealTimeStylusDataInterest> {
        let mut result__: <RealTimeStylusDataInterest as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<RealTimeStylusDataInterest>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStylusAsyncPlugin {
    type Vtable = IStylusAsyncPlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2815207514, 12732, 19666, [170, 220, 50, 137, 163, 175, 17, 200]);
}
impl ::std::convert::From<IStylusAsyncPlugin> for ::windows::runtime::IUnknown {
    fn from(value: IStylusAsyncPlugin) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IStylusAsyncPlugin> for ::windows::runtime::IUnknown {
    fn from(value: &IStylusAsyncPlugin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStylusAsyncPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStylusAsyncPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IStylusAsyncPlugin> for IStylusPlugin {
    fn from(value: IStylusAsyncPlugin) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStylusAsyncPlugin> for IStylusPlugin {
    fn from(value: &IStylusAsyncPlugin) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStylusPlugin> for IStylusAsyncPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStylusPlugin> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IStylusPlugin>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStylusPlugin> for &IStylusAsyncPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStylusPlugin> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IStylusPlugin>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylusAsyncPlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, tcid: u32, sid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, tcid: u32, sid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pguidid: *const ::windows::runtime::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pitablet: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, itabletindex: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, piplugin: ::windows::runtime::RawPtr, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::runtime::HRESULT, lptrkey: *mut isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatainterest: *mut RealTimeStylusDataInterest) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStylusPlugin(pub ::windows::runtime::IUnknown);
impl IStylusPlugin {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RealTimeStylusEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(ctcidcount), ::std::mem::transmute(ptcids)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RealTimeStylusDisabled<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(ctcidcount), ::std::mem::transmute(ptcids)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn StylusInRange<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(tcid), ::std::mem::transmute(sid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn StylusOutOfRange<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(tcid), ::std::mem::transmute(sid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusDown<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpropcountperpkt), ::std::mem::transmute(ppacket), ::std::mem::transmute(ppinoutpkt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusUp<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpropcountperpkt), ::std::mem::transmute(ppacket), ::std::mem::transmute(ppinoutpkt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusButtonDown<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(sid), ::std::mem::transmute(pguidstylusbutton), ::std::mem::transmute(pstyluspos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusButtonUp<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(sid), ::std::mem::transmute(pguidstylusbutton), ::std::mem::transmute(pstyluspos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InAirPackets<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpktcount), ::std::mem::transmute(cpktbufflength), ::std::mem::transmute(ppackets), ::std::mem::transmute(pcinoutpkts), ::std::mem::transmute(ppinoutpkts)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Packets<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpktcount), ::std::mem::transmute(cpktbufflength), ::std::mem::transmute(ppackets), ::std::mem::transmute(pcinoutpkts), ::std::mem::transmute(ppinoutpkts)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CustomStylusDataAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pguidid: *const ::windows::runtime::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pguidid), ::std::mem::transmute(cbdata), ::std::mem::transmute(pbdata)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SystemEvent<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>, Param4: ::windows::runtime::IntoParam<'a, SYSTEM_EVENT_DATA>>(&self, pirtssrc: Param0, tcid: u32, sid: u32, event: u16, eventdata: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(tcid), ::std::mem::transmute(sid), ::std::mem::transmute(event), eventdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn TabletAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>, Param1: ::windows::runtime::IntoParam<'a, IInkTablet>>(&self, pirtssrc: Param0, pitablet: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), pitablet.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn TabletRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, itabletindex: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(itabletindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Error<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>, Param1: ::windows::runtime::IntoParam<'a, IStylusPlugin>>(&self, pirtssrc: Param0, piplugin: Param1, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::runtime::HRESULT, lptrkey: *mut isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), piplugin.into_param().abi(), ::std::mem::transmute(datainterest), ::std::mem::transmute(hrerrorcode), ::std::mem::transmute(lptrkey)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn UpdateMapping<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DataInterest(&self) -> ::windows::runtime::Result<RealTimeStylusDataInterest> {
        let mut result__: <RealTimeStylusDataInterest as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<RealTimeStylusDataInterest>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStylusPlugin {
    type Vtable = IStylusPlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2819897048, 18263, 20433, [161, 133, 19, 63, 151, 198, 197, 69]);
}
impl ::std::convert::From<IStylusPlugin> for ::windows::runtime::IUnknown {
    fn from(value: IStylusPlugin) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IStylusPlugin> for ::windows::runtime::IUnknown {
    fn from(value: &IStylusPlugin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStylusPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStylusPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylusPlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, tcid: u32, sid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, tcid: u32, sid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pguidid: *const ::windows::runtime::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pitablet: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, itabletindex: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, piplugin: ::windows::runtime::RawPtr, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::runtime::HRESULT, lptrkey: *mut isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatainterest: *mut RealTimeStylusDataInterest) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IStylusSyncPlugin(pub ::windows::runtime::IUnknown);
impl IStylusSyncPlugin {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RealTimeStylusEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(ctcidcount), ::std::mem::transmute(ptcids)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn RealTimeStylusDisabled<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(ctcidcount), ::std::mem::transmute(ptcids)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn StylusInRange<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(tcid), ::std::mem::transmute(sid)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn StylusOutOfRange<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, tcid: u32, sid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(tcid), ::std::mem::transmute(sid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusDown<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpropcountperpkt), ::std::mem::transmute(ppacket), ::std::mem::transmute(ppinoutpkt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusUp<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpropcountperpkt), ::std::mem::transmute(ppacket), ::std::mem::transmute(ppinoutpkt)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusButtonDown<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(sid), ::std::mem::transmute(pguidstylusbutton), ::std::mem::transmute(pstyluspos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn StylusButtonUp<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(sid), ::std::mem::transmute(pguidstylusbutton), ::std::mem::transmute(pstyluspos)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InAirPackets<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpktcount), ::std::mem::transmute(cpktbufflength), ::std::mem::transmute(ppackets), ::std::mem::transmute(pcinoutpkts), ::std::mem::transmute(ppinoutpkts)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Packets<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pstylusinfo), ::std::mem::transmute(cpktcount), ::std::mem::transmute(cpktbufflength), ::std::mem::transmute(ppackets), ::std::mem::transmute(pcinoutpkts), ::std::mem::transmute(ppinoutpkts)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CustomStylusDataAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, pguidid: *const ::windows::runtime::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(pguidid), ::std::mem::transmute(cbdata), ::std::mem::transmute(pbdata)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SystemEvent<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>, Param4: ::windows::runtime::IntoParam<'a, SYSTEM_EVENT_DATA>>(&self, pirtssrc: Param0, tcid: u32, sid: u32, event: u16, eventdata: Param4) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(tcid), ::std::mem::transmute(sid), ::std::mem::transmute(event), eventdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn TabletAdded<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>, Param1: ::windows::runtime::IntoParam<'a, IInkTablet>>(&self, pirtssrc: Param0, pitablet: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), pitablet.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn TabletRemoved<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0, itabletindex: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), ::std::mem::transmute(itabletindex)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Error<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>, Param1: ::windows::runtime::IntoParam<'a, IStylusPlugin>>(&self, pirtssrc: Param0, piplugin: Param1, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::runtime::HRESULT, lptrkey: *mut isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi(), piplugin.into_param().abi(), ::std::mem::transmute(datainterest), ::std::mem::transmute(hrerrorcode), ::std::mem::transmute(lptrkey)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn UpdateMapping<'a, Param0: ::windows::runtime::IntoParam<'a, IRealTimeStylus>>(&self, pirtssrc: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pirtssrc.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DataInterest(&self) -> ::windows::runtime::Result<RealTimeStylusDataInterest> {
        let mut result__: <RealTimeStylusDataInterest as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<RealTimeStylusDataInterest>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStylusSyncPlugin {
    type Vtable = IStylusSyncPlugin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2706878836, 18479, 19825, [163, 246, 58, 65, 221, 209, 27, 233]);
}
impl ::std::convert::From<IStylusSyncPlugin> for ::windows::runtime::IUnknown {
    fn from(value: IStylusSyncPlugin) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IStylusSyncPlugin> for ::windows::runtime::IUnknown {
    fn from(value: &IStylusSyncPlugin) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IStylusSyncPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IStylusSyncPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IStylusSyncPlugin> for IStylusPlugin {
    fn from(value: IStylusSyncPlugin) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IStylusSyncPlugin> for IStylusPlugin {
    fn from(value: &IStylusSyncPlugin) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStylusPlugin> for IStylusSyncPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStylusPlugin> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IStylusPlugin>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IStylusPlugin> for &IStylusSyncPlugin {
    fn into_param(self) -> ::windows::runtime::Param<'a, IStylusPlugin> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IStylusPlugin>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylusSyncPlugin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, ctcidcount: u32, ptcids: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, tcid: u32, sid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, tcid: u32, sid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpropcountperpkt: u32, ppacket: *const i32, ppinoutpkt: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, sid: u32, pguidstylusbutton: *const ::windows::runtime::GUID, pstyluspos: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pstylusinfo: *const StylusInfo, cpktcount: u32, cpktbufflength: u32, ppackets: *const i32, pcinoutpkts: *mut u32, ppinoutpkts: *mut *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pguidid: *const ::windows::runtime::GUID, cbdata: u32, pbdata: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, tcid: u32, sid: u32, event: u16, eventdata: SYSTEM_EVENT_DATA) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, pitablet: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, itabletindex: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr, piplugin: ::windows::runtime::RawPtr, datainterest: RealTimeStylusDataInterest, hrerrorcode: ::windows::runtime::HRESULT, lptrkey: *mut isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pirtssrc: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatainterest: *mut RealTimeStylusDataInterest) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITextInputPanel(pub ::windows::runtime::IUnknown);
impl ITextInputPanel {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn AttachedEditWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetAttachedEditWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, attachededitwindow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), attachededitwindow.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CurrentInteractionMode(&self) -> ::windows::runtime::Result<InteractionMode> {
        let mut result__: <InteractionMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InteractionMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DefaultInPlaceState(&self) -> ::windows::runtime::Result<InPlaceState> {
        let mut result__: <InPlaceState as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InPlaceState>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDefaultInPlaceState(&self, state: InPlaceState) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(state)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CurrentInPlaceState(&self) -> ::windows::runtime::Result<InPlaceState> {
        let mut result__: <InPlaceState as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InPlaceState>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn DefaultInputArea(&self) -> ::windows::runtime::Result<PanelInputArea> {
        let mut result__: <PanelInputArea as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PanelInputArea>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetDefaultInputArea(&self, area: PanelInputArea) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(area)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CurrentInputArea(&self) -> ::windows::runtime::Result<PanelInputArea> {
        let mut result__: <PanelInputArea as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<PanelInputArea>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CurrentCorrectionMode(&self) -> ::windows::runtime::Result<CorrectionMode> {
        let mut result__: <CorrectionMode as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<CorrectionMode>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn PreferredInPlaceDirection(&self) -> ::windows::runtime::Result<InPlaceDirection> {
        let mut result__: <InPlaceDirection as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<InPlaceDirection>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetPreferredInPlaceDirection(&self, direction: InPlaceDirection) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(direction)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn ExpandPostInsertionCorrection(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetExpandPostInsertionCorrection<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, expand: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), expand.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InPlaceVisibleOnFocus(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetInPlaceVisibleOnFocus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, visible: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), visible.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InPlaceBoundingRectangle(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn PopUpCorrectionHeight(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn PopDownCorrectionHeight(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CommitPendingInput(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn SetInPlaceVisibility<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, visible: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), visible.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetInPlacePosition(&self, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(xposition), ::std::mem::transmute(yposition), ::std::mem::transmute(position)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn SetInPlaceHoverTargetPosition(&self, xposition: i32, yposition: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(xposition), ::std::mem::transmute(yposition)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, ITextInputPanelEventSink>>(&self, eventsink: Param0, eventmask: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), eventsink.into_param().abi(), ::std::mem::transmute(eventmask)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn Unadvise<'a, Param0: ::windows::runtime::IntoParam<'a, ITextInputPanelEventSink>>(&self, eventsink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), eventsink.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextInputPanel {
    type Vtable = ITextInputPanel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1802134949, 27379, 18114, [182, 234, 86, 205, 31, 128, 223, 113]);
}
impl ::std::convert::From<ITextInputPanel> for ::windows::runtime::IUnknown {
    fn from(value: ITextInputPanel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextInputPanel> for ::windows::runtime::IUnknown {
    fn from(value: &ITextInputPanel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextInputPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextInputPanel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachededitwindow: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, attachededitwindow: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, currentinteractionmode: *mut InteractionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: *mut InPlaceState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: InPlaceState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, state: *mut InPlaceState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, area: *mut PanelInputArea) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, area: PanelInputArea) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, area: *mut PanelInputArea) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, mode: *mut CorrectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, direction: *mut InPlaceDirection) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, direction: InPlaceDirection) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, expand: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, expand: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visible: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visible: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, boundingrectangle: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, height: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, height: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, visible: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xposition: i32, yposition: i32, position: CorrectionPosition) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xposition: i32, yposition: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventsink: ::windows::runtime::RawPtr, eventmask: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventsink: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITextInputPanelEventSink(pub ::windows::runtime::IUnknown);
impl ITextInputPanelEventSink {
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn InPlaceStateChanging(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(oldinplacestate), ::std::mem::transmute(newinplacestate)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn InPlaceStateChanged(&self, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(oldinplacestate), ::std::mem::transmute(newinplacestate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InPlaceSizeChanging<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, oldboundingrectangle: Param0, newboundingrectangle: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), oldboundingrectangle.into_param().abi(), newboundingrectangle.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InPlaceSizeChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, oldboundingrectangle: Param0, newboundingrectangle: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), oldboundingrectangle.into_param().abi(), newboundingrectangle.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn InputAreaChanging(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(oldinputarea), ::std::mem::transmute(newinputarea)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn InputAreaChanged(&self, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(oldinputarea), ::std::mem::transmute(newinputarea)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CorrectionModeChanging(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(oldcorrectionmode), ::std::mem::transmute(newcorrectionmode)).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn CorrectionModeChanged(&self, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(oldcorrectionmode), ::std::mem::transmute(newcorrectionmode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InPlaceVisibilityChanging<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, oldvisible: Param0, newvisible: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), oldvisible.into_param().abi(), newvisible.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn InPlaceVisibilityChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, oldvisible: Param0, newvisible: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), oldvisible.into_param().abi(), newvisible.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Com`*"]
    pub unsafe fn TextInserting(&self, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(ink)).ok()
    }
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_System_Com`*"]
    pub unsafe fn TextInserted(&self, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(ink)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITextInputPanelEventSink {
    type Vtable = ITextInputPanelEventSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(659948552, 36452, 20449, [128, 78, 66, 18, 1, 88, 75, 49]);
}
impl ::std::convert::From<ITextInputPanelEventSink> for ::windows::runtime::IUnknown {
    fn from(value: ITextInputPanelEventSink) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextInputPanelEventSink> for ::windows::runtime::IUnknown {
    fn from(value: &ITextInputPanelEventSink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextInputPanelEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextInputPanelEventSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanelEventSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldinplacestate: InPlaceState, newinplacestate: InPlaceState) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldboundingrectangle: super::super::Foundation::RECT, newboundingrectangle: super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldinputarea: PanelInputArea, newinputarea: PanelInputArea) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldcorrectionmode: CorrectionMode, newcorrectionmode: CorrectionMode) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oldvisible: super::super::Foundation::BOOL, newvisible: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ink: *const super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITextInputPanelRunInfo(pub ::windows::runtime::IUnknown);
impl ITextInputPanelRunInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn IsTipRunning(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITextInputPanelRunInfo {
    type Vtable = ITextInputPanelRunInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2671920488, 6432, 18636, [152, 17, 169, 147, 203, 245, 173, 186]);
}
impl ::std::convert::From<ITextInputPanelRunInfo> for ::windows::runtime::IUnknown {
    fn from(value: ITextInputPanelRunInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITextInputPanelRunInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ITextInputPanelRunInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITextInputPanelRunInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITextInputPanelRunInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextInputPanelRunInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfrunning: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITipAutoCompleteClient(pub ::windows::runtime::IUnknown);
impl ITipAutoCompleteClient {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn AdviseProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ITipAutoCompleteProvider>>(&self, hwndfield: Param0, piprovider: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwndfield.into_param().abi(), piprovider.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn UnadviseProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ITipAutoCompleteProvider>>(&self, hwndfield: Param0, piprovider: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), hwndfield.into_param().abi(), piprovider.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_UI_TabletPC`*"]
    pub unsafe fn UserSelection(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn PreferredRects(&self, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(prcaclist), ::std::mem::transmute(prcfield), ::std::mem::transmute(prcmodifiedaclist), ::std::mem::transmute(pfshownabovetip)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn RequestShowUI<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwndlist: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), hwndlist.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITipAutoCompleteClient {
    type Vtable = ITipAutoCompleteClient_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1577553411, 33381, 19390, [148, 135, 210, 66, 237, 190, 249, 16]);
}
impl ::std::convert::From<ITipAutoCompleteClient> for ::windows::runtime::IUnknown {
    fn from(value: ITipAutoCompleteClient) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITipAutoCompleteClient> for ::windows::runtime::IUnknown {
    fn from(value: &ITipAutoCompleteClient) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITipAutoCompleteClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITipAutoCompleteClient {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipAutoCompleteClient_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndfield: super::super::Foundation::HWND, piprovider: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndfield: super::super::Foundation::HWND, piprovider: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prcaclist: *const super::super::Foundation::RECT, prcfield: *const super::super::Foundation::RECT, prcmodifiedaclist: *mut super::super::Foundation::RECT, pfshownabovetip: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndlist: super::super::Foundation::HWND, pfallowshowing: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ITipAutoCompleteProvider(pub ::windows::runtime::IUnknown);
impl ITipAutoCompleteProvider {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn UpdatePendingText<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrpendingtext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), bstrpendingtext.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
    pub unsafe fn Show<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fshow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fshow.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITipAutoCompleteProvider {
    type Vtable = ITipAutoCompleteProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2087515245, 33796, 18105, [173, 51, 245, 182, 3, 109, 64, 7]);
}
impl ::std::convert::From<ITipAutoCompleteProvider> for ::windows::runtime::IUnknown {
    fn from(value: ITipAutoCompleteProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ITipAutoCompleteProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ITipAutoCompleteProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ITipAutoCompleteProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ITipAutoCompleteProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITipAutoCompleteProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bstrpendingtext: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fshow: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InPlaceDirection(pub i32);
pub const InPlaceDirection_Auto: InPlaceDirection = InPlaceDirection(0i32);
pub const InPlaceDirection_Bottom: InPlaceDirection = InPlaceDirection(1i32);
pub const InPlaceDirection_Top: InPlaceDirection = InPlaceDirection(2i32);
impl ::std::convert::From<i32> for InPlaceDirection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InPlaceDirection {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InPlaceState(pub i32);
pub const InPlaceState_Auto: InPlaceState = InPlaceState(0i32);
pub const InPlaceState_HoverTarget: InPlaceState = InPlaceState(1i32);
pub const InPlaceState_Expanded: InPlaceState = InPlaceState(2i32);
impl ::std::convert::From<i32> for InPlaceState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InPlaceState {
    type Abi = Self;
}
pub const Ink: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(333335106, 36129, 19598, [191, 156, 143, 105, 203, 6, 143, 202]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkApplicationGesture {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkApplicationGesture {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkBoundingBoxMode(pub i32);
pub const IBBM_Default: InkBoundingBoxMode = InkBoundingBoxMode(0i32);
pub const IBBM_NoCurveFit: InkBoundingBoxMode = InkBoundingBoxMode(1i32);
pub const IBBM_CurveFit: InkBoundingBoxMode = InkBoundingBoxMode(2i32);
pub const IBBM_PointsOnly: InkBoundingBoxMode = InkBoundingBoxMode(3i32);
pub const IBBM_Union: InkBoundingBoxMode = InkBoundingBoxMode(4i32);
impl ::std::convert::From<i32> for InkBoundingBoxMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkBoundingBoxMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkClipboardFormats {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkClipboardFormats {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkClipboardModes(pub i32);
pub const ICB_Copy: InkClipboardModes = InkClipboardModes(0i32);
pub const ICB_Cut: InkClipboardModes = InkClipboardModes(1i32);
pub const ICB_ExtractOnly: InkClipboardModes = InkClipboardModes(48i32);
pub const ICB_DelayedCopy: InkClipboardModes = InkClipboardModes(32i32);
pub const ICB_Default: InkClipboardModes = InkClipboardModes(0i32);
impl ::std::convert::From<i32> for InkClipboardModes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkClipboardModes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkCollectionMode(pub i32);
pub const ICM_InkOnly: InkCollectionMode = InkCollectionMode(0i32);
pub const ICM_GestureOnly: InkCollectionMode = InkCollectionMode(1i32);
pub const ICM_InkAndGesture: InkCollectionMode = InkCollectionMode(2i32);
impl ::std::convert::From<i32> for InkCollectionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkCollectionMode {
    type Abi = Self;
}
pub const InkCollector: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1140528467, 44404, 20200, [136, 228, 62, 109, 170, 201, 21, 219]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const InkCollectorClipInkToMargin: i32 = 0i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const InkCollectorDefaultMargin: i32 = -2147483648i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkCollectorEventInterest {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkCollectorEventInterest {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkCursorButtonState(pub i32);
pub const ICBS_Unavailable: InkCursorButtonState = InkCursorButtonState(0i32);
pub const ICBS_Up: InkCursorButtonState = InkCursorButtonState(1i32);
pub const ICBS_Down: InkCursorButtonState = InkCursorButtonState(2i32);
impl ::std::convert::From<i32> for InkCursorButtonState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkCursorButtonState {
    type Abi = Self;
}
pub const InkDisp: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2474383924, 5405, 17936, [156, 166, 168, 204, 155, 219, 93, 131]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkDisplayMode(pub i32);
pub const IDM_Ink: InkDisplayMode = InkDisplayMode(0i32);
pub const IDM_Text: InkDisplayMode = InkDisplayMode(1i32);
impl ::std::convert::From<i32> for InkDisplayMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkDisplayMode {
    type Abi = Self;
}
pub const InkDivider: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2287269536, 18051, 19175, [145, 145, 117, 47, 230, 70, 18, 195]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkDivisionType(pub i32);
pub const IDT_Segment: InkDivisionType = InkDivisionType(0i32);
pub const IDT_Line: InkDivisionType = InkDivisionType(1i32);
pub const IDT_Paragraph: InkDivisionType = InkDivisionType(2i32);
pub const IDT_Drawing: InkDivisionType = InkDivisionType(3i32);
impl ::std::convert::From<i32> for InkDivisionType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkDivisionType {
    type Abi = Self;
}
pub const InkDrawingAttributes: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3636408994, 1445, 17603, [179, 170, 94, 128, 172, 125, 37, 118]);
pub const InkEdit: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3855243765, 22468, 19928, [155, 214, 29, 238, 237, 210, 122, 244]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkEditStatus(pub i32);
pub const IES_Idle: InkEditStatus = InkEditStatus(0i32);
pub const IES_Collecting: InkEditStatus = InkEditStatus(1i32);
pub const IES_Recognizing: InkEditStatus = InkEditStatus(2i32);
impl ::std::convert::From<i32> for InkEditStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkEditStatus {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkExtractFlags(pub i32);
pub const IEF_CopyFromOriginal: InkExtractFlags = InkExtractFlags(0i32);
pub const IEF_RemoveFromOriginal: InkExtractFlags = InkExtractFlags(1i32);
pub const IEF_Default: InkExtractFlags = InkExtractFlags(1i32);
impl ::std::convert::From<i32> for InkExtractFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkExtractFlags {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkInsertMode(pub i32);
pub const IEM_InsertText: InkInsertMode = InkInsertMode(0i32);
pub const IEM_InsertInk: InkInsertMode = InkInsertMode(1i32);
impl ::std::convert::From<i32> for InkInsertMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkInsertMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const InkMaxTransparencyValue: i32 = 255i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const InkMinTransparencyValue: i32 = 0i32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkMode(pub i32);
pub const IEM_Disabled: InkMode = InkMode(0i32);
pub const IEM_Ink: InkMode = InkMode(1i32);
pub const IEM_InkAndGesture: InkMode = InkMode(2i32);
impl ::std::convert::From<i32> for InkMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkMouseButton(pub i32);
pub const IMF_Left: InkMouseButton = InkMouseButton(1i32);
pub const IMF_Right: InkMouseButton = InkMouseButton(2i32);
pub const IMF_Middle: InkMouseButton = InkMouseButton(4i32);
impl ::std::convert::From<i32> for InkMouseButton {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkMouseButton {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkMousePointer {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkMousePointer {
    type Abi = Self;
}
pub const InkOverlay: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1708131910, 52707, 19080, [145, 99, 103, 105, 240, 241, 169, 125]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkOverlayAttachMode(pub i32);
pub const IOAM_Behind: InkOverlayAttachMode = InkOverlayAttachMode(0i32);
pub const IOAM_InFront: InkOverlayAttachMode = InkOverlayAttachMode(1i32);
impl ::std::convert::From<i32> for InkOverlayAttachMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkOverlayAttachMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkOverlayEditingMode(pub i32);
pub const IOEM_Ink: InkOverlayEditingMode = InkOverlayEditingMode(0i32);
pub const IOEM_Delete: InkOverlayEditingMode = InkOverlayEditingMode(1i32);
pub const IOEM_Select: InkOverlayEditingMode = InkOverlayEditingMode(2i32);
impl ::std::convert::From<i32> for InkOverlayEditingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkOverlayEditingMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkOverlayEraserMode(pub i32);
pub const IOERM_StrokeErase: InkOverlayEraserMode = InkOverlayEraserMode(0i32);
pub const IOERM_PointErase: InkOverlayEraserMode = InkOverlayEraserMode(1i32);
impl ::std::convert::From<i32> for InkOverlayEraserMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkOverlayEraserMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPenTip(pub i32);
pub const IPT_Ball: InkPenTip = InkPenTip(0i32);
pub const IPT_Rectangle: InkPenTip = InkPenTip(1i32);
impl ::std::convert::From<i32> for InkPenTip {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkPenTip {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPersistenceCompressionMode(pub i32);
pub const IPCM_Default: InkPersistenceCompressionMode = InkPersistenceCompressionMode(0i32);
pub const IPCM_MaximumCompression: InkPersistenceCompressionMode = InkPersistenceCompressionMode(1i32);
pub const IPCM_NoCompression: InkPersistenceCompressionMode = InkPersistenceCompressionMode(2i32);
impl ::std::convert::From<i32> for InkPersistenceCompressionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkPersistenceCompressionMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPersistenceFormat(pub i32);
pub const IPF_InkSerializedFormat: InkPersistenceFormat = InkPersistenceFormat(0i32);
pub const IPF_Base64InkSerializedFormat: InkPersistenceFormat = InkPersistenceFormat(1i32);
pub const IPF_GIF: InkPersistenceFormat = InkPersistenceFormat(2i32);
pub const IPF_Base64GIF: InkPersistenceFormat = InkPersistenceFormat(3i32);
impl ::std::convert::From<i32> for InkPersistenceFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkPersistenceFormat {
    type Abi = Self;
}
pub const InkPicture: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(77718867, 65078, 20446, [134, 94, 52, 65, 148, 230, 148, 36]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPictureSizeMode(pub i32);
pub const IPSM_AutoSize: InkPictureSizeMode = InkPictureSizeMode(0i32);
pub const IPSM_CenterImage: InkPictureSizeMode = InkPictureSizeMode(1i32);
pub const IPSM_Normal: InkPictureSizeMode = InkPictureSizeMode(2i32);
pub const IPSM_StretchImage: InkPictureSizeMode = InkPictureSizeMode(3i32);
impl ::std::convert::From<i32> for InkPictureSizeMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkPictureSizeMode {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkRasterOperation {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkRasterOperation {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
pub struct InkRecoGuide {
    pub rectWritingBox: super::super::Foundation::RECT,
    pub rectDrawnBox: super::super::Foundation::RECT,
    pub cRows: i32,
    pub cColumns: i32,
    pub midline: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl InkRecoGuide {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for InkRecoGuide {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for InkRecoGuide {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("InkRecoGuide").field("rectWritingBox", &self.rectWritingBox).field("rectDrawnBox", &self.rectDrawnBox).field("cRows", &self.cRows).field("cColumns", &self.cColumns).field("midline", &self.midline).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for InkRecoGuide {
    fn eq(&self, other: &Self) -> bool {
        self.rectWritingBox == other.rectWritingBox && self.rectDrawnBox == other.rectDrawnBox && self.cRows == other.cRows && self.cColumns == other.cColumns && self.midline == other.midline
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for InkRecoGuide {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for InkRecoGuide {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkRecognitionAlternatesSelection(pub i32);
pub const IRAS_Start: InkRecognitionAlternatesSelection = InkRecognitionAlternatesSelection(0i32);
pub const IRAS_DefaultCount: InkRecognitionAlternatesSelection = InkRecognitionAlternatesSelection(10i32);
pub const IRAS_All: InkRecognitionAlternatesSelection = InkRecognitionAlternatesSelection(-1i32);
impl ::std::convert::From<i32> for InkRecognitionAlternatesSelection {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkRecognitionAlternatesSelection {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkRecognitionConfidence(pub i32);
pub const IRC_Strong: InkRecognitionConfidence = InkRecognitionConfidence(0i32);
pub const IRC_Intermediate: InkRecognitionConfidence = InkRecognitionConfidence(1i32);
pub const IRC_Poor: InkRecognitionConfidence = InkRecognitionConfidence(2i32);
impl ::std::convert::From<i32> for InkRecognitionConfidence {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkRecognitionConfidence {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkRecognitionModes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkRecognitionModes {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkRecognitionStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkRecognitionStatus {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkRecognizerCapabilities {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkRecognizerCapabilities {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkRecognizerCharacterAutoCompletionMode(pub i32);
pub const IRCACM_Full: InkRecognizerCharacterAutoCompletionMode = InkRecognizerCharacterAutoCompletionMode(0i32);
pub const IRCACM_Prefix: InkRecognizerCharacterAutoCompletionMode = InkRecognizerCharacterAutoCompletionMode(1i32);
pub const IRCACM_Random: InkRecognizerCharacterAutoCompletionMode = InkRecognizerCharacterAutoCompletionMode(2i32);
impl ::std::convert::From<i32> for InkRecognizerCharacterAutoCompletionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkRecognizerCharacterAutoCompletionMode {
    type Abi = Self;
}
pub const InkRecognizerContext: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2864998967, 37417, 20416, [140, 206, 68, 151, 86, 155, 244, 209]);
pub const InkRecognizerGuide: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2272319809, 42554, 18033, [163, 117, 40, 85, 161, 142, 186, 115]);
pub const InkRecognizers: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2681530376, 63206, 20069, [152, 211, 170, 57, 5, 76, 18, 85]);
pub const InkRectangle: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1135637286, 43744, 19298, [168, 61, 95, 215, 104, 183, 53, 60]);
pub const InkRenderer: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2619131620, 55275, 20203, [144, 145, 21, 167, 200, 121, 30, 217]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkSelectionConstants(pub i32);
pub const ISC_FirstElement: InkSelectionConstants = InkSelectionConstants(0i32);
pub const ISC_AllElements: InkSelectionConstants = InkSelectionConstants(-1i32);
impl ::std::convert::From<i32> for InkSelectionConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkSelectionConstants {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkShiftKeyModifierFlags(pub i32);
pub const IKM_Shift: InkShiftKeyModifierFlags = InkShiftKeyModifierFlags(1i32);
pub const IKM_Control: InkShiftKeyModifierFlags = InkShiftKeyModifierFlags(2i32);
pub const IKM_Alt: InkShiftKeyModifierFlags = InkShiftKeyModifierFlags(4i32);
impl ::std::convert::From<i32> for InkShiftKeyModifierFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkShiftKeyModifierFlags {
    type Abi = Self;
}
pub const InkStrokes: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1223987644, 9230, 18528, [176, 121, 161, 233, 77, 61, 44, 134]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkSystemGesture {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkSystemGesture {
    type Abi = Self;
}
pub const InkTablets: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1850723090, 20746, 19776, [147, 4, 29, 161, 10, 233, 20, 124]);
pub const InkTransform: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3822442812, 5731, 19064, [161, 167, 34, 55, 93, 254, 186, 238]);
pub const InkWordList: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2649247892, 63263, 17649, [132, 113, 21, 162, 250, 118, 252, 243]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InteractionMode(pub i32);
pub const InteractionMode_InPlace: InteractionMode = InteractionMode(0i32);
pub const InteractionMode_Floating: InteractionMode = InteractionMode(1i32);
pub const InteractionMode_DockedTop: InteractionMode = InteractionMode(2i32);
pub const InteractionMode_DockedBottom: InteractionMode = InteractionMode(3i32);
impl ::std::convert::From<i32> for InteractionMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InteractionMode {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn IsStringSupported<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hrc: Param0, wcstring: u32, pwcstring: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsStringSupported(hrc: HRECOCONTEXT, wcstring: u32, pwcstring: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        IsStringSupported(hrc.into_param().abi(), ::std::mem::transmute(wcstring), pwcstring.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct KEYMODIFIER(pub i32);
pub const KEYMODIFIER_CONTROL: KEYMODIFIER = KEYMODIFIER(1i32);
pub const KEYMODIFIER_MENU: KEYMODIFIER = KEYMODIFIER(2i32);
pub const KEYMODIFIER_SHIFT: KEYMODIFIER = KEYMODIFIER(4i32);
pub const KEYMODIFIER_WIN: KEYMODIFIER = KEYMODIFIER(8i32);
pub const KEYMODIFIER_ALTGR: KEYMODIFIER = KEYMODIFIER(16i32);
pub const KEYMODIFIER_EXT: KEYMODIFIER = KEYMODIFIER(32i32);
impl ::std::convert::From<i32> for KEYMODIFIER {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KEYMODIFIER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
pub struct LATTICE_METRICS {
    pub lsBaseline: LINE_SEGMENT,
    pub iMidlineOffset: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl LATTICE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LATTICE_METRICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LATTICE_METRICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LATTICE_METRICS").field("lsBaseline", &self.lsBaseline).field("iMidlineOffset", &self.iMidlineOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LATTICE_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.lsBaseline == other.lsBaseline && self.iMidlineOffset == other.iMidlineOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LATTICE_METRICS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LATTICE_METRICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LINE_METRICS(pub i32);
pub const LM_BASELINE: LINE_METRICS = LINE_METRICS(0i32);
pub const LM_MIDLINE: LINE_METRICS = LINE_METRICS(1i32);
pub const LM_ASCENDER: LINE_METRICS = LINE_METRICS(2i32);
pub const LM_DESCENDER: LINE_METRICS = LINE_METRICS(3i32);
impl ::std::convert::From<i32> for LINE_METRICS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LINE_METRICS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
pub struct LINE_SEGMENT {
    pub PtA: super::super::Foundation::POINT,
    pub PtB: super::super::Foundation::POINT,
}
#[cfg(feature = "Win32_Foundation")]
impl LINE_SEGMENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for LINE_SEGMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for LINE_SEGMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("LINE_SEGMENT").field("PtA", &self.PtA).field("PtB", &self.PtB).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for LINE_SEGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.PtA == other.PtA && self.PtB == other.PtB
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for LINE_SEGMENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for LINE_SEGMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn LoadCachedAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(clsid: Param0, precoattributes: *mut RECO_ATTRS) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LoadCachedAttributes(clsid: ::windows::runtime::GUID, precoattributes: *mut RECO_ATTRS) -> ::windows::runtime::HRESULT;
        }
        LoadCachedAttributes(clsid.into_param().abi(), ::std::mem::transmute(precoattributes)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for MICUIELEMENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MICUIELEMENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MICUIELEMENTSTATE(pub i32);
pub const MICUIELEMENTSTATE_NORMAL: MICUIELEMENTSTATE = MICUIELEMENTSTATE(1i32);
pub const MICUIELEMENTSTATE_HOT: MICUIELEMENTSTATE = MICUIELEMENTSTATE(2i32);
pub const MICUIELEMENTSTATE_PRESSED: MICUIELEMENTSTATE = MICUIELEMENTSTATE(3i32);
pub const MICUIELEMENTSTATE_DISABLED: MICUIELEMENTSTATE = MICUIELEMENTSTATE(4i32);
impl ::std::convert::From<i32> for MICUIELEMENTSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MICUIELEMENTSTATE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn MakeWordList<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOGNIZER>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hrec: Param0, pbuffer: Param1, phwl: *mut HRECOWORDLIST) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MakeWordList(hrec: HRECOGNIZER, pbuffer: super::super::Foundation::PWSTR, phwl: *mut HRECOWORDLIST) -> ::windows::runtime::HRESULT;
        }
        MakeWordList(hrec.into_param().abi(), pbuffer.into_param().abi(), ::std::mem::transmute(phwl)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MathInputControl: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3311501676, 5336, 16528, [131, 12, 152, 217, 148, 178, 28, 123]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MouseButton(pub i32);
pub const NO_BUTTON: MouseButton = MouseButton(0i32);
pub const LEFT_BUTTON: MouseButton = MouseButton(1i32);
pub const RIGHT_BUTTON: MouseButton = MouseButton(2i32);
pub const MIDDLE_BUTTON: MouseButton = MouseButton(4i32);
impl ::std::convert::From<i32> for MouseButton {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MouseButton {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const NUM_FLICK_DIRECTIONS: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct PACKET_DESCRIPTION {
    pub cbPacketSize: u32,
    pub cPacketProperties: u32,
    pub pPacketProperties: *mut PACKET_PROPERTY,
    pub cButtons: u32,
    pub pguidButtons: *mut ::windows::runtime::GUID,
}
impl PACKET_DESCRIPTION {}
impl ::std::default::Default for PACKET_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PACKET_DESCRIPTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PACKET_DESCRIPTION").field("cbPacketSize", &self.cbPacketSize).field("cPacketProperties", &self.cPacketProperties).field("pPacketProperties", &self.pPacketProperties).field("cButtons", &self.cButtons).field("pguidButtons", &self.pguidButtons).finish()
    }
}
impl ::std::cmp::PartialEq for PACKET_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.cbPacketSize == other.cbPacketSize && self.cPacketProperties == other.cPacketProperties && self.pPacketProperties == other.pPacketProperties && self.cButtons == other.cButtons && self.pguidButtons == other.pguidButtons
    }
}
impl ::std::cmp::Eq for PACKET_DESCRIPTION {}
unsafe impl ::windows::runtime::Abi for PACKET_DESCRIPTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct PACKET_PROPERTY {
    pub guid: ::windows::runtime::GUID,
    pub PropertyMetrics: PROPERTY_METRICS,
}
impl PACKET_PROPERTY {}
impl ::std::default::Default for PACKET_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PACKET_PROPERTY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PACKET_PROPERTY").field("guid", &self.guid).field("PropertyMetrics", &self.PropertyMetrics).finish()
    }
}
impl ::std::cmp::PartialEq for PACKET_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.guid == other.guid && self.PropertyMetrics == other.PropertyMetrics
    }
}
impl ::std::cmp::Eq for PACKET_PROPERTY {}
unsafe impl ::windows::runtime::Abi for PACKET_PROPERTY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct PROPERTY_METRICS {
    pub nLogicalMin: i32,
    pub nLogicalMax: i32,
    pub Units: PROPERTY_UNITS,
    pub fResolution: f32,
}
impl PROPERTY_METRICS {}
impl ::std::default::Default for PROPERTY_METRICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PROPERTY_METRICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PROPERTY_METRICS").field("nLogicalMin", &self.nLogicalMin).field("nLogicalMax", &self.nLogicalMax).field("Units", &self.Units).field("fResolution", &self.fResolution).finish()
    }
}
impl ::std::cmp::PartialEq for PROPERTY_METRICS {
    fn eq(&self, other: &Self) -> bool {
        self.nLogicalMin == other.nLogicalMin && self.nLogicalMax == other.nLogicalMax && self.Units == other.Units && self.fResolution == other.fResolution
    }
}
impl ::std::cmp::Eq for PROPERTY_METRICS {}
unsafe impl ::windows::runtime::Abi for PROPERTY_METRICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for PROPERTY_UNITS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROPERTY_UNITS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PanelInputArea(pub i32);
pub const PanelInputArea_Auto: PanelInputArea = PanelInputArea(0i32);
pub const PanelInputArea_Keyboard: PanelInputArea = PanelInputArea(1i32);
pub const PanelInputArea_WritingPad: PanelInputArea = PanelInputArea(2i32);
pub const PanelInputArea_CharacterPad: PanelInputArea = PanelInputArea(3i32);
impl ::std::convert::From<i32> for PanelInputArea {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PanelInputArea {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PanelType(pub i32);
pub const PT_Default: PanelType = PanelType(0i32);
pub const PT_Inactive: PanelType = PanelType(1i32);
pub const PT_Handwriting: PanelType = PanelType(2i32);
pub const PT_Keyboard: PanelType = PanelType(3i32);
impl ::std::convert::From<i32> for PanelType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PanelType {
    type Abi = Self;
}
pub const PenInputPanel: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4148487318, 7002, 18590, [129, 220, 251, 215, 172, 98, 152, 168]);
pub const PenInputPanel_Internal: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2150309817, 1387, 18208, [176, 204, 128, 210, 59, 113, 23, 30]);
pub type PfnRecoCallback = unsafe extern "system" fn(param0: u32, param1: *mut u8, param2: HRECOCONTEXT) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn Process<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pbpartialprocessing: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Process(hrc: HRECOCONTEXT, pbpartialprocessing: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        Process(hrc.into_param().abi(), ::std::mem::transmute(pbpartialprocessing)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct RECO_ATTRS {
    pub dwRecoCapabilityFlags: u32,
    pub awcVendorName: [u16; 32],
    pub awcFriendlyName: [u16; 64],
    pub awLanguageId: [u16; 64],
}
impl RECO_ATTRS {}
impl ::std::default::Default for RECO_ATTRS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RECO_ATTRS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RECO_ATTRS").field("dwRecoCapabilityFlags", &self.dwRecoCapabilityFlags).field("awcVendorName", &self.awcVendorName).field("awcFriendlyName", &self.awcFriendlyName).field("awLanguageId", &self.awLanguageId).finish()
    }
}
impl ::std::cmp::PartialEq for RECO_ATTRS {
    fn eq(&self, other: &Self) -> bool {
        self.dwRecoCapabilityFlags == other.dwRecoCapabilityFlags && self.awcVendorName == other.awcVendorName && self.awcFriendlyName == other.awcFriendlyName && self.awLanguageId == other.awLanguageId
    }
}
impl ::std::cmp::Eq for RECO_ATTRS {}
unsafe impl ::windows::runtime::Abi for RECO_ATTRS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
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
impl RECO_GUIDE {}
impl ::std::default::Default for RECO_GUIDE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RECO_GUIDE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RECO_GUIDE")
            .field("xOrigin", &self.xOrigin)
            .field("yOrigin", &self.yOrigin)
            .field("cxBox", &self.cxBox)
            .field("cyBox", &self.cyBox)
            .field("cxBase", &self.cxBase)
            .field("cyBase", &self.cyBase)
            .field("cHorzBox", &self.cHorzBox)
            .field("cVertBox", &self.cVertBox)
            .field("cyMid", &self.cyMid)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RECO_GUIDE {
    fn eq(&self, other: &Self) -> bool {
        self.xOrigin == other.xOrigin && self.yOrigin == other.yOrigin && self.cxBox == other.cxBox && self.cyBox == other.cyBox && self.cxBase == other.cxBase && self.cyBase == other.cyBase && self.cHorzBox == other.cHorzBox && self.cVertBox == other.cVertBox && self.cyMid == other.cyMid
    }
}
impl ::std::cmp::Eq for RECO_GUIDE {}
unsafe impl ::windows::runtime::Abi for RECO_GUIDE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct RECO_LATTICE {
    pub ulColumnCount: u32,
    pub pLatticeColumns: *mut RECO_LATTICE_COLUMN,
    pub ulPropertyCount: u32,
    pub pGuidProperties: *mut ::windows::runtime::GUID,
    pub ulBestResultColumnCount: u32,
    pub pulBestResultColumns: *mut u32,
    pub pulBestResultIndexes: *mut u32,
}
impl RECO_LATTICE {}
impl ::std::default::Default for RECO_LATTICE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RECO_LATTICE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RECO_LATTICE")
            .field("ulColumnCount", &self.ulColumnCount)
            .field("pLatticeColumns", &self.pLatticeColumns)
            .field("ulPropertyCount", &self.ulPropertyCount)
            .field("pGuidProperties", &self.pGuidProperties)
            .field("ulBestResultColumnCount", &self.ulBestResultColumnCount)
            .field("pulBestResultColumns", &self.pulBestResultColumns)
            .field("pulBestResultIndexes", &self.pulBestResultIndexes)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RECO_LATTICE {
    fn eq(&self, other: &Self) -> bool {
        self.ulColumnCount == other.ulColumnCount && self.pLatticeColumns == other.pLatticeColumns && self.ulPropertyCount == other.ulPropertyCount && self.pGuidProperties == other.pGuidProperties && self.ulBestResultColumnCount == other.ulBestResultColumnCount && self.pulBestResultColumns == other.pulBestResultColumns && self.pulBestResultIndexes == other.pulBestResultIndexes
    }
}
impl ::std::cmp::Eq for RECO_LATTICE {}
unsafe impl ::windows::runtime::Abi for RECO_LATTICE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct RECO_LATTICE_COLUMN {
    pub key: u32,
    pub cpProp: RECO_LATTICE_PROPERTIES,
    pub cStrokes: u32,
    pub pStrokes: *mut u32,
    pub cLatticeElements: u32,
    pub pLatticeElements: *mut RECO_LATTICE_ELEMENT,
}
impl RECO_LATTICE_COLUMN {}
impl ::std::default::Default for RECO_LATTICE_COLUMN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RECO_LATTICE_COLUMN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RECO_LATTICE_COLUMN").field("key", &self.key).field("cpProp", &self.cpProp).field("cStrokes", &self.cStrokes).field("pStrokes", &self.pStrokes).field("cLatticeElements", &self.cLatticeElements).field("pLatticeElements", &self.pLatticeElements).finish()
    }
}
impl ::std::cmp::PartialEq for RECO_LATTICE_COLUMN {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.cpProp == other.cpProp && self.cStrokes == other.cStrokes && self.pStrokes == other.pStrokes && self.cLatticeElements == other.cLatticeElements && self.pLatticeElements == other.pLatticeElements
    }
}
impl ::std::cmp::Eq for RECO_LATTICE_COLUMN {}
unsafe impl ::windows::runtime::Abi for RECO_LATTICE_COLUMN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct RECO_LATTICE_ELEMENT {
    pub score: i32,
    pub r#type: u16,
    pub pData: *mut u8,
    pub ulNextColumn: u32,
    pub ulStrokeNumber: u32,
    pub epProp: RECO_LATTICE_PROPERTIES,
}
impl RECO_LATTICE_ELEMENT {}
impl ::std::default::Default for RECO_LATTICE_ELEMENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RECO_LATTICE_ELEMENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RECO_LATTICE_ELEMENT").field("score", &self.score).field("r#type", &self.r#type).field("pData", &self.pData).field("ulNextColumn", &self.ulNextColumn).field("ulStrokeNumber", &self.ulStrokeNumber).field("epProp", &self.epProp).finish()
    }
}
impl ::std::cmp::PartialEq for RECO_LATTICE_ELEMENT {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score && self.r#type == other.r#type && self.pData == other.pData && self.ulNextColumn == other.ulNextColumn && self.ulStrokeNumber == other.ulStrokeNumber && self.epProp == other.epProp
    }
}
impl ::std::cmp::Eq for RECO_LATTICE_ELEMENT {}
unsafe impl ::windows::runtime::Abi for RECO_LATTICE_ELEMENT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct RECO_LATTICE_PROPERTIES {
    pub cProperties: u32,
    pub apProps: *mut *mut RECO_LATTICE_PROPERTY,
}
impl RECO_LATTICE_PROPERTIES {}
impl ::std::default::Default for RECO_LATTICE_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RECO_LATTICE_PROPERTIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RECO_LATTICE_PROPERTIES").field("cProperties", &self.cProperties).field("apProps", &self.apProps).finish()
    }
}
impl ::std::cmp::PartialEq for RECO_LATTICE_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.cProperties == other.cProperties && self.apProps == other.apProps
    }
}
impl ::std::cmp::Eq for RECO_LATTICE_PROPERTIES {}
unsafe impl ::windows::runtime::Abi for RECO_LATTICE_PROPERTIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct RECO_LATTICE_PROPERTY {
    pub guidProperty: ::windows::runtime::GUID,
    pub cbPropertyValue: u16,
    pub pPropertyValue: *mut u8,
}
impl RECO_LATTICE_PROPERTY {}
impl ::std::default::Default for RECO_LATTICE_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RECO_LATTICE_PROPERTY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RECO_LATTICE_PROPERTY").field("guidProperty", &self.guidProperty).field("cbPropertyValue", &self.cbPropertyValue).field("pPropertyValue", &self.pPropertyValue).finish()
    }
}
impl ::std::cmp::PartialEq for RECO_LATTICE_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.guidProperty == other.guidProperty && self.cbPropertyValue == other.cbPropertyValue && self.pPropertyValue == other.pPropertyValue
    }
}
impl ::std::cmp::Eq for RECO_LATTICE_PROPERTY {}
unsafe impl ::windows::runtime::Abi for RECO_LATTICE_PROPERTY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct RECO_RANGE {
    pub iwcBegin: u32,
    pub cCount: u32,
}
impl RECO_RANGE {}
impl ::std::default::Default for RECO_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RECO_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RECO_RANGE").field("iwcBegin", &self.iwcBegin).field("cCount", &self.cCount).finish()
    }
}
impl ::std::cmp::PartialEq for RECO_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.iwcBegin == other.iwcBegin && self.cCount == other.cCount
    }
}
impl ::std::cmp::Eq for RECO_RANGE {}
unsafe impl ::windows::runtime::Abi for RECO_RANGE {
    type Abi = Self;
}
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
pub const RealTimeStylus: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3798677101, 63896, 17358, [131, 111, 203, 109, 144, 68, 50, 176]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for RealTimeStylusDataInterest {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RealTimeStylusDataInterest {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RealTimeStylusLockType(pub i32);
pub const RTSLT_ObjLock: RealTimeStylusLockType = RealTimeStylusLockType(1i32);
pub const RTSLT_SyncEventLock: RealTimeStylusLockType = RealTimeStylusLockType(2i32);
pub const RTSLT_AsyncEventLock: RealTimeStylusLockType = RealTimeStylusLockType(4i32);
pub const RTSLT_ExcludeCallback: RealTimeStylusLockType = RealTimeStylusLockType(8i32);
pub const RTSLT_SyncObjLock: RealTimeStylusLockType = RealTimeStylusLockType(11i32);
pub const RTSLT_AsyncObjLock: RealTimeStylusLockType = RealTimeStylusLockType(13i32);
impl ::std::convert::From<i32> for RealTimeStylusLockType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RealTimeStylusLockType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub const SAFE_PARTIAL: u32 = 1u32;
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SCROLLDIRECTION(pub i32);
pub const SCROLLDIRECTION_UP: SCROLLDIRECTION = SCROLLDIRECTION(0i32);
pub const SCROLLDIRECTION_DOWN: SCROLLDIRECTION = SCROLLDIRECTION(1i32);
impl ::std::convert::From<i32> for SCROLLDIRECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SCROLLDIRECTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct STROKE_RANGE {
    pub iStrokeBegin: u32,
    pub iStrokeEnd: u32,
}
impl STROKE_RANGE {}
impl ::std::default::Default for STROKE_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for STROKE_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("STROKE_RANGE").field("iStrokeBegin", &self.iStrokeBegin).field("iStrokeEnd", &self.iStrokeEnd).finish()
    }
}
impl ::std::cmp::PartialEq for STROKE_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.iStrokeBegin == other.iStrokeBegin && self.iStrokeEnd == other.iStrokeEnd
    }
}
impl ::std::cmp::Eq for STROKE_RANGE {}
unsafe impl ::windows::runtime::Abi for STROKE_RANGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
pub struct SYSTEM_EVENT_DATA {
    pub bModifier: u8,
    pub wKey: u16,
    pub xPos: i32,
    pub yPos: i32,
    pub bCursorMode: u8,
    pub dwButtonState: u32,
}
impl SYSTEM_EVENT_DATA {}
impl ::std::default::Default for SYSTEM_EVENT_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYSTEM_EVENT_DATA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYSTEM_EVENT_DATA").field("bModifier", &self.bModifier).field("wKey", &self.wKey).field("xPos", &self.xPos).field("yPos", &self.yPos).field("bCursorMode", &self.bCursorMode).field("dwButtonState", &self.dwButtonState).finish()
    }
}
impl ::std::cmp::PartialEq for SYSTEM_EVENT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.bModifier == other.bModifier && self.wKey == other.wKey && self.xPos == other.xPos && self.yPos == other.yPos && self.bCursorMode == other.bCursorMode && self.dwButtonState == other.dwButtonState
    }
}
impl ::std::cmp::Eq for SYSTEM_EVENT_DATA {}
unsafe impl ::windows::runtime::Abi for SYSTEM_EVENT_DATA {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ScrollBarsConstants(pub i32);
pub const rtfNone: ScrollBarsConstants = ScrollBarsConstants(0i32);
pub const rtfHorizontal: ScrollBarsConstants = ScrollBarsConstants(1i32);
pub const rtfVertical: ScrollBarsConstants = ScrollBarsConstants(2i32);
pub const rtfBoth: ScrollBarsConstants = ScrollBarsConstants(3i32);
impl ::std::convert::From<i32> for ScrollBarsConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ScrollBarsConstants {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SelAlignmentConstants(pub i32);
pub const rtfLeft: SelAlignmentConstants = SelAlignmentConstants(0i32);
pub const rtfRight: SelAlignmentConstants = SelAlignmentConstants(1i32);
pub const rtfCenter: SelAlignmentConstants = SelAlignmentConstants(2i32);
impl ::std::convert::From<i32> for SelAlignmentConstants {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SelAlignmentConstants {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for SelectionHitResult {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SelectionHitResult {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn SetEnabledUnicodeRanges<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, cranges: u32, pcr: *mut CHARACTER_RANGE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEnabledUnicodeRanges(hrc: HRECOCONTEXT, cranges: u32, pcr: *mut CHARACTER_RANGE) -> ::windows::runtime::HRESULT;
        }
        SetEnabledUnicodeRanges(hrc.into_param().abi(), ::std::mem::transmute(cranges), ::std::mem::transmute(pcr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn SetFactoid<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hrc: Param0, cwcfactoid: u32, pwcfactoid: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFactoid(hrc: HRECOCONTEXT, cwcfactoid: u32, pwcfactoid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        SetFactoid(hrc.into_param().abi(), ::std::mem::transmute(cwcfactoid), pwcfactoid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn SetFlags<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetFlags(hrc: HRECOCONTEXT, dwflags: u32) -> ::windows::runtime::HRESULT;
        }
        SetFlags(hrc.into_param().abi(), ::std::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn SetGuide<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>>(hrc: Param0, pguide: *const RECO_GUIDE, iindex: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetGuide(hrc: HRECOCONTEXT, pguide: *const RECO_GUIDE, iindex: u32) -> ::windows::runtime::HRESULT;
        }
        SetGuide(hrc.into_param().abi(), ::std::mem::transmute(pguide), ::std::mem::transmute(iindex)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn SetTextContext<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hrc: Param0, cwcbefore: u32, pwcbefore: Param2, cwcafter: u32, pwcafter: Param4) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTextContext(hrc: HRECOCONTEXT, cwcbefore: u32, pwcbefore: super::super::Foundation::PWSTR, cwcafter: u32, pwcafter: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        SetTextContext(hrc.into_param().abi(), ::std::mem::transmute(cwcbefore), pwcbefore.into_param().abi(), ::std::mem::transmute(cwcafter), pwcafter.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[inline]
pub unsafe fn SetWordList<'a, Param0: ::windows::runtime::IntoParam<'a, HRECOCONTEXT>, Param1: ::windows::runtime::IntoParam<'a, HRECOWORDLIST>>(hrc: Param0, hwl: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWordList(hrc: HRECOCONTEXT, hwl: HRECOWORDLIST) -> ::windows::runtime::HRESULT;
        }
        SetWordList(hrc.into_param().abi(), hwl.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const SketchInk: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4029223041, 59516, 19975, [151, 218, 160, 160, 55, 97, 229, 134]);
pub const StrokeBuilder: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3893415655, 28241, 19632, [170, 58, 11, 152, 91, 112, 218, 247]);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_UI_TabletPC`, `Win32_Foundation`*"]
pub struct StylusInfo {
    pub tcid: u32,
    pub cid: u32,
    pub bIsInvertedCursor: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl StylusInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for StylusInfo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for StylusInfo {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("StylusInfo").field("tcid", &self.tcid).field("cid", &self.cid).field("bIsInvertedCursor", &self.bIsInvertedCursor).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for StylusInfo {
    fn eq(&self, other: &Self) -> bool {
        self.tcid == other.tcid && self.cid == other.cid && self.bIsInvertedCursor == other.bIsInvertedCursor
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for StylusInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for StylusInfo {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct StylusQueue(pub i32);
pub const SyncStylusQueue: StylusQueue = StylusQueue(1i32);
pub const AsyncStylusQueueImmediate: StylusQueue = StylusQueue(2i32);
pub const AsyncStylusQueue: StylusQueue = StylusQueue(3i32);
impl ::std::convert::From<i32> for StylusQueue {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for StylusQueue {
    type Abi = Self;
}
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
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TabletDeviceKind(pub i32);
pub const TDK_Mouse: TabletDeviceKind = TabletDeviceKind(0i32);
pub const TDK_Pen: TabletDeviceKind = TabletDeviceKind(1i32);
pub const TDK_Touch: TabletDeviceKind = TabletDeviceKind(2i32);
impl ::std::convert::From<i32> for TabletDeviceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TabletDeviceKind {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TabletHardwareCapabilities(pub i32);
pub const THWC_Integrated: TabletHardwareCapabilities = TabletHardwareCapabilities(1i32);
pub const THWC_CursorMustTouch: TabletHardwareCapabilities = TabletHardwareCapabilities(2i32);
pub const THWC_HardProximity: TabletHardwareCapabilities = TabletHardwareCapabilities(4i32);
pub const THWC_CursorsHavePhysicalIds: TabletHardwareCapabilities = TabletHardwareCapabilities(8i32);
impl ::std::convert::From<i32> for TabletHardwareCapabilities {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TabletHardwareCapabilities {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for TabletPropertyMetricUnit {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TabletPropertyMetricUnit {
    type Abi = Self;
}
pub const TextInputPanel: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4189161943, 8843, 20267, [134, 80, 185, 127, 89, 224, 44, 140]);
pub const TipAutoCompleteClient: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2155617900, 7424, 17727, [185, 32, 182, 27, 183, 205, 217, 151]);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VisualState(pub i32);
pub const InPlace: VisualState = VisualState(0i32);
pub const Floating: VisualState = VisualState(1i32);
pub const DockedTop: VisualState = VisualState(2i32);
pub const DockedBottom: VisualState = VisualState(3i32);
pub const Closed: VisualState = VisualState(4i32);
impl ::std::convert::From<i32> for VisualState {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VisualState {
    type Abi = Self;
}
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
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IInkCollectorEvents(pub ::windows::runtime::IUnknown);
impl _IInkCollectorEvents {}
unsafe impl ::windows::runtime::Interface for _IInkCollectorEvents {
    type Vtable = _IInkCollectorEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(296059890, 28973, 20458, [171, 207, 171, 74, 243, 142, 160, 107]);
}
impl ::std::convert::From<_IInkCollectorEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IInkCollectorEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&_IInkCollectorEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IInkCollectorEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IInkCollectorEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IInkCollectorEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IInkCollectorEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IInkCollectorEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IInkCollectorEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IInkCollectorEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IInkCollectorEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IInkCollectorEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IInkCollectorEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IInkEditEvents(pub ::windows::runtime::IUnknown);
impl _IInkEditEvents {}
unsafe impl ::windows::runtime::Interface for _IInkEditEvents {
    type Vtable = _IInkEditEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3820009367, 42798, 18139, [160, 215, 108, 158, 186, 142, 155, 188]);
}
impl ::std::convert::From<_IInkEditEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IInkEditEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&_IInkEditEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IInkEditEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IInkEditEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IInkEditEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IInkEditEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IInkEditEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IInkEditEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IInkEditEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IInkEditEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IInkEditEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IInkEditEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IInkEvents(pub ::windows::runtime::IUnknown);
impl _IInkEvents {}
unsafe impl ::windows::runtime::Interface for _IInkEvents {
    type Vtable = _IInkEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1115363429, 51775, 18330, [131, 169, 15, 66, 15, 42, 0, 115]);
}
impl ::std::convert::From<_IInkEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IInkEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&_IInkEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IInkEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IInkEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IInkEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IInkEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IInkEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IInkEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IInkEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IInkEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IInkEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IInkEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IInkOverlayEvents(pub ::windows::runtime::IUnknown);
impl _IInkOverlayEvents {}
unsafe impl ::windows::runtime::Interface for _IInkOverlayEvents {
    type Vtable = _IInkOverlayEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(823630697, 58723, 18590, [177, 111, 113, 47, 30, 138, 6, 81]);
}
impl ::std::convert::From<_IInkOverlayEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IInkOverlayEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&_IInkOverlayEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IInkOverlayEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IInkOverlayEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IInkOverlayEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IInkOverlayEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IInkOverlayEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IInkOverlayEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IInkOverlayEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IInkOverlayEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IInkOverlayEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IInkOverlayEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IInkPictureEvents(pub ::windows::runtime::IUnknown);
impl _IInkPictureEvents {}
unsafe impl ::windows::runtime::Interface for _IInkPictureEvents {
    type Vtable = _IInkPictureEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1627344878, 8959, 17540, [172, 193, 211, 8, 217, 205, 126, 163]);
}
impl ::std::convert::From<_IInkPictureEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IInkPictureEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&_IInkPictureEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IInkPictureEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IInkPictureEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IInkPictureEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IInkPictureEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IInkPictureEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IInkPictureEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IInkPictureEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IInkPictureEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IInkPictureEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IInkPictureEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IInkRecognitionEvents(pub ::windows::runtime::IUnknown);
impl _IInkRecognitionEvents {}
unsafe impl ::windows::runtime::Interface for _IInkRecognitionEvents {
    type Vtable = _IInkRecognitionEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(398256431, 11809, 18429, [157, 51, 60, 106, 251, 253, 140, 89]);
}
impl ::std::convert::From<_IInkRecognitionEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IInkRecognitionEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&_IInkRecognitionEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IInkRecognitionEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IInkRecognitionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IInkRecognitionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IInkRecognitionEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IInkRecognitionEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IInkRecognitionEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IInkRecognitionEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IInkRecognitionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IInkRecognitionEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IInkRecognitionEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IInkStrokesEvents(pub ::windows::runtime::IUnknown);
impl _IInkStrokesEvents {}
unsafe impl ::windows::runtime::Interface for _IInkStrokesEvents {
    type Vtable = _IInkStrokesEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4080030700, 23845, 17162, [146, 143, 118, 166, 73, 29, 222, 21]);
}
impl ::std::convert::From<_IInkStrokesEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IInkStrokesEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&_IInkStrokesEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IInkStrokesEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IInkStrokesEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IInkStrokesEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IInkStrokesEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IInkStrokesEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IInkStrokesEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IInkStrokesEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IInkStrokesEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IInkStrokesEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IInkStrokesEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IMathInputControlEvents(pub ::windows::runtime::IUnknown);
impl _IMathInputControlEvents {}
unsafe impl ::windows::runtime::Interface for _IMathInputControlEvents {
    type Vtable = _IMathInputControlEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1748186805, 42109, 17240, [150, 249, 135, 90, 71, 42, 231, 10]);
}
impl ::std::convert::From<_IMathInputControlEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IMathInputControlEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&_IMathInputControlEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IMathInputControlEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IMathInputControlEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IMathInputControlEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IMathInputControlEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IMathInputControlEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IMathInputControlEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IMathInputControlEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IMathInputControlEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IMathInputControlEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IMathInputControlEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct _IPenInputPanelEvents(pub ::windows::runtime::IUnknown);
impl _IPenInputPanelEvents {}
unsafe impl ::windows::runtime::Interface for _IPenInputPanelEvents {
    type Vtable = _IPenInputPanelEvents_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3085208026, 14105, 17311, [132, 143, 231, 172, 189, 130, 15, 23]);
}
impl ::std::convert::From<_IPenInputPanelEvents> for ::windows::runtime::IUnknown {
    fn from(value: _IPenInputPanelEvents) -> Self {
        value.0
    }
}
impl ::std::convert::From<&_IPenInputPanelEvents> for ::windows::runtime::IUnknown {
    fn from(value: &_IPenInputPanelEvents) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for _IPenInputPanelEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a _IPenInputPanelEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<_IPenInputPanelEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: _IPenInputPanelEvents) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&_IPenInputPanelEvents> for super::super::System::Ole::Automation::IDispatch {
    fn from(value: &_IPenInputPanelEvents) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for _IPenInputPanelEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch> for &_IPenInputPanelEvents {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::super::System::Ole::Automation::IDispatch>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct _IPenInputPanelEvents_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pctinfo: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dispidmember: i32, riid: *const ::windows::runtime::GUID, lcid: u32, wflags: u16, pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS, pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>, puargerr: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole_Automation")))] usize,
);
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct enumGetCandidateFlags(pub i32);
pub const TCF_ALLOW_RECOGNITION: enumGetCandidateFlags = enumGetCandidateFlags(1i32);
pub const TCF_FORCE_RECOGNITION: enumGetCandidateFlags = enumGetCandidateFlags(2i32);
impl ::std::convert::From<i32> for enumGetCandidateFlags {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for enumGetCandidateFlags {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct enumINKMETRIC_FLAGS(pub i32);
pub const IMF_FONT_SELECTED_IN_HDC: enumINKMETRIC_FLAGS = enumINKMETRIC_FLAGS(1i32);
pub const IMF_ITALIC: enumINKMETRIC_FLAGS = enumINKMETRIC_FLAGS(2i32);
pub const IMF_BOLD: enumINKMETRIC_FLAGS = enumINKMETRIC_FLAGS(4i32);
impl ::std::convert::From<i32> for enumINKMETRIC_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for enumINKMETRIC_FLAGS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_UI_TabletPC`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct enumRECO_TYPE(pub i32);
pub const RECO_TYPE_WSTRING: enumRECO_TYPE = enumRECO_TYPE(0i32);
pub const RECO_TYPE_WCHAR: enumRECO_TYPE = enumRECO_TYPE(1i32);
impl ::std::convert::From<i32> for enumRECO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for enumRECO_TYPE {
    type Abi = Self;
}
