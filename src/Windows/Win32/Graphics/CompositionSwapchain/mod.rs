#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
impl ::core::marker::Copy for CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for CompositionFrameDisplayInstance {
    fn clone(&self) -> Self {
        *self
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
pub type CompositionFrameInstanceKind = i32;
pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = 0i32;
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = 1i32;
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = 2i32;
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
#[repr(transparent)]
pub struct ICompositionFramePresentStatistics(::windows::core::IUnknown);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &ICompositionFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for ICompositionFramePresentStatistics {
    type Vtable = ICompositionFramePresentStatisticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab41d127_c101_4c0a_911d_f9f2e9d08e64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionFramePresentStatisticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))] usize,
);
#[repr(transparent)]
pub struct IIndependentFlipFramePresentStatistics(::windows::core::IUnknown);
impl IIndependentFlipFramePresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
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
        let mut result__: SystemInterruptTime = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__);
        result__
    }
    pub unsafe fn GetPresentDuration(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__);
        result__
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IIndependentFlipFramePresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IIndependentFlipFramePresentStatistics {
    type Vtable = IIndependentFlipFramePresentStatisticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c93be27_ad94_4da0_8fd4_2413132d124e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndependentFlipFramePresentStatisticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID),
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime),
);
#[repr(transparent)]
pub struct IPresentStatistics(::windows::core::IUnknown);
impl IPresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPresentStatistics {
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
unsafe impl ::windows::core::Interface for IPresentStatistics {
    type Vtable = IPresentStatisticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb44b8bda_7282_495d_9dd7_ceadd8b4bb86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatisticsVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind);
#[repr(transparent)]
pub struct IPresentStatusPresentStatistics(::windows::core::IUnknown);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPresentStatusPresentStatistics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IPresentStatusPresentStatistics {
    type Vtable = IPresentStatusPresentStatisticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9ed2a41_79cb_435e_964e_c8553055420c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatusPresentStatisticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> PresentStatus,
);
#[repr(transparent)]
pub struct IPresentationBuffer(::windows::core::IUnknown);
impl IPresentationBuffer {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAvailableEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    pub unsafe fn IsAvailable(&self) -> ::windows::core::Result<u8> {
        let mut result__: u8 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u8>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPresentationBuffer {
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
unsafe impl ::windows::core::Interface for IPresentationBuffer {
    type Vtable = IPresentationBufferVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e217d3a_5abb_4138_9a13_a775593c89ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationBufferVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isavailable: *mut u8) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IPresentationContent(::windows::core::IUnknown);
impl IPresentationContent {
    pub unsafe fn SetTag(&self, tag: usize) {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(tag)))
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPresentationContent {
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
unsafe impl ::windows::core::Interface for IPresentationContent {
    type Vtable = IPresentationContentVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5668bb79_3d8e_415c_b215_f38020f2d252);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationContentVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: usize));
#[repr(transparent)]
pub struct IPresentationFactory(::windows::core::IUnknown);
impl IPresentationFactory {
    pub unsafe fn IsPresentationSupported(&self) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn IsPresentationSupportedWithIndependentFlip(&self) -> u8 {
        ::core::mem::transmute((::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)))
    }
    pub unsafe fn CreatePresentationManager(&self) -> ::windows::core::Result<IPresentationManager> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPresentationManager>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPresentationFactory {
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
unsafe impl ::windows::core::Interface for IPresentationFactory {
    type Vtable = IPresentationFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8fb37b58_1d74_4f64_a49c_1f97a80a2ec0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u8,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u8,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppresentationmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IPresentationManager(::windows::core::IUnknown);
impl IPresentationManager {
    pub unsafe fn AddBufferFromResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, resource: Param0) -> ::windows::core::Result<IPresentationBuffer> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), resource.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPresentationBuffer>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePresentationSurface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, compositionsurfacehandle: Param0) -> ::windows::core::Result<IPresentationSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), compositionsurfacehandle.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IPresentationSurface>(result__)
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
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresentStatisticsAvailableEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    pub unsafe fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(presentstatisticskind), ::core::mem::transmute(enabled)).ok()
    }
    pub unsafe fn GetNextPresentStatistics(&self) -> ::windows::core::Result<IPresentStatistics> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IPresentStatistics>(result__)
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPresentationManager {
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
unsafe impl ::windows::core::Interface for IPresentationManager {
    type Vtable = IPresentationManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb562f82_6292_470a_88b1_843661e7f20c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void, presentationbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targettime: SystemInterruptTime) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forcevsyncinterrupt: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentidtocancelfrom: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextpresentstatistics: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IPresentationSurface(::windows::core::IUnknown);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IPresentationSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
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
unsafe impl ::windows::core::Interface for IPresentationSurface {
    type Vtable = IPresentationSurfaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x956710fb_ea40_4eba_a3eb_4375a0eb4edc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationSurfaceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: usize),
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentationbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *const PresentationTransform) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, output: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::HRESULT,
);
pub type PresentStatisticsKind = i32;
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = 1i32;
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = 2i32;
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = 3i32;
pub type PresentStatus = i32;
pub const PresentStatus_Queued: PresentStatus = 0i32;
pub const PresentStatus_Skipped: PresentStatus = 1i32;
pub const PresentStatus_Canceled: PresentStatus = 2i32;
#[repr(C)]
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
pub struct SystemInterruptTime {
    pub value: u64,
}
impl ::core::marker::Copy for SystemInterruptTime {}
impl ::core::clone::Clone for SystemInterruptTime {
    fn clone(&self) -> Self {
        *self
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
