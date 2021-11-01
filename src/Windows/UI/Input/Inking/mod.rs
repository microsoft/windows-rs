#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Input_Inking_Analysis")]
pub mod Analysis;
#[cfg(feature = "UI_Input_Inking_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking_Preview")]
pub mod Preview;
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HandwritingLineHeight(pub i32);
impl HandwritingLineHeight {
    pub const Small: HandwritingLineHeight = HandwritingLineHeight(0i32);
    pub const Medium: HandwritingLineHeight = HandwritingLineHeight(1i32);
    pub const Large: HandwritingLineHeight = HandwritingLineHeight(2i32);
}
impl ::std::convert::From<i32> for HandwritingLineHeight {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HandwritingLineHeight {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for HandwritingLineHeight {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.HandwritingLineHeight;i4)");
}
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkDrawingAttributes(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkDrawingAttributes {
    type Vtable = IInkDrawingAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2543982444, 26484, 18605, [132, 240, 72, 245, 169, 190, 116, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PenTipShape) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PenTipShape) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkDrawingAttributes2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkDrawingAttributes2 {
    type Vtable = IInkDrawingAttributes2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2091607304, 36548, 17149, [165, 165, 228, 183, 209, 213, 49, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkDrawingAttributes3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkDrawingAttributes3 {
    type Vtable = IInkDrawingAttributes3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1912733698, 32091, 18064, [138, 244, 230, 100, 203, 226, 183, 79]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InkDrawingAttributesKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkDrawingAttributes4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkDrawingAttributes4 {
    type Vtable = IInkDrawingAttributes4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4016430117, 40729, 17773, [145, 163, 188, 58, 61, 145, 197, 251]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkDrawingAttributes5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkDrawingAttributes5 {
    type Vtable = IInkDrawingAttributes5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3508183227, 1909, 18514, [174, 100, 65, 20, 58, 122, 230, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes5_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkDrawingAttributesPencilProperties(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkDrawingAttributesPencilProperties {
    type Vtable = IInkDrawingAttributesPencilProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1327838411, 11654, 16827, [176, 232, 228, 194, 160, 37, 60, 82]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributesPencilProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkDrawingAttributesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkDrawingAttributesStatics {
    type Vtable = IInkDrawingAttributesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4147241023, 6757, 18530, [150, 203, 110, 22, 101, 225, 127, 109]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkInputConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkInputConfiguration {
    type Vtable = IInkInputConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2477166020, 2939, 18903, [179, 79, 153, 1, 229, 36, 220, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkInputConfiguration2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkInputConfiguration2 {
    type Vtable = IInkInputConfiguration2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1791108910, 33204, 23748, [163, 109, 208, 87, 195, 135, 223, 218]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkInputProcessingConfiguration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkInputProcessingConfiguration {
    type Vtable = IInkInputProcessingConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(662231134, 13258, 19206, [166, 211, 172, 57, 69, 17, 109, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputProcessingConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InkInputProcessingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InkInputProcessingMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InkInputRightDragAction) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InkInputRightDragAction) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkManager {
    type Vtable = IInkManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1195668349, 26395, 16739, [156, 149, 78, 141, 122, 3, 95, 225]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InkManipulationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InkManipulationMode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerpoint: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerpoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerpoint: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, drawingattributes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkModelerAttributes(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkModelerAttributes {
    type Vtable = IInkModelerAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3134398247, 3289, 19453, [182, 243, 158, 3, 186, 141, 116, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkModelerAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkModelerAttributes2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkModelerAttributes2 {
    type Vtable = IInkModelerAttributes2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2261897370, 20216, 24101, [183, 188, 182, 84, 36, 241, 107, 179]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkModelerAttributes2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPoint(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPoint {
    type Vtable = IInkPoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2676434731, 34188, 18085, [155, 65, 209, 149, 151, 4, 89, 253]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPoint_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPoint2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPoint2 {
    type Vtable = IInkPoint2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4222206967, 44630, 19804, [189, 47, 10, 196, 95, 94, 74, 249]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPoint2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Input_Inking`*"]
pub struct IInkPointFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPointFactory {
    type Vtable = IInkPointFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(702928156, 51599, 16477, [159, 59, 229, 62, 49, 6, 141, 77]);
}
impl IInkPointFactory {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn CreateInkPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0, pressure: f32) -> ::windows::runtime::Result<InkPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), position.into_param().abi(), pressure, &mut result__).from_abi::<InkPoint>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInkPointFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{29e5d51c-c98f-405d-9f3b-e53e31068d4d}");
}
impl ::std::convert::From<IInkPointFactory> for ::windows::runtime::IUnknown {
    fn from(value: IInkPointFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInkPointFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IInkPointFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkPointFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInkPointFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IInkPointFactory> for ::windows::runtime::IInspectable {
    fn from(value: IInkPointFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkPointFactory> for ::windows::runtime::IInspectable {
    fn from(value: &IInkPointFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInkPointFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInkPointFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPointFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: super::super::super::Foundation::Point, pressure: f32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPointFactory2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPointFactory2 {
    type Vtable = IInkPointFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3759431301, 56063, 17906, [173, 105, 5, 13, 130, 86, 162, 9]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPointFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPresenter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPresenter {
    type Vtable = IInkPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2795204834, 34939, 17807, [177, 115, 79, 228, 67, 137, 48, 163]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Core::CoreInputDeviceTypes) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InkPresenterPredefinedConfiguration) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPresenter2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPresenter2 {
    type Vtable = IInkPresenter2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3478382098, 39476, 4582, [159, 51, 162, 79, 192, 217, 100, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InkHighContrastAdjustment) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: InkHighContrastAdjustment) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPresenter3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPresenter3 {
    type Vtable = IInkPresenter3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1373752969, 54141, 19088, [131, 252, 127, 94, 157, 251, 242, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPresenterProtractor(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPresenterProtractor {
    type Vtable = IInkPresenterProtractor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2112090794, 61292, 20113, [167, 59, 91, 112, 213, 111, 189, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterProtractor_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Color) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPresenterProtractorFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPresenterProtractorFactory {
    type Vtable = IInkPresenterProtractorFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(838927305, 26874, 18409, [129, 39, 131, 112, 113, 31, 196, 108]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterProtractorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkpresenter: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPresenterRuler(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPresenterRuler {
    type Vtable = IInkPresenterRuler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1826258266, 57031, 19927, [135, 122, 33, 51, 241, 131, 212, 138]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRuler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: f64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkPresenterRuler2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPresenterRuler2 {
    type Vtable = IInkPresenterRuler2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1158876609, 48225, 17620, [164, 35, 84, 113, 42, 230, 113, 196]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRuler2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Input_Inking`*"]
pub struct IInkPresenterRulerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPresenterRulerFactory {
    type Vtable = IInkPresenterRulerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(875961323, 36865, 19019, [166, 144, 105, 219, 175, 99, 229, 1]);
}
impl IInkPresenterRulerFactory {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, InkPresenter>>(&self, inkpresenter: Param0) -> ::windows::runtime::Result<InkPresenterRuler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<InkPresenterRuler>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInkPresenterRulerFactory {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{34361beb-9001-4a4b-a690-69dbaf63e501}");
}
impl ::std::convert::From<IInkPresenterRulerFactory> for ::windows::runtime::IUnknown {
    fn from(value: IInkPresenterRulerFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInkPresenterRulerFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IInkPresenterRulerFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IInkPresenterRulerFactory> for ::windows::runtime::IInspectable {
    fn from(value: IInkPresenterRulerFactory) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkPresenterRulerFactory> for ::windows::runtime::IInspectable {
    fn from(value: &IInkPresenterRulerFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRulerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkpresenter: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Input_Inking`*"]
pub struct IInkPresenterStencil(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkPresenterStencil {
    type Vtable = IInkPresenterStencil_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(819015021, 15878, 19714, [177, 22, 39, 127, 181, 216, 173, 220]);
}
impl IInkPresenterStencil {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<InkPresenterStencilKind> {
        let this = self;
        unsafe {
            let mut result__: InkPresenterStencilKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn ForegroundColor(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn Transform(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn SetTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInkPresenterStencil {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{30d12d6d-3e06-4d02-b116-277fb5d8addc}");
}
impl ::std::convert::From<IInkPresenterStencil> for ::windows::runtime::IUnknown {
    fn from(value: IInkPresenterStencil) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInkPresenterStencil> for ::windows::runtime::IUnknown {
    fn from(value: &IInkPresenterStencil) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkPresenterStencil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInkPresenterStencil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IInkPresenterStencil> for ::windows::runtime::IInspectable {
    fn from(value: IInkPresenterStencil) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkPresenterStencil> for ::windows::runtime::IInspectable {
    fn from(value: &IInkPresenterStencil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInkPresenterStencil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInkPresenterStencil {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterStencil_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut InkPresenterStencilKind) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::Color) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkRecognitionResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkRecognitionResult {
    type Vtable = IInkRecognitionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(910563988, 20584, 16623, [138, 5, 44, 47, 182, 9, 8, 162]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkRecognizer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkRecognizer {
    type Vtable = IInkRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(125619875, 36941, 17450, [177, 81, 170, 202, 54, 49, 196, 59]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizer_abi(
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
#[doc = "*Required features: `UI_Input_Inking`*"]
pub struct IInkRecognizerContainer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkRecognizerContainer {
    type Vtable = IInkRecognizerContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2806880817, 32839, 18072, [169, 18, 248, 42, 80, 133, 1, 47]);
}
impl IInkRecognizerContainer {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetDefaultRecognizer<'a, Param0: ::windows::runtime::IntoParam<'a, InkRecognizer>>(&self, recognizer: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), recognizer.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Foundation_Collections`*"]
    pub fn RecognizeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, InkStrokeContainer>>(&self, strokecollection: Param0, recognitiontarget: InkRecognitionTarget) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), strokecollection.into_param().abi(), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetRecognizers(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInkRecognizerContainer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a74d9a31-8047-4698-a912-f82a5085012f}");
}
impl ::std::convert::From<IInkRecognizerContainer> for ::windows::runtime::IUnknown {
    fn from(value: IInkRecognizerContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInkRecognizerContainer> for ::windows::runtime::IUnknown {
    fn from(value: &IInkRecognizerContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IInkRecognizerContainer> for ::windows::runtime::IInspectable {
    fn from(value: IInkRecognizerContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkRecognizerContainer> for ::windows::runtime::IInspectable {
    fn from(value: &IInkRecognizerContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognizer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokecollection: ::windows::runtime::RawPtr, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStroke(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStroke {
    type Vtable = IInkStroke_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353652064, 52451, 20431, [157, 82, 17, 81, 138, 182, 175, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStroke2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStroke2 {
    type Vtable = IInkStroke2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1572463860, 47866, 19937, [137, 211, 32, 27, 30, 215, 216, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStroke3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStroke3 {
    type Vtable = IInkStroke3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1249932148, 38041, 16669, [161, 196, 104, 133, 93, 3, 214, 95]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStroke4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStroke4 {
    type Vtable = IInkStroke4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3445318373, 46825, 23441, [165, 119, 25, 33, 210, 52, 134, 144]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke4_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStrokeBuilder(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokeBuilder {
    type Vtable = IInkStrokeBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2193347036, 7267, 16860, [158, 7, 75, 74, 112, 206, 216, 1]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerpoint: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerpoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerpoint: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, points: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, drawingattributes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStrokeBuilder2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokeBuilder2 {
    type Vtable = IInkStrokeBuilder2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3179461671, 29471, 19644, [187, 191, 109, 70, 128, 68, 241, 229]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkpoints: ::windows::runtime::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStrokeBuilder3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokeBuilder3 {
    type Vtable = IInkStrokeBuilder3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2999394253, 21618, 18097, [168, 29, 195, 122, 61, 22, 148, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inkpoints: ::windows::runtime::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: ::windows::runtime::RawPtr, strokeduration: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc = "*Required features: `UI_Input_Inking`*"]
pub struct IInkStrokeContainer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokeContainer {
    type Vtable = IInkStrokeContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(581749702, 64169, 20244, [182, 140, 246, 206, 230, 112, 174, 22]);
}
impl IInkStrokeContainer {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn AddStroke<'a, Param0: ::windows::runtime::IntoParam<'a, InkStroke>>(&self, stroke: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn DeleteSelected(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn MoveSelected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, translation: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), translation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Foundation_Collections`*"]
    pub fn SelectWithPolyLine<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, polyline: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), polyline.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn SelectWithLine<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, from: Param0, to: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), from.into_param().abi(), to.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn CopySelectedToClipboard(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn PasteFromClipboard<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn CanPasteFromClipboard(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Storage_Streams`*"]
    pub fn LoadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Storage_Streams`*"]
    pub fn SaveAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn UpdateRecognitionResults<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(&self, recognitionresults: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), recognitionresults.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetStrokes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetRecognitionResults(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IInkStrokeContainer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{22accbc6-faa9-4f14-b68c-f6cee670ae16}");
}
impl ::std::convert::From<IInkStrokeContainer> for ::windows::runtime::IUnknown {
    fn from(value: IInkStrokeContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IInkStrokeContainer> for ::windows::runtime::IUnknown {
    fn from(value: &IInkStrokeContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IInkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IInkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<IInkStrokeContainer> for ::windows::runtime::IInspectable {
    fn from(value: IInkStrokeContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IInkStrokeContainer> for ::windows::runtime::IInspectable {
    fn from(value: &IInkStrokeContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IInkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IInkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stroke: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, translation: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, polyline: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, position: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, inputstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, recognitionresults: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStrokeContainer2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokeContainer2 {
    type Vtable = IInkStrokeContainer2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2298598244, 55862, 19407, [158, 92, 209, 149, 130, 89, 149, 180]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, strokes: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStrokeContainer3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokeContainer3 {
    type Vtable = IInkStrokeContainer3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1023917733, 47850, 19586, [167, 25, 123, 131, 218, 16, 103, 210]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, outputstream: ::windows::runtime::RawPtr, inkpersistenceformat: InkPersistenceFormat, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStrokeInput(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokeInput {
    type Vtable = IInkStrokeInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3476029051, 24080, 17350, [160, 128, 136, 242, 110, 29, 198, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeInput_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStrokeRenderingSegment(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokeRenderingSegment {
    type Vtable = IInkStrokeRenderingSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1750142751, 35043, 18298, [162, 250, 86, 159, 95, 31, 155, 213]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeRenderingSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut f32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStrokesCollectedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokesCollectedEventArgs {
    type Vtable = IInkStrokesCollectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3304321577, 6456, 18780, [180, 217, 109, 228, 176, 141, 72, 17]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokesCollectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkStrokesErasedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkStrokesErasedEventArgs {
    type Vtable = IInkStrokesErasedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2753653282, 5379, 20159, [143, 245, 45, 232, 69, 132, 168, 170]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokesErasedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkSynchronizer(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkSynchronizer {
    type Vtable = IInkSynchronizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2610864480, 44699, 17913, [132, 7, 75, 73, 59, 22, 54, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkSynchronizer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkUnprocessedInput(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkUnprocessedInput {
    type Vtable = IInkUnprocessedInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3678684640, 33688, 18721, [172, 59, 171, 151, 140, 91, 162, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkUnprocessedInput_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPenAndInkSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPenAndInkSettings {
    type Vtable = IPenAndInkSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3157060495, 102, 17576, [187, 122, 184, 57, 179, 222, 184, 245]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut PenHandedness) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut HandwritingLineHeight) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPenAndInkSettings2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPenAndInkSettings2 {
    type Vtable = IPenAndInkSettings2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(845339219, 8004, 21986, [153, 41, 235, 247, 126, 84, 129, 184]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: PenHandedness) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IPenAndInkSettingsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPenAndInkSettingsStatics {
    type Vtable = IPenAndInkSettingsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3983396918, 22280, 23612, [150, 219, 242, 245, 82, 234, 182, 65]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkDrawingAttributes(::windows::runtime::IInspectable);
impl InkDrawingAttributes {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkDrawingAttributes, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Color(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn PenTip(&self) -> ::windows::runtime::Result<PenTipShape> {
        let this = self;
        unsafe {
            let mut result__: PenTipShape = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PenTipShape>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetPenTip(&self, value: PenTipShape) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn Size(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn SetSize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IgnorePressure(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIgnorePressure(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn FitToCurve(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetFitToCurve(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn PenTipTransform(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::runtime::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn SetPenTipTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn DrawAsHighlighter(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetDrawAsHighlighter(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<InkDrawingAttributesKind> {
        let this = &::windows::runtime::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__: InkDrawingAttributesKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributesKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn PencilProperties(&self) -> ::windows::runtime::Result<InkDrawingAttributesPencilProperties> {
        let this = &::windows::runtime::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributesPencilProperties>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn CreateForPencil() -> ::windows::runtime::Result<InkDrawingAttributes> {
        Self::IInkDrawingAttributesStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributes>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IgnoreTilt(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIgnoreTilt(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn ModelerAttributes(&self) -> ::windows::runtime::Result<InkModelerAttributes> {
        let this = &::windows::runtime::Interface::cast::<IInkDrawingAttributes5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkModelerAttributes>(result__)
        }
    }
    pub fn IInkDrawingAttributesStatics<R, F: FnOnce(&IInkDrawingAttributesStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkDrawingAttributes, IInkDrawingAttributesStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkDrawingAttributes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkDrawingAttributes;{97a2176c-6774-48ad-84f0-48f5a9be74f9})");
}
unsafe impl ::windows::runtime::Interface for InkDrawingAttributes {
    type Vtable = IInkDrawingAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2543982444, 26484, 18605, [132, 240, 72, 245, 169, 190, 116, 249]);
}
impl ::windows::runtime::RuntimeName for InkDrawingAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributes";
}
impl ::std::convert::From<InkDrawingAttributes> for ::windows::runtime::IUnknown {
    fn from(value: InkDrawingAttributes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkDrawingAttributes> for ::windows::runtime::IUnknown {
    fn from(value: &InkDrawingAttributes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkDrawingAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkDrawingAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkDrawingAttributes> for ::windows::runtime::IInspectable {
    fn from(value: InkDrawingAttributes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkDrawingAttributes> for ::windows::runtime::IInspectable {
    fn from(value: &InkDrawingAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkDrawingAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkDrawingAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkDrawingAttributes {}
unsafe impl ::std::marker::Sync for InkDrawingAttributes {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkDrawingAttributesKind(pub i32);
impl InkDrawingAttributesKind {
    pub const Default: InkDrawingAttributesKind = InkDrawingAttributesKind(0i32);
    pub const Pencil: InkDrawingAttributesKind = InkDrawingAttributesKind(1i32);
}
impl ::std::convert::From<i32> for InkDrawingAttributesKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkDrawingAttributesKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkDrawingAttributesKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkDrawingAttributesKind;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkDrawingAttributesPencilProperties(::windows::runtime::IInspectable);
impl InkDrawingAttributesPencilProperties {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Opacity(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetOpacity(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkDrawingAttributesPencilProperties {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties;{4f2534cb-2d86-41bb-b0e8-e4c2a0253c52})");
}
unsafe impl ::windows::runtime::Interface for InkDrawingAttributesPencilProperties {
    type Vtable = IInkDrawingAttributesPencilProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1327838411, 11654, 16827, [176, 232, 228, 194, 160, 37, 60, 82]);
}
impl ::windows::runtime::RuntimeName for InkDrawingAttributesPencilProperties {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties";
}
impl ::std::convert::From<InkDrawingAttributesPencilProperties> for ::windows::runtime::IUnknown {
    fn from(value: InkDrawingAttributesPencilProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkDrawingAttributesPencilProperties> for ::windows::runtime::IUnknown {
    fn from(value: &InkDrawingAttributesPencilProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkDrawingAttributesPencilProperties> for ::windows::runtime::IInspectable {
    fn from(value: InkDrawingAttributesPencilProperties) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkDrawingAttributesPencilProperties> for ::windows::runtime::IInspectable {
    fn from(value: &InkDrawingAttributesPencilProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkDrawingAttributesPencilProperties {}
unsafe impl ::std::marker::Sync for InkDrawingAttributesPencilProperties {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkHighContrastAdjustment(pub i32);
impl InkHighContrastAdjustment {
    pub const UseSystemColorsWhenNecessary: InkHighContrastAdjustment = InkHighContrastAdjustment(0i32);
    pub const UseSystemColors: InkHighContrastAdjustment = InkHighContrastAdjustment(1i32);
    pub const UseOriginalColors: InkHighContrastAdjustment = InkHighContrastAdjustment(2i32);
}
impl ::std::convert::From<i32> for InkHighContrastAdjustment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkHighContrastAdjustment {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkHighContrastAdjustment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkHighContrastAdjustment;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkInputConfiguration(::windows::runtime::IInspectable);
impl InkInputConfiguration {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsPrimaryBarrelButtonInputEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsPrimaryBarrelButtonInputEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsEraserInputEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsEraserInputEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsPenHapticFeedbackEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsPenHapticFeedbackEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkInputConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkInputConfiguration;{93a68dc4-0b7b-49d7-b34f-9901e524dcf2})");
}
unsafe impl ::windows::runtime::Interface for InkInputConfiguration {
    type Vtable = IInkInputConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2477166020, 2939, 18903, [179, 79, 153, 1, 229, 36, 220, 242]);
}
impl ::windows::runtime::RuntimeName for InkInputConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputConfiguration";
}
impl ::std::convert::From<InkInputConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: InkInputConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkInputConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &InkInputConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkInputConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkInputConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkInputConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: InkInputConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkInputConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &InkInputConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkInputConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkInputConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkInputConfiguration {}
unsafe impl ::std::marker::Sync for InkInputConfiguration {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkInputProcessingConfiguration(::windows::runtime::IInspectable);
impl InkInputProcessingConfiguration {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Mode(&self) -> ::windows::runtime::Result<InkInputProcessingMode> {
        let this = self;
        unsafe {
            let mut result__: InkInputProcessingMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkInputProcessingMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetMode(&self, value: InkInputProcessingMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn RightDragAction(&self) -> ::windows::runtime::Result<InkInputRightDragAction> {
        let this = self;
        unsafe {
            let mut result__: InkInputRightDragAction = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkInputRightDragAction>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetRightDragAction(&self, value: InkInputRightDragAction) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkInputProcessingConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkInputProcessingConfiguration;{2778d85e-33ca-4b06-a6d3-ac3945116d37})");
}
unsafe impl ::windows::runtime::Interface for InkInputProcessingConfiguration {
    type Vtable = IInkInputProcessingConfiguration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(662231134, 13258, 19206, [166, 211, 172, 57, 69, 17, 109, 55]);
}
impl ::windows::runtime::RuntimeName for InkInputProcessingConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputProcessingConfiguration";
}
impl ::std::convert::From<InkInputProcessingConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: InkInputProcessingConfiguration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkInputProcessingConfiguration> for ::windows::runtime::IUnknown {
    fn from(value: &InkInputProcessingConfiguration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkInputProcessingConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: InkInputProcessingConfiguration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkInputProcessingConfiguration> for ::windows::runtime::IInspectable {
    fn from(value: &InkInputProcessingConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkInputProcessingConfiguration {}
unsafe impl ::std::marker::Sync for InkInputProcessingConfiguration {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkInputProcessingMode(pub i32);
impl InkInputProcessingMode {
    pub const None: InkInputProcessingMode = InkInputProcessingMode(0i32);
    pub const Inking: InkInputProcessingMode = InkInputProcessingMode(1i32);
    pub const Erasing: InkInputProcessingMode = InkInputProcessingMode(2i32);
}
impl ::std::convert::From<i32> for InkInputProcessingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkInputProcessingMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkInputProcessingMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputProcessingMode;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkInputRightDragAction(pub i32);
impl InkInputRightDragAction {
    pub const LeaveUnprocessed: InkInputRightDragAction = InkInputRightDragAction(0i32);
    pub const AllowProcessing: InkInputRightDragAction = InkInputRightDragAction(1i32);
}
impl ::std::convert::From<i32> for InkInputRightDragAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkInputRightDragAction {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkInputRightDragAction {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputRightDragAction;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkManager(::windows::runtime::IInspectable);
impl InkManager {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkManager, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Mode(&self) -> ::windows::runtime::Result<InkManipulationMode> {
        let this = self;
        unsafe {
            let mut result__: InkManipulationMode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkManipulationMode>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetMode(&self, value: InkManipulationMode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn ProcessPointerDown<'a, Param0: ::windows::runtime::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), pointerpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn ProcessPointerUpdate<'a, Param0: ::windows::runtime::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn ProcessPointerUp<'a, Param0: ::windows::runtime::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetDefaultDrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, InkDrawingAttributes>>(&self, drawingattributes: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), drawingattributes.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Foundation_Collections`*"]
    pub fn RecognizeAsync2(&self, recognitiontarget: InkRecognitionTarget) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetDefaultRecognizer<'a, Param0: ::windows::runtime::IntoParam<'a, InkRecognizer>>(&self, recognizer: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), recognizer.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Foundation_Collections`*"]
    pub fn RecognizeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, InkStrokeContainer>>(&self, strokecollection: Param0, recognitiontarget: InkRecognitionTarget) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = &::windows::runtime::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), strokecollection.into_param().abi(), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetRecognizers(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = &::windows::runtime::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn AddStroke<'a, Param0: ::windows::runtime::IntoParam<'a, InkStroke>>(&self, stroke: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn DeleteSelected(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn MoveSelected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, translation: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), translation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Foundation_Collections`*"]
    pub fn SelectWithPolyLine<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, polyline: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), polyline.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn SelectWithLine<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, from: Param0, to: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), from.into_param().abi(), to.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn CopySelectedToClipboard(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn PasteFromClipboard<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn CanPasteFromClipboard(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Storage_Streams`*"]
    pub fn LoadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Storage_Streams`*"]
    pub fn SaveAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn UpdateRecognitionResults<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(&self, recognitionresults: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), recognitionresults.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetStrokes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetRecognitionResults(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkManager;{4744737d-671b-4163-9c95-4e8d7a035fe1})");
}
unsafe impl ::windows::runtime::Interface for InkManager {
    type Vtable = IInkManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1195668349, 26395, 16739, [156, 149, 78, 141, 122, 3, 95, 225]);
}
impl ::windows::runtime::RuntimeName for InkManager {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkManager";
}
impl ::std::convert::From<InkManager> for ::windows::runtime::IUnknown {
    fn from(value: InkManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkManager> for ::windows::runtime::IUnknown {
    fn from(value: &InkManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkManager> for ::windows::runtime::IInspectable {
    fn from(value: InkManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkManager> for ::windows::runtime::IInspectable {
    fn from(value: &InkManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkManager> for IInkRecognizerContainer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkManager) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkManager> for IInkRecognizerContainer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkManager) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkRecognizerContainer> for InkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkRecognizerContainer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkRecognizerContainer> for &InkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkRecognizerContainer> {
        ::std::convert::TryInto::<IInkRecognizerContainer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<InkManager> for IInkStrokeContainer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkManager) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkManager> for IInkStrokeContainer {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkManager) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkStrokeContainer> for InkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkStrokeContainer> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkStrokeContainer> for &InkManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkStrokeContainer> {
        ::std::convert::TryInto::<IInkStrokeContainer>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkManipulationMode(pub i32);
impl InkManipulationMode {
    pub const Inking: InkManipulationMode = InkManipulationMode(0i32);
    pub const Erasing: InkManipulationMode = InkManipulationMode(1i32);
    pub const Selecting: InkManipulationMode = InkManipulationMode(2i32);
}
impl ::std::convert::From<i32> for InkManipulationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkManipulationMode {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkManipulationMode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkManipulationMode;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkModelerAttributes(::windows::runtime::IInspectable);
impl InkModelerAttributes {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn PredictionTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn SetPredictionTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn ScalingFactor(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetScalingFactor(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn UseVelocityBasedPressure(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetUseVelocityBasedPressure(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkModelerAttributes {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkModelerAttributes;{bad31f27-0cd9-4bfd-b6f3-9e03ba8d7454})");
}
unsafe impl ::windows::runtime::Interface for InkModelerAttributes {
    type Vtable = IInkModelerAttributes_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3134398247, 3289, 19453, [182, 243, 158, 3, 186, 141, 116, 84]);
}
impl ::windows::runtime::RuntimeName for InkModelerAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkModelerAttributes";
}
impl ::std::convert::From<InkModelerAttributes> for ::windows::runtime::IUnknown {
    fn from(value: InkModelerAttributes) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkModelerAttributes> for ::windows::runtime::IUnknown {
    fn from(value: &InkModelerAttributes) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkModelerAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkModelerAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkModelerAttributes> for ::windows::runtime::IInspectable {
    fn from(value: InkModelerAttributes) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkModelerAttributes> for ::windows::runtime::IInspectable {
    fn from(value: &InkModelerAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkModelerAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkModelerAttributes {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkModelerAttributes {}
unsafe impl ::std::marker::Sync for InkModelerAttributes {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPersistenceFormat(pub i32);
impl InkPersistenceFormat {
    pub const GifWithEmbeddedIsf: InkPersistenceFormat = InkPersistenceFormat(0i32);
    pub const Isf: InkPersistenceFormat = InkPersistenceFormat(1i32);
}
impl ::std::convert::From<i32> for InkPersistenceFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkPersistenceFormat {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkPersistenceFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPersistenceFormat;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkPoint(::windows::runtime::IInspectable);
impl InkPoint {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Pressure(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn CreateInkPoint<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(position: Param0, pressure: f32) -> ::windows::runtime::Result<InkPoint> {
        Self::IInkPointFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), position.into_param().abi(), pressure, &mut result__).from_abi::<InkPoint>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn TiltX(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn TiltY(&self) -> ::windows::runtime::Result<f32> {
        let this = &::windows::runtime::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Timestamp(&self) -> ::windows::runtime::Result<u64> {
        let this = &::windows::runtime::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__: u64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn CreateInkPointWithTiltAndTimestamp<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(position: Param0, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64) -> ::windows::runtime::Result<InkPoint> {
        Self::IInkPointFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), position.into_param().abi(), pressure, tiltx, tilty, timestamp, &mut result__).from_abi::<InkPoint>(result__)
        })
    }
    pub fn IInkPointFactory<R, F: FnOnce(&IInkPointFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkPoint, IInkPointFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInkPointFactory2<R, F: FnOnce(&IInkPointFactory2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkPoint, IInkPointFactory2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkPoint {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPoint;{9f87272b-858c-46a5-9b41-d195970459fd})");
}
unsafe impl ::windows::runtime::Interface for InkPoint {
    type Vtable = IInkPoint_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2676434731, 34188, 18085, [155, 65, 209, 149, 151, 4, 89, 253]);
}
impl ::windows::runtime::RuntimeName for InkPoint {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPoint";
}
impl ::std::convert::From<InkPoint> for ::windows::runtime::IUnknown {
    fn from(value: InkPoint) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkPoint> for ::windows::runtime::IUnknown {
    fn from(value: &InkPoint) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkPoint> for ::windows::runtime::IInspectable {
    fn from(value: InkPoint) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkPoint> for ::windows::runtime::IInspectable {
    fn from(value: &InkPoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkPoint {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkPoint {}
unsafe impl ::std::marker::Sync for InkPoint {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkPresenter(::windows::runtime::IInspectable);
impl InkPresenter {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsInputEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Input_Inking`, `UI_Core`*"]
    pub fn InputDeviceTypes(&self) -> ::windows::runtime::Result<super::super::Core::CoreInputDeviceTypes> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CoreInputDeviceTypes = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreInputDeviceTypes>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    #[doc = "*Required features: `UI_Input_Inking`, `UI_Core`*"]
    pub fn SetInputDeviceTypes(&self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn UnprocessedInput(&self) -> ::windows::runtime::Result<InkUnprocessedInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkUnprocessedInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn StrokeInput(&self) -> ::windows::runtime::Result<InkStrokeInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkStrokeInput>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn InputProcessingConfiguration(&self) -> ::windows::runtime::Result<InkInputProcessingConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkInputProcessingConfiguration>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn StrokeContainer(&self) -> ::windows::runtime::Result<InkStrokeContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkStrokeContainer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetStrokeContainer<'a, Param0: ::windows::runtime::IntoParam<'a, InkStrokeContainer>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn CopyDefaultDrawingAttributes(&self) -> ::windows::runtime::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributes>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn UpdateDefaultDrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, InkDrawingAttributes>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn ActivateCustomDrying(&self) -> ::windows::runtime::Result<InkSynchronizer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkSynchronizer>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetPredefinedConfiguration(&self, value: InkPresenterPredefinedConfiguration) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn StrokesCollected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemoveStrokesCollected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn StrokesErased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemoveStrokesErased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn HighContrastAdjustment(&self) -> ::windows::runtime::Result<InkHighContrastAdjustment> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenter2>(self)?;
        unsafe {
            let mut result__: InkHighContrastAdjustment = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkHighContrastAdjustment>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetHighContrastAdjustment(&self, value: InkHighContrastAdjustment) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenter2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn InputConfiguration(&self) -> ::windows::runtime::Result<InkInputConfiguration> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenter3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkInputConfiguration>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkPresenter {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenter;{a69b70e2-887b-458f-b173-4fe4438930a3})");
}
unsafe impl ::windows::runtime::Interface for InkPresenter {
    type Vtable = IInkPresenter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2795204834, 34939, 17807, [177, 115, 79, 228, 67, 137, 48, 163]);
}
impl ::windows::runtime::RuntimeName for InkPresenter {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenter";
}
impl ::std::convert::From<InkPresenter> for ::windows::runtime::IUnknown {
    fn from(value: InkPresenter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkPresenter> for ::windows::runtime::IUnknown {
    fn from(value: &InkPresenter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkPresenter> for ::windows::runtime::IInspectable {
    fn from(value: InkPresenter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkPresenter> for ::windows::runtime::IInspectable {
    fn from(value: &InkPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkPresenter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkPresenter {}
unsafe impl ::std::marker::Sync for InkPresenter {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPresenterPredefinedConfiguration(pub i32);
impl InkPresenterPredefinedConfiguration {
    pub const SimpleSinglePointer: InkPresenterPredefinedConfiguration = InkPresenterPredefinedConfiguration(0i32);
    pub const SimpleMultiplePointer: InkPresenterPredefinedConfiguration = InkPresenterPredefinedConfiguration(1i32);
}
impl ::std::convert::From<i32> for InkPresenterPredefinedConfiguration {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkPresenterPredefinedConfiguration {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkPresenterPredefinedConfiguration {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterPredefinedConfiguration;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkPresenterProtractor(::windows::runtime::IInspectable);
impl InkPresenterProtractor {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<InkPresenterStencilKind> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: InkPresenterStencilKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn ForegroundColor(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn Transform(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn SetTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn AreTickMarksVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn AreRaysVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetAreRaysVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsCenterMarkerVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsCenterMarkerVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsAngleReadoutVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsAngleReadoutVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsResizable(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsResizable(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Radius(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetRadius(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn AccentColor(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetAccentColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, InkPresenter>>(inkpresenter: Param0) -> ::windows::runtime::Result<InkPresenterProtractor> {
        Self::IInkPresenterProtractorFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<InkPresenterProtractor>(result__)
        })
    }
    pub fn IInkPresenterProtractorFactory<R, F: FnOnce(&IInkPresenterProtractorFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkPresenterProtractor, IInkPresenterProtractorFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkPresenterProtractor {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenterProtractor;{7de3f2aa-ef6c-4e91-a73b-5b70d56fbd17})");
}
unsafe impl ::windows::runtime::Interface for InkPresenterProtractor {
    type Vtable = IInkPresenterProtractor_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2112090794, 61292, 20113, [167, 59, 91, 112, 213, 111, 189, 23]);
}
impl ::windows::runtime::RuntimeName for InkPresenterProtractor {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterProtractor";
}
impl ::std::convert::From<InkPresenterProtractor> for ::windows::runtime::IUnknown {
    fn from(value: InkPresenterProtractor) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkPresenterProtractor> for ::windows::runtime::IUnknown {
    fn from(value: &InkPresenterProtractor) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkPresenterProtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkPresenterProtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkPresenterProtractor> for ::windows::runtime::IInspectable {
    fn from(value: InkPresenterProtractor) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkPresenterProtractor> for ::windows::runtime::IInspectable {
    fn from(value: &InkPresenterProtractor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkPresenterProtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkPresenterProtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkPresenterProtractor> for IInkPresenterStencil {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkPresenterProtractor) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkPresenterProtractor> for IInkPresenterStencil {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkPresenterProtractor) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkPresenterStencil> for InkPresenterProtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkPresenterStencil> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkPresenterStencil> for &InkPresenterProtractor {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkPresenterStencil> {
        ::std::convert::TryInto::<IInkPresenterStencil>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkPresenterProtractor {}
unsafe impl ::std::marker::Sync for InkPresenterProtractor {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkPresenterRuler(::windows::runtime::IInspectable);
impl InkPresenterRuler {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Length(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetLength(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Width(&self) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetWidth(&self, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Kind(&self) -> ::windows::runtime::Result<InkPresenterStencilKind> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: InkPresenterStencilKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn BackgroundColor(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn ForegroundColor(&self) -> ::windows::runtime::Result<super::super::Color> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn Transform(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn SetTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, InkPresenter>>(inkpresenter: Param0) -> ::windows::runtime::Result<InkPresenterRuler> {
        Self::IInkPresenterRulerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<InkPresenterRuler>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn AreTickMarksVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsCompassVisible(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetIsCompassVisible(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn IInkPresenterRulerFactory<R, F: FnOnce(&IInkPresenterRulerFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkPresenterRuler, IInkPresenterRulerFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkPresenterRuler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenterRuler;{6cda7d5a-dec7-4dd7-877a-2133f183d48a})");
}
unsafe impl ::windows::runtime::Interface for InkPresenterRuler {
    type Vtable = IInkPresenterRuler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1826258266, 57031, 19927, [135, 122, 33, 51, 241, 131, 212, 138]);
}
impl ::windows::runtime::RuntimeName for InkPresenterRuler {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterRuler";
}
impl ::std::convert::From<InkPresenterRuler> for ::windows::runtime::IUnknown {
    fn from(value: InkPresenterRuler) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkPresenterRuler> for ::windows::runtime::IUnknown {
    fn from(value: &InkPresenterRuler) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkPresenterRuler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkPresenterRuler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkPresenterRuler> for ::windows::runtime::IInspectable {
    fn from(value: InkPresenterRuler) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkPresenterRuler> for ::windows::runtime::IInspectable {
    fn from(value: &InkPresenterRuler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkPresenterRuler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkPresenterRuler {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<InkPresenterRuler> for IInkPresenterStencil {
    type Error = ::windows::runtime::Error;
    fn try_from(value: InkPresenterRuler) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&InkPresenterRuler> for IInkPresenterStencil {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &InkPresenterRuler) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkPresenterStencil> for InkPresenterRuler {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkPresenterStencil> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkPresenterStencil> for &InkPresenterRuler {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkPresenterStencil> {
        ::std::convert::TryInto::<IInkPresenterStencil>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for InkPresenterRuler {}
unsafe impl ::std::marker::Sync for InkPresenterRuler {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPresenterStencilKind(pub i32);
impl InkPresenterStencilKind {
    pub const Other: InkPresenterStencilKind = InkPresenterStencilKind(0i32);
    pub const Ruler: InkPresenterStencilKind = InkPresenterStencilKind(1i32);
    pub const Protractor: InkPresenterStencilKind = InkPresenterStencilKind(2i32);
}
impl ::std::convert::From<i32> for InkPresenterStencilKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkPresenterStencilKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkPresenterStencilKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterStencilKind;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkRecognitionResult(::windows::runtime::IInspectable);
impl InkRecognitionResult {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetTextCandidates(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetStrokes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkRecognitionResult {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognitionResult;{36461a94-5068-40ef-8a05-2c2fb60908a2})");
}
unsafe impl ::windows::runtime::Interface for InkRecognitionResult {
    type Vtable = IInkRecognitionResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(910563988, 20584, 16623, [138, 5, 44, 47, 182, 9, 8, 162]);
}
impl ::windows::runtime::RuntimeName for InkRecognitionResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognitionResult";
}
impl ::std::convert::From<InkRecognitionResult> for ::windows::runtime::IUnknown {
    fn from(value: InkRecognitionResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkRecognitionResult> for ::windows::runtime::IUnknown {
    fn from(value: &InkRecognitionResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkRecognitionResult> for ::windows::runtime::IInspectable {
    fn from(value: InkRecognitionResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkRecognitionResult> for ::windows::runtime::IInspectable {
    fn from(value: &InkRecognitionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkRecognitionResult {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkRecognitionResult {}
unsafe impl ::std::marker::Sync for InkRecognitionResult {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkRecognitionTarget(pub i32);
impl InkRecognitionTarget {
    pub const All: InkRecognitionTarget = InkRecognitionTarget(0i32);
    pub const Selected: InkRecognitionTarget = InkRecognitionTarget(1i32);
    pub const Recent: InkRecognitionTarget = InkRecognitionTarget(2i32);
}
impl ::std::convert::From<i32> for InkRecognitionTarget {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for InkRecognitionTarget {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for InkRecognitionTarget {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkRecognitionTarget;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkRecognizer(::windows::runtime::IInspectable);
impl InkRecognizer {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkRecognizer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognizer;{077ccea3-904d-442a-b151-aaca3631c43b})");
}
unsafe impl ::windows::runtime::Interface for InkRecognizer {
    type Vtable = IInkRecognizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(125619875, 36941, 17450, [177, 81, 170, 202, 54, 49, 196, 59]);
}
impl ::windows::runtime::RuntimeName for InkRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizer";
}
impl ::std::convert::From<InkRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: InkRecognizer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkRecognizer> for ::windows::runtime::IUnknown {
    fn from(value: &InkRecognizer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkRecognizer> for ::windows::runtime::IInspectable {
    fn from(value: InkRecognizer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkRecognizer> for ::windows::runtime::IInspectable {
    fn from(value: &InkRecognizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkRecognizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkRecognizerContainer(::windows::runtime::IInspectable);
impl InkRecognizerContainer {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkRecognizerContainer, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetDefaultRecognizer<'a, Param0: ::windows::runtime::IntoParam<'a, InkRecognizer>>(&self, recognizer: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), recognizer.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Foundation_Collections`*"]
    pub fn RecognizeAsync<'a, Param0: ::windows::runtime::IntoParam<'a, InkStrokeContainer>>(&self, strokecollection: Param0, recognitiontarget: InkRecognitionTarget) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), strokecollection.into_param().abi(), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetRecognizers(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkRecognizerContainer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognizerContainer;{a74d9a31-8047-4698-a912-f82a5085012f})");
}
unsafe impl ::windows::runtime::Interface for InkRecognizerContainer {
    type Vtable = IInkRecognizerContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2806880817, 32839, 18072, [169, 18, 248, 42, 80, 133, 1, 47]);
}
impl ::windows::runtime::RuntimeName for InkRecognizerContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizerContainer";
}
impl ::std::convert::From<InkRecognizerContainer> for ::windows::runtime::IUnknown {
    fn from(value: InkRecognizerContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkRecognizerContainer> for ::windows::runtime::IUnknown {
    fn from(value: &InkRecognizerContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkRecognizerContainer> for ::windows::runtime::IInspectable {
    fn from(value: InkRecognizerContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkRecognizerContainer> for ::windows::runtime::IInspectable {
    fn from(value: &InkRecognizerContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<InkRecognizerContainer> for IInkRecognizerContainer {
    fn from(value: InkRecognizerContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkRecognizerContainer> for IInkRecognizerContainer {
    fn from(value: &InkRecognizerContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkRecognizerContainer> for InkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkRecognizerContainer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInkRecognizerContainer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkRecognizerContainer> for &InkRecognizerContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkRecognizerContainer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInkRecognizerContainer>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkStroke(::windows::runtime::IInspectable);
impl InkStroke {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn DrawingAttributes(&self) -> ::windows::runtime::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributes>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetDrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, InkDrawingAttributes>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Selected(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetSelected(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Recognized(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetRenderingSegments(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Clone(&self) -> ::windows::runtime::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn PointTransform(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::runtime::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Numerics`*"]
    pub fn SetPointTransform<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkStroke2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetInkPoints(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkPoint>> {
        let this = &::windows::runtime::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkPoint>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn StrokeStartedTime(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = &::windows::runtime::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn SetStrokeStartedTime<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn StrokeDuration(&self) -> ::windows::runtime::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::runtime::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn SetStrokeDuration<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn PointerId(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<IInkStroke4>(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkStroke {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStroke;{15144d60-cce3-4fcf-9d52-11518ab6afd4})");
}
unsafe impl ::windows::runtime::Interface for InkStroke {
    type Vtable = IInkStroke_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(353652064, 52451, 20431, [157, 82, 17, 81, 138, 182, 175, 212]);
}
impl ::windows::runtime::RuntimeName for InkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStroke";
}
impl ::std::convert::From<InkStroke> for ::windows::runtime::IUnknown {
    fn from(value: InkStroke) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkStroke> for ::windows::runtime::IUnknown {
    fn from(value: &InkStroke) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkStroke {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkStroke {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkStroke> for ::windows::runtime::IInspectable {
    fn from(value: InkStroke) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkStroke> for ::windows::runtime::IInspectable {
    fn from(value: &InkStroke) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkStroke {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkStroke {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkStroke {}
unsafe impl ::std::marker::Sync for InkStroke {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkStrokeBuilder(::windows::runtime::IInspectable);
impl InkStrokeBuilder {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkStrokeBuilder, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn BeginStroke<'a, Param0: ::windows::runtime::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), pointerpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn AppendToStroke<'a, Param0: ::windows::runtime::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::runtime::Result<super::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<super::PointerPoint>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn EndStroke<'a, Param0: ::windows::runtime::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::runtime::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Foundation_Collections`*"]
    pub fn CreateStroke<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, points: Param0) -> ::windows::runtime::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), points.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetDefaultDrawingAttributes<'a, Param0: ::windows::runtime::IntoParam<'a, InkDrawingAttributes>>(&self, drawingattributes: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), drawingattributes.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`, `Foundation_Numerics`*"]
    pub fn CreateStrokeFromInkPoints<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InkPoint>>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, inkpoints: Param0, transform: Param1) -> ::windows::runtime::Result<InkStroke> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeBuilder2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), inkpoints.into_param().abi(), transform.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Foundation_Collections`, `Foundation_Numerics`*"]
    pub fn CreateStrokeFromInkPoints2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InkPoint>>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>,
    >(
        &self,
        inkpoints: Param0,
        transform: Param1,
        strokestartedtime: Param2,
        strokeduration: Param3,
    ) -> ::windows::runtime::Result<InkStroke> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeBuilder3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), inkpoints.into_param().abi(), transform.into_param().abi(), strokestartedtime.into_param().abi(), strokeduration.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkStrokeBuilder {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeBuilder;{82bbd1dc-1c63-41dc-9e07-4b4a70ced801})");
}
unsafe impl ::windows::runtime::Interface for InkStrokeBuilder {
    type Vtable = IInkStrokeBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2193347036, 7267, 16860, [158, 7, 75, 74, 112, 206, 216, 1]);
}
impl ::windows::runtime::RuntimeName for InkStrokeBuilder {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeBuilder";
}
impl ::std::convert::From<InkStrokeBuilder> for ::windows::runtime::IUnknown {
    fn from(value: InkStrokeBuilder) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkStrokeBuilder> for ::windows::runtime::IUnknown {
    fn from(value: &InkStrokeBuilder) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkStrokeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkStrokeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkStrokeBuilder> for ::windows::runtime::IInspectable {
    fn from(value: InkStrokeBuilder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkStrokeBuilder> for ::windows::runtime::IInspectable {
    fn from(value: &InkStrokeBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkStrokeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkStrokeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkStrokeContainer(::windows::runtime::IInspectable);
impl InkStrokeContainer {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkStrokeContainer, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn BoundingRect(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn AddStroke<'a, Param0: ::windows::runtime::IntoParam<'a, InkStroke>>(&self, stroke: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn DeleteSelected(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn MoveSelected<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, translation: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), translation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Foundation_Collections`*"]
    pub fn SelectWithPolyLine<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, polyline: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), polyline.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn SelectWithLine<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, from: Param0, to: Param1) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), from.into_param().abi(), to.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn CopySelectedToClipboard(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn PasteFromClipboard<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn CanPasteFromClipboard(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Storage_Streams`*"]
    pub fn LoadAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, inputstream: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Storage_Streams`*"]
    pub fn SaveAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn UpdateRecognitionResults<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(&self, recognitionresults: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), recognitionresults.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetStrokes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn GetRecognitionResults(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn AddStrokes<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InkStroke>>>(&self, strokes: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), strokes.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `Storage_Streams`*"]
    pub fn SaveWithFormatAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0, inkpersistenceformat: InkPersistenceFormat) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), outputstream.into_param().abi(), inkpersistenceformat, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn GetStrokeById(&self, id: u32) -> ::windows::runtime::Result<InkStroke> {
        let this = &::windows::runtime::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), id, &mut result__).from_abi::<InkStroke>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkStrokeContainer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeContainer;{22accbc6-faa9-4f14-b68c-f6cee670ae16})");
}
unsafe impl ::windows::runtime::Interface for InkStrokeContainer {
    type Vtable = IInkStrokeContainer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(581749702, 64169, 20244, [182, 140, 246, 206, 230, 112, 174, 22]);
}
impl ::windows::runtime::RuntimeName for InkStrokeContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeContainer";
}
impl ::std::convert::From<InkStrokeContainer> for ::windows::runtime::IUnknown {
    fn from(value: InkStrokeContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkStrokeContainer> for ::windows::runtime::IUnknown {
    fn from(value: &InkStrokeContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkStrokeContainer> for ::windows::runtime::IInspectable {
    fn from(value: InkStrokeContainer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkStrokeContainer> for ::windows::runtime::IInspectable {
    fn from(value: &InkStrokeContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<InkStrokeContainer> for IInkStrokeContainer {
    fn from(value: InkStrokeContainer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkStrokeContainer> for IInkStrokeContainer {
    fn from(value: &InkStrokeContainer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkStrokeContainer> for InkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkStrokeContainer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInkStrokeContainer>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IInkStrokeContainer> for &InkStrokeContainer {
    fn into_param(self) -> ::windows::runtime::Param<'a, IInkStrokeContainer> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IInkStrokeContainer>::into(::std::clone::Clone::clone(self)))
    }
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkStrokeInput(::windows::runtime::IInspectable);
impl InkStrokeInput {
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn StrokeStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemoveStrokeStarted<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn StrokeContinued<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemoveStrokeContinued<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn StrokeEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemoveStrokeEnded<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn StrokeCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemoveStrokeCanceled<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn InkPresenter(&self) -> ::windows::runtime::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenter>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkStrokeInput {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeInput;{cf2ffe7b-5e10-43c6-a080-88f26e1dc67d})");
}
unsafe impl ::windows::runtime::Interface for InkStrokeInput {
    type Vtable = IInkStrokeInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3476029051, 24080, 17350, [160, 128, 136, 242, 110, 29, 198, 125]);
}
impl ::windows::runtime::RuntimeName for InkStrokeInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeInput";
}
impl ::std::convert::From<InkStrokeInput> for ::windows::runtime::IUnknown {
    fn from(value: InkStrokeInput) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkStrokeInput> for ::windows::runtime::IUnknown {
    fn from(value: &InkStrokeInput) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkStrokeInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkStrokeInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkStrokeInput> for ::windows::runtime::IInspectable {
    fn from(value: InkStrokeInput) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkStrokeInput> for ::windows::runtime::IInspectable {
    fn from(value: &InkStrokeInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkStrokeInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkStrokeInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkStrokeInput {}
unsafe impl ::std::marker::Sync for InkStrokeInput {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkStrokeRenderingSegment(::windows::runtime::IInspectable);
impl InkStrokeRenderingSegment {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn Position(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn BezierControlPoint1(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn BezierControlPoint2(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Pressure(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn TiltX(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn TiltY(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn Twist(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkStrokeRenderingSegment {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeRenderingSegment;{68510f1f-88e3-477a-a2fa-569f5f1f9bd5})");
}
unsafe impl ::windows::runtime::Interface for InkStrokeRenderingSegment {
    type Vtable = IInkStrokeRenderingSegment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1750142751, 35043, 18298, [162, 250, 86, 159, 95, 31, 155, 213]);
}
impl ::windows::runtime::RuntimeName for InkStrokeRenderingSegment {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeRenderingSegment";
}
impl ::std::convert::From<InkStrokeRenderingSegment> for ::windows::runtime::IUnknown {
    fn from(value: InkStrokeRenderingSegment) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkStrokeRenderingSegment> for ::windows::runtime::IUnknown {
    fn from(value: &InkStrokeRenderingSegment) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkStrokeRenderingSegment> for ::windows::runtime::IInspectable {
    fn from(value: InkStrokeRenderingSegment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkStrokeRenderingSegment> for ::windows::runtime::IInspectable {
    fn from(value: &InkStrokeRenderingSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkStrokeRenderingSegment {}
unsafe impl ::std::marker::Sync for InkStrokeRenderingSegment {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkStrokesCollectedEventArgs(::windows::runtime::IInspectable);
impl InkStrokesCollectedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn Strokes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkStrokesCollectedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokesCollectedEventArgs;{c4f3f229-1938-495c-b4d9-6de4b08d4811})");
}
unsafe impl ::windows::runtime::Interface for InkStrokesCollectedEventArgs {
    type Vtable = IInkStrokesCollectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3304321577, 6456, 18780, [180, 217, 109, 228, 176, 141, 72, 17]);
}
impl ::windows::runtime::RuntimeName for InkStrokesCollectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesCollectedEventArgs";
}
impl ::std::convert::From<InkStrokesCollectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: InkStrokesCollectedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkStrokesCollectedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InkStrokesCollectedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkStrokesCollectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: InkStrokesCollectedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkStrokesCollectedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &InkStrokesCollectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkStrokesErasedEventArgs(::windows::runtime::IInspectable);
impl InkStrokesErasedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn Strokes(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkStrokesErasedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokesErasedEventArgs;{a4216a22-1503-4ebf-8ff5-2de84584a8aa})");
}
unsafe impl ::windows::runtime::Interface for InkStrokesErasedEventArgs {
    type Vtable = IInkStrokesErasedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2753653282, 5379, 20159, [143, 245, 45, 232, 69, 132, 168, 170]);
}
impl ::windows::runtime::RuntimeName for InkStrokesErasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesErasedEventArgs";
}
impl ::std::convert::From<InkStrokesErasedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: InkStrokesErasedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkStrokesErasedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &InkStrokesErasedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkStrokesErasedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: InkStrokesErasedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkStrokesErasedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &InkStrokesErasedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkSynchronizer(::windows::runtime::IInspectable);
impl InkSynchronizer {
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation_Collections`*"]
    pub fn BeginDry(&self) -> ::windows::runtime::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn EndDry(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkSynchronizer {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkSynchronizer;{9b9ea160-ae9b-45f9-8407-4b493b163661})");
}
unsafe impl ::windows::runtime::Interface for InkSynchronizer {
    type Vtable = IInkSynchronizer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2610864480, 44699, 17913, [132, 7, 75, 73, 59, 22, 54, 97]);
}
impl ::windows::runtime::RuntimeName for InkSynchronizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkSynchronizer";
}
impl ::std::convert::From<InkSynchronizer> for ::windows::runtime::IUnknown {
    fn from(value: InkSynchronizer) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkSynchronizer> for ::windows::runtime::IUnknown {
    fn from(value: &InkSynchronizer) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkSynchronizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkSynchronizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkSynchronizer> for ::windows::runtime::IInspectable {
    fn from(value: InkSynchronizer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkSynchronizer> for ::windows::runtime::IInspectable {
    fn from(value: &InkSynchronizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkSynchronizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkSynchronizer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkUnprocessedInput(::windows::runtime::IInspectable);
impl InkUnprocessedInput {
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn PointerEntered<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemovePointerEntered<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn PointerHovered<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemovePointerHovered<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn PointerExited<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemovePointerExited<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn PointerPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemovePointerPressed<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn PointerMoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemovePointerMoved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn PointerReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemovePointerReleased<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`, `UI_Core`*"]
    pub fn PointerLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Input_Inking`, `Foundation`*"]
    pub fn RemovePointerLost<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn InkPresenter(&self) -> ::windows::runtime::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenter>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkUnprocessedInput {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkUnprocessedInput;{db4445e0-8398-4921-ac3b-ab978c5ba256})");
}
unsafe impl ::windows::runtime::Interface for InkUnprocessedInput {
    type Vtable = IInkUnprocessedInput_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3678684640, 33688, 18721, [172, 59, 171, 151, 140, 91, 162, 86]);
}
impl ::windows::runtime::RuntimeName for InkUnprocessedInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkUnprocessedInput";
}
impl ::std::convert::From<InkUnprocessedInput> for ::windows::runtime::IUnknown {
    fn from(value: InkUnprocessedInput) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkUnprocessedInput> for ::windows::runtime::IUnknown {
    fn from(value: &InkUnprocessedInput) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkUnprocessedInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkUnprocessedInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkUnprocessedInput> for ::windows::runtime::IInspectable {
    fn from(value: InkUnprocessedInput) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkUnprocessedInput> for ::windows::runtime::IInspectable {
    fn from(value: &InkUnprocessedInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkUnprocessedInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkUnprocessedInput {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkUnprocessedInput {}
unsafe impl ::std::marker::Sync for InkUnprocessedInput {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct PenAndInkSettings(::windows::runtime::IInspectable);
impl PenAndInkSettings {
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsHandwritingDirectlyIntoTextFieldEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn PenHandedness(&self) -> ::windows::runtime::Result<PenHandedness> {
        let this = self;
        unsafe {
            let mut result__: PenHandedness = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PenHandedness>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn HandwritingLineHeight(&self) -> ::windows::runtime::Result<HandwritingLineHeight> {
        let this = self;
        unsafe {
            let mut result__: HandwritingLineHeight = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<HandwritingLineHeight>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn FontFamilyName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn UserConsentsToHandwritingTelemetryCollection(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn IsTouchHandwritingEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<PenAndInkSettings> {
        Self::IPenAndInkSettingsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<PenAndInkSettings>(result__)
        })
    }
    #[doc = "*Required features: `UI_Input_Inking`*"]
    pub fn SetPenHandedness(&self, value: PenHandedness) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IPenAndInkSettings2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    pub fn IPenAndInkSettingsStatics<R, F: FnOnce(&IPenAndInkSettingsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PenAndInkSettings, IPenAndInkSettingsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PenAndInkSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.PenAndInkSettings;{bc2ceb8f-0066-44a8-bb7a-b839b3deb8f5})");
}
unsafe impl ::windows::runtime::Interface for PenAndInkSettings {
    type Vtable = IPenAndInkSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3157060495, 102, 17576, [187, 122, 184, 57, 179, 222, 184, 245]);
}
impl ::windows::runtime::RuntimeName for PenAndInkSettings {
    const NAME: &'static str = "Windows.UI.Input.Inking.PenAndInkSettings";
}
impl ::std::convert::From<PenAndInkSettings> for ::windows::runtime::IUnknown {
    fn from(value: PenAndInkSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&PenAndInkSettings> for ::windows::runtime::IUnknown {
    fn from(value: &PenAndInkSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for PenAndInkSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &PenAndInkSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<PenAndInkSettings> for ::windows::runtime::IInspectable {
    fn from(value: PenAndInkSettings) -> Self {
        value.0
    }
}
impl ::std::convert::From<&PenAndInkSettings> for ::windows::runtime::IInspectable {
    fn from(value: &PenAndInkSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for PenAndInkSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a PenAndInkSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for PenAndInkSettings {}
unsafe impl ::std::marker::Sync for PenAndInkSettings {}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PenHandedness(pub i32);
impl PenHandedness {
    pub const Right: PenHandedness = PenHandedness(0i32);
    pub const Left: PenHandedness = PenHandedness(1i32);
}
impl ::std::convert::From<i32> for PenHandedness {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PenHandedness {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PenHandedness {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenHandedness;i4)");
}
#[doc = "*Required features: `UI_Input_Inking`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PenTipShape(pub i32);
impl PenTipShape {
    pub const Circle: PenTipShape = PenTipShape(0i32);
    pub const Rectangle: PenTipShape = PenTipShape(1i32);
}
impl ::std::convert::From<i32> for PenTipShape {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PenTipShape {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PenTipShape {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenTipShape;i4)");
}
