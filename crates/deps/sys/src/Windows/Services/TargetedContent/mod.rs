#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct TargetedContentAppInstallationState(pub i32);
impl TargetedContentAppInstallationState {
    pub const NotApplicable: TargetedContentAppInstallationState = TargetedContentAppInstallationState(0i32);
    pub const NotInstalled: TargetedContentAppInstallationState = TargetedContentAppInstallationState(1i32);
    pub const Installed: TargetedContentAppInstallationState = TargetedContentAppInstallationState(2i32);
}
#[repr(transparent)]
pub struct TargetedContentAvailability(pub i32);
impl TargetedContentAvailability {
    pub const None: TargetedContentAvailability = TargetedContentAvailability(0i32);
    pub const Partial: TargetedContentAvailability = TargetedContentAvailability(1i32);
    pub const All: TargetedContentAvailability = TargetedContentAvailability(2i32);
}
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
#[repr(transparent)]
pub struct TargetedContentInteraction(pub i32);
impl TargetedContentInteraction {
    pub const Impression: TargetedContentInteraction = TargetedContentInteraction(0i32);
    pub const ClickThrough: TargetedContentInteraction = TargetedContentInteraction(1i32);
    pub const Hover: TargetedContentInteraction = TargetedContentInteraction(2i32);
    pub const Like: TargetedContentInteraction = TargetedContentInteraction(3i32);
    pub const Dislike: TargetedContentInteraction = TargetedContentInteraction(4i32);
    pub const Dismiss: TargetedContentInteraction = TargetedContentInteraction(5i32);
    pub const Ineligible: TargetedContentInteraction = TargetedContentInteraction(6i32);
    pub const Accept: TargetedContentInteraction = TargetedContentInteraction(7i32);
    pub const Decline: TargetedContentInteraction = TargetedContentInteraction(8i32);
    pub const Defer: TargetedContentInteraction = TargetedContentInteraction(9i32);
    pub const Canceled: TargetedContentInteraction = TargetedContentInteraction(10i32);
    pub const Conversion: TargetedContentInteraction = TargetedContentInteraction(11i32);
    pub const Opportunity: TargetedContentInteraction = TargetedContentInteraction(12i32);
}
#[repr(transparent)]
pub struct TargetedContentItem(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentItemState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentObjectKind(pub i32);
impl TargetedContentObjectKind {
    pub const Collection: TargetedContentObjectKind = TargetedContentObjectKind(0i32);
    pub const Item: TargetedContentObjectKind = TargetedContentObjectKind(1i32);
    pub const Value: TargetedContentObjectKind = TargetedContentObjectKind(2i32);
}
#[repr(transparent)]
pub struct TargetedContentStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentSubscription(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentSubscriptionOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentValue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TargetedContentValueKind(pub i32);
impl TargetedContentValueKind {
    pub const String: TargetedContentValueKind = TargetedContentValueKind(0i32);
    pub const Uri: TargetedContentValueKind = TargetedContentValueKind(1i32);
    pub const Number: TargetedContentValueKind = TargetedContentValueKind(2i32);
    pub const Boolean: TargetedContentValueKind = TargetedContentValueKind(3i32);
    pub const File: TargetedContentValueKind = TargetedContentValueKind(4i32);
    pub const ImageFile: TargetedContentValueKind = TargetedContentValueKind(5i32);
    pub const Action: TargetedContentValueKind = TargetedContentValueKind(6i32);
    pub const Strings: TargetedContentValueKind = TargetedContentValueKind(7i32);
    pub const Uris: TargetedContentValueKind = TargetedContentValueKind(8i32);
    pub const Numbers: TargetedContentValueKind = TargetedContentValueKind(9i32);
    pub const Booleans: TargetedContentValueKind = TargetedContentValueKind(10i32);
    pub const Files: TargetedContentValueKind = TargetedContentValueKind(11i32);
    pub const ImageFiles: TargetedContentValueKind = TargetedContentValueKind(12i32);
    pub const Actions: TargetedContentValueKind = TargetedContentValueKind(13i32);
}
