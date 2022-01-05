#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAboutDataImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn DefaultAppName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultAppName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AppNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn DateOfManufacture(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetDateOfManufacture(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
    fn DefaultDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultDescription(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Descriptions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn DefaultManufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDefaultManufacturer(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Manufacturers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetModelNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SoftwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSoftwareVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SupportUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSupportUrl(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetAppId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAboutDataViewImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<i32>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn AJSoftwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AppId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn DateOfManufacture(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn DefaultLanguage(&self) -> ::windows::core::Result<super::super::Globalization::Language>;
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HardwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SoftwareVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedLanguages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Globalization::Language>>;
    fn SupportUrl(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Manufacturer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAboutDataViewStaticsImpl: Sized {
    fn GetDataBySessionPortAsync(&self, uniquename: &::windows::core::HSTRING, busattachment: &::core::option::Option<AllJoynBusAttachment>, sessionport: u16) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>;
    fn GetDataBySessionPortWithLanguageAsync(&self, uniquename: &::windows::core::HSTRING, busattachment: &::core::option::Option<AllJoynBusAttachment>, sessionport: u16, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>;
}
#[cfg(feature = "deprecated")]
pub trait IAllJoynAcceptSessionJoinerImpl: Sized {
    fn Accept(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAcceptSessionJoinerEventArgsImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionPort(&self) -> ::windows::core::Result<u16>;
    fn TrafficType(&self) -> ::windows::core::Result<AllJoynTrafficType>;
    fn SamePhysicalNode(&self) -> ::windows::core::Result<bool>;
    fn SameNetwork(&self) -> ::windows::core::Result<bool>;
    fn Accept(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAcceptSessionJoinerEventArgsFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING, sessionport: u16, traffictype: AllJoynTrafficType, proximity: u8, acceptsessionjoiner: &::core::option::Option<IAllJoynAcceptSessionJoiner>) -> ::windows::core::Result<AllJoynAcceptSessionJoinerEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynAuthenticationCompleteEventArgsImpl: Sized {
    fn AuthenticationMechanism(&self) -> ::windows::core::Result<AllJoynAuthenticationMechanism>;
    fn PeerUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachmentImpl: Sized {
    fn AboutData(&self) -> ::windows::core::Result<AllJoynAboutData>;
    fn ConnectionSpecification(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<AllJoynBusAttachmentState>;
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PingAsync(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>>;
    fn Connect(&self) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn StateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynBusAttachmentStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationMechanisms(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<AllJoynAuthenticationMechanism>>;
    fn CredentialsRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCredentialsRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CredentialsVerificationRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynCredentialsVerificationRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCredentialsVerificationRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn AuthenticationComplete(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAuthenticationCompleteEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAuthenticationComplete(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachment2Impl: Sized {
    fn GetAboutDataAsync(&self, serviceinfo: &::core::option::Option<AllJoynServiceInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>;
    fn GetAboutDataWithLanguageAsync(&self, serviceinfo: &::core::option::Option<AllJoynServiceInfo>, language: &::core::option::Option<super::super::Globalization::Language>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynAboutDataView>>;
    fn AcceptSessionJoinerRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynAcceptSessionJoinerEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAcceptSessionJoinerRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SessionJoined(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusAttachment, AllJoynSessionJoinedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSessionJoined(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachmentFactoryImpl: Sized {
    fn Create(&self, connectionspecification: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynBusAttachment>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachmentStateChangedEventArgsImpl: Sized {
    fn State(&self) -> ::windows::core::Result<AllJoynBusAttachmentState>;
    fn Status(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusAttachmentStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AllJoynBusAttachment>;
    fn GetWatcher(&self, requiredinterfaces: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::Enumeration::DeviceWatcher>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusObjectImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn AddProducer(&self, producer: &::core::option::Option<IAllJoynProducer>) -> ::windows::core::Result<()>;
    fn BusAttachment(&self) -> ::windows::core::Result<AllJoynBusAttachment>;
    fn Session(&self) -> ::windows::core::Result<AllJoynSession>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynBusObject, AllJoynBusObjectStoppedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusObjectFactoryImpl: Sized {
    fn Create(&self, objectpath: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynBusObject>;
    fn CreateWithBusAttachment(&self, objectpath: &::windows::core::HSTRING, busattachment: &::core::option::Option<AllJoynBusAttachment>) -> ::windows::core::Result<AllJoynBusObject>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusObjectStoppedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynBusObjectStoppedEventArgsFactoryImpl: Sized {
    fn Create(&self, status: i32) -> ::windows::core::Result<AllJoynBusObjectStoppedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynCredentialsImpl: Sized {
    fn AuthenticationMechanism(&self) -> ::windows::core::Result<AllJoynAuthenticationMechanism>;
    fn Certificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn SetCertificate(&self, value: &::core::option::Option<super::super::Security::Cryptography::Certificates::Certificate>) -> ::windows::core::Result<()>;
    fn PasswordCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetPasswordCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn Timeout(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SetTimeout(&self, value: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynCredentialsRequestedEventArgsImpl: Sized {
    fn AttemptCount(&self) -> ::windows::core::Result<u16>;
    fn Credentials(&self) -> ::windows::core::Result<AllJoynCredentials>;
    fn PeerUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RequestedUserName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynCredentialsVerificationRequestedEventArgsImpl: Sized {
    fn AuthenticationMechanism(&self) -> ::windows::core::Result<AllJoynAuthenticationMechanism>;
    fn PeerUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PeerCertificate(&self) -> ::windows::core::Result<super::super::Security::Cryptography::Certificates::Certificate>;
    fn PeerCertificateErrorSeverity(&self) -> ::windows::core::Result<super::super::Networking::Sockets::SocketSslErrorSeverity>;
    fn PeerCertificateErrors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::ChainValidationResult>>;
    fn PeerIntermediateCertificates(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Security::Cryptography::Certificates::Certificate>>;
    fn Accept(&self) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynMessageInfoImpl: Sized {
    fn SenderUniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynMessageInfoFactoryImpl: Sized {
    fn Create(&self, senderuniquename: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynMessageInfo>;
}
#[cfg(feature = "deprecated")]
pub trait IAllJoynProducerImpl: Sized {
    fn SetBusObject(&self, busobject: &::core::option::Option<AllJoynBusObject>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynProducerStoppedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynProducerStoppedEventArgsFactoryImpl: Sized {
    fn Create(&self, status: i32) -> ::windows::core::Result<AllJoynProducerStoppedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ObjectPath(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SessionPort(&self) -> ::windows::core::Result<u16>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING, objectpath: &::windows::core::HSTRING, sessionport: u16) -> ::windows::core::Result<AllJoynServiceInfo>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoRemovedEventArgsImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoRemovedEventArgsFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynServiceInfoRemovedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynServiceInfoStaticsImpl: Sized {
    fn FromIdAsync(&self, deviceid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynServiceInfo>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<i32>;
    fn Status(&self) -> ::windows::core::Result<i32>;
    fn RemoveMemberAsync(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<i32>>;
    fn MemberAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMemberAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn MemberRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionMemberRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveMemberRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Lost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<AllJoynSession, AllJoynSessionLostEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveLost(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionJoinedEventArgsImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<AllJoynSession>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionJoinedEventArgsFactoryImpl: Sized {
    fn Create(&self, session: &::core::option::Option<AllJoynSession>) -> ::windows::core::Result<AllJoynSessionJoinedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionLostEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<AllJoynSessionLostReason>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionLostEventArgsFactoryImpl: Sized {
    fn Create(&self, reason: AllJoynSessionLostReason) -> ::windows::core::Result<AllJoynSessionLostEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionMemberAddedEventArgsImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionMemberAddedEventArgsFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynSessionMemberAddedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionMemberRemovedEventArgsImpl: Sized {
    fn UniqueName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionMemberRemovedEventArgsFactoryImpl: Sized {
    fn Create(&self, uniquename: &::windows::core::HSTRING) -> ::windows::core::Result<AllJoynSessionMemberRemovedEventArgs>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynSessionStaticsImpl: Sized {
    fn GetFromServiceInfoAsync(&self, serviceinfo: &::core::option::Option<AllJoynServiceInfo>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynSession>>;
    fn GetFromServiceInfoAndBusAttachmentAsync(&self, serviceinfo: &::core::option::Option<AllJoynServiceInfo>, busattachment: &::core::option::Option<AllJoynBusAttachment>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AllJoynSession>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynStatusStaticsImpl: Sized {
    fn Ok(&self) -> ::windows::core::Result<i32>;
    fn Fail(&self) -> ::windows::core::Result<i32>;
    fn OperationTimedOut(&self) -> ::windows::core::Result<i32>;
    fn OtherEndClosed(&self) -> ::windows::core::Result<i32>;
    fn ConnectionRefused(&self) -> ::windows::core::Result<i32>;
    fn AuthenticationFailed(&self) -> ::windows::core::Result<i32>;
    fn AuthenticationRejectedByUser(&self) -> ::windows::core::Result<i32>;
    fn SslConnectFailed(&self) -> ::windows::core::Result<i32>;
    fn SslIdentityVerificationFailed(&self) -> ::windows::core::Result<i32>;
    fn InsufficientSecurity(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument1(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument2(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument3(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument4(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument5(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument6(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument7(&self) -> ::windows::core::Result<i32>;
    fn InvalidArgument8(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynWatcherStoppedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IAllJoynWatcherStoppedEventArgsFactoryImpl: Sized {
    fn Create(&self, status: i32) -> ::windows::core::Result<AllJoynWatcherStoppedEventArgs>;
}
