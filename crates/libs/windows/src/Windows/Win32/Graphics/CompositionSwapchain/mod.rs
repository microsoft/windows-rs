#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[inline]
pub unsafe fn CreatePresentationFactory<P0>(d3ddevice: P0, riid: *const ::windows_core::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
{
    ::windows_targets::link!("dcomp.dll" "system" fn CreatePresentationFactory(d3ddevice : * mut::core::ffi::c_void, riid : *const ::windows_core::GUID, presentationfactory : *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT);
    CreatePresentationFactory(d3ddevice.into_param().abi(), riid, presentationfactory).ok()
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct ICompositionFramePresentStatistics(::windows_core::IUnknown);
impl ICompositionFramePresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        (::windows_core::Interface::vtable(self).base__.GetPresentId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        (::windows_core::Interface::vtable(self).base__.GetKind)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetContentTag(&self) -> usize {
        (::windows_core::Interface::vtable(self).GetContentTag)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        (::windows_core::Interface::vtable(self).GetCompositionFrameId)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
        (::windows_core::Interface::vtable(self).GetDisplayInstanceArray)(::windows_core::Interface::as_raw(self), displayinstancearraycount, displayinstancearray)
    }
}
::windows_core::imp::interface_hierarchy!(ICompositionFramePresentStatistics, ::windows_core::IUnknown, IPresentStatistics);
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
unsafe impl ::windows_core::Interface for ICompositionFramePresentStatistics {
    type Vtable = ICompositionFramePresentStatistics_Vtbl;
}
impl ::core::clone::Clone for ICompositionFramePresentStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ICompositionFramePresentStatistics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xab41d127_c101_4c0a_911d_f9f2e9d08e64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionFramePresentStatistics_Vtbl {
    pub base__: IPresentStatistics_Vtbl,
    pub GetContentTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> usize,
    pub GetCompositionFrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub GetDisplayInstanceArray: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance),
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common")))]
    GetDisplayInstanceArray: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IIndependentFlipFramePresentStatistics(::windows_core::IUnknown);
impl IIndependentFlipFramePresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        (::windows_core::Interface::vtable(self).base__.GetPresentId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        (::windows_core::Interface::vtable(self).base__.GetKind)(::windows_core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetOutputAdapterLUID)(::windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetOutputVidPnSourceId(&self) -> u32 {
        (::windows_core::Interface::vtable(self).GetOutputVidPnSourceId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetContentTag(&self) -> usize {
        (::windows_core::Interface::vtable(self).GetContentTag)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetDisplayedTime(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::core::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetDisplayedTime)(::windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetPresentDuration(&self) -> SystemInterruptTime {
        let mut result__: SystemInterruptTime = ::core::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPresentDuration)(::windows_core::Interface::as_raw(self), &mut result__);
        result__
    }
}
::windows_core::imp::interface_hierarchy!(IIndependentFlipFramePresentStatistics, ::windows_core::IUnknown, IPresentStatistics);
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
unsafe impl ::windows_core::Interface for IIndependentFlipFramePresentStatistics {
    type Vtable = IIndependentFlipFramePresentStatistics_Vtbl;
}
impl ::core::clone::Clone for IIndependentFlipFramePresentStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIndependentFlipFramePresentStatistics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8c93be27_ad94_4da0_8fd4_2413132d124e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndependentFlipFramePresentStatistics_Vtbl {
    pub base__: IPresentStatistics_Vtbl,
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
pub struct IPresentStatistics(::windows_core::IUnknown);
impl IPresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        (::windows_core::Interface::vtable(self).GetPresentId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        (::windows_core::Interface::vtable(self).GetKind)(::windows_core::Interface::as_raw(self))
    }
}
::windows_core::imp::interface_hierarchy!(IPresentStatistics, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IPresentStatistics {
    type Vtable = IPresentStatistics_Vtbl;
}
impl ::core::clone::Clone for IPresentStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPresentStatistics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xb44b8bda_7282_495d_9dd7_ceadd8b4bb86);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatistics_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub GetPresentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentStatusPresentStatistics(::windows_core::IUnknown);
impl IPresentStatusPresentStatistics {
    pub unsafe fn GetPresentId(&self) -> u64 {
        (::windows_core::Interface::vtable(self).base__.GetPresentId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetKind(&self) -> PresentStatisticsKind {
        (::windows_core::Interface::vtable(self).base__.GetKind)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetCompositionFrameId(&self) -> u64 {
        (::windows_core::Interface::vtable(self).GetCompositionFrameId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn GetPresentStatus(&self) -> PresentStatus {
        (::windows_core::Interface::vtable(self).GetPresentStatus)(::windows_core::Interface::as_raw(self))
    }
}
::windows_core::imp::interface_hierarchy!(IPresentStatusPresentStatistics, ::windows_core::IUnknown, IPresentStatistics);
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
unsafe impl ::windows_core::Interface for IPresentStatusPresentStatistics {
    type Vtable = IPresentStatusPresentStatistics_Vtbl;
}
impl ::core::clone::Clone for IPresentStatusPresentStatistics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPresentStatusPresentStatistics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc9ed2a41_79cb_435e_964e_c8553055420c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentStatusPresentStatistics_Vtbl {
    pub base__: IPresentStatistics_Vtbl,
    pub GetCompositionFrameId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub GetPresentStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> PresentStatus,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationBuffer(::windows_core::IUnknown);
impl IPresentationBuffer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAvailableEvent(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetAvailableEvent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsAvailable(&self) -> ::windows_core::Result<u8> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsAvailable)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPresentationBuffer, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IPresentationBuffer {
    type Vtable = IPresentationBuffer_Vtbl;
}
impl ::core::clone::Clone for IPresentationBuffer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPresentationBuffer {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2e217d3a_5abb_4138_9a13_a775593c89ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationBuffer_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAvailableEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAvailableEvent: usize,
    pub IsAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isavailable: *mut u8) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationContent(::windows_core::IUnknown);
impl IPresentationContent {
    pub unsafe fn SetTag(&self, tag: usize) {
        (::windows_core::Interface::vtable(self).SetTag)(::windows_core::Interface::as_raw(self), tag)
    }
}
::windows_core::imp::interface_hierarchy!(IPresentationContent, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IPresentationContent {
    type Vtable = IPresentationContent_Vtbl;
}
impl ::core::clone::Clone for IPresentationContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPresentationContent {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5668bb79_3d8e_415c_b215_f38020f2d252);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationContent_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub SetTag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tag: usize),
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationFactory(::windows_core::IUnknown);
impl IPresentationFactory {
    pub unsafe fn IsPresentationSupported(&self) -> u8 {
        (::windows_core::Interface::vtable(self).IsPresentationSupported)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn IsPresentationSupportedWithIndependentFlip(&self) -> u8 {
        (::windows_core::Interface::vtable(self).IsPresentationSupportedWithIndependentFlip)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn CreatePresentationManager(&self) -> ::windows_core::Result<IPresentationManager> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePresentationManager)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPresentationFactory, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IPresentationFactory {
    type Vtable = IPresentationFactory_Vtbl;
}
impl ::core::clone::Clone for IPresentationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPresentationFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x8fb37b58_1d74_4f64_a49c_1f97a80a2ec0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationFactory_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub IsPresentationSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u8,
    pub IsPresentationSupportedWithIndependentFlip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u8,
    pub CreatePresentationManager: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pppresentationmanager: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationManager(::windows_core::IUnknown);
impl IPresentationManager {
    pub unsafe fn AddBufferFromResource<P0>(&self, resource: P0) -> ::windows_core::Result<IPresentationBuffer>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).AddBufferFromResource)(::windows_core::Interface::as_raw(self), resource.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePresentationSurface<P0>(&self, compositionsurfacehandle: P0) -> ::windows_core::Result<IPresentationSurface>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).CreatePresentationSurface)(::windows_core::Interface::as_raw(self), compositionsurfacehandle.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNextPresentId(&self) -> u64 {
        (::windows_core::Interface::vtable(self).GetNextPresentId)(::windows_core::Interface::as_raw(self))
    }
    pub unsafe fn SetTargetTime(&self, targettime: SystemInterruptTime) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTargetTime)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(targettime)).ok()
    }
    pub unsafe fn SetPreferredPresentDuration(&self, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetPreferredPresentDuration)(::windows_core::Interface::as_raw(self), ::core::mem::transmute(preferredduration), ::core::mem::transmute(deviationtolerance)).ok()
    }
    pub unsafe fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).ForceVSyncInterrupt)(::windows_core::Interface::as_raw(self), forcevsyncinterrupt).ok()
    }
    pub unsafe fn Present(&self) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).Present)(::windows_core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetPresentRetiringFence(&self, riid: *const ::windows_core::GUID) -> ::windows_core::Result<*mut ::core::ffi::c_void> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPresentRetiringFence)(::windows_core::Interface::as_raw(self), riid, &mut result__).from_abi(result__)
    }
    pub unsafe fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).CancelPresentsFrom)(::windows_core::Interface::as_raw(self), presentidtocancelfrom).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLostEvent(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetLostEvent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPresentStatisticsAvailableEvent(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetPresentStatisticsAvailableEvent)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).EnablePresentStatisticsKind)(::windows_core::Interface::as_raw(self), presentstatisticskind, enabled).ok()
    }
    pub unsafe fn GetNextPresentStatistics(&self) -> ::windows_core::Result<IPresentStatistics> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetNextPresentStatistics)(::windows_core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows_core::imp::interface_hierarchy!(IPresentationManager, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IPresentationManager {
    type Vtable = IPresentationManager_Vtbl;
}
impl ::core::clone::Clone for IPresentationManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPresentationManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb562f82_6292_470a_88b1_843661e7f20c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationManager_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub AddBufferFromResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void, presentationbuffer: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CreatePresentationSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreatePresentationSurface: usize,
    pub GetNextPresentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u64,
    pub SetTargetTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targettime: SystemInterruptTime) -> ::windows_core::HRESULT,
    pub SetPreferredPresentDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows_core::HRESULT,
    pub ForceVSyncInterrupt: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, forcevsyncinterrupt: u8) -> ::windows_core::HRESULT,
    pub Present: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetPresentRetiringFence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CancelPresentsFrom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentidtocancelfrom: u64) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetLostEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetLostEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPresentStatisticsAvailableEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPresentStatisticsAvailableEvent: usize,
    pub EnablePresentStatisticsKind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows_core::HRESULT,
    pub GetNextPresentStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nextpresentstatistics: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
