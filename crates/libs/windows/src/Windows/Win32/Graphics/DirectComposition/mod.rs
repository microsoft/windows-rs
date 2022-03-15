#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct COMPOSITION_FRAME_ID_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(2i32);
impl ::core::marker::Copy for COMPOSITION_FRAME_ID_TYPE {}
impl ::core::clone::Clone for COMPOSITION_FRAME_ID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for COMPOSITION_FRAME_ID_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for COMPOSITION_FRAME_ID_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for COMPOSITION_FRAME_ID_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("COMPOSITION_FRAME_ID_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
impl ::core::marker::Copy for COMPOSITION_FRAME_STATS {}
impl ::core::clone::Clone for COMPOSITION_FRAME_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_FRAME_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_FRAME_STATS").field("startTime", &self.startTime).field("targetTime", &self.targetTime).field("framePeriod", &self.framePeriod).finish()
    }
}
unsafe impl ::windows::core::Abi for COMPOSITION_FRAME_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPOSITION_FRAME_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITION_FRAME_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPOSITION_FRAME_STATS {}
impl ::core::default::Default for COMPOSITION_FRAME_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
impl ::core::marker::Copy for COMPOSITION_STATS {}
impl ::core::clone::Clone for COMPOSITION_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_STATS").field("presentCount", &self.presentCount).field("refreshCount", &self.refreshCount).field("virtualRefreshCount", &self.virtualRefreshCount).field("time", &self.time).finish()
    }
}
unsafe impl ::windows::core::Abi for COMPOSITION_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPOSITION_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITION_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPOSITION_STATS {}
impl ::core::default::Default for COMPOSITION_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: super::super::Foundation::LUID,
    pub renderAdapterLuid: super::super::Foundation::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPOSITION_TARGET_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPOSITION_TARGET_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_TARGET_ID").field("displayAdapterLuid", &self.displayAdapterLuid).field("renderAdapterLuid", &self.renderAdapterLuid).field("vidPnSourceId", &self.vidPnSourceId).field("vidPnTargetId", &self.vidPnTargetId).field("uniqueId", &self.uniqueId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMPOSITION_TARGET_ID {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITION_TARGET_ID>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPOSITION_TARGET_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
