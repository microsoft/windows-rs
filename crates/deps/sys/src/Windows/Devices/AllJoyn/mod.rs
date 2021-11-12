#![allow(non_snake_case, non_camel_case_types)]
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
#[repr(C)]
pub struct AllJoynAuthenticationMechanism(i32);
#[repr(transparent)]
pub struct AllJoynBusAttachment(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AllJoynBusAttachmentState(i32);
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
#[repr(C)]
pub struct AllJoynSessionLostReason(i32);
#[repr(transparent)]
pub struct AllJoynSessionMemberAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynSessionMemberRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AllJoynStatus(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AllJoynTrafficType(i32);
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
