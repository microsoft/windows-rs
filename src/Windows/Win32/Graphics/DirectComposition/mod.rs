#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct COMPOSITION_FRAME_ID_TYPE(pub i32);
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(0i32);
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(1i32);
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(2i32);
impl ::core::convert::From<i32> for COMPOSITION_FRAME_ID_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for COMPOSITION_FRAME_ID_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
impl COMPOSITION_FRAME_STATS {}
impl ::core::default::Default for COMPOSITION_FRAME_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COMPOSITION_FRAME_STATS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COMPOSITION_FRAME_STATS").field("startTime", &self.startTime).field("targetTime", &self.targetTime).field("framePeriod", &self.framePeriod).finish()
    }
}
impl ::core::cmp::PartialEq for COMPOSITION_FRAME_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.startTime == other.startTime && self.targetTime == other.targetTime && self.framePeriod == other.framePeriod
    }
}
impl ::core::cmp::Eq for COMPOSITION_FRAME_STATS {}
unsafe impl ::windows::core::Abi for COMPOSITION_FRAME_STATS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
impl COMPOSITION_STATS {}
impl ::core::default::Default for COMPOSITION_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COMPOSITION_STATS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COMPOSITION_STATS").field("presentCount", &self.presentCount).field("refreshCount", &self.refreshCount).field("virtualRefreshCount", &self.virtualRefreshCount).field("time", &self.time).finish()
    }
}
impl ::core::cmp::PartialEq for COMPOSITION_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.presentCount == other.presentCount && self.refreshCount == other.refreshCount && self.virtualRefreshCount == other.virtualRefreshCount && self.time == other.time
    }
}
impl ::core::cmp::Eq for COMPOSITION_STATS {}
unsafe impl ::windows::core::Abi for COMPOSITION_STATS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: super::super::Foundation::LUID,
    pub renderAdapterLuid: super::super::Foundation::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMPOSITION_TARGET_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMPOSITION_TARGET_ID {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COMPOSITION_TARGET_ID").field("displayAdapterLuid", &self.displayAdapterLuid).field("renderAdapterLuid", &self.renderAdapterLuid).field("vidPnSourceId", &self.vidPnSourceId).field("vidPnTargetId", &self.vidPnTargetId).field("uniqueId", &self.uniqueId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_ID {
    fn eq(&self, other: &Self) -> bool {
        self.displayAdapterLuid == other.displayAdapterLuid && self.renderAdapterLuid == other.renderAdapterLuid && self.vidPnSourceId == other.vidPnSourceId && self.vidPnTargetId == other.vidPnTargetId && self.uniqueId == other.uniqueId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMPOSITION_TARGET_ID {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
impl COMPOSITION_TARGET_STATS {}
impl ::core::default::Default for COMPOSITION_TARGET_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for COMPOSITION_TARGET_STATS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("COMPOSITION_TARGET_STATS").field("outstandingPresents", &self.outstandingPresents).field("presentTime", &self.presentTime).field("vblankDuration", &self.vblankDuration).field("presentedStats", &self.presentedStats).field("completedStats", &self.completedStats).finish()
    }
}
impl ::core::cmp::PartialEq for COMPOSITION_TARGET_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.outstandingPresents == other.outstandingPresents && self.presentTime == other.presentTime && self.vblankDuration == other.vblankDuration && self.presentedStats == other.presentedStats && self.completedStats == other.completedStats
    }
}
impl ::core::cmp::Eq for COMPOSITION_TARGET_STATS {}
unsafe impl ::windows::core::Abi for COMPOSITION_TARGET_STATS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(pub i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(0i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(-1i32);
impl ::core::convert::From<i32> for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_BACKFACE_VISIBILITY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(pub i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(0i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(-1i32);
impl ::core::convert::From<i32> for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DCOMPOSITION_BORDER_MODE(pub i32);
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(0i32);
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(1i32);
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(-1i32);
impl ::core::convert::From<i32> for DCOMPOSITION_BORDER_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_BORDER_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DCOMPOSITION_COMPOSITE_MODE(pub i32);
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(0i32);
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(2i32);
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(-1i32);
impl ::core::convert::From<i32> for DCOMPOSITION_COMPOSITE_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_COMPOSITE_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DCOMPOSITION_DEPTH_MODE(pub i32);
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(0i32);
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(1i32);
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(3i32);
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(-1i32);
impl ::core::convert::From<i32> for DCOMPOSITION_DEPTH_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_DEPTH_MODE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DCOMPOSITION_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DCOMPOSITION_FRAME_STATISTICS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DCOMPOSITION_FRAME_STATISTICS")
            .field("lastFrameTime", &self.lastFrameTime)
            .field("currentCompositionRate", &self.currentCompositionRate)
            .field("currentTime", &self.currentTime)
            .field("timeFrequency", &self.timeFrequency)
            .field("nextEstimatedFrameTime", &self.nextEstimatedFrameTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DCOMPOSITION_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.lastFrameTime == other.lastFrameTime && self.currentCompositionRate == other.currentCompositionRate && self.currentTime == other.currentTime && self.timeFrequency == other.timeFrequency && self.nextEstimatedFrameTime == other.nextEstimatedFrameTime
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for DCOMPOSITION_FRAME_STATISTICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DCOMPOSITION_OPACITY_MODE(pub i32);
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(0i32);
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(1i32);
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(-1i32);
impl ::core::convert::From<i32> for DCOMPOSITION_OPACITY_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DCOMPOSITION_OPACITY_MODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi`*"]
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
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[inline]
pub unsafe fn DCompositionCreateDevice2<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(renderingdevice: Param0, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice2(renderingdevice: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DCompositionCreateDevice2(renderingdevice.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(dcompositiondevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[inline]
pub unsafe fn DCompositionCreateDevice3<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(renderingdevice: Param0, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice3(renderingdevice: ::windows::core::RawPtr, iid: *const ::windows::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        DCompositionCreateDevice3(renderingdevice.into_param().abi(), ::core::mem::transmute(iid), ::core::mem::transmute(dcompositiondevice)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, surfacehandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        DCompositionCreateSurfaceHandle(::core::mem::transmute(desiredaccess), ::core::mem::transmute(securityattributes), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[inline]
pub unsafe fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE) -> ::windows::core::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE, frameid: *mut u64) -> ::windows::core::HRESULT;
        }
        let mut result__: <u64 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        DCompositionGetFrameId(::core::mem::transmute(frameidtype), &mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
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
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID) -> ::windows::core::Result<COMPOSITION_TARGET_STATS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID, targetstats: *mut COMPOSITION_TARGET_STATS) -> ::windows::core::HRESULT;
        }
        let mut result__: <COMPOSITION_TARGET_STATS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        DCompositionGetTargetStatistics(::core::mem::transmute(frameid), ::core::mem::transmute(targetid), &mut result__).from_abi::<COMPOSITION_TARGET_STATS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl DCompositionInkTrailPoint {}
impl ::core::default::Default for DCompositionInkTrailPoint {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DCompositionInkTrailPoint {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DCompositionInkTrailPoint").field("x", &self.x).field("y", &self.y).field("radius", &self.radius).finish()
    }
}
impl ::core::cmp::PartialEq for DCompositionInkTrailPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.radius == other.radius
    }
}
impl ::core::cmp::Eq for DCompositionInkTrailPoint {}
unsafe impl ::windows::core::Abi for DCompositionInkTrailPoint {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionWaitForCompositorClock(count: u32, handles: *const super::super::Foundation::HANDLE, timeoutinms: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionWaitForCompositorClock(count: u32, handles: *const super::super::Foundation::HANDLE, timeoutinms: u32) -> u32;
        }
        ::core::mem::transmute(DCompositionWaitForCompositorClock(::core::mem::transmute(count), ::core::mem::transmute(handles), ::core::mem::transmute(timeoutinms)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionAffineTransform2DEffect(pub ::windows::core::IUnknown);
impl IDCompositionAffineTransform2DEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInterpolationMode(&self, interpolationmode: super::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetBorderMode(&self, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Foundation_Numerics`*"]
    pub unsafe fn SetTransformMatrix(&self, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(transformmatrix)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransformMatrixElement<'a, Param2: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransformMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetSharpness<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetSharpness2(&self, sharpness: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(sharpness)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionAffineTransform2DEffect {
    type Vtable = IDCompositionAffineTransform2DEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0b74b9e8_cdd6_492f_bbbc_5ed32157026d);
}
impl ::core::convert::From<IDCompositionAffineTransform2DEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionAffineTransform2DEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionAffineTransform2DEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAffineTransform2DEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interpolationmode: super::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bordermode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, sharpness: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionAnimation(pub ::windows::core::IUnknown);
impl IDCompositionAnimation {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAbsoluteBeginTime(&self, begintime: i64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(begintime)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn AddCubic(&self, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(beginoffset), ::core::mem::transmute(constantcoefficient), ::core::mem::transmute(linearcoefficient), ::core::mem::transmute(quadraticcoefficient), ::core::mem::transmute(cubiccoefficient)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn AddSinusoidal(&self, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(beginoffset), ::core::mem::transmute(bias), ::core::mem::transmute(amplitude), ::core::mem::transmute(frequency), ::core::mem::transmute(phase)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn AddRepeat(&self, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(beginoffset), ::core::mem::transmute(durationtorepeat)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn End(&self, endoffset: f64, endvalue: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(endoffset), ::core::mem::transmute(endvalue)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionAnimation {
    type Vtable = IDCompositionAnimation_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbfd91d9_51b2_45e4_b3de_d19ccfb863c5);
}
impl ::core::convert::From<IDCompositionAnimation> for ::windows::core::IUnknown {
    fn from(value: IDCompositionAnimation) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionAnimation> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionAnimation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionAnimation {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAnimation_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, begintime: i64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, beginoffset: f64, constantcoefficient: f32, linearcoefficient: f32, quadraticcoefficient: f32, cubiccoefficient: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, beginoffset: f64, bias: f32, amplitude: f32, frequency: f32, phase: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, beginoffset: f64, durationtorepeat: f64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, endoffset: f64, endvalue: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionArithmeticCompositeEffect(pub ::windows::core::IUnknown);
impl IDCompositionArithmeticCompositeEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetCoefficients(&self, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(coefficients)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetClampOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clampoutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), clampoutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCoefficient1<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCoefficient12(&self, coeffcient1: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(coeffcient1)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCoefficient2<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCoefficient22(&self, coefficient2: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(coefficient2)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCoefficient3<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCoefficient32(&self, coefficient3: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(coefficient3)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCoefficient4<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCoefficient42(&self, coefficient4: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(coefficient4)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionArithmeticCompositeEffect {
    type Vtable = IDCompositionArithmeticCompositeEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b67dfa8_e3dd_4e61_b640_46c2f3d739dc);
}
impl ::core::convert::From<IDCompositionArithmeticCompositeEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionArithmeticCompositeEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionArithmeticCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionArithmeticCompositeEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coefficients: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coeffcient1: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coefficient2: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coefficient3: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, coefficient4: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionBlendEffect(pub ::windows::core::IUnknown);
impl IDCompositionBlendEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionBlendEffect {
    type Vtable = IDCompositionBlendEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33ecdc0a_578a_4a11_9c14_0cb90517f9c5);
}
impl ::core::convert::From<IDCompositionBlendEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionBlendEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionBlendEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBlendEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: super::Direct2D::Common::D2D1_BLEND_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionBrightnessEffect(pub ::windows::core::IUnknown);
impl IDCompositionBrightnessEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetWhitePoint(&self, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(whitepoint)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetBlackPoint(&self, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(blackpoint)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetWhitePointX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetWhitePointX2(&self, whitepointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(whitepointx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetWhitePointY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetWhitePointY2(&self, whitepointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(whitepointy)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlackPointX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlackPointX2(&self, blackpointx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(blackpointx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlackPointY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlackPointY2(&self, blackpointy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(blackpointy)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionBrightnessEffect {
    type Vtable = IDCompositionBrightnessEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6027496e_cb3a_49ab_934f_d798da4f7da6);
}
impl ::core::convert::From<IDCompositionBrightnessEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionBrightnessEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBrightnessEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, whitepoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blackpoint: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, whitepointx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, whitepointy: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blackpointx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blackpointy: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionClip(pub ::windows::core::IUnknown);
impl IDCompositionClip {}
unsafe impl ::windows::core::Interface for IDCompositionClip {
    type Vtable = IDCompositionClip_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x64ac3703_9d3f_45ec_a109_7cac0e7a13a7);
}
impl ::core::convert::From<IDCompositionClip> for ::windows::core::IUnknown {
    fn from(value: IDCompositionClip) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionClip> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionClip) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionClip_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionColorMatrixEffect(pub ::windows::core::IUnknown);
impl IDCompositionColorMatrixEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetMatrix(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetMatrixElement<'a, Param2: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetAlphaMode(&self, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetClampOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clamp: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), clamp.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionColorMatrixEffect {
    type Vtable = IDCompositionColorMatrixEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1170a22_3ce2_4966_90d4_55408bfc84c4);
}
impl ::core::convert::From<IDCompositionColorMatrixEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionColorMatrixEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionColorMatrixEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const super::Direct2D::Common::D2D_MATRIX_5X4_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: super::Direct2D::Common::D2D1_COLORMATRIX_ALPHA_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clamp: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionCompositeEffect(pub ::windows::core::IUnknown);
impl IDCompositionCompositeEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetMode(&self, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionCompositeEffect {
    type Vtable = IDCompositionCompositeEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x576616c0_a231_494d_a38d_00fd5ec4db46);
}
impl ::core::convert::From<IDCompositionCompositeEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionCompositeEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionCompositeEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: super::Direct2D::Common::D2D1_COMPOSITE_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionDelegatedInkTrail(pub ::windows::core::IUnknown);
impl IDCompositionDelegatedInkTrail {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn AddTrailPoints(&self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(inkpoints), ::core::mem::transmute(inkpointscount), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn AddTrailPointsWithPrediction(&self, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32) -> ::windows::core::Result<u32> {
        let mut result__: <u32 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(inkpoints), ::core::mem::transmute(inkpointscount), ::core::mem::transmute(predictedinkpoints), ::core::mem::transmute(predictedinkpointscount), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn RemoveTrailPoints(&self, generationid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(generationid)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn StartNewTrail(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDelegatedInkTrail {
    type Vtable = IDCompositionDelegatedInkTrail_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc2448e9b_547d_4057_8cf5_8144ede1c2da);
}
impl ::core::convert::From<IDCompositionDelegatedInkTrail> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDelegatedInkTrail) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionDelegatedInkTrail> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDelegatedInkTrail) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDelegatedInkTrail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDelegatedInkTrail {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDelegatedInkTrail_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inkpoints: *const DCompositionInkTrailPoint, inkpointscount: u32, predictedinkpoints: *const DCompositionInkTrailPoint, predictedinkpointscount: u32, generationid: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, generationid: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionDesktopDevice(pub ::windows::core::IUnknown);
impl IDCompositionDesktopDevice {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: <DCOMPOSITION_FRAME_STATISTICS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2> {
        let mut result__: <IDCompositionVisual2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionVisual2>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateSurfaceFactory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows::core::Result<IDCompositionSurfaceFactory> {
        let mut result__: <IDCompositionSurfaceFactory as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), renderingdevice.into_param().abi(), &mut result__).from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__: <IDCompositionTranslateTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__: <IDCompositionScaleTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionScaleTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__: <IDCompositionRotateTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRotateTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__: <IDCompositionSkewTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionSkewTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__: <IDCompositionMatrixTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTransformGroup(&self, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__: <IDCompositionTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(transforms), ::core::mem::transmute(elements), &mut result__).from_abi::<IDCompositionTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__: <IDCompositionTranslateTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__: <IDCompositionScaleTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__: <IDCompositionRotateTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__: <IDCompositionMatrixTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__: <IDCompositionTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(transforms3d), ::core::mem::transmute(elements), &mut result__).from_abi::<IDCompositionTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__: <IDCompositionEffectGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionEffectGroup>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__: <IDCompositionRectangleClip as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRectangleClip>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__: <IDCompositionAnimation as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionAnimation>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn CreateTargetForHwnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hwnd: Param0, topmost: Param1) -> ::windows::core::Result<IDCompositionTarget> {
        let mut result__: <IDCompositionTarget as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), topmost.into_param().abi(), &mut result__).from_abi::<IDCompositionTarget>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn CreateSurfaceFromHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, handle: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), handle.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn CreateSurfaceFromHwnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDesktopDevice {
    type Vtable = IDCompositionDesktopDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f4633fe_1e08_4cb8_8c75_ce24333f5602);
}
impl ::core::convert::From<IDCompositionDesktopDevice> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDesktopDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionDesktopDevice> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDesktopDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionDevice2> for &IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionDevice2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDesktopDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, renderingdevice: ::windows::core::RawPtr, surfacefactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handle: super::super::Foundation::HANDLE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionDevice(pub ::windows::core::IUnknown);
impl IDCompositionDevice {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: <DCOMPOSITION_FRAME_STATISTICS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn CreateTargetForHwnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hwnd: Param0, topmost: Param1) -> ::windows::core::Result<IDCompositionTarget> {
        let mut result__: <IDCompositionTarget as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), topmost.into_param().abi(), &mut result__).from_abi::<IDCompositionTarget>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual> {
        let mut result__: <IDCompositionVisual as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionVisual>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn CreateSurfaceFromHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, handle: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), handle.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn CreateSurfaceFromHwnd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: <::windows::core::IUnknown as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), hwnd.into_param().abi(), &mut result__).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__: <IDCompositionTranslateTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__: <IDCompositionScaleTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionScaleTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__: <IDCompositionRotateTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRotateTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__: <IDCompositionSkewTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionSkewTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__: <IDCompositionMatrixTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTransformGroup(&self, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__: <IDCompositionTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(transforms), ::core::mem::transmute(elements), &mut result__).from_abi::<IDCompositionTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__: <IDCompositionTranslateTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__: <IDCompositionScaleTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__: <IDCompositionRotateTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__: <IDCompositionMatrixTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__: <IDCompositionTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(transforms3d), ::core::mem::transmute(elements), &mut result__).from_abi::<IDCompositionTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__: <IDCompositionEffectGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionEffectGroup>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__: <IDCompositionRectangleClip as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRectangleClip>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__: <IDCompositionAnimation as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionAnimation>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn CheckDeviceState(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDevice {
    type Vtable = IDCompositionDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc37ea93a_e7aa_450d_b16f_9746cb0407f3);
}
impl ::core::convert::From<IDCompositionDevice> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionDevice> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, topmost: super::super::Foundation::BOOL, target: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, handle: super::super::Foundation::HANDLE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, hwnd: super::super::Foundation::HWND, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pfvalid: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionDevice2(pub ::windows::core::IUnknown);
impl IDCompositionDevice2 {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: <DCOMPOSITION_FRAME_STATISTICS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2> {
        let mut result__: <IDCompositionVisual2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionVisual2>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateSurfaceFactory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows::core::Result<IDCompositionSurfaceFactory> {
        let mut result__: <IDCompositionSurfaceFactory as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), renderingdevice.into_param().abi(), &mut result__).from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__: <IDCompositionTranslateTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__: <IDCompositionScaleTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionScaleTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__: <IDCompositionRotateTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRotateTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__: <IDCompositionSkewTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionSkewTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__: <IDCompositionMatrixTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTransformGroup(&self, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__: <IDCompositionTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(transforms), ::core::mem::transmute(elements), &mut result__).from_abi::<IDCompositionTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__: <IDCompositionTranslateTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__: <IDCompositionScaleTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__: <IDCompositionRotateTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__: <IDCompositionMatrixTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__: <IDCompositionTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(transforms3d), ::core::mem::transmute(elements), &mut result__).from_abi::<IDCompositionTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__: <IDCompositionEffectGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionEffectGroup>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__: <IDCompositionRectangleClip as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRectangleClip>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__: <IDCompositionAnimation as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionAnimation>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDevice2 {
    type Vtable = IDCompositionDevice2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75f6468d_1b8e_447c_9bc6_75fea80b5b25);
}
impl ::core::convert::From<IDCompositionDevice2> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDevice2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionDevice2> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDevice2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDevice2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, renderingdevice: ::windows::core::RawPtr, surfacefactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionDevice3(pub ::windows::core::IUnknown);
impl IDCompositionDevice3 {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn GetFrameStatistics(&self) -> ::windows::core::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: <DCOMPOSITION_FRAME_STATISTICS as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateVisual(&self) -> ::windows::core::Result<IDCompositionVisual2> {
        let mut result__: <IDCompositionVisual2 as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionVisual2>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateSurfaceFactory<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, renderingdevice: Param0) -> ::windows::core::Result<IDCompositionSurfaceFactory> {
        let mut result__: <IDCompositionSurfaceFactory as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), renderingdevice.into_param().abi(), &mut result__).from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionVirtualSurface>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTranslateTransform(&self) -> ::windows::core::Result<IDCompositionTranslateTransform> {
        let mut result__: <IDCompositionTranslateTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTranslateTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateScaleTransform(&self) -> ::windows::core::Result<IDCompositionScaleTransform> {
        let mut result__: <IDCompositionScaleTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionScaleTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRotateTransform(&self) -> ::windows::core::Result<IDCompositionRotateTransform> {
        let mut result__: <IDCompositionRotateTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRotateTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateSkewTransform(&self) -> ::windows::core::Result<IDCompositionSkewTransform> {
        let mut result__: <IDCompositionSkewTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionSkewTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateMatrixTransform(&self) -> ::windows::core::Result<IDCompositionMatrixTransform> {
        let mut result__: <IDCompositionMatrixTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionMatrixTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTransformGroup(&self, transforms: *const ::core::option::Option<IDCompositionTransform>, elements: u32) -> ::windows::core::Result<IDCompositionTransform> {
        let mut result__: <IDCompositionTransform as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(transforms), ::core::mem::transmute(elements), &mut result__).from_abi::<IDCompositionTransform>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTranslateTransform3D(&self) -> ::windows::core::Result<IDCompositionTranslateTransform3D> {
        let mut result__: <IDCompositionTranslateTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateScaleTransform3D(&self) -> ::windows::core::Result<IDCompositionScaleTransform3D> {
        let mut result__: <IDCompositionScaleTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRotateTransform3D(&self) -> ::windows::core::Result<IDCompositionRotateTransform3D> {
        let mut result__: <IDCompositionRotateTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateMatrixTransform3D(&self) -> ::windows::core::Result<IDCompositionMatrixTransform3D> {
        let mut result__: <IDCompositionMatrixTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTransform3DGroup(&self, transforms3d: *const ::core::option::Option<IDCompositionTransform3D>, elements: u32) -> ::windows::core::Result<IDCompositionTransform3D> {
        let mut result__: <IDCompositionTransform3D as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(transforms3d), ::core::mem::transmute(elements), &mut result__).from_abi::<IDCompositionTransform3D>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::core::Result<IDCompositionEffectGroup> {
        let mut result__: <IDCompositionEffectGroup as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionEffectGroup>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateRectangleClip(&self) -> ::windows::core::Result<IDCompositionRectangleClip> {
        let mut result__: <IDCompositionRectangleClip as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionRectangleClip>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateAnimation(&self) -> ::windows::core::Result<IDCompositionAnimation> {
        let mut result__: <IDCompositionAnimation as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionAnimation>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateGaussianBlurEffect(&self) -> ::windows::core::Result<IDCompositionGaussianBlurEffect> {
        let mut result__: <IDCompositionGaussianBlurEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionGaussianBlurEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateBrightnessEffect(&self) -> ::windows::core::Result<IDCompositionBrightnessEffect> {
        let mut result__: <IDCompositionBrightnessEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionBrightnessEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateColorMatrixEffect(&self) -> ::windows::core::Result<IDCompositionColorMatrixEffect> {
        let mut result__: <IDCompositionColorMatrixEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionColorMatrixEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateShadowEffect(&self) -> ::windows::core::Result<IDCompositionShadowEffect> {
        let mut result__: <IDCompositionShadowEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionShadowEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateHueRotationEffect(&self) -> ::windows::core::Result<IDCompositionHueRotationEffect> {
        let mut result__: <IDCompositionHueRotationEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionHueRotationEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateSaturationEffect(&self) -> ::windows::core::Result<IDCompositionSaturationEffect> {
        let mut result__: <IDCompositionSaturationEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionSaturationEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTurbulenceEffect(&self) -> ::windows::core::Result<IDCompositionTurbulenceEffect> {
        let mut result__: <IDCompositionTurbulenceEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTurbulenceEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateLinearTransferEffect(&self) -> ::windows::core::Result<IDCompositionLinearTransferEffect> {
        let mut result__: <IDCompositionLinearTransferEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionLinearTransferEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateTableTransferEffect(&self) -> ::windows::core::Result<IDCompositionTableTransferEffect> {
        let mut result__: <IDCompositionTableTransferEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionTableTransferEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateCompositeEffect(&self) -> ::windows::core::Result<IDCompositionCompositeEffect> {
        let mut result__: <IDCompositionCompositeEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionCompositeEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateBlendEffect(&self) -> ::windows::core::Result<IDCompositionBlendEffect> {
        let mut result__: <IDCompositionBlendEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).34)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionBlendEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateArithmeticCompositeEffect(&self) -> ::windows::core::Result<IDCompositionArithmeticCompositeEffect> {
        let mut result__: <IDCompositionArithmeticCompositeEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).35)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionArithmeticCompositeEffect>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateAffineTransform2DEffect(&self) -> ::windows::core::Result<IDCompositionAffineTransform2DEffect> {
        let mut result__: <IDCompositionAffineTransform2DEffect as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).36)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionAffineTransform2DEffect>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDevice3 {
    type Vtable = IDCompositionDevice3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0987cb06_f916_48bf_8d35_ce7641781bd9);
}
impl ::core::convert::From<IDCompositionDevice3> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDevice3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionDevice3> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDevice3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionDevice2> for &IDCompositionDevice3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionDevice2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, statistics: *mut DCOMPOSITION_FRAME_STATISTICS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, renderingdevice: ::windows::core::RawPtr, surfacefactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, translatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaletransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rotatetransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, skewtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrixtransform: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transforms: *const ::windows::core::RawPtr, elements: u32, transformgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, translatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaletransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rotatetransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrixtransform3d: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transforms3d: *const ::windows::core::RawPtr, elements: u32, transform3dgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effectgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, gaussianblureffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, brightnesseffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, colormatrixeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, shadoweffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, huerotationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, saturationeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, turbulenceeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lineartransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tabletransfereffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blendeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, arithmeticcompositeeffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, affinetransform2deffect: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionDeviceDebug(pub ::windows::core::IUnknown);
impl IDCompositionDeviceDebug {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn EnableDebugCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn DisableDebugCounters(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionDeviceDebug {
    type Vtable = IDCompositionDeviceDebug_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1a3c64a_224f_4a81_9773_4f03a89d3c6c);
}
impl ::core::convert::From<IDCompositionDeviceDebug> for ::windows::core::IUnknown {
    fn from(value: IDCompositionDeviceDebug) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionDeviceDebug> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionDeviceDebug) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionDeviceDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionDeviceDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDeviceDebug_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionEffect(pub ::windows::core::IUnknown);
impl IDCompositionEffect {}
unsafe impl ::windows::core::Interface for IDCompositionEffect {
    type Vtable = IDCompositionEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec81b08f_bfcb_4e8d_b193_a915587999e8);
}
impl ::core::convert::From<IDCompositionEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffect_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionEffectGroup(pub ::windows::core::IUnknown);
impl IDCompositionEffectGroup {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOpacity<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransform3D<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform3D>>(&self, transform3d: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), transform3d.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionEffectGroup {
    type Vtable = IDCompositionEffectGroup_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa7929a74_e6b2_4bd6_8b95_4040119ca34d);
}
impl ::core::convert::From<IDCompositionEffectGroup> for ::windows::core::IUnknown {
    fn from(value: IDCompositionEffectGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionEffectGroup> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionEffectGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionEffectGroup {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffectGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transform3d: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionFilterEffect(pub ::windows::core::IUnknown);
impl IDCompositionFilterEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionFilterEffect {
    type Vtable = IDCompositionFilterEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30c421d5_8cb2_4e9f_b133_37be270d4ac2);
}
impl ::core::convert::From<IDCompositionFilterEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionFilterEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionFilterEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionFilterEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionFilterEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionFilterEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionFilterEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionFilterEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionGaussianBlurEffect(pub ::windows::core::IUnknown);
impl IDCompositionGaussianBlurEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetStandardDeviation<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetBorderMode(&self, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionGaussianBlurEffect {
    type Vtable = IDCompositionGaussianBlurEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x45d4d0b7_1bd4_454e_8894_2bfa68443033);
}
impl ::core::convert::From<IDCompositionGaussianBlurEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionGaussianBlurEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionGaussianBlurEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionGaussianBlurEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, amount: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: super::Direct2D::Common::D2D1_BORDER_MODE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionHueRotationEffect(pub ::windows::core::IUnknown);
impl IDCompositionHueRotationEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngle<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngle2(&self, amountdegrees: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(amountdegrees)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionHueRotationEffect {
    type Vtable = IDCompositionHueRotationEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6db9f920_0770_4781_b0c6_381912f9d167);
}
impl ::core::convert::From<IDCompositionHueRotationEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionHueRotationEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionHueRotationEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, amountdegrees: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionInkTrailDevice(pub ::windows::core::IUnknown);
impl IDCompositionInkTrailDevice {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateDelegatedInkTrail(&self) -> ::windows::core::Result<IDCompositionDelegatedInkTrail> {
        let mut result__: <IDCompositionDelegatedInkTrail as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IDCompositionDelegatedInkTrail>(result__)
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn CreateDelegatedInkTrailForSwapChain<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, swapchain: Param0) -> ::windows::core::Result<IDCompositionDelegatedInkTrail> {
        let mut result__: <IDCompositionDelegatedInkTrail as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), swapchain.into_param().abi(), &mut result__).from_abi::<IDCompositionDelegatedInkTrail>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDCompositionInkTrailDevice {
    type Vtable = IDCompositionInkTrailDevice_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf0c7cec_cdeb_4d4a_b91c_721bf22f4e6c);
}
impl ::core::convert::From<IDCompositionInkTrailDevice> for ::windows::core::IUnknown {
    fn from(value: IDCompositionInkTrailDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionInkTrailDevice> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionInkTrailDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionInkTrailDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionInkTrailDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionInkTrailDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, swapchain: ::windows::core::RawPtr, inktrail: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionLinearTransferEffect(pub ::windows::core::IUnknown);
impl IDCompositionLinearTransferEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRedYIntercept<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRedYIntercept2(&self, redyintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(redyintercept)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRedSlope<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRedSlope2(&self, redslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(redslope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetRedDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, reddisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), reddisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetGreenYIntercept<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetGreenYIntercept2(&self, greenyintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(greenyintercept)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetGreenSlope<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetGreenSlope2(&self, greenslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(greenslope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetGreenDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, greendisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), greendisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlueYIntercept<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlueYIntercept2(&self, blueyintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(blueyintercept)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlueSlope<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlueSlope2(&self, blueslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(blueslope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetBlueDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bluedisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), bluedisable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAlphaYIntercept<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAlphaYIntercept2(&self, alphayintercept: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(alphayintercept)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAlphaSlope<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAlphaSlope2(&self, alphaslope: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(alphaslope)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetAlphaDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, alphadisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), alphadisable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetClampOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clampoutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), clampoutput.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionLinearTransferEffect {
    type Vtable = IDCompositionLinearTransferEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4305ee5b_c4a0_4c88_9385_67124e017683);
}
impl ::core::convert::From<IDCompositionLinearTransferEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionLinearTransferEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionLinearTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionLinearTransferEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, redyintercept: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, redslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, greenyintercept: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, greenslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blueyintercept: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, blueslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alphayintercept: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alphaslope: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionMatrixTransform(pub ::windows::core::IUnknown);
impl IDCompositionMatrixTransform {
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Foundation_Numerics`*"]
    pub unsafe fn SetMatrix(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetMatrixElement<'a, Param2: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionMatrixTransform {
    type Vtable = IDCompositionMatrixTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x16cdff07_c503_419c_83f2_0965c7af1fa6);
}
impl ::core::convert::From<IDCompositionMatrixTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionMatrixTransform3D(pub ::windows::core::IUnknown);
impl IDCompositionMatrixTransform3D {
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct3D`*"]
    pub unsafe fn SetMatrix(&self, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetMatrixElement<'a, Param2: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, row: i32, column: i32, animation: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetMatrixElement2(&self, row: i32, column: i32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(row), ::core::mem::transmute(column), ::core::mem::transmute(value)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionMatrixTransform3D {
    type Vtable = IDCompositionMatrixTransform3D_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4b3363f0_643b_41b7_b6e0_ccf22d34467c);
}
impl ::core::convert::From<IDCompositionMatrixTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionMatrixTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform3D_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const super::Direct3D::D3DMATRIX) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, row: i32, column: i32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, row: i32, column: i32, value: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionRectangleClip(pub ::windows::core::IUnknown);
impl IDCompositionRectangleClip {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetLeft<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetLeft2(&self, left: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(left)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTop<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTop2(&self, top: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(top)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRight<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRight2(&self, right: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(right)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottom<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottom2(&self, bottom: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(bottom)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTopLeftRadiusX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTopLeftRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTopLeftRadiusY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTopLeftRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTopRightRadiusX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTopRightRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTopRightRadiusY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTopRightRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottomLeftRadiusX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottomLeftRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottomLeftRadiusY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottomLeftRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottomRightRadiusX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottomRightRadiusX2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottomRightRadiusY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBottomRightRadiusY2(&self, radius: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(radius)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionRectangleClip {
    type Vtable = IDCompositionRectangleClip_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9842ad7d_d9cf_4908_aed7_48b51da5e7c2);
}
impl ::core::convert::From<IDCompositionRectangleClip> for ::windows::core::IUnknown {
    fn from(value: IDCompositionRectangleClip) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionRectangleClip> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionRectangleClip) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionRectangleClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionRectangleClip {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionClip> for &IDCompositionRectangleClip {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionClip> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRectangleClip_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, left: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, top: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, right: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bottom: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, radius: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, radius: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, radius: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, radius: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, radius: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, radius: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, radius: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, radius: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionRotateTransform(pub ::windows::core::IUnknown);
impl IDCompositionRotateTransform {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngle<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(angle)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionRotateTransform {
    type Vtable = IDCompositionRotateTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x641ed83c_ae96_46c5_90dc_32774cc5c6d5);
}
impl ::core::convert::From<IDCompositionRotateTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionRotateTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, angle: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centerx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centery: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionRotateTransform3D(pub ::windows::core::IUnknown);
impl IDCompositionRotateTransform3D {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngle<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(angle)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAxisX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAxisX2(&self, axisx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(axisx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAxisY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAxisY2(&self, axisy: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(axisy)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAxisZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAxisZ2(&self, axisz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(axisz)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerz)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionRotateTransform3D {
    type Vtable = IDCompositionRotateTransform3D_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd8f5b23f_d429_4a91_b55a_d2f45fd75b18);
}
impl ::core::convert::From<IDCompositionRotateTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionRotateTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform3D_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, angle: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, axisx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, axisy: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, axisz: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centerx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centery: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centerz: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionSaturationEffect(pub ::windows::core::IUnknown);
impl IDCompositionSaturationEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetSaturation<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetSaturation2(&self, ratio: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(ratio)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSaturationEffect {
    type Vtable = IDCompositionSaturationEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa08debda_3258_4fa4_9f16_9174d3fe93b1);
}
impl ::core::convert::From<IDCompositionSaturationEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionSaturationEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSaturationEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ratio: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionScaleTransform(pub ::windows::core::IUnknown);
impl IDCompositionScaleTransform {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(scalex)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(scaley)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionScaleTransform {
    type Vtable = IDCompositionScaleTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71fde914_40ef_45ef_bd51_68b037c339f9);
}
impl ::core::convert::From<IDCompositionScaleTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionScaleTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scalex: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaley: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centerx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centery: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionScaleTransform3D(pub ::windows::core::IUnknown);
impl IDCompositionScaleTransform3D {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(scalex)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(scaley)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetScaleZ2(&self, scalez: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(scalez)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerz)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionScaleTransform3D {
    type Vtable = IDCompositionScaleTransform3D_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a9e9ead_364b_4b15_a7c4_a1997f78b389);
}
impl ::core::convert::From<IDCompositionScaleTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionScaleTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform3D_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scalex: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scaley: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scalez: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centerx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centery: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centerz: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionShadowEffect(pub ::windows::core::IUnknown);
impl IDCompositionShadowEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetStandardDeviation<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetColor(&self, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRed<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRed2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetGreen<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetGreen2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlue<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlue2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAlpha<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAlpha2(&self, amount: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(amount)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionShadowEffect {
    type Vtable = IDCompositionShadowEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ad18ac0_cfd2_4c2f_bb62_96e54fdb6879);
}
impl ::core::convert::From<IDCompositionShadowEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionShadowEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionShadowEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionShadowEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, amount: f32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: *const super::Direct2D::Common::D2D_VECTOR_4F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, amount: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, amount: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, amount: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, amount: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionSkewTransform(pub ::windows::core::IUnknown);
impl IDCompositionSkewTransform {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngleX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngleX2(&self, anglex: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(anglex)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngleY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAngleY2(&self, angley: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(angley)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(centerx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(centery)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSkewTransform {
    type Vtable = IDCompositionSkewTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe57aa735_dcdb_4c72_9c61_0591f58889ee);
}
impl ::core::convert::From<IDCompositionSkewTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionSkewTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionSkewTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSkewTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, anglex: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, angley: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centerx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, centery: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionSurface(pub ::windows::core::IUnknown);
impl IDCompositionSurface {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSurface {
    type Vtable = IDCompositionSurface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb8a4953_2c99_4f5a_96f5_4819027fa3ac);
}
impl ::core::convert::From<IDCompositionSurface> for ::windows::core::IUnknown {
    fn from(value: IDCompositionSurface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionSurface> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionSurface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurface_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionSurfaceFactory(pub ::windows::core::IUnknown);
impl IDCompositionSurfaceFactory {
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateSurface(&self, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi_Common`*"]
    pub unsafe fn CreateVirtualSurface(&self, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(initialwidth), ::core::mem::transmute(initialheight), ::core::mem::transmute(pixelformat), ::core::mem::transmute(alphamode), &mut result__).from_abi::<IDCompositionVirtualSurface>(result__)
    }
}
unsafe impl ::windows::core::Interface for IDCompositionSurfaceFactory {
    type Vtable = IDCompositionSurfaceFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe334bc12_3937_4e02_85eb_fcf4eb30d2c8);
}
impl ::core::convert::From<IDCompositionSurfaceFactory> for ::windows::core::IUnknown {
    fn from(value: IDCompositionSurfaceFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionSurfaceFactory> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionSurfaceFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionSurfaceFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionSurfaceFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurfaceFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, width: u32, height: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, surface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, initialwidth: u32, initialheight: u32, pixelformat: super::Dxgi::Common::DXGI_FORMAT, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE, virtualsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionTableTransferEffect(pub ::windows::core::IUnknown);
impl IDCompositionTableTransferEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRedTable(&self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(tablevalues), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetGreenTable(&self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(tablevalues), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlueTable(&self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(tablevalues), ::core::mem::transmute(count)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAlphaTable(&self, tablevalues: *const f32, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(tablevalues), ::core::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetRedDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, reddisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), reddisable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetGreenDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, greendisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), greendisable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetBlueDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bluedisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), bluedisable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetAlphaDisable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, alphadisable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), alphadisable.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetClampOutput<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, clampoutput: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), clampoutput.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRedTableValue<'a, Param1: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRedTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetGreenTableValue<'a, Param1: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetGreenTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlueTableValue<'a, Param1: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBlueTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAlphaTableValue<'a, Param1: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, index: u32, animation: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetAlphaTableValue2(&self, index: u32, value: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), ::core::mem::transmute(value)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTableTransferEffect {
    type Vtable = IDCompositionTableTransferEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b7e82e2_69c5_4eb4_a5f5_a7033f5132cd);
}
impl ::core::convert::From<IDCompositionTableTransferEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionTableTransferEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionTableTransferEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTableTransferEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, tablevalues: *const f32, count: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, reddisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, greendisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bluedisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, alphadisable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clampoutput: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, value: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, value: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionTarget(pub ::windows::core::IUnknown);
impl IDCompositionTarget {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetRoot<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTarget {
    type Vtable = IDCompositionTarget_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeacdd04c_117e_4e17_88f4_d1b12b0e3d89);
}
impl ::core::convert::From<IDCompositionTarget> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionTarget> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTarget {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionTransform(pub ::windows::core::IUnknown);
impl IDCompositionTransform {}
unsafe impl ::windows::core::Interface for IDCompositionTransform {
    type Vtable = IDCompositionTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd55faa7_37e0_4c20_95d2_9be45bc33f55);
}
impl ::core::convert::From<IDCompositionTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionTransform3D(pub ::windows::core::IUnknown);
impl IDCompositionTransform3D {}
unsafe impl ::windows::core::Interface for IDCompositionTransform3D {
    type Vtable = IDCompositionTransform3D_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71185722_246b_41f2_aad1_0443f7f4bfc2);
}
impl ::core::convert::From<IDCompositionTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTransform3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTransform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform3D_abi(pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32, pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionTranslateTransform(pub ::windows::core::IUnknown);
impl IDCompositionTranslateTransform {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTranslateTransform {
    type Vtable = IDCompositionTranslateTransform_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06791122_c6f0_417d_8323_269e987f5954);
}
impl ::core::convert::From<IDCompositionTranslateTransform> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform> for &IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionTranslateTransform {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsety: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionTranslateTransform3D(pub ::windows::core::IUnknown);
impl IDCompositionTranslateTransform3D {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetz)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTranslateTransform3D {
    type Vtable = IDCompositionTranslateTransform3D_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91636d4b_9ba1_4532_aaf7_e3344994d788);
}
impl ::core::convert::From<IDCompositionTranslateTransform3D> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionTranslateTransform3D> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionTransform3D> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionTranslateTransform3D {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform3D_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsety: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetz: f32) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionTurbulenceEffect(pub ::windows::core::IUnknown);
impl IDCompositionTurbulenceEffect {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetInput<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, index: u32, input: Param1, flags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(index), input.into_param().abi(), ::core::mem::transmute(flags)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetOffset(&self, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offset)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetBaseFrequency(&self, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(frequency)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetSize(&self, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(size)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetNumOctaves(&self, numoctaves: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(numoctaves)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetSeed(&self, seed: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(seed)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetNoise(&self, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(noise)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetStitchable<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, stitchable: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), stitchable.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionTurbulenceEffect {
    type Vtable = IDCompositionTurbulenceEffect_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6a55bda_c09c_49f3_9193_a41922c89715);
}
impl ::core::convert::From<IDCompositionTurbulenceEffect> for ::windows::core::IUnknown {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionTurbulenceEffect> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionFilterEffect> for &IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionFilterEffect> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionEffect> for &IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionEffect> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTurbulenceEffect_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, index: u32, input: ::windows::core::RawPtr, flags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offset: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, frequency: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, size: *const super::Direct2D::Common::D2D_VECTOR_2F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, numoctaves: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, seed: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, noise: super::Direct2D::Common::D2D1_TURBULENCE_NOISE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, stitchable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionVirtualSurface(pub ::windows::core::IUnknown);
impl IDCompositionVirtualSurface {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(updaterect), ::core::mem::transmute(iid), ::core::mem::transmute(updateobject), ::core::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(scrollrect), ::core::mem::transmute(cliprect), ::core::mem::transmute(offsetx), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn Resize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(width), ::core::mem::transmute(height)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn Trim(&self, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(rectangles), ::core::mem::transmute(count)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVirtualSurface {
    type Vtable = IDCompositionVirtualSurface_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xae471c51_5f53_4a24_8d3e_d0c39c30b3f0);
}
impl ::core::convert::From<IDCompositionVirtualSurface> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVirtualSurface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionVirtualSurface> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVirtualSurface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionSurface> for &IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionSurface> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVirtualSurface_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::core::GUID, updateobject: *mut *mut ::core::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, width: u32, height: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rectangles: *const super::super::Foundation::RECT, count: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionVisual(pub ::windows::core::IUnknown);
impl IDCompositionVisual {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Foundation_Numerics`*"]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransformParent<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetEffect<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), effect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), clip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn AddVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn RemoveVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(compositemode)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisual {
    type Vtable = IDCompositionVisual_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d93059d_097b_4651_9a60_f0f25116e2f3);
}
impl ::core::convert::From<IDCompositionVisual> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVisual) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionVisual> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVisual) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVisual {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsety: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionVisual2(pub ::windows::core::IUnknown);
impl IDCompositionVisual2 {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Foundation_Numerics`*"]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransformParent<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetEffect<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), effect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), clip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn AddVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn RemoveVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(compositemode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(visibility)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisual2 {
    type Vtable = IDCompositionVisual2_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8de1639_4331_4b26_bc5f_6a321d347a85);
}
impl ::core::convert::From<IDCompositionVisual2> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVisual2) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionVisual2> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVisual2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVisual2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVisual2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual> for &IDCompositionVisual2 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual2_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsety: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionVisual3(pub ::windows::core::IUnknown);
impl IDCompositionVisual3 {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Foundation_Numerics`*"]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransformParent<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetEffect<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), effect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), clip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn AddVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn RemoveVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(compositemode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(visibility)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn DisableHeatMap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetDepthMode(&self, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).26)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetZ<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).27)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).28)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetz)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOpacity<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).29)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).30)(::core::mem::transmute_copy(self), ::core::mem::transmute(opacity)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransform3<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform3D>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).31)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetTransform4(&self, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).32)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn SetVisible<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(&self, visible: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).33)(::core::mem::transmute_copy(self), visible.into_param().abi()).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisual3 {
    type Vtable = IDCompositionVisual3_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2775f462_b6c1_4015_b0be_b3e7d6a4976d);
}
impl ::core::convert::From<IDCompositionVisual3> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVisual3) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionVisual3> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVisual3) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisualDebug> for &IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisualDebug> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual2> for &IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual2> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual> for &IDCompositionVisual3 {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual3_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsety: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: DCOMPOSITION_DEPTH_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetz: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, opacity: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const super::Direct2D::Common::D2D_MATRIX_4X4_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IDCompositionVisualDebug(pub ::windows::core::IUnknown);
impl IDCompositionVisualDebug {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsetx)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionAnimation>>(&self, animation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), animation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransform<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionTransform>>(&self, transform: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), transform.into_param().abi()).ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Foundation_Numerics`*"]
    pub unsafe fn SetTransform2(&self, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(matrix)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetTransformParent<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetEffect<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionEffect>>(&self, effect: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), effect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBitmapInterpolationMode(&self, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(interpolationmode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBorderMode(&self, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(bordermode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetClip<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionClip>>(&self, clip: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), clip.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn SetClip2(&self, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(rect)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, content: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), content.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    pub unsafe fn AddVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0, insertabove: Param1, referencevisual: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), visual.into_param().abi(), insertabove.into_param().abi(), referencevisual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn RemoveVisual<'a, Param0: ::windows::core::IntoParam<'a, IDCompositionVisual>>(&self, visual: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), visual.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetCompositeMode(&self, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(compositemode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetOpacityMode(&self, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(mode)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn SetBackFaceVisibility(&self, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(visibility)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")]
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Direct2D_Common`*"]
    pub unsafe fn EnableHeatMap(&self, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(color)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn DisableHeatMap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).23)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).24)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).25)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for IDCompositionVisualDebug {
    type Vtable = IDCompositionVisualDebug_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfed2b808_5eb4_43a0_aea3_35f65280f91b);
}
impl ::core::convert::From<IDCompositionVisualDebug> for ::windows::core::IUnknown {
    fn from(value: IDCompositionVisualDebug) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDCompositionVisualDebug> for ::windows::core::IUnknown {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual2> for &IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual2> {
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
impl<'a> ::windows::core::IntoParam<'a, IDCompositionVisual> for &IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::core::Param<'a, IDCompositionVisual> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisualDebug_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsetx: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, animation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, offsety: f32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, transform: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, matrix: *const super::super::super::Foundation::Numerics::Matrix3x2) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, effect: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, bordermode: DCOMPOSITION_BORDER_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, clip: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, rect: *const super::Direct2D::Common::D2D_RECT_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr, insertabove: super::super::Foundation::BOOL, referencevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, compositemode: DCOMPOSITION_COMPOSITE_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, mode: DCOMPOSITION_OPACITY_MODE) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, visibility: DCOMPOSITION_BACKFACE_VISIBILITY) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D_Common")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, color: *const super::Direct2D::Common::D2D1_COLOR_F) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D_Common"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
