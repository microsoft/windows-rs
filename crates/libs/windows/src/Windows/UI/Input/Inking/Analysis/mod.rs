#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisInkBullet(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalysisInkBullet {
    type Vtable = IInkAnalysisInkBullet_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisInkBullet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisInkBullet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee049368_6110_4136_95f9_ee809fc20030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkBullet_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisInkDrawing(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalysisInkDrawing {
    type Vtable = IInkAnalysisInkDrawing_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisInkDrawing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisInkDrawing {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a85ed1f_1fe4_4e15_898c_8e112377e021);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkDrawing_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub DrawingKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisDrawingKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Center: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Center: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Points: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Points: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisInkWord(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalysisInkWord {
    type Vtable = IInkAnalysisInkWord_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisInkWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisInkWord {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bd228ad_83af_4034_8f3b_f8687dfff436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkWord_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub TextAlternates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    TextAlternates: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisLine(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalysisLine {
    type Vtable = IInkAnalysisLine_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisLine {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa06d048d_2b8d_4754_ad5a_d0871193a956);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisLine_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IndentLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisListItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalysisListItem {
    type Vtable = IInkAnalysisListItem_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisListItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisListItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4e3c23f_c4c3_4c3a_a1a6_9d85547ee586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisListItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct IInkAnalysisNode(::windows::core::IUnknown);
impl IInkAnalysisNode {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IInkAnalysisNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IInkAnalysisNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkAnalysisNode {}
impl ::core::fmt::Debug for IInkAnalysisNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkAnalysisNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IInkAnalysisNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{30831f05-5f64-4a2c-ba37-4f4887879574}");
}
unsafe impl ::windows::core::Interface for IInkAnalysisNode {
    type Vtable = IInkAnalysisNode_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisNode {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30831f05_5f64_4a2c_ba37_4f4887879574);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisNode_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisNodeKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub BoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BoundingRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RotatedBoundingRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RotatedBoundingRect: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Children: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Children: usize,
    pub Parent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetStrokeIds: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetStrokeIds: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisParagraph(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalysisParagraph {
    type Vtable = IInkAnalysisParagraph_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisParagraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisParagraph {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9ad045c_0cd1_4dd4_a68b_eb1f12b3d727);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisParagraph_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalysisResult {
    type Vtable = IInkAnalysisResult_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8948ba79_a243_4aa3_a294_1f98bd0ff580);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkAnalysisStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisRoot(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalysisRoot {
    type Vtable = IInkAnalysisRoot_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisRoot {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisRoot {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fb6a3c4_2fde_4061_8502_a90f32545b84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisRoot_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub FindNodes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodekind: InkAnalysisNodeKind, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FindNodes: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalysisWritingRegion(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalysisWritingRegion {
    type Vtable = IInkAnalysisWritingRegion_Vtbl;
}
impl ::core::clone::Clone for IInkAnalysisWritingRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalysisWritingRegion {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd6d6231_bd16_4663_b5ae_941d3043ef5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisWritingRegion_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub RecognizedText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkAnalyzer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkAnalyzer {
    type Vtable = IInkAnalyzer_Vtbl;
}
impl ::core::clone::Clone for IInkAnalyzer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalyzer {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf12b8f95_0866_4dc5_8c77_f88614dfe38c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalyzer_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AnalysisRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsAnalyzing: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AddDataForStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stroke: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AddDataForStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AddDataForStrokes: usize,
    pub ClearDataForAllStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveDataForStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RemoveDataForStrokes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeids: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RemoveDataForStrokes: usize,
    pub ReplaceDataForStroke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stroke: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStrokeDataKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AnalyzeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AnalyzeAsync: usize,
}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct IInkAnalyzerFactory(::windows::core::IUnknown);
impl IInkAnalyzerFactory {
    pub fn CreateAnalyzer(&self) -> ::windows::core::Result<InkAnalyzer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalyzer>();
            (::windows::core::Interface::vtable(this).CreateAnalyzer)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows::imp::interface_hierarchy!(IInkAnalyzerFactory, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::cmp::PartialEq for IInkAnalyzerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkAnalyzerFactory {}
impl ::core::fmt::Debug for IInkAnalyzerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkAnalyzerFactory").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for IInkAnalyzerFactory {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{29138686-1963-49d8-9589-e14384c769e3}");
}
unsafe impl ::windows::core::Interface for IInkAnalyzerFactory {
    type Vtable = IInkAnalyzerFactory_Vtbl;
}
impl ::core::clone::Clone for IInkAnalyzerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IInkAnalyzerFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29138686_1963_49d8_9589_e14384c769e3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalyzerFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateAnalyzer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisInkBullet(::windows::core::IUnknown);
impl InkAnalysisInkBullet {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RecognizedText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisInkBullet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisInkBullet {}
impl ::core::fmt::Debug for InkAnalysisInkBullet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisInkBullet").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisInkBullet {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet;{ee049368-6110-4136-95f9-ee809fc20030})");
}
impl ::core::clone::Clone for InkAnalysisInkBullet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisInkBullet {
    type Vtable = IInkAnalysisInkBullet_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisInkBullet {
    const IID: ::windows::core::GUID = <IInkAnalysisInkBullet as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisInkBullet {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet";
}
::windows::imp::interface_hierarchy!(InkAnalysisInkBullet, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IInkAnalysisNode> for InkAnalysisInkBullet {}
unsafe impl ::core::marker::Send for InkAnalysisInkBullet {}
unsafe impl ::core::marker::Sync for InkAnalysisInkBullet {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisInkDrawing(::windows::core::IUnknown);
impl InkAnalysisInkDrawing {
    pub fn DrawingKind(&self) -> ::windows::core::Result<InkAnalysisDrawingKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisDrawingKind>();
            (::windows::core::Interface::vtable(this).DrawingKind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Center(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Point>();
            (::windows::core::Interface::vtable(this).Center)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Points(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).Points)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisInkDrawing {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisInkDrawing {}
impl ::core::fmt::Debug for InkAnalysisInkDrawing {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisInkDrawing").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisInkDrawing {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing;{6a85ed1f-1fe4-4e15-898c-8e112377e021})");
}
impl ::core::clone::Clone for InkAnalysisInkDrawing {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisInkDrawing {
    type Vtable = IInkAnalysisInkDrawing_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisInkDrawing {
    const IID: ::windows::core::GUID = <IInkAnalysisInkDrawing as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisInkDrawing {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing";
}
::windows::imp::interface_hierarchy!(InkAnalysisInkDrawing, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IInkAnalysisNode> for InkAnalysisInkDrawing {}
unsafe impl ::core::marker::Send for InkAnalysisInkDrawing {}
unsafe impl ::core::marker::Sync for InkAnalysisInkDrawing {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisInkWord(::windows::core::IUnknown);
impl InkAnalysisInkWord {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RecognizedText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn TextAlternates(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).TextAlternates)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisInkWord {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisInkWord {}
impl ::core::fmt::Debug for InkAnalysisInkWord {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisInkWord").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisInkWord {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord;{4bd228ad-83af-4034-8f3b-f8687dfff436})");
}
impl ::core::clone::Clone for InkAnalysisInkWord {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisInkWord {
    type Vtable = IInkAnalysisInkWord_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisInkWord {
    const IID: ::windows::core::GUID = <IInkAnalysisInkWord as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisInkWord {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord";
}
::windows::imp::interface_hierarchy!(InkAnalysisInkWord, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IInkAnalysisNode> for InkAnalysisInkWord {}
unsafe impl ::core::marker::Send for InkAnalysisInkWord {}
unsafe impl ::core::marker::Sync for InkAnalysisInkWord {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisLine(::windows::core::IUnknown);
impl InkAnalysisLine {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RecognizedText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IndentLevel(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<i32>();
            (::windows::core::Interface::vtable(this).IndentLevel)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisLine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisLine {}
impl ::core::fmt::Debug for InkAnalysisLine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisLine").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisLine {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisLine;{a06d048d-2b8d-4754-ad5a-d0871193a956})");
}
impl ::core::clone::Clone for InkAnalysisLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisLine {
    type Vtable = IInkAnalysisLine_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisLine {
    const IID: ::windows::core::GUID = <IInkAnalysisLine as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisLine {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisLine";
}
::windows::imp::interface_hierarchy!(InkAnalysisLine, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IInkAnalysisNode> for InkAnalysisLine {}
unsafe impl ::core::marker::Send for InkAnalysisLine {}
unsafe impl ::core::marker::Sync for InkAnalysisLine {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisListItem(::windows::core::IUnknown);
impl InkAnalysisListItem {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RecognizedText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisListItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisListItem {}
impl ::core::fmt::Debug for InkAnalysisListItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisListItem").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisListItem {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisListItem;{b4e3c23f-c4c3-4c3a-a1a6-9d85547ee586})");
}
impl ::core::clone::Clone for InkAnalysisListItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisListItem {
    type Vtable = IInkAnalysisListItem_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisListItem {
    const IID: ::windows::core::GUID = <IInkAnalysisListItem as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisListItem {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisListItem";
}
::windows::imp::interface_hierarchy!(InkAnalysisListItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IInkAnalysisNode> for InkAnalysisListItem {}
unsafe impl ::core::marker::Send for InkAnalysisListItem {}
unsafe impl ::core::marker::Sync for InkAnalysisListItem {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisNode(::windows::core::IUnknown);
impl InkAnalysisNode {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisNode {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisNode {}
impl ::core::fmt::Debug for InkAnalysisNode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisNode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisNode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisNode;{30831f05-5f64-4a2c-ba37-4f4887879574})");
}
impl ::core::clone::Clone for InkAnalysisNode {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisNode {
    type Vtable = IInkAnalysisNode_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisNode {
    const IID: ::windows::core::GUID = <IInkAnalysisNode as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisNode";
}
::windows::imp::interface_hierarchy!(InkAnalysisNode, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IInkAnalysisNode> for InkAnalysisNode {}
unsafe impl ::core::marker::Send for InkAnalysisNode {}
unsafe impl ::core::marker::Sync for InkAnalysisNode {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisParagraph(::windows::core::IUnknown);
impl InkAnalysisParagraph {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RecognizedText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisParagraph {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisParagraph {}
impl ::core::fmt::Debug for InkAnalysisParagraph {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisParagraph").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisParagraph {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph;{d9ad045c-0cd1-4dd4-a68b-eb1f12b3d727})");
}
impl ::core::clone::Clone for InkAnalysisParagraph {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisParagraph {
    type Vtable = IInkAnalysisParagraph_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisParagraph {
    const IID: ::windows::core::GUID = <IInkAnalysisParagraph as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisParagraph {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph";
}
::windows::imp::interface_hierarchy!(InkAnalysisParagraph, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IInkAnalysisNode> for InkAnalysisParagraph {}
unsafe impl ::core::marker::Send for InkAnalysisParagraph {}
unsafe impl ::core::marker::Sync for InkAnalysisParagraph {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisResult(::windows::core::IUnknown);
impl InkAnalysisResult {
    pub fn Status(&self) -> ::windows::core::Result<InkAnalysisStatus> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisStatus>();
            (::windows::core::Interface::vtable(this).Status)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisResult {}
impl ::core::fmt::Debug for InkAnalysisResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisResult;{8948ba79-a243-4aa3-a294-1f98bd0ff580})");
}
impl ::core::clone::Clone for InkAnalysisResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisResult {
    type Vtable = IInkAnalysisResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisResult {
    const IID: ::windows::core::GUID = <IInkAnalysisResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisResult";
}
::windows::imp::interface_hierarchy!(InkAnalysisResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkAnalysisResult {}
unsafe impl ::core::marker::Sync for InkAnalysisResult {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisRoot(::windows::core::IUnknown);
impl InkAnalysisRoot {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RecognizedText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindNodes(&self, nodekind: InkAnalysisNodeKind) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).FindNodes)(::windows::core::Interface::as_raw(this), nodekind, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisRoot {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisRoot {}
impl ::core::fmt::Debug for InkAnalysisRoot {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisRoot").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisRoot {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisRoot;{3fb6a3c4-2fde-4061-8502-a90f32545b84})");
}
impl ::core::clone::Clone for InkAnalysisRoot {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisRoot {
    type Vtable = IInkAnalysisRoot_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisRoot {
    const IID: ::windows::core::GUID = <IInkAnalysisRoot as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisRoot {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisRoot";
}
::windows::imp::interface_hierarchy!(InkAnalysisRoot, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IInkAnalysisNode> for InkAnalysisRoot {}
unsafe impl ::core::marker::Send for InkAnalysisRoot {}
unsafe impl ::core::marker::Sync for InkAnalysisRoot {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalysisWritingRegion(::windows::core::IUnknown);
impl InkAnalysisWritingRegion {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisNodeKind>();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Rect>();
            (::windows::core::Interface::vtable(this).BoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>();
            (::windows::core::Interface::vtable(this).RotatedBoundingRect)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>();
            (::windows::core::Interface::vtable(this).Children)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IInkAnalysisNode>();
            (::windows::core::Interface::vtable(this).Parent)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::ComInterface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).GetStrokeIds)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).RecognizedText)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalysisWritingRegion {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalysisWritingRegion {}
impl ::core::fmt::Debug for InkAnalysisWritingRegion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisWritingRegion").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisWritingRegion {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion;{dd6d6231-bd16-4663-b5ae-941d3043ef5b})");
}
impl ::core::clone::Clone for InkAnalysisWritingRegion {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalysisWritingRegion {
    type Vtable = IInkAnalysisWritingRegion_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalysisWritingRegion {
    const IID: ::windows::core::GUID = <IInkAnalysisWritingRegion as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalysisWritingRegion {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion";
}
::windows::imp::interface_hierarchy!(InkAnalysisWritingRegion, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IInkAnalysisNode> for InkAnalysisWritingRegion {}
unsafe impl ::core::marker::Send for InkAnalysisWritingRegion {}
unsafe impl ::core::marker::Sync for InkAnalysisWritingRegion {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
pub struct InkAnalyzer(::windows::core::IUnknown);
impl InkAnalyzer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<InkAnalyzer, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn AnalysisRoot(&self) -> ::windows::core::Result<InkAnalysisRoot> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<InkAnalysisRoot>();
            (::windows::core::Interface::vtable(this).AnalysisRoot)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsAnalyzing(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsAnalyzing)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AddDataForStroke(&self, stroke: &super::InkStroke) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDataForStroke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(stroke)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddDataForStrokes<P0>(&self, strokes: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::super::Foundation::Collections::IIterable<super::InkStroke>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddDataForStrokes)(::windows::core::Interface::as_raw(this), strokes.try_into_param()?.abi()).ok() }
    }
    pub fn ClearDataForAllStrokes(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ClearDataForAllStrokes)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn RemoveDataForStroke(&self, strokeid: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDataForStroke)(::windows::core::Interface::as_raw(this), strokeid).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveDataForStrokes<P0>(&self, strokeids: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::super::super::Foundation::Collections::IIterable<u32>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDataForStrokes)(::windows::core::Interface::as_raw(this), strokeids.try_into_param()?.abi()).ok() }
    }
    pub fn ReplaceDataForStroke(&self, stroke: &super::InkStroke) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceDataForStroke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(stroke)).ok() }
    }
    pub fn SetStrokeDataKind(&self, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStrokeDataKind)(::windows::core::Interface::as_raw(this), strokeid, strokekind).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AnalyzeAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>>();
            (::windows::core::Interface::vtable(this).AnalyzeAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for InkAnalyzer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkAnalyzer {}
impl ::core::fmt::Debug for InkAnalyzer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalyzer").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalyzer {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalyzer;{f12b8f95-0866-4dc5-8c77-f88614dfe38c})");
}
impl ::core::clone::Clone for InkAnalyzer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for InkAnalyzer {
    type Vtable = IInkAnalyzer_Vtbl;
}
unsafe impl ::windows::core::ComInterface for InkAnalyzer {
    const IID: ::windows::core::GUID = <IInkAnalyzer as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for InkAnalyzer {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalyzer";
}
::windows::imp::interface_hierarchy!(InkAnalyzer, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for InkAnalyzer {}
unsafe impl ::core::marker::Sync for InkAnalyzer {}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkAnalysisDrawingKind(pub i32);
impl InkAnalysisDrawingKind {
    pub const Drawing: Self = Self(0i32);
    pub const Circle: Self = Self(1i32);
    pub const Ellipse: Self = Self(2i32);
    pub const Triangle: Self = Self(3i32);
    pub const IsoscelesTriangle: Self = Self(4i32);
    pub const EquilateralTriangle: Self = Self(5i32);
    pub const RightTriangle: Self = Self(6i32);
    pub const Quadrilateral: Self = Self(7i32);
    pub const Rectangle: Self = Self(8i32);
    pub const Square: Self = Self(9i32);
    pub const Diamond: Self = Self(10i32);
    pub const Trapezoid: Self = Self(11i32);
    pub const Parallelogram: Self = Self(12i32);
    pub const Pentagon: Self = Self(13i32);
    pub const Hexagon: Self = Self(14i32);
}
impl ::core::marker::Copy for InkAnalysisDrawingKind {}
impl ::core::clone::Clone for InkAnalysisDrawingKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkAnalysisDrawingKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for InkAnalysisDrawingKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for InkAnalysisDrawingKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisDrawingKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisDrawingKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisDrawingKind;i4)");
}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkAnalysisNodeKind(pub i32);
impl InkAnalysisNodeKind {
    pub const UnclassifiedInk: Self = Self(0i32);
    pub const Root: Self = Self(1i32);
    pub const WritingRegion: Self = Self(2i32);
    pub const Paragraph: Self = Self(3i32);
    pub const Line: Self = Self(4i32);
    pub const InkWord: Self = Self(5i32);
    pub const InkBullet: Self = Self(6i32);
    pub const InkDrawing: Self = Self(7i32);
    pub const ListItem: Self = Self(8i32);
}
impl ::core::marker::Copy for InkAnalysisNodeKind {}
impl ::core::clone::Clone for InkAnalysisNodeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkAnalysisNodeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for InkAnalysisNodeKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for InkAnalysisNodeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisNodeKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisNodeKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisNodeKind;i4)");
}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkAnalysisStatus(pub i32);
impl InkAnalysisStatus {
    pub const Updated: Self = Self(0i32);
    pub const Unchanged: Self = Self(1i32);
}
impl ::core::marker::Copy for InkAnalysisStatus {}
impl ::core::clone::Clone for InkAnalysisStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkAnalysisStatus {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for InkAnalysisStatus {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for InkAnalysisStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisStatus").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisStatus {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStatus;i4)");
}
#[doc = "*Required features: `\"UI_Input_Inking_Analysis\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct InkAnalysisStrokeKind(pub i32);
impl InkAnalysisStrokeKind {
    pub const Auto: Self = Self(0i32);
    pub const Writing: Self = Self(1i32);
    pub const Drawing: Self = Self(2i32);
}
impl ::core::marker::Copy for InkAnalysisStrokeKind {}
impl ::core::clone::Clone for InkAnalysisStrokeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for InkAnalysisStrokeKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for InkAnalysisStrokeKind {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for InkAnalysisStrokeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkAnalysisStrokeKind").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for InkAnalysisStrokeKind {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStrokeKind;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
