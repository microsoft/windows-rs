#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AllJoynAboutData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynAboutDataView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynAcceptSessionJoinerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynAuthenticationCompleteEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynAuthenticationMechanism(pub i32);
impl AllJoynAuthenticationMechanism {
    pub const None: Self = Self(0i32);
    pub const SrpAnonymous: Self = Self(1i32);
    pub const SrpLogon: Self = Self(2i32);
    pub const EcdheNull: Self = Self(3i32);
    pub const EcdhePsk: Self = Self(4i32);
    pub const EcdheEcdsa: Self = Self(5i32);
    pub const EcdheSpeke: Self = Self(6i32);
}
impl ::core::marker::Copy for AllJoynAuthenticationMechanism {}
impl ::core::clone::Clone for AllJoynAuthenticationMechanism {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynBusAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynBusAttachmentState(pub i32);
impl AllJoynBusAttachmentState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connecting: Self = Self(1i32);
    pub const Connected: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
}
impl ::core::marker::Copy for AllJoynBusAttachmentState {}
impl ::core::clone::Clone for AllJoynBusAttachmentState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynBusAttachmentStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynBusObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynBusObjectStoppedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynCredentials(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynCredentialsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynCredentialsVerificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynMessageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynProducerStoppedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynServiceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynServiceInfoRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynSessionJoinedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynSessionLostEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynSessionLostReason(pub i32);
impl AllJoynSessionLostReason {
    pub const None: Self = Self(0i32);
    pub const ProducerLeftSession: Self = Self(1i32);
    pub const ProducerClosedAbruptly: Self = Self(2i32);
    pub const RemovedByProducer: Self = Self(3i32);
    pub const LinkTimeout: Self = Self(4i32);
    pub const Other: Self = Self(5i32);
}
impl ::core::marker::Copy for AllJoynSessionLostReason {}
impl ::core::clone::Clone for AllJoynSessionLostReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynSessionMemberAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynSessionMemberRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynTrafficType(pub i32);
impl AllJoynTrafficType {
    pub const Unknown: Self = Self(0i32);
    pub const Messages: Self = Self(1i32);
    pub const RawUnreliable: Self = Self(2i32);
    pub const RawReliable: Self = Self(4i32);
}
impl ::core::marker::Copy for AllJoynTrafficType {}
impl ::core::clone::Clone for AllJoynTrafficType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynWatcherStoppedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynAboutData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynAboutDataView(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynAboutDataViewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynAcceptSessionJoiner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynAcceptSessionJoinerEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynAcceptSessionJoinerEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynAuthenticationCompleteEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynBusAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynBusAttachment2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynBusAttachmentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynBusAttachmentStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynBusAttachmentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynBusObject(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynBusObjectFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynBusObjectStoppedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynBusObjectStoppedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynCredentials(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynCredentialsRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynCredentialsVerificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynMessageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynMessageInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynProducer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynProducerStoppedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynProducerStoppedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynServiceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynServiceInfoFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynServiceInfoRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynServiceInfoRemovedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynServiceInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSessionJoinedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSessionJoinedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSessionLostEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSessionLostEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSessionMemberAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSessionMemberAddedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSessionMemberRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSessionMemberRemovedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynSessionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynStatusStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynWatcherStoppedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAllJoynWatcherStoppedEventArgsFactory(pub *mut ::core::ffi::c_void);
