#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
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
impl ::core::marker::Copy for CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for CompositionFrameDisplayInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for CompositionFrameDisplayInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CompositionFrameDisplayInstance").field("displayAdapterLUID", &self.displayAdapterLUID).field("displayVidPnSourceId", &self.displayVidPnSourceId).field("displayUniqueId", &self.displayUniqueId).field("renderAdapterLUID", &self.renderAdapterLUID).field("instanceKind", &self.instanceKind).field("finalTransform", &self.finalTransform).field("requiredCrossAdapterCopy", &self.requiredCrossAdapterCopy).field("colorSpace", &self.colorSpace).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
unsafe impl ::windows::core::Abi for CompositionFrameDisplayInstance {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for CompositionFrameDisplayInstance {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CompositionFrameDisplayInstance>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for CompositionFrameDisplayInstance {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CompositionFrameInstanceKind(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(0i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(1i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = CompositionFrameInstanceKind(2i32);
impl ::core::marker::Copy for CompositionFrameInstanceKind {}
impl ::core::clone::Clone for CompositionFrameInstanceKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CompositionFrameInstanceKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CompositionFrameInstanceKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for CompositionFrameInstanceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionFrameInstanceKind").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[inline]
pub unsafe fn CreatePresentationFactory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(d3ddevice: Param0, riid: *const ::windows::core::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePresentationFactory(d3ddevice: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        CreatePresentationFactory(d3ddevice.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(presentationfactory)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct ICompositionFramePresentStatistics(::windows::core::IUnknown);
impl ICompositionFramePresentStatistics {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetPresentId)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetKind)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetContentTag(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetContentTag)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetCompositionFrameId)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
        (::windows::core::Interface::vtable(self).GetDisplayInstanceArray)(::core::mem::transmute_copy(self), ::core::mem::transmute(displayinstancearraycount), ::core::mem::transmute(displayinstancearray))
    }
}
impl ::core::convert::From<ICompositionFramePresentStatistics> for ::windows::core::IUnknown {
    fn from(value: ICompositionFramePresentStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICompositionFramePresentStatistics> for ::windows::core::IUnknown {
    fn from(value: &ICompositionFramePresentStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPresentStatistics> for &'a ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentStatistics> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICompositionFramePresentStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICompositionFramePresentStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICompositionFramePresentStatistics {}
impl ::core::fmt::Debug for ICompositionFramePresentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICompositionFramePresentStatistics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICompositionFramePresentStatistics {
    type Vtable = ICompositionFramePresentStatistics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab41d127_c101_4c0a_911d_f9f2e9d08e64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionFramePresentStatistics_Vtbl {
    pub base: IPresentStatistics_Vtbl,
    pub GetContentTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
    pub GetCompositionFrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDisplayInstanceArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDisplayInstanceArray: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IIndependentFlipFramePresentStatistics(::windows::core::IUnknown);
impl IIndependentFlipFramePresentStatistics {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetPresentId)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetKind)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOutputAdapterLUID)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetOutputVidPnSourceId(&self) -> u32 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetOutputVidPnSourceId)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetContentTag(&self) -> usize {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetContentTag)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetDisplayedTime(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDisplayedTime)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetPresentDuration(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPresentDuration)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
}
impl ::core::convert::From<IIndependentFlipFramePresentStatistics> for ::windows::core::IUnknown {
    fn from(value: IIndependentFlipFramePresentStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIndependentFlipFramePresentStatistics> for ::windows::core::IUnknown {
    fn from(value: &IIndependentFlipFramePresentStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPresentStatistics> for &'a IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentStatistics> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIndependentFlipFramePresentStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IIndependentFlipFramePresentStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIndependentFlipFramePresentStatistics {}
impl ::core::fmt::Debug for IIndependentFlipFramePresentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIndependentFlipFramePresentStatistics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IIndependentFlipFramePresentStatistics {
    type Vtable = IIndependentFlipFramePresentStatistics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c93be27_ad94_4da0_8fd4_2413132d124e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndependentFlipFramePresentStatistics_Vtbl {
    pub base: IPresentStatistics_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetOutputAdapterLUID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID),
    #[cfg(not(feature = "Win32_Foundation"))]
    GetOutputAdapterLUID: usize,
    pub GetOutputVidPnSourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub GetContentTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
    pub GetDisplayedTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime),
    pub GetPresentDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime),
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentStatistics(::windows::core::IUnknown);
impl IPresentStatistics {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetPresentId)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetKind)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IPresentStatistics> for ::windows::core::IUnknown {
    fn from(value: IPresentStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPresentStatistics> for ::windows::core::IUnknown {
    fn from(value: &IPresentStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPresentStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPresentStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentStatistics {}
impl ::core::fmt::Debug for IPresentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentStatistics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPresentStatistics {
    type Vtable = IPresentStatistics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb44b8bda_7282_495d_9dd7_ceadd8b4bb86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatistics_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub GetPresentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentStatusPresentStatistics(::windows::core::IUnknown);
impl IPresentStatusPresentStatistics {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetPresentId)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).base.GetKind)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetCompositionFrameId)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetPresentStatus(&self) -> PresentStatus {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetPresentStatus)(::core::mem::transmute_copy(self)))
    }
}
impl ::core::convert::From<IPresentStatusPresentStatistics> for ::windows::core::IUnknown {
    fn from(value: IPresentStatusPresentStatistics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPresentStatusPresentStatistics> for ::windows::core::IUnknown {
    fn from(value: &IPresentStatusPresentStatistics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPresentStatistics> for &'a IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentStatistics> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPresentStatusPresentStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPresentStatusPresentStatistics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentStatusPresentStatistics {}
impl ::core::fmt::Debug for IPresentStatusPresentStatistics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentStatusPresentStatistics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPresentStatusPresentStatistics {
    type Vtable = IPresentStatusPresentStatistics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9ed2a41_79cb_435e_964e_c8553055420c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatusPresentStatistics_Vtbl {
    pub base: IPresentStatistics_Vtbl,
    pub GetCompositionFrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetPresentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> PresentStatus,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationBuffer(::windows::core::IUnknown);
impl IPresentationBuffer {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAvailableEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAvailableEvent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn IsAvailable(&self) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsAvailable)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
    }
}
impl ::core::convert::From<IPresentationBuffer> for ::windows::core::IUnknown {
    fn from(value: IPresentationBuffer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPresentationBuffer> for ::windows::core::IUnknown {
    fn from(value: &IPresentationBuffer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationBuffer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPresentationBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPresentationBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationBuffer {}
impl ::core::fmt::Debug for IPresentationBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationBuffer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPresentationBuffer {
    type Vtable = IPresentationBuffer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e217d3a_5abb_4138_9a13_a775593c89ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationBuffer_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAvailableEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAvailableEvent: usize,
    pub IsAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isavailable: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationContent(::windows::core::IUnknown);
impl IPresentationContent {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn SetTag(&self, tag: usize) {
        (::windows::core::Interface::vtable(self).SetTag)(::core::mem::transmute_copy(self), ::core::mem::transmute(tag))
    }
}
impl ::core::convert::From<IPresentationContent> for ::windows::core::IUnknown {
    fn from(value: IPresentationContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPresentationContent> for ::windows::core::IUnknown {
    fn from(value: &IPresentationContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPresentationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPresentationContent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationContent {}
impl ::core::fmt::Debug for IPresentationContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationContent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPresentationContent {
    type Vtable = IPresentationContent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5668bb79_3d8e_415c_b215_f38020f2d252);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationContent_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: usize),
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationFactory(::windows::core::IUnknown);
impl IPresentationFactory {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn IsPresentationSupported(&self) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsPresentationSupported)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn IsPresentationSupportedWithIndependentFlip(&self) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).IsPresentationSupportedWithIndependentFlip)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn CreatePresentationManager(&self) -> ::windows::core::Result<IPresentationManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePresentationManager)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPresentationManager>(result__)
    }
}
impl ::core::convert::From<IPresentationFactory> for ::windows::core::IUnknown {
    fn from(value: IPresentationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPresentationFactory> for ::windows::core::IUnknown {
    fn from(value: &IPresentationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPresentationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPresentationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationFactory {}
impl ::core::fmt::Debug for IPresentationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPresentationFactory {
    type Vtable = IPresentationFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fb37b58_1d74_4f64_a49c_1f97a80a2ec0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationFactory_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub IsPresentationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u8,
    pub IsPresentationSupportedWithIndependentFlip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u8,
    pub CreatePresentationManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppresentationmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationManager(::windows::core::IUnknown);
impl IPresentationManager {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn AddBufferFromResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, resource: Param0) -> ::windows::core::Result<IPresentationBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddBufferFromResource)(::core::mem::transmute_copy(self), resource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPresentationBuffer>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePresentationSurface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, compositionsurfacehandle: Param0) -> ::windows::core::Result<IPresentationSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreatePresentationSurface)(::core::mem::transmute_copy(self), compositionsurfacehandle.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPresentationSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetNextPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).GetNextPresentId)(::core::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn SetTargetTime<'a, Param0: ::windows::core::IntoParam<'a, SystemInterruptTime>>(&self, targettime: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTargetTime)(::core::mem::transmute_copy(self), targettime.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn SetPreferredPresentDuration<'a, Param0: ::windows::core::IntoParam<'a, SystemInterruptTime>, Param1: ::windows::core::IntoParam<'a, SystemInterruptTime>>(&self, preferredduration: Param0, deviationtolerance: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPreferredPresentDuration)(::core::mem::transmute_copy(self), preferredduration.into_param().abi(), deviationtolerance.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ForceVSyncInterrupt)(::core::mem::transmute_copy(self), ::core::mem::transmute(forcevsyncinterrupt)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn Present(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Present)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetPresentRetiringFence(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut ::core::ffi::c_void> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPresentRetiringFence)(::core::mem::transmute_copy(self), ::core::mem::transmute(riid), ::core::mem::transmute(&mut result__)).from_abi::<*mut ::core::ffi::c_void>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CancelPresentsFrom)(::core::mem::transmute_copy(self), ::core::mem::transmute(presentidtocancelfrom)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLostEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetLostEvent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresentStatisticsAvailableEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPresentStatisticsAvailableEvent)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnablePresentStatisticsKind)(::core::mem::transmute_copy(self), ::core::mem::transmute(presentstatisticskind), ::core::mem::transmute(enabled)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn GetNextPresentStatistics(&self) -> ::windows::core::Result<IPresentStatistics> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetNextPresentStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPresentStatistics>(result__)
    }
}
impl ::core::convert::From<IPresentationManager> for ::windows::core::IUnknown {
    fn from(value: IPresentationManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPresentationManager> for ::windows::core::IUnknown {
    fn from(value: &IPresentationManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPresentationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPresentationManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationManager {}
impl ::core::fmt::Debug for IPresentationManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPresentationManager {
    type Vtable = IPresentationManager_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb562f82_6292_470a_88b1_843661e7f20c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationManager_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddBufferFromResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void, presentationbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePresentationSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePresentationSurface: usize,
    pub GetNextPresentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub SetTargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targettime: SystemInterruptTime) -> ::windows::core::HRESULT,
    pub SetPreferredPresentDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows::core::HRESULT,
    pub ForceVSyncInterrupt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forcevsyncinterrupt: u8) -> ::windows::core::HRESULT,
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPresentRetiringFence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CancelPresentsFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentidtocancelfrom: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLostEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLostEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPresentStatisticsAvailableEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPresentStatisticsAvailableEvent: usize,
    pub EnablePresentStatisticsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::HRESULT,
    pub GetNextPresentStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextpresentstatistics: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationSurface(::windows::core::IUnknown);
impl IPresentationSurface {
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn SetTag(&self, tag: usize) {
        (::windows::core::Interface::vtable(self).base.SetTag)(::core::mem::transmute_copy(self), ::core::mem::transmute(tag))
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn SetBuffer<'a, Param0: ::windows::core::IntoParam<'a, IPresentationBuffer>>(&self, presentationbuffer: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBuffer)(::core::mem::transmute_copy(self), presentationbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColorSpace)(::core::mem::transmute_copy(self), ::core::mem::transmute(colorspace)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(alphamode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSourceRect)(::core::mem::transmute_copy(self), ::core::mem::transmute(sourcerect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn SetTransform(&self, transform: *const PresentationTransform) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(transform)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn RestrictToOutput<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, output: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestrictToOutput)(::core::mem::transmute_copy(self), output.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn SetDisableReadback(&self, value: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisableReadback)(::core::mem::transmute_copy(self), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
    pub unsafe fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLetterboxingMargins)(::core::mem::transmute_copy(self), ::core::mem::transmute(leftletterboxsize), ::core::mem::transmute(topletterboxsize), ::core::mem::transmute(rightletterboxsize), ::core::mem::transmute(bottomletterboxsize)).ok()
    }
}
impl ::core::convert::From<IPresentationSurface> for ::windows::core::IUnknown {
    fn from(value: IPresentationSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPresentationSurface> for ::windows::core::IUnknown {
    fn from(value: &IPresentationSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPresentationSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPresentationSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, IPresentationContent> for &'a IPresentationSurface {
    fn into_param(self) -> ::windows::core::Param<'a, IPresentationContent> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPresentationSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPresentationSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPresentationSurface {}
impl ::core::fmt::Debug for IPresentationSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPresentationSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPresentationSurface {
    type Vtable = IPresentationSurface_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x956710fb_ea40_4eba_a3eb_4375a0eb4edc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationSurface_Vtbl {
    pub base: IPresentationContent_Vtbl,
    pub SetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentationbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetAlphaMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSourceRect: usize,
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *const PresentationTransform) -> ::windows::core::HRESULT,
    pub RestrictToOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, output: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetDisableReadback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub SetLetterboxingMargins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PresentStatisticsKind(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = PresentStatisticsKind(1i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = PresentStatisticsKind(2i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = PresentStatisticsKind(3i32);
impl ::core::marker::Copy for PresentStatisticsKind {}
impl ::core::clone::Clone for PresentStatisticsKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PresentStatisticsKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PresentStatisticsKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PresentStatisticsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PresentStatisticsKind").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PresentStatus(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatus_Queued: PresentStatus = PresentStatus(0i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatus_Skipped: PresentStatus = PresentStatus(1i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatus_Canceled: PresentStatus = PresentStatus(2i32);
impl ::core::marker::Copy for PresentStatus {}
impl ::core::clone::Clone for PresentStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PresentStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PresentStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for PresentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PresentStatus").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub struct PresentationTransform {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
impl ::core::marker::Copy for PresentationTransform {}
impl ::core::clone::Clone for PresentationTransform {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PresentationTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PresentationTransform").field("M11", &self.M11).field("M12", &self.M12).field("M21", &self.M21).field("M22", &self.M22).field("M31", &self.M31).field("M32", &self.M32).finish()
    }
}
unsafe impl ::windows::core::Abi for PresentationTransform {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PresentationTransform {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PresentationTransform>()) == 0 }
    }
}
impl ::core::cmp::Eq for PresentationTransform {}
impl ::core::default::Default for PresentationTransform {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub struct SystemInterruptTime {
    pub value: u64,
}
impl ::core::marker::Copy for SystemInterruptTime {}
impl ::core::clone::Clone for SystemInterruptTime {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SystemInterruptTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SystemInterruptTime").field("value", &self.value).finish()
    }
}
unsafe impl ::windows::core::Abi for SystemInterruptTime {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SystemInterruptTime {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SystemInterruptTime>()) == 0 }
    }
}
impl ::core::cmp::Eq for SystemInterruptTime {}
impl ::core::default::Default for SystemInterruptTime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
