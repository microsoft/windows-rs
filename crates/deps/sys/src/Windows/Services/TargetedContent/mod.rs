#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ITargetedContentAction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentContainer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentContainerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentImage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentItemState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentSubscription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentSubscriptionOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentSubscriptionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITargetedContentValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentAction(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TargetedContentAppInstallationState(i32);
#[repr(C)]
pub struct TargetedContentAvailability(i32);
#[repr(transparent)]
pub struct TargetedContentAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentContainer(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TargetedContentContract(i32);
#[repr(transparent)]
pub struct TargetedContentFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentImage(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TargetedContentInteraction(i32);
#[repr(transparent)]
pub struct TargetedContentItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentItemState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentObject(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TargetedContentObjectKind(i32);
#[repr(transparent)]
pub struct TargetedContentStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentSubscription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentSubscriptionOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentValue(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TargetedContentValueKind(i32);
