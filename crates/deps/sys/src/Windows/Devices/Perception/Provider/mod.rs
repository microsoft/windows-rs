#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IKnownPerceptionFrameKindStatics(pub *mut ::core::ffi::c_void);
pub struct IPerceptionControlGroup(pub *mut ::core::ffi::c_void);
pub struct IPerceptionControlGroupFactory(pub *mut ::core::ffi::c_void);
pub struct IPerceptionCorrelation(pub *mut ::core::ffi::c_void);
pub struct IPerceptionCorrelationFactory(pub *mut ::core::ffi::c_void);
pub struct IPerceptionCorrelationGroup(pub *mut ::core::ffi::c_void);
pub struct IPerceptionCorrelationGroupFactory(pub *mut ::core::ffi::c_void);
pub struct IPerceptionFaceAuthenticationGroup(pub *mut ::core::ffi::c_void);
pub struct IPerceptionFaceAuthenticationGroupFactory(pub *mut ::core::ffi::c_void);
pub struct IPerceptionFrame(pub *mut ::core::ffi::c_void);
pub struct IPerceptionFrameProvider(pub *mut ::core::ffi::c_void);
pub struct IPerceptionFrameProviderInfo(pub *mut ::core::ffi::c_void);
pub struct IPerceptionFrameProviderManager(pub *mut ::core::ffi::c_void);
pub struct IPerceptionFrameProviderManagerServiceStatics(pub *mut ::core::ffi::c_void);
pub struct IPerceptionPropertyChangeRequest(pub *mut ::core::ffi::c_void);
pub struct IPerceptionVideoFrameAllocator(pub *mut ::core::ffi::c_void);
pub struct IPerceptionVideoFrameAllocatorFactory(pub *mut ::core::ffi::c_void);
pub struct KnownPerceptionFrameKind(i32);
pub struct PerceptionControlGroup(i32);
pub struct PerceptionCorrelation(i32);
pub struct PerceptionCorrelationGroup(i32);
pub struct PerceptionFaceAuthenticationGroup(i32);
pub struct PerceptionFrame(i32);
pub struct PerceptionFrameProviderInfo(i32);
pub struct PerceptionFrameProviderManagerService(i32);
pub struct PerceptionPropertyChangeRequest(i32);
pub struct PerceptionStartFaceAuthenticationHandler(pub *mut ::core::ffi::c_void);
pub struct PerceptionStopFaceAuthenticationHandler(pub *mut ::core::ffi::c_void);
pub struct PerceptionVideoFrameAllocator(i32);
