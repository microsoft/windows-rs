#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for CompositionFrameDisplayInstance {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ::std::fmt::Debug for CompositionFrameDisplayInstance {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
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
impl ::std::cmp::PartialEq for CompositionFrameDisplayInstance {
    fn eq(&self, other: &Self) -> bool {
        self.displayAdapterLUID == other.displayAdapterLUID && self.displayVidPnSourceId == other.displayVidPnSourceId && self.displayUniqueId == other.displayUniqueId && self.renderAdapterLUID == other.renderAdapterLUID && self.instanceKind == other.instanceKind && self.finalTransform == other.finalTransform && self.requiredCrossAdapterCopy == other.requiredCrossAdapterCopy && self.colorSpace == other.colorSpace
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
impl ::std::cmp::Eq for CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
unsafe impl ::windows::runtime::Abi for CompositionFrameDisplayInstance {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CompositionFrameInstanceKind(pub i32);
pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(0i32);
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(1i32);
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = CompositionFrameInstanceKind(2i32);
impl ::std::convert::From<i32> for CompositionFrameInstanceKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CompositionFrameInstanceKind {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[inline]
pub unsafe fn CreatePresentationFactory<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(d3ddevice: Param0, riid: *const ::windows::runtime::GUID, presentationfactory: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePresentationFactory(d3ddevice: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, presentationfactory: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        CreatePresentationFactory(d3ddevice.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(presentationfactory)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICompositionFramePresentStatistics(::windows::runtime::IUnknown);
impl ICompositionFramePresentStatistics {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetContentTag(&self) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(displayinstancearraycount), ::std::mem::transmute(displayinstancearray)))
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionFramePresentStatistics {
    type Vtable = ICompositionFramePresentStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2873217319, 49409, 19466, [145, 29, 249, 242, 233, 208, 142, 100]);
}
impl ::std::convert::From<ICompositionFramePresentStatistics> for IPresentStatistics {
    fn from(value: ICompositionFramePresentStatistics) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICompositionFramePresentStatistics> for IPresentStatistics {
    fn from(value: &ICompositionFramePresentStatistics) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPresentStatistics>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for &ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPresentStatistics>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IIndependentFlipFramePresentStatistics(::windows::runtime::IUnknown);
impl IIndependentFlipFramePresentStatistics {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::std::default::Default::default();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetOutputVidPnSourceId(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetContentTag(&self) -> usize {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetDisplayedTime(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::std::default::Default::default();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentDuration(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::std::default::Default::default();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__);
        result__
    }
}
unsafe impl ::windows::runtime::Interface for IIndependentFlipFramePresentStatistics {
    type Vtable = IIndependentFlipFramePresentStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2358492711, 44436, 19872, [143, 212, 36, 19, 19, 45, 18, 78]);
}
impl ::std::convert::From<IIndependentFlipFramePresentStatistics> for IPresentStatistics {
    fn from(value: IIndependentFlipFramePresentStatistics) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IIndependentFlipFramePresentStatistics> for IPresentStatistics {
    fn from(value: &IIndependentFlipFramePresentStatistics) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPresentStatistics>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for &IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPresentStatistics>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPresentStatistics(::windows::runtime::IUnknown);
impl IPresentStatistics {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IPresentStatistics {
    type Vtable = IPresentStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3024849882, 29314, 18781, [157, 215, 206, 173, 216, 180, 187, 134]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPresentStatusPresentStatistics(::windows::runtime::IUnknown);
impl IPresentStatusPresentStatistics {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentStatus(&self) -> PresentStatus {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)))
    }
}
unsafe impl ::windows::runtime::Interface for IPresentStatusPresentStatistics {
    type Vtable = IPresentStatusPresentStatistics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3387763265, 31179, 17246, [150, 78, 200, 85, 48, 85, 66, 12]);
}
impl ::std::convert::From<IPresentStatusPresentStatistics> for IPresentStatistics {
    fn from(value: IPresentStatusPresentStatistics) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPresentStatusPresentStatistics> for IPresentStatistics {
    fn from(value: &IPresentStatusPresentStatistics) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPresentStatistics>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentStatistics> for &IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentStatistics> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPresentStatistics>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPresentationBuffer(::windows::runtime::IUnknown);
impl IPresentationBuffer {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn GetAvailableEvent(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn IsAvailable(&self) -> ::windows::runtime::Result<u8> {
        let mut result__: <u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u8>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationBuffer {
    type Vtable = IPresentationBuffer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(773946682, 23227, 16696, [154, 19, 167, 117, 89, 60, 137, 202]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPresentationContent(::windows::runtime::IUnknown);
impl IPresentationContent {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetTag(&self, tag: usize) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(tag)))
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationContent {
    type Vtable = IPresentationContent_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1449704313, 15758, 16732, [178, 21, 243, 128, 32, 242, 210, 82]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPresentationFactory(::windows::runtime::IUnknown);
impl IPresentationFactory {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn IsPresentationSupported(&self) -> u8 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn IsPresentationSupportedWithIndependentFlip(&self) -> u8 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn CreatePresentationManager(&self) -> ::windows::runtime::Result<IPresentationManager> {
        let mut result__: <IPresentationManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IPresentationManager>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationFactory {
    type Vtable = IPresentationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2410904408, 7540, 20324, [164, 156, 31, 151, 168, 10, 46, 192]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPresentationManager(::windows::runtime::IUnknown);
impl IPresentationManager {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn AddBufferFromResource<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, resource: Param0) -> ::windows::runtime::Result<IPresentationBuffer> {
        let mut result__: <IPresentationBuffer as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), resource.into_param().abi(), &mut result__).from_abi::<IPresentationBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn CreatePresentationSurface<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, compositionsurfacehandle: Param0) -> ::windows::runtime::Result<IPresentationSurface> {
        let mut result__: <IPresentationSurface as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), compositionsurfacehandle.into_param().abi(), &mut result__).from_abi::<IPresentationSurface>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetNextPresentId(&self) -> u64 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetTargetTime<'a, Param0: ::windows::runtime::IntoParam<'a, SystemInterruptTime>>(&self, targettime: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), targettime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetPreferredPresentDuration<'a, Param0: ::windows::runtime::IntoParam<'a, SystemInterruptTime>, Param1: ::windows::runtime::IntoParam<'a, SystemInterruptTime>>(&self, preferredduration: Param0, deviationtolerance: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), preferredduration.into_param().abi(), deviationtolerance.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(forcevsyncinterrupt)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn Present(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetPresentRetiringFence(&self, riid: *const ::windows::runtime::GUID, fence: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(fence)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(presentidtocancelfrom)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn GetLostEvent(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn GetPresentStatisticsAvailableEvent(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(presentstatisticskind), ::std::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn GetNextPresentStatistics(&self) -> ::windows::runtime::Result<IPresentStatistics> {
        let mut result__: <IPresentStatistics as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IPresentStatistics>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationManager {
    type Vtable = IPresentationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4216729474, 25234, 18186, [136, 177, 132, 54, 97, 231, 242, 12]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, fence: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPresentationSurface(::windows::runtime::IUnknown);
impl IPresentationSurface {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetTag(&self, tag: usize) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(tag)))
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, IPresentationBuffer>>(&self, presentationbuffer: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), presentationbuffer.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetColorSpace(&self, colorspace: super::Dxgi::DXGI_COLOR_SPACE_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(colorspace)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetAlphaMode(&self, alphamode: super::Dxgi::DXGI_ALPHA_MODE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(alphamode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`, `Win32_Foundation`*"]
    pub unsafe fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(sourcerect)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetTransform(&self, transform: *const PresentationTransform) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(transform)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn RestrictToOutput<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, output: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), output.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetDisableReadback(&self, value: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub unsafe fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(leftletterboxsize), ::std::mem::transmute(topletterboxsize), ::std::mem::transmute(rightletterboxsize), ::std::mem::transmute(bottomletterboxsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPresentationSurface {
    type Vtable = IPresentationSurface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2506559739, 59968, 20154, [163, 235, 67, 117, 160, 235, 78, 220]);
}
impl ::std::convert::From<IPresentationSurface> for IPresentationContent {
    fn from(value: IPresentationSurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPresentationSurface> for IPresentationContent {
    fn from(value: &IPresentationSurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentationContent> for IPresentationSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentationContent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPresentationContent>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPresentationContent> for &IPresentationSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPresentationContent> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPresentationContent>::into(::std::clone::Clone::clone(self)))
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PresentStatisticsKind(pub i32);
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = PresentStatisticsKind(1i32);
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = PresentStatisticsKind(2i32);
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = PresentStatisticsKind(3i32);
impl ::std::convert::From<i32> for PresentStatisticsKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PresentStatisticsKind {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PresentStatus(pub i32);
pub const PresentStatus_Queued: PresentStatus = PresentStatus(0i32);
pub const PresentStatus_Skipped: PresentStatus = PresentStatus(1i32);
pub const PresentStatus_Canceled: PresentStatus = PresentStatus(2i32);
impl ::std::convert::From<i32> for PresentStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PresentStatus {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
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
impl ::std::default::Default for PresentationTransform {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PresentationTransform {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PresentationTransform").field("M11", &self.M11).field("M12", &self.M12).field("M21", &self.M21).field("M22", &self.M22).field("M31", &self.M31).field("M32", &self.M32).finish()
    }
}
impl ::std::cmp::PartialEq for PresentationTransform {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M21 == other.M21 && self.M22 == other.M22 && self.M31 == other.M31 && self.M32 == other.M32
    }
}
impl ::std::cmp::Eq for PresentationTransform {}
unsafe impl ::windows::runtime::Abi for PresentationTransform {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
pub struct SystemInterruptTime {
    pub value: u64,
}
impl SystemInterruptTime {}
impl ::std::default::Default for SystemInterruptTime {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SystemInterruptTime {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SystemInterruptTime").field("value", &self.value).finish()
    }
}
impl ::std::cmp::PartialEq for SystemInterruptTime {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl ::std::cmp::Eq for SystemInterruptTime {}
unsafe impl ::windows::runtime::Abi for SystemInterruptTime {
    type Abi = Self;
}
