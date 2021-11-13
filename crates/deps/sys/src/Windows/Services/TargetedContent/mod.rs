#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ITargetedContentAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentAction {}
impl ::core::clone::Clone for ITargetedContentAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentAvailabilityChangedEventArgs {}
impl ::core::clone::Clone for ITargetedContentAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentChangedEventArgs {}
impl ::core::clone::Clone for ITargetedContentChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentCollection {}
impl ::core::clone::Clone for ITargetedContentCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentContainer {}
impl ::core::clone::Clone for ITargetedContentContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentContainerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentContainerStatics {}
impl ::core::clone::Clone for ITargetedContentContainerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentImage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentImage {}
impl ::core::clone::Clone for ITargetedContentImage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentItem {}
impl ::core::clone::Clone for ITargetedContentItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentItemState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentItemState {}
impl ::core::clone::Clone for ITargetedContentItemState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentObject {}
impl ::core::clone::Clone for ITargetedContentObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentStateChangedEventArgs {}
impl ::core::clone::Clone for ITargetedContentStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentSubscription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentSubscription {}
impl ::core::clone::Clone for ITargetedContentSubscription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentSubscriptionOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentSubscriptionOptions {}
impl ::core::clone::Clone for ITargetedContentSubscriptionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentSubscriptionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentSubscriptionStatics {}
impl ::core::clone::Clone for ITargetedContentSubscriptionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITargetedContentValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITargetedContentValue {}
impl ::core::clone::Clone for ITargetedContentValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentAction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentAction {}
impl ::core::clone::Clone for TargetedContentAction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentAppInstallationState(pub i32);
impl TargetedContentAppInstallationState {
    pub const NotApplicable: Self = Self(0i32);
    pub const NotInstalled: Self = Self(1i32);
    pub const Installed: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAppInstallationState {}
impl ::core::clone::Clone for TargetedContentAppInstallationState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentAvailability(pub i32);
impl TargetedContentAvailability {
    pub const None: Self = Self(0i32);
    pub const Partial: Self = Self(1i32);
    pub const All: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentAvailability {}
impl ::core::clone::Clone for TargetedContentAvailability {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentAvailabilityChangedEventArgs {}
impl ::core::clone::Clone for TargetedContentAvailabilityChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentChangedEventArgs {}
impl ::core::clone::Clone for TargetedContentChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentCollection {}
impl ::core::clone::Clone for TargetedContentCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentContainer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentContainer {}
impl ::core::clone::Clone for TargetedContentContainer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentFile {}
impl ::core::clone::Clone for TargetedContentFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentImage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentImage {}
impl ::core::clone::Clone for TargetedContentImage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentInteraction(pub i32);
impl TargetedContentInteraction {
    pub const Impression: Self = Self(0i32);
    pub const ClickThrough: Self = Self(1i32);
    pub const Hover: Self = Self(2i32);
    pub const Like: Self = Self(3i32);
    pub const Dislike: Self = Self(4i32);
    pub const Dismiss: Self = Self(5i32);
    pub const Ineligible: Self = Self(6i32);
    pub const Accept: Self = Self(7i32);
    pub const Decline: Self = Self(8i32);
    pub const Defer: Self = Self(9i32);
    pub const Canceled: Self = Self(10i32);
    pub const Conversion: Self = Self(11i32);
    pub const Opportunity: Self = Self(12i32);
}
impl ::core::marker::Copy for TargetedContentInteraction {}
impl ::core::clone::Clone for TargetedContentInteraction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentItem(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentItem {}
impl ::core::clone::Clone for TargetedContentItem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentItemState(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentItemState {}
impl ::core::clone::Clone for TargetedContentItemState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentObject {}
impl ::core::clone::Clone for TargetedContentObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentObjectKind(pub i32);
impl TargetedContentObjectKind {
    pub const Collection: Self = Self(0i32);
    pub const Item: Self = Self(1i32);
    pub const Value: Self = Self(2i32);
}
impl ::core::marker::Copy for TargetedContentObjectKind {}
impl ::core::clone::Clone for TargetedContentObjectKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentStateChangedEventArgs {}
impl ::core::clone::Clone for TargetedContentStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentSubscription(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentSubscription {}
impl ::core::clone::Clone for TargetedContentSubscription {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentSubscriptionOptions(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentSubscriptionOptions {}
impl ::core::clone::Clone for TargetedContentSubscriptionOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentValue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TargetedContentValue {}
impl ::core::clone::Clone for TargetedContentValue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TargetedContentValueKind(pub i32);
impl TargetedContentValueKind {
    pub const String: Self = Self(0i32);
    pub const Uri: Self = Self(1i32);
    pub const Number: Self = Self(2i32);
    pub const Boolean: Self = Self(3i32);
    pub const File: Self = Self(4i32);
    pub const ImageFile: Self = Self(5i32);
    pub const Action: Self = Self(6i32);
    pub const Strings: Self = Self(7i32);
    pub const Uris: Self = Self(8i32);
    pub const Numbers: Self = Self(9i32);
    pub const Booleans: Self = Self(10i32);
    pub const Files: Self = Self(11i32);
    pub const ImageFiles: Self = Self(12i32);
    pub const Actions: Self = Self(13i32);
}
impl ::core::marker::Copy for TargetedContentValueKind {}
impl ::core::clone::Clone for TargetedContentValueKind {
    fn clone(&self) -> Self {
        *self
    }
}
