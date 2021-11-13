#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AllJoynAboutData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynAboutData {}
impl ::core::clone::Clone for AllJoynAboutData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynAboutDataView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynAboutDataView {}
impl ::core::clone::Clone for AllJoynAboutDataView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynAcceptSessionJoinerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynAcceptSessionJoinerEventArgs {}
impl ::core::clone::Clone for AllJoynAcceptSessionJoinerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynAuthenticationCompleteEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynAuthenticationCompleteEventArgs {}
impl ::core::clone::Clone for AllJoynAuthenticationCompleteEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AllJoynBusAttachment {}
impl ::core::clone::Clone for AllJoynBusAttachment {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AllJoynBusAttachmentStateChangedEventArgs {}
impl ::core::clone::Clone for AllJoynBusAttachmentStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynBusObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynBusObject {}
impl ::core::clone::Clone for AllJoynBusObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynBusObjectStoppedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynBusObjectStoppedEventArgs {}
impl ::core::clone::Clone for AllJoynBusObjectStoppedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynCredentials(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynCredentials {}
impl ::core::clone::Clone for AllJoynCredentials {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynCredentialsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynCredentialsRequestedEventArgs {}
impl ::core::clone::Clone for AllJoynCredentialsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynCredentialsVerificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynCredentialsVerificationRequestedEventArgs {}
impl ::core::clone::Clone for AllJoynCredentialsVerificationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynMessageInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynMessageInfo {}
impl ::core::clone::Clone for AllJoynMessageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynProducerStoppedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynProducerStoppedEventArgs {}
impl ::core::clone::Clone for AllJoynProducerStoppedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynServiceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynServiceInfo {}
impl ::core::clone::Clone for AllJoynServiceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynServiceInfoRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynServiceInfoRemovedEventArgs {}
impl ::core::clone::Clone for AllJoynServiceInfoRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynSession {}
impl ::core::clone::Clone for AllJoynSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynSessionJoinedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynSessionJoinedEventArgs {}
impl ::core::clone::Clone for AllJoynSessionJoinedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynSessionLostEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynSessionLostEventArgs {}
impl ::core::clone::Clone for AllJoynSessionLostEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AllJoynSessionMemberAddedEventArgs {}
impl ::core::clone::Clone for AllJoynSessionMemberAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AllJoynSessionMemberRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AllJoynSessionMemberRemovedEventArgs {}
impl ::core::clone::Clone for AllJoynSessionMemberRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for AllJoynWatcherStoppedEventArgs {}
impl ::core::clone::Clone for AllJoynWatcherStoppedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynAboutData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynAboutData {}
impl ::core::clone::Clone for IAllJoynAboutData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynAboutDataView(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynAboutDataView {}
impl ::core::clone::Clone for IAllJoynAboutDataView {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynAboutDataViewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynAboutDataViewStatics {}
impl ::core::clone::Clone for IAllJoynAboutDataViewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynAcceptSessionJoiner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynAcceptSessionJoiner {}
impl ::core::clone::Clone for IAllJoynAcceptSessionJoiner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynAcceptSessionJoinerEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynAcceptSessionJoinerEventArgs {}
impl ::core::clone::Clone for IAllJoynAcceptSessionJoinerEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynAcceptSessionJoinerEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynAcceptSessionJoinerEventArgsFactory {}
impl ::core::clone::Clone for IAllJoynAcceptSessionJoinerEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynAuthenticationCompleteEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynAuthenticationCompleteEventArgs {}
impl ::core::clone::Clone for IAllJoynAuthenticationCompleteEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynBusAttachment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynBusAttachment {}
impl ::core::clone::Clone for IAllJoynBusAttachment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynBusAttachment2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynBusAttachment2 {}
impl ::core::clone::Clone for IAllJoynBusAttachment2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynBusAttachmentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynBusAttachmentFactory {}
impl ::core::clone::Clone for IAllJoynBusAttachmentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynBusAttachmentStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynBusAttachmentStateChangedEventArgs {}
impl ::core::clone::Clone for IAllJoynBusAttachmentStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynBusAttachmentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynBusAttachmentStatics {}
impl ::core::clone::Clone for IAllJoynBusAttachmentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynBusObject(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynBusObject {}
impl ::core::clone::Clone for IAllJoynBusObject {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynBusObjectFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynBusObjectFactory {}
impl ::core::clone::Clone for IAllJoynBusObjectFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynBusObjectStoppedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynBusObjectStoppedEventArgs {}
impl ::core::clone::Clone for IAllJoynBusObjectStoppedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynBusObjectStoppedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynBusObjectStoppedEventArgsFactory {}
impl ::core::clone::Clone for IAllJoynBusObjectStoppedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynCredentials(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynCredentials {}
impl ::core::clone::Clone for IAllJoynCredentials {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynCredentialsRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynCredentialsRequestedEventArgs {}
impl ::core::clone::Clone for IAllJoynCredentialsRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynCredentialsVerificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynCredentialsVerificationRequestedEventArgs {}
impl ::core::clone::Clone for IAllJoynCredentialsVerificationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynMessageInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynMessageInfo {}
impl ::core::clone::Clone for IAllJoynMessageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynMessageInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynMessageInfoFactory {}
impl ::core::clone::Clone for IAllJoynMessageInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynProducer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynProducer {}
impl ::core::clone::Clone for IAllJoynProducer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynProducerStoppedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynProducerStoppedEventArgs {}
impl ::core::clone::Clone for IAllJoynProducerStoppedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynProducerStoppedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynProducerStoppedEventArgsFactory {}
impl ::core::clone::Clone for IAllJoynProducerStoppedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynServiceInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynServiceInfo {}
impl ::core::clone::Clone for IAllJoynServiceInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynServiceInfoFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynServiceInfoFactory {}
impl ::core::clone::Clone for IAllJoynServiceInfoFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynServiceInfoRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynServiceInfoRemovedEventArgs {}
impl ::core::clone::Clone for IAllJoynServiceInfoRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynServiceInfoRemovedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynServiceInfoRemovedEventArgsFactory {}
impl ::core::clone::Clone for IAllJoynServiceInfoRemovedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynServiceInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynServiceInfoStatics {}
impl ::core::clone::Clone for IAllJoynServiceInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSession {}
impl ::core::clone::Clone for IAllJoynSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSessionJoinedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSessionJoinedEventArgs {}
impl ::core::clone::Clone for IAllJoynSessionJoinedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSessionJoinedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSessionJoinedEventArgsFactory {}
impl ::core::clone::Clone for IAllJoynSessionJoinedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSessionLostEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSessionLostEventArgs {}
impl ::core::clone::Clone for IAllJoynSessionLostEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSessionLostEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSessionLostEventArgsFactory {}
impl ::core::clone::Clone for IAllJoynSessionLostEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSessionMemberAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSessionMemberAddedEventArgs {}
impl ::core::clone::Clone for IAllJoynSessionMemberAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSessionMemberAddedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSessionMemberAddedEventArgsFactory {}
impl ::core::clone::Clone for IAllJoynSessionMemberAddedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSessionMemberRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSessionMemberRemovedEventArgs {}
impl ::core::clone::Clone for IAllJoynSessionMemberRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSessionMemberRemovedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSessionMemberRemovedEventArgsFactory {}
impl ::core::clone::Clone for IAllJoynSessionMemberRemovedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynSessionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynSessionStatics {}
impl ::core::clone::Clone for IAllJoynSessionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynStatusStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynStatusStatics {}
impl ::core::clone::Clone for IAllJoynStatusStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynWatcherStoppedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynWatcherStoppedEventArgs {}
impl ::core::clone::Clone for IAllJoynWatcherStoppedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAllJoynWatcherStoppedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAllJoynWatcherStoppedEventArgsFactory {}
impl ::core::clone::Clone for IAllJoynWatcherStoppedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
