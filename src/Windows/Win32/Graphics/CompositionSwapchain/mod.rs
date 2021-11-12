#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct CompositionFrameDisplayInstance {
    pub displayAdapterLUID: super::super::Foundation::LUID,
    pub displayVidPnSourceId: u32,
    pub displayUniqueId: u32,
    pub renderAdapterLUID: super::super::Foundation::LUID,
    pub instanceKind: CompositionFrameInstanceKind,
    pub finalTransform: PresentationTransform,
    pub requiredCrossAdapterCopy: u8,
    pub colorSpace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for CompositionFrameDisplayInstance {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for CompositionFrameDisplayInstance {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CompositionFrameDisplayInstance")
            .field("displayAdapterLUID", &self.displayAdapterLUID)
            .field("displayVidPnSourceId", &self.displayVidPnSourceId)
            .field("displayUniqueId", &self.displayUniqueId)
            .field("renderAdapterLUID", &self.renderAdapterLUID)
            .field("instanceKind", &self.instanceKind)
            .field("finalTransform", &self.finalTransform)
            .field("requiredCrossAdapterCopy", &self.requiredCrossAdapterCopy)
            .field("colorSpace", &self.colorSpace)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for CompositionFrameDisplayInstance {
    fn eq(&self, other: &Self) -> bool {
        self.displayAdapterLUID == other.displayAdapterLUID && self.displayVidPnSourceId == other.displayVidPnSourceId && self.displayUniqueId == other.displayUniqueId && self.renderAdapterLUID == other.renderAdapterLUID && self.instanceKind == other.instanceKind && self.finalTransform == other.finalTransform && self.requiredCrossAdapterCopy == other.requiredCrossAdapterCopy && self.colorSpace == other.colorSpace
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for CompositionFrameDisplayInstance {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CompositionFrameInstanceKind(pub i32);
pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(0i32);
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(1i32);
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = CompositionFrameInstanceKind(2i32);
impl ::core::convert::From<i32> for CompositionFrameInstanceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for CompositionFrameInstanceKind {
    type Abi = Self;
}
#[inline]
pub unsafe fn CreatePresentationFactory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(d3ddevice: Param0, riid: *const ::windows::core::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePresentationFactory(d3ddevice: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CreatePresentationFactory(d3ddevice.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(presentationfactory)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICompositionFramePresentStatistics(pub ::windows::core::IUnknown);
impl ICompositionFramePresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetContentTag(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(displayinstancearraycount), ::core::mem::transmute(displayinstancearray)))
    }
}
unsafe impl ::windows::core::Interface for ICompositionFramePresentStatistics {
    type Vtable = ICompositionFramePresentStatistics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab41d127_c101_4c0a_911d_f9f2e9d08e64);
}
impl ::core::convert::From<ICompositionFramePresentStatistics> for ::windows::core::IUnknown {
    fn from(value: ICompositionFramePresentStatistics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositionFramePresentStatistics> for ::windows::core::IUnknown {
    fn from(value: &ICompositionFramePresentStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<ICompositionFramePresentStatistics> for IPresentStatistics {
    fn from(value: ICompositionFramePresentStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionFramePresentStatistics> for IPresentStatistics {
    fn from(value: &ICompositionFramePresentStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPresentStatistics> for ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentStatistics> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPresentStatistics> for &ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentStatistics> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionFramePresentStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> PresentStatisticsKind,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u64,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IIndependentFlipFramePresentStatistics(pub ::windows::core::IUnknown);
impl IIndependentFlipFramePresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::default::Default::default();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    pub unsafe fn GetOutputVidPnSourceId(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetContentTag(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetDisplayedTime(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::core::default::Default::default();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    pub unsafe fn GetPresentDuration(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::core::default::Default::default();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
}
unsafe impl ::windows::core::Interface for IIndependentFlipFramePresentStatistics {
    type Vtable = IIndependentFlipFramePresentStatistics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c93be27_ad94_4da0_8fd4_2413132d124e);
}
impl ::core::convert::From<IIndependentFlipFramePresentStatistics> for ::windows::core::IUnknown {
    fn from(value: IIndependentFlipFramePresentStatistics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IIndependentFlipFramePresentStatistics> for ::windows::core::IUnknown {
    fn from(value: &IIndependentFlipFramePresentStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IIndependentFlipFramePresentStatistics> for IPresentStatistics {
    fn from(value: IIndependentFlipFramePresentStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIndependentFlipFramePresentStatistics> for IPresentStatistics {
    fn from(value: &IIndependentFlipFramePresentStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPresentStatistics> for IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentStatistics> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPresentStatistics> for &IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentStatistics> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndependentFlipFramePresentStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> PresentStatisticsKind,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut super::super::Foundation::LUID),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SystemInterruptTime),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut SystemInterruptTime),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentStatistics(pub ::windows::core::IUnknown);
impl IPresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IPresentStatistics {
    type Vtable = IPresentStatistics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb44b8bda_7282_495d_9dd7_ceadd8b4bb86);
}
impl ::core::convert::From<IPresentStatistics> for ::windows::core::IUnknown {
    fn from(value: IPresentStatistics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentStatistics> for ::windows::core::IUnknown {
    fn from(value: &IPresentStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> PresentStatisticsKind,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentStatusPresentStatistics(pub ::windows::core::IUnknown);
impl IPresentStatusPresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetPresentStatus(&self) -> PresentStatus {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::core::Interface for IPresentStatusPresentStatistics {
    type Vtable = IPresentStatusPresentStatistics_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9ed2a41_79cb_435e_964e_c8553055420c);
}
impl ::core::convert::From<IPresentStatusPresentStatistics> for ::windows::core::IUnknown {
    fn from(value: IPresentStatusPresentStatistics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentStatusPresentStatistics> for ::windows::core::IUnknown {
    fn from(value: &IPresentStatusPresentStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPresentStatusPresentStatistics> for IPresentStatistics {
    fn from(value: IPresentStatusPresentStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPresentStatusPresentStatistics> for IPresentStatistics {
    fn from(value: &IPresentStatusPresentStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPresentStatistics> for IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentStatistics> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPresentStatistics> for &IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentStatistics> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatusPresentStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> PresentStatisticsKind,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> PresentStatus,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationBuffer(pub ::windows::core::IUnknown);
impl IPresentationBuffer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAvailableEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    pub unsafe fn IsAvailable(&self) -> ::windows::core::Result<u8> {
        let mut result__: <u8 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u8>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPresentationBuffer {
    type Vtable = IPresentationBuffer_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e217d3a_5abb_4138_9a13_a775593c89ca);
}
impl ::core::convert::From<IPresentationBuffer> for ::windows::core::IUnknown {
    fn from(value: IPresentationBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationBuffer> for ::windows::core::IUnknown {
    fn from(value: &IPresentationBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, isavailable: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationContent(pub ::windows::core::IUnknown);
impl IPresentationContent {
    pub unsafe fn SetTag(&self, tag: usize) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tag)))
    }
}
unsafe impl ::windows::core::Interface for IPresentationContent {
    type Vtable = IPresentationContent_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5668bb79_3d8e_415c_b215_f38020f2d252);
}
impl ::core::convert::From<IPresentationContent> for ::windows::core::IUnknown {
    fn from(value: IPresentationContent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationContent> for ::windows::core::IUnknown {
    fn from(value: &IPresentationContent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationContent_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tag: usize),
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationFactory(pub ::windows::core::IUnknown);
impl IPresentationFactory {
    pub unsafe fn IsPresentationSupported(&self) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn IsPresentationSupportedWithIndependentFlip(&self) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn CreatePresentationManager(&self) -> ::windows::core::Result<IPresentationManager> {
        let mut result__: <IPresentationManager as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPresentationManager>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPresentationFactory {
    type Vtable = IPresentationFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fb37b58_1d74_4f64_a49c_1f97a80a2ec0);
}
impl ::core::convert::From<IPresentationFactory> for ::windows::core::IUnknown {
    fn from(value: IPresentationFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationFactory> for ::windows::core::IUnknown {
    fn from(value: &IPresentationFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u8,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u8,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pppresentationmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationManager(pub ::windows::core::IUnknown);
impl IPresentationManager {
    pub unsafe fn AddBufferFromResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, resource: Param0) -> ::windows::core::Result<IPresentationBuffer> {
        let mut result__: <IPresentationBuffer as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), resource.into_param().abi(), &mut result__).from_abi::<IPresentationBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePresentationSurface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, compositionsurfacehandle: Param0) -> ::windows::core::Result<IPresentationSurface> {
        let mut result__: <IPresentationSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), compositionsurfacehandle.into_param().abi(), &mut result__).from_abi::<IPresentationSurface>(result__)
    }
    pub unsafe fn GetNextPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn SetTargetTime<'a, Param0: ::windows::core::IntoParam<'a, SystemInterruptTime>>(&self, targettime: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), targettime.into_param().abi()).ok()
    }
    pub unsafe fn SetPreferredPresentDuration<'a, Param0: ::windows::core::IntoParam<'a, SystemInterruptTime>, Param1: ::windows::core::IntoParam<'a, SystemInterruptTime>>(&self, preferredduration: Param0, deviationtolerance: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), preferredduration.into_param().abi(), deviationtolerance.into_param().abi()).ok()
    }
    pub unsafe fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(forcevsyncinterrupt)).ok()
    }
    pub unsafe fn Present(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetPresentRetiringFence(&self, riid: *const ::windows::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(fence)).ok()
    }
    pub unsafe fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(presentidtocancelfrom)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLostEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresentStatisticsAvailableEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    pub unsafe fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(presentstatisticskind), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn GetNextPresentStatistics(&self) -> ::windows::core::Result<IPresentStatistics> {
        let mut result__: <IPresentStatistics as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPresentStatistics>(result__)
    }
}
unsafe impl ::windows::core::Interface for IPresentationManager {
    type Vtable = IPresentationManager_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb562f82_6292_470a_88b1_843661e7f20c);
}
impl ::core::convert::From<IPresentationManager> for ::windows::core::IUnknown {
    fn from(value: IPresentationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationManager> for ::windows::core::IUnknown {
    fn from(value: &IPresentationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, resource: ::windows::core::RawPtr, presentationbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, targettime: SystemInterruptTime) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, forcevsyncinterrupt: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presentidtocancelfrom: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, nextpresentstatistics: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationSurface(pub ::windows::core::IUnknown);
impl IPresentationSurface {
    pub unsafe fn SetTag(&self, tag: usize) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tag)))
    }
    pub unsafe fn SetBuffer<'a, Param0: ::windows::core::IntoParam<'a, IPresentationBuffer>>(&self, presentationbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), presentationbuffer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorspace)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(alphamode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sourcerect)).ok()
    }
    pub unsafe fn SetTransform(&self, transform: *const PresentationTransform) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(transform)).ok()
    }
    pub unsafe fn RestrictToOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, output: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), output.into_param().abi()).ok()
    }
    pub unsafe fn SetDisableReadback(&self, value: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(leftletterboxsize), ::core::mem::transmute(topletterboxsize), ::core::mem::transmute(rightletterboxsize), ::core::mem::transmute(bottomletterboxsize)).ok()
    }
}
unsafe impl ::windows::core::Interface for IPresentationSurface {
    type Vtable = IPresentationSurface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x956710fb_ea40_4eba_a3eb_4375a0eb4edc);
}
impl ::core::convert::From<IPresentationSurface> for ::windows::core::IUnknown {
    fn from(value: IPresentationSurface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationSurface> for ::windows::core::IUnknown {
    fn from(value: &IPresentationSurface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<IPresentationSurface> for IPresentationContent {
    fn from(value: IPresentationSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPresentationSurface> for IPresentationContent {
    fn from(value: &IPresentationSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPresentationContent> for IPresentationSurface {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentationContent> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IPresentationContent> for &IPresentationSurface {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentationContent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationSurface_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tag: usize),
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, presentationbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transform: *const PresentationTransform) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, output: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::HRESULT,
);
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PresentStatisticsKind(pub i32);
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = PresentStatisticsKind(1i32);
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = PresentStatisticsKind(2i32);
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = PresentStatisticsKind(3i32);
impl ::core::convert::From<i32> for PresentStatisticsKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PresentStatisticsKind {
    type Abi = Self;
}
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PresentStatus(pub i32);
pub const PresentStatus_Queued: PresentStatus = PresentStatus(0i32);
pub const PresentStatus_Skipped: PresentStatus = PresentStatus(1i32);
pub const PresentStatus_Canceled: PresentStatus = PresentStatus(2i32);
impl ::core::convert::From<i32> for PresentStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PresentStatus {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct PresentationTransform {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
impl PresentationTransform {}
impl ::core::default::Default for PresentationTransform {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for PresentationTransform {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("PresentationTransform").field("M11", &self.M11).field("M12", &self.M12).field("M21", &self.M21).field("M22", &self.M22).field("M31", &self.M31).field("M32", &self.M32).finish()
    }
}
impl ::core::cmp::PartialEq for PresentationTransform {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M21 == other.M21 && self.M22 == other.M22 && self.M31 == other.M31 && self.M32 == other.M32
    }
}
impl ::core::cmp::Eq for PresentationTransform {}
unsafe impl ::windows::core::Abi for PresentationTransform {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
pub struct SystemInterruptTime {
    pub value: u64,
}
impl SystemInterruptTime {}
impl ::core::default::Default for SystemInterruptTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for SystemInterruptTime {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("SystemInterruptTime").field("value", &self.value).finish()
    }
}
impl ::core::cmp::PartialEq for SystemInterruptTime {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl ::core::cmp::Eq for SystemInterruptTime {}
unsafe impl ::windows::core::Abi for SystemInterruptTime {
    type Abi = Self;
}
