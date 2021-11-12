#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CompositionDebugHeatMaps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CompositionDebugOverdrawContentKinds(pub u32);
impl CompositionDebugOverdrawContentKinds {
    pub const None: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(0u32);
    pub const OffscreenRendered: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(1u32);
    pub const Colors: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(2u32);
    pub const Effects: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(4u32);
    pub const Shadows: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(8u32);
    pub const Lights: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(16u32);
    pub const Surfaces: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(32u32);
    pub const SwapChains: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(64u32);
    pub const All: CompositionDebugOverdrawContentKinds = CompositionDebugOverdrawContentKinds(4294967295u32);
}
#[repr(transparent)]
pub struct CompositionDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDebugHeatMaps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDebugSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositionDebugSettingsStatics(pub *mut ::core::ffi::c_void);
