#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct COMPOSITION_FRAME_ID_TYPE(pub i32);
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(0i32);
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE =
    COMPOSITION_FRAME_ID_TYPE(1i32);
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE =
    COMPOSITION_FRAME_ID_TYPE(2i32);
impl ::std::convert::From<i32> for COMPOSITION_FRAME_ID_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for COMPOSITION_FRAME_ID_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
impl COMPOSITION_FRAME_STATS {}
impl ::std::default::Default for COMPOSITION_FRAME_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMPOSITION_FRAME_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMPOSITION_FRAME_STATS")
            .field("startTime", &self.startTime)
            .field("targetTime", &self.targetTime)
            .field("framePeriod", &self.framePeriod)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COMPOSITION_FRAME_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.startTime == other.startTime
            && self.targetTime == other.targetTime
            && self.framePeriod == other.framePeriod
    }
}
impl ::std::cmp::Eq for COMPOSITION_FRAME_STATS {}
unsafe impl ::windows::runtime::Abi for COMPOSITION_FRAME_STATS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
impl COMPOSITION_STATS {}
impl ::std::default::Default for COMPOSITION_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMPOSITION_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMPOSITION_STATS")
            .field("presentCount", &self.presentCount)
            .field("refreshCount", &self.refreshCount)
            .field("virtualRefreshCount", &self.virtualRefreshCount)
            .field("time", &self.time)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COMPOSITION_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.presentCount == other.presentCount
            && self.refreshCount == other.refreshCount
            && self.virtualRefreshCount == other.virtualRefreshCount
            && self.time == other.time
    }
}
impl ::std::cmp::Eq for COMPOSITION_STATS {}
unsafe impl ::windows::runtime::Abi for COMPOSITION_STATS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
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
impl ::std::default::Default for COMPOSITION_TARGET_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for COMPOSITION_TARGET_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMPOSITION_TARGET_ID")
            .field("displayAdapterLuid", &self.displayAdapterLuid)
            .field("renderAdapterLuid", &self.renderAdapterLuid)
            .field("vidPnSourceId", &self.vidPnSourceId)
            .field("vidPnTargetId", &self.vidPnTargetId)
            .field("uniqueId", &self.uniqueId)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for COMPOSITION_TARGET_ID {
    fn eq(&self, other: &Self) -> bool {
        self.displayAdapterLuid == other.displayAdapterLuid
            && self.renderAdapterLuid == other.renderAdapterLuid
            && self.vidPnSourceId == other.vidPnSourceId
            && self.vidPnTargetId == other.vidPnTargetId
            && self.uniqueId == other.uniqueId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for COMPOSITION_TARGET_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
impl COMPOSITION_TARGET_STATS {}
impl ::std::default::Default for COMPOSITION_TARGET_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMPOSITION_TARGET_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMPOSITION_TARGET_STATS")
            .field("outstandingPresents", &self.outstandingPresents)
            .field("presentTime", &self.presentTime)
            .field("vblankDuration", &self.vblankDuration)
            .field("presentedStats", &self.presentedStats)
            .field("completedStats", &self.completedStats)
            .finish()
    }
}
impl ::std::cmp::PartialEq for COMPOSITION_TARGET_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.outstandingPresents == other.outstandingPresents
            && self.presentTime == other.presentTime
            && self.vblankDuration == other.vblankDuration
            && self.presentedStats == other.presentedStats
            && self.completedStats == other.completedStats
    }
}
impl ::std::cmp::Eq for COMPOSITION_TARGET_STATS {}
unsafe impl ::windows::runtime::Abi for COMPOSITION_TARGET_STATS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(pub i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY =
    DCOMPOSITION_BACKFACE_VISIBILITY(0i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY =
    DCOMPOSITION_BACKFACE_VISIBILITY(1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY =
    DCOMPOSITION_BACKFACE_VISIBILITY(-1i32);
impl ::std::convert::From<i32> for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DCOMPOSITION_BACKFACE_VISIBILITY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(pub i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR:
    DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(0i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE =
    DCOMPOSITION_BITMAP_INTERPOLATION_MODE(1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE =
    DCOMPOSITION_BITMAP_INTERPOLATION_MODE(-1i32);
impl ::std::convert::From<i32> for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DCOMPOSITION_BORDER_MODE(pub i32);
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(0i32);
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(1i32);
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE =
    DCOMPOSITION_BORDER_MODE(-1i32);
impl ::std::convert::From<i32> for DCOMPOSITION_BORDER_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DCOMPOSITION_BORDER_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DCOMPOSITION_COMPOSITE_MODE(pub i32);
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE =
    DCOMPOSITION_COMPOSITE_MODE(0i32);
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE =
    DCOMPOSITION_COMPOSITE_MODE(1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE =
    DCOMPOSITION_COMPOSITE_MODE(2i32);
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE =
    DCOMPOSITION_COMPOSITE_MODE(-1i32);
impl ::std::convert::From<i32> for DCOMPOSITION_COMPOSITE_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DCOMPOSITION_COMPOSITE_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DCOMPOSITION_DEPTH_MODE(pub i32);
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(0i32);
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(1i32);
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(3i32);
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(-1i32);
impl ::std::convert::From<i32> for DCOMPOSITION_DEPTH_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DCOMPOSITION_DEPTH_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::std::default::Default for DCOMPOSITION_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::std::fmt::Debug for DCOMPOSITION_FRAME_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DCOMPOSITION_FRAME_STATISTICS")
            .field("lastFrameTime", &self.lastFrameTime)
            .field("currentCompositionRate", &self.currentCompositionRate)
            .field("currentTime", &self.currentTime)
            .field("timeFrequency", &self.timeFrequency)
            .field("nextEstimatedFrameTime", &self.nextEstimatedFrameTime)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::std::cmp::PartialEq for DCOMPOSITION_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.lastFrameTime == other.lastFrameTime
            && self.currentCompositionRate == other.currentCompositionRate
            && self.currentTime == other.currentTime
            && self.timeFrequency == other.timeFrequency
            && self.nextEstimatedFrameTime == other.nextEstimatedFrameTime
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
impl ::std::cmp::Eq for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi")]
unsafe impl ::windows::runtime::Abi for DCOMPOSITION_FRAME_STATISTICS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DCOMPOSITION_OPACITY_MODE(pub i32);
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE =
    DCOMPOSITION_OPACITY_MODE(0i32);
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE =
    DCOMPOSITION_OPACITY_MODE(1i32);
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE =
    DCOMPOSITION_OPACITY_MODE(-1i32);
impl ::std::convert::From<i32> for DCOMPOSITION_OPACITY_MODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DCOMPOSITION_OPACITY_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DCompositionAttachMouseDragToHwnd<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    visual: Param0,
    hwnd: Param1,
    enable: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionAttachMouseDragToHwnd(
                visual: ::windows::runtime::RawPtr,
                hwnd: super::super::Foundation::HWND,
                enable: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        DCompositionAttachMouseDragToHwnd(
            visual.into_param().abi(),
            hwnd.into_param().abi(),
            enable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DCompositionAttachMouseWheelToHwnd<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    visual: Param0,
    hwnd: Param1,
    enable: Param2,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionAttachMouseWheelToHwnd(
                visual: ::windows::runtime::RawPtr,
                hwnd: super::super::Foundation::HWND,
                enable: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        DCompositionAttachMouseWheelToHwnd(
            visual.into_param().abi(),
            hwnd.into_param().abi(),
            enable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DCompositionBoostCompositorClock<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
>(
    enable: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionBoostCompositorClock(
                enable: super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        DCompositionBoostCompositorClock(enable.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub unsafe fn DCompositionCreateDevice<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::Dxgi::IDXGIDevice>,
>(
    dxgidevice: Param0,
    iid: *const ::windows::runtime::GUID,
    dcompositiondevice: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice(
                dxgidevice: ::windows::runtime::RawPtr,
                iid: *const ::windows::runtime::GUID,
                dcompositiondevice: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        DCompositionCreateDevice(
            dxgidevice.into_param().abi(),
            ::std::mem::transmute(iid),
            ::std::mem::transmute(dcompositiondevice),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DCompositionCreateDevice2<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
>(
    renderingdevice: Param0,
    iid: *const ::windows::runtime::GUID,
    dcompositiondevice: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice2(
                renderingdevice: ::windows::runtime::RawPtr,
                iid: *const ::windows::runtime::GUID,
                dcompositiondevice: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        DCompositionCreateDevice2(
            renderingdevice.into_param().abi(),
            ::std::mem::transmute(iid),
            ::std::mem::transmute(dcompositiondevice),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DCompositionCreateDevice3<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
>(
    renderingdevice: Param0,
    iid: *const ::windows::runtime::GUID,
    dcompositiondevice: *mut *mut ::std::ffi::c_void,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateDevice3(
                renderingdevice: ::windows::runtime::RawPtr,
                iid: *const ::windows::runtime::GUID,
                dcompositiondevice: *mut *mut ::std::ffi::c_void,
            ) -> ::windows::runtime::HRESULT;
        }
        DCompositionCreateDevice3(
            renderingdevice.into_param().abi(),
            ::std::mem::transmute(iid),
            ::std::mem::transmute(dcompositiondevice),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub unsafe fn DCompositionCreateSurfaceHandle(
    desiredaccess: u32,
    securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionCreateSurfaceHandle(
                desiredaccess: u32,
                securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES,
                surfacehandle: *mut super::super::Foundation::HANDLE,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DCompositionCreateSurfaceHandle(
            ::std::mem::transmute(desiredaccess),
            ::std::mem::transmute(securityattributes),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub unsafe fn DCompositionGetFrameId(
    frameidtype: COMPOSITION_FRAME_ID_TYPE,
) -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetFrameId(
                frameidtype: COMPOSITION_FRAME_ID_TYPE,
                frameid: *mut u64,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DCompositionGetFrameId(::std::mem::transmute(frameidtype), &mut result__)
            .from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DCompositionGetStatistics(
    frameid: u64,
    framestats: *mut COMPOSITION_FRAME_STATS,
    targetidcount: u32,
    targetids: *mut COMPOSITION_TARGET_ID,
    actualtargetidcount: *mut u32,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetStatistics(
                frameid: u64,
                framestats: *mut COMPOSITION_FRAME_STATS,
                targetidcount: u32,
                targetids: *mut COMPOSITION_TARGET_ID,
                actualtargetidcount: *mut u32,
            ) -> ::windows::runtime::HRESULT;
        }
        DCompositionGetStatistics(
            ::std::mem::transmute(frameid),
            ::std::mem::transmute(framestats),
            ::std::mem::transmute(targetidcount),
            ::std::mem::transmute(targetids),
            ::std::mem::transmute(actualtargetidcount),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DCompositionGetTargetStatistics(
    frameid: u64,
    targetid: *const COMPOSITION_TARGET_ID,
) -> ::windows::runtime::Result<COMPOSITION_TARGET_STATS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionGetTargetStatistics(
                frameid: u64,
                targetid: *const COMPOSITION_TARGET_ID,
                targetstats: *mut COMPOSITION_TARGET_STATS,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <COMPOSITION_TARGET_STATS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DCompositionGetTargetStatistics(
            ::std::mem::transmute(frameid),
            ::std::mem::transmute(targetid),
            &mut result__,
        )
        .from_abi::<COMPOSITION_TARGET_STATS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl DCompositionInkTrailPoint {}
impl ::std::default::Default for DCompositionInkTrailPoint {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DCompositionInkTrailPoint {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DCompositionInkTrailPoint")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("radius", &self.radius)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DCompositionInkTrailPoint {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.radius == other.radius
    }
}
impl ::std::cmp::Eq for DCompositionInkTrailPoint {}
unsafe impl ::windows::runtime::Abi for DCompositionInkTrailPoint {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DCompositionWaitForCompositorClock(
    count: u32,
    handles: *const super::super::Foundation::HANDLE,
    timeoutinms: u32,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DCompositionWaitForCompositorClock(
                count: u32,
                handles: *const super::super::Foundation::HANDLE,
                timeoutinms: u32,
            ) -> u32;
        }
        ::std::mem::transmute(DCompositionWaitForCompositorClock(
            ::std::mem::transmute(count),
            ::std::mem::transmute(handles),
            ::std::mem::transmute(timeoutinms),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionAffineTransform2DEffect(::windows::runtime::IUnknown);
impl IDCompositionAffineTransform2DEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetInterpolationMode(
        &self,
        interpolationmode: super::Direct2D::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interpolationmode),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetBorderMode(
        &self,
        bordermode: super::Direct2D::D2D1_BORDER_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bordermode),
        )
        .ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransformMatrix(
        &self,
        transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transformmatrix),
        )
        .ok()
    }
    pub unsafe fn SetTransformMatrixElement<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        row: i32,
        column: i32,
        animation: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(row),
            ::std::mem::transmute(column),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTransformMatrixElement2(
        &self,
        row: i32,
        column: i32,
        value: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(row),
            ::std::mem::transmute(column),
            ::std::mem::transmute(value),
        )
        .ok()
    }
    pub unsafe fn SetSharpness<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetSharpness2(&self, sharpness: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(sharpness),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionAffineTransform2DEffect {
    type Vtable = IDCompositionAffineTransform2DEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        192199144,
        52694,
        18735,
        [187, 188, 94, 211, 33, 87, 2, 109],
    );
}
impl ::std::convert::From<IDCompositionAffineTransform2DEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionAffineTransform2DEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionAffineTransform2DEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionAffineTransform2DEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionAffineTransform2DEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionAffineTransform2DEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionAffineTransform2DEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionAffineTransform2DEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionAffineTransform2DEffect> for IDCompositionEffect {
    fn from(value: IDCompositionAffineTransform2DEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionAffineTransform2DEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionAffineTransform2DEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for IDCompositionAffineTransform2DEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionAffineTransform2DEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAffineTransform2DEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interpolationmode: super::Direct2D::D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bordermode: super::Direct2D::D2D1_BORDER_MODE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transformmatrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        row: i32,
        column: i32,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        row: i32,
        column: i32,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharpness: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionAnimation(::windows::runtime::IUnknown);
impl IDCompositionAnimation {
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetAbsoluteBeginTime(&self, begintime: i64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(begintime),
        )
        .ok()
    }
    pub unsafe fn AddCubic(
        &self,
        beginoffset: f64,
        constantcoefficient: f32,
        linearcoefficient: f32,
        quadraticcoefficient: f32,
        cubiccoefficient: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(beginoffset),
            ::std::mem::transmute(constantcoefficient),
            ::std::mem::transmute(linearcoefficient),
            ::std::mem::transmute(quadraticcoefficient),
            ::std::mem::transmute(cubiccoefficient),
        )
        .ok()
    }
    pub unsafe fn AddSinusoidal(
        &self,
        beginoffset: f64,
        bias: f32,
        amplitude: f32,
        frequency: f32,
        phase: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(beginoffset),
            ::std::mem::transmute(bias),
            ::std::mem::transmute(amplitude),
            ::std::mem::transmute(frequency),
            ::std::mem::transmute(phase),
        )
        .ok()
    }
    pub unsafe fn AddRepeat(
        &self,
        beginoffset: f64,
        durationtorepeat: f64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(beginoffset),
            ::std::mem::transmute(durationtorepeat),
        )
        .ok()
    }
    pub unsafe fn End(&self, endoffset: f64, endvalue: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(endoffset),
            ::std::mem::transmute(endvalue),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionAnimation {
    type Vtable = IDCompositionAnimation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3422392793,
        20914,
        17892,
        [179, 222, 209, 156, 207, 184, 99, 197],
    );
}
impl ::std::convert::From<IDCompositionAnimation> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionAnimation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionAnimation> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionAnimation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionAnimation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionAnimation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionAnimation_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        begintime: i64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        beginoffset: f64,
        constantcoefficient: f32,
        linearcoefficient: f32,
        quadraticcoefficient: f32,
        cubiccoefficient: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        beginoffset: f64,
        bias: f32,
        amplitude: f32,
        frequency: f32,
        phase: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        beginoffset: f64,
        durationtorepeat: f64,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        endoffset: f64,
        endvalue: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionArithmeticCompositeEffect(::windows::runtime::IUnknown);
impl IDCompositionArithmeticCompositeEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetCoefficients(
        &self,
        coefficients: *const super::Direct2D::D2D_VECTOR_4F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(coefficients),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        clampoutput: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            clampoutput.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCoefficient1<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCoefficient12(&self, coeffcient1: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(coeffcient1),
        )
        .ok()
    }
    pub unsafe fn SetCoefficient2<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCoefficient22(&self, coefficient2: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(coefficient2),
        )
        .ok()
    }
    pub unsafe fn SetCoefficient3<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCoefficient32(&self, coefficient3: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(coefficient3),
        )
        .ok()
    }
    pub unsafe fn SetCoefficient4<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCoefficient42(&self, coefficient4: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(coefficient4),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionArithmeticCompositeEffect {
    type Vtable = IDCompositionArithmeticCompositeEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        996663208,
        58333,
        20065,
        [182, 64, 70, 194, 243, 215, 57, 220],
    );
}
impl ::std::convert::From<IDCompositionArithmeticCompositeEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionArithmeticCompositeEffect>
    for ::windows::runtime::IUnknown
{
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionArithmeticCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionArithmeticCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionArithmeticCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionArithmeticCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionArithmeticCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionArithmeticCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionArithmeticCompositeEffect> for IDCompositionEffect {
    fn from(value: IDCompositionArithmeticCompositeEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionArithmeticCompositeEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionArithmeticCompositeEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for IDCompositionArithmeticCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionArithmeticCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionArithmeticCompositeEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coefficients: *const super::Direct2D::D2D_VECTOR_4F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clampoutput: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coeffcient1: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coefficient2: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coefficient3: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        coefficient4: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionBlendEffect(::windows::runtime::IUnknown);
impl IDCompositionBlendEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetMode(
        &self,
        mode: super::Direct2D::D2D1_BLEND_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionBlendEffect {
    type Vtable = IDCompositionBlendEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        871160842,
        22410,
        18961,
        [156, 20, 12, 185, 5, 23, 249, 197],
    );
}
impl ::std::convert::From<IDCompositionBlendEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionBlendEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionBlendEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionBlendEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionBlendEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionBlendEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionBlendEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionBlendEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect> for IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionBlendEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionBlendEffect> for IDCompositionEffect {
    fn from(value: IDCompositionBlendEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionBlendEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionBlendEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionBlendEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBlendEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: super::Direct2D::D2D1_BLEND_MODE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionBrightnessEffect(::windows::runtime::IUnknown);
impl IDCompositionBrightnessEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetWhitePoint(
        &self,
        whitepoint: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(whitepoint),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetBlackPoint(
        &self,
        blackpoint: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(blackpoint),
        )
        .ok()
    }
    pub unsafe fn SetWhitePointX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetWhitePointX2(&self, whitepointx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(whitepointx),
        )
        .ok()
    }
    pub unsafe fn SetWhitePointY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetWhitePointY2(&self, whitepointy: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(whitepointy),
        )
        .ok()
    }
    pub unsafe fn SetBlackPointX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBlackPointX2(&self, blackpointx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(blackpointx),
        )
        .ok()
    }
    pub unsafe fn SetBlackPointY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBlackPointY2(&self, blackpointy: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(blackpointy),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionBrightnessEffect {
    type Vtable = IDCompositionBrightnessEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1613187438,
        52026,
        18859,
        [147, 79, 215, 152, 218, 79, 125, 166],
    );
}
impl ::std::convert::From<IDCompositionBrightnessEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionBrightnessEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionBrightnessEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionBrightnessEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionBrightnessEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionBrightnessEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionBrightnessEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionBrightnessEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionBrightnessEffect> for IDCompositionEffect {
    fn from(value: IDCompositionBrightnessEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionBrightnessEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionBrightnessEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionBrightnessEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionBrightnessEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        whitepoint: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        blackpoint: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        whitepointx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        whitepointy: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        blackpointx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        blackpointy: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionClip(::windows::runtime::IUnknown);
impl IDCompositionClip {}
unsafe impl ::windows::runtime::Interface for IDCompositionClip {
    type Vtable = IDCompositionClip_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1689007875,
        40255,
        17900,
        [161, 9, 124, 172, 14, 122, 19, 167],
    );
}
impl ::std::convert::From<IDCompositionClip> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionClip) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionClip> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionClip) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionClip {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionClip {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionClip_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionColorMatrixEffect(::windows::runtime::IUnknown);
impl IDCompositionColorMatrixEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetMatrix(
        &self,
        matrix: *const super::Direct2D::D2D_MATRIX_5X4_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(matrix),
        )
        .ok()
    }
    pub unsafe fn SetMatrixElement<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        row: i32,
        column: i32,
        animation: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(row),
            ::std::mem::transmute(column),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetMatrixElement2(
        &self,
        row: i32,
        column: i32,
        value: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(row),
            ::std::mem::transmute(column),
            ::std::mem::transmute(value),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetAlphaMode(
        &self,
        mode: super::Direct2D::D2D1_COLORMATRIX_ALPHA_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        clamp: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            clamp.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionColorMatrixEffect {
    type Vtable = IDCompositionColorMatrixEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3239512610,
        15586,
        18790,
        [144, 212, 85, 64, 139, 252, 132, 196],
    );
}
impl ::std::convert::From<IDCompositionColorMatrixEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionColorMatrixEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionColorMatrixEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionColorMatrixEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionColorMatrixEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionColorMatrixEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionColorMatrixEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionColorMatrixEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionColorMatrixEffect> for IDCompositionEffect {
    fn from(value: IDCompositionColorMatrixEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionColorMatrixEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionColorMatrixEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionColorMatrixEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionColorMatrixEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionColorMatrixEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrix: *const super::Direct2D::D2D_MATRIX_5X4_F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        row: i32,
        column: i32,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        row: i32,
        column: i32,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: super::Direct2D::D2D1_COLORMATRIX_ALPHA_MODE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clamp: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionCompositeEffect(::windows::runtime::IUnknown);
impl IDCompositionCompositeEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetMode(
        &self,
        mode: super::Direct2D::D2D1_COMPOSITE_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionCompositeEffect {
    type Vtable = IDCompositionCompositeEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1466308288,
        41521,
        18765,
        [163, 141, 0, 253, 94, 196, 219, 70],
    );
}
impl ::std::convert::From<IDCompositionCompositeEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionCompositeEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionCompositeEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionCompositeEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionCompositeEffect> for IDCompositionEffect {
    fn from(value: IDCompositionCompositeEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionCompositeEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionCompositeEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionCompositeEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionCompositeEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: super::Direct2D::D2D1_COMPOSITE_MODE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionDelegatedInkTrail(::windows::runtime::IUnknown);
impl IDCompositionDelegatedInkTrail {
    pub unsafe fn AddTrailPoints(
        &self,
        inkpoints: *const DCompositionInkTrailPoint,
        inkpointscount: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(inkpoints),
            ::std::mem::transmute(inkpointscount),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn AddTrailPointsWithPrediction(
        &self,
        inkpoints: *const DCompositionInkTrailPoint,
        inkpointscount: u32,
        predictedinkpoints: *const DCompositionInkTrailPoint,
        predictedinkpointscount: u32,
    ) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(inkpoints),
            ::std::mem::transmute(inkpointscount),
            ::std::mem::transmute(predictedinkpoints),
            ::std::mem::transmute(predictedinkpointscount),
            &mut result__,
        )
        .from_abi::<u32>(result__)
    }
    pub unsafe fn RemoveTrailPoints(&self, generationid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(generationid),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn StartNewTrail(
        &self,
        color: *const super::Direct2D::D2D1_COLOR_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(color),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionDelegatedInkTrail {
    type Vtable = IDCompositionDelegatedInkTrail_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3259272859,
        21629,
        16471,
        [140, 245, 129, 68, 237, 225, 194, 218],
    );
}
impl ::std::convert::From<IDCompositionDelegatedInkTrail> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionDelegatedInkTrail) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionDelegatedInkTrail> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionDelegatedInkTrail) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionDelegatedInkTrail
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionDelegatedInkTrail
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDelegatedInkTrail_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inkpoints: *const DCompositionInkTrailPoint,
        inkpointscount: u32,
        generationid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inkpoints: *const DCompositionInkTrailPoint,
        inkpointscount: u32,
        predictedinkpoints: *const DCompositionInkTrailPoint,
        predictedinkpointscount: u32,
        generationid: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        generationid: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        color: *const super::Direct2D::D2D1_COLOR_F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionDesktopDevice(::windows::runtime::IUnknown);
impl IDCompositionDesktopDevice {
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetFrameStatistics(
        &self,
    ) -> ::windows::runtime::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: <DCOMPOSITION_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows::runtime::Result<IDCompositionVisual2> {
        let mut result__: <IDCompositionVisual2 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionVisual2>(result__)
    }
    pub unsafe fn CreateSurfaceFactory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        renderingdevice: Param0,
    ) -> ::windows::runtime::Result<IDCompositionSurfaceFactory> {
        let mut result__: <IDCompositionSurfaceFactory as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            renderingdevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateSurface(
        &self,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateVirtualSurface(
        &self,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(initialwidth),
            ::std::mem::transmute(initialheight),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionVirtualSurface>(result__)
    }
    pub unsafe fn CreateTranslateTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTranslateTransform> {
        let mut result__: <IDCompositionTranslateTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTranslateTransform>(result__)
    }
    pub unsafe fn CreateScaleTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionScaleTransform> {
        let mut result__: <IDCompositionScaleTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionScaleTransform>(result__)
    }
    pub unsafe fn CreateRotateTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRotateTransform> {
        let mut result__: <IDCompositionRotateTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRotateTransform>(result__)
    }
    pub unsafe fn CreateSkewTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionSkewTransform> {
        let mut result__: <IDCompositionSkewTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionSkewTransform>(result__)
    }
    pub unsafe fn CreateMatrixTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionMatrixTransform> {
        let mut result__: <IDCompositionMatrixTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionMatrixTransform>(result__)
    }
    pub unsafe fn CreateTransformGroup(
        &self,
        transforms: *const ::std::option::Option<IDCompositionTransform>,
        elements: u32,
    ) -> ::windows::runtime::Result<IDCompositionTransform> {
        let mut result__: <IDCompositionTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transforms),
            ::std::mem::transmute(elements),
            &mut result__,
        )
        .from_abi::<IDCompositionTransform>(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTranslateTransform3D> {
        let mut result__: <IDCompositionTranslateTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    pub unsafe fn CreateScaleTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionScaleTransform3D> {
        let mut result__: <IDCompositionScaleTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    pub unsafe fn CreateRotateTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRotateTransform3D> {
        let mut result__: <IDCompositionRotateTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionMatrixTransform3D> {
        let mut result__: <IDCompositionMatrixTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    pub unsafe fn CreateTransform3DGroup(
        &self,
        transforms3d: *const ::std::option::Option<IDCompositionTransform3D>,
        elements: u32,
    ) -> ::windows::runtime::Result<IDCompositionTransform3D> {
        let mut result__: <IDCompositionTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transforms3d),
            ::std::mem::transmute(elements),
            &mut result__,
        )
        .from_abi::<IDCompositionTransform3D>(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::runtime::Result<IDCompositionEffectGroup> {
        let mut result__: <IDCompositionEffectGroup as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionEffectGroup>(result__)
    }
    pub unsafe fn CreateRectangleClip(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRectangleClip> {
        let mut result__: <IDCompositionRectangleClip as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRectangleClip>(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows::runtime::Result<IDCompositionAnimation> {
        let mut result__: <IDCompositionAnimation as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionAnimation>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTargetForHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hwnd: Param0,
        topmost: Param1,
    ) -> ::windows::runtime::Result<IDCompositionTarget> {
        let mut result__: <IDCompositionTarget as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            hwnd.into_param().abi(),
            topmost.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDCompositionTarget>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHandle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        handle: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            handle.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        hwnd: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            hwnd.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionDesktopDevice {
    type Vtable = IDCompositionDesktopDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1598436350,
        7688,
        19640,
        [140, 117, 206, 36, 51, 63, 86, 2],
    );
}
impl ::std::convert::From<IDCompositionDesktopDevice> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionDesktopDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionDesktopDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionDesktopDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionDesktopDevice
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionDesktopDevice
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionDesktopDevice> for IDCompositionDevice2 {
    fn from(value: IDCompositionDesktopDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionDesktopDevice> for IDCompositionDevice2 {
    fn from(value: &IDCompositionDesktopDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionDevice2> for IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionDevice2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionDevice2> for &IDCompositionDesktopDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionDevice2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDesktopDevice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        statistics: *mut DCOMPOSITION_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        renderingdevice: ::windows::runtime::RawPtr,
        surfacefactory: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        surface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        virtualsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        translatetransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaletransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotatetransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        skewtransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrixtransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transforms: *const ::windows::runtime::RawPtr,
        elements: u32,
        transformgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        translatetransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaletransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotatetransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrixtransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transforms3d: *const ::windows::runtime::RawPtr,
        elements: u32,
        transform3dgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clip: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        topmost: super::super::Foundation::BOOL,
        target: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handle: super::super::Foundation::HANDLE,
        surface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        surface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionDevice(::windows::runtime::IUnknown);
impl IDCompositionDevice {
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetFrameStatistics(
        &self,
    ) -> ::windows::runtime::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: <DCOMPOSITION_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTargetForHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        hwnd: Param0,
        topmost: Param1,
    ) -> ::windows::runtime::Result<IDCompositionTarget> {
        let mut result__: <IDCompositionTarget as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            hwnd.into_param().abi(),
            topmost.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDCompositionTarget>(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows::runtime::Result<IDCompositionVisual> {
        let mut result__: <IDCompositionVisual as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionVisual>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateSurface(
        &self,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateVirtualSurface(
        &self,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(initialwidth),
            ::std::mem::transmute(initialheight),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionVirtualSurface>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHandle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    >(
        &self,
        handle: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            handle.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSurfaceFromHwnd<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>,
    >(
        &self,
        hwnd: Param0,
    ) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            hwnd.into_param().abi(),
            &mut result__,
        )
        .from_abi::<::windows::runtime::IUnknown>(result__)
    }
    pub unsafe fn CreateTranslateTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTranslateTransform> {
        let mut result__: <IDCompositionTranslateTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTranslateTransform>(result__)
    }
    pub unsafe fn CreateScaleTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionScaleTransform> {
        let mut result__: <IDCompositionScaleTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionScaleTransform>(result__)
    }
    pub unsafe fn CreateRotateTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRotateTransform> {
        let mut result__: <IDCompositionRotateTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRotateTransform>(result__)
    }
    pub unsafe fn CreateSkewTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionSkewTransform> {
        let mut result__: <IDCompositionSkewTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionSkewTransform>(result__)
    }
    pub unsafe fn CreateMatrixTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionMatrixTransform> {
        let mut result__: <IDCompositionMatrixTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionMatrixTransform>(result__)
    }
    pub unsafe fn CreateTransformGroup(
        &self,
        transforms: *const ::std::option::Option<IDCompositionTransform>,
        elements: u32,
    ) -> ::windows::runtime::Result<IDCompositionTransform> {
        let mut result__: <IDCompositionTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transforms),
            ::std::mem::transmute(elements),
            &mut result__,
        )
        .from_abi::<IDCompositionTransform>(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTranslateTransform3D> {
        let mut result__: <IDCompositionTranslateTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    pub unsafe fn CreateScaleTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionScaleTransform3D> {
        let mut result__: <IDCompositionScaleTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    pub unsafe fn CreateRotateTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRotateTransform3D> {
        let mut result__: <IDCompositionRotateTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionMatrixTransform3D> {
        let mut result__: <IDCompositionMatrixTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    pub unsafe fn CreateTransform3DGroup(
        &self,
        transforms3d: *const ::std::option::Option<IDCompositionTransform3D>,
        elements: u32,
    ) -> ::windows::runtime::Result<IDCompositionTransform3D> {
        let mut result__: <IDCompositionTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transforms3d),
            ::std::mem::transmute(elements),
            &mut result__,
        )
        .from_abi::<IDCompositionTransform3D>(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::runtime::Result<IDCompositionEffectGroup> {
        let mut result__: <IDCompositionEffectGroup as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionEffectGroup>(result__)
    }
    pub unsafe fn CreateRectangleClip(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRectangleClip> {
        let mut result__: <IDCompositionRectangleClip as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRectangleClip>(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows::runtime::Result<IDCompositionAnimation> {
        let mut result__: <IDCompositionAnimation as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionAnimation>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CheckDeviceState(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BOOL>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionDevice {
    type Vtable = IDCompositionDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3279857978,
        59306,
        17677,
        [177, 111, 151, 70, 203, 4, 7, 243],
    );
}
impl ::std::convert::From<IDCompositionDevice> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        statistics: *mut DCOMPOSITION_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        topmost: super::super::Foundation::BOOL,
        target: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        surface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        virtualsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handle: super::super::Foundation::HANDLE,
        surface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hwnd: super::super::Foundation::HWND,
        surface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        translatetransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaletransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotatetransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        skewtransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrixtransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transforms: *const ::windows::runtime::RawPtr,
        elements: u32,
        transformgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        translatetransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaletransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotatetransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrixtransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transforms3d: *const ::windows::runtime::RawPtr,
        elements: u32,
        transform3dgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clip: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pfvalid: *mut super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionDevice2(::windows::runtime::IUnknown);
impl IDCompositionDevice2 {
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetFrameStatistics(
        &self,
    ) -> ::windows::runtime::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: <DCOMPOSITION_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows::runtime::Result<IDCompositionVisual2> {
        let mut result__: <IDCompositionVisual2 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionVisual2>(result__)
    }
    pub unsafe fn CreateSurfaceFactory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        renderingdevice: Param0,
    ) -> ::windows::runtime::Result<IDCompositionSurfaceFactory> {
        let mut result__: <IDCompositionSurfaceFactory as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            renderingdevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateSurface(
        &self,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateVirtualSurface(
        &self,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(initialwidth),
            ::std::mem::transmute(initialheight),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionVirtualSurface>(result__)
    }
    pub unsafe fn CreateTranslateTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTranslateTransform> {
        let mut result__: <IDCompositionTranslateTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTranslateTransform>(result__)
    }
    pub unsafe fn CreateScaleTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionScaleTransform> {
        let mut result__: <IDCompositionScaleTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionScaleTransform>(result__)
    }
    pub unsafe fn CreateRotateTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRotateTransform> {
        let mut result__: <IDCompositionRotateTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRotateTransform>(result__)
    }
    pub unsafe fn CreateSkewTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionSkewTransform> {
        let mut result__: <IDCompositionSkewTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionSkewTransform>(result__)
    }
    pub unsafe fn CreateMatrixTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionMatrixTransform> {
        let mut result__: <IDCompositionMatrixTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionMatrixTransform>(result__)
    }
    pub unsafe fn CreateTransformGroup(
        &self,
        transforms: *const ::std::option::Option<IDCompositionTransform>,
        elements: u32,
    ) -> ::windows::runtime::Result<IDCompositionTransform> {
        let mut result__: <IDCompositionTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transforms),
            ::std::mem::transmute(elements),
            &mut result__,
        )
        .from_abi::<IDCompositionTransform>(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTranslateTransform3D> {
        let mut result__: <IDCompositionTranslateTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    pub unsafe fn CreateScaleTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionScaleTransform3D> {
        let mut result__: <IDCompositionScaleTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    pub unsafe fn CreateRotateTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRotateTransform3D> {
        let mut result__: <IDCompositionRotateTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionMatrixTransform3D> {
        let mut result__: <IDCompositionMatrixTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    pub unsafe fn CreateTransform3DGroup(
        &self,
        transforms3d: *const ::std::option::Option<IDCompositionTransform3D>,
        elements: u32,
    ) -> ::windows::runtime::Result<IDCompositionTransform3D> {
        let mut result__: <IDCompositionTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transforms3d),
            ::std::mem::transmute(elements),
            &mut result__,
        )
        .from_abi::<IDCompositionTransform3D>(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::runtime::Result<IDCompositionEffectGroup> {
        let mut result__: <IDCompositionEffectGroup as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionEffectGroup>(result__)
    }
    pub unsafe fn CreateRectangleClip(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRectangleClip> {
        let mut result__: <IDCompositionRectangleClip as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRectangleClip>(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows::runtime::Result<IDCompositionAnimation> {
        let mut result__: <IDCompositionAnimation as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionAnimation>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionDevice2 {
    type Vtable = IDCompositionDevice2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1979074189,
        7054,
        17532,
        [155, 198, 117, 254, 168, 11, 91, 37],
    );
}
impl ::std::convert::From<IDCompositionDevice2> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionDevice2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionDevice2> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionDevice2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionDevice2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        statistics: *mut DCOMPOSITION_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        renderingdevice: ::windows::runtime::RawPtr,
        surfacefactory: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        surface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        virtualsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        translatetransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaletransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotatetransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        skewtransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrixtransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transforms: *const ::windows::runtime::RawPtr,
        elements: u32,
        transformgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        translatetransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaletransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotatetransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrixtransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transforms3d: *const ::windows::runtime::RawPtr,
        elements: u32,
        transform3dgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clip: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionDevice3(::windows::runtime::IUnknown);
impl IDCompositionDevice3 {
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn WaitForCommitCompletion(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn GetFrameStatistics(
        &self,
    ) -> ::windows::runtime::Result<DCOMPOSITION_FRAME_STATISTICS> {
        let mut result__: <DCOMPOSITION_FRAME_STATISTICS as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<DCOMPOSITION_FRAME_STATISTICS>(result__)
    }
    pub unsafe fn CreateVisual(&self) -> ::windows::runtime::Result<IDCompositionVisual2> {
        let mut result__: <IDCompositionVisual2 as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionVisual2>(result__)
    }
    pub unsafe fn CreateSurfaceFactory<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        renderingdevice: Param0,
    ) -> ::windows::runtime::Result<IDCompositionSurfaceFactory> {
        let mut result__: <IDCompositionSurfaceFactory as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            renderingdevice.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDCompositionSurfaceFactory>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateSurface(
        &self,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateVirtualSurface(
        &self,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(initialwidth),
            ::std::mem::transmute(initialheight),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionVirtualSurface>(result__)
    }
    pub unsafe fn CreateTranslateTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTranslateTransform> {
        let mut result__: <IDCompositionTranslateTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTranslateTransform>(result__)
    }
    pub unsafe fn CreateScaleTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionScaleTransform> {
        let mut result__: <IDCompositionScaleTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionScaleTransform>(result__)
    }
    pub unsafe fn CreateRotateTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRotateTransform> {
        let mut result__: <IDCompositionRotateTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRotateTransform>(result__)
    }
    pub unsafe fn CreateSkewTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionSkewTransform> {
        let mut result__: <IDCompositionSkewTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionSkewTransform>(result__)
    }
    pub unsafe fn CreateMatrixTransform(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionMatrixTransform> {
        let mut result__: <IDCompositionMatrixTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionMatrixTransform>(result__)
    }
    pub unsafe fn CreateTransformGroup(
        &self,
        transforms: *const ::std::option::Option<IDCompositionTransform>,
        elements: u32,
    ) -> ::windows::runtime::Result<IDCompositionTransform> {
        let mut result__: <IDCompositionTransform as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transforms),
            ::std::mem::transmute(elements),
            &mut result__,
        )
        .from_abi::<IDCompositionTransform>(result__)
    }
    pub unsafe fn CreateTranslateTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTranslateTransform3D> {
        let mut result__: <IDCompositionTranslateTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTranslateTransform3D>(result__)
    }
    pub unsafe fn CreateScaleTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionScaleTransform3D> {
        let mut result__: <IDCompositionScaleTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionScaleTransform3D>(result__)
    }
    pub unsafe fn CreateRotateTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRotateTransform3D> {
        let mut result__: <IDCompositionRotateTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRotateTransform3D>(result__)
    }
    pub unsafe fn CreateMatrixTransform3D(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionMatrixTransform3D> {
        let mut result__: <IDCompositionMatrixTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionMatrixTransform3D>(result__)
    }
    pub unsafe fn CreateTransform3DGroup(
        &self,
        transforms3d: *const ::std::option::Option<IDCompositionTransform3D>,
        elements: u32,
    ) -> ::windows::runtime::Result<IDCompositionTransform3D> {
        let mut result__: <IDCompositionTransform3D as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(transforms3d),
            ::std::mem::transmute(elements),
            &mut result__,
        )
        .from_abi::<IDCompositionTransform3D>(result__)
    }
    pub unsafe fn CreateEffectGroup(&self) -> ::windows::runtime::Result<IDCompositionEffectGroup> {
        let mut result__: <IDCompositionEffectGroup as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionEffectGroup>(result__)
    }
    pub unsafe fn CreateRectangleClip(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionRectangleClip> {
        let mut result__: <IDCompositionRectangleClip as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionRectangleClip>(result__)
    }
    pub unsafe fn CreateAnimation(&self) -> ::windows::runtime::Result<IDCompositionAnimation> {
        let mut result__: <IDCompositionAnimation as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionAnimation>(result__)
    }
    pub unsafe fn CreateGaussianBlurEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionGaussianBlurEffect> {
        let mut result__: <IDCompositionGaussianBlurEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionGaussianBlurEffect>(result__)
    }
    pub unsafe fn CreateBrightnessEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionBrightnessEffect> {
        let mut result__: <IDCompositionBrightnessEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionBrightnessEffect>(result__)
    }
    pub unsafe fn CreateColorMatrixEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionColorMatrixEffect> {
        let mut result__: <IDCompositionColorMatrixEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionColorMatrixEffect>(result__)
    }
    pub unsafe fn CreateShadowEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionShadowEffect> {
        let mut result__: <IDCompositionShadowEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionShadowEffect>(result__)
    }
    pub unsafe fn CreateHueRotationEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionHueRotationEffect> {
        let mut result__: <IDCompositionHueRotationEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionHueRotationEffect>(result__)
    }
    pub unsafe fn CreateSaturationEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionSaturationEffect> {
        let mut result__: <IDCompositionSaturationEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionSaturationEffect>(result__)
    }
    pub unsafe fn CreateTurbulenceEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTurbulenceEffect> {
        let mut result__: <IDCompositionTurbulenceEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTurbulenceEffect>(result__)
    }
    pub unsafe fn CreateLinearTransferEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionLinearTransferEffect> {
        let mut result__: <IDCompositionLinearTransferEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionLinearTransferEffect>(result__)
    }
    pub unsafe fn CreateTableTransferEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionTableTransferEffect> {
        let mut result__: <IDCompositionTableTransferEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionTableTransferEffect>(result__)
    }
    pub unsafe fn CreateCompositeEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionCompositeEffect> {
        let mut result__: <IDCompositionCompositeEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionCompositeEffect>(result__)
    }
    pub unsafe fn CreateBlendEffect(&self) -> ::windows::runtime::Result<IDCompositionBlendEffect> {
        let mut result__: <IDCompositionBlendEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).34)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionBlendEffect>(result__)
    }
    pub unsafe fn CreateArithmeticCompositeEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionArithmeticCompositeEffect> {
        let mut result__: <IDCompositionArithmeticCompositeEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).35)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionArithmeticCompositeEffect>(result__)
    }
    pub unsafe fn CreateAffineTransform2DEffect(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionAffineTransform2DEffect> {
        let mut result__: <IDCompositionAffineTransform2DEffect as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionAffineTransform2DEffect>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionDevice3 {
    type Vtable = IDCompositionDevice3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        159894278,
        63766,
        18623,
        [141, 53, 206, 118, 65, 120, 27, 217],
    );
}
impl ::std::convert::From<IDCompositionDevice3> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionDevice3> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionDevice3> for IDCompositionDevice2 {
    fn from(value: IDCompositionDevice3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionDevice3> for IDCompositionDevice2 {
    fn from(value: &IDCompositionDevice3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionDevice2> for IDCompositionDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionDevice2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionDevice2> for &IDCompositionDevice3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionDevice2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionDevice2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDevice3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        statistics: *mut DCOMPOSITION_FRAME_STATISTICS,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        renderingdevice: ::windows::runtime::RawPtr,
        surfacefactory: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        surface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        virtualsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        translatetransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaletransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotatetransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        skewtransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrixtransform: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transforms: *const ::windows::runtime::RawPtr,
        elements: u32,
        transformgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        translatetransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaletransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rotatetransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrixtransform3d: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transforms3d: *const ::windows::runtime::RawPtr,
        elements: u32,
        transform3dgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effectgroup: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clip: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        gaussianblureffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        brightnesseffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        colormatrixeffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        shadoweffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        huerotationeffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        saturationeffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        turbulenceeffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        lineartransfereffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tabletransfereffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositeeffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        blendeffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        arithmeticcompositeeffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        affinetransform2deffect: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionDeviceDebug(::windows::runtime::IUnknown);
impl IDCompositionDeviceDebug {
    pub unsafe fn EnableDebugCounters(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisableDebugCounters(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionDeviceDebug {
    type Vtable = IDCompositionDeviceDebug_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2711864906,
        8783,
        19073,
        [151, 115, 79, 3, 168, 157, 60, 108],
    );
}
impl ::std::convert::From<IDCompositionDeviceDebug> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionDeviceDebug) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionDeviceDebug> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionDeviceDebug) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionDeviceDebug
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionDeviceDebug
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionDeviceDebug_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionEffect(::windows::runtime::IUnknown);
impl IDCompositionEffect {}
unsafe impl ::windows::runtime::Interface for IDCompositionEffect {
    type Vtable = IDCompositionEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3967922319,
        49099,
        20109,
        [177, 147, 169, 21, 88, 121, 153, 232],
    );
}
impl ::std::convert::From<IDCompositionEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionEffectGroup(::windows::runtime::IUnknown);
impl IDCompositionEffectGroup {
    pub unsafe fn SetOpacity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(opacity),
        )
        .ok()
    }
    pub unsafe fn SetTransform3D<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>,
    >(
        &self,
        transform3d: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            transform3d.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionEffectGroup {
    type Vtable = IDCompositionEffectGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2811402868,
        59058,
        19414,
        [139, 149, 64, 64, 17, 156, 163, 77],
    );
}
impl ::std::convert::From<IDCompositionEffectGroup> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionEffectGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionEffectGroup> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionEffectGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionEffectGroup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionEffectGroup
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionEffectGroup> for IDCompositionEffect {
    fn from(value: IDCompositionEffectGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionEffectGroup> for IDCompositionEffect {
    fn from(value: &IDCompositionEffectGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionEffectGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionEffectGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionEffectGroup_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        opacity: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transform3d: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionFilterEffect(::windows::runtime::IUnknown);
impl IDCompositionFilterEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionFilterEffect {
    type Vtable = IDCompositionFilterEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        818160085,
        36018,
        20127,
        [177, 51, 55, 190, 39, 13, 74, 194],
    );
}
impl ::std::convert::From<IDCompositionFilterEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionFilterEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionFilterEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionFilterEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionFilterEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionFilterEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionFilterEffect> for IDCompositionEffect {
    fn from(value: IDCompositionFilterEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionFilterEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionFilterEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionFilterEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionFilterEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionFilterEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionGaussianBlurEffect(::windows::runtime::IUnknown);
impl IDCompositionGaussianBlurEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn SetStandardDeviation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(amount),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetBorderMode(
        &self,
        mode: super::Direct2D::D2D1_BORDER_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionGaussianBlurEffect {
    type Vtable = IDCompositionGaussianBlurEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1171574967,
        7124,
        17742,
        [136, 148, 43, 250, 104, 68, 48, 51],
    );
}
impl ::std::convert::From<IDCompositionGaussianBlurEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionGaussianBlurEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionGaussianBlurEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionGaussianBlurEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionGaussianBlurEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionGaussianBlurEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionGaussianBlurEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionGaussianBlurEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionGaussianBlurEffect> for IDCompositionEffect {
    fn from(value: IDCompositionGaussianBlurEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionGaussianBlurEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionGaussianBlurEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for IDCompositionGaussianBlurEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionGaussianBlurEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionGaussianBlurEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: super::Direct2D::D2D1_BORDER_MODE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionHueRotationEffect(::windows::runtime::IUnknown);
impl IDCompositionHueRotationEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn SetAngle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAngle2(&self, amountdegrees: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(amountdegrees),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionHueRotationEffect {
    type Vtable = IDCompositionHueRotationEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1840904480,
        1904,
        18305,
        [176, 198, 56, 25, 18, 249, 209, 103],
    );
}
impl ::std::convert::From<IDCompositionHueRotationEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionHueRotationEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionHueRotationEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionHueRotationEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionHueRotationEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionHueRotationEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionHueRotationEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionHueRotationEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionHueRotationEffect> for IDCompositionEffect {
    fn from(value: IDCompositionHueRotationEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionHueRotationEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionHueRotationEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionHueRotationEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionHueRotationEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionHueRotationEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amountdegrees: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionInkTrailDevice(::windows::runtime::IUnknown);
impl IDCompositionInkTrailDevice {
    pub unsafe fn CreateDelegatedInkTrail(
        &self,
    ) -> ::windows::runtime::Result<IDCompositionDelegatedInkTrail> {
        let mut result__: <IDCompositionDelegatedInkTrail as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IDCompositionDelegatedInkTrail>(result__)
    }
    pub unsafe fn CreateDelegatedInkTrailForSwapChain<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        swapchain: Param0,
    ) -> ::windows::runtime::Result<IDCompositionDelegatedInkTrail> {
        let mut result__: <IDCompositionDelegatedInkTrail as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            swapchain.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IDCompositionDelegatedInkTrail>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionInkTrailDevice {
    type Vtable = IDCompositionInkTrailDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3742137580,
        52715,
        19786,
        [185, 28, 114, 27, 242, 47, 78, 108],
    );
}
impl ::std::convert::From<IDCompositionInkTrailDevice> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionInkTrailDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionInkTrailDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionInkTrailDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionInkTrailDevice
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionInkTrailDevice
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionInkTrailDevice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        inktrail: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        swapchain: ::windows::runtime::RawPtr,
        inktrail: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionLinearTransferEffect(::windows::runtime::IUnknown);
impl IDCompositionLinearTransferEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn SetRedYIntercept<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetRedYIntercept2(&self, redyintercept: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(redyintercept),
        )
        .ok()
    }
    pub unsafe fn SetRedSlope<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetRedSlope2(&self, redslope: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(redslope),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRedDisable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        reddisable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            reddisable.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetGreenYIntercept<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetGreenYIntercept2(
        &self,
        greenyintercept: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(greenyintercept),
        )
        .ok()
    }
    pub unsafe fn SetGreenSlope<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetGreenSlope2(&self, greenslope: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(greenslope),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGreenDisable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        greendisable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            greendisable.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBlueYIntercept<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBlueYIntercept2(&self, blueyintercept: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(blueyintercept),
        )
        .ok()
    }
    pub unsafe fn SetBlueSlope<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBlueSlope2(&self, blueslope: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(blueslope),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlueDisable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bluedisable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            bluedisable.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAlphaYIntercept<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAlphaYIntercept2(
        &self,
        alphayintercept: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(alphayintercept),
        )
        .ok()
    }
    pub unsafe fn SetAlphaSlope<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAlphaSlope2(&self, alphaslope: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(alphaslope),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlphaDisable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        alphadisable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            alphadisable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        clampoutput: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            clampoutput.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionLinearTransferEffect {
    type Vtable = IDCompositionLinearTransferEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1124462171,
        50336,
        19592,
        [147, 133, 103, 18, 78, 1, 118, 131],
    );
}
impl ::std::convert::From<IDCompositionLinearTransferEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionLinearTransferEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionLinearTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionLinearTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionLinearTransferEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionLinearTransferEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionLinearTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionLinearTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionLinearTransferEffect> for IDCompositionEffect {
    fn from(value: IDCompositionLinearTransferEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionLinearTransferEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionLinearTransferEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for IDCompositionLinearTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionLinearTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionLinearTransferEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        redyintercept: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        redslope: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reddisable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        greenyintercept: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        greenslope: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        greendisable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        blueyintercept: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        blueslope: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bluedisable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        alphayintercept: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        alphaslope: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        alphadisable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clampoutput: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionMatrixTransform(::windows::runtime::IUnknown);
impl IDCompositionMatrixTransform {
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetMatrix(
        &self,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(matrix),
        )
        .ok()
    }
    pub unsafe fn SetMatrixElement<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        row: i32,
        column: i32,
        animation: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(row),
            ::std::mem::transmute(column),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetMatrixElement2(
        &self,
        row: i32,
        column: i32,
        value: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(row),
            ::std::mem::transmute(column),
            ::std::mem::transmute(value),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionMatrixTransform {
    type Vtable = IDCompositionMatrixTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        382598919,
        50435,
        16796,
        [131, 242, 9, 101, 199, 175, 31, 166],
    );
}
impl ::std::convert::From<IDCompositionMatrixTransform> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionMatrixTransform> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionMatrixTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionMatrixTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionMatrixTransform> for IDCompositionTransform {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionMatrixTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform>
    for IDCompositionMatrixTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform>
    for &IDCompositionMatrixTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionMatrixTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionMatrixTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for IDCompositionMatrixTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for &IDCompositionMatrixTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionMatrixTransform> for IDCompositionEffect {
    fn from(value: IDCompositionMatrixTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionMatrixTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionMatrixTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionMatrixTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        row: i32,
        column: i32,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        row: i32,
        column: i32,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionMatrixTransform3D(::windows::runtime::IUnknown);
impl IDCompositionMatrixTransform3D {
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub unsafe fn SetMatrix(
        &self,
        matrix: *const super::Direct3D9::D3DMATRIX,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(matrix),
        )
        .ok()
    }
    pub unsafe fn SetMatrixElement<
        'a,
        Param2: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        row: i32,
        column: i32,
        animation: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(row),
            ::std::mem::transmute(column),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetMatrixElement2(
        &self,
        row: i32,
        column: i32,
        value: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(row),
            ::std::mem::transmute(column),
            ::std::mem::transmute(value),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionMatrixTransform3D {
    type Vtable = IDCompositionMatrixTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1261659120,
        25659,
        16823,
        [182, 224, 204, 242, 45, 52, 70, 124],
    );
}
impl ::std::convert::From<IDCompositionMatrixTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionMatrixTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionMatrixTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionMatrixTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionMatrixTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionMatrixTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for IDCompositionMatrixTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for &IDCompositionMatrixTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionMatrixTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionMatrixTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionMatrixTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionMatrixTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionMatrixTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionMatrixTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionMatrixTransform3D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D9")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrix: *const super::Direct3D9::D3DMATRIX,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D9"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        row: i32,
        column: i32,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        row: i32,
        column: i32,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionRectangleClip(::windows::runtime::IUnknown);
impl IDCompositionRectangleClip {
    pub unsafe fn SetLeft<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>>(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetLeft2(&self, left: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(left),
        )
        .ok()
    }
    pub unsafe fn SetTop<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>>(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTop2(&self, top: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(top),
        )
        .ok()
    }
    pub unsafe fn SetRight<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetRight2(&self, right: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(right),
        )
        .ok()
    }
    pub unsafe fn SetBottom<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBottom2(&self, bottom: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bottom),
        )
        .ok()
    }
    pub unsafe fn SetTopLeftRadiusX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTopLeftRadiusX2(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(radius),
        )
        .ok()
    }
    pub unsafe fn SetTopLeftRadiusY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTopLeftRadiusY2(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(radius),
        )
        .ok()
    }
    pub unsafe fn SetTopRightRadiusX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTopRightRadiusX2(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(radius),
        )
        .ok()
    }
    pub unsafe fn SetTopRightRadiusY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetTopRightRadiusY2(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(radius),
        )
        .ok()
    }
    pub unsafe fn SetBottomLeftRadiusX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBottomLeftRadiusX2(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(radius),
        )
        .ok()
    }
    pub unsafe fn SetBottomLeftRadiusY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBottomLeftRadiusY2(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(radius),
        )
        .ok()
    }
    pub unsafe fn SetBottomRightRadiusX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBottomRightRadiusX2(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(radius),
        )
        .ok()
    }
    pub unsafe fn SetBottomRightRadiusY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBottomRightRadiusY2(&self, radius: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(radius),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionRectangleClip {
    type Vtable = IDCompositionRectangleClip_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2554506621,
        55759,
        18696,
        [174, 215, 72, 181, 29, 165, 231, 194],
    );
}
impl ::std::convert::From<IDCompositionRectangleClip> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionRectangleClip) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionRectangleClip> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionRectangleClip) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionRectangleClip
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionRectangleClip
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionRectangleClip> for IDCompositionClip {
    fn from(value: IDCompositionRectangleClip) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionRectangleClip> for IDCompositionClip {
    fn from(value: &IDCompositionRectangleClip) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionClip> for IDCompositionRectangleClip {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionClip> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionClip>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionClip> for &IDCompositionRectangleClip {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionClip> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionClip>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRectangleClip_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        left: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        top: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        right: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bottom: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        radius: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionRotateTransform(::windows::runtime::IUnknown);
impl IDCompositionRotateTransform {
    pub unsafe fn SetAngle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(angle),
        )
        .ok()
    }
    pub unsafe fn SetCenterX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centerx),
        )
        .ok()
    }
    pub unsafe fn SetCenterY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centery),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionRotateTransform {
    type Vtable = IDCompositionRotateTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1679743036,
        44694,
        18117,
        [144, 220, 50, 119, 76, 197, 198, 213],
    );
}
impl ::std::convert::From<IDCompositionRotateTransform> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionRotateTransform> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionRotateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionRotateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionRotateTransform> for IDCompositionTransform {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionRotateTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform>
    for IDCompositionRotateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform>
    for &IDCompositionRotateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionRotateTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionRotateTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for IDCompositionRotateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for &IDCompositionRotateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionRotateTransform> for IDCompositionEffect {
    fn from(value: IDCompositionRotateTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionRotateTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionRotateTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionRotateTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        angle: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centerx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centery: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionRotateTransform3D(::windows::runtime::IUnknown);
impl IDCompositionRotateTransform3D {
    pub unsafe fn SetAngle<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAngle2(&self, angle: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(angle),
        )
        .ok()
    }
    pub unsafe fn SetAxisX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAxisX2(&self, axisx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(axisx),
        )
        .ok()
    }
    pub unsafe fn SetAxisY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAxisY2(&self, axisy: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(axisy),
        )
        .ok()
    }
    pub unsafe fn SetAxisZ<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAxisZ2(&self, axisz: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(axisz),
        )
        .ok()
    }
    pub unsafe fn SetCenterX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centerx),
        )
        .ok()
    }
    pub unsafe fn SetCenterY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centery),
        )
        .ok()
    }
    pub unsafe fn SetCenterZ<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centerz),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionRotateTransform3D {
    type Vtable = IDCompositionRotateTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3639980607,
        54313,
        19089,
        [181, 90, 210, 244, 95, 215, 91, 24],
    );
}
impl ::std::convert::From<IDCompositionRotateTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionRotateTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionRotateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionRotateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionRotateTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionRotateTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for IDCompositionRotateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for &IDCompositionRotateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionRotateTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionRotateTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionRotateTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionRotateTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionRotateTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionRotateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionRotateTransform3D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        angle: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        axisx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        axisy: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        axisz: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centerx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centery: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centerz: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionSaturationEffect(::windows::runtime::IUnknown);
impl IDCompositionSaturationEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn SetSaturation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetSaturation2(&self, ratio: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(ratio),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionSaturationEffect {
    type Vtable = IDCompositionSaturationEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2693655514,
        12888,
        20388,
        [159, 22, 145, 116, 211, 254, 147, 177],
    );
}
impl ::std::convert::From<IDCompositionSaturationEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionSaturationEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionSaturationEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionSaturationEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionSaturationEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionSaturationEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionSaturationEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionSaturationEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionSaturationEffect> for IDCompositionEffect {
    fn from(value: IDCompositionSaturationEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionSaturationEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionSaturationEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionSaturationEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSaturationEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        ratio: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionScaleTransform(::windows::runtime::IUnknown);
impl IDCompositionScaleTransform {
    pub unsafe fn SetScaleX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(scalex),
        )
        .ok()
    }
    pub unsafe fn SetScaleY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(scaley),
        )
        .ok()
    }
    pub unsafe fn SetCenterX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centerx),
        )
        .ok()
    }
    pub unsafe fn SetCenterY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centery),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionScaleTransform {
    type Vtable = IDCompositionScaleTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1912465684,
        16623,
        17903,
        [189, 81, 104, 176, 55, 195, 57, 249],
    );
}
impl ::std::convert::From<IDCompositionScaleTransform> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionScaleTransform> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionScaleTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionScaleTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionScaleTransform> for IDCompositionTransform {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionScaleTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform>
    for &IDCompositionScaleTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionScaleTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionScaleTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for IDCompositionScaleTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for &IDCompositionScaleTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionScaleTransform> for IDCompositionEffect {
    fn from(value: IDCompositionScaleTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionScaleTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionScaleTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionScaleTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scalex: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaley: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centerx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centery: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionScaleTransform3D(::windows::runtime::IUnknown);
impl IDCompositionScaleTransform3D {
    pub unsafe fn SetScaleX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetScaleX2(&self, scalex: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(scalex),
        )
        .ok()
    }
    pub unsafe fn SetScaleY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetScaleY2(&self, scaley: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(scaley),
        )
        .ok()
    }
    pub unsafe fn SetScaleZ<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetScaleZ2(&self, scalez: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(scalez),
        )
        .ok()
    }
    pub unsafe fn SetCenterX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centerx),
        )
        .ok()
    }
    pub unsafe fn SetCenterY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centery),
        )
        .ok()
    }
    pub unsafe fn SetCenterZ<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterZ2(&self, centerz: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centerz),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionScaleTransform3D {
    type Vtable = IDCompositionScaleTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        715038381,
        13899,
        19221,
        [167, 196, 161, 153, 127, 120, 179, 137],
    );
}
impl ::std::convert::From<IDCompositionScaleTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionScaleTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionScaleTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionScaleTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionScaleTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionScaleTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for IDCompositionScaleTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for &IDCompositionScaleTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionScaleTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionScaleTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionScaleTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionScaleTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionScaleTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionScaleTransform3D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scalex: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scaley: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scalez: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centerx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centery: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centerz: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionShadowEffect(::windows::runtime::IUnknown);
impl IDCompositionShadowEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn SetStandardDeviation<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetStandardDeviation2(&self, amount: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(amount),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetColor(
        &self,
        color: *const super::Direct2D::D2D_VECTOR_4F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(color),
        )
        .ok()
    }
    pub unsafe fn SetRed<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>>(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetRed2(&self, amount: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(amount),
        )
        .ok()
    }
    pub unsafe fn SetGreen<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetGreen2(&self, amount: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(amount),
        )
        .ok()
    }
    pub unsafe fn SetBlue<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>>(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBlue2(&self, amount: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(amount),
        )
        .ok()
    }
    pub unsafe fn SetAlpha<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAlpha2(&self, amount: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(amount),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionShadowEffect {
    type Vtable = IDCompositionShadowEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1255246528,
        53202,
        19503,
        [187, 98, 150, 229, 79, 219, 104, 121],
    );
}
impl ::std::convert::From<IDCompositionShadowEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionShadowEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionShadowEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionShadowEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionShadowEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionShadowEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionShadowEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionShadowEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionShadowEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionShadowEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionShadowEffect> for IDCompositionEffect {
    fn from(value: IDCompositionShadowEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionShadowEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionShadowEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionShadowEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionShadowEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: f32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        color: *const super::Direct2D::D2D_VECTOR_4F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        amount: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionSkewTransform(::windows::runtime::IUnknown);
impl IDCompositionSkewTransform {
    pub unsafe fn SetAngleX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAngleX2(&self, anglex: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(anglex),
        )
        .ok()
    }
    pub unsafe fn SetAngleY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAngleY2(&self, angley: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(angley),
        )
        .ok()
    }
    pub unsafe fn SetCenterX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterX2(&self, centerx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centerx),
        )
        .ok()
    }
    pub unsafe fn SetCenterY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetCenterY2(&self, centery: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(centery),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionSkewTransform {
    type Vtable = IDCompositionSkewTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3850020661,
        56539,
        19570,
        [156, 97, 5, 145, 245, 136, 137, 238],
    );
}
impl ::std::convert::From<IDCompositionSkewTransform> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionSkewTransform> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionSkewTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionSkewTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionSkewTransform> for IDCompositionTransform {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionSkewTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform> for &IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionSkewTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionSkewTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for IDCompositionSkewTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for &IDCompositionSkewTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionSkewTransform> for IDCompositionEffect {
    fn from(value: IDCompositionSkewTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionSkewTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionSkewTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionSkewTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSkewTransform_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        anglex: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        angley: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centerx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        centery: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionSurface(::windows::runtime::IUnknown);
impl IDCompositionSurface {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw(
        &self,
        updaterect: *const super::super::Foundation::RECT,
        iid: *const ::windows::runtime::GUID,
        updateobject: *mut *mut ::std::ffi::c_void,
        updateoffset: *mut super::super::Foundation::POINT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(updaterect),
            ::std::mem::transmute(iid),
            ::std::mem::transmute(updateobject),
            ::std::mem::transmute(updateoffset),
        )
        .ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(
        &self,
        scrollrect: *const super::super::Foundation::RECT,
        cliprect: *const super::super::Foundation::RECT,
        offsetx: i32,
        offsety: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(scrollrect),
            ::std::mem::transmute(cliprect),
            ::std::mem::transmute(offsetx),
            ::std::mem::transmute(offsety),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionSurface {
    type Vtable = IDCompositionSurface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3146402131,
        11417,
        20314,
        [150, 245, 72, 25, 2, 127, 163, 172],
    );
}
impl ::std::convert::From<IDCompositionSurface> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionSurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionSurface> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionSurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurface_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        updaterect: *const super::super::Foundation::RECT,
        iid: *const ::windows::runtime::GUID,
        updateobject: *mut *mut ::std::ffi::c_void,
        updateoffset: *mut super::super::Foundation::POINT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scrollrect: *const super::super::Foundation::RECT,
        cliprect: *const super::super::Foundation::RECT,
        offsetx: i32,
        offsety: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionSurfaceFactory(::windows::runtime::IUnknown);
impl IDCompositionSurfaceFactory {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateSurface(
        &self,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionSurface> {
        let mut result__: <IDCompositionSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionSurface>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub unsafe fn CreateVirtualSurface(
        &self,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
    ) -> ::windows::runtime::Result<IDCompositionVirtualSurface> {
        let mut result__: <IDCompositionVirtualSurface as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(initialwidth),
            ::std::mem::transmute(initialheight),
            ::std::mem::transmute(pixelformat),
            ::std::mem::transmute(alphamode),
            &mut result__,
        )
        .from_abi::<IDCompositionVirtualSurface>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionSurfaceFactory {
    type Vtable = IDCompositionSurfaceFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3811884050,
        14647,
        19970,
        [133, 235, 252, 244, 235, 48, 210, 200],
    );
}
impl ::std::convert::From<IDCompositionSurfaceFactory> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionSurfaceFactory) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionSurfaceFactory> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionSurfaceFactory) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionSurfaceFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionSurfaceFactory
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionSurfaceFactory_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        surface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        initialwidth: u32,
        initialheight: u32,
        pixelformat: super::Dxgi::DXGI_FORMAT,
        alphamode: super::Dxgi::DXGI_ALPHA_MODE,
        virtualsurface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionTableTransferEffect(::windows::runtime::IUnknown);
impl IDCompositionTableTransferEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    pub unsafe fn SetRedTable(
        &self,
        tablevalues: *const f32,
        count: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(tablevalues),
            ::std::mem::transmute(count),
        )
        .ok()
    }
    pub unsafe fn SetGreenTable(
        &self,
        tablevalues: *const f32,
        count: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(tablevalues),
            ::std::mem::transmute(count),
        )
        .ok()
    }
    pub unsafe fn SetBlueTable(
        &self,
        tablevalues: *const f32,
        count: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(tablevalues),
            ::std::mem::transmute(count),
        )
        .ok()
    }
    pub unsafe fn SetAlphaTable(
        &self,
        tablevalues: *const f32,
        count: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(tablevalues),
            ::std::mem::transmute(count),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetRedDisable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        reddisable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            reddisable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetGreenDisable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        greendisable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            greendisable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBlueDisable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        bluedisable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            bluedisable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAlphaDisable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        alphadisable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            alphadisable.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetClampOutput<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        clampoutput: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            clampoutput.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetRedTableValue<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        index: u32,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetRedTableValue2(
        &self,
        index: u32,
        value: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(value),
        )
        .ok()
    }
    pub unsafe fn SetGreenTableValue<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        index: u32,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetGreenTableValue2(
        &self,
        index: u32,
        value: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(value),
        )
        .ok()
    }
    pub unsafe fn SetBlueTableValue<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        index: u32,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBlueTableValue2(
        &self,
        index: u32,
        value: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(value),
        )
        .ok()
    }
    pub unsafe fn SetAlphaTableValue<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        index: u32,
        animation: Param1,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetAlphaTableValue2(
        &self,
        index: u32,
        value: f32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            ::std::mem::transmute(value),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionTableTransferEffect {
    type Vtable = IDCompositionTableTransferEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2608759522,
        27077,
        20148,
        [165, 245, 167, 3, 63, 81, 50, 205],
    );
}
impl ::std::convert::From<IDCompositionTableTransferEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTableTransferEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionTableTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionTableTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionTableTransferEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTableTransferEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionTableTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionTableTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionTableTransferEffect> for IDCompositionEffect {
    fn from(value: IDCompositionTableTransferEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTableTransferEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionTableTransferEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for IDCompositionTableTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionTableTransferEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTableTransferEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tablevalues: *const f32,
        count: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tablevalues: *const f32,
        count: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tablevalues: *const f32,
        count: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        tablevalues: *const f32,
        count: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        reddisable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        greendisable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bluedisable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        alphadisable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clampoutput: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionTarget(::windows::runtime::IUnknown);
impl IDCompositionTarget {
    pub unsafe fn SetRoot<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>>(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionTarget {
    type Vtable = IDCompositionTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3939356748,
        4478,
        19991,
        [136, 244, 209, 177, 43, 14, 61, 137],
    );
}
impl ::std::convert::From<IDCompositionTarget> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionTarget) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionTarget) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTarget_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionTransform(::windows::runtime::IUnknown);
impl IDCompositionTransform {}
unsafe impl ::windows::runtime::Interface for IDCompositionTransform {
    type Vtable = IDCompositionTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4250270375,
        14304,
        19488,
        [149, 210, 155, 228, 91, 195, 63, 85],
    );
}
impl ::std::convert::From<IDCompositionTransform> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTransform> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D> for IDCompositionTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D> for &IDCompositionTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionTransform> for IDCompositionEffect {
    fn from(value: IDCompositionTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionTransform {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionTransform3D(::windows::runtime::IUnknown);
impl IDCompositionTransform3D {}
unsafe impl ::windows::runtime::Interface for IDCompositionTransform3D {
    type Vtable = IDCompositionTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1897420578,
        9323,
        16882,
        [170, 209, 4, 67, 247, 244, 191, 194],
    );
}
impl ::std::convert::From<IDCompositionTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionTransform3D {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTransform3D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionTranslateTransform(::windows::runtime::IUnknown);
impl IDCompositionTranslateTransform {
    pub unsafe fn SetOffsetX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsetx),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsety),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionTranslateTransform {
    type Vtable = IDCompositionTranslateTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        108597538,
        50928,
        16765,
        [131, 35, 38, 158, 152, 127, 89, 84],
    );
}
impl ::std::convert::From<IDCompositionTranslateTransform> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTranslateTransform> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionTranslateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionTranslateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionTranslateTransform> for IDCompositionTransform {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTranslateTransform> for IDCompositionTransform {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform>
    for IDCompositionTranslateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform>
    for &IDCompositionTranslateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionTranslateTransform> for IDCompositionTransform3D {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTranslateTransform> for IDCompositionTransform3D {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for IDCompositionTranslateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for &IDCompositionTranslateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionTranslateTransform> for IDCompositionEffect {
    fn from(value: IDCompositionTranslateTransform) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTranslateTransform> for IDCompositionEffect {
    fn from(value: &IDCompositionTranslateTransform) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for IDCompositionTranslateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionTranslateTransform
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsetx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsety: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionTranslateTransform3D(::windows::runtime::IUnknown);
impl IDCompositionTranslateTransform3D {
    pub unsafe fn SetOffsetX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsetx),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsety),
        )
        .ok()
    }
    pub unsafe fn SetOffsetZ<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsetz),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionTranslateTransform3D {
    type Vtable = IDCompositionTranslateTransform3D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2439212363,
        39841,
        17714,
        [170, 247, 227, 52, 73, 148, 215, 136],
    );
}
impl ::std::convert::From<IDCompositionTranslateTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTranslateTransform3D> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionTranslateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionTranslateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionTranslateTransform3D> for IDCompositionTransform3D {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTranslateTransform3D> for IDCompositionTransform3D {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for IDCompositionTranslateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>
    for &IDCompositionTranslateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionTransform3D> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionTransform3D>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionTranslateTransform3D> for IDCompositionEffect {
    fn from(value: IDCompositionTranslateTransform3D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTranslateTransform3D> for IDCompositionEffect {
    fn from(value: &IDCompositionTranslateTransform3D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for IDCompositionTranslateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect>
    for &IDCompositionTranslateTransform3D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTranslateTransform3D_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsetx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsety: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsetz: f32,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionTurbulenceEffect(::windows::runtime::IUnknown);
impl IDCompositionTurbulenceEffect {
    pub unsafe fn SetInput<
        'a,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        index: u32,
        input: Param1,
        flags: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            input.into_param().abi(),
            ::std::mem::transmute(flags),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetOffset(
        &self,
        offset: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offset),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetBaseFrequency(
        &self,
        frequency: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(frequency),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetSize(
        &self,
        size: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(size),
        )
        .ok()
    }
    pub unsafe fn SetNumOctaves(&self, numoctaves: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(numoctaves),
        )
        .ok()
    }
    pub unsafe fn SetSeed(&self, seed: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(seed),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetNoise(
        &self,
        noise: super::Direct2D::D2D1_TURBULENCE_NOISE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(noise),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStitchable<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        stitchable: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            stitchable.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionTurbulenceEffect {
    type Vtable = IDCompositionTurbulenceEffect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2795854810,
        49308,
        18931,
        [145, 147, 164, 25, 34, 200, 151, 21],
    );
}
impl ::std::convert::From<IDCompositionTurbulenceEffect> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTurbulenceEffect> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionTurbulenceEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionTurbulenceEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionTurbulenceEffect> for IDCompositionFilterEffect {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTurbulenceEffect> for IDCompositionFilterEffect {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for IDCompositionTurbulenceEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionFilterEffect>
    for &IDCompositionTurbulenceEffect
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionFilterEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionFilterEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionTurbulenceEffect> for IDCompositionEffect {
    fn from(value: IDCompositionTurbulenceEffect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionTurbulenceEffect> for IDCompositionEffect {
    fn from(value: &IDCompositionTurbulenceEffect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionEffect> for &IDCompositionTurbulenceEffect {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionEffect> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionEffect>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionTurbulenceEffect_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: u32,
        input: ::windows::runtime::RawPtr,
        flags: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offset: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        frequency: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        size: *const super::Direct2D::D2D_VECTOR_2F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        numoctaves: u32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        seed: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        noise: super::Direct2D::D2D1_TURBULENCE_NOISE,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        stitchable: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionVirtualSurface(::windows::runtime::IUnknown);
impl IDCompositionVirtualSurface {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn BeginDraw(
        &self,
        updaterect: *const super::super::Foundation::RECT,
        iid: *const ::windows::runtime::GUID,
        updateobject: *mut *mut ::std::ffi::c_void,
        updateoffset: *mut super::super::Foundation::POINT,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(updaterect),
            ::std::mem::transmute(iid),
            ::std::mem::transmute(updateobject),
            ::std::mem::transmute(updateoffset),
        )
        .ok()
    }
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SuspendDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn ResumeDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Scroll(
        &self,
        scrollrect: *const super::super::Foundation::RECT,
        cliprect: *const super::super::Foundation::RECT,
        offsetx: i32,
        offsety: i32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(scrollrect),
            ::std::mem::transmute(cliprect),
            ::std::mem::transmute(offsetx),
            ::std::mem::transmute(offsety),
        )
        .ok()
    }
    pub unsafe fn Resize(&self, width: u32, height: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Trim(
        &self,
        rectangles: *const super::super::Foundation::RECT,
        count: u32,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rectangles),
            ::std::mem::transmute(count),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionVirtualSurface {
    type Vtable = IDCompositionVirtualSurface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2923895889,
        24403,
        18980,
        [141, 62, 208, 195, 156, 48, 179, 240],
    );
}
impl ::std::convert::From<IDCompositionVirtualSurface> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionVirtualSurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVirtualSurface> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionVirtualSurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionVirtualSurface
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionVirtualSurface
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionVirtualSurface> for IDCompositionSurface {
    fn from(value: IDCompositionVirtualSurface) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVirtualSurface> for IDCompositionSurface {
    fn from(value: &IDCompositionVirtualSurface) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionSurface> for IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionSurface> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionSurface>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionSurface> for &IDCompositionVirtualSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionSurface> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionSurface>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVirtualSurface_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        updaterect: *const super::super::Foundation::RECT,
        iid: *const ::windows::runtime::GUID,
        updateobject: *mut *mut ::std::ffi::c_void,
        updateoffset: *mut super::super::Foundation::POINT,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scrollrect: *const super::super::Foundation::RECT,
        cliprect: *const super::super::Foundation::RECT,
        offsetx: i32,
        offsety: i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        width: u32,
        height: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rectangles: *const super::super::Foundation::RECT,
        count: u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionVisual(::windows::runtime::IUnknown);
impl IDCompositionVisual {
    pub unsafe fn SetOffsetX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsetx),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsety),
        )
        .ok()
    }
    pub unsafe fn SetTransform<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionTransform>,
    >(
        &self,
        transform: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            transform.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(
        &self,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(matrix),
        )
        .ok()
    }
    pub unsafe fn SetTransformParent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetEffect<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionEffect>>(
        &self,
        effect: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            effect.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(
        &self,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interpolationmode),
        )
        .ok()
    }
    pub unsafe fn SetBorderMode(
        &self,
        bordermode: DCOMPOSITION_BORDER_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bordermode),
        )
        .ok()
    }
    pub unsafe fn SetClip<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionClip>>(
        &self,
        clip: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            clip.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetClip2(
        &self,
        rect: *const super::Direct2D::D2D_RECT_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rect),
        )
        .ok()
    }
    pub unsafe fn SetContent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        content: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            content.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param2: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
        insertabove: Param1,
        referencevisual: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
            insertabove.into_param().abi(),
            referencevisual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetCompositeMode(
        &self,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(compositemode),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionVisual {
    type Vtable = IDCompositionVisual_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1301480861,
        2427,
        18001,
        [154, 96, 240, 242, 81, 22, 226, 243],
    );
}
impl ::std::convert::From<IDCompositionVisual> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionVisual) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisual> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionVisual) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionVisual {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsetx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsety: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transform: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effect: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bordermode: DCOMPOSITION_BORDER_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clip: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rect: *const super::Direct2D::D2D_RECT_F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
        insertabove: super::super::Foundation::BOOL,
        referencevisual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionVisual2(::windows::runtime::IUnknown);
impl IDCompositionVisual2 {
    pub unsafe fn SetOffsetX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsetx),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsety),
        )
        .ok()
    }
    pub unsafe fn SetTransform<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionTransform>,
    >(
        &self,
        transform: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            transform.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(
        &self,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(matrix),
        )
        .ok()
    }
    pub unsafe fn SetTransformParent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetEffect<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionEffect>>(
        &self,
        effect: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            effect.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(
        &self,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interpolationmode),
        )
        .ok()
    }
    pub unsafe fn SetBorderMode(
        &self,
        bordermode: DCOMPOSITION_BORDER_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bordermode),
        )
        .ok()
    }
    pub unsafe fn SetClip<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionClip>>(
        &self,
        clip: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            clip.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetClip2(
        &self,
        rect: *const super::Direct2D::D2D_RECT_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rect),
        )
        .ok()
    }
    pub unsafe fn SetContent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        content: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            content.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param2: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
        insertabove: Param1,
        referencevisual: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
            insertabove.into_param().abi(),
            referencevisual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetCompositeMode(
        &self,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(compositemode),
        )
        .ok()
    }
    pub unsafe fn SetOpacityMode(
        &self,
        mode: DCOMPOSITION_OPACITY_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
    pub unsafe fn SetBackFaceVisibility(
        &self,
        visibility: DCOMPOSITION_BACKFACE_VISIBILITY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(visibility),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionVisual2 {
    type Vtable = IDCompositionVisual2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3906868793,
        17201,
        19238,
        [188, 95, 106, 50, 29, 52, 122, 133],
    );
}
impl ::std::convert::From<IDCompositionVisual2> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionVisual2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisual2> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionVisual2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionVisual2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionVisual2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionVisual2> for IDCompositionVisual {
    fn from(value: IDCompositionVisual2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisual2> for IDCompositionVisual {
    fn from(value: &IDCompositionVisual2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual> for IDCompositionVisual2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual> for &IDCompositionVisual2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual2_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsetx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsety: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transform: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effect: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bordermode: DCOMPOSITION_BORDER_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clip: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rect: *const super::Direct2D::D2D_RECT_F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
        insertabove: super::super::Foundation::BOOL,
        referencevisual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: DCOMPOSITION_OPACITY_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visibility: DCOMPOSITION_BACKFACE_VISIBILITY,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionVisual3(::windows::runtime::IUnknown);
impl IDCompositionVisual3 {
    pub unsafe fn SetOffsetX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsetx),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsety),
        )
        .ok()
    }
    pub unsafe fn SetTransform<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionTransform>,
    >(
        &self,
        transform: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            transform.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(
        &self,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(matrix),
        )
        .ok()
    }
    pub unsafe fn SetTransformParent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetEffect<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionEffect>>(
        &self,
        effect: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            effect.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(
        &self,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interpolationmode),
        )
        .ok()
    }
    pub unsafe fn SetBorderMode(
        &self,
        bordermode: DCOMPOSITION_BORDER_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bordermode),
        )
        .ok()
    }
    pub unsafe fn SetClip<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionClip>>(
        &self,
        clip: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            clip.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetClip2(
        &self,
        rect: *const super::Direct2D::D2D_RECT_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rect),
        )
        .ok()
    }
    pub unsafe fn SetContent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        content: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            content.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param2: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
        insertabove: Param1,
        referencevisual: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
            insertabove.into_param().abi(),
            referencevisual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetCompositeMode(
        &self,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(compositemode),
        )
        .ok()
    }
    pub unsafe fn SetOpacityMode(
        &self,
        mode: DCOMPOSITION_OPACITY_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
    pub unsafe fn SetBackFaceVisibility(
        &self,
        visibility: DCOMPOSITION_BACKFACE_VISIBILITY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(visibility),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn EnableHeatMap(
        &self,
        color: *const super::Direct2D::D2D1_COLOR_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(color),
        )
        .ok()
    }
    pub unsafe fn DisableHeatMap(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetDepthMode(
        &self,
        mode: DCOMPOSITION_DEPTH_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
    pub unsafe fn SetOffsetZ<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetZ2(&self, offsetz: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).28)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsetz),
        )
        .ok()
    }
    pub unsafe fn SetOpacity<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOpacity2(&self, opacity: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(opacity),
        )
        .ok()
    }
    pub unsafe fn SetTransform3<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionTransform3D>,
    >(
        &self,
        transform: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(
            ::std::mem::transmute_copy(self),
            transform.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetTransform4(
        &self,
        matrix: *const super::Direct2D::D2D_MATRIX_4X4_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(matrix),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVisible<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
    >(
        &self,
        visible: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(
            ::std::mem::transmute_copy(self),
            visible.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionVisual3 {
    type Vtable = IDCompositionVisual3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        662041698,
        46785,
        16405,
        [176, 190, 179, 231, 214, 164, 151, 109],
    );
}
impl ::std::convert::From<IDCompositionVisual3> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisual3> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IDCompositionVisual3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionVisual3> for IDCompositionVisualDebug {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisual3> for IDCompositionVisualDebug {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisualDebug> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisualDebug> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisualDebug>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisualDebug> for &IDCompositionVisual3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisualDebug> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisualDebug>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionVisual3> for IDCompositionVisual2 {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisual3> for IDCompositionVisual2 {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual2> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual2> for &IDCompositionVisual3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionVisual3> for IDCompositionVisual {
    fn from(value: IDCompositionVisual3) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisual3> for IDCompositionVisual {
    fn from(value: &IDCompositionVisual3) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual> for IDCompositionVisual3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual> for &IDCompositionVisual3 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisual3_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsetx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsety: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transform: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effect: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bordermode: DCOMPOSITION_BORDER_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clip: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rect: *const super::Direct2D::D2D_RECT_F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
        insertabove: super::super::Foundation::BOOL,
        referencevisual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: DCOMPOSITION_OPACITY_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visibility: DCOMPOSITION_BACKFACE_VISIBILITY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        color: *const super::Direct2D::D2D1_COLOR_F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: DCOMPOSITION_DEPTH_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsetz: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        opacity: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transform: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrix: *const super::Direct2D::D2D_MATRIX_4X4_F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visible: super::super::Foundation::BOOL,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IDCompositionVisualDebug(::windows::runtime::IUnknown);
impl IDCompositionVisualDebug {
    pub unsafe fn SetOffsetX<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetX2(&self, offsetx: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsetx),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionAnimation>,
    >(
        &self,
        animation: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(
            ::std::mem::transmute_copy(self),
            animation.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetOffsetY2(&self, offsety: f32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(offsety),
        )
        .ok()
    }
    pub unsafe fn SetTransform<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionTransform>,
    >(
        &self,
        transform: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            transform.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Foundation_Numerics")]
    pub unsafe fn SetTransform2(
        &self,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(matrix),
        )
        .ok()
    }
    pub unsafe fn SetTransformParent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetEffect<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionEffect>>(
        &self,
        effect: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            effect.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn SetBitmapInterpolationMode(
        &self,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(interpolationmode),
        )
        .ok()
    }
    pub unsafe fn SetBorderMode(
        &self,
        bordermode: DCOMPOSITION_BORDER_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(bordermode),
        )
        .ok()
    }
    pub unsafe fn SetClip<'a, Param0: ::windows::runtime::IntoParam<'a, IDCompositionClip>>(
        &self,
        clip: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            clip.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn SetClip2(
        &self,
        rect: *const super::Direct2D::D2D_RECT_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(rect),
        )
        .ok()
    }
    pub unsafe fn SetContent<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
    >(
        &self,
        content: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            content.into_param().abi(),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>,
        Param2: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
        insertabove: Param1,
        referencevisual: Param2,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
            insertabove.into_param().abi(),
            referencevisual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveVisual<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, IDCompositionVisual>,
    >(
        &self,
        visual: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            visual.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn RemoveAllVisuals(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetCompositeMode(
        &self,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(compositemode),
        )
        .ok()
    }
    pub unsafe fn SetOpacityMode(
        &self,
        mode: DCOMPOSITION_OPACITY_MODE,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(mode),
        )
        .ok()
    }
    pub unsafe fn SetBackFaceVisibility(
        &self,
        visibility: DCOMPOSITION_BACKFACE_VISIBILITY,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(visibility),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub unsafe fn EnableHeatMap(
        &self,
        color: *const super::Direct2D::D2D1_COLOR_F,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(color),
        )
        .ok()
    }
    pub unsafe fn DisableHeatMap(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn EnableRedrawRegions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn DisableRedrawRegions(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDCompositionVisualDebug {
    type Vtable = IDCompositionVisualDebug_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4275222536,
        24244,
        17312,
        [174, 163, 53, 246, 82, 128, 249, 27],
    );
}
impl ::std::convert::From<IDCompositionVisualDebug> for ::windows::runtime::IUnknown {
    fn from(value: IDCompositionVisualDebug) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisualDebug> for ::windows::runtime::IUnknown {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IDCompositionVisualDebug
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IDCompositionVisualDebug
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IDCompositionVisualDebug> for IDCompositionVisual2 {
    fn from(value: IDCompositionVisualDebug) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisualDebug> for IDCompositionVisual2 {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual2> for IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual2>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual2> for &IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual2> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual2>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
impl ::std::convert::From<IDCompositionVisualDebug> for IDCompositionVisual {
    fn from(value: IDCompositionVisualDebug) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDCompositionVisualDebug> for IDCompositionVisual {
    fn from(value: &IDCompositionVisualDebug) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual> for IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDCompositionVisual> for &IDCompositionVisualDebug {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDCompositionVisual> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDCompositionVisual>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDCompositionVisualDebug_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsetx: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        animation: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        offsety: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        transform: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        matrix: *const super::super::super::Foundation::Numerics::Matrix3x2,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        effect: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        interpolationmode: DCOMPOSITION_BITMAP_INTERPOLATION_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        bordermode: DCOMPOSITION_BORDER_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        clip: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        rect: *const super::Direct2D::D2D_RECT_F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        content: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
        insertabove: super::super::Foundation::BOOL,
        referencevisual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visual: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        compositemode: DCOMPOSITION_COMPOSITE_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        mode: DCOMPOSITION_OPACITY_MODE,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        visibility: DCOMPOSITION_BACKFACE_VISIBILITY,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        color: *const super::Direct2D::D2D1_COLOR_F,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
