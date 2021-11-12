#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IKnownPerceptionFrameKindStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionControlGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionControlGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionCorrelation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionCorrelationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionCorrelationGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionCorrelationGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionFaceAuthenticationGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionFaceAuthenticationGroupFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionFrameProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionFrameProviderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionFrameProviderManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionFrameProviderManagerServiceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionPropertyChangeRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionVideoFrameAllocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionVideoFrameAllocatorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionControlGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionCorrelation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionCorrelationGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionFaceAuthenticationGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionFrameProviderInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionPropertyChangeRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionStartFaceAuthenticationHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionStopFaceAuthenticationHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionVideoFrameAllocator(pub *mut ::core::ffi::c_void);
