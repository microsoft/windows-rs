#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Perception_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IKnownCameraIntrinsicsPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownCameraIntrinsicsPropertiesStatics {}
impl ::core::clone::Clone for IKnownCameraIntrinsicsPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownPerceptionColorFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownPerceptionColorFrameSourcePropertiesStatics {}
impl ::core::clone::Clone for IKnownPerceptionColorFrameSourcePropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownPerceptionDepthFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownPerceptionDepthFrameSourcePropertiesStatics {}
impl ::core::clone::Clone for IKnownPerceptionDepthFrameSourcePropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownPerceptionFrameSourcePropertiesStatics {}
impl ::core::clone::Clone for IKnownPerceptionFrameSourcePropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownPerceptionFrameSourcePropertiesStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownPerceptionFrameSourcePropertiesStatics2 {}
impl ::core::clone::Clone for IKnownPerceptionFrameSourcePropertiesStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownPerceptionInfraredFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownPerceptionInfraredFrameSourcePropertiesStatics {}
impl ::core::clone::Clone for IKnownPerceptionInfraredFrameSourcePropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownPerceptionVideoFrameSourcePropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownPerceptionVideoFrameSourcePropertiesStatics {}
impl ::core::clone::Clone for IKnownPerceptionVideoFrameSourcePropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownPerceptionVideoProfilePropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownPerceptionVideoProfilePropertiesStatics {}
impl ::core::clone::Clone for IKnownPerceptionVideoProfilePropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionColorFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionColorFrame {}
impl ::core::clone::Clone for IPerceptionColorFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionColorFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionColorFrameArrivedEventArgs {}
impl ::core::clone::Clone for IPerceptionColorFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionColorFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionColorFrameReader {}
impl ::core::clone::Clone for IPerceptionColorFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionColorFrameSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionColorFrameSource {}
impl ::core::clone::Clone for IPerceptionColorFrameSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionColorFrameSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionColorFrameSource2 {}
impl ::core::clone::Clone for IPerceptionColorFrameSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionColorFrameSourceAddedEventArgs {}
impl ::core::clone::Clone for IPerceptionColorFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionColorFrameSourceRemovedEventArgs {}
impl ::core::clone::Clone for IPerceptionColorFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionColorFrameSourceStatics {}
impl ::core::clone::Clone for IPerceptionColorFrameSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionColorFrameSourceWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionColorFrameSourceWatcher {}
impl ::core::clone::Clone for IPerceptionColorFrameSourceWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionControlSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionControlSession {}
impl ::core::clone::Clone for IPerceptionControlSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthCorrelatedCameraIntrinsics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthCorrelatedCameraIntrinsics {}
impl ::core::clone::Clone for IPerceptionDepthCorrelatedCameraIntrinsics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthCorrelatedCoordinateMapper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthCorrelatedCoordinateMapper {}
impl ::core::clone::Clone for IPerceptionDepthCorrelatedCoordinateMapper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthFrame {}
impl ::core::clone::Clone for IPerceptionDepthFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthFrameArrivedEventArgs {}
impl ::core::clone::Clone for IPerceptionDepthFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthFrameReader {}
impl ::core::clone::Clone for IPerceptionDepthFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthFrameSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthFrameSource {}
impl ::core::clone::Clone for IPerceptionDepthFrameSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthFrameSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthFrameSource2 {}
impl ::core::clone::Clone for IPerceptionDepthFrameSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthFrameSourceAddedEventArgs {}
impl ::core::clone::Clone for IPerceptionDepthFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthFrameSourceRemovedEventArgs {}
impl ::core::clone::Clone for IPerceptionDepthFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthFrameSourceStatics {}
impl ::core::clone::Clone for IPerceptionDepthFrameSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionDepthFrameSourceWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionDepthFrameSourceWatcher {}
impl ::core::clone::Clone for IPerceptionDepthFrameSourceWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionFrameSourcePropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionFrameSourcePropertiesChangedEventArgs {}
impl ::core::clone::Clone for IPerceptionFrameSourcePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionFrameSourcePropertyChangeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionFrameSourcePropertyChangeResult {}
impl ::core::clone::Clone for IPerceptionFrameSourcePropertyChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionInfraredFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionInfraredFrame {}
impl ::core::clone::Clone for IPerceptionInfraredFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionInfraredFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionInfraredFrameArrivedEventArgs {}
impl ::core::clone::Clone for IPerceptionInfraredFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionInfraredFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionInfraredFrameReader {}
impl ::core::clone::Clone for IPerceptionInfraredFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionInfraredFrameSource {}
impl ::core::clone::Clone for IPerceptionInfraredFrameSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionInfraredFrameSource2 {}
impl ::core::clone::Clone for IPerceptionInfraredFrameSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionInfraredFrameSourceAddedEventArgs {}
impl ::core::clone::Clone for IPerceptionInfraredFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionInfraredFrameSourceRemovedEventArgs {}
impl ::core::clone::Clone for IPerceptionInfraredFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionInfraredFrameSourceStatics {}
impl ::core::clone::Clone for IPerceptionInfraredFrameSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionInfraredFrameSourceWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionInfraredFrameSourceWatcher {}
impl ::core::clone::Clone for IPerceptionInfraredFrameSourceWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPerceptionVideoProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPerceptionVideoProfile {}
impl ::core::clone::Clone for IPerceptionVideoProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionColorFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionColorFrame {}
impl ::core::clone::Clone for PerceptionColorFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionColorFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionColorFrameArrivedEventArgs {}
impl ::core::clone::Clone for PerceptionColorFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionColorFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionColorFrameReader {}
impl ::core::clone::Clone for PerceptionColorFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionColorFrameSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionColorFrameSource {}
impl ::core::clone::Clone for PerceptionColorFrameSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionColorFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionColorFrameSourceAddedEventArgs {}
impl ::core::clone::Clone for PerceptionColorFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionColorFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionColorFrameSourceRemovedEventArgs {}
impl ::core::clone::Clone for PerceptionColorFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionColorFrameSourceWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionColorFrameSourceWatcher {}
impl ::core::clone::Clone for PerceptionColorFrameSourceWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionControlSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionControlSession {}
impl ::core::clone::Clone for PerceptionControlSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionDepthCorrelatedCameraIntrinsics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionDepthCorrelatedCameraIntrinsics {}
impl ::core::clone::Clone for PerceptionDepthCorrelatedCameraIntrinsics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionDepthCorrelatedCoordinateMapper(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionDepthCorrelatedCoordinateMapper {}
impl ::core::clone::Clone for PerceptionDepthCorrelatedCoordinateMapper {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionDepthFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionDepthFrame {}
impl ::core::clone::Clone for PerceptionDepthFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionDepthFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionDepthFrameArrivedEventArgs {}
impl ::core::clone::Clone for PerceptionDepthFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionDepthFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionDepthFrameReader {}
impl ::core::clone::Clone for PerceptionDepthFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionDepthFrameSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionDepthFrameSource {}
impl ::core::clone::Clone for PerceptionDepthFrameSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionDepthFrameSourceAddedEventArgs {}
impl ::core::clone::Clone for PerceptionDepthFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionDepthFrameSourceRemovedEventArgs {}
impl ::core::clone::Clone for PerceptionDepthFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionDepthFrameSourceWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionDepthFrameSourceWatcher {}
impl ::core::clone::Clone for PerceptionDepthFrameSourceWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionFrameSourceAccessStatus(pub i32);
impl PerceptionFrameSourceAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const DeniedByUser: Self = Self(2i32);
    pub const DeniedBySystem: Self = Self(3i32);
}
impl ::core::marker::Copy for PerceptionFrameSourceAccessStatus {}
impl ::core::clone::Clone for PerceptionFrameSourceAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionFrameSourcePropertiesChangedEventArgs {}
impl ::core::clone::Clone for PerceptionFrameSourcePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionFrameSourcePropertyChangeResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionFrameSourcePropertyChangeResult {}
impl ::core::clone::Clone for PerceptionFrameSourcePropertyChangeResult {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PerceptionFrameSourcePropertyChangeStatus {}
impl ::core::clone::Clone for PerceptionFrameSourcePropertyChangeStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionInfraredFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionInfraredFrame {}
impl ::core::clone::Clone for PerceptionInfraredFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionInfraredFrameArrivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionInfraredFrameArrivedEventArgs {}
impl ::core::clone::Clone for PerceptionInfraredFrameArrivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionInfraredFrameReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionInfraredFrameReader {}
impl ::core::clone::Clone for PerceptionInfraredFrameReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionInfraredFrameSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionInfraredFrameSource {}
impl ::core::clone::Clone for PerceptionInfraredFrameSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionInfraredFrameSourceAddedEventArgs {}
impl ::core::clone::Clone for PerceptionInfraredFrameSourceAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionInfraredFrameSourceRemovedEventArgs {}
impl ::core::clone::Clone for PerceptionInfraredFrameSourceRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionInfraredFrameSourceWatcher(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionInfraredFrameSourceWatcher {}
impl ::core::clone::Clone for PerceptionInfraredFrameSourceWatcher {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PerceptionVideoProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PerceptionVideoProfile {}
impl ::core::clone::Clone for PerceptionVideoProfile {
    fn clone(&self) -> Self {
        *self
    }
}
