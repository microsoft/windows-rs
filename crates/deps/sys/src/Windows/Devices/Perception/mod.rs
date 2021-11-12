#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Perception_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IKnownCameraIntrinsicsPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownPerceptionColorFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownPerceptionDepthFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownPerceptionInfraredFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownPerceptionVideoFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownPerceptionVideoProfilePropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionColorFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionColorFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionColorFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionColorFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionColorFrameSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionControlSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthCorrelatedCameraIntrinsics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthCorrelatedCoordinateMapper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthFrameSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionFrameSourcePropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionFrameSourcePropertyChangeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionInfraredFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionInfraredFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionInfraredFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionVideoProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionColorFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionColorFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionColorFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionColorFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionColorFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionColorFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionColorFrameSourceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionControlSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionDepthCorrelatedCameraIntrinsics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionDepthCorrelatedCoordinateMapper(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionDepthFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionDepthFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionDepthFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionDepthFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionFrameSourceAccessStatus(pub i32);
impl PerceptionFrameSourceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertyChangeResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertyChangeStatus(pub i32);
impl PerceptionFrameSourcePropertyChangeStatus {
    pub const Unknown: Self = Self(0i32);
    pub const Accepted: Self = Self(1i32);
    pub const LostControl: Self = Self(2i32);
    pub const PropertyNotSupported: Self = Self(3i32);
    pub const PropertyReadOnly: Self = Self(4i32);
    pub const ValueOutOfRange: Self = Self(5i32);
}
#[repr(transparent)]
pub struct PerceptionInfraredFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionInfraredFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionInfraredFrameReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionInfraredFrameSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceWatcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionVideoProfile(pub *mut ::core::ffi::c_void);
