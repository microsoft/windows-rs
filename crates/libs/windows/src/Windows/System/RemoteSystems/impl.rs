#[cfg(feature = "implement_exclusive")]
pub trait IKnownRemoteSystemCapabilitiesStaticsImpl: Sized {
    fn AppService(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LaunchUri(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RemoteSession(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SpatialEntity(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Status(&self) -> ::windows::core::Result<RemoteSystemStatus>;
    fn IsAvailableByProximity(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem2Impl: Sized {
    fn IsAvailableBySpatialProximity(&self) -> ::windows::core::Result<bool>;
    fn GetCapabilitySupportedAsync(&self, capabilityname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem3Impl: Sized {
    fn ManufacturerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ModelDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem4Impl: Sized {
    fn Platform(&self) -> ::windows::core::Result<RemoteSystemPlatform>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem5Impl: Sized {
    fn Apps(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<RemoteSystemApp>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystem6Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAddedEventArgsImpl: Sized {
    fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAppImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsAvailableByProximity(&self) -> ::windows::core::Result<bool>;
    fn IsAvailableBySpatialProximity(&self) -> ::windows::core::Result<bool>;
    fn Attributes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemApp2Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
    fn ConnectionToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAppRegistrationImpl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
    fn Attributes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>;
    fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAppRegistrationStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<RemoteSystemAppRegistration>;
    fn GetForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<RemoteSystemAppRegistration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAuthorizationKindFilterImpl: Sized {
    fn RemoteSystemAuthorizationKind(&self) -> ::windows::core::Result<RemoteSystemAuthorizationKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemAuthorizationKindFilterFactoryImpl: Sized {
    fn Create(&self, remotesystemauthorizationkind: RemoteSystemAuthorizationKind) -> ::windows::core::Result<RemoteSystemAuthorizationKindFilter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionInfoImpl: Sized {
    fn IsProximal(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionInfoStaticsImpl: Sized {
    fn TryCreateFromAppServiceConnection(&self, connection: &::core::option::Option<super::super::ApplicationModel::AppService::AppServiceConnection>) -> ::windows::core::Result<RemoteSystemConnectionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestImpl: Sized {
    fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequest2Impl: Sized {
    fn RemoteSystemApp(&self) -> ::windows::core::Result<RemoteSystemApp>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequest3Impl: Sized {
    fn ConnectionToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestFactoryImpl: Sized {
    fn Create(&self, remotesystem: &::core::option::Option<RemoteSystem>) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestStaticsImpl: Sized {
    fn CreateForApp(&self, remotesystemapp: &::core::option::Option<RemoteSystemApp>) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemConnectionRequestStatics2Impl: Sized {
    fn CreateFromConnectionToken(&self, connectiontoken: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
    fn CreateFromConnectionTokenForUser(&self, user: &::core::option::Option<super::User>, connectiontoken: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemConnectionRequest>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemDiscoveryTypeFilterImpl: Sized {
    fn RemoteSystemDiscoveryType(&self) -> ::windows::core::Result<RemoteSystemDiscoveryType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemDiscoveryTypeFilterFactoryImpl: Sized {
    fn Create(&self, discoverytype: RemoteSystemDiscoveryType) -> ::windows::core::Result<RemoteSystemDiscoveryTypeFilter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemEnumerationCompletedEventArgsImpl: Sized {}
pub trait IRemoteSystemFilterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindFilterImpl: Sized {
    fn RemoteSystemKinds(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindFilterFactoryImpl: Sized {
    fn Create(&self, remotesystemkinds: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<RemoteSystemKindFilter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindStaticsImpl: Sized {
    fn Phone(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Hub(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Holographic(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Desktop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Xbox(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemKindStatics2Impl: Sized {
    fn Iot(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Tablet(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Laptop(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemRemovedEventArgsImpl: Sized {
    fn RemoteSystemId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ControllerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Disconnected(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSession, RemoteSystemSessionDisconnectedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisconnected(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateParticipantWatcher(&self) -> ::windows::core::Result<RemoteSystemSessionParticipantWatcher>;
    fn SendInvitationAsync(&self, invitee: &::core::option::Option<RemoteSystem>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionAddedEventArgsImpl: Sized {
    fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionControllerImpl: Sized {
    fn JoinRequested(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionController, RemoteSystemSessionJoinRequestedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveJoinRequested(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoveParticipantAsync(&self, pparticipant: &::core::option::Option<RemoteSystemSessionParticipant>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn CreateSessionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionCreationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionControllerFactoryImpl: Sized {
    fn CreateController(&self, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemSessionController>;
    fn CreateControllerWithSessionOptions(&self, displayname: &::windows::core::HSTRING, options: &::core::option::Option<RemoteSystemSessionOptions>) -> ::windows::core::Result<RemoteSystemSessionController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionCreationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionCreationStatus>;
    fn Session(&self) -> ::windows::core::Result<RemoteSystemSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionDisconnectedEventArgsImpl: Sized {
    fn Reason(&self) -> ::windows::core::Result<RemoteSystemSessionDisconnectedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInfoImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ControllerDisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn JoinAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemSessionJoinResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInvitationImpl: Sized {
    fn Sender(&self) -> ::windows::core::Result<RemoteSystem>;
    fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInvitationListenerImpl: Sized {
    fn InvitationReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionInvitationListener, RemoteSystemSessionInvitationReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveInvitationReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionInvitationReceivedEventArgsImpl: Sized {
    fn Invitation(&self) -> ::windows::core::Result<RemoteSystemSessionInvitation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionJoinRequestImpl: Sized {
    fn Participant(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
    fn Accept(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionJoinRequestedEventArgsImpl: Sized {
    fn JoinRequest(&self) -> ::windows::core::Result<RemoteSystemSessionJoinRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionJoinResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionJoinStatus>;
    fn Session(&self) -> ::windows::core::Result<RemoteSystemSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionMessageChannelImpl: Sized {
    fn Session(&self) -> ::windows::core::Result<RemoteSystemSession>;
    fn BroadcastValueSetAsync(&self, messagedata: &::core::option::Option<super::super::Foundation::Collections::ValueSet>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SendValueSetAsync(&self, messagedata: &::core::option::Option<super::super::Foundation::Collections::ValueSet>, participant: &::core::option::Option<RemoteSystemSessionParticipant>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn SendValueSetToParticipantsAsync(&self, messagedata: &::core::option::Option<super::super::Foundation::Collections::ValueSet>, participants: &::core::option::Option<super::super::Foundation::Collections::IIterable<RemoteSystemSessionParticipant>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn ValueSetReceived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionMessageChannel, RemoteSystemSessionValueSetReceivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveValueSetReceived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionMessageChannelFactoryImpl: Sized {
    fn Create(&self, session: &::core::option::Option<RemoteSystemSession>, channelname: &::windows::core::HSTRING) -> ::windows::core::Result<RemoteSystemSessionMessageChannel>;
    fn CreateWithReliability(&self, session: &::core::option::Option<RemoteSystemSession>, channelname: &::windows::core::HSTRING, reliability: RemoteSystemSessionMessageChannelReliability) -> ::windows::core::Result<RemoteSystemSessionMessageChannel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionOptionsImpl: Sized {
    fn IsInviteOnly(&self) -> ::windows::core::Result<bool>;
    fn SetIsInviteOnly(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantImpl: Sized {
    fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem>;
    fn GetHostNames(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Networking::HostName>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantAddedEventArgsImpl: Sized {
    fn Participant(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantRemovedEventArgsImpl: Sized {
    fn Participant(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionParticipantWatcherImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionParticipantWatcherStatus>;
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, RemoteSystemSessionParticipantRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionParticipantWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionRemovedEventArgsImpl: Sized {
    fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionStaticsImpl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<RemoteSystemSessionWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionUpdatedEventArgsImpl: Sized {
    fn SessionInfo(&self) -> ::windows::core::Result<RemoteSystemSessionInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionValueSetReceivedEventArgsImpl: Sized {
    fn Sender(&self) -> ::windows::core::Result<RemoteSystemSessionParticipant>;
    fn Message(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemSessionWatcherImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<RemoteSystemSessionWatcherStatus>;
    fn Added(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Updated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Removed(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemSessionWatcher, RemoteSystemSessionRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStaticsImpl: Sized {
    fn FindByHostNameAsync(&self, hostname: &::core::option::Option<super::super::Networking::HostName>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystem>>;
    fn CreateWatcher(&self) -> ::windows::core::Result<RemoteSystemWatcher>;
    fn CreateWatcherWithFilters(&self, filters: &::core::option::Option<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>) -> ::windows::core::Result<RemoteSystemWatcher>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<RemoteSystemAccessStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatics2Impl: Sized {
    fn IsAuthorizationKindEnabled(&self, kind: RemoteSystemAuthorizationKind) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatics3Impl: Sized {
    fn CreateWatcherForUser(&self, user: &::core::option::Option<super::User>) -> ::windows::core::Result<RemoteSystemWatcher>;
    fn CreateWatcherWithFiltersForUser(&self, user: &::core::option::Option<super::User>, filters: &::core::option::Option<super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>>) -> ::windows::core::Result<RemoteSystemWatcher>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatusTypeFilterImpl: Sized {
    fn RemoteSystemStatusType(&self) -> ::windows::core::Result<RemoteSystemStatusType>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemStatusTypeFilterFactoryImpl: Sized {
    fn Create(&self, remotesystemstatustype: RemoteSystemStatusType) -> ::windows::core::Result<RemoteSystemStatusTypeFilter>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemUpdatedEventArgsImpl: Sized {
    fn RemoteSystem(&self) -> ::windows::core::Result<RemoteSystem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcherImpl: Sized {
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn RemoteSystemAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteSystemAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoteSystemUpdated(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemUpdatedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteSystemUpdated(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn RemoteSystemRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRemoteSystemRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcher2Impl: Sized {
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemEnumerationCompletedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ErrorOccurred(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<RemoteSystemWatcher, RemoteSystemWatcherErrorOccurredEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveErrorOccurred(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcher3Impl: Sized {
    fn User(&self) -> ::windows::core::Result<super::User>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWatcherErrorOccurredEventArgsImpl: Sized {
    fn Error(&self) -> ::windows::core::Result<RemoteSystemWatcherError>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWebAccountFilterImpl: Sized {
    fn Account(&self) -> ::windows::core::Result<super::super::Security::Credentials::WebAccount>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IRemoteSystemWebAccountFilterFactoryImpl: Sized {
    fn Create(&self, account: &::core::option::Option<super::super::Security::Credentials::WebAccount>) -> ::windows::core::Result<RemoteSystemWebAccountFilter>;
}