impl ::core::marker::Copy for COMPOSITION_TARGET_STATS {}
impl ::core::clone::Clone for COMPOSITION_TARGET_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMPOSITION_TARGET_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMPOSITION_TARGET_STATS").field("outstandingPresents", &self.outstandingPresents).field("presentTime", &self.presentTime).field("vblankDuration", &self.vblankDuration).field("presentedStats", &self.presentedStats).field("completedStats", &self.completedStats).finish()
    }
}
unsafe impl ::windows::core::Abi for COMPOSITION_TARGET_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMPOSITION_TARGET_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMPOSITION_TARGET_STATS {}
impl ::core::default::Default for COMPOSITION_TARGET_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BACKFACE_VISIBILITY {}
impl ::core::clone::Clone for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_BACKFACE_VISIBILITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BACKFACE_VISIBILITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BITMAP_INTERPOLATION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_BORDER_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BORDER_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BORDER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_BORDER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_BORDER_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_BORDER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_BORDER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_COMPOSITE_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(2i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_COMPOSITE_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_COMPOSITE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_COMPOSITE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_COMPOSITE_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_COMPOSITE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_COMPOSITE_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_DEPTH_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(3i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_DEPTH_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_DEPTH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_DEPTH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_DEPTH_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_DEPTH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_DEPTH_MODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DCOMPOSITION_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DCOMPOSITION_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCOMPOSITION_FRAME_STATISTICS").field("lastFrameTime", &self.lastFrameTime).field("currentCompositionRate", &self.currentCompositionRate).field("currentTime", &self.currentTime).field("timeFrequency", &self.timeFrequency).field("nextEstimatedFrameTime", &self.nextEstimatedFrameTime).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for DCOMPOSITION_FRAME_STATISTICS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DCOMPOSITION_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCOMPOSITION_FRAME_STATISTICS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DCOMPOSITION_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DCOMPOSITION_OPACITY_MODE(pub i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(0i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(1i32);
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_OPACITY_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_OPACITY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DCOMPOSITION_OPACITY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_OPACITY_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for DCOMPOSITION_OPACITY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DCOMPOSITION_OPACITY_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionAttachMouseDragToHwnd<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(visual: Param0, hwnd: Param1, enable: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionAttachMouseDragToHwnd(visual: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        DCompositionAttachMouseDragToHwnd(visual.into_param().abi(), hwnd.into_param().abi(), enable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionAttachMouseWheelToHwnd<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(visual: Param0, hwnd: Param1, enable: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionAttachMouseWheelToHwnd(visual: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        DCompositionAttachMouseWheelToHwnd(visual.into_param().abi(), hwnd.into_param().abi(), enable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionBoostCompositorClock<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(enable: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionBoostCompositorClock(enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        DCompositionBoostCompositorClock(enable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[inline]
pub unsafe fn DCompositionCreateDevice<'a, Param0: ::windows::core::IntoParam<'a, super::Dxgi::IDXGIDevice>>(dxgidevice: Param0, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice(dxgidevice: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DCompositionCreateDevice(dxgidevice.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(dcompositiondevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[inline]
pub unsafe fn DCompositionCreateDevice2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(renderingdevice: Param0, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice2(renderingdevice: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DCompositionCreateDevice2(renderingdevice.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(dcompositiondevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[inline]
pub unsafe fn DCompositionCreateDevice3<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(renderingdevice: Param0, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice3(renderingdevice: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DCompositionCreateDevice3(renderingdevice.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(dcompositiondevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, surfacehandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        DCompositionCreateSurfaceHandle(::core::mem::transmute(desiredaccess), ::core::mem::transmute(securityattributes), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[inline]
pub unsafe fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE, frameid: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: u64 = ::core::mem::zeroed();
        DCompositionGetFrameId(::core::mem::transmute(frameidtype), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: *mut COMPOSITION_TARGET_ID, actualtargetidcount: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: *mut COMPOSITION_TARGET_ID, actualtargetidcount: *mut u32) -> ::windows::core::HRESULT;
        }
        DCompositionGetStatistics(::core::mem::transmute(frameid), ::core::mem::transmute(framestats), ::core::mem::transmute(targetidcount), ::core::mem::transmute(targetids), ::core::mem::transmute(actualtargetidcount)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID) -> ::windows::core::Result<COMPOSITION_TARGET_STATS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID, targetstats: *mut COMPOSITION_TARGET_STATS) -> ::windows::core::HRESULT;
        }
        let mut result__: COMPOSITION_TARGET_STATS = ::core::mem::zeroed();
        DCompositionGetTargetStatistics(::core::mem::transmute(frameid), ::core::mem::transmute(targetid), ::core::mem::transmute(&mut result__)).from_abi::<COMPOSITION_TARGET_STATS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl ::core::marker::Copy for DCompositionInkTrailPoint {}
impl ::core::clone::Clone for DCompositionInkTrailPoint {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DCompositionInkTrailPoint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DCompositionInkTrailPoint").field("x", &self.x).field("y", &self.y).field("radius", &self.radius).finish()
    }
}
unsafe impl ::windows::core::Abi for DCompositionInkTrailPoint {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DCompositionInkTrailPoint {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DCompositionInkTrailPoint>()) == 0 }
    }
}
impl ::core::cmp::Eq for DCompositionInkTrailPoint {}
impl ::core::default::Default for DCompositionInkTrailPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionWaitForCompositorClock(handles: &[super::super::Foundation::HANDLE], timeoutinms: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionWaitForCompositorClock(count: u32, handles: *const super::super::Foundation::HANDLE, timeoutinms: u32) -> u32;
        }
        ::core::mem::transmute(DCompositionWaitForCompositorClock(handles.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(handles)), ::core::mem::transmute(timeoutinms)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionAffineTransform2DEffect(::windows::core::IUnknown);
impl IDCompositionAffineTransform2DEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBorderMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransformMatrix(&self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformMatrix)(::core::mem::transmute_copy(self), ::core::mem::transmute(transformmatrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransformMatrixElement<'a, Param2: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformMatrixElement)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformMatrixElement2)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetSharpness<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSharpness)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetSharpness2(&self, sharpness: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSharpness2)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharpness)).ok()
    }
}
impl ::core::convert::From<IDCompositionAffineTransform2DEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionAffineTransform2DEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionAffineTransform2DEffect> for IDCompositionEffect {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionAffineTransform2DEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionAffineTransform2DEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionAffineTransform2DEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionAffineTransform2DEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionAffineTransform2DEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionAffineTransform2DEffect {}
impl ::core::fmt::Debug for IDCompositionAffineTransform2DEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionAffineTransform2DEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionAffineTransform2DEffect {
    type Vtable = IDCompositionAffineTransform2DEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b74b9e8_cdd6_492f_bbbc_5ed32157026d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAffineTransform2DEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: super::Direct2D::Common::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetInterpolationMode: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransformMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransformMatrix: usize,
    pub SetTransformMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTransformMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
    pub SetSharpness: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetSharpness2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sharpness: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionAnimation(::windows::core::IUnknown);
impl IDCompositionAnimation {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAbsoluteBeginTime(&self, begintime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAbsoluteBeginTime)(::core::mem::transmute_copy(self), ::core::mem::transmute(begintime)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddCubic)(::core::mem::transmute_copy(self), ::core::mem::transmute(beginoffset), ::core::mem::transmute(constantcoefficient), ::core::mem::transmute(linearcoefficient), ::core::mem::transmute(quadraticcoefficient), ::core::mem::transmute(cubiccoefficient)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddSinusoidal)(::core::mem::transmute_copy(self), ::core::mem::transmute(beginoffset), ::core::mem::transmute(bias), ::core::mem::transmute(amplitude), ::core::mem::transmute(frequency), ::core::mem::transmute(phase)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddRepeat)(::core::mem::transmute_copy(self), ::core::mem::transmute(beginoffset), ::core::mem::transmute(durationtorepeat)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn End(&self, endoffset: f64, endvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).End)(::core::mem::transmute_copy(self), ::core::mem::transmute(endoffset), ::core::mem::transmute(endvalue)).ok()
    }
}
impl ::core::convert::From<IDCompositionAnimation> for ::windows::core::IUnknown {
    fn from(value: IDCompositionAnimation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionAnimation> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionAnimation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionAnimation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionAnimation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionAnimation {}
impl ::core::fmt::Debug for IDCompositionAnimation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionAnimation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionAnimation {
    type Vtable = IDCompositionAnimation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAnimation_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetAbsoluteBeginTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, begintime: i64) -> ::windows::core::HRESULT,
    pub AddCubic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT,
    pub AddSinusoidal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT,
    pub AddRepeat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::HRESULT,
    pub End: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endoffset: f64, endvalue: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionArithmeticCompositeEffect(::windows::core::IUnknown);
impl IDCompositionArithmeticCompositeEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetCoefficients(&self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficients)(::core::mem::transmute_copy(self), ::core::mem::transmute(coefficients)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clampoutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClampOutput)(::core::mem::transmute_copy(self), clampoutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCoefficient1<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient1)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCoefficient12(&self, coeffcient1: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient12)(::core::mem::transmute_copy(self), ::core::mem::transmute(coeffcient1)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCoefficient2<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient2)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCoefficient22(&self, coefficient2: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient22)(::core::mem::transmute_copy(self), ::core::mem::transmute(coefficient2)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCoefficient3<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCoefficient32(&self, coefficient3: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient32)(::core::mem::transmute_copy(self), ::core::mem::transmute(coefficient3)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCoefficient4<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient4)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCoefficient42(&self, coefficient4: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCoefficient42)(::core::mem::transmute_copy(self), ::core::mem::transmute(coefficient4)).ok()
    }
}
impl ::core::convert::From<IDCompositionArithmeticCompositeEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionArithmeticCompositeEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionArithmeticCompositeEffect> for IDCompositionEffect {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionArithmeticCompositeEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionArithmeticCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionArithmeticCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionArithmeticCompositeEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionArithmeticCompositeEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionArithmeticCompositeEffect {}
impl ::core::fmt::Debug for IDCompositionArithmeticCompositeEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionArithmeticCompositeEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionArithmeticCompositeEffect {
    type Vtable = IDCompositionArithmeticCompositeEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b67dfa8_e3dd_4e61_b640_46c2f3d739dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionArithmeticCompositeEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetCoefficients: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetCoefficients: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
    pub SetCoefficient1: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCoefficient12: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coeffcient1: f32) -> ::windows::core::HRESULT,
    pub SetCoefficient2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCoefficient22: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient2: f32) -> ::windows::core::HRESULT,
    pub SetCoefficient3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCoefficient32: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient3: f32) -> ::windows::core::HRESULT,
    pub SetCoefficient4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCoefficient42: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, coefficient4: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionBlendEffect(::windows::core::IUnknown);
impl IDCompositionBlendEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
}
impl ::core::convert::From<IDCompositionBlendEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionBlendEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBlendEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionBlendEffect> for IDCompositionEffect {
    fn from(value: IDCompositionBlendEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBlendEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionBlendEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionBlendEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBlendEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionBlendEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionBlendEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionBlendEffect {}
impl ::core::fmt::Debug for IDCompositionBlendEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionBlendEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionBlendEffect {
    type Vtable = IDCompositionBlendEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ecdc0a_578a_4a11_9c14_0cb90517f9c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBlendEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionBrightnessEffect(::windows::core::IUnknown);
impl IDCompositionBrightnessEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetWhitePoint(&self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(whitepoint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBlackPoint(&self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlackPoint)(::core::mem::transmute_copy(self), ::core::mem::transmute(blackpoint)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetWhitePointX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePointX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetWhitePointX2(&self, whitepointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePointX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(whitepointx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetWhitePointY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePointY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetWhitePointY2(&self, whitepointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWhitePointY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(whitepointy)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlackPointX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlackPointX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlackPointX2(&self, blackpointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlackPointX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(blackpointx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlackPointY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlackPointY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlackPointY2(&self, blackpointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlackPointY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(blackpointy)).ok()
    }
}
impl ::core::convert::From<IDCompositionBrightnessEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBrightnessEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionBrightnessEffect> for IDCompositionEffect {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBrightnessEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionBrightnessEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionBrightnessEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionBrightnessEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionBrightnessEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionBrightnessEffect {}
impl ::core::fmt::Debug for IDCompositionBrightnessEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionBrightnessEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionBrightnessEffect {
    type Vtable = IDCompositionBrightnessEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6027496e_cb3a_49ab_934f_d798da4f7da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBrightnessEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetWhitePoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetWhitePoint: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBlackPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBlackPoint: usize,
    pub SetWhitePointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetWhitePointX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointx: f32) -> ::windows::core::HRESULT,
    pub SetWhitePointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetWhitePointY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, whitepointy: f32) -> ::windows::core::HRESULT,
    pub SetBlackPointX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBlackPointX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpointx: f32) -> ::windows::core::HRESULT,
    pub SetBlackPointY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBlackPointY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blackpointy: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionClip(::windows::core::IUnknown);
impl IDCompositionClip {}
impl ::core::convert::From<IDCompositionClip> for ::windows::core::IUnknown {
    fn from(value: IDCompositionClip) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionClip> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionClip) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionClip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionClip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionClip {}
impl ::core::fmt::Debug for IDCompositionClip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionClip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionClip {
    type Vtable = IDCompositionClip_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64ac3703_9d3f_45ec_a109_7cac0e7a13a7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionClip_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionColorMatrixEffect(::windows::core::IUnknown);
impl IDCompositionColorMatrixEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrix)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetMatrixElement<'a, Param2: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixElement)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixElement2)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clamp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClampOutput)(::core::mem::transmute_copy(self), clamp.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionColorMatrixEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionColorMatrixEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionColorMatrixEffect> for IDCompositionEffect {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionColorMatrixEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionColorMatrixEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionColorMatrixEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionColorMatrixEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionColorMatrixEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionColorMatrixEffect {}
impl ::core::fmt::Debug for IDCompositionColorMatrixEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionColorMatrixEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionColorMatrixEffect {
    type Vtable = IDCompositionColorMatrixEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1170a22_3ce2_4966_90d4_55408bfc84c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionColorMatrixEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetAlphaMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetAlphaMode: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clamp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionCompositeEffect(::windows::core::IUnknown);
impl IDCompositionCompositeEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
}
impl ::core::convert::From<IDCompositionCompositeEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionCompositeEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionCompositeEffect> for IDCompositionEffect {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionCompositeEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionCompositeEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionCompositeEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionCompositeEffect {}
impl ::core::fmt::Debug for IDCompositionCompositeEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionCompositeEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionCompositeEffect {
    type Vtable = IDCompositionCompositeEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x576616c0_a231_494d_a38d_00fd5ec4db46);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionCompositeEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetMode: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDelegatedInkTrail(::windows::core::IUnknown);
impl IDCompositionDelegatedInkTrail {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn AddTrailPoints(&self, inkpoints: &[DCompositionInkTrailPoint]) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddTrailPoints)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(inkpoints)), inkpoints.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn AddTrailPointsWithPrediction(&self, inkpoints: &[DCompositionInkTrailPoint], predictedinkpoints: &[DCompositionInkTrailPoint]) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AddTrailPointsWithPrediction)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(inkpoints)), inkpoints.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(predictedinkpoints)), predictedinkpoints.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn RemoveTrailPoints(&self, generationid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveTrailPoints)(::core::mem::transmute_copy(self), ::core::mem::transmute(generationid)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartNewTrail)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
}
impl ::core::convert::From<IDCompositionDelegatedInkTrail> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDelegatedInkTrail) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDelegatedInkTrail> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDelegatedInkTrail) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDelegatedInkTrail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDelegatedInkTrail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDelegatedInkTrail {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDelegatedInkTrail {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDelegatedInkTrail {}
impl ::core::fmt::Debug for IDCompositionDelegatedInkTrail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDelegatedInkTrail").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDelegatedInkTrail {
    type Vtable = IDCompositionDelegatedInkTrail_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2448e9b_547d_4057_8cf5_8144ede1c2da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDelegatedInkTrail_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub AddTrailPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT,
    pub AddTrailPointsWithPrediction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT,
    pub RemoveTrailPoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, generationid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub StartNewTrail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    StartNewTrail: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDesktopDevice(::windows::core::IUnknown);
impl IDCompositionDesktopDevice {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.WaitForCommitCompletion)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: DCOMPOSITION_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateVisual)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionVisual2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateSurfaceFactory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows::core::Result<IDCompositionSurfaceFactory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSurfaceFactory)(::core::mem::transmute_copy(self), renderingdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateVirtualSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateTranslateTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateScaleTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionScaleTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRotateTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRotateTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSkewTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSkewTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateTransformGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(transforms)), transforms.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateTranslateTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateScaleTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRotateTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateMatrixTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateTransform3DGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(transforms3d)), transforms3d.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateEffectGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionEffectGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRectangleClip)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRectangleClip>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateAnimation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionAnimation>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTargetForHwnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hwnd: Param0, topmost: Param1) -> ::windows::core::Result<IDCompositionTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTargetForHwnd)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), topmost.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTarget>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, handle: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSurfaceFromHandle)(::core::mem::transmute_copy(self), handle.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHwnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSurfaceFromHwnd)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<IDCompositionDesktopDevice> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDesktopDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDesktopDevice> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDesktopDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionDesktopDevice> for IDCompositionDevice2 {
    fn from(value: IDCompositionDesktopDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDesktopDevice> for IDCompositionDevice2 {
    fn from(value: &IDCompositionDesktopDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionDevice2> for IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionDevice2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionDevice2> for &'a IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionDevice2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDesktopDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDesktopDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDesktopDevice {}
impl ::core::fmt::Debug for IDCompositionDesktopDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDesktopDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDesktopDevice {
    type Vtable = IDCompositionDesktopDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f4633fe_1e08_4cb8_8c75_ce24333f5602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDesktopDevice_Vtbl {
    pub base: IDCompositionDevice2_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetForHwnd: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHwnd: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDevice(::windows::core::IUnknown);
impl IDCompositionDevice {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForCommitCompletion)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: DCOMPOSITION_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTargetForHwnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hwnd: Param0, topmost: Param1) -> ::windows::core::Result<IDCompositionTarget> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTargetForHwnd)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), topmost.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTarget>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVisual)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionVisual>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVirtualSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, handle: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSurfaceFromHandle)(::core::mem::transmute_copy(self), handle.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHwnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSurfaceFromHwnd)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTranslateTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateScaleTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionScaleTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRotateTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRotateTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSkewTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSkewTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTransformGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(transforms)), transforms.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTranslateTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateScaleTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRotateTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateMatrixTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTransform3DGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(transforms3d)), transforms3d.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateEffectGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionEffectGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRectangleClip)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRectangleClip>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAnimation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionAnimation>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CheckDeviceState)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
impl ::core::convert::From<IDCompositionDevice> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDevice> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDevice {}
impl ::core::fmt::Debug for IDCompositionDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDevice {
    type Vtable = IDCompositionDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc37ea93a_e7aa_450d_b16f_9746cb0407f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateTargetForHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateTargetForHwnd: usize,
    pub CreateVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: super::super::Foundation::HANDLE, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHandle: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateSurfaceFromHwnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, surface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateSurfaceFromHwnd: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CheckDeviceState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CheckDeviceState: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDevice2(::windows::core::IUnknown);
impl IDCompositionDevice2 {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).WaitForCommitCompletion)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: DCOMPOSITION_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVisual)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionVisual2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateSurfaceFactory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows::core::Result<IDCompositionSurfaceFactory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSurfaceFactory)(::core::mem::transmute_copy(self), renderingdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVirtualSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTranslateTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateScaleTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionScaleTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRotateTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRotateTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSkewTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSkewTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTransformGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(transforms)), transforms.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTranslateTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateScaleTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRotateTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateMatrixTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTransform3DGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(transforms3d)), transforms3d.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateEffectGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionEffectGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateRectangleClip)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRectangleClip>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAnimation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionAnimation>(result__)
    }
}
impl ::core::convert::From<IDCompositionDevice2> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDevice2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDevice2> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDevice2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDevice2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDevice2 {}
impl ::core::fmt::Debug for IDCompositionDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDevice2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDevice2 {
    type Vtable = IDCompositionDevice2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75f6468d_1b8e_447c_9bc6_75fea80b5b25);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Commit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub WaitForCommitCompletion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetFrameStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetFrameStatistics: usize,
    pub CreateVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSurfaceFactory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, renderingdevice: *mut ::core::ffi::c_void, surfacefactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
    pub CreateTranslateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateScaleTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateRotateTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSkewTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateTransformGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateTranslateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateScaleTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateRotateTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateMatrixTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateTransform3DGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateEffectGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateRectangleClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateAnimation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDevice3(::windows::core::IUnknown);
impl IDCompositionDevice3 {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Commit)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.WaitForCommitCompletion)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: DCOMPOSITION_FRAME_STATISTICS = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetFrameStatistics)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateVisual)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionVisual2>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateSurfaceFactory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows::core::Result<IDCompositionSurfaceFactory> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSurfaceFactory)(::core::mem::transmute_copy(self), renderingdevice.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateVirtualSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateTranslateTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateScaleTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionScaleTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRotateTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRotateTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateSkewTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSkewTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateMatrixTransform)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTransformGroup(&self, transforms: &[::core::option::Option<IDCompositionTransform>]) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateTransformGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(transforms)), transforms.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTransform>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateTranslateTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateScaleTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRotateTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateMatrixTransform3D)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: &[::core::option::Option<IDCompositionTransform3D>]) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateTransform3DGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(transforms3d)), transforms3d.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTransform3D>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateEffectGroup)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionEffectGroup>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateRectangleClip)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionRectangleClip>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CreateAnimation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionAnimation>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateGaussianBlurEffect(&self) -> ::windows::core::Result<IDCompositionGaussianBlurEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateGaussianBlurEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionGaussianBlurEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateBrightnessEffect(&self) -> ::windows::core::Result<IDCompositionBrightnessEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBrightnessEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionBrightnessEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateColorMatrixEffect(&self) -> ::windows::core::Result<IDCompositionColorMatrixEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateColorMatrixEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionColorMatrixEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateShadowEffect(&self) -> ::windows::core::Result<IDCompositionShadowEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateShadowEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionShadowEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateHueRotationEffect(&self) -> ::windows::core::Result<IDCompositionHueRotationEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateHueRotationEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionHueRotationEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateSaturationEffect(&self) -> ::windows::core::Result<IDCompositionSaturationEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSaturationEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSaturationEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTurbulenceEffect(&self) -> ::windows::core::Result<IDCompositionTurbulenceEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTurbulenceEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTurbulenceEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateLinearTransferEffect(&self) -> ::windows::core::Result<IDCompositionLinearTransferEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateLinearTransferEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionLinearTransferEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateTableTransferEffect(&self) -> ::windows::core::Result<IDCompositionTableTransferEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateTableTransferEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionTableTransferEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateCompositeEffect(&self) -> ::windows::core::Result<IDCompositionCompositeEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateCompositeEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionCompositeEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateBlendEffect(&self) -> ::windows::core::Result<IDCompositionBlendEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateBlendEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionBlendEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateArithmeticCompositeEffect(&self) -> ::windows::core::Result<IDCompositionArithmeticCompositeEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateArithmeticCompositeEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionArithmeticCompositeEffect>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateAffineTransform2DEffect(&self) -> ::windows::core::Result<IDCompositionAffineTransform2DEffect> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAffineTransform2DEffect)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionAffineTransform2DEffect>(result__)
    }
}
impl ::core::convert::From<IDCompositionDevice3> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDevice3> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionDevice3> for IDCompositionDevice2 {
    fn from(value: IDCompositionDevice3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDevice3> for IDCompositionDevice2 {
    fn from(value: &IDCompositionDevice3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionDevice2> for IDCompositionDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionDevice2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionDevice2> for &'a IDCompositionDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionDevice2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDevice3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDevice3 {}
impl ::core::fmt::Debug for IDCompositionDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDevice3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDevice3 {
    type Vtable = IDCompositionDevice3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0987cb06_f916_48bf_8d35_ce7641781bd9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice3_Vtbl {
    pub base: IDCompositionDevice2_Vtbl,
    pub CreateGaussianBlurEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, gaussianblureffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateBrightnessEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, brightnesseffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateColorMatrixEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, colormatrixeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateShadowEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, shadoweffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateHueRotationEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, huerotationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateSaturationEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, saturationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateTurbulenceEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, turbulenceeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateLinearTransferEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lineartransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateTableTransferEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tabletransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateCompositeEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateBlendEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blendeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateArithmeticCompositeEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, arithmeticcompositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateAffineTransform2DEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, affinetransform2deffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionDeviceDebug(::windows::core::IUnknown);
impl IDCompositionDeviceDebug {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn EnableDebugCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableDebugCounters)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn DisableDebugCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableDebugCounters)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDCompositionDeviceDebug> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDeviceDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionDeviceDebug> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDeviceDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDeviceDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDeviceDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionDeviceDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionDeviceDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionDeviceDebug {}
impl ::core::fmt::Debug for IDCompositionDeviceDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionDeviceDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDeviceDebug {
    type Vtable = IDCompositionDeviceDebug_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1a3c64a_224f_4a81_9773_4f03a89d3c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDeviceDebug_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub EnableDebugCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableDebugCounters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionEffect(::windows::core::IUnknown);
impl IDCompositionEffect {}
impl ::core::convert::From<IDCompositionEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionEffect {}
impl ::core::fmt::Debug for IDCompositionEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionEffect {
    type Vtable = IDCompositionEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec81b08f_bfcb_4e8d_b193_a915587999e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffect_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionEffectGroup(::windows::core::IUnknown);
impl IDCompositionEffectGroup {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOpacity<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacity)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacity2)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransform3D<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform3D>>(&self, transform3d: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransform3D)(::core::mem::transmute_copy(self), transform3d.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionEffectGroup> for ::windows::core::IUnknown {
    fn from(value: IDCompositionEffectGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionEffectGroup> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionEffectGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionEffectGroup> for IDCompositionEffect {
    fn from(value: IDCompositionEffectGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionEffectGroup> for IDCompositionEffect {
    fn from(value: &IDCompositionEffectGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionEffectGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionEffectGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionEffectGroup {}
impl ::core::fmt::Debug for IDCompositionEffectGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionEffectGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionEffectGroup {
    type Vtable = IDCompositionEffectGroup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7929a74_e6b2_4bd6_8b95_4040119ca34d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffectGroup_Vtbl {
    pub base: IDCompositionEffect_Vtbl,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub SetTransform3D: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform3d: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionFilterEffect(::windows::core::IUnknown);
impl IDCompositionFilterEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
}
impl ::core::convert::From<IDCompositionFilterEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionFilterEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionFilterEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionFilterEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionFilterEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionFilterEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionFilterEffect> for IDCompositionEffect {
    fn from(value: IDCompositionFilterEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionFilterEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionFilterEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionFilterEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionFilterEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionFilterEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionFilterEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionFilterEffect {}
impl ::core::fmt::Debug for IDCompositionFilterEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionFilterEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionFilterEffect {
    type Vtable = IDCompositionFilterEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30c421d5_8cb2_4e9f_b133_37be270d4ac2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionFilterEffect_Vtbl {
    pub base: IDCompositionEffect_Vtbl,
    pub SetInput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, input: *mut ::core::ffi::c_void, flags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionGaussianBlurEffect(::windows::core::IUnknown);
impl IDCompositionGaussianBlurEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetStandardDeviation<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStandardDeviation)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStandardDeviation2)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBorderMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
}
impl ::core::convert::From<IDCompositionGaussianBlurEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionGaussianBlurEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionGaussianBlurEffect> for IDCompositionEffect {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionGaussianBlurEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionGaussianBlurEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionGaussianBlurEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionGaussianBlurEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionGaussianBlurEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionGaussianBlurEffect {}
impl ::core::fmt::Debug for IDCompositionGaussianBlurEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionGaussianBlurEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionGaussianBlurEffect {
    type Vtable = IDCompositionGaussianBlurEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d4d0b7_1bd4_454e_8894_2bfa68443033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionGaussianBlurEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBorderMode: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionHueRotationEffect(::windows::core::IUnknown);
impl IDCompositionHueRotationEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngle<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngle)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngle2(&self, amountdegrees: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngle2)(::core::mem::transmute_copy(self), ::core::mem::transmute(amountdegrees)).ok()
    }
}
impl ::core::convert::From<IDCompositionHueRotationEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionHueRotationEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionHueRotationEffect> for IDCompositionEffect {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionHueRotationEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionHueRotationEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionHueRotationEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionHueRotationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionHueRotationEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionHueRotationEffect {}
impl ::core::fmt::Debug for IDCompositionHueRotationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionHueRotationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionHueRotationEffect {
    type Vtable = IDCompositionHueRotationEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6db9f920_0770_4781_b0c6_381912f9d167);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionHueRotationEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amountdegrees: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionInkTrailDevice(::windows::core::IUnknown);
impl IDCompositionInkTrailDevice {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateDelegatedInkTrail(&self) -> ::windows::core::Result<IDCompositionDelegatedInkTrail> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDelegatedInkTrail)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionDelegatedInkTrail>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn CreateDelegatedInkTrailForSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, swapchain: Param0) -> ::windows::core::Result<IDCompositionDelegatedInkTrail> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateDelegatedInkTrailForSwapChain)(::core::mem::transmute_copy(self), swapchain.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionDelegatedInkTrail>(result__)
    }
}
impl ::core::convert::From<IDCompositionInkTrailDevice> for ::windows::core::IUnknown {
    fn from(value: IDCompositionInkTrailDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionInkTrailDevice> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionInkTrailDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionInkTrailDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionInkTrailDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionInkTrailDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionInkTrailDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionInkTrailDevice {}
impl ::core::fmt::Debug for IDCompositionInkTrailDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionInkTrailDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionInkTrailDevice {
    type Vtable = IDCompositionInkTrailDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0c7cec_cdeb_4d4a_b91c_721bf22f4e6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionInkTrailDevice_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub CreateDelegatedInkTrail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateDelegatedInkTrailForSwapChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, swapchain: *mut ::core::ffi::c_void, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionLinearTransferEffect(::windows::core::IUnknown);
impl IDCompositionLinearTransferEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRedYIntercept<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedYIntercept)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRedYIntercept2(&self, redyintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedYIntercept2)(::core::mem::transmute_copy(self), ::core::mem::transmute(redyintercept)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRedSlope<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedSlope)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRedSlope2(&self, redslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedSlope2)(::core::mem::transmute_copy(self), ::core::mem::transmute(redslope)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRedDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, reddisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedDisable)(::core::mem::transmute_copy(self), reddisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetGreenYIntercept<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenYIntercept)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetGreenYIntercept2(&self, greenyintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenYIntercept2)(::core::mem::transmute_copy(self), ::core::mem::transmute(greenyintercept)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetGreenSlope<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenSlope)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetGreenSlope2(&self, greenslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenSlope2)(::core::mem::transmute_copy(self), ::core::mem::transmute(greenslope)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGreenDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, greendisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenDisable)(::core::mem::transmute_copy(self), greendisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlueYIntercept<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueYIntercept)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlueYIntercept2(&self, blueyintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueYIntercept2)(::core::mem::transmute_copy(self), ::core::mem::transmute(blueyintercept)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlueSlope<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueSlope)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlueSlope2(&self, blueslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueSlope2)(::core::mem::transmute_copy(self), ::core::mem::transmute(blueslope)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlueDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bluedisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueDisable)(::core::mem::transmute_copy(self), bluedisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAlphaYIntercept<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaYIntercept)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaYIntercept2)(::core::mem::transmute_copy(self), ::core::mem::transmute(alphayintercept)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAlphaSlope<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaSlope)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAlphaSlope2(&self, alphaslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaSlope2)(::core::mem::transmute_copy(self), ::core::mem::transmute(alphaslope)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlphaDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, alphadisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaDisable)(::core::mem::transmute_copy(self), alphadisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clampoutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClampOutput)(::core::mem::transmute_copy(self), clampoutput.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionLinearTransferEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionLinearTransferEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionLinearTransferEffect> for IDCompositionEffect {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionLinearTransferEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionLinearTransferEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionLinearTransferEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionLinearTransferEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionLinearTransferEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionLinearTransferEffect {}
impl ::core::fmt::Debug for IDCompositionLinearTransferEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionLinearTransferEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionLinearTransferEffect {
    type Vtable = IDCompositionLinearTransferEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4305ee5b_c4a0_4c88_9385_67124e017683);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionLinearTransferEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    pub SetRedYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRedYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redyintercept: f32) -> ::windows::core::HRESULT,
    pub SetRedSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRedSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, redslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRedDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRedDisable: usize,
    pub SetGreenYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetGreenYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greenyintercept: f32) -> ::windows::core::HRESULT,
    pub SetGreenSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetGreenSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greenslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGreenDisable: usize,
    pub SetBlueYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBlueYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blueyintercept: f32) -> ::windows::core::HRESULT,
    pub SetBlueSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBlueSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blueslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlueDisable: usize,
    pub SetAlphaYIntercept: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAlphaYIntercept2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphayintercept: f32) -> ::windows::core::HRESULT,
    pub SetAlphaSlope: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAlphaSlope2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphaslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphaDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionMatrixTransform(::windows::core::IUnknown);
impl IDCompositionMatrixTransform {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrix)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetMatrixElement<'a, Param2: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixElement)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixElement2)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform> for IDCompositionEffect {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform> for IDCompositionTransform {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionMatrixTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionMatrixTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionMatrixTransform {}
impl ::core::fmt::Debug for IDCompositionMatrixTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionMatrixTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionMatrixTransform {
    type Vtable = IDCompositionMatrixTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16cdff07_c503_419c_83f2_0965c7af1fa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform_Vtbl {
    pub base: IDCompositionTransform_Vtbl,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionMatrixTransform3D(::windows::core::IUnknown);
impl IDCompositionMatrixTransform3D {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn SetMatrix(&self, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrix)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetMatrixElement<'a, Param2: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixElement)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMatrixElement2)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionMatrixTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionMatrixTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionMatrixTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionMatrixTransform3D {}
impl ::core::fmt::Debug for IDCompositionMatrixTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionMatrixTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionMatrixTransform3D {
    type Vtable = IDCompositionMatrixTransform3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b3363f0_643b_41b7_b6e0_ccf22d34467c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform3D_Vtbl {
    pub base: IDCompositionTransform3D_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))]
    SetMatrix: usize,
    pub SetMatrixElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetMatrixElement2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionRectangleClip(::windows::core::IUnknown);
impl IDCompositionRectangleClip {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetLeft<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLeft)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetLeft2(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLeft2)(::core::mem::transmute_copy(self), ::core::mem::transmute(left)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTop<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTop)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTop2(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTop2)(::core::mem::transmute_copy(self), ::core::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRight<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRight)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRight2(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRight2)(::core::mem::transmute_copy(self), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottom<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottom)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottom2(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottom2)(::core::mem::transmute_copy(self), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTopLeftRadiusX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopLeftRadiusX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTopLeftRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopLeftRadiusX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTopLeftRadiusY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopLeftRadiusY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTopLeftRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopLeftRadiusY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTopRightRadiusX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopRightRadiusX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTopRightRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopRightRadiusX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTopRightRadiusY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopRightRadiusY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTopRightRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTopRightRadiusY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottomLeftRadiusX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomLeftRadiusX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottomLeftRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomLeftRadiusX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottomLeftRadiusY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomLeftRadiusY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottomLeftRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomLeftRadiusY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottomRightRadiusX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomRightRadiusX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottomRightRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomRightRadiusX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottomRightRadiusY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomRightRadiusY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBottomRightRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottomRightRadiusY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
}
impl ::core::convert::From<IDCompositionRectangleClip> for ::windows::core::IUnknown {
    fn from(value: IDCompositionRectangleClip) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRectangleClip> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionRectangleClip) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionRectangleClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionRectangleClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRectangleClip> for IDCompositionClip {
    fn from(value: IDCompositionRectangleClip) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRectangleClip> for IDCompositionClip {
    fn from(value: &IDCompositionRectangleClip) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionClip> for IDCompositionRectangleClip {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionClip> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionClip> for &'a IDCompositionRectangleClip {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionClip> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionRectangleClip {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionRectangleClip {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionRectangleClip {}
impl ::core::fmt::Debug for IDCompositionRectangleClip {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionRectangleClip").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionRectangleClip {
    type Vtable = IDCompositionRectangleClip_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9842ad7d_d9cf_4908_aed7_48b51da5e7c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRectangleClip_Vtbl {
    pub base: IDCompositionClip_Vtbl,
    pub SetLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetLeft2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: f32) -> ::windows::core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTop2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: f32) -> ::windows::core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRight2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: f32) -> ::windows::core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBottom2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: f32) -> ::windows::core::HRESULT,
    pub SetTopLeftRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTopLeftRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetTopLeftRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTopLeftRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetTopRightRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTopRightRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetTopRightRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetTopRightRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetBottomLeftRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBottomLeftRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetBottomLeftRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBottomLeftRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetBottomRightRadiusX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBottomRightRadiusX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
    pub SetBottomRightRadiusY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBottomRightRadiusY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, radius: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionRotateTransform(::windows::core::IUnknown);
impl IDCompositionRotateTransform {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngle<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngle)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngle2)(::core::mem::transmute_copy(self), ::core::mem::transmute(angle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
}
impl ::core::convert::From<IDCompositionRotateTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform> for IDCompositionEffect {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform> for IDCompositionTransform {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionRotateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionRotateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionRotateTransform {}
impl ::core::fmt::Debug for IDCompositionRotateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionRotateTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionRotateTransform {
    type Vtable = IDCompositionRotateTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x641ed83c_ae96_46c5_90dc_32774cc5c6d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform_Vtbl {
    pub base: IDCompositionTransform_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionRotateTransform3D(::windows::core::IUnknown);
impl IDCompositionRotateTransform3D {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngle<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngle)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngle2)(::core::mem::transmute_copy(self), ::core::mem::transmute(angle)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAxisX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAxisX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAxisX2(&self, axisx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAxisX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(axisx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAxisY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAxisY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAxisY2(&self, axisy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAxisY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(axisy)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAxisZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAxisZ)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAxisZ2(&self, axisz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAxisZ2)(::core::mem::transmute_copy(self), ::core::mem::transmute(axisz)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterZ)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterZ2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerz)).ok()
    }
}
impl ::core::convert::From<IDCompositionRotateTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionRotateTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionRotateTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionRotateTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionRotateTransform3D {}
impl ::core::fmt::Debug for IDCompositionRotateTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionRotateTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionRotateTransform3D {
    type Vtable = IDCompositionRotateTransform3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f5b23f_d429_4a91_b55a_d2f45fd75b18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform3D_Vtbl {
    pub base: IDCompositionTransform3D_Vtbl,
    pub SetAngle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAngle2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angle: f32) -> ::windows::core::HRESULT,
    pub SetAxisX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAxisX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisx: f32) -> ::windows::core::HRESULT,
    pub SetAxisY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAxisY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisy: f32) -> ::windows::core::HRESULT,
    pub SetAxisZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAxisZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, axisz: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionSaturationEffect(::windows::core::IUnknown);
impl IDCompositionSaturationEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetSaturation<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSaturation)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetSaturation2(&self, ratio: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSaturation2)(::core::mem::transmute_copy(self), ::core::mem::transmute(ratio)).ok()
    }
}
impl ::core::convert::From<IDCompositionSaturationEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSaturationEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSaturationEffect> for IDCompositionEffect {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSaturationEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSaturationEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSaturationEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionSaturationEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionSaturationEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSaturationEffect {}
impl ::core::fmt::Debug for IDCompositionSaturationEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSaturationEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSaturationEffect {
    type Vtable = IDCompositionSaturationEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa08debda_3258_4fa4_9f16_9174d3fe93b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSaturationEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    pub SetSaturation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetSaturation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ratio: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionScaleTransform(::windows::core::IUnknown);
impl IDCompositionScaleTransform {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(scalex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(scaley)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
}
impl ::core::convert::From<IDCompositionScaleTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform> for IDCompositionEffect {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform> for IDCompositionTransform {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionScaleTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionScaleTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionScaleTransform {}
impl ::core::fmt::Debug for IDCompositionScaleTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionScaleTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionScaleTransform {
    type Vtable = IDCompositionScaleTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71fde914_40ef_45ef_bd51_68b037c339f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform_Vtbl {
    pub base: IDCompositionTransform_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionScaleTransform3D(::windows::core::IUnknown);
impl IDCompositionScaleTransform3D {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(scalex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(scaley)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleZ)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetScaleZ2(&self, scalez: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetScaleZ2)(::core::mem::transmute_copy(self), ::core::mem::transmute(scalez)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterZ)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterZ2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerz)).ok()
    }
}
impl ::core::convert::From<IDCompositionScaleTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionScaleTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionScaleTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionScaleTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionScaleTransform3D {}
impl ::core::fmt::Debug for IDCompositionScaleTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionScaleTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionScaleTransform3D {
    type Vtable = IDCompositionScaleTransform3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a9e9ead_364b_4b15_a7c4_a1997f78b389);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform3D_Vtbl {
    pub base: IDCompositionTransform3D_Vtbl,
    pub SetScaleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetScaleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalex: f32) -> ::windows::core::HRESULT,
    pub SetScaleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetScaleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scaley: f32) -> ::windows::core::HRESULT,
    pub SetScaleZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetScaleZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalez: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
    pub SetCenterZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerz: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionShadowEffect(::windows::core::IUnknown);
impl IDCompositionShadowEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetStandardDeviation<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStandardDeviation)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStandardDeviation2)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetColor(&self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColor)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRed<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRed)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRed2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRed2)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetGreen<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreen)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetGreen2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreen2)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlue<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlue)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlue2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlue2)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAlpha<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlpha)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAlpha2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlpha2)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
}
impl ::core::convert::From<IDCompositionShadowEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionShadowEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionShadowEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionShadowEffect> for IDCompositionEffect {
    fn from(value: IDCompositionShadowEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionShadowEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionShadowEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionShadowEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionShadowEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionShadowEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionShadowEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionShadowEffect {}
impl ::core::fmt::Debug for IDCompositionShadowEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionShadowEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionShadowEffect {
    type Vtable = IDCompositionShadowEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ad18ac0_cfd2_4c2f_bb62_96e54fdb6879);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionShadowEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    pub SetStandardDeviation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetStandardDeviation2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetColor: usize,
    pub SetRed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRed2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    pub SetGreen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetGreen2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    pub SetBlue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBlue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
    pub SetAlpha: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAlpha2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, amount: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionSkewTransform(::windows::core::IUnknown);
impl IDCompositionSkewTransform {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngleX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngleX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngleX2(&self, anglex: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngleX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(anglex)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngleY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngleY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAngleY2(&self, angley: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAngleY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(angley)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCenterY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
}
impl ::core::convert::From<IDCompositionSkewTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSkewTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSkewTransform> for IDCompositionEffect {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSkewTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSkewTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSkewTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionSkewTransform> for IDCompositionTransform {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSkewTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionSkewTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionSkewTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSkewTransform {}
impl ::core::fmt::Debug for IDCompositionSkewTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSkewTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSkewTransform {
    type Vtable = IDCompositionSkewTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe57aa735_dcdb_4c72_9c61_0591f58889ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSkewTransform_Vtbl {
    pub base: IDCompositionTransform_Vtbl,
    pub SetAngleX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAngleX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, anglex: f32) -> ::windows::core::HRESULT,
    pub SetAngleY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAngleY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, angley: f32) -> ::windows::core::HRESULT,
    pub SetCenterX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centerx: f32) -> ::windows::core::HRESULT,
    pub SetCenterY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetCenterY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, centery: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionSurface(::windows::core::IUnknown);
impl IDCompositionSurface {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginDraw)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EndDraw)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SuspendDraw)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResumeDraw)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Scroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
}
impl ::core::convert::From<IDCompositionSurface> for ::windows::core::IUnknown {
    fn from(value: IDCompositionSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSurface> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSurface {}
impl ::core::fmt::Debug for IDCompositionSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSurface {
    type Vtable = IDCompositionSurface_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb8a4953_2c99_4f5a_96f5_4819027fa3ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurface_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub BeginDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    BeginDraw: usize,
    pub EndDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SuspendDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ResumeDraw: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Scroll: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionSurfaceFactory(::windows::core::IUnknown);
impl IDCompositionSurfaceFactory {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionSurface>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateVirtualSurface)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), ::core::mem::transmute(&mut result__)).from_abi::<IDCompositionVirtualSurface>(result__)
    }
}
impl ::core::convert::From<IDCompositionSurfaceFactory> for ::windows::core::IUnknown {
    fn from(value: IDCompositionSurfaceFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionSurfaceFactory> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionSurfaceFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionSurfaceFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionSurfaceFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionSurfaceFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionSurfaceFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionSurfaceFactory {}
impl ::core::fmt::Debug for IDCompositionSurfaceFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionSurfaceFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSurfaceFactory {
    type Vtable = IDCompositionSurfaceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe334bc12_3937_4e02_85eb_fcf4eb30d2c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurfaceFactory_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateSurface: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub CreateVirtualSurface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    CreateVirtualSurface: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTableTransferEffect(::windows::core::IUnknown);
impl IDCompositionTableTransferEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRedTable(&self, tablevalues: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedTable)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(tablevalues)), tablevalues.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetGreenTable(&self, tablevalues: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenTable)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(tablevalues)), tablevalues.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlueTable(&self, tablevalues: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueTable)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(tablevalues)), tablevalues.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAlphaTable(&self, tablevalues: &[f32]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaTable)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(tablevalues)), tablevalues.len() as _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRedDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, reddisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedDisable)(::core::mem::transmute_copy(self), reddisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGreenDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, greendisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenDisable)(::core::mem::transmute_copy(self), greendisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlueDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bluedisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueDisable)(::core::mem::transmute_copy(self), bluedisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlphaDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, alphadisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaDisable)(::core::mem::transmute_copy(self), alphadisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clampoutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClampOutput)(::core::mem::transmute_copy(self), clampoutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRedTableValue<'a, Param1: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedTableValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRedTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRedTableValue2)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetGreenTableValue<'a, Param1: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenTableValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetGreenTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetGreenTableValue2)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlueTableValue<'a, Param1: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueTableValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBlueTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBlueTableValue2)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAlphaTableValue<'a, Param1: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaTableValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetAlphaTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAlphaTableValue2)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
}
impl ::core::convert::From<IDCompositionTableTransferEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTableTransferEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTableTransferEffect> for IDCompositionEffect {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTableTransferEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTableTransferEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTableTransferEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTableTransferEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTableTransferEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTableTransferEffect {}
impl ::core::fmt::Debug for IDCompositionTableTransferEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTableTransferEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTableTransferEffect {
    type Vtable = IDCompositionTableTransferEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e82e2_69c5_4eb4_a5f5_a7033f5132cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTableTransferEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    pub SetRedTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    pub SetGreenTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    pub SetBlueTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    pub SetAlphaTable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetRedDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetRedDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetGreenDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetGreenDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetBlueDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetBlueDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAlphaDisable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAlphaDisable: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetClampOutput: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetClampOutput: usize,
    pub SetRedTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRedTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT,
    pub SetGreenTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetGreenTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT,
    pub SetBlueTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBlueTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT,
    pub SetAlphaTableValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetAlphaTableValue2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: u32, value: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTarget(::windows::core::IUnknown);
impl IDCompositionTarget {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetRoot<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRoot)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionTarget> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTarget) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTarget> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTarget) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTarget {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTarget {}
impl ::core::fmt::Debug for IDCompositionTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTarget {
    type Vtable = IDCompositionTarget_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeacdd04c_117e_4e17_88f4_d1b12b0e3d89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTarget_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTransform(::windows::core::IUnknown);
impl IDCompositionTransform {}
impl ::core::convert::From<IDCompositionTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTransform> for IDCompositionEffect {
    fn from(value: IDCompositionTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTransform {}
impl ::core::fmt::Debug for IDCompositionTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTransform {
    type Vtable = IDCompositionTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd55faa7_37e0_4c20_95d2_9be45bc33f55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform_Vtbl {
    pub base: IDCompositionTransform3D_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTransform3D(::windows::core::IUnknown);
impl IDCompositionTransform3D {}
impl ::core::convert::From<IDCompositionTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTransform3D {}
impl ::core::fmt::Debug for IDCompositionTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTransform3D {
    type Vtable = IDCompositionTransform3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71185722_246b_41f2_aad1_0443f7f4bfc2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform3D_Vtbl {
    pub base: IDCompositionEffect_Vtbl,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTranslateTransform(::windows::core::IUnknown);
impl IDCompositionTranslateTransform {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform> for IDCompositionEffect {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform> for IDCompositionTransform {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &'a IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTranslateTransform {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTranslateTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTranslateTransform {}
impl ::core::fmt::Debug for IDCompositionTranslateTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTranslateTransform").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTranslateTransform {
    type Vtable = IDCompositionTranslateTransform_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06791122_c6f0_417d_8323_269e987f5954);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform_Vtbl {
    pub base: IDCompositionTransform_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTranslateTransform3D(::windows::core::IUnknown);
impl IDCompositionTranslateTransform3D {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetZ)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetZ2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetz)).ok()
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTranslateTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &'a IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTranslateTransform3D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTranslateTransform3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTranslateTransform3D {}
impl ::core::fmt::Debug for IDCompositionTranslateTransform3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTranslateTransform3D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTranslateTransform3D {
    type Vtable = IDCompositionTranslateTransform3D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91636d4b_9ba1_4532_aaf7_e3344994d788);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform3D_Vtbl {
    pub base: IDCompositionTransform3D_Vtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionTurbulenceEffect(::windows::core::IUnknown);
impl IDCompositionTurbulenceEffect {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetInput)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetOffset(&self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffset)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetBaseFrequency(&self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBaseFrequency)(::core::mem::transmute_copy(self), ::core::mem::transmute(frequency)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetSize(&self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSize)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetNumOctaves(&self, numoctaves: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNumOctaves)(::core::mem::transmute_copy(self), ::core::mem::transmute(numoctaves)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetSeed(&self, seed: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSeed)(::core::mem::transmute_copy(self), ::core::mem::transmute(seed)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetNoise)(::core::mem::transmute_copy(self), ::core::mem::transmute(noise)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStitchable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stitchable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetStitchable)(::core::mem::transmute_copy(self), stitchable.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionTurbulenceEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTurbulenceEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTurbulenceEffect> for IDCompositionEffect {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTurbulenceEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &'a IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionTurbulenceEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionTurbulenceEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &'a IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionTurbulenceEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionTurbulenceEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionTurbulenceEffect {}
impl ::core::fmt::Debug for IDCompositionTurbulenceEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionTurbulenceEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTurbulenceEffect {
    type Vtable = IDCompositionTurbulenceEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6a55bda_c09c_49f3_9193_a41922c89715);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTurbulenceEffect_Vtbl {
    pub base: IDCompositionFilterEffect_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetOffset: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetBaseFrequency: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetBaseFrequency: usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetSize: usize,
    pub SetNumOctaves: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, numoctaves: u32) -> ::windows::core::HRESULT,
    pub SetSeed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, seed: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetNoise: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetNoise: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetStitchable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stitchable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetStitchable: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVirtualSurface(::windows::core::IUnknown);
impl IDCompositionVirtualSurface {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.BeginDraw)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EndDraw)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SuspendDraw)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ResumeDraw)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Scroll)(::core::mem::transmute_copy(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn Resize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Resize)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Trim(&self, rectangles: &[super::super::Foundation::RECT]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Trim)(::core::mem::transmute_copy(self), ::core::mem::transmute(::windows::core::as_ptr_or_null(rectangles)), rectangles.len() as _).ok()
    }
}
impl ::core::convert::From<IDCompositionVirtualSurface> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVirtualSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVirtualSurface> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVirtualSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVirtualSurface> for IDCompositionSurface {
    fn from(value: IDCompositionVirtualSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVirtualSurface> for IDCompositionSurface {
    fn from(value: &IDCompositionVirtualSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionSurface> for IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionSurface> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionSurface> for &'a IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionSurface> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVirtualSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVirtualSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVirtualSurface {}
impl ::core::fmt::Debug for IDCompositionVirtualSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVirtualSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVirtualSurface {
    type Vtable = IDCompositionVirtualSurface_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae471c51_5f53_4a24_8d3e_d0c39c30b3f0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVirtualSurface_Vtbl {
    pub base: IDCompositionSurface_Vtbl,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Trim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Trim: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVisual(::windows::core::IUnknown);
impl IDCompositionVisual {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransform)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransform2)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransformParent<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransformParent)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetEffect<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEffect)(::core::mem::transmute_copy(self), effect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBitmapInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBorderMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClip)(::core::mem::transmute_copy(self), clip.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetClip2)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddVisual)(::core::mem::transmute_copy(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn RemoveVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveVisual)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAllVisuals)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCompositeMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(compositemode)).ok()
    }
}
impl ::core::convert::From<IDCompositionVisual> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVisual) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVisual) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVisual {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisual {}
impl ::core::fmt::Debug for IDCompositionVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisual").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisual {
    type Vtable = IDCompositionVisual_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d93059d_097b_4651_9a60_f0f25116e2f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetOffsetX: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOffsetX2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetx: f32) -> ::windows::core::HRESULT,
    pub SetOffsetY: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOffsetY2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsety: f32) -> ::windows::core::HRESULT,
    pub SetTransform: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetTransform2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetTransform2: usize,
    pub SetTransformParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetBitmapInterpolationMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    pub SetBorderMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT,
    pub SetClip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetClip2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetClip2: usize,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub AddVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddVisual: usize,
    pub RemoveVisual: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub RemoveAllVisuals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetCompositeMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVisual2(::windows::core::IUnknown);
impl IDCompositionVisual2 {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOffsetX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOffsetX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOffsetY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOffsetY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransform)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransform2)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransformParent<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTransformParent)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetEffect<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEffect)(::core::mem::transmute_copy(self), effect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBitmapInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBorderMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClip)(::core::mem::transmute_copy(self), clip.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetClip2)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.AddVisual)(::core::mem::transmute_copy(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn RemoveVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveVisual)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.RemoveAllVisuals)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCompositeMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(compositemode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacityMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBackFaceVisibility)(::core::mem::transmute_copy(self), ::core::mem::transmute(visibility)).ok()
    }
}
impl ::core::convert::From<IDCompositionVisual2> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVisual2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual2> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVisual2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVisual2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVisual2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisual2> for IDCompositionVisual {
    fn from(value: IDCompositionVisual2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual2> for IDCompositionVisual {
    fn from(value: &IDCompositionVisual2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual> for IDCompositionVisual2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual> for &'a IDCompositionVisual2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVisual2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVisual2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisual2 {}
impl ::core::fmt::Debug for IDCompositionVisual2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisual2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisual2 {
    type Vtable = IDCompositionVisual2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8de1639_4331_4b26_bc5f_6a321d347a85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual2_Vtbl {
    pub base: IDCompositionVisual_Vtbl,
    pub SetOpacityMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT,
    pub SetBackFaceVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVisual3(::windows::core::IUnknown);
impl IDCompositionVisual3 {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetOffsetX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetOffsetX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetOffsetY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetOffsetY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetTransform)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetTransform2)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransformParent<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetTransformParent)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetEffect<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetEffect)(::core::mem::transmute_copy(self), effect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetBitmapInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetBorderMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetClip)(::core::mem::transmute_copy(self), clip.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetClip2)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.AddVisual)(::core::mem::transmute_copy(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn RemoveVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.RemoveVisual)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.RemoveAllVisuals)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.base.SetCompositeMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(compositemode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOpacityMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetBackFaceVisibility)(::core::mem::transmute_copy(self), ::core::mem::transmute(visibility)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EnableHeatMap)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn DisableHeatMap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DisableHeatMap)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.EnableRedrawRegions)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.DisableRedrawRegions)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetDepthMode(&self, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDepthMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetZ)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOffsetZ2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetz)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOpacity<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacity)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOpacity2)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransform3<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform3D>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransform3)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetTransform4(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTransform4)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVisible<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, visible: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetVisible)(::core::mem::transmute_copy(self), visible.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IDCompositionVisual3> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual3> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisual3> for IDCompositionVisual {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual3> for IDCompositionVisual {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual> for &'a IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisual3> for IDCompositionVisual2 {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual3> for IDCompositionVisual2 {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual2> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual2> for &'a IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisual3> for IDCompositionVisualDebug {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisual3> for IDCompositionVisualDebug {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisualDebug> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisualDebug> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisualDebug> for &'a IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisualDebug> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVisual3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVisual3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisual3 {}
impl ::core::fmt::Debug for IDCompositionVisual3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisual3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisual3 {
    type Vtable = IDCompositionVisual3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2775f462_b6c1_4015_b0be_b3e7d6a4976d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual3_Vtbl {
    pub base: IDCompositionVisualDebug_Vtbl,
    pub SetDepthMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::HRESULT,
    pub SetOffsetZ: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOffsetZ2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offsetz: f32) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetOpacity2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, opacity: f32) -> ::windows::core::HRESULT,
    pub SetTransform3: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub SetTransform4: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    SetTransform4: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVisible: usize,
}
#[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
#[repr(transparent)]
pub struct IDCompositionVisualDebug(::windows::core::IUnknown);
impl IDCompositionVisualDebug {
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOffsetX)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOffsetX2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOffsetY)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetOffsetY2)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetTransform)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetTransform2)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetTransformParent<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetTransformParent)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetEffect<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetEffect)(::core::mem::transmute_copy(self), effect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetBitmapInterpolationMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetBorderMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetClip)(::core::mem::transmute_copy(self), clip.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetClip2)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetContent)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.AddVisual)(::core::mem::transmute_copy(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn RemoveVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RemoveVisual)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.RemoveAllVisuals)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.base.SetCompositeMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(compositemode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOpacityMode)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetBackFaceVisibility)(::core::mem::transmute_copy(self), ::core::mem::transmute(visibility)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`, `\"Win32_Graphics_Direct2D_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableHeatMap)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn DisableHeatMap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableHeatMap)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableRedrawRegions)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_DirectComposition\"`*"]
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisableRedrawRegions)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDCompositionVisualDebug> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVisualDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisualDebug> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisualDebug> for IDCompositionVisual {
    fn from(value: IDCompositionVisualDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisualDebug> for IDCompositionVisual {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual> for IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual> for &'a IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDCompositionVisualDebug> for IDCompositionVisual2 {
    fn from(value: IDCompositionVisualDebug) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDCompositionVisualDebug> for IDCompositionVisual2 {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual2> for IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual2> for &'a IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDCompositionVisualDebug {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDCompositionVisualDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDCompositionVisualDebug {}
impl ::core::fmt::Debug for IDCompositionVisualDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDCompositionVisualDebug").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisualDebug {
    type Vtable = IDCompositionVisualDebug_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfed2b808_5eb4_43a0_aea3_35f65280f91b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisualDebug_Vtbl {
    pub base: IDCompositionVisual2_Vtbl,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    pub EnableHeatMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))]
    EnableHeatMap: usize,
    pub DisableHeatMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub EnableRedrawRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub DisableRedrawRegions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
