#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalysisInkBullet(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisInkBullet {
    type Vtable = IInkAnalysisInkBullet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee049368_6110_4136_95f9_ee809fc20030);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkBullet_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalysisInkDrawing(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisInkDrawing {
    type Vtable = IInkAnalysisInkDrawing_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a85ed1f_1fe4_4e15_898c_8e112377e021);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkDrawing_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InkAnalysisDrawingKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalysisInkWord(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisInkWord {
    type Vtable = IInkAnalysisInkWord_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bd228ad_83af_4034_8f3b_f8687dfff436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkWord_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalysisLine(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisLine {
    type Vtable = IInkAnalysisLine_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa06d048d_2b8d_4754_ad5a_d0871193a956);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisLine_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalysisListItem(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisListItem {
    type Vtable = IInkAnalysisListItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4e3c23f_c4c3_4c3a_a1a6_9d85547ee586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisListItem_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInkAnalysisNode(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisNode {
    type Vtable = IInkAnalysisNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30831f05_5f64_4a2c_ba37_4f4887879574);
}
impl IInkAnalysisNode {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IInkAnalysisNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{30831f05-5f64-4a2c-ba37-4f4887879574}");
}
impl ::core::convert::From<IInkAnalysisNode> for ::windows::core::IUnknown {
    fn from(value: IInkAnalysisNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInkAnalysisNode> for ::windows::core::IUnknown {
    fn from(value: &IInkAnalysisNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInkAnalysisNode> for ::windows::core::IInspectable {
    fn from(value: IInkAnalysisNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInkAnalysisNode> for ::windows::core::IInspectable {
    fn from(value: &IInkAnalysisNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisNode_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InkAnalysisNodeKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalysisParagraph(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisParagraph {
    type Vtable = IInkAnalysisParagraph_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9ad045c_0cd1_4dd4_a68b_eb1f12b3d727);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisParagraph_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalysisResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisResult {
    type Vtable = IInkAnalysisResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8948ba79_a243_4aa3_a294_1f98bd0ff580);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InkAnalysisStatus) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalysisRoot(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisRoot {
    type Vtable = IInkAnalysisRoot_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fb6a3c4_2fde_4061_8502_a90f32545b84);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisRoot_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nodekind: InkAnalysisNodeKind, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalysisWritingRegion(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalysisWritingRegion {
    type Vtable = IInkAnalysisWritingRegion_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd6d6231_bd16_4663_b5ae_941d3043ef5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisWritingRegion_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkAnalyzer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalyzer {
    type Vtable = IInkAnalyzer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf12b8f95_0866_4dc5_8c77_f88614dfe38c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalyzer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, strokeid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, strokeids: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInkAnalyzerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkAnalyzerFactory {
    type Vtable = IInkAnalyzerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29138686_1963_49d8_9589_e14384c769e3);
}
impl IInkAnalyzerFactory {
    pub fn CreateAnalyzer(&self) -> ::windows::core::Result<InkAnalyzer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalyzer>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IInkAnalyzerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{29138686-1963-49d8-9589-e14384c769e3}");
}
impl ::core::convert::From<IInkAnalyzerFactory> for ::windows::core::IUnknown {
    fn from(value: IInkAnalyzerFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInkAnalyzerFactory> for ::windows::core::IUnknown {
    fn from(value: &IInkAnalyzerFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkAnalyzerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkAnalyzerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInkAnalyzerFactory> for ::windows::core::IInspectable {
    fn from(value: IInkAnalyzerFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInkAnalyzerFactory> for ::windows::core::IInspectable {
    fn from(value: &IInkAnalyzerFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkAnalyzerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInkAnalyzerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalyzerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkAnalysisDrawingKind(pub i32);
impl InkAnalysisDrawingKind {
    pub const Drawing: InkAnalysisDrawingKind = InkAnalysisDrawingKind(0i32);
    pub const Circle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(1i32);
    pub const Ellipse: InkAnalysisDrawingKind = InkAnalysisDrawingKind(2i32);
    pub const Triangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(3i32);
    pub const IsoscelesTriangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(4i32);
    pub const EquilateralTriangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(5i32);
    pub const RightTriangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(6i32);
    pub const Quadrilateral: InkAnalysisDrawingKind = InkAnalysisDrawingKind(7i32);
    pub const Rectangle: InkAnalysisDrawingKind = InkAnalysisDrawingKind(8i32);
    pub const Square: InkAnalysisDrawingKind = InkAnalysisDrawingKind(9i32);
    pub const Diamond: InkAnalysisDrawingKind = InkAnalysisDrawingKind(10i32);
    pub const Trapezoid: InkAnalysisDrawingKind = InkAnalysisDrawingKind(11i32);
    pub const Parallelogram: InkAnalysisDrawingKind = InkAnalysisDrawingKind(12i32);
    pub const Pentagon: InkAnalysisDrawingKind = InkAnalysisDrawingKind(13i32);
    pub const Hexagon: InkAnalysisDrawingKind = InkAnalysisDrawingKind(14i32);
}
impl ::core::convert::From<i32> for InkAnalysisDrawingKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkAnalysisDrawingKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisDrawingKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisDrawingKind;i4)");
}
impl ::windows::core::DefaultType for InkAnalysisDrawingKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisInkBullet(pub ::windows::core::IInspectable);
impl InkAnalysisInkBullet {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisInkBullet {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet;{ee049368-6110-4136-95f9-ee809fc20030})");
}
unsafe impl ::windows::core::Interface for InkAnalysisInkBullet {
    type Vtable = IInkAnalysisInkBullet_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xee049368_6110_4136_95f9_ee809fc20030);
}
impl ::windows::core::RuntimeName for InkAnalysisInkBullet {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet";
}
impl ::core::convert::From<InkAnalysisInkBullet> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisInkBullet) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisInkBullet> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisInkBullet) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisInkBullet> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisInkBullet) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisInkBullet> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisInkBullet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkAnalysisInkBullet> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: InkAnalysisInkBullet) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisInkBullet> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkAnalysisInkBullet) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisInkBullet {}
unsafe impl ::core::marker::Sync for InkAnalysisInkBullet {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisInkDrawing(pub ::windows::core::IInspectable);
impl InkAnalysisInkDrawing {
    pub fn DrawingKind(&self) -> ::windows::core::Result<InkAnalysisDrawingKind> {
        let this = self;
        unsafe {
            let mut result__: InkAnalysisDrawingKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisDrawingKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Center(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn Points(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisInkDrawing {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing;{6a85ed1f-1fe4-4e15-898c-8e112377e021})");
}
unsafe impl ::windows::core::Interface for InkAnalysisInkDrawing {
    type Vtable = IInkAnalysisInkDrawing_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a85ed1f_1fe4_4e15_898c_8e112377e021);
}
impl ::windows::core::RuntimeName for InkAnalysisInkDrawing {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing";
}
impl ::core::convert::From<InkAnalysisInkDrawing> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisInkDrawing) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisInkDrawing> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisInkDrawing) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisInkDrawing> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisInkDrawing) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisInkDrawing> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisInkDrawing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkAnalysisInkDrawing> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: InkAnalysisInkDrawing) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisInkDrawing> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkAnalysisInkDrawing) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisInkDrawing {}
unsafe impl ::core::marker::Sync for InkAnalysisInkDrawing {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisInkWord(pub ::windows::core::IInspectable);
impl InkAnalysisInkWord {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TextAlternates(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisInkWord {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord;{4bd228ad-83af-4034-8f3b-f8687dfff436})");
}
unsafe impl ::windows::core::Interface for InkAnalysisInkWord {
    type Vtable = IInkAnalysisInkWord_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4bd228ad_83af_4034_8f3b_f8687dfff436);
}
impl ::windows::core::RuntimeName for InkAnalysisInkWord {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord";
}
impl ::core::convert::From<InkAnalysisInkWord> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisInkWord) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisInkWord> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisInkWord) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisInkWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisInkWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisInkWord> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisInkWord) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisInkWord> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisInkWord) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisInkWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisInkWord {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkAnalysisInkWord> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: InkAnalysisInkWord) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisInkWord> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkAnalysisInkWord) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisInkWord {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisInkWord {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisInkWord {}
unsafe impl ::core::marker::Sync for InkAnalysisInkWord {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisLine(pub ::windows::core::IInspectable);
impl InkAnalysisLine {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn IndentLevel(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisLine {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisLine;{a06d048d-2b8d-4754-ad5a-d0871193a956})");
}
unsafe impl ::windows::core::Interface for InkAnalysisLine {
    type Vtable = IInkAnalysisLine_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa06d048d_2b8d_4754_ad5a_d0871193a956);
}
impl ::windows::core::RuntimeName for InkAnalysisLine {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisLine";
}
impl ::core::convert::From<InkAnalysisLine> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisLine) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisLine> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisLine) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisLine> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisLine) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisLine> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisLine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisLine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkAnalysisLine> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: InkAnalysisLine) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisLine> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkAnalysisLine) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisLine {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisLine {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisLine {}
unsafe impl ::core::marker::Sync for InkAnalysisLine {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisListItem(pub ::windows::core::IInspectable);
impl InkAnalysisListItem {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisListItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisListItem;{b4e3c23f-c4c3-4c3a-a1a6-9d85547ee586})");
}
unsafe impl ::windows::core::Interface for InkAnalysisListItem {
    type Vtable = IInkAnalysisListItem_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb4e3c23f_c4c3_4c3a_a1a6_9d85547ee586);
}
impl ::windows::core::RuntimeName for InkAnalysisListItem {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisListItem";
}
impl ::core::convert::From<InkAnalysisListItem> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisListItem) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisListItem> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisListItem) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisListItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisListItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisListItem> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisListItem) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisListItem> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisListItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisListItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisListItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkAnalysisListItem> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: InkAnalysisListItem) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisListItem> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkAnalysisListItem) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisListItem {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisListItem {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisListItem {}
unsafe impl ::core::marker::Sync for InkAnalysisListItem {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisNode(pub ::windows::core::IInspectable);
impl InkAnalysisNode {
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisNode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisNode;{30831f05-5f64-4a2c-ba37-4f4887879574})");
}
unsafe impl ::windows::core::Interface for InkAnalysisNode {
    type Vtable = IInkAnalysisNode_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30831f05_5f64_4a2c_ba37_4f4887879574);
}
impl ::windows::core::RuntimeName for InkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisNode";
}
impl ::core::convert::From<InkAnalysisNode> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisNode) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisNode> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisNode) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisNode> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisNode) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisNode> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InkAnalysisNode> for IInkAnalysisNode {
    fn from(value: InkAnalysisNode) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkAnalysisNode> for IInkAnalysisNode {
    fn from(value: &InkAnalysisNode) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisNode {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkAnalysisNode {}
unsafe impl ::core::marker::Sync for InkAnalysisNode {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkAnalysisNodeKind(pub i32);
impl InkAnalysisNodeKind {
    pub const UnclassifiedInk: InkAnalysisNodeKind = InkAnalysisNodeKind(0i32);
    pub const Root: InkAnalysisNodeKind = InkAnalysisNodeKind(1i32);
    pub const WritingRegion: InkAnalysisNodeKind = InkAnalysisNodeKind(2i32);
    pub const Paragraph: InkAnalysisNodeKind = InkAnalysisNodeKind(3i32);
    pub const Line: InkAnalysisNodeKind = InkAnalysisNodeKind(4i32);
    pub const InkWord: InkAnalysisNodeKind = InkAnalysisNodeKind(5i32);
    pub const InkBullet: InkAnalysisNodeKind = InkAnalysisNodeKind(6i32);
    pub const InkDrawing: InkAnalysisNodeKind = InkAnalysisNodeKind(7i32);
    pub const ListItem: InkAnalysisNodeKind = InkAnalysisNodeKind(8i32);
}
impl ::core::convert::From<i32> for InkAnalysisNodeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkAnalysisNodeKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisNodeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisNodeKind;i4)");
}
impl ::windows::core::DefaultType for InkAnalysisNodeKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisParagraph(pub ::windows::core::IInspectable);
impl InkAnalysisParagraph {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisParagraph {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph;{d9ad045c-0cd1-4dd4-a68b-eb1f12b3d727})");
}
unsafe impl ::windows::core::Interface for InkAnalysisParagraph {
    type Vtable = IInkAnalysisParagraph_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd9ad045c_0cd1_4dd4_a68b_eb1f12b3d727);
}
impl ::windows::core::RuntimeName for InkAnalysisParagraph {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph";
}
impl ::core::convert::From<InkAnalysisParagraph> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisParagraph) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisParagraph> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisParagraph) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisParagraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisParagraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisParagraph> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisParagraph) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisParagraph> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisParagraph) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisParagraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisParagraph {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkAnalysisParagraph> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: InkAnalysisParagraph) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisParagraph> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkAnalysisParagraph) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisParagraph {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisParagraph {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisParagraph {}
unsafe impl ::core::marker::Sync for InkAnalysisParagraph {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisResult(pub ::windows::core::IInspectable);
impl InkAnalysisResult {
    pub fn Status(&self) -> ::windows::core::Result<InkAnalysisStatus> {
        let this = self;
        unsafe {
            let mut result__: InkAnalysisStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisStatus>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisResult;{8948ba79-a243-4aa3-a294-1f98bd0ff580})");
}
unsafe impl ::windows::core::Interface for InkAnalysisResult {
    type Vtable = IInkAnalysisResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8948ba79_a243_4aa3_a294_1f98bd0ff580);
}
impl ::windows::core::RuntimeName for InkAnalysisResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisResult";
}
impl ::core::convert::From<InkAnalysisResult> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisResult> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisResult> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisResult> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisResult {}
unsafe impl ::core::marker::Sync for InkAnalysisResult {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisRoot(pub ::windows::core::IInspectable);
impl InkAnalysisRoot {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindNodes(&self, nodekind: InkAnalysisNodeKind) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), nodekind, &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisRoot {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisRoot;{3fb6a3c4-2fde-4061-8502-a90f32545b84})");
}
unsafe impl ::windows::core::Interface for InkAnalysisRoot {
    type Vtable = IInkAnalysisRoot_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fb6a3c4_2fde_4061_8502_a90f32545b84);
}
impl ::windows::core::RuntimeName for InkAnalysisRoot {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisRoot";
}
impl ::core::convert::From<InkAnalysisRoot> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisRoot) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisRoot> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisRoot) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisRoot> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisRoot) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisRoot> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisRoot) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisRoot {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkAnalysisRoot> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: InkAnalysisRoot) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisRoot> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkAnalysisRoot) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisRoot {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisRoot {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisRoot {}
unsafe impl ::core::marker::Sync for InkAnalysisRoot {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkAnalysisStatus(pub i32);
impl InkAnalysisStatus {
    pub const Updated: InkAnalysisStatus = InkAnalysisStatus(0i32);
    pub const Unchanged: InkAnalysisStatus = InkAnalysisStatus(1i32);
}
impl ::core::convert::From<i32> for InkAnalysisStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkAnalysisStatus {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStatus;i4)");
}
impl ::windows::core::DefaultType for InkAnalysisStatus {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkAnalysisStrokeKind(pub i32);
impl InkAnalysisStrokeKind {
    pub const Auto: InkAnalysisStrokeKind = InkAnalysisStrokeKind(0i32);
    pub const Writing: InkAnalysisStrokeKind = InkAnalysisStrokeKind(1i32);
    pub const Drawing: InkAnalysisStrokeKind = InkAnalysisStrokeKind(2i32);
}
impl ::core::convert::From<i32> for InkAnalysisStrokeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkAnalysisStrokeKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisStrokeKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStrokeKind;i4)");
}
impl ::windows::core::DefaultType for InkAnalysisStrokeKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalysisWritingRegion(pub ::windows::core::IInspectable);
impl InkAnalysisWritingRegion {
    pub fn RecognizedText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkAnalysisNodeKind> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::core::Result<IInkAnalysisNode> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::core::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::core::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalysisWritingRegion {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion;{dd6d6231-bd16-4663-b5ae-941d3043ef5b})");
}
unsafe impl ::windows::core::Interface for InkAnalysisWritingRegion {
    type Vtable = IInkAnalysisWritingRegion_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd6d6231_bd16_4663_b5ae_941d3043ef5b);
}
impl ::windows::core::RuntimeName for InkAnalysisWritingRegion {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion";
}
impl ::core::convert::From<InkAnalysisWritingRegion> for ::windows::core::IUnknown {
    fn from(value: InkAnalysisWritingRegion) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalysisWritingRegion> for ::windows::core::IUnknown {
    fn from(value: &InkAnalysisWritingRegion) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalysisWritingRegion> for ::windows::core::IInspectable {
    fn from(value: InkAnalysisWritingRegion) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalysisWritingRegion> for ::windows::core::IInspectable {
    fn from(value: &InkAnalysisWritingRegion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkAnalysisWritingRegion> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: InkAnalysisWritingRegion) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkAnalysisWritingRegion> for IInkAnalysisNode {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkAnalysisWritingRegion) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::core::Param<'a, IInkAnalysisNode> {
        ::core::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkAnalysisWritingRegion {}
unsafe impl ::core::marker::Sync for InkAnalysisWritingRegion {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkAnalyzer(pub ::windows::core::IInspectable);
impl InkAnalyzer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkAnalyzer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AnalysisRoot(&self) -> ::windows::core::Result<InkAnalysisRoot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisRoot>(result__)
        }
    }
    pub fn IsAnalyzing(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AddDataForStroke<'a, Param0: ::windows::core::IntoParam<'a, super::InkStroke>>(&self, stroke: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddDataForStrokes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::InkStroke>>>(&self, strokes: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), strokes.into_param().abi()).ok() }
    }
    pub fn ClearDataForAllStrokes(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn RemoveDataForStroke(&self, strokeid: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), strokeid).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveDataForStrokes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<u32>>>(&self, strokeids: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), strokeids.into_param().abi()).ok() }
    }
    pub fn ReplaceDataForStroke<'a, Param0: ::windows::core::IntoParam<'a, super::InkStroke>>(&self, stroke: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    pub fn SetStrokeDataKind(&self, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), strokeid, strokekind).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AnalyzeAsync(&self) -> ::windows::core::Result<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkAnalyzer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalyzer;{f12b8f95-0866-4dc5-8c77-f88614dfe38c})");
}
unsafe impl ::windows::core::Interface for InkAnalyzer {
    type Vtable = IInkAnalyzer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf12b8f95_0866_4dc5_8c77_f88614dfe38c);
}
impl ::windows::core::RuntimeName for InkAnalyzer {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalyzer";
}
impl ::core::convert::From<InkAnalyzer> for ::windows::core::IUnknown {
    fn from(value: InkAnalyzer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkAnalyzer> for ::windows::core::IUnknown {
    fn from(value: &InkAnalyzer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkAnalyzer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkAnalyzer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkAnalyzer> for ::windows::core::IInspectable {
    fn from(value: InkAnalyzer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkAnalyzer> for ::windows::core::IInspectable {
    fn from(value: &InkAnalyzer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkAnalyzer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkAnalyzer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkAnalyzer {}
unsafe impl ::core::marker::Sync for InkAnalyzer {}
