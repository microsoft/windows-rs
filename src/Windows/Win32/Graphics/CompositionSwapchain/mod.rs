#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
pub struct CompositionFrameDisplayInstance {
    pub displayAdapterLUID: super::super::Foundation::LUID,
    pub displayVidPnSourceId: u32,
    pub displayUniqueId: u32,
    pub renderAdapterLUID: super::super::Foundation::LUID,
    pub instanceKind: CompositionFrameInstanceKind,
    pub finalTransform: PresentationTransform,
    pub requiredCrossAdapterCopy: u8,
    pub colorSpace: super::Dxgi::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ::core::default::Default for CompositionFrameDisplayInstance {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ::core::cmp::PartialEq for CompositionFrameDisplayInstance {
    fn eq(&self, other: &Self) -> bool {
        self.displayAdapterLUID == other.displayAdapterLUID && self.displayVidPnSourceId == other.displayVidPnSourceId && self.displayUniqueId == other.displayUniqueId && self.renderAdapterLUID == other.renderAdapterLUID && self.instanceKind == other.instanceKind && self.finalTransform == other.finalTransform && self.requiredCrossAdapterCopy == other.requiredCrossAdapterCopy && self.colorSpace == other.colorSpace
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ::core::cmp::Eq for CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
unsafe impl ::windows::runtime::Abi for CompositionFrameDisplayInstance {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
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
unsafe impl ::windows::runtime::Abi for CompositionFrameInstanceKind {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[inline]
pub unsafe fn CreatePresentationFactory<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(d3ddevice: Param0, riid: *const ::windows::runtime::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePresentationFactory(d3ddevice: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CreatePresentationFactory(d3ddevice.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(presentationfactory)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ICompositionFramePresentStatistics(pub ::windows::runtime::IUnknown);
impl ICompositionFramePresentStatistics {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetContentTag(&self) -> usize {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(displayinstancearraycount), ::core::mem::transmute(displayinstancearray)))
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionFramePresentStatistics {
    type Vtable = ICompositionFramePresentStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2873217319, 49409, 19466, [145, 29, 249, 242, 233, 208, 142, 100]);
}
impl ::core::convert::From<ICompositionFramePresentStatistics> for ::windows::runtime::IUnknown {
    fn from(value: ICompositionFramePresentStatistics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ICompositionFramePresentStatistics> for ::windows::runtime::IUnknown {
    fn from(value: &ICompositionFramePresentStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for &ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionFramePresentStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> PresentStatisticsKind,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u64,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))] usize,
);
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IIndependentFlipFramePresentStatistics(pub ::windows::runtime::IUnknown);
impl IIndependentFlipFramePresentStatistics {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::default::Default::default();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetOutputVidPnSourceId(&self) -> u32 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetContentTag(&self) -> usize {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetDisplayedTime(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::core::default::Default::default();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentDuration(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::core::default::Default::default();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
}
unsafe impl ::windows::runtime::Interface for IIndependentFlipFramePresentStatistics {
    type Vtable = IIndependentFlipFramePresentStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2358492711, 44436, 19872, [143, 212, 36, 19, 19, 45, 18, 78]);
}
impl ::core::convert::From<IIndependentFlipFramePresentStatistics> for ::windows::runtime::IUnknown {
    fn from(value: IIndependentFlipFramePresentStatistics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IIndependentFlipFramePresentStatistics> for ::windows::runtime::IUnknown {
    fn from(value: &IIndependentFlipFramePresentStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for &IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndependentFlipFramePresentStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> PresentStatisticsKind,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut super::super::Foundation::LUID),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemInterruptTime),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut SystemInterruptTime),
);
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentStatistics(pub ::windows::runtime::IUnknown);
impl IPresentStatistics {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IPresentStatistics {
    type Vtable = IPresentStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3024849882, 29314, 18781, [157, 215, 206, 173, 216, 180, 187, 134]);
}
impl ::core::convert::From<IPresentStatistics> for ::windows::runtime::IUnknown {
    fn from(value: IPresentStatistics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentStatistics> for ::windows::runtime::IUnknown {
    fn from(value: &IPresentStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> PresentStatisticsKind,
);
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentStatusPresentStatistics(pub ::windows::runtime::IUnknown);
impl IPresentStatusPresentStatistics {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentStatus(&self) -> PresentStatus {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IPresentStatusPresentStatistics {
    type Vtable = IPresentStatusPresentStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3387763265, 31179, 17246, [150, 78, 200, 85, 48, 85, 66, 12]);
}
impl ::core::convert::From<IPresentStatusPresentStatistics> for ::windows::runtime::IUnknown {
    fn from(value: IPresentStatusPresentStatistics) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentStatusPresentStatistics> for ::windows::runtime::IUnknown {
    fn from(value: &IPresentStatusPresentStatistics) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for &IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatusPresentStatistics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> PresentStatisticsKind,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> PresentStatus,
);
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationBuffer(pub ::windows::runtime::IUnknown);
impl IPresentationBuffer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn GetAvailableEvent(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn IsAvailable(&self) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u8>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationBuffer {
    type Vtable = IPresentationBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(773946682, 23227, 16696, [154, 19, 167, 117, 89, 60, 137, 202]);
}
impl ::core::convert::From<IPresentationBuffer> for ::windows::runtime::IUnknown {
    fn from(value: IPresentationBuffer) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationBuffer> for ::windows::runtime::IUnknown {
    fn from(value: &IPresentationBuffer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPresentationBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPresentationBuffer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationBuffer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, isavailable: *mut u8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationContent(pub ::windows::runtime::IUnknown);
impl IPresentationContent {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetTag(&self, tag: usize) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tag)))
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationContent {
    type Vtable = IPresentationContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1449704313, 15758, 16732, [178, 21, 243, 128, 32, 242, 210, 82]);
}
impl ::core::convert::From<IPresentationContent> for ::windows::runtime::IUnknown {
    fn from(value: IPresentationContent) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationContent> for ::windows::runtime::IUnknown {
    fn from(value: &IPresentationContent) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPresentationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPresentationContent {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationContent_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tag: usize),
);
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationFactory(pub ::windows::runtime::IUnknown);
impl IPresentationFactory {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn IsPresentationSupported(&self) -> u8 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn IsPresentationSupportedWithIndependentFlip(&self) -> u8 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn CreatePresentationManager(&self) -> ::windows::runtime::Result<IPresentationManager> {
        let mut result__: <IPresentationManager as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPresentationManager>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationFactory {
    type Vtable = IPresentationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2410904408, 7540, 20324, [164, 156, 31, 151, 168, 10, 46, 192]);
}
impl ::core::convert::From<IPresentationFactory> for ::windows::runtime::IUnknown {
    fn from(value: IPresentationFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IPresentationFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPresentationFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPresentationFactory {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u8,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u8,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppresentationmanager: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationManager(pub ::windows::runtime::IUnknown);
impl IPresentationManager {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn AddBufferFromResource<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, resource: Param0) -> ::windows::runtime::Result<IPresentationBuffer> {
        let mut result__: <IPresentationBuffer as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), resource.into_param().abi(), &mut result__).from_abi::<IPresentationBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn CreatePresentationSurface<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, compositionsurfacehandle: Param0) -> ::windows::runtime::Result<IPresentationSurface> {
        let mut result__: <IPresentationSurface as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), compositionsurfacehandle.into_param().abi(), &mut result__).from_abi::<IPresentationSurface>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetNextPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetTargetTime<'a, Param0: ::windows::runtime::IntoParam<'a, SystemInterruptTime>>(&self, targettime: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), targettime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetPreferredPresentDuration<'a, Param0: ::windows::runtime::IntoParam<'a, SystemInterruptTime>, Param1: ::windows::runtime::IntoParam<'a, SystemInterruptTime>>(&self, preferredduration: Param0, deviationtolerance: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), preferredduration.into_param().abi(), deviationtolerance.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(forcevsyncinterrupt)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn Present(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentRetiringFence(&self, riid: *const ::windows::runtime::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(fence)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(presentidtocancelfrom)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn GetLostEvent(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn GetPresentStatisticsAvailableEvent(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(presentstatisticskind), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetNextPresentStatistics(&self) -> ::windows::runtime::Result<IPresentStatistics> {
        let mut result__: <IPresentStatistics as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IPresentStatistics>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationManager {
    type Vtable = IPresentationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4216729474, 25234, 18186, [136, 177, 132, 54, 97, 231, 242, 12]);
}
impl ::core::convert::From<IPresentationManager> for ::windows::runtime::IUnknown {
    fn from(value: IPresentationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationManager> for ::windows::runtime::IUnknown {
    fn from(value: &IPresentationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPresentationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPresentationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resource: ::windows::runtime::RawPtr, presentationbuffer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u64,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targettime: SystemInterruptTime) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, forcevsyncinterrupt: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presentidtocancelfrom: u64) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nextpresentstatistics: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IPresentationSurface(pub ::windows::runtime::IUnknown);
impl IPresentationSurface {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetTag(&self, tag: usize) {
        ::core::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tag)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IPresentationBuffer>>(&self, presentationbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), presentationbuffer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetColorSpace(&self, colorspace: super::Dxgi::DXGI_COLOR_SPACE_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorspace)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetAlphaMode(&self, alphamode: super::Dxgi::DXGI_ALPHA_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(alphamode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(sourcerect)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetTransform(&self, transform: *const PresentationTransform) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(transform)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn RestrictToOutput<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, output: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), output.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetDisableReadback(&self, value: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(leftletterboxsize), ::core::mem::transmute(topletterboxsize), ::core::mem::transmute(rightletterboxsize), ::core::mem::transmute(bottomletterboxsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationSurface {
    type Vtable = IPresentationSurface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2506559739, 59968, 20154, [163, 235, 67, 117, 160, 235, 78, 220]);
}
impl ::core::convert::From<IPresentationSurface> for ::windows::runtime::IUnknown {
    fn from(value: IPresentationSurface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IPresentationSurface> for ::windows::runtime::IUnknown {
    fn from(value: &IPresentationSurface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IPresentationSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IPresentationSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IPresentationContent> for IPresentationSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentationContent> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentationContent> for &IPresentationSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentationContent> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationSurface_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tag: usize),
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presentationbuffer: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, colorspace: super::Dxgi::DXGI_COLOR_SPACE_TYPE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, alphamode: super::Dxgi::DXGI_ALPHA_MODE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sourcerect: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, transform: *const PresentationTransform) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, output: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
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
unsafe impl ::windows::runtime::Abi for PresentStatisticsKind {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
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
unsafe impl ::windows::runtime::Abi for PresentStatus {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
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
unsafe impl ::windows::runtime::Abi for PresentationTransform {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
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
unsafe impl ::windows::runtime::Abi for SystemInterruptTime {
    type Abi = Self;
}
