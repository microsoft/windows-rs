#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "UI_Input_Inking_Analysis")]
pub mod Analysis;
#[cfg(feature = "UI_Input_Inking_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking_Preview")]
pub mod Preview;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct HandwritingLineHeight(pub i32);
impl HandwritingLineHeight {
    pub const Small: HandwritingLineHeight = HandwritingLineHeight(0i32);
    pub const Medium: HandwritingLineHeight = HandwritingLineHeight(1i32);
    pub const Large: HandwritingLineHeight = HandwritingLineHeight(2i32);
}
impl ::core::convert::From<i32> for HandwritingLineHeight {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for HandwritingLineHeight {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for HandwritingLineHeight {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.HandwritingLineHeight;i4)");
}
impl ::windows::core::DefaultType for HandwritingLineHeight {
    type DefaultType = Self;
}
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkDrawingAttributes(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes {
    type Vtable = IInkDrawingAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97a2176c_6774_48ad_84f0_48f5a9be74f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PenTipShape) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PenTipShape) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkDrawingAttributes2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes2 {
    type Vtable = IInkDrawingAttributes2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cab6508_8ec4_42fd_a5a5_e4b7d1d5316d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkDrawingAttributes3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes3 {
    type Vtable = IInkDrawingAttributes3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72020002_7d5b_4690_8af4_e664cbe2b74f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InkDrawingAttributesKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkDrawingAttributes4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes4 {
    type Vtable = IInkDrawingAttributes4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef65dc25_9f19_456d_91a3_bc3a3d91c5fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkDrawingAttributes5(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes5 {
    type Vtable = IInkDrawingAttributes5_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd11aa0bb_0775_4852_ae64_41143a7ae6c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes5_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkDrawingAttributesPencilProperties(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkDrawingAttributesPencilProperties {
    type Vtable = IInkDrawingAttributesPencilProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f2534cb_2d86_41bb_b0e8_e4c2a0253c52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributesPencilProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkDrawingAttributesStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkDrawingAttributesStatics {
    type Vtable = IInkDrawingAttributesStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf731e03f_1a65_4862_96cb_6e1665e17f6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributesStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkInputConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkInputConfiguration {
    type Vtable = IInkInputConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93a68dc4_0b7b_49d7_b34f_9901e524dcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkInputConfiguration2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkInputConfiguration2 {
    type Vtable = IInkInputConfiguration2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac2272e_81b4_5cc4_a36d_d057c387dfda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputConfiguration2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkInputProcessingConfiguration(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkInputProcessingConfiguration {
    type Vtable = IInkInputProcessingConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2778d85e_33ca_4b06_a6d3_ac3945116d37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputProcessingConfiguration_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InkInputProcessingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: InkInputProcessingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InkInputRightDragAction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: InkInputRightDragAction) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkManager(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkManager {
    type Vtable = IInkManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4744737d_671b_4163_9c95_4e8d7a035fe1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InkManipulationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: InkManipulationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerpoint: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkModelerAttributes(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkModelerAttributes {
    type Vtable = IInkModelerAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad31f27_0cd9_4bfd_b6f3_9e03ba8d7454);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkModelerAttributes_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkModelerAttributes2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkModelerAttributes2 {
    type Vtable = IInkModelerAttributes2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86d1d09a_4ef8_5e25_b7bc_b65424f16bb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkModelerAttributes2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPoint(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPoint {
    type Vtable = IInkPoint_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f87272b_858c_46a5_9b41_d195970459fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPoint_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPoint2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPoint2 {
    type Vtable = IInkPoint2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfba9c3f7_ae56_4d5c_bd2f_0ac45f5e4af9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPoint2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInkPointFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPointFactory {
    type Vtable = IInkPointFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29e5d51c_c98f_405d_9f3b_e53e31068d4d);
}
impl IInkPointFactory {
    #[cfg(feature = "Foundation")]
    pub fn CreateInkPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0, pressure: f32) -> ::windows::core::Result<InkPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), pressure, &mut result__).from_abi::<InkPoint>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IInkPointFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{29e5d51c-c98f-405d-9f3b-e53e31068d4d}");
}
impl ::core::convert::From<IInkPointFactory> for ::windows::core::IUnknown {
    fn from(value: IInkPointFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInkPointFactory> for ::windows::core::IUnknown {
    fn from(value: &IInkPointFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkPointFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkPointFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInkPointFactory> for ::windows::core::IInspectable {
    fn from(value: IInkPointFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInkPointFactory> for ::windows::core::IInspectable {
    fn from(value: &IInkPointFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkPointFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInkPointFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPointFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: super::super::super::Foundation::Point, pressure: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPointFactory2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPointFactory2 {
    type Vtable = IInkPointFactory2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0145e85_daff_45f2_ad69_050d8256a209);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPointFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPresenter(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPresenter {
    type Vtable = IInkPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa69b70e2_887b_458f_b173_4fe4438930a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: InkPresenterPredefinedConfiguration) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPresenter2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPresenter2 {
    type Vtable = IInkPresenter2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf53e612_9a34_11e6_9f33_a24fc0d9649c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InkHighContrastAdjustment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: InkHighContrastAdjustment) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPresenter3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPresenter3 {
    type Vtable = IInkPresenter3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e1ce89_d37d_4a90_83fc_7f5e9dfbf217);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPresenterProtractor(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPresenterProtractor {
    type Vtable = IInkPresenterProtractor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de3f2aa_ef6c_4e91_a73b_5b70d56fbd17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterProtractor_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Color) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPresenterProtractorFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPresenterProtractorFactory {
    type Vtable = IInkPresenterProtractorFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x320103c9_68fa_47e9_8127_8370711fc46c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterProtractorFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPresenterRuler(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPresenterRuler {
    type Vtable = IInkPresenterRuler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cda7d5a_dec7_4dd7_877a_2133f183d48a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRuler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkPresenterRuler2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPresenterRuler2 {
    type Vtable = IInkPresenterRuler2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45130dc1_bc61_44d4_a423_54712ae671c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRuler2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInkPresenterRulerFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPresenterRulerFactory {
    type Vtable = IInkPresenterRulerFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34361beb_9001_4a4b_a690_69dbaf63e501);
}
impl IInkPresenterRulerFactory {
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, InkPresenter>>(&self, inkpresenter: Param0) -> ::windows::core::Result<InkPresenterRuler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<InkPresenterRuler>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IInkPresenterRulerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{34361beb-9001-4a4b-a690-69dbaf63e501}");
}
impl ::core::convert::From<IInkPresenterRulerFactory> for ::windows::core::IUnknown {
    fn from(value: IInkPresenterRulerFactory) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInkPresenterRulerFactory> for ::windows::core::IUnknown {
    fn from(value: &IInkPresenterRulerFactory) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInkPresenterRulerFactory> for ::windows::core::IInspectable {
    fn from(value: IInkPresenterRulerFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInkPresenterRulerFactory> for ::windows::core::IInspectable {
    fn from(value: &IInkPresenterRulerFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRulerFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInkPresenterStencil(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkPresenterStencil {
    type Vtable = IInkPresenterStencil_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30d12d6d_3e06_4d02_b116_277fb5d8addc);
}
impl IInkPresenterStencil {
    pub fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind> {
        let this = self;
        unsafe {
            let mut result__: InkPresenterStencilKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IInkPresenterStencil {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{30d12d6d-3e06-4d02-b116-277fb5d8addc}");
}
impl ::core::convert::From<IInkPresenterStencil> for ::windows::core::IUnknown {
    fn from(value: IInkPresenterStencil) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInkPresenterStencil> for ::windows::core::IUnknown {
    fn from(value: &IInkPresenterStencil) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkPresenterStencil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkPresenterStencil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInkPresenterStencil> for ::windows::core::IInspectable {
    fn from(value: IInkPresenterStencil) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInkPresenterStencil> for ::windows::core::IInspectable {
    fn from(value: &IInkPresenterStencil) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkPresenterStencil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInkPresenterStencil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterStencil_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut InkPresenterStencilKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::Color) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkRecognitionResult(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkRecognitionResult {
    type Vtable = IInkRecognitionResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36461a94_5068_40ef_8a05_2c2fb60908a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionResult_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkRecognizer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkRecognizer {
    type Vtable = IInkRecognizer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x077ccea3_904d_442a_b151_aaca3631c43b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizer_abi(
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
pub struct IInkRecognizerContainer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkRecognizerContainer {
    type Vtable = IInkRecognizerContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74d9a31_8047_4698_a912_f82a5085012f);
}
impl IInkRecognizerContainer {
    pub fn SetDefaultRecognizer<'a, Param0: ::windows::core::IntoParam<'a, InkRecognizer>>(&self, recognizer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), recognizer.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RecognizeAsync<'a, Param0: ::windows::core::IntoParam<'a, InkStrokeContainer>>(&self, strokecollection: Param0, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), strokecollection.into_param().abi(), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IInkRecognizerContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a74d9a31-8047-4698-a912-f82a5085012f}");
}
impl ::core::convert::From<IInkRecognizerContainer> for ::windows::core::IUnknown {
    fn from(value: IInkRecognizerContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInkRecognizerContainer> for ::windows::core::IUnknown {
    fn from(value: &IInkRecognizerContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInkRecognizerContainer> for ::windows::core::IInspectable {
    fn from(value: IInkRecognizerContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInkRecognizerContainer> for ::windows::core::IInspectable {
    fn from(value: &IInkRecognizerContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, recognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, strokecollection: ::windows::core::RawPtr, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStroke(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStroke {
    type Vtable = IInkStroke_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15144d60_cce3_4fcf_9d52_11518ab6afd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStroke2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStroke2 {
    type Vtable = IInkStroke2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5db9e4f4_bafa_4de1_89d3_201b1ed7d89b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStroke3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStroke3 {
    type Vtable = IInkStroke3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a807374_9499_411d_a1c4_68855d03d65f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStroke4(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStroke4 {
    type Vtable = IInkStroke4_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd5b62e5_b6e9_5b91_a577_1921d2348690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke4_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStrokeBuilder(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokeBuilder {
    type Vtable = IInkStrokeBuilder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82bbd1dc_1c63_41dc_9e07_4b4a70ced801);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, points: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStrokeBuilder2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokeBuilder2 {
    type Vtable = IInkStrokeBuilder2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd82bc27_731f_4cbc_bbbf_6d468044f1e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inkpoints: ::windows::core::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStrokeBuilder3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokeBuilder3 {
    type Vtable = IInkStrokeBuilder3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2c71fcd_5472_46b1_a81d_c37a3d169441);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inkpoints: ::windows::core::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: ::windows::core::RawPtr, strokeduration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IInkStrokeContainer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokeContainer {
    type Vtable = IInkStrokeContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22accbc6_faa9_4f14_b68c_f6cee670ae16);
}
impl IInkStrokeContainer {
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn AddStroke<'a, Param0: ::windows::core::IntoParam<'a, InkStroke>>(&self, stroke: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MoveSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, translation: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), translation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SelectWithPolyLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, polyline: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), polyline.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectWithLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, from: Param0, to: Param1) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), from.into_param().abi(), to.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PasteFromClipboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, inputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(&self, recognitionresults: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), recognitionresults.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for IInkStrokeContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{22accbc6-faa9-4f14-b68c-f6cee670ae16}");
}
impl ::core::convert::From<IInkStrokeContainer> for ::windows::core::IUnknown {
    fn from(value: IInkStrokeContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IInkStrokeContainer> for ::windows::core::IUnknown {
    fn from(value: &IInkStrokeContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IInkStrokeContainer> for ::windows::core::IInspectable {
    fn from(value: IInkStrokeContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IInkStrokeContainer> for ::windows::core::IInspectable {
    fn from(value: &IInkStrokeContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, translation: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, polyline: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, position: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, recognitionresults: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStrokeContainer2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokeContainer2 {
    type Vtable = IInkStrokeContainer2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8901d364_da36_4bcf_9e5c_d195825995b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStrokeContainer3(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokeContainer3 {
    type Vtable = IInkStrokeContainer3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d07bea5_baea_4c82_a719_7b83da1067d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, outputstream: ::windows::core::RawPtr, inkpersistenceformat: InkPersistenceFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, id: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStrokeInput(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokeInput {
    type Vtable = IInkStrokeInput_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2ffe7b_5e10_43c6_a080_88f26e1dc67d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeInput_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStrokeRenderingSegment(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokeRenderingSegment {
    type Vtable = IInkStrokeRenderingSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68510f1f_88e3_477a_a2fa_569f5f1f9bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeRenderingSegment_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStrokesCollectedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokesCollectedEventArgs {
    type Vtable = IInkStrokesCollectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4f3f229_1938_495c_b4d9_6de4b08d4811);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokesCollectedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkStrokesErasedEventArgs(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkStrokesErasedEventArgs {
    type Vtable = IInkStrokesErasedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4216a22_1503_4ebf_8ff5_2de84584a8aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokesErasedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkSynchronizer(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkSynchronizer {
    type Vtable = IInkSynchronizer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b9ea160_ae9b_45f9_8407_4b493b163661);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkSynchronizer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IInkUnprocessedInput(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IInkUnprocessedInput {
    type Vtable = IInkUnprocessedInput_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb4445e0_8398_4921_ac3b_ab978c5ba256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkUnprocessedInput_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPenAndInkSettings(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPenAndInkSettings {
    type Vtable = IPenAndInkSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc2ceb8f_0066_44a8_bb7a_b839b3deb8f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut PenHandedness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut HandwritingLineHeight) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPenAndInkSettings2(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPenAndInkSettings2 {
    type Vtable = IPenAndInkSettings2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3262da53_1f44_55e2_9929_ebf77e5481b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettings2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: PenHandedness) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPenAndInkSettingsStatics(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPenAndInkSettingsStatics {
    type Vtable = IPenAndInkSettingsStatics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed6dd036_5708_5c3c_96db_f2f552eab641);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettingsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkDrawingAttributes(pub ::windows::core::IInspectable);
impl InkDrawingAttributes {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkDrawingAttributes, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Color(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PenTip(&self) -> ::windows::core::Result<PenTipShape> {
        let this = self;
        unsafe {
            let mut result__: PenTipShape = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PenTipShape>(result__)
        }
    }
    pub fn SetPenTip(&self, value: PenTipShape) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn IgnorePressure(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnorePressure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn FitToCurve(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetFitToCurve(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PenTipTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPenTipTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn DrawAsHighlighter(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetDrawAsHighlighter(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkDrawingAttributesKind> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__: InkDrawingAttributesKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributesKind>(result__)
        }
    }
    pub fn PencilProperties(&self) -> ::windows::core::Result<InkDrawingAttributesPencilProperties> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributesPencilProperties>(result__)
        }
    }
    pub fn CreateForPencil() -> ::windows::core::Result<InkDrawingAttributes> {
        Self::IInkDrawingAttributesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributes>(result__)
        })
    }
    pub fn IgnoreTilt(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnoreTilt(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ModelerAttributes(&self) -> ::windows::core::Result<InkModelerAttributes> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkModelerAttributes>(result__)
        }
    }
    pub fn IInkDrawingAttributesStatics<R, F: FnOnce(&IInkDrawingAttributesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkDrawingAttributes, IInkDrawingAttributesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InkDrawingAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkDrawingAttributes;{97a2176c-6774-48ad-84f0-48f5a9be74f9})");
}
unsafe impl ::windows::core::Interface for InkDrawingAttributes {
    type Vtable = IInkDrawingAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97a2176c_6774_48ad_84f0_48f5a9be74f9);
}
impl ::windows::core::RuntimeName for InkDrawingAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributes";
}
impl ::core::convert::From<InkDrawingAttributes> for ::windows::core::IUnknown {
    fn from(value: InkDrawingAttributes) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkDrawingAttributes> for ::windows::core::IUnknown {
    fn from(value: &InkDrawingAttributes) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkDrawingAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkDrawingAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkDrawingAttributes> for ::windows::core::IInspectable {
    fn from(value: InkDrawingAttributes) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkDrawingAttributes> for ::windows::core::IInspectable {
    fn from(value: &InkDrawingAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkDrawingAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkDrawingAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkDrawingAttributes {}
unsafe impl ::core::marker::Sync for InkDrawingAttributes {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkDrawingAttributesKind(pub i32);
impl InkDrawingAttributesKind {
    pub const Default: InkDrawingAttributesKind = InkDrawingAttributesKind(0i32);
    pub const Pencil: InkDrawingAttributesKind = InkDrawingAttributesKind(1i32);
}
impl ::core::convert::From<i32> for InkDrawingAttributesKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkDrawingAttributesKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkDrawingAttributesKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkDrawingAttributesKind;i4)");
}
impl ::windows::core::DefaultType for InkDrawingAttributesKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkDrawingAttributesPencilProperties(pub ::windows::core::IInspectable);
impl InkDrawingAttributesPencilProperties {
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InkDrawingAttributesPencilProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties;{4f2534cb-2d86-41bb-b0e8-e4c2a0253c52})");
}
unsafe impl ::windows::core::Interface for InkDrawingAttributesPencilProperties {
    type Vtable = IInkDrawingAttributesPencilProperties_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f2534cb_2d86_41bb_b0e8_e4c2a0253c52);
}
impl ::windows::core::RuntimeName for InkDrawingAttributesPencilProperties {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties";
}
impl ::core::convert::From<InkDrawingAttributesPencilProperties> for ::windows::core::IUnknown {
    fn from(value: InkDrawingAttributesPencilProperties) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkDrawingAttributesPencilProperties> for ::windows::core::IUnknown {
    fn from(value: &InkDrawingAttributesPencilProperties) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkDrawingAttributesPencilProperties> for ::windows::core::IInspectable {
    fn from(value: InkDrawingAttributesPencilProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkDrawingAttributesPencilProperties> for ::windows::core::IInspectable {
    fn from(value: &InkDrawingAttributesPencilProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkDrawingAttributesPencilProperties {}
unsafe impl ::core::marker::Sync for InkDrawingAttributesPencilProperties {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkHighContrastAdjustment(pub i32);
impl InkHighContrastAdjustment {
    pub const UseSystemColorsWhenNecessary: InkHighContrastAdjustment = InkHighContrastAdjustment(0i32);
    pub const UseSystemColors: InkHighContrastAdjustment = InkHighContrastAdjustment(1i32);
    pub const UseOriginalColors: InkHighContrastAdjustment = InkHighContrastAdjustment(2i32);
}
impl ::core::convert::From<i32> for InkHighContrastAdjustment {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkHighContrastAdjustment {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkHighContrastAdjustment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkHighContrastAdjustment;i4)");
}
impl ::windows::core::DefaultType for InkHighContrastAdjustment {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkInputConfiguration(pub ::windows::core::IInspectable);
impl InkInputConfiguration {
    pub fn IsPrimaryBarrelButtonInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPrimaryBarrelButtonInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsEraserInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEraserInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsPenHapticFeedbackEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsPenHapticFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkInputConfiguration;{93a68dc4-0b7b-49d7-b34f-9901e524dcf2})");
}
unsafe impl ::windows::core::Interface for InkInputConfiguration {
    type Vtable = IInkInputConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93a68dc4_0b7b_49d7_b34f_9901e524dcf2);
}
impl ::windows::core::RuntimeName for InkInputConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputConfiguration";
}
impl ::core::convert::From<InkInputConfiguration> for ::windows::core::IUnknown {
    fn from(value: InkInputConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkInputConfiguration> for ::windows::core::IUnknown {
    fn from(value: &InkInputConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkInputConfiguration> for ::windows::core::IInspectable {
    fn from(value: InkInputConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkInputConfiguration> for ::windows::core::IInspectable {
    fn from(value: &InkInputConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkInputConfiguration {}
unsafe impl ::core::marker::Sync for InkInputConfiguration {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkInputProcessingConfiguration(pub ::windows::core::IInspectable);
impl InkInputProcessingConfiguration {
    pub fn Mode(&self) -> ::windows::core::Result<InkInputProcessingMode> {
        let this = self;
        unsafe {
            let mut result__: InkInputProcessingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkInputProcessingMode>(result__)
        }
    }
    pub fn SetMode(&self, value: InkInputProcessingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RightDragAction(&self) -> ::windows::core::Result<InkInputRightDragAction> {
        let this = self;
        unsafe {
            let mut result__: InkInputRightDragAction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkInputRightDragAction>(result__)
        }
    }
    pub fn SetRightDragAction(&self, value: InkInputRightDragAction) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputProcessingConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkInputProcessingConfiguration;{2778d85e-33ca-4b06-a6d3-ac3945116d37})");
}
unsafe impl ::windows::core::Interface for InkInputProcessingConfiguration {
    type Vtable = IInkInputProcessingConfiguration_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2778d85e_33ca_4b06_a6d3_ac3945116d37);
}
impl ::windows::core::RuntimeName for InkInputProcessingConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputProcessingConfiguration";
}
impl ::core::convert::From<InkInputProcessingConfiguration> for ::windows::core::IUnknown {
    fn from(value: InkInputProcessingConfiguration) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkInputProcessingConfiguration> for ::windows::core::IUnknown {
    fn from(value: &InkInputProcessingConfiguration) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkInputProcessingConfiguration> for ::windows::core::IInspectable {
    fn from(value: InkInputProcessingConfiguration) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkInputProcessingConfiguration> for ::windows::core::IInspectable {
    fn from(value: &InkInputProcessingConfiguration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkInputProcessingConfiguration {}
unsafe impl ::core::marker::Sync for InkInputProcessingConfiguration {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkInputProcessingMode(pub i32);
impl InkInputProcessingMode {
    pub const None: InkInputProcessingMode = InkInputProcessingMode(0i32);
    pub const Inking: InkInputProcessingMode = InkInputProcessingMode(1i32);
    pub const Erasing: InkInputProcessingMode = InkInputProcessingMode(2i32);
}
impl ::core::convert::From<i32> for InkInputProcessingMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkInputProcessingMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkInputProcessingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputProcessingMode;i4)");
}
impl ::windows::core::DefaultType for InkInputProcessingMode {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkInputRightDragAction(pub i32);
impl InkInputRightDragAction {
    pub const LeaveUnprocessed: InkInputRightDragAction = InkInputRightDragAction(0i32);
    pub const AllowProcessing: InkInputRightDragAction = InkInputRightDragAction(1i32);
}
impl ::core::convert::From<i32> for InkInputRightDragAction {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkInputRightDragAction {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkInputRightDragAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputRightDragAction;i4)");
}
impl ::windows::core::DefaultType for InkInputRightDragAction {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkManager(pub ::windows::core::IInspectable);
impl InkManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkManager, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn Mode(&self) -> ::windows::core::Result<InkManipulationMode> {
        let this = self;
        unsafe {
            let mut result__: InkManipulationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkManipulationMode>(result__)
        }
    }
    pub fn SetMode(&self, value: InkManipulationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn ProcessPointerDown<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi()).ok() }
    }
    pub fn ProcessPointerUpdate<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ProcessPointerUp<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn SetDefaultDrawingAttributes<'a, Param0: ::windows::core::IntoParam<'a, InkDrawingAttributes>>(&self, drawingattributes: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), drawingattributes.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RecognizeAsync2(&self, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    pub fn SetDefaultRecognizer<'a, Param0: ::windows::core::IntoParam<'a, InkRecognizer>>(&self, recognizer: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), recognizer.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RecognizeAsync<'a, Param0: ::windows::core::IntoParam<'a, InkStrokeContainer>>(&self, strokecollection: Param0, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = &::windows::core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), strokecollection.into_param().abi(), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = &::windows::core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn AddStroke<'a, Param0: ::windows::core::IntoParam<'a, InkStroke>>(&self, stroke: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MoveSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, translation: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), translation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SelectWithPolyLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, polyline: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), polyline.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectWithLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, from: Param0, to: Param1) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), from.into_param().abi(), to.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PasteFromClipboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, inputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(&self, recognitionresults: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), recognitionresults.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkManager;{4744737d-671b-4163-9c95-4e8d7a035fe1})");
}
unsafe impl ::windows::core::Interface for InkManager {
    type Vtable = IInkManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4744737d_671b_4163_9c95_4e8d7a035fe1);
}
impl ::windows::core::RuntimeName for InkManager {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkManager";
}
impl ::core::convert::From<InkManager> for ::windows::core::IUnknown {
    fn from(value: InkManager) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkManager> for ::windows::core::IUnknown {
    fn from(value: &InkManager) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkManager> for ::windows::core::IInspectable {
    fn from(value: InkManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkManager> for ::windows::core::IInspectable {
    fn from(value: &InkManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkManager> for IInkRecognizerContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: InkManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkManager> for IInkRecognizerContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkRecognizerContainer> for InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, IInkRecognizerContainer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkRecognizerContainer> for &InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, IInkRecognizerContainer> {
        ::core::convert::TryInto::<IInkRecognizerContainer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::convert::TryFrom<InkManager> for IInkStrokeContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: InkManager) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkManager> for IInkStrokeContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkManager) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkStrokeContainer> for InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, IInkStrokeContainer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkStrokeContainer> for &InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, IInkStrokeContainer> {
        ::core::convert::TryInto::<IInkStrokeContainer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkManipulationMode(pub i32);
impl InkManipulationMode {
    pub const Inking: InkManipulationMode = InkManipulationMode(0i32);
    pub const Erasing: InkManipulationMode = InkManipulationMode(1i32);
    pub const Selecting: InkManipulationMode = InkManipulationMode(2i32);
}
impl ::core::convert::From<i32> for InkManipulationMode {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkManipulationMode {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkManipulationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkManipulationMode;i4)");
}
impl ::windows::core::DefaultType for InkManipulationMode {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkModelerAttributes(pub ::windows::core::IInspectable);
impl InkModelerAttributes {
    #[cfg(feature = "Foundation")]
    pub fn PredictionTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetPredictionTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ScalingFactor(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn SetScalingFactor(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn UseVelocityBasedPressure(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetUseVelocityBasedPressure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InkModelerAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkModelerAttributes;{bad31f27-0cd9-4bfd-b6f3-9e03ba8d7454})");
}
unsafe impl ::windows::core::Interface for InkModelerAttributes {
    type Vtable = IInkModelerAttributes_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad31f27_0cd9_4bfd_b6f3_9e03ba8d7454);
}
impl ::windows::core::RuntimeName for InkModelerAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkModelerAttributes";
}
impl ::core::convert::From<InkModelerAttributes> for ::windows::core::IUnknown {
    fn from(value: InkModelerAttributes) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkModelerAttributes> for ::windows::core::IUnknown {
    fn from(value: &InkModelerAttributes) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkModelerAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkModelerAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkModelerAttributes> for ::windows::core::IInspectable {
    fn from(value: InkModelerAttributes) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkModelerAttributes> for ::windows::core::IInspectable {
    fn from(value: &InkModelerAttributes) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkModelerAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkModelerAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkModelerAttributes {}
unsafe impl ::core::marker::Sync for InkModelerAttributes {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPersistenceFormat(pub i32);
impl InkPersistenceFormat {
    pub const GifWithEmbeddedIsf: InkPersistenceFormat = InkPersistenceFormat(0i32);
    pub const Isf: InkPersistenceFormat = InkPersistenceFormat(1i32);
}
impl ::core::convert::From<i32> for InkPersistenceFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkPersistenceFormat {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkPersistenceFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPersistenceFormat;i4)");
}
impl ::windows::core::DefaultType for InkPersistenceFormat {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkPoint(pub ::windows::core::IInspectable);
impl InkPoint {
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateInkPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(position: Param0, pressure: f32) -> ::windows::core::Result<InkPoint> {
        Self::IInkPointFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), pressure, &mut result__).from_abi::<InkPoint>(result__)
        })
    }
    pub fn TiltX(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn TiltY(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateInkPointWithTiltAndTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(position: Param0, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64) -> ::windows::core::Result<InkPoint> {
        Self::IInkPointFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), pressure, tiltx, tilty, timestamp, &mut result__).from_abi::<InkPoint>(result__)
        })
    }
    pub fn IInkPointFactory<R, F: FnOnce(&IInkPointFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkPoint, IInkPointFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IInkPointFactory2<R, F: FnOnce(&IInkPointFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkPoint, IInkPointFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InkPoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPoint;{9f87272b-858c-46a5-9b41-d195970459fd})");
}
unsafe impl ::windows::core::Interface for InkPoint {
    type Vtable = IInkPoint_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f87272b_858c_46a5_9b41_d195970459fd);
}
impl ::windows::core::RuntimeName for InkPoint {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPoint";
}
impl ::core::convert::From<InkPoint> for ::windows::core::IUnknown {
    fn from(value: InkPoint) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkPoint> for ::windows::core::IUnknown {
    fn from(value: &InkPoint) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkPoint> for ::windows::core::IInspectable {
    fn from(value: InkPoint) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkPoint> for ::windows::core::IInspectable {
    fn from(value: &InkPoint) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkPoint {}
unsafe impl ::core::marker::Sync for InkPoint {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkPresenter(pub ::windows::core::IInspectable);
impl InkPresenter {
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "UI_Core")]
    pub fn InputDeviceTypes(&self) -> ::windows::core::Result<super::super::Core::CoreInputDeviceTypes> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CoreInputDeviceTypes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreInputDeviceTypes>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn SetInputDeviceTypes(&self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn UnprocessedInput(&self) -> ::windows::core::Result<InkUnprocessedInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkUnprocessedInput>(result__)
        }
    }
    pub fn StrokeInput(&self) -> ::windows::core::Result<InkStrokeInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkStrokeInput>(result__)
        }
    }
    pub fn InputProcessingConfiguration(&self) -> ::windows::core::Result<InkInputProcessingConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkInputProcessingConfiguration>(result__)
        }
    }
    pub fn StrokeContainer(&self) -> ::windows::core::Result<InkStrokeContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkStrokeContainer>(result__)
        }
    }
    pub fn SetStrokeContainer<'a, Param0: ::windows::core::IntoParam<'a, InkStrokeContainer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn CopyDefaultDrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributes>(result__)
        }
    }
    pub fn UpdateDefaultDrawingAttributes<'a, Param0: ::windows::core::IntoParam<'a, InkDrawingAttributes>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ActivateCustomDrying(&self) -> ::windows::core::Result<InkSynchronizer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkSynchronizer>(result__)
        }
    }
    pub fn SetPredefinedConfiguration(&self, value: InkPresenterPredefinedConfiguration) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StrokesCollected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokesCollected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StrokesErased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokesErased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn HighContrastAdjustment(&self) -> ::windows::core::Result<InkHighContrastAdjustment> {
        let this = &::windows::core::Interface::cast::<IInkPresenter2>(self)?;
        unsafe {
            let mut result__: InkHighContrastAdjustment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkHighContrastAdjustment>(result__)
        }
    }
    pub fn SetHighContrastAdjustment(&self, value: InkHighContrastAdjustment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn InputConfiguration(&self) -> ::windows::core::Result<InkInputConfiguration> {
        let this = &::windows::core::Interface::cast::<IInkPresenter3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkInputConfiguration>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenter;{a69b70e2-887b-458f-b173-4fe4438930a3})");
}
unsafe impl ::windows::core::Interface for InkPresenter {
    type Vtable = IInkPresenter_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa69b70e2_887b_458f_b173_4fe4438930a3);
}
impl ::windows::core::RuntimeName for InkPresenter {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenter";
}
impl ::core::convert::From<InkPresenter> for ::windows::core::IUnknown {
    fn from(value: InkPresenter) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkPresenter> for ::windows::core::IUnknown {
    fn from(value: &InkPresenter) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkPresenter> for ::windows::core::IInspectable {
    fn from(value: InkPresenter) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkPresenter> for ::windows::core::IInspectable {
    fn from(value: &InkPresenter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkPresenter {}
unsafe impl ::core::marker::Sync for InkPresenter {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPresenterPredefinedConfiguration(pub i32);
impl InkPresenterPredefinedConfiguration {
    pub const SimpleSinglePointer: InkPresenterPredefinedConfiguration = InkPresenterPredefinedConfiguration(0i32);
    pub const SimpleMultiplePointer: InkPresenterPredefinedConfiguration = InkPresenterPredefinedConfiguration(1i32);
}
impl ::core::convert::From<i32> for InkPresenterPredefinedConfiguration {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkPresenterPredefinedConfiguration {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkPresenterPredefinedConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterPredefinedConfiguration;i4)");
}
impl ::windows::core::DefaultType for InkPresenterPredefinedConfiguration {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkPresenterProtractor(pub ::windows::core::IInspectable);
impl InkPresenterProtractor {
    pub fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: InkPresenterStencilKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AreRaysVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAreRaysVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsCenterMarkerVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCenterMarkerVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsAngleReadoutVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsAngleReadoutVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsResizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsResizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Radius(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetRadius(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn AccentColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetAccentColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, InkPresenter>>(inkpresenter: Param0) -> ::windows::core::Result<InkPresenterProtractor> {
        Self::IInkPresenterProtractorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<InkPresenterProtractor>(result__)
        })
    }
    pub fn IInkPresenterProtractorFactory<R, F: FnOnce(&IInkPresenterProtractorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkPresenterProtractor, IInkPresenterProtractorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterProtractor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenterProtractor;{7de3f2aa-ef6c-4e91-a73b-5b70d56fbd17})");
}
unsafe impl ::windows::core::Interface for InkPresenterProtractor {
    type Vtable = IInkPresenterProtractor_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de3f2aa_ef6c_4e91_a73b_5b70d56fbd17);
}
impl ::windows::core::RuntimeName for InkPresenterProtractor {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterProtractor";
}
impl ::core::convert::From<InkPresenterProtractor> for ::windows::core::IUnknown {
    fn from(value: InkPresenterProtractor) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkPresenterProtractor> for ::windows::core::IUnknown {
    fn from(value: &InkPresenterProtractor) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkPresenterProtractor> for ::windows::core::IInspectable {
    fn from(value: InkPresenterProtractor) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkPresenterProtractor> for ::windows::core::IInspectable {
    fn from(value: &InkPresenterProtractor) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkPresenterProtractor> for IInkPresenterStencil {
    type Error = ::windows::core::Error;
    fn try_from(value: InkPresenterProtractor) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkPresenterProtractor> for IInkPresenterStencil {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkPresenterProtractor) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkPresenterStencil> for InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, IInkPresenterStencil> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkPresenterStencil> for &InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, IInkPresenterStencil> {
        ::core::convert::TryInto::<IInkPresenterStencil>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkPresenterProtractor {}
unsafe impl ::core::marker::Sync for InkPresenterProtractor {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkPresenterRuler(pub ::windows::core::IInspectable);
impl InkPresenterRuler {
    pub fn Length(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Width(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    pub fn SetWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: InkPresenterStencilKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, InkPresenter>>(inkpresenter: Param0) -> ::windows::core::Result<InkPresenterRuler> {
        Self::IInkPresenterRulerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<InkPresenterRuler>(result__)
        })
    }
    pub fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IsCompassVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsCompassVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IInkPresenterRulerFactory<R, F: FnOnce(&IInkPresenterRulerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkPresenterRuler, IInkPresenterRulerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterRuler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenterRuler;{6cda7d5a-dec7-4dd7-877a-2133f183d48a})");
}
unsafe impl ::windows::core::Interface for InkPresenterRuler {
    type Vtable = IInkPresenterRuler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cda7d5a_dec7_4dd7_877a_2133f183d48a);
}
impl ::windows::core::RuntimeName for InkPresenterRuler {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterRuler";
}
impl ::core::convert::From<InkPresenterRuler> for ::windows::core::IUnknown {
    fn from(value: InkPresenterRuler) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkPresenterRuler> for ::windows::core::IUnknown {
    fn from(value: &InkPresenterRuler) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkPresenterRuler> for ::windows::core::IInspectable {
    fn from(value: InkPresenterRuler) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkPresenterRuler> for ::windows::core::IInspectable {
    fn from(value: &InkPresenterRuler) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<InkPresenterRuler> for IInkPresenterStencil {
    type Error = ::windows::core::Error;
    fn try_from(value: InkPresenterRuler) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkPresenterRuler> for IInkPresenterStencil {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkPresenterRuler) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkPresenterStencil> for InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, IInkPresenterStencil> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkPresenterStencil> for &InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, IInkPresenterStencil> {
        ::core::convert::TryInto::<IInkPresenterStencil>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for InkPresenterRuler {}
unsafe impl ::core::marker::Sync for InkPresenterRuler {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkPresenterStencilKind(pub i32);
impl InkPresenterStencilKind {
    pub const Other: InkPresenterStencilKind = InkPresenterStencilKind(0i32);
    pub const Ruler: InkPresenterStencilKind = InkPresenterStencilKind(1i32);
    pub const Protractor: InkPresenterStencilKind = InkPresenterStencilKind(2i32);
}
impl ::core::convert::From<i32> for InkPresenterStencilKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkPresenterStencilKind {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkPresenterStencilKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterStencilKind;i4)");
}
impl ::windows::core::DefaultType for InkPresenterStencilKind {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkRecognitionResult(pub ::windows::core::IInspectable);
impl InkRecognitionResult {
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextCandidates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognitionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognitionResult;{36461a94-5068-40ef-8a05-2c2fb60908a2})");
}
unsafe impl ::windows::core::Interface for InkRecognitionResult {
    type Vtable = IInkRecognitionResult_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36461a94_5068_40ef_8a05_2c2fb60908a2);
}
impl ::windows::core::RuntimeName for InkRecognitionResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognitionResult";
}
impl ::core::convert::From<InkRecognitionResult> for ::windows::core::IUnknown {
    fn from(value: InkRecognitionResult) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkRecognitionResult> for ::windows::core::IUnknown {
    fn from(value: &InkRecognitionResult) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkRecognitionResult> for ::windows::core::IInspectable {
    fn from(value: InkRecognitionResult) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkRecognitionResult> for ::windows::core::IInspectable {
    fn from(value: &InkRecognitionResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkRecognitionResult {}
unsafe impl ::core::marker::Sync for InkRecognitionResult {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct InkRecognitionTarget(pub i32);
impl InkRecognitionTarget {
    pub const All: InkRecognitionTarget = InkRecognitionTarget(0i32);
    pub const Selected: InkRecognitionTarget = InkRecognitionTarget(1i32);
    pub const Recent: InkRecognitionTarget = InkRecognitionTarget(2i32);
}
impl ::core::convert::From<i32> for InkRecognitionTarget {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for InkRecognitionTarget {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for InkRecognitionTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkRecognitionTarget;i4)");
}
impl ::windows::core::DefaultType for InkRecognitionTarget {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkRecognizer(pub ::windows::core::IInspectable);
impl InkRecognizer {
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognizer;{077ccea3-904d-442a-b151-aaca3631c43b})");
}
unsafe impl ::windows::core::Interface for InkRecognizer {
    type Vtable = IInkRecognizer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x077ccea3_904d_442a_b151_aaca3631c43b);
}
impl ::windows::core::RuntimeName for InkRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizer";
}
impl ::core::convert::From<InkRecognizer> for ::windows::core::IUnknown {
    fn from(value: InkRecognizer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkRecognizer> for ::windows::core::IUnknown {
    fn from(value: &InkRecognizer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkRecognizer> for ::windows::core::IInspectable {
    fn from(value: InkRecognizer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkRecognizer> for ::windows::core::IInspectable {
    fn from(value: &InkRecognizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkRecognizerContainer(pub ::windows::core::IInspectable);
impl InkRecognizerContainer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkRecognizerContainer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn SetDefaultRecognizer<'a, Param0: ::windows::core::IntoParam<'a, InkRecognizer>>(&self, recognizer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), recognizer.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RecognizeAsync<'a, Param0: ::windows::core::IntoParam<'a, InkStrokeContainer>>(&self, strokecollection: Param0, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), strokecollection.into_param().abi(), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognizerContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognizerContainer;{a74d9a31-8047-4698-a912-f82a5085012f})");
}
unsafe impl ::windows::core::Interface for InkRecognizerContainer {
    type Vtable = IInkRecognizerContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74d9a31_8047_4698_a912_f82a5085012f);
}
impl ::windows::core::RuntimeName for InkRecognizerContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizerContainer";
}
impl ::core::convert::From<InkRecognizerContainer> for ::windows::core::IUnknown {
    fn from(value: InkRecognizerContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkRecognizerContainer> for ::windows::core::IUnknown {
    fn from(value: &InkRecognizerContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkRecognizerContainer> for ::windows::core::IInspectable {
    fn from(value: InkRecognizerContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkRecognizerContainer> for ::windows::core::IInspectable {
    fn from(value: &InkRecognizerContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InkRecognizerContainer> for IInkRecognizerContainer {
    fn from(value: InkRecognizerContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkRecognizerContainer> for IInkRecognizerContainer {
    fn from(value: &InkRecognizerContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkRecognizerContainer> for InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IInkRecognizerContainer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkRecognizerContainer> for &InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IInkRecognizerContainer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkStroke(pub ::windows::core::IInspectable);
impl InkStroke {
    pub fn DrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributes>(result__)
        }
    }
    pub fn SetDrawingAttributes<'a, Param0: ::windows::core::IntoParam<'a, InkDrawingAttributes>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn Selected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetSelected(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn Recognized(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRenderingSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>>(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PointTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPointTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStroke2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInkPoints(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkPoint>> {
        let this = &::windows::core::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkPoint>>(result__)
        }
    }
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn StrokeStartedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStrokeStartedTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn StrokeDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SetStrokeDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkStroke4>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkStroke {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStroke;{15144d60-cce3-4fcf-9d52-11518ab6afd4})");
}
unsafe impl ::windows::core::Interface for InkStroke {
    type Vtable = IInkStroke_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15144d60_cce3_4fcf_9d52_11518ab6afd4);
}
impl ::windows::core::RuntimeName for InkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStroke";
}
impl ::core::convert::From<InkStroke> for ::windows::core::IUnknown {
    fn from(value: InkStroke) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkStroke> for ::windows::core::IUnknown {
    fn from(value: &InkStroke) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkStroke> for ::windows::core::IInspectable {
    fn from(value: InkStroke) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkStroke> for ::windows::core::IInspectable {
    fn from(value: &InkStroke) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkStroke {}
unsafe impl ::core::marker::Sync for InkStroke {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkStrokeBuilder(pub ::windows::core::IInspectable);
impl InkStrokeBuilder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkStrokeBuilder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn BeginStroke<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi()).ok() }
    }
    pub fn AppendToStroke<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<super::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<super::PointerPoint>(result__)
        }
    }
    pub fn EndStroke<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn CreateStroke<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, points: Param0) -> ::windows::core::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), points.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    pub fn SetDefaultDrawingAttributes<'a, Param0: ::windows::core::IntoParam<'a, InkDrawingAttributes>>(&self, drawingattributes: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), drawingattributes.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub fn CreateStrokeFromInkPoints<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InkPoint>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, inkpoints: Param0, transform: Param1) -> ::windows::core::Result<InkStroke> {
        let this = &::windows::core::Interface::cast::<IInkStrokeBuilder2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpoints.into_param().abi(), transform.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub fn CreateStrokeFromInkPoints2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InkPoint>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, inkpoints: Param0, transform: Param1, strokestartedtime: Param2, strokeduration: Param3) -> ::windows::core::Result<InkStroke> {
        let this = &::windows::core::Interface::cast::<IInkStrokeBuilder3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpoints.into_param().abi(), transform.into_param().abi(), strokestartedtime.into_param().abi(), strokeduration.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeBuilder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeBuilder;{82bbd1dc-1c63-41dc-9e07-4b4a70ced801})");
}
unsafe impl ::windows::core::Interface for InkStrokeBuilder {
    type Vtable = IInkStrokeBuilder_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82bbd1dc_1c63_41dc_9e07_4b4a70ced801);
}
impl ::windows::core::RuntimeName for InkStrokeBuilder {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeBuilder";
}
impl ::core::convert::From<InkStrokeBuilder> for ::windows::core::IUnknown {
    fn from(value: InkStrokeBuilder) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkStrokeBuilder> for ::windows::core::IUnknown {
    fn from(value: &InkStrokeBuilder) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokeBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkStrokeBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkStrokeBuilder> for ::windows::core::IInspectable {
    fn from(value: InkStrokeBuilder) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkStrokeBuilder> for ::windows::core::IInspectable {
    fn from(value: &InkStrokeBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokeBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkStrokeBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkStrokeContainer(pub ::windows::core::IInspectable);
impl InkStrokeContainer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkStrokeContainer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn AddStroke<'a, Param0: ::windows::core::IntoParam<'a, InkStroke>>(&self, stroke: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn MoveSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, translation: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), translation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SelectWithPolyLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, polyline: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), polyline.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SelectWithLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, from: Param0, to: Param1) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), from.into_param().abi(), to.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    pub fn PasteFromClipboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    pub fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, inputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(&self, recognitionresults: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), recognitionresults.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddStrokes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InkStroke>>>(&self, strokes: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), strokes.into_param().abi()).ok() }
    }
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveWithFormatAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0, inkpersistenceformat: InkPersistenceFormat) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), inkpersistenceformat, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    pub fn GetStrokeById(&self, id: u32) -> ::windows::core::Result<InkStroke> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id, &mut result__).from_abi::<InkStroke>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeContainer;{22accbc6-faa9-4f14-b68c-f6cee670ae16})");
}
unsafe impl ::windows::core::Interface for InkStrokeContainer {
    type Vtable = IInkStrokeContainer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22accbc6_faa9_4f14_b68c_f6cee670ae16);
}
impl ::windows::core::RuntimeName for InkStrokeContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeContainer";
}
impl ::core::convert::From<InkStrokeContainer> for ::windows::core::IUnknown {
    fn from(value: InkStrokeContainer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkStrokeContainer> for ::windows::core::IUnknown {
    fn from(value: &InkStrokeContainer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkStrokeContainer> for ::windows::core::IInspectable {
    fn from(value: InkStrokeContainer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkStrokeContainer> for ::windows::core::IInspectable {
    fn from(value: &InkStrokeContainer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<InkStrokeContainer> for IInkStrokeContainer {
    fn from(value: InkStrokeContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokeContainer> for IInkStrokeContainer {
    fn from(value: &InkStrokeContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkStrokeContainer> for InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IInkStrokeContainer> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkStrokeContainer> for &InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IInkStrokeContainer> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkStrokeInput(pub ::windows::core::IInspectable);
impl InkStrokeInput {
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeContinued<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeContinued<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenter>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeInput {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeInput;{cf2ffe7b-5e10-43c6-a080-88f26e1dc67d})");
}
unsafe impl ::windows::core::Interface for InkStrokeInput {
    type Vtable = IInkStrokeInput_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2ffe7b_5e10_43c6_a080_88f26e1dc67d);
}
impl ::windows::core::RuntimeName for InkStrokeInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeInput";
}
impl ::core::convert::From<InkStrokeInput> for ::windows::core::IUnknown {
    fn from(value: InkStrokeInput) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkStrokeInput> for ::windows::core::IUnknown {
    fn from(value: &InkStrokeInput) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokeInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkStrokeInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkStrokeInput> for ::windows::core::IInspectable {
    fn from(value: InkStrokeInput) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkStrokeInput> for ::windows::core::IInspectable {
    fn from(value: &InkStrokeInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokeInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkStrokeInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkStrokeInput {}
unsafe impl ::core::marker::Sync for InkStrokeInput {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkStrokeRenderingSegment(pub ::windows::core::IInspectable);
impl InkStrokeRenderingSegment {
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BezierControlPoint1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn BezierControlPoint2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    pub fn Pressure(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn TiltX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn TiltY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    pub fn Twist(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeRenderingSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeRenderingSegment;{68510f1f-88e3-477a-a2fa-569f5f1f9bd5})");
}
unsafe impl ::windows::core::Interface for InkStrokeRenderingSegment {
    type Vtable = IInkStrokeRenderingSegment_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68510f1f_88e3_477a_a2fa_569f5f1f9bd5);
}
impl ::windows::core::RuntimeName for InkStrokeRenderingSegment {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeRenderingSegment";
}
impl ::core::convert::From<InkStrokeRenderingSegment> for ::windows::core::IUnknown {
    fn from(value: InkStrokeRenderingSegment) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkStrokeRenderingSegment> for ::windows::core::IUnknown {
    fn from(value: &InkStrokeRenderingSegment) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkStrokeRenderingSegment> for ::windows::core::IInspectable {
    fn from(value: InkStrokeRenderingSegment) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkStrokeRenderingSegment> for ::windows::core::IInspectable {
    fn from(value: &InkStrokeRenderingSegment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkStrokeRenderingSegment {}
unsafe impl ::core::marker::Sync for InkStrokeRenderingSegment {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkStrokesCollectedEventArgs(pub ::windows::core::IInspectable);
impl InkStrokesCollectedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokesCollectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokesCollectedEventArgs;{c4f3f229-1938-495c-b4d9-6de4b08d4811})");
}
unsafe impl ::windows::core::Interface for InkStrokesCollectedEventArgs {
    type Vtable = IInkStrokesCollectedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4f3f229_1938_495c_b4d9_6de4b08d4811);
}
impl ::windows::core::RuntimeName for InkStrokesCollectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesCollectedEventArgs";
}
impl ::core::convert::From<InkStrokesCollectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: InkStrokesCollectedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkStrokesCollectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InkStrokesCollectedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkStrokesCollectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: InkStrokesCollectedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkStrokesCollectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InkStrokesCollectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkStrokesErasedEventArgs(pub ::windows::core::IInspectable);
impl InkStrokesErasedEventArgs {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokesErasedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokesErasedEventArgs;{a4216a22-1503-4ebf-8ff5-2de84584a8aa})");
}
unsafe impl ::windows::core::Interface for InkStrokesErasedEventArgs {
    type Vtable = IInkStrokesErasedEventArgs_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4216a22_1503_4ebf_8ff5_2de84584a8aa);
}
impl ::windows::core::RuntimeName for InkStrokesErasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesErasedEventArgs";
}
impl ::core::convert::From<InkStrokesErasedEventArgs> for ::windows::core::IUnknown {
    fn from(value: InkStrokesErasedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkStrokesErasedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InkStrokesErasedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkStrokesErasedEventArgs> for ::windows::core::IInspectable {
    fn from(value: InkStrokesErasedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkStrokesErasedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InkStrokesErasedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkSynchronizer(pub ::windows::core::IInspectable);
impl InkSynchronizer {
    #[cfg(feature = "Foundation_Collections")]
    pub fn BeginDry(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    pub fn EndDry(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for InkSynchronizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkSynchronizer;{9b9ea160-ae9b-45f9-8407-4b493b163661})");
}
unsafe impl ::windows::core::Interface for InkSynchronizer {
    type Vtable = IInkSynchronizer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b9ea160_ae9b_45f9_8407_4b493b163661);
}
impl ::windows::core::RuntimeName for InkSynchronizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkSynchronizer";
}
impl ::core::convert::From<InkSynchronizer> for ::windows::core::IUnknown {
    fn from(value: InkSynchronizer) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkSynchronizer> for ::windows::core::IUnknown {
    fn from(value: &InkSynchronizer) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkSynchronizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkSynchronizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkSynchronizer> for ::windows::core::IInspectable {
    fn from(value: InkSynchronizer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkSynchronizer> for ::windows::core::IInspectable {
    fn from(value: &InkSynchronizer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkSynchronizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkSynchronizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct InkUnprocessedInput(pub ::windows::core::IInspectable);
impl InkUnprocessedInput {
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerHovered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerHovered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    pub fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenter>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for InkUnprocessedInput {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkUnprocessedInput;{db4445e0-8398-4921-ac3b-ab978c5ba256})");
}
unsafe impl ::windows::core::Interface for InkUnprocessedInput {
    type Vtable = IInkUnprocessedInput_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb4445e0_8398_4921_ac3b_ab978c5ba256);
}
impl ::windows::core::RuntimeName for InkUnprocessedInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkUnprocessedInput";
}
impl ::core::convert::From<InkUnprocessedInput> for ::windows::core::IUnknown {
    fn from(value: InkUnprocessedInput) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&InkUnprocessedInput> for ::windows::core::IUnknown {
    fn from(value: &InkUnprocessedInput) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkUnprocessedInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a InkUnprocessedInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<InkUnprocessedInput> for ::windows::core::IInspectable {
    fn from(value: InkUnprocessedInput) -> Self {
        value.0
    }
}
impl ::core::convert::From<&InkUnprocessedInput> for ::windows::core::IInspectable {
    fn from(value: &InkUnprocessedInput) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkUnprocessedInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a InkUnprocessedInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for InkUnprocessedInput {}
unsafe impl ::core::marker::Sync for InkUnprocessedInput {}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PenAndInkSettings(pub ::windows::core::IInspectable);
impl PenAndInkSettings {
    pub fn IsHandwritingDirectlyIntoTextFieldEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn PenHandedness(&self) -> ::windows::core::Result<PenHandedness> {
        let this = self;
        unsafe {
            let mut result__: PenHandedness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PenHandedness>(result__)
        }
    }
    pub fn HandwritingLineHeight(&self) -> ::windows::core::Result<HandwritingLineHeight> {
        let this = self;
        unsafe {
            let mut result__: HandwritingLineHeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HandwritingLineHeight>(result__)
        }
    }
    pub fn FontFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn UserConsentsToHandwritingTelemetryCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn IsTouchHandwritingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::core::Result<PenAndInkSettings> {
        Self::IPenAndInkSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PenAndInkSettings>(result__)
        })
    }
    pub fn SetPenHandedness(&self, value: PenHandedness) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPenAndInkSettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn IPenAndInkSettingsStatics<R, F: FnOnce(&IPenAndInkSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PenAndInkSettings, IPenAndInkSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for PenAndInkSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.PenAndInkSettings;{bc2ceb8f-0066-44a8-bb7a-b839b3deb8f5})");
}
unsafe impl ::windows::core::Interface for PenAndInkSettings {
    type Vtable = IPenAndInkSettings_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc2ceb8f_0066_44a8_bb7a_b839b3deb8f5);
}
impl ::windows::core::RuntimeName for PenAndInkSettings {
    const NAME: &'static str = "Windows.UI.Input.Inking.PenAndInkSettings";
}
impl ::core::convert::From<PenAndInkSettings> for ::windows::core::IUnknown {
    fn from(value: PenAndInkSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PenAndInkSettings> for ::windows::core::IUnknown {
    fn from(value: &PenAndInkSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PenAndInkSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PenAndInkSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PenAndInkSettings> for ::windows::core::IInspectable {
    fn from(value: PenAndInkSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PenAndInkSettings> for ::windows::core::IInspectable {
    fn from(value: &PenAndInkSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PenAndInkSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PenAndInkSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for PenAndInkSettings {}
unsafe impl ::core::marker::Sync for PenAndInkSettings {}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PenHandedness(pub i32);
impl PenHandedness {
    pub const Right: PenHandedness = PenHandedness(0i32);
    pub const Left: PenHandedness = PenHandedness(1i32);
}
impl ::core::convert::From<i32> for PenHandedness {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PenHandedness {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PenHandedness {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenHandedness;i4)");
}
impl ::windows::core::DefaultType for PenHandedness {
    type DefaultType = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PenTipShape(pub i32);
impl PenTipShape {
    pub const Circle: PenTipShape = PenTipShape(0i32);
    pub const Rectangle: PenTipShape = PenTipShape(1i32);
}
impl ::core::convert::From<i32> for PenTipShape {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PenTipShape {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PenTipShape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenTipShape;i4)");
}
impl ::windows::core::DefaultType for PenTipShape {
    type DefaultType = Self;
}