pub struct IPresentationSurface(::windows_core::IUnknown);
impl IPresentationSurface {
    pub unsafe fn SetTag(&self, tag: usize) {
        (::windows_core::Interface::vtable(self).base__.SetTag)(::windows_core::Interface::as_raw(self), tag)
    }
    pub unsafe fn SetBuffer<P0>(&self, presentationbuffer: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<IPresentationBuffer>,
    {
        (::windows_core::Interface::vtable(self).SetBuffer)(::windows_core::Interface::as_raw(self), presentationbuffer.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetColorSpace)(::windows_core::Interface::as_raw(self), colorspace).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetAlphaMode)(::windows_core::Interface::as_raw(self), alphamode).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetSourceRect)(::windows_core::Interface::as_raw(self), sourcerect).ok()
    }
    pub unsafe fn SetTransform(&self, transform: *const PresentationTransform) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetTransform)(::windows_core::Interface::as_raw(self), transform).ok()
    }
    pub unsafe fn RestrictToOutput<P0>(&self, output: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::IUnknown>,
    {
        (::windows_core::Interface::vtable(self).RestrictToOutput)(::windows_core::Interface::as_raw(self), output.into_param().abi()).ok()
    }
    pub unsafe fn SetDisableReadback(&self, value: u8) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetDisableReadback)(::windows_core::Interface::as_raw(self), value).ok()
    }
    pub unsafe fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).SetLetterboxingMargins)(::windows_core::Interface::as_raw(self), leftletterboxsize, topletterboxsize, rightletterboxsize, bottomletterboxsize).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IPresentationSurface, ::windows_core::IUnknown, IPresentationContent);
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
unsafe impl ::windows_core::Interface for IPresentationSurface {
    type Vtable = IPresentationSurface_Vtbl;
}
impl ::core::clone::Clone for IPresentationSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IPresentationSurface {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x956710fb_ea40_4eba_a3eb_4375a0eb4edc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPresentationSurface_Vtbl {
    pub base__: IPresentationContent_Vtbl,
    pub SetBuffer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presentationbuffer: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetColorSpace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetColorSpace: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    SetAlphaMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSourceRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSourceRect: usize,
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: *const PresentationTransform) -> ::windows_core::HRESULT,
    pub RestrictToOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, output: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetDisableReadback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u8) -> ::windows_core::HRESULT,
    pub SetLetterboxingMargins: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(0i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = CompositionFrameInstanceKind(2i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(1i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = PresentStatisticsKind(2i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = PresentStatisticsKind(3i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = PresentStatisticsKind(1i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatus_Canceled: PresentStatus = PresentStatus(2i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatus_Queued: PresentStatus = PresentStatus(0i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
pub const PresentStatus_Skipped: PresentStatus = PresentStatus(1i32);
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CompositionFrameInstanceKind(pub i32);
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
impl ::windows_core::TypeKind for CompositionFrameInstanceKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for CompositionFrameInstanceKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CompositionFrameInstanceKind").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PresentStatisticsKind(pub i32);
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
impl ::windows_core::TypeKind for PresentStatisticsKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PresentStatisticsKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PresentStatisticsKind").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_CompositionSwapchain\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PresentStatus(pub i32);
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
impl ::windows_core::TypeKind for PresentStatus {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for PresentStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PresentStatus").field(&self.0).finish()
    }
}
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
impl ::windows_core::TypeKind for CompositionFrameDisplayInstance {
    type TypeKind = ::windows_core::CopyType;
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
impl ::core::default::Default for CompositionFrameDisplayInstance {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
impl ::windows_core::TypeKind for PresentationTransform {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PresentationTransform {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M21 == other.M21 && self.M22 == other.M22 && self.M31 == other.M31 && self.M32 == other.M32
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
impl ::windows_core::TypeKind for SystemInterruptTime {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for SystemInterruptTime {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
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
