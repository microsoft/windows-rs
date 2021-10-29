#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalysisInkBullet(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisInkBullet {
    type Vtable = IInkAnalysisInkBullet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3993277288, 24848, 16694, [149, 249, 238, 128, 159, 194, 0, 48]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkBullet_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalysisInkDrawing(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisInkDrawing {
    type Vtable = IInkAnalysisInkDrawing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1787161887, 8164, 19989, [137, 140, 142, 17, 35, 119, 224, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkDrawing_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InkAnalysisDrawingKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalysisInkWord(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisInkWord {
    type Vtable = IInkAnalysisInkWord_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1272064173, 33711, 16436, [143, 59, 248, 104, 125, 255, 244, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisInkWord_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalysisLine(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisLine {
    type Vtable = IInkAnalysisLine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2691499149, 11149, 18260, [173, 90, 208, 135, 17, 147, 169, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisLine_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalysisListItem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisListItem {
    type Vtable = IInkAnalysisListItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3034825279, 50371, 19514, [161, 166, 157, 133, 84, 126, 229, 134]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisListItem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkAnalysisNode(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisNode {
    type Vtable = IInkAnalysisNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(813899525, 24420, 18988, [186, 55, 79, 72, 135, 135, 149, 116]);
}
impl IInkAnalysisNode {
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInkAnalysisNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{30831f05-5f64-4a2c-ba37-4f4887879574}");
}
impl ::std::convert::From<IInkAnalysisNode> for ::windows::runtime::IUnknown {
    fn from(value: IInkAnalysisNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInkAnalysisNode> for ::windows::runtime::IUnknown {
    fn from(value: &IInkAnalysisNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IInkAnalysisNode> for ::windows::runtime::IInspectable {
    fn from(value: IInkAnalysisNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkAnalysisNode> for ::windows::runtime::IInspectable {
    fn from(value: &IInkAnalysisNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisNode_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InkAnalysisNodeKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalysisParagraph(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisParagraph {
    type Vtable = IInkAnalysisParagraph_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3651994716, 3281, 19924, [166, 139, 235, 31, 18, 179, 215, 39]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisParagraph_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalysisResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisResult {
    type Vtable = IInkAnalysisResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2303244921, 41539, 19107, [162, 148, 31, 152, 189, 15, 245, 128]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InkAnalysisStatus) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalysisRoot(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisRoot {
    type Vtable = IInkAnalysisRoot_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1068934084, 12254, 16481, [133, 2, 169, 15, 50, 84, 91, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisRoot_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nodekind: InkAnalysisNodeKind, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalysisWritingRegion(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalysisWritingRegion {
    type Vtable = IInkAnalysisWritingRegion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3714933297, 48406, 18019, [181, 174, 148, 29, 48, 67, 239, 91]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalysisWritingRegion_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkAnalyzer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalyzer {
    type Vtable = IInkAnalyzer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4046163861, 2150, 19909, [140, 119, 248, 134, 20, 223, 227, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalyzer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stroke: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokeid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokeids: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stroke: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IInkAnalyzerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkAnalyzerFactory {
    type Vtable = IInkAnalyzerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(689145478, 6499, 18904, [149, 137, 225, 67, 132, 199, 105, 227]);
}
impl IInkAnalyzerFactory {
    pub fn CreateAnalyzer(&self) -> ::windows::runtime::Result<InkAnalyzer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalyzer>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInkAnalyzerFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{29138686-1963-49d8-9589-e14384c769e3}");
}
impl ::std::convert::From<IInkAnalyzerFactory> for ::windows::runtime::IUnknown {
    fn from(value: IInkAnalyzerFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInkAnalyzerFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IInkAnalyzerFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkAnalyzerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInkAnalyzerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IInkAnalyzerFactory> for ::windows::runtime::IInspectable {
    fn from(value: IInkAnalyzerFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkAnalyzerFactory> for ::windows::runtime::IInspectable {
    fn from(value: &IInkAnalyzerFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInkAnalyzerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInkAnalyzerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkAnalyzerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkAnalysisDrawingKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkAnalysisDrawingKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisDrawingKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisDrawingKind;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisInkBullet(::windows::runtime::IInspectable);
impl InkAnalysisInkBullet {
    pub fn RecognizedText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisInkBullet {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet;{ee049368-6110-4136-95f9-ee809fc20030})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisInkBullet {
    type Vtable = IInkAnalysisInkBullet_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3993277288, 24848, 16694, [149, 249, 238, 128, 159, 194, 0, 48]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisInkBullet {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkBullet";
}
impl ::std::convert::From<InkAnalysisInkBullet> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisInkBullet) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisInkBullet> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisInkBullet) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisInkBullet> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisInkBullet) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisInkBullet> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisInkBullet) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkAnalysisInkBullet> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkAnalysisInkBullet) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkAnalysisInkBullet> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkAnalysisInkBullet) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisInkBullet {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::std::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkAnalysisInkBullet {}
unsafe impl ::std::marker::Sync for InkAnalysisInkBullet {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisInkDrawing(::windows::runtime::IInspectable);
impl InkAnalysisInkDrawing {
    pub fn DrawingKind(&self) -> ::windows::runtime::Result<InkAnalysisDrawingKind> {
        let this = self;
        unsafe {
            let mut result__: InkAnalysisDrawingKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisDrawingKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Center(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn Points(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisInkDrawing {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing;{6a85ed1f-1fe4-4e15-898c-8e112377e021})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisInkDrawing {
    type Vtable = IInkAnalysisInkDrawing_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1787161887, 8164, 19989, [137, 140, 142, 17, 35, 119, 224, 33]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisInkDrawing {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkDrawing";
}
impl ::std::convert::From<InkAnalysisInkDrawing> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisInkDrawing) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisInkDrawing> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisInkDrawing) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisInkDrawing> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisInkDrawing) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisInkDrawing> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisInkDrawing) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkAnalysisInkDrawing> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkAnalysisInkDrawing) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkAnalysisInkDrawing> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkAnalysisInkDrawing) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisInkDrawing {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::std::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkAnalysisInkDrawing {}
unsafe impl ::std::marker::Sync for InkAnalysisInkDrawing {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisInkWord(::windows::runtime::IInspectable);
impl InkAnalysisInkWord {
    pub fn RecognizedText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn TextAlternates(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisInkWord {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord;{4bd228ad-83af-4034-8f3b-f8687dfff436})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisInkWord {
    type Vtable = IInkAnalysisInkWord_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1272064173, 33711, 16436, [143, 59, 248, 104, 125, 255, 244, 54]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisInkWord {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisInkWord";
}
impl ::std::convert::From<InkAnalysisInkWord> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisInkWord) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisInkWord> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisInkWord) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisInkWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisInkWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisInkWord> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisInkWord) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisInkWord> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisInkWord) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisInkWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisInkWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkAnalysisInkWord> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkAnalysisInkWord) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkAnalysisInkWord> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkAnalysisInkWord) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for InkAnalysisInkWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisInkWord {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::std::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkAnalysisInkWord {}
unsafe impl ::std::marker::Sync for InkAnalysisInkWord {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisLine(::windows::runtime::IInspectable);
impl InkAnalysisLine {
    pub fn RecognizedText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn IndentLevel(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisLine {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisLine;{a06d048d-2b8d-4754-ad5a-d0871193a956})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisLine {
    type Vtable = IInkAnalysisLine_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2691499149, 11149, 18260, [173, 90, 208, 135, 17, 147, 169, 86]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisLine {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisLine";
}
impl ::std::convert::From<InkAnalysisLine> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisLine) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisLine> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisLine) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisLine> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisLine) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisLine> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisLine) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkAnalysisLine> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkAnalysisLine) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkAnalysisLine> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkAnalysisLine) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for InkAnalysisLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisLine {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::std::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkAnalysisLine {}
unsafe impl ::std::marker::Sync for InkAnalysisLine {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisListItem(::windows::runtime::IInspectable);
impl InkAnalysisListItem {
    pub fn RecognizedText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisListItem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisListItem;{b4e3c23f-c4c3-4c3a-a1a6-9d85547ee586})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisListItem {
    type Vtable = IInkAnalysisListItem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3034825279, 50371, 19514, [161, 166, 157, 133, 84, 126, 229, 134]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisListItem {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisListItem";
}
impl ::std::convert::From<InkAnalysisListItem> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisListItem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisListItem> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisListItem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisListItem> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisListItem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisListItem> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisListItem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkAnalysisListItem> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkAnalysisListItem) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkAnalysisListItem> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkAnalysisListItem) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for InkAnalysisListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisListItem {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::std::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkAnalysisListItem {}
unsafe impl ::std::marker::Sync for InkAnalysisListItem {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisNode(::windows::runtime::IInspectable);
impl InkAnalysisNode {
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = self;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisNode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisNode;{30831f05-5f64-4a2c-ba37-4f4887879574})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisNode {
    type Vtable = IInkAnalysisNode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(813899525, 24420, 18988, [186, 55, 79, 72, 135, 135, 149, 116]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisNode {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisNode";
}
impl ::std::convert::From<InkAnalysisNode> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisNode> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisNode> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisNode) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisNode> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisNode) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<InkAnalysisNode> for IInkAnalysisNode {
    fn from(value: InkAnalysisNode) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisNode> for IInkAnalysisNode {
    fn from(value: &InkAnalysisNode) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for InkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInkAnalysisNode>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisNode {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInkAnalysisNode>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for InkAnalysisNode {}
unsafe impl ::std::marker::Sync for InkAnalysisNode {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for InkAnalysisNodeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkAnalysisNodeKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisNodeKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisNodeKind;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisParagraph(::windows::runtime::IInspectable);
impl InkAnalysisParagraph {
    pub fn RecognizedText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisParagraph {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph;{d9ad045c-0cd1-4dd4-a68b-eb1f12b3d727})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisParagraph {
    type Vtable = IInkAnalysisParagraph_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3651994716, 3281, 19924, [166, 139, 235, 31, 18, 179, 215, 39]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisParagraph {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisParagraph";
}
impl ::std::convert::From<InkAnalysisParagraph> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisParagraph) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisParagraph> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisParagraph) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisParagraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisParagraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisParagraph> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisParagraph) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisParagraph> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisParagraph) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisParagraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisParagraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkAnalysisParagraph> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkAnalysisParagraph) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkAnalysisParagraph> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkAnalysisParagraph) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for InkAnalysisParagraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisParagraph {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::std::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkAnalysisParagraph {}
unsafe impl ::std::marker::Sync for InkAnalysisParagraph {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisResult(::windows::runtime::IInspectable);
impl InkAnalysisResult {
    pub fn Status(&self) -> ::windows::runtime::Result<InkAnalysisStatus> {
        let this = self;
        unsafe {
            let mut result__: InkAnalysisStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisStatus>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisResult;{8948ba79-a243-4aa3-a294-1f98bd0ff580})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisResult {
    type Vtable = IInkAnalysisResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2303244921, 41539, 19107, [162, 148, 31, 152, 189, 15, 245, 128]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisResult";
}
impl ::std::convert::From<InkAnalysisResult> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisResult> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisResult> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisResult> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkAnalysisResult {}
unsafe impl ::std::marker::Sync for InkAnalysisResult {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisRoot(::windows::runtime::IInspectable);
impl InkAnalysisRoot {
    pub fn RecognizedText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn FindNodes(&self, nodekind: InkAnalysisNodeKind) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), nodekind, &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisRoot {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisRoot;{3fb6a3c4-2fde-4061-8502-a90f32545b84})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisRoot {
    type Vtable = IInkAnalysisRoot_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1068934084, 12254, 16481, [133, 2, 169, 15, 50, 84, 91, 132]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisRoot {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisRoot";
}
impl ::std::convert::From<InkAnalysisRoot> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisRoot) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisRoot> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisRoot) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisRoot> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisRoot) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisRoot> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisRoot) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkAnalysisRoot> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkAnalysisRoot) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkAnalysisRoot> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkAnalysisRoot) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for InkAnalysisRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisRoot {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::std::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkAnalysisRoot {}
unsafe impl ::std::marker::Sync for InkAnalysisRoot {}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkAnalysisStatus(pub i32);
impl InkAnalysisStatus {
    pub const Updated: InkAnalysisStatus = InkAnalysisStatus(0i32);
    pub const Unchanged: InkAnalysisStatus = InkAnalysisStatus(1i32);
}
impl ::std::convert::From<i32> for InkAnalysisStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkAnalysisStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStatus;i4)");
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkAnalysisStrokeKind(pub i32);
impl InkAnalysisStrokeKind {
    pub const Auto: InkAnalysisStrokeKind = InkAnalysisStrokeKind(0i32);
    pub const Writing: InkAnalysisStrokeKind = InkAnalysisStrokeKind(1i32);
    pub const Drawing: InkAnalysisStrokeKind = InkAnalysisStrokeKind(2i32);
}
impl ::std::convert::From<i32> for InkAnalysisStrokeKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkAnalysisStrokeKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisStrokeKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.Analysis.InkAnalysisStrokeKind;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalysisWritingRegion(::windows::runtime::IInspectable);
impl InkAnalysisWritingRegion {
    pub fn RecognizedText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    pub fn Kind(&self) -> ::windows::runtime::Result<InkAnalysisNodeKind> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: InkAnalysisNodeKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisNodeKind>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RotatedBoundingRect(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<super::super::super::super::Foundation::Point>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Children(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<IInkAnalysisNode>>(result__)
        }
    }
    pub fn Parent(&self) -> ::windows::runtime::Result<IInkAnalysisNode> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<IInkAnalysisNode>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokeIds(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::Collections::IVectorView<u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkAnalysisNode>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::Collections::IVectorView<u32>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalysisWritingRegion {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion;{dd6d6231-bd16-4663-b5ae-941d3043ef5b})");
}
unsafe impl ::windows::runtime::Interface for InkAnalysisWritingRegion {
    type Vtable = IInkAnalysisWritingRegion_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3714933297, 48406, 18019, [181, 174, 148, 29, 48, 67, 239, 91]);
}
impl ::windows::runtime::RuntimeName for InkAnalysisWritingRegion {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalysisWritingRegion";
}
impl ::std::convert::From<InkAnalysisWritingRegion> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalysisWritingRegion) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalysisWritingRegion> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalysisWritingRegion) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalysisWritingRegion> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalysisWritingRegion) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalysisWritingRegion> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalysisWritingRegion) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkAnalysisWritingRegion> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkAnalysisWritingRegion) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkAnalysisWritingRegion> for IInkAnalysisNode {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkAnalysisWritingRegion) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkAnalysisNode> for &InkAnalysisWritingRegion {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkAnalysisNode> {
        ::std::convert::TryInto::<IInkAnalysisNode>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkAnalysisWritingRegion {}
unsafe impl ::std::marker::Sync for InkAnalysisWritingRegion {}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkAnalyzer(::windows::runtime::IInspectable);
impl InkAnalyzer {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkAnalyzer, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn AnalysisRoot(&self) -> ::windows::runtime::Result<InkAnalysisRoot> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkAnalysisRoot>(result__)
        }
    }
    pub fn IsAnalyzing(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn AddDataForStroke<'a, Param0: ::windows::runtime::IntoParam<'a, super::InkStroke>>(&self, stroke: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddDataForStrokes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<super::InkStroke>>>(&self, strokes: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), strokes.into_param().abi()).ok() }
    }
    pub fn ClearDataForAllStrokes(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this)).ok() }
    }
    pub fn RemoveDataForStroke(&self, strokeid: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), strokeid).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveDataForStrokes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Collections::IIterable<u32>>>(&self, strokeids: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), strokeids.into_param().abi()).ok() }
    }
    pub fn ReplaceDataForStroke<'a, Param0: ::windows::runtime::IntoParam<'a, super::InkStroke>>(&self, stroke: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    pub fn SetStrokeDataKind(&self, strokeid: u32, strokekind: InkAnalysisStrokeKind) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), strokeid, strokekind).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn AnalyzeAsync(&self) -> ::windows::runtime::Result<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::super::Foundation::IAsyncOperation<InkAnalysisResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkAnalyzer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.Analysis.InkAnalyzer;{f12b8f95-0866-4dc5-8c77-f88614dfe38c})");
}
unsafe impl ::windows::runtime::Interface for InkAnalyzer {
    type Vtable = IInkAnalyzer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4046163861, 2150, 19909, [140, 119, 248, 134, 20, 223, 227, 140]);
}
impl ::windows::runtime::RuntimeName for InkAnalyzer {
    const NAME: &'static str = "Windows.UI.Input.Inking.Analysis.InkAnalyzer";
}
impl ::std::convert::From<InkAnalyzer> for ::windows::runtime::IUnknown {
    fn from(value: InkAnalyzer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkAnalyzer> for ::windows::runtime::IUnknown {
    fn from(value: &InkAnalyzer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkAnalyzer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkAnalyzer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkAnalyzer> for ::windows::runtime::IInspectable {
    fn from(value: InkAnalyzer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkAnalyzer> for ::windows::runtime::IInspectable {
    fn from(value: &InkAnalyzer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkAnalyzer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkAnalyzer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkAnalyzer {}
unsafe impl ::std::marker::Sync for InkAnalyzer {}
