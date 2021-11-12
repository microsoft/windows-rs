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
    pub const None: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(0i32);
    pub const SrpAnonymous: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(1i32);
    pub const SrpLogon: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(2i32);
    pub const EcdheNull: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(3i32);
    pub const EcdhePsk: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(4i32);
    pub const EcdheEcdsa: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(5i32);
    pub const EcdheSpeke: AllJoynAuthenticationMechanism = AllJoynAuthenticationMechanism(6i32);
}
#[repr(transparent)]
pub struct AllJoynBusAttachment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynBusAttachmentState(pub i32);
impl AllJoynBusAttachmentState {
    pub const Disconnected: AllJoynBusAttachmentState = AllJoynBusAttachmentState(0i32);
    pub const Connecting: AllJoynBusAttachmentState = AllJoynBusAttachmentState(1i32);
    pub const Connected: AllJoynBusAttachmentState = AllJoynBusAttachmentState(2i32);
    pub const Disconnecting: AllJoynBusAttachmentState = AllJoynBusAttachmentState(3i32);
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
    pub const None: AllJoynSessionLostReason = AllJoynSessionLostReason(0i32);
    pub const ProducerLeftSession: AllJoynSessionLostReason = AllJoynSessionLostReason(1i32);
    pub const ProducerClosedAbruptly: AllJoynSessionLostReason = AllJoynSessionLostReason(2i32);
    pub const RemovedByProducer: AllJoynSessionLostReason = AllJoynSessionLostReason(3i32);
    pub const LinkTimeout: AllJoynSessionLostReason = AllJoynSessionLostReason(4i32);
    pub const Other: AllJoynSessionLostReason = AllJoynSessionLostReason(5i32);
}
#[repr(transparent)]
pub struct AllJoynSessionMemberAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynSessionMemberRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynTrafficType(pub i32);
impl AllJoynTrafficType {
    pub const Unknown: AllJoynTrafficType = AllJoynTrafficType(0i32);
    pub const Messages: AllJoynTrafficType = AllJoynTrafficType(1i32);
    pub const RawUnreliable: AllJoynTrafficType = AllJoynTrafficType(2i32);
    pub const RawReliable: AllJoynTrafficType = AllJoynTrafficType(4i32);
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
