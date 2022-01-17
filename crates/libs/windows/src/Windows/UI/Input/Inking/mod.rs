#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_Input_Inking_Analysis")]
pub mod Analysis;
#[cfg(feature = "UI_Input_Inking_Core")]
pub mod Core;
#[cfg(feature = "UI_Input_Inking_Preview")]
pub mod Preview;
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HandwritingLineHeight(pub i32);
impl HandwritingLineHeight {
    pub const Small: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const Large: Self = Self(2i32);
}
impl ::core::marker::Copy for HandwritingLineHeight {}
impl ::core::clone::Clone for HandwritingLineHeight {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for HandwritingLineHeight {
    type Abi = Self;
}
impl ::core::fmt::Debug for HandwritingLineHeight {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HandwritingLineHeight").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HandwritingLineHeight {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.HandwritingLineHeight;i4)");
}
impl ::windows::core::DefaultType for HandwritingLineHeight {
    type DefaultType = Self;
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes {
    type Vtable = IInkDrawingAttributesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97a2176c_6774_48ad_84f0_48f5a9be74f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PenTipShape) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PenTipShape) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes2 {
    type Vtable = IInkDrawingAttributes2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7cab6508_8ec4_42fd_a5a5_e4b7d1d5316d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes3 {
    type Vtable = IInkDrawingAttributes3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72020002_7d5b_4690_8af4_e664cbe2b74f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkDrawingAttributesKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes4 {
    type Vtable = IInkDrawingAttributes4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef65dc25_9f19_456d_91a3_bc3a3d91c5fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributes5(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkDrawingAttributes5 {
    type Vtable = IInkDrawingAttributes5Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd11aa0bb_0775_4852_ae64_41143a7ae6c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributes5Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributesPencilProperties(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkDrawingAttributesPencilProperties {
    type Vtable = IInkDrawingAttributesPencilPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f2534cb_2d86_41bb_b0e8_e4c2a0253c52);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributesPencilPropertiesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkDrawingAttributesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkDrawingAttributesStatics {
    type Vtable = IInkDrawingAttributesStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf731e03f_1a65_4862_96cb_6e1665e17f6d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkDrawingAttributesStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkInputConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkInputConfiguration {
    type Vtable = IInkInputConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93a68dc4_0b7b_49d7_b34f_9901e524dcf2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkInputConfiguration2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkInputConfiguration2 {
    type Vtable = IInkInputConfiguration2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ac2272e_81b4_5cc4_a36d_d057c387dfda);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputConfiguration2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkInputProcessingConfiguration(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkInputProcessingConfiguration {
    type Vtable = IInkInputProcessingConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2778d85e_33ca_4b06_a6d3_ac3945116d37);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkInputProcessingConfigurationVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkInputProcessingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkInputProcessingMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkInputRightDragAction) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkInputRightDragAction) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkManager {
    type Vtable = IInkManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4744737d_671b_4163_9c95_4e8d7a035fe1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkManipulationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkManipulationMode) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkModelerAttributes(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkModelerAttributes {
    type Vtable = IInkModelerAttributesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad31f27_0cd9_4bfd_b6f3_9e03ba8d7454);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkModelerAttributesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkModelerAttributes2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkModelerAttributes2 {
    type Vtable = IInkModelerAttributes2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86d1d09a_4ef8_5e25_b7bc_b65424f16bb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkModelerAttributes2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPoint(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPoint {
    type Vtable = IInkPointVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f87272b_858c_46a5_9b41_d195970459fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPointVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPoint2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPoint2 {
    type Vtable = IInkPoint2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfba9c3f7_ae56_4d5c_bd2f_0ac45f5e4af9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPoint2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct IInkPointFactory(::windows::core::IUnknown);
impl IInkPointFactory {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInkPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0, pressure: f32) -> ::windows::core::Result<InkPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), pressure, &mut result__).from_abi::<InkPoint>(result__)
        }
    }
}
impl ::core::convert::From<IInkPointFactory> for ::windows::core::IInspectable {
    fn from(value: IInkPointFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkPointFactory> for ::windows::core::IInspectable {
    fn from(value: &IInkPointFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkPointFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IInkPointFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInkPointFactory> for ::windows::core::IUnknown {
    fn from(value: IInkPointFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkPointFactory> for ::windows::core::IUnknown {
    fn from(value: &IInkPointFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkPointFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkPointFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkPointFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkPointFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkPointFactory {}
impl ::core::fmt::Debug for IInkPointFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPointFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkPointFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{29e5d51c-c98f-405d-9f3b-e53e31068d4d}");
}
unsafe impl ::windows::core::Interface for IInkPointFactory {
    type Vtable = IInkPointFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x29e5d51c_c98f_405d_9f3b_e53e31068d4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPointFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPointFactory2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPointFactory2 {
    type Vtable = IInkPointFactory2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0145e85_daff_45f2_ad69_050d8256a209);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPointFactory2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPresenter {
    type Vtable = IInkPresenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa69b70e2_887b_458f_b173_4fe4438930a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    #[cfg(feature = "UI_Core")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkPresenterPredefinedConfiguration) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenter2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPresenter2 {
    type Vtable = IInkPresenter2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf53e612_9a34_11e6_9f33_a24fc0d9649c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkHighContrastAdjustment) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: InkHighContrastAdjustment) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenter3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPresenter3 {
    type Vtable = IInkPresenter3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51e1ce89_d37d_4a90_83fc_7f5e9dfbf217);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenter3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenterProtractor(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPresenterProtractor {
    type Vtable = IInkPresenterProtractorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de3f2aa_ef6c_4e91_a73b_5b70d56fbd17);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterProtractorVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenterProtractorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPresenterProtractorFactory {
    type Vtable = IInkPresenterProtractorFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x320103c9_68fa_47e9_8127_8370711fc46c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterProtractorFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenterRuler(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPresenterRuler {
    type Vtable = IInkPresenterRulerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cda7d5a_dec7_4dd7_877a_2133f183d48a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRulerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkPresenterRuler2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkPresenterRuler2 {
    type Vtable = IInkPresenterRuler2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45130dc1_bc61_44d4_a423_54712ae671c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRuler2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct IInkPresenterRulerFactory(::windows::core::IUnknown);
impl IInkPresenterRulerFactory {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, InkPresenter>>(&self, inkpresenter: Param0) -> ::windows::core::Result<InkPresenterRuler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<InkPresenterRuler>(result__)
        }
    }
}
impl ::core::convert::From<IInkPresenterRulerFactory> for ::windows::core::IInspectable {
    fn from(value: IInkPresenterRulerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkPresenterRulerFactory> for ::windows::core::IInspectable {
    fn from(value: &IInkPresenterRulerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInkPresenterRulerFactory> for ::windows::core::IUnknown {
    fn from(value: IInkPresenterRulerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkPresenterRulerFactory> for ::windows::core::IUnknown {
    fn from(value: &IInkPresenterRulerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkPresenterRulerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkPresenterRulerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkPresenterRulerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkPresenterRulerFactory {}
impl ::core::fmt::Debug for IInkPresenterRulerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPresenterRulerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkPresenterRulerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{34361beb-9001-4a4b-a690-69dbaf63e501}");
}
unsafe impl ::windows::core::Interface for IInkPresenterRulerFactory {
    type Vtable = IInkPresenterRulerFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34361beb_9001_4a4b_a690_69dbaf63e501);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterRulerFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpresenter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct IInkPresenterStencil(::windows::core::IUnknown);
impl IInkPresenterStencil {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind> {
        let this = self;
        unsafe {
            let mut result__: InkPresenterStencilKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IInkPresenterStencil> for ::windows::core::IInspectable {
    fn from(value: IInkPresenterStencil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkPresenterStencil> for ::windows::core::IInspectable {
    fn from(value: &IInkPresenterStencil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkPresenterStencil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IInkPresenterStencil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInkPresenterStencil> for ::windows::core::IUnknown {
    fn from(value: IInkPresenterStencil) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkPresenterStencil> for ::windows::core::IUnknown {
    fn from(value: &IInkPresenterStencil) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkPresenterStencil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkPresenterStencil {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkPresenterStencil {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkPresenterStencil {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkPresenterStencil {}
impl ::core::fmt::Debug for IInkPresenterStencil {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkPresenterStencil").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkPresenterStencil {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{30d12d6d-3e06-4d02-b116-277fb5d8addc}");
}
unsafe impl ::windows::core::Interface for IInkPresenterStencil {
    type Vtable = IInkPresenterStencilVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30d12d6d_3e06_4d02_b116_277fb5d8addc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkPresenterStencilVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut InkPresenterStencilKind) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkRecognitionResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkRecognitionResult {
    type Vtable = IInkRecognitionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36461a94_5068_40ef_8a05_2c2fb60908a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognitionResultVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkRecognizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkRecognizer {
    type Vtable = IInkRecognizerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x077ccea3_904d_442a_b151_aaca3631c43b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct IInkRecognizerContainer(::windows::core::IUnknown);
impl IInkRecognizerContainer {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetDefaultRecognizer<'a, Param0: ::windows::core::IntoParam<'a, InkRecognizer>>(&self, recognizer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), recognizer.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RecognizeAsync<'a, Param0: ::windows::core::IntoParam<'a, InkStrokeContainer>>(&self, strokecollection: Param0, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), strokecollection.into_param().abi(), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
}
impl ::core::convert::From<IInkRecognizerContainer> for ::windows::core::IInspectable {
    fn from(value: IInkRecognizerContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkRecognizerContainer> for ::windows::core::IInspectable {
    fn from(value: &IInkRecognizerContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IInkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInkRecognizerContainer> for ::windows::core::IUnknown {
    fn from(value: IInkRecognizerContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkRecognizerContainer> for ::windows::core::IUnknown {
    fn from(value: &IInkRecognizerContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkRecognizerContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkRecognizerContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkRecognizerContainer {}
impl ::core::fmt::Debug for IInkRecognizerContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkRecognizerContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkRecognizerContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a74d9a31-8047-4698-a912-f82a5085012f}");
}
unsafe impl ::windows::core::Interface for IInkRecognizerContainer {
    type Vtable = IInkRecognizerContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74d9a31_8047_4698_a912_f82a5085012f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkRecognizerContainerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognizer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokecollection: ::windows::core::RawPtr, recognitiontarget: InkRecognitionTarget, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStroke(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStroke {
    type Vtable = IInkStrokeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15144d60_cce3_4fcf_9d52_11518ab6afd4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStroke2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStroke2 {
    type Vtable = IInkStroke2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5db9e4f4_bafa_4de1_89d3_201b1ed7d89b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStroke3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStroke3 {
    type Vtable = IInkStroke3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a807374_9499_411d_a1c4_68855d03d65f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStroke4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStroke4 {
    type Vtable = IInkStroke4Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcd5b62e5_b6e9_5b91_a577_1921d2348690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStroke4Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeBuilder(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStrokeBuilder {
    type Vtable = IInkStrokeBuilderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82bbd1dc_1c63_41dc_9e07_4b4a70ced801);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilderVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pointerpoint: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, points: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, drawingattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeBuilder2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStrokeBuilder2 {
    type Vtable = IInkStrokeBuilder2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbd82bc27_731f_4cbc_bbbf_6d468044f1e5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: ::windows::core::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Foundation_Numerics")))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeBuilder3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStrokeBuilder3 {
    type Vtable = IInkStrokeBuilder3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb2c71fcd_5472_46b1_a81d_c37a3d169441);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeBuilder3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: ::windows::core::RawPtr, transform: super::super::super::Foundation::Numerics::Matrix3x2, strokestartedtime: ::windows::core::RawPtr, strokeduration: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics")))] usize,
);
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct IInkStrokeContainer(::windows::core::IUnknown);
impl IInkStrokeContainer {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn AddStroke<'a, Param0: ::windows::core::IntoParam<'a, InkStroke>>(&self, stroke: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, translation: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), translation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SelectWithPolyLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, polyline: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), polyline.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectWithLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, from: Param0, to: Param1) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), from.into_param().abi(), to.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PasteFromClipboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, inputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(&self, recognitionresults: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), recognitionresults.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
}
impl ::core::convert::From<IInkStrokeContainer> for ::windows::core::IInspectable {
    fn from(value: IInkStrokeContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkStrokeContainer> for ::windows::core::IInspectable {
    fn from(value: &IInkStrokeContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &IInkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInkStrokeContainer> for ::windows::core::IUnknown {
    fn from(value: IInkStrokeContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInkStrokeContainer> for ::windows::core::IUnknown {
    fn from(value: &IInkStrokeContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IInkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInkStrokeContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInkStrokeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInkStrokeContainer {}
impl ::core::fmt::Debug for IInkStrokeContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInkStrokeContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInkStrokeContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{22accbc6-faa9-4f14-b68c-f6cee670ae16}");
}
unsafe impl ::windows::core::Interface for IInkStrokeContainer {
    type Vtable = IInkStrokeContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22accbc6_faa9_4f14_b68c_f6cee670ae16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stroke: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translation: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, polyline: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, from: super::super::super::Foundation::Point, to: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, position: super::super::super::Foundation::Point, result__: *mut super::super::super::Foundation::Rect) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, recognitionresults: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeContainer2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStrokeContainer2 {
    type Vtable = IInkStrokeContainer2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8901d364_da36_4bcf_9e5c_d195825995b4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strokes: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeContainer3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStrokeContainer3 {
    type Vtable = IInkStrokeContainer3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d07bea5_baea_4c82_a719_7b83da1067d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeContainer3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: ::windows::core::RawPtr, inkpersistenceformat: InkPersistenceFormat, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeInput(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStrokeInput {
    type Vtable = IInkStrokeInputVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2ffe7b_5e10_43c6_a080_88f26e1dc67d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeInputVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokeRenderingSegment(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStrokeRenderingSegment {
    type Vtable = IInkStrokeRenderingSegmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68510f1f_88e3_477a_a2fa_569f5f1f9bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokeRenderingSegmentVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Point) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokesCollectedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStrokesCollectedEventArgs {
    type Vtable = IInkStrokesCollectedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4f3f229_1938_495c_b4d9_6de4b08d4811);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokesCollectedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkStrokesErasedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkStrokesErasedEventArgs {
    type Vtable = IInkStrokesErasedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4216a22_1503_4ebf_8ff5_2de84584a8aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkStrokesErasedEventArgsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkSynchronizer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkSynchronizer {
    type Vtable = IInkSynchronizerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b9ea160_ae9b_45f9_8407_4b493b163661);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkSynchronizerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IInkUnprocessedInput(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IInkUnprocessedInput {
    type Vtable = IInkUnprocessedInputVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb4445e0_8398_4921_ac3b_ab978c5ba256);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkUnprocessedInputVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "UI_Core")))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenAndInkSettings(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenAndInkSettings {
    type Vtable = IPenAndInkSettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc2ceb8f_0066_44a8_bb7a_b839b3deb8f5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettingsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PenHandedness) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut HandwritingLineHeight) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenAndInkSettings2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenAndInkSettings2 {
    type Vtable = IPenAndInkSettings2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3262da53_1f44_55e2_9929_ebf77e5481b8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettings2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PenHandedness) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IPenAndInkSettingsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPenAndInkSettingsStatics {
    type Vtable = IPenAndInkSettingsStaticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed6dd036_5708_5c3c_96db_f2f552eab641);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPenAndInkSettingsStaticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkDrawingAttributes(::windows::core::IUnknown);
impl InkDrawingAttributes {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkDrawingAttributes, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Color(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn PenTip(&self) -> ::windows::core::Result<PenTipShape> {
        let this = self;
        unsafe {
            let mut result__: PenTipShape = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PenTipShape>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetPenTip(&self, value: PenTipShape) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Size>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IgnorePressure(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIgnorePressure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn FitToCurve(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetFitToCurve(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PenTipTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPenTipTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn DrawAsHighlighter(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetDrawAsHighlighter(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Kind(&self) -> ::windows::core::Result<InkDrawingAttributesKind> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__: InkDrawingAttributesKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributesKind>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn PencilProperties(&self) -> ::windows::core::Result<InkDrawingAttributesPencilProperties> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributesPencilProperties>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IgnoreTilt(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIgnoreTilt(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes4>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn ModelerAttributes(&self) -> ::windows::core::Result<InkModelerAttributes> {
        let this = &::windows::core::Interface::cast::<IInkDrawingAttributes5>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkModelerAttributes>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn CreateForPencil() -> ::windows::core::Result<InkDrawingAttributes> {
        Self::IInkDrawingAttributesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributes>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInkDrawingAttributesStatics<R, F: FnOnce(&IInkDrawingAttributesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkDrawingAttributes, IInkDrawingAttributesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InkDrawingAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkDrawingAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkDrawingAttributes {}
impl ::core::fmt::Debug for InkDrawingAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDrawingAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkDrawingAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkDrawingAttributes;{97a2176c-6774-48ad-84f0-48f5a9be74f9})");
}
unsafe impl ::windows::core::Interface for InkDrawingAttributes {
    type Vtable = IInkDrawingAttributesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97a2176c_6774_48ad_84f0_48f5a9be74f9);
}
impl ::windows::core::RuntimeName for InkDrawingAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributes";
}
impl ::core::convert::From<InkDrawingAttributes> for ::windows::core::IUnknown {
    fn from(value: InkDrawingAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkDrawingAttributes> for ::windows::core::IUnknown {
    fn from(value: &InkDrawingAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkDrawingAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkDrawingAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkDrawingAttributes> for ::windows::core::IInspectable {
    fn from(value: InkDrawingAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkDrawingAttributes> for ::windows::core::IInspectable {
    fn from(value: &InkDrawingAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkDrawingAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkDrawingAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkDrawingAttributes {}
unsafe impl ::core::marker::Sync for InkDrawingAttributes {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkDrawingAttributesKind(pub i32);
impl InkDrawingAttributesKind {
    pub const Default: Self = Self(0i32);
    pub const Pencil: Self = Self(1i32);
}
impl ::core::marker::Copy for InkDrawingAttributesKind {}
impl ::core::clone::Clone for InkDrawingAttributesKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for InkDrawingAttributesKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkDrawingAttributesKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDrawingAttributesKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkDrawingAttributesKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkDrawingAttributesKind;i4)");
}
impl ::windows::core::DefaultType for InkDrawingAttributesKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkDrawingAttributesPencilProperties(::windows::core::IUnknown);
impl InkDrawingAttributesPencilProperties {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for InkDrawingAttributesPencilProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkDrawingAttributesPencilProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkDrawingAttributesPencilProperties {}
impl ::core::fmt::Debug for InkDrawingAttributesPencilProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkDrawingAttributesPencilProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkDrawingAttributesPencilProperties {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties;{4f2534cb-2d86-41bb-b0e8-e4c2a0253c52})");
}
unsafe impl ::windows::core::Interface for InkDrawingAttributesPencilProperties {
    type Vtable = IInkDrawingAttributesPencilPropertiesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f2534cb_2d86_41bb_b0e8_e4c2a0253c52);
}
impl ::windows::core::RuntimeName for InkDrawingAttributesPencilProperties {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkDrawingAttributesPencilProperties";
}
impl ::core::convert::From<InkDrawingAttributesPencilProperties> for ::windows::core::IUnknown {
    fn from(value: InkDrawingAttributesPencilProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkDrawingAttributesPencilProperties> for ::windows::core::IUnknown {
    fn from(value: &InkDrawingAttributesPencilProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkDrawingAttributesPencilProperties> for ::windows::core::IInspectable {
    fn from(value: InkDrawingAttributesPencilProperties) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkDrawingAttributesPencilProperties> for ::windows::core::IInspectable {
    fn from(value: &InkDrawingAttributesPencilProperties) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkDrawingAttributesPencilProperties {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkDrawingAttributesPencilProperties {}
unsafe impl ::core::marker::Sync for InkDrawingAttributesPencilProperties {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkHighContrastAdjustment(pub i32);
impl InkHighContrastAdjustment {
    pub const UseSystemColorsWhenNecessary: Self = Self(0i32);
    pub const UseSystemColors: Self = Self(1i32);
    pub const UseOriginalColors: Self = Self(2i32);
}
impl ::core::marker::Copy for InkHighContrastAdjustment {}
impl ::core::clone::Clone for InkHighContrastAdjustment {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for InkHighContrastAdjustment {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkHighContrastAdjustment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkHighContrastAdjustment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkHighContrastAdjustment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkHighContrastAdjustment;i4)");
}
impl ::windows::core::DefaultType for InkHighContrastAdjustment {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkInputConfiguration(::windows::core::IUnknown);
impl InkInputConfiguration {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsPrimaryBarrelButtonInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsPrimaryBarrelButtonInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsEraserInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsEraserInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsPenHapticFeedbackEnabled(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsPenHapticFeedbackEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkInputConfiguration2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for InkInputConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkInputConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkInputConfiguration {}
impl ::core::fmt::Debug for InkInputConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInputConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkInputConfiguration;{93a68dc4-0b7b-49d7-b34f-9901e524dcf2})");
}
unsafe impl ::windows::core::Interface for InkInputConfiguration {
    type Vtable = IInkInputConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93a68dc4_0b7b_49d7_b34f_9901e524dcf2);
}
impl ::windows::core::RuntimeName for InkInputConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputConfiguration";
}
impl ::core::convert::From<InkInputConfiguration> for ::windows::core::IUnknown {
    fn from(value: InkInputConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkInputConfiguration> for ::windows::core::IUnknown {
    fn from(value: &InkInputConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkInputConfiguration> for ::windows::core::IInspectable {
    fn from(value: InkInputConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkInputConfiguration> for ::windows::core::IInspectable {
    fn from(value: &InkInputConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkInputConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkInputConfiguration {}
unsafe impl ::core::marker::Sync for InkInputConfiguration {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkInputProcessingConfiguration(::windows::core::IUnknown);
impl InkInputProcessingConfiguration {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Mode(&self) -> ::windows::core::Result<InkInputProcessingMode> {
        let this = self;
        unsafe {
            let mut result__: InkInputProcessingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkInputProcessingMode>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetMode(&self, value: InkInputProcessingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn RightDragAction(&self) -> ::windows::core::Result<InkInputRightDragAction> {
        let this = self;
        unsafe {
            let mut result__: InkInputRightDragAction = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkInputRightDragAction>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetRightDragAction(&self, value: InkInputRightDragAction) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for InkInputProcessingConfiguration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkInputProcessingConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkInputProcessingConfiguration {}
impl ::core::fmt::Debug for InkInputProcessingConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInputProcessingConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputProcessingConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkInputProcessingConfiguration;{2778d85e-33ca-4b06-a6d3-ac3945116d37})");
}
unsafe impl ::windows::core::Interface for InkInputProcessingConfiguration {
    type Vtable = IInkInputProcessingConfigurationVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2778d85e_33ca_4b06_a6d3_ac3945116d37);
}
impl ::windows::core::RuntimeName for InkInputProcessingConfiguration {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkInputProcessingConfiguration";
}
impl ::core::convert::From<InkInputProcessingConfiguration> for ::windows::core::IUnknown {
    fn from(value: InkInputProcessingConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkInputProcessingConfiguration> for ::windows::core::IUnknown {
    fn from(value: &InkInputProcessingConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkInputProcessingConfiguration> for ::windows::core::IInspectable {
    fn from(value: InkInputProcessingConfiguration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkInputProcessingConfiguration> for ::windows::core::IInspectable {
    fn from(value: &InkInputProcessingConfiguration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkInputProcessingConfiguration {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkInputProcessingConfiguration {}
unsafe impl ::core::marker::Sync for InkInputProcessingConfiguration {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkInputProcessingMode(pub i32);
impl InkInputProcessingMode {
    pub const None: Self = Self(0i32);
    pub const Inking: Self = Self(1i32);
    pub const Erasing: Self = Self(2i32);
}
impl ::core::marker::Copy for InkInputProcessingMode {}
impl ::core::clone::Clone for InkInputProcessingMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for InkInputProcessingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkInputProcessingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInputProcessingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputProcessingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputProcessingMode;i4)");
}
impl ::windows::core::DefaultType for InkInputProcessingMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkInputRightDragAction(pub i32);
impl InkInputRightDragAction {
    pub const LeaveUnprocessed: Self = Self(0i32);
    pub const AllowProcessing: Self = Self(1i32);
}
impl ::core::marker::Copy for InkInputRightDragAction {}
impl ::core::clone::Clone for InkInputRightDragAction {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for InkInputRightDragAction {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkInputRightDragAction {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkInputRightDragAction").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkInputRightDragAction {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkInputRightDragAction;i4)");
}
impl ::windows::core::DefaultType for InkInputRightDragAction {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkManager(::windows::core::IUnknown);
impl InkManager {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkManager, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Mode(&self) -> ::windows::core::Result<InkManipulationMode> {
        let this = self;
        unsafe {
            let mut result__: InkManipulationMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkManipulationMode>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetMode(&self, value: InkManipulationMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn ProcessPointerDown<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn ProcessPointerUpdate<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn ProcessPointerUp<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetDefaultDrawingAttributes<'a, Param0: ::windows::core::IntoParam<'a, InkDrawingAttributes>>(&self, drawingattributes: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), drawingattributes.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RecognizeAsync2(&self, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetDefaultRecognizer<'a, Param0: ::windows::core::IntoParam<'a, InkRecognizer>>(&self, recognizer: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), recognizer.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RecognizeAsync<'a, Param0: ::windows::core::IntoParam<'a, InkStrokeContainer>>(&self, strokecollection: Param0, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = &::windows::core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), strokecollection.into_param().abi(), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = &::windows::core::Interface::cast::<IInkRecognizerContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn AddStroke<'a, Param0: ::windows::core::IntoParam<'a, InkStroke>>(&self, stroke: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, translation: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), translation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SelectWithPolyLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, polyline: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), polyline.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectWithLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, from: Param0, to: Param1) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), from.into_param().abi(), to.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PasteFromClipboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, inputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(&self, recognitionresults: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), recognitionresults.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkManager {}
impl ::core::fmt::Debug for InkManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkManager {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkManager;{4744737d-671b-4163-9c95-4e8d7a035fe1})");
}
unsafe impl ::windows::core::Interface for InkManager {
    type Vtable = IInkManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4744737d_671b_4163_9c95_4e8d7a035fe1);
}
impl ::windows::core::RuntimeName for InkManager {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkManager";
}
impl ::core::convert::From<InkManager> for ::windows::core::IUnknown {
    fn from(value: InkManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkManager> for ::windows::core::IUnknown {
    fn from(value: &InkManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkManager> for ::windows::core::IInspectable {
    fn from(value: InkManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkManager> for ::windows::core::IInspectable {
    fn from(value: &InkManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkManipulationMode(pub i32);
impl InkManipulationMode {
    pub const Inking: Self = Self(0i32);
    pub const Erasing: Self = Self(1i32);
    pub const Selecting: Self = Self(2i32);
}
impl ::core::marker::Copy for InkManipulationMode {}
impl ::core::clone::Clone for InkManipulationMode {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for InkManipulationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkManipulationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkManipulationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkManipulationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkManipulationMode;i4)");
}
impl ::windows::core::DefaultType for InkManipulationMode {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkModelerAttributes(::windows::core::IUnknown);
impl InkModelerAttributes {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PredictionTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::TimeSpan = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetPredictionTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn ScalingFactor(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetScalingFactor(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn UseVelocityBasedPressure(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetUseVelocityBasedPressure(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkModelerAttributes2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
}
impl ::core::clone::Clone for InkModelerAttributes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkModelerAttributes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkModelerAttributes {}
impl ::core::fmt::Debug for InkModelerAttributes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkModelerAttributes").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkModelerAttributes {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkModelerAttributes;{bad31f27-0cd9-4bfd-b6f3-9e03ba8d7454})");
}
unsafe impl ::windows::core::Interface for InkModelerAttributes {
    type Vtable = IInkModelerAttributesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad31f27_0cd9_4bfd_b6f3_9e03ba8d7454);
}
impl ::windows::core::RuntimeName for InkModelerAttributes {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkModelerAttributes";
}
impl ::core::convert::From<InkModelerAttributes> for ::windows::core::IUnknown {
    fn from(value: InkModelerAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkModelerAttributes> for ::windows::core::IUnknown {
    fn from(value: &InkModelerAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkModelerAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkModelerAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkModelerAttributes> for ::windows::core::IInspectable {
    fn from(value: InkModelerAttributes) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkModelerAttributes> for ::windows::core::IInspectable {
    fn from(value: &InkModelerAttributes) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkModelerAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkModelerAttributes {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkModelerAttributes {}
unsafe impl ::core::marker::Sync for InkModelerAttributes {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkPersistenceFormat(pub i32);
impl InkPersistenceFormat {
    pub const GifWithEmbeddedIsf: Self = Self(0i32);
    pub const Isf: Self = Self(1i32);
}
impl ::core::marker::Copy for InkPersistenceFormat {}
impl ::core::clone::Clone for InkPersistenceFormat {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for InkPersistenceFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPersistenceFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPersistenceFormat").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPersistenceFormat {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPersistenceFormat;i4)");
}
impl ::windows::core::DefaultType for InkPersistenceFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkPoint(::windows::core::IUnknown);
impl InkPoint {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Pressure(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn TiltX(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn TiltY(&self) -> ::windows::core::Result<f32> {
        let this = &::windows::core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Timestamp(&self) -> ::windows::core::Result<u64> {
        let this = &::windows::core::Interface::cast::<IInkPoint2>(self)?;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInkPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(position: Param0, pressure: f32) -> ::windows::core::Result<InkPoint> {
        Self::IInkPointFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), pressure, &mut result__).from_abi::<InkPoint>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInkPointWithTiltAndTimestamp<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(position: Param0, pressure: f32, tiltx: f32, tilty: f32, timestamp: u64) -> ::windows::core::Result<InkPoint> {
        Self::IInkPointFactory2(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), position.into_param().abi(), pressure, tiltx, tilty, timestamp, &mut result__).from_abi::<InkPoint>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IInkPointFactory<R, F: FnOnce(&IInkPointFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkPoint, IInkPointFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IInkPointFactory2<R, F: FnOnce(&IInkPointFactory2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkPoint, IInkPointFactory2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InkPoint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkPoint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkPoint {}
impl ::core::fmt::Debug for InkPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPoint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPoint {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPoint;{9f87272b-858c-46a5-9b41-d195970459fd})");
}
unsafe impl ::windows::core::Interface for InkPoint {
    type Vtable = IInkPointVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9f87272b_858c_46a5_9b41_d195970459fd);
}
impl ::windows::core::RuntimeName for InkPoint {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPoint";
}
impl ::core::convert::From<InkPoint> for ::windows::core::IUnknown {
    fn from(value: InkPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkPoint> for ::windows::core::IUnknown {
    fn from(value: &InkPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkPoint> for ::windows::core::IInspectable {
    fn from(value: InkPoint) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkPoint> for ::windows::core::IInspectable {
    fn from(value: &InkPoint) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkPoint {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkPoint {}
unsafe impl ::core::marker::Sync for InkPoint {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkPresenter(::windows::core::IUnknown);
impl InkPresenter {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsInputEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsInputEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'UI_Core'*"]
    #[cfg(feature = "UI_Core")]
    pub fn InputDeviceTypes(&self) -> ::windows::core::Result<super::super::Core::CoreInputDeviceTypes> {
        let this = self;
        unsafe {
            let mut result__: super::super::Core::CoreInputDeviceTypes = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Core::CoreInputDeviceTypes>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'UI_Core'*"]
    #[cfg(feature = "UI_Core")]
    pub fn SetInputDeviceTypes(&self, value: super::super::Core::CoreInputDeviceTypes) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn UnprocessedInput(&self) -> ::windows::core::Result<InkUnprocessedInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkUnprocessedInput>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn StrokeInput(&self) -> ::windows::core::Result<InkStrokeInput> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkStrokeInput>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn InputProcessingConfiguration(&self) -> ::windows::core::Result<InkInputProcessingConfiguration> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkInputProcessingConfiguration>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn StrokeContainer(&self) -> ::windows::core::Result<InkStrokeContainer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkStrokeContainer>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetStrokeContainer<'a, Param0: ::windows::core::IntoParam<'a, InkStrokeContainer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn CopyDefaultDrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributes>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn UpdateDefaultDrawingAttributes<'a, Param0: ::windows::core::IntoParam<'a, InkDrawingAttributes>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn ActivateCustomDrying(&self) -> ::windows::core::Result<InkSynchronizer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkSynchronizer>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetPredefinedConfiguration(&self, value: InkPresenterPredefinedConfiguration) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StrokesCollected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesCollectedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokesCollected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StrokesErased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkPresenter, InkStrokesErasedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).21)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokesErased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).22)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn HighContrastAdjustment(&self) -> ::windows::core::Result<InkHighContrastAdjustment> {
        let this = &::windows::core::Interface::cast::<IInkPresenter2>(self)?;
        unsafe {
            let mut result__: InkHighContrastAdjustment = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkHighContrastAdjustment>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetHighContrastAdjustment(&self, value: InkHighContrastAdjustment) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenter2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn InputConfiguration(&self) -> ::windows::core::Result<InkInputConfiguration> {
        let this = &::windows::core::Interface::cast::<IInkPresenter3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkInputConfiguration>(result__)
        }
    }
}
impl ::core::clone::Clone for InkPresenter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkPresenter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkPresenter {}
impl ::core::fmt::Debug for InkPresenter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenter;{a69b70e2-887b-458f-b173-4fe4438930a3})");
}
unsafe impl ::windows::core::Interface for InkPresenter {
    type Vtable = IInkPresenterVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa69b70e2_887b_458f_b173_4fe4438930a3);
}
impl ::windows::core::RuntimeName for InkPresenter {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenter";
}
impl ::core::convert::From<InkPresenter> for ::windows::core::IUnknown {
    fn from(value: InkPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkPresenter> for ::windows::core::IUnknown {
    fn from(value: &InkPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkPresenter> for ::windows::core::IInspectable {
    fn from(value: InkPresenter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkPresenter> for ::windows::core::IInspectable {
    fn from(value: &InkPresenter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkPresenter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkPresenter {}
unsafe impl ::core::marker::Sync for InkPresenter {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkPresenterPredefinedConfiguration(pub i32);
impl InkPresenterPredefinedConfiguration {
    pub const SimpleSinglePointer: Self = Self(0i32);
    pub const SimpleMultiplePointer: Self = Self(1i32);
}
impl ::core::marker::Copy for InkPresenterPredefinedConfiguration {}
impl ::core::clone::Clone for InkPresenterPredefinedConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for InkPresenterPredefinedConfiguration {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPresenterPredefinedConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenterPredefinedConfiguration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterPredefinedConfiguration {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterPredefinedConfiguration;i4)");
}
impl ::windows::core::DefaultType for InkPresenterPredefinedConfiguration {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkPresenterProtractor(::windows::core::IUnknown);
impl InkPresenterProtractor {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn AreRaysVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetAreRaysVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsCenterMarkerVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsCenterMarkerVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsAngleReadoutVisible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsAngleReadoutVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsResizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsResizable(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Radius(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetRadius(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn AccentColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetAccentColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, InkPresenter>>(inkpresenter: Param0) -> ::windows::core::Result<InkPresenterProtractor> {
        Self::IInkPresenterProtractorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<InkPresenterProtractor>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: InkPresenterStencilKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc(hidden)]
    pub fn IInkPresenterProtractorFactory<R, F: FnOnce(&IInkPresenterProtractorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkPresenterProtractor, IInkPresenterProtractorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InkPresenterProtractor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkPresenterProtractor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkPresenterProtractor {}
impl ::core::fmt::Debug for InkPresenterProtractor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenterProtractor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterProtractor {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenterProtractor;{7de3f2aa-ef6c-4e91-a73b-5b70d56fbd17})");
}
unsafe impl ::windows::core::Interface for InkPresenterProtractor {
    type Vtable = IInkPresenterProtractorVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7de3f2aa_ef6c_4e91_a73b_5b70d56fbd17);
}
impl ::windows::core::RuntimeName for InkPresenterProtractor {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterProtractor";
}
impl ::core::convert::From<InkPresenterProtractor> for ::windows::core::IUnknown {
    fn from(value: InkPresenterProtractor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkPresenterProtractor> for ::windows::core::IUnknown {
    fn from(value: &InkPresenterProtractor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkPresenterProtractor> for ::windows::core::IInspectable {
    fn from(value: InkPresenterProtractor) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkPresenterProtractor> for ::windows::core::IInspectable {
    fn from(value: &InkPresenterProtractor) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkPresenterProtractor {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkPresenterRuler(::windows::core::IUnknown);
impl InkPresenterRuler {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Length(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Width(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetWidth(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn AreTickMarksVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetAreTickMarksVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsCompassVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsCompassVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterRuler2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, InkPresenter>>(inkpresenter: Param0) -> ::windows::core::Result<InkPresenterRuler> {
        Self::IInkPresenterRulerFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpresenter.into_param().abi(), &mut result__).from_abi::<InkPresenterRuler>(result__)
        })
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Kind(&self) -> ::windows::core::Result<InkPresenterStencilKind> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: InkPresenterStencilKind = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenterStencilKind>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsVisible(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetIsVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Transform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkPresenterStencil>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc(hidden)]
    pub fn IInkPresenterRulerFactory<R, F: FnOnce(&IInkPresenterRulerFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkPresenterRuler, IInkPresenterRulerFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for InkPresenterRuler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkPresenterRuler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkPresenterRuler {}
impl ::core::fmt::Debug for InkPresenterRuler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenterRuler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterRuler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkPresenterRuler;{6cda7d5a-dec7-4dd7-877a-2133f183d48a})");
}
unsafe impl ::windows::core::Interface for InkPresenterRuler {
    type Vtable = IInkPresenterRulerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6cda7d5a_dec7_4dd7_877a_2133f183d48a);
}
impl ::windows::core::RuntimeName for InkPresenterRuler {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkPresenterRuler";
}
impl ::core::convert::From<InkPresenterRuler> for ::windows::core::IUnknown {
    fn from(value: InkPresenterRuler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkPresenterRuler> for ::windows::core::IUnknown {
    fn from(value: &InkPresenterRuler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkPresenterRuler> for ::windows::core::IInspectable {
    fn from(value: InkPresenterRuler) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkPresenterRuler> for ::windows::core::IInspectable {
    fn from(value: &InkPresenterRuler) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkPresenterRuler {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkPresenterStencilKind(pub i32);
impl InkPresenterStencilKind {
    pub const Other: Self = Self(0i32);
    pub const Ruler: Self = Self(1i32);
    pub const Protractor: Self = Self(2i32);
}
impl ::core::marker::Copy for InkPresenterStencilKind {}
impl ::core::clone::Clone for InkPresenterStencilKind {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for InkPresenterStencilKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkPresenterStencilKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkPresenterStencilKind").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkPresenterStencilKind {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkPresenterStencilKind;i4)");
}
impl ::windows::core::DefaultType for InkPresenterStencilKind {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkRecognitionResult(::windows::core::IUnknown);
impl InkRecognitionResult {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetTextCandidates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkRecognitionResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkRecognitionResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkRecognitionResult {}
impl ::core::fmt::Debug for InkRecognitionResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognitionResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognitionResult;{36461a94-5068-40ef-8a05-2c2fb60908a2})");
}
unsafe impl ::windows::core::Interface for InkRecognitionResult {
    type Vtable = IInkRecognitionResultVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36461a94_5068_40ef_8a05_2c2fb60908a2);
}
impl ::windows::core::RuntimeName for InkRecognitionResult {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognitionResult";
}
impl ::core::convert::From<InkRecognitionResult> for ::windows::core::IUnknown {
    fn from(value: InkRecognitionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkRecognitionResult> for ::windows::core::IUnknown {
    fn from(value: &InkRecognitionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkRecognitionResult> for ::windows::core::IInspectable {
    fn from(value: InkRecognitionResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkRecognitionResult> for ::windows::core::IInspectable {
    fn from(value: &InkRecognitionResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkRecognitionResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkRecognitionResult {}
unsafe impl ::core::marker::Sync for InkRecognitionResult {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct InkRecognitionTarget(pub i32);
impl InkRecognitionTarget {
    pub const All: Self = Self(0i32);
    pub const Selected: Self = Self(1i32);
    pub const Recent: Self = Self(2i32);
}
impl ::core::marker::Copy for InkRecognitionTarget {}
impl ::core::clone::Clone for InkRecognitionTarget {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for InkRecognitionTarget {
    type Abi = Self;
}
impl ::core::fmt::Debug for InkRecognitionTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognitionTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognitionTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.InkRecognitionTarget;i4)");
}
impl ::windows::core::DefaultType for InkRecognitionTarget {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkRecognizer(::windows::core::IUnknown);
impl InkRecognizer {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for InkRecognizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkRecognizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkRecognizer {}
impl ::core::fmt::Debug for InkRecognizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognizer;{077ccea3-904d-442a-b151-aaca3631c43b})");
}
unsafe impl ::windows::core::Interface for InkRecognizer {
    type Vtable = IInkRecognizerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x077ccea3_904d_442a_b151_aaca3631c43b);
}
impl ::windows::core::RuntimeName for InkRecognizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizer";
}
impl ::core::convert::From<InkRecognizer> for ::windows::core::IUnknown {
    fn from(value: InkRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkRecognizer> for ::windows::core::IUnknown {
    fn from(value: &InkRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkRecognizer> for ::windows::core::IInspectable {
    fn from(value: InkRecognizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkRecognizer> for ::windows::core::IInspectable {
    fn from(value: &InkRecognizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkRecognizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkRecognizerContainer(::windows::core::IUnknown);
impl InkRecognizerContainer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkRecognizerContainer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetDefaultRecognizer<'a, Param0: ::windows::core::IntoParam<'a, InkRecognizer>>(&self, recognizer: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), recognizer.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn RecognizeAsync<'a, Param0: ::windows::core::IntoParam<'a, InkStrokeContainer>>(&self, strokecollection: Param0, recognitiontarget: InkRecognitionTarget) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), strokecollection.into_param().abi(), recognitiontarget, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognizers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognizer>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkRecognizerContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkRecognizerContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkRecognizerContainer {}
impl ::core::fmt::Debug for InkRecognizerContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkRecognizerContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkRecognizerContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkRecognizerContainer;{a74d9a31-8047-4698-a912-f82a5085012f})");
}
unsafe impl ::windows::core::Interface for InkRecognizerContainer {
    type Vtable = IInkRecognizerContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa74d9a31_8047_4698_a912_f82a5085012f);
}
impl ::windows::core::RuntimeName for InkRecognizerContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkRecognizerContainer";
}
impl ::core::convert::From<InkRecognizerContainer> for ::windows::core::IUnknown {
    fn from(value: InkRecognizerContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkRecognizerContainer> for ::windows::core::IUnknown {
    fn from(value: &InkRecognizerContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkRecognizerContainer> for ::windows::core::IInspectable {
    fn from(value: InkRecognizerContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkRecognizerContainer> for ::windows::core::IInspectable {
    fn from(value: &InkRecognizerContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkRecognizerContainer> for IInkRecognizerContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: InkRecognizerContainer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkRecognizerContainer> for IInkRecognizerContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkRecognizerContainer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkRecognizerContainer> for InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IInkRecognizerContainer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkRecognizerContainer> for &InkRecognizerContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IInkRecognizerContainer> {
        ::core::convert::TryInto::<IInkRecognizerContainer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkStroke(::windows::core::IUnknown);
impl InkStroke {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn DrawingAttributes(&self) -> ::windows::core::Result<InkDrawingAttributes> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkDrawingAttributes>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetDrawingAttributes<'a, Param0: ::windows::core::IntoParam<'a, InkDrawingAttributes>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Selected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetSelected(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Recognized(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRenderingSegments(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStrokeRenderingSegment>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Clone(&self) -> ::windows::core::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn PointTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix3x2> {
        let this = &::windows::core::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::Numerics::Matrix3x2 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Numerics::Matrix3x2>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Numerics'*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetPointTransform<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStroke2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInkPoints(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkPoint>> {
        let this = &::windows::core::Interface::cast::<IInkStroke2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkPoint>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Id(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StrokeStartedTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStrokeStartedTime<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn StrokeDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStrokeDuration<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStroke3>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn PointerId(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<IInkStroke4>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStroke {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStroke {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStroke {}
impl ::core::fmt::Debug for InkStroke {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStroke").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStroke {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStroke;{15144d60-cce3-4fcf-9d52-11518ab6afd4})");
}
unsafe impl ::windows::core::Interface for InkStroke {
    type Vtable = IInkStrokeVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15144d60_cce3_4fcf_9d52_11518ab6afd4);
}
impl ::windows::core::RuntimeName for InkStroke {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStroke";
}
impl ::core::convert::From<InkStroke> for ::windows::core::IUnknown {
    fn from(value: InkStroke) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStroke> for ::windows::core::IUnknown {
    fn from(value: &InkStroke) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkStroke> for ::windows::core::IInspectable {
    fn from(value: InkStroke) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStroke> for ::windows::core::IInspectable {
    fn from(value: &InkStroke) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkStroke {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkStroke {}
unsafe impl ::core::marker::Sync for InkStroke {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkStrokeBuilder(::windows::core::IUnknown);
impl InkStrokeBuilder {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkStrokeBuilder, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn BeginStroke<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn AppendToStroke<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<super::PointerPoint> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<super::PointerPoint>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn EndStroke<'a, Param0: ::windows::core::IntoParam<'a, super::PointerPoint>>(&self, pointerpoint: Param0) -> ::windows::core::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), pointerpoint.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn CreateStroke<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, points: Param0) -> ::windows::core::Result<InkStroke> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), points.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetDefaultDrawingAttributes<'a, Param0: ::windows::core::IntoParam<'a, InkDrawingAttributes>>(&self, drawingattributes: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), drawingattributes.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections', 'Foundation_Numerics'*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub fn CreateStrokeFromInkPoints<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InkPoint>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>>(&self, inkpoints: Param0, transform: Param1) -> ::windows::core::Result<InkStroke> {
        let this = &::windows::core::Interface::cast::<IInkStrokeBuilder2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpoints.into_param().abi(), transform.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Foundation_Collections', 'Foundation_Numerics'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "Foundation_Numerics"))]
    pub fn CreateStrokeFromInkPoints2<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InkPoint>>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Numerics::Matrix3x2>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>>(&self, inkpoints: Param0, transform: Param1, strokestartedtime: Param2, strokeduration: Param3) -> ::windows::core::Result<InkStroke> {
        let this = &::windows::core::Interface::cast::<IInkStrokeBuilder3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), inkpoints.into_param().abi(), transform.into_param().abi(), strokestartedtime.into_param().abi(), strokeduration.into_param().abi(), &mut result__).from_abi::<InkStroke>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokeBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokeBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokeBuilder {}
impl ::core::fmt::Debug for InkStrokeBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokeBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeBuilder {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeBuilder;{82bbd1dc-1c63-41dc-9e07-4b4a70ced801})");
}
unsafe impl ::windows::core::Interface for InkStrokeBuilder {
    type Vtable = IInkStrokeBuilderVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82bbd1dc_1c63_41dc_9e07_4b4a70ced801);
}
impl ::windows::core::RuntimeName for InkStrokeBuilder {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeBuilder";
}
impl ::core::convert::From<InkStrokeBuilder> for ::windows::core::IUnknown {
    fn from(value: InkStrokeBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokeBuilder> for ::windows::core::IUnknown {
    fn from(value: &InkStrokeBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokeBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkStrokeBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkStrokeBuilder> for ::windows::core::IInspectable {
    fn from(value: InkStrokeBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokeBuilder> for ::windows::core::IInspectable {
    fn from(value: &InkStrokeBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokeBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkStrokeBuilder {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkStrokeContainer(::windows::core::IUnknown);
impl InkStrokeContainer {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<InkStrokeContainer, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BoundingRect(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn AddStroke<'a, Param0: ::windows::core::IntoParam<'a, InkStroke>>(&self, stroke: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stroke.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteSelected(&self) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveSelected<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, translation: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), translation.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Foundation_Collections'*"]
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SelectWithPolyLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Point>>>(&self, polyline: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), polyline.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn SelectWithLine<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, from: Param0, to: Param1) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), from.into_param().abi(), to.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn CopySelectedToClipboard(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn PasteFromClipboard<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Point>>(&self, position: Param0) -> ::windows::core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Rect = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), position.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::Rect>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn CanPasteFromClipboard(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IInputStream>>(&self, inputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncActionWithProgress<u64>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), inputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncActionWithProgress<u64>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn UpdateRecognitionResults<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>>(&self, recognitionresults: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), recognitionresults.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetStrokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRecognitionResults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkRecognitionResult>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AddStrokes<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<InkStroke>>>(&self, strokes: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), strokes.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'Storage_Streams'*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveWithFormatAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Storage::Streams::IOutputStream>>(&self, outputstream: Param0, inkpersistenceformat: InkPersistenceFormat) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), outputstream.into_param().abi(), inkpersistenceformat, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn GetStrokeById(&self, id: u32) -> ::windows::core::Result<InkStroke> {
        let this = &::windows::core::Interface::cast::<IInkStrokeContainer3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), id, &mut result__).from_abi::<InkStroke>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokeContainer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokeContainer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokeContainer {}
impl ::core::fmt::Debug for InkStrokeContainer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokeContainer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeContainer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeContainer;{22accbc6-faa9-4f14-b68c-f6cee670ae16})");
}
unsafe impl ::windows::core::Interface for InkStrokeContainer {
    type Vtable = IInkStrokeContainerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22accbc6_faa9_4f14_b68c_f6cee670ae16);
}
impl ::windows::core::RuntimeName for InkStrokeContainer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeContainer";
}
impl ::core::convert::From<InkStrokeContainer> for ::windows::core::IUnknown {
    fn from(value: InkStrokeContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokeContainer> for ::windows::core::IUnknown {
    fn from(value: &InkStrokeContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkStrokeContainer> for ::windows::core::IInspectable {
    fn from(value: InkStrokeContainer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokeContainer> for ::windows::core::IInspectable {
    fn from(value: &InkStrokeContainer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<InkStrokeContainer> for IInkStrokeContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: InkStrokeContainer) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&InkStrokeContainer> for IInkStrokeContainer {
    type Error = ::windows::core::Error;
    fn try_from(value: &InkStrokeContainer) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkStrokeContainer> for InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IInkStrokeContainer> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IInkStrokeContainer> for &InkStrokeContainer {
    fn into_param(self) -> ::windows::core::Param<'a, IInkStrokeContainer> {
        ::core::convert::TryInto::<IInkStrokeContainer>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkStrokeInput(::windows::core::IUnknown);
impl InkStrokeInput {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeStarted<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeContinued<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeContinued<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeEnded<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn StrokeCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkStrokeInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStrokeCanceled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenter>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokeInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokeInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokeInput {}
impl ::core::fmt::Debug for InkStrokeInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokeInput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeInput {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeInput;{cf2ffe7b-5e10-43c6-a080-88f26e1dc67d})");
}
unsafe impl ::windows::core::Interface for InkStrokeInput {
    type Vtable = IInkStrokeInputVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf2ffe7b_5e10_43c6_a080_88f26e1dc67d);
}
impl ::windows::core::RuntimeName for InkStrokeInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeInput";
}
impl ::core::convert::From<InkStrokeInput> for ::windows::core::IUnknown {
    fn from(value: InkStrokeInput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokeInput> for ::windows::core::IUnknown {
    fn from(value: &InkStrokeInput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokeInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkStrokeInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkStrokeInput> for ::windows::core::IInspectable {
    fn from(value: InkStrokeInput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokeInput> for ::windows::core::IInspectable {
    fn from(value: &InkStrokeInput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokeInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkStrokeInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkStrokeInput {}
unsafe impl ::core::marker::Sync for InkStrokeInput {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkStrokeRenderingSegment(::windows::core::IUnknown);
impl InkStrokeRenderingSegment {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BezierControlPoint1(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn BezierControlPoint2(&self) -> ::windows::core::Result<super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Point = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Point>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Pressure(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn TiltX(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn TiltY(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn Twist(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f32>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokeRenderingSegment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokeRenderingSegment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokeRenderingSegment {}
impl ::core::fmt::Debug for InkStrokeRenderingSegment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokeRenderingSegment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokeRenderingSegment {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokeRenderingSegment;{68510f1f-88e3-477a-a2fa-569f5f1f9bd5})");
}
unsafe impl ::windows::core::Interface for InkStrokeRenderingSegment {
    type Vtable = IInkStrokeRenderingSegmentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x68510f1f_88e3_477a_a2fa_569f5f1f9bd5);
}
impl ::windows::core::RuntimeName for InkStrokeRenderingSegment {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokeRenderingSegment";
}
impl ::core::convert::From<InkStrokeRenderingSegment> for ::windows::core::IUnknown {
    fn from(value: InkStrokeRenderingSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokeRenderingSegment> for ::windows::core::IUnknown {
    fn from(value: &InkStrokeRenderingSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkStrokeRenderingSegment> for ::windows::core::IInspectable {
    fn from(value: InkStrokeRenderingSegment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokeRenderingSegment> for ::windows::core::IInspectable {
    fn from(value: &InkStrokeRenderingSegment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkStrokeRenderingSegment {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkStrokeRenderingSegment {}
unsafe impl ::core::marker::Sync for InkStrokeRenderingSegment {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkStrokesCollectedEventArgs(::windows::core::IUnknown);
impl InkStrokesCollectedEventArgs {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokesCollectedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokesCollectedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokesCollectedEventArgs {}
impl ::core::fmt::Debug for InkStrokesCollectedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokesCollectedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokesCollectedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokesCollectedEventArgs;{c4f3f229-1938-495c-b4d9-6de4b08d4811})");
}
unsafe impl ::windows::core::Interface for InkStrokesCollectedEventArgs {
    type Vtable = IInkStrokesCollectedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc4f3f229_1938_495c_b4d9_6de4b08d4811);
}
impl ::windows::core::RuntimeName for InkStrokesCollectedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesCollectedEventArgs";
}
impl ::core::convert::From<InkStrokesCollectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: InkStrokesCollectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokesCollectedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InkStrokesCollectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkStrokesCollectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: InkStrokesCollectedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokesCollectedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InkStrokesCollectedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkStrokesCollectedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkStrokesErasedEventArgs(::windows::core::IUnknown);
impl InkStrokesErasedEventArgs {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Strokes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
}
impl ::core::clone::Clone for InkStrokesErasedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkStrokesErasedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkStrokesErasedEventArgs {}
impl ::core::fmt::Debug for InkStrokesErasedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkStrokesErasedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkStrokesErasedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkStrokesErasedEventArgs;{a4216a22-1503-4ebf-8ff5-2de84584a8aa})");
}
unsafe impl ::windows::core::Interface for InkStrokesErasedEventArgs {
    type Vtable = IInkStrokesErasedEventArgsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4216a22_1503_4ebf_8ff5_2de84584a8aa);
}
impl ::windows::core::RuntimeName for InkStrokesErasedEventArgs {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkStrokesErasedEventArgs";
}
impl ::core::convert::From<InkStrokesErasedEventArgs> for ::windows::core::IUnknown {
    fn from(value: InkStrokesErasedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokesErasedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &InkStrokesErasedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkStrokesErasedEventArgs> for ::windows::core::IInspectable {
    fn from(value: InkStrokesErasedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkStrokesErasedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &InkStrokesErasedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkStrokesErasedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkSynchronizer(::windows::core::IUnknown);
impl InkSynchronizer {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation_Collections'*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BeginDry(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<InkStroke>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<InkStroke>>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn EndDry(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for InkSynchronizer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkSynchronizer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkSynchronizer {}
impl ::core::fmt::Debug for InkSynchronizer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkSynchronizer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkSynchronizer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkSynchronizer;{9b9ea160-ae9b-45f9-8407-4b493b163661})");
}
unsafe impl ::windows::core::Interface for InkSynchronizer {
    type Vtable = IInkSynchronizerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b9ea160_ae9b_45f9_8407_4b493b163661);
}
impl ::windows::core::RuntimeName for InkSynchronizer {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkSynchronizer";
}
impl ::core::convert::From<InkSynchronizer> for ::windows::core::IUnknown {
    fn from(value: InkSynchronizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkSynchronizer> for ::windows::core::IUnknown {
    fn from(value: &InkSynchronizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkSynchronizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkSynchronizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkSynchronizer> for ::windows::core::IInspectable {
    fn from(value: InkSynchronizer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkSynchronizer> for ::windows::core::IInspectable {
    fn from(value: &InkSynchronizer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkSynchronizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkSynchronizer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct InkUnprocessedInput(::windows::core::IUnknown);
impl InkUnprocessedInput {
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerEntered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerHovered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerHovered<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerExited<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerPressed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerMoved<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerReleased<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation', 'UI_Core'*"]
    #[cfg(all(feature = "Foundation", feature = "UI_Core"))]
    pub fn PointerLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<InkUnprocessedInput, super::super::Core::PointerEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).18)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking', 'Foundation'*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePointerLost<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, cookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).19)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn InkPresenter(&self) -> ::windows::core::Result<InkPresenter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).20)(::core::mem::transmute_copy(this), &mut result__).from_abi::<InkPresenter>(result__)
        }
    }
}
impl ::core::clone::Clone for InkUnprocessedInput {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for InkUnprocessedInput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for InkUnprocessedInput {}
impl ::core::fmt::Debug for InkUnprocessedInput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("InkUnprocessedInput").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for InkUnprocessedInput {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.InkUnprocessedInput;{db4445e0-8398-4921-ac3b-ab978c5ba256})");
}
unsafe impl ::windows::core::Interface for InkUnprocessedInput {
    type Vtable = IInkUnprocessedInputVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb4445e0_8398_4921_ac3b_ab978c5ba256);
}
impl ::windows::core::RuntimeName for InkUnprocessedInput {
    const NAME: &'static str = "Windows.UI.Input.Inking.InkUnprocessedInput";
}
impl ::core::convert::From<InkUnprocessedInput> for ::windows::core::IUnknown {
    fn from(value: InkUnprocessedInput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkUnprocessedInput> for ::windows::core::IUnknown {
    fn from(value: &InkUnprocessedInput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for InkUnprocessedInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &InkUnprocessedInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<InkUnprocessedInput> for ::windows::core::IInspectable {
    fn from(value: InkUnprocessedInput) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&InkUnprocessedInput> for ::windows::core::IInspectable {
    fn from(value: &InkUnprocessedInput) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for InkUnprocessedInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &InkUnprocessedInput {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for InkUnprocessedInput {}
unsafe impl ::core::marker::Sync for InkUnprocessedInput {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
pub struct PenAndInkSettings(::windows::core::IUnknown);
impl PenAndInkSettings {
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsHandwritingDirectlyIntoTextFieldEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn PenHandedness(&self) -> ::windows::core::Result<PenHandedness> {
        let this = self;
        unsafe {
            let mut result__: PenHandedness = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PenHandedness>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn HandwritingLineHeight(&self) -> ::windows::core::Result<HandwritingLineHeight> {
        let this = self;
        unsafe {
            let mut result__: HandwritingLineHeight = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<HandwritingLineHeight>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn FontFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn UserConsentsToHandwritingTelemetryCollection(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn IsTouchHandwritingEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn SetPenHandedness(&self, value: PenHandedness) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IPenAndInkSettings2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: 'UI_Input_Inking'*"]
    pub fn GetDefault() -> ::windows::core::Result<PenAndInkSettings> {
        Self::IPenAndInkSettingsStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<PenAndInkSettings>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPenAndInkSettingsStatics<R, F: FnOnce(&IPenAndInkSettingsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PenAndInkSettings, IPenAndInkSettingsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PenAndInkSettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PenAndInkSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PenAndInkSettings {}
impl ::core::fmt::Debug for PenAndInkSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenAndInkSettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PenAndInkSettings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Input.Inking.PenAndInkSettings;{bc2ceb8f-0066-44a8-bb7a-b839b3deb8f5})");
}
unsafe impl ::windows::core::Interface for PenAndInkSettings {
    type Vtable = IPenAndInkSettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc2ceb8f_0066_44a8_bb7a_b839b3deb8f5);
}
impl ::windows::core::RuntimeName for PenAndInkSettings {
    const NAME: &'static str = "Windows.UI.Input.Inking.PenAndInkSettings";
}
impl ::core::convert::From<PenAndInkSettings> for ::windows::core::IUnknown {
    fn from(value: PenAndInkSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenAndInkSettings> for ::windows::core::IUnknown {
    fn from(value: &PenAndInkSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PenAndInkSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &PenAndInkSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PenAndInkSettings> for ::windows::core::IInspectable {
    fn from(value: PenAndInkSettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PenAndInkSettings> for ::windows::core::IInspectable {
    fn from(value: &PenAndInkSettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PenAndInkSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &PenAndInkSettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PenAndInkSettings {}
unsafe impl ::core::marker::Sync for PenAndInkSettings {}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PenHandedness(pub i32);
impl PenHandedness {
    pub const Right: Self = Self(0i32);
    pub const Left: Self = Self(1i32);
}
impl ::core::marker::Copy for PenHandedness {}
impl ::core::clone::Clone for PenHandedness {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PenHandedness {
    type Abi = Self;
}
impl ::core::fmt::Debug for PenHandedness {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenHandedness").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PenHandedness {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenHandedness;i4)");
}
impl ::windows::core::DefaultType for PenHandedness {
    type DefaultType = Self;
}
#[doc = "*Required features: 'UI_Input_Inking'*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PenTipShape(pub i32);
impl PenTipShape {
    pub const Circle: Self = Self(0i32);
    pub const Rectangle: Self = Self(1i32);
}
impl ::core::marker::Copy for PenTipShape {}
impl ::core::clone::Clone for PenTipShape {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PenTipShape {
    type Abi = Self;
}
impl ::core::fmt::Debug for PenTipShape {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PenTipShape").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PenTipShape {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Input.Inking.PenTipShape;i4)");
}
impl ::windows::core::DefaultType for PenTipShape {
    type DefaultType = Self;
}
